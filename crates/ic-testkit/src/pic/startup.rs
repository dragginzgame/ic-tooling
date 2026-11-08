use std::{
    fmt::Write as _,
    fs::{self, File, OpenOptions},
    io,
    path::{Path, PathBuf},
    process::{Command, ExitStatus, Stdio},
    thread,
    time::{Duration, Instant},
};

#[cfg(unix)]
use std::os::unix::fs::OpenOptionsExt as _;

use crate::pocket_ic::{ClientError, PocketIc, PocketIcBuilder};
use ic_host_fs::{directory::create_unique_directory, owned_directory::OwnedDirectory};
use ic_host_process::child::{CleanupError, CleanupPolicy, OwnedChild};

const STARTUP_POLL_INTERVAL: Duration = Duration::from_millis(20);
const SERVER_OUTPUT_LIMIT: usize = 16 * 1024;
// PocketIC publishes a decimal u16 and a newline. Leave whitespace room
// without allowing readiness polling to read an arbitrary-size file.
const SERVER_PORT_FILE_LIMIT: usize = 64;

/// Explicit bounded source and policy for one `PocketIC` startup.
///
/// Spawn-only settings cannot be attached to a borrowed connection. Callers own
/// binary admission, output retention and namespace custody.
#[derive(Clone, Debug, Eq, PartialEq)]
pub enum PocketIcStartupConfig {
    /// Spawn and own one exact server executable.
    Spawn {
        /// Caller-selected, already prepared server binary.
        server_binary: PathBuf,
        /// Complete startup deadline; must be positive.
        timeout: Duration,
        /// Optional hard lifetime, in whole seconds; `None` disables it.
        server_hard_ttl: Option<Duration>,
        /// Optional operation-idle lifetime; `None` uses `PocketIC`'s default.
        server_idle_ttl: Option<Duration>,
        /// New caller-owned stdout/stderr files, or temporary output when absent.
        /// Existing entries are refused. Created files survive failure and teardown.
        server_output_files: Option<(PathBuf, PathBuf)>,
    },
    /// Borrow an existing server without claiming its process lifetime.
    Connect {
        /// Existing server URL, admitted by the upstream builder.
        server_url: String,
        /// Complete startup deadline; must be positive.
        timeout: Duration,
    },
}

/// Caller-owned `PocketIC` server process with bounded startup and output capture.
///
/// Dropping the handle terminates and waits for the managed child. On Unix,
/// teardown also terminates descendants remaining in its owned process group.
/// Callers may create several instances through [`Self::url`] and
/// [`PocketIcStartupConfig::Connect`] while retaining explicit server ownership.
/// The handle is process-local and does not coordinate ownership across Cargo
/// or test-runner processes; use an externally owned server with bounded
/// connect mode for that topology.
/// The handle owns no binary discovery, download, cache, or compatibility policy.
pub struct PocketIcManagedServer {
    server: ManagedServer,
    url: String,
}

/// An upstream instance together with the server owned by its construction.
///
/// Call [`Self::close`] to await instance deletion while retaining server custody.
/// Rust Drop performs no client network I/O; it stops an owned server and all its
/// instances. Connect mode borrows the caller's server, so explicit close matters.
/// [`Self::into_parts`] transfers teardown ordering and reconciliation to the caller.
///
/// Field projection cannot silently detach the client from its server:
/// ```compile_fail
/// use ic_testkit::pic::PocketIcInstance;
/// fn detach<T>(owned: PocketIcInstance<T>) -> T {
///     owned.instance
/// }
/// ```
/// Nor can a client borrow outlive a temporary owner:
/// ```compile_fail
/// use ic_testkit::pic::PocketIcInstance;
/// let client = PocketIcInstance::from_borrowed_server(String::new()).instance();
/// assert!(client.is_empty());
/// ```
#[must_use]
pub struct PocketIcInstance<T> {
    instance: T,
    server: Option<PocketIcManagedServer>,
}

impl<T> PocketIcInstance<T> {
    /// Retain an existing client whose server lifetime is owned elsewhere.
    ///
    /// This creates no server or instance and claims no server process authority.
    /// The client retains its upstream deletion behavior; its external server
    /// must outlive it. Use a startup configuration for a managed server owner.
    pub const fn from_borrowed_server(instance: T) -> Self {
        Self {
            instance,
            server: None,
        }
    }

    /// Borrow the client while retaining this record's managed-server ownership.
    #[must_use]
    pub const fn instance(&self) -> &T {
        &self.instance
    }

    /// Mutably borrow the client while retaining managed-server ownership.
    /// Supports client setup such as `PocketIc::make_live` without splitting the
    /// owner. Replacing the client transfers responsibility for the old client's
    /// server lifetime to the caller, just as with [`Self::into_parts`].
    ///
    /// The borrow cannot outlive the owner:
    /// ```compile_fail
    /// use ic_testkit::pic::PocketIcInstance;
    /// let client = PocketIcInstance::from_borrowed_server(String::new()).instance_mut();
    /// client.push_str("gateway");
    /// ```
    pub const fn instance_mut(&mut self) -> &mut T {
        &mut self.instance
    }

    /// Borrow the exact managed server, or `None` for an externally owned server.
    #[must_use]
    pub const fn server(&self) -> Option<&PocketIcManagedServer> {
        self.server.as_ref()
    }

    /// Transfer the client and optional server, without deleting or stopping either.
    ///
    /// The caller must retain the server until client deletion is complete. For
    /// a nonblocking client, explicitly await `instance.drop()` before dropping
    /// the returned server. No implicit asynchronous or blocking cleanup is added.
    #[must_use]
    pub fn into_parts(self) -> (T, Option<PocketIcManagedServer>) {
        (self.instance, self.server)
    }
}

impl PocketIcInstance<PocketIc> {
    /// Delete this instance while retaining its managed server until the request finishes.
    /// Dropping the future stops an owned server; borrowed servers require caller reconciliation.
    /// # Errors
    /// Returns the client deletion failure, without retrying it.
    pub async fn close(self) -> Result<(), ClientError> {
        let (instance, server) = self.into_parts();
        let result = instance.drop().await;
        drop(server);
        result
    }
}

/// Bounded lossy UTF-8 output captured from a managed `PocketIC` server.
///
/// Each stream retains at most the first 16 KiB. A textual suffix reports the
/// number of omitted bytes when truncation occurred. Unreadable streams and
/// paths replaced with non-regular files are omitted.
#[derive(Clone, Debug, Default, Eq, PartialEq)]
pub struct PocketIcManagedServerOutput {
    stdout: String,
    stderr: String,
}

/// Original startup failure, bounded server output and typed cleanup failures.
///
/// Match [`Self::failure`] to classify the original cause. Cleanup diagnostics
/// never replace it, and command/server failures retain separate ownership.
/// Output is an inert bounded snapshot; it grants no process authority.
#[derive(Debug)]
pub struct PocketIcStartupError {
    failure: Box<PocketIcStartupFailure>,
    output: PocketIcManagedServerOutput,
    command_cleanup: Option<Box<CleanupError>>,
    server_cleanup: Option<Box<CleanupError>>,
}

impl PocketIcStartupError {
    fn new(failure: PocketIcStartupFailure) -> Self {
        Self {
            failure: Box::new(failure),
            output: PocketIcManagedServerOutput::default(),
            command_cleanup: None,
            server_cleanup: None,
        }
    }

    /// Original cause, unaffected by subsequent cleanup failure.
    #[must_use]
    pub fn failure(&self) -> &PocketIcStartupFailure {
        &self.failure
    }

    /// Bounded output from the server owned by this operation, if any.
    #[must_use]
    pub const fn output(&self) -> &PocketIcManagedServerOutput {
        &self.output
    }

    /// Failure cleaning the command's owned process group/direct child.
    #[must_use]
    pub fn command_cleanup(&self) -> Option<&CleanupError> {
        self.command_cleanup.as_deref()
    }

    /// Failure cleaning the managed server's owned process group/direct child.
    #[must_use]
    pub fn server_cleanup(&self) -> Option<&CleanupError> {
        self.server_cleanup.as_deref()
    }

    fn with_cleanup(
        mut self,
        command: Option<CleanupError>,
        server: Option<(PocketIcManagedServerOutput, Option<CleanupError>)>,
    ) -> Self {
        self.command_cleanup = command.map(Box::new);
        if let Some((output, cleanup)) = server {
            self.output = output;
            self.server_cleanup = cleanup.map(Box::new);
        }
        self
    }
}

/// Original failure, independent of output capture and cleanup outcomes.
#[non_exhaustive]
#[derive(Debug, thiserror::Error)]
pub enum PocketIcStartupFailure {
    /// Neither the shared server URL nor an explicit executable was configured.
    #[error(
        "configure IC_TESTKIT_POCKET_IC_URL or POCKET_IC_BIN; prepare a verified binary with make install-server"
    )]
    NotConfigured,
    /// A selected environment value was empty or was not valid Unicode.
    #[error("invalid selected environment value: {variable}")]
    InvalidEnvironment {
        /// Selected environment variable name.
        variable: &'static str,
    },
    /// The bounded version probe failed, including nonzero exit or timeout.
    #[error("PocketIC version probe failed: {source}")]
    ServerVersionProbe {
        /// Underlying operation failure.
        source: ic_host_process::tool::ToolError,
    },
    /// The selected executable does not report the qualified server identity.
    #[error("PocketIC version mismatch: expected {expected:?}, observed {:?}", String::from_utf8_lossy(.observed))]
    ServerVersionMismatch {
        /// Required server identity.
        expected: String,
        /// Captured version-probe output.
        observed: Vec<u8>,
    },
    /// Command execution failed.
    #[error("command {} failed: {source}", .program.display())]
    CommandRun {
        /// Selected command executable.
        program: PathBuf,
        /// Underlying operation failure.
        source: io::Error,
    },
    /// The caller supplied a zero timeout or unusable hard TTL.
    #[error("{message}")]
    InvalidConfiguration {
        /// Configuration diagnostic.
        message: String,
    },
    /// A caller-provided existing server URL could not be parsed.
    #[error("invalid PocketIC server URL {server_url:?}: {message}")]
    InvalidServerUrl {
        /// Caller-provided server URL.
        server_url: String,
        /// Configuration diagnostic.
        message: String,
    },
    /// Preparing or inspecting bounded startup files failed.
    #[error("failed to {operation} at {}: {source}", .path.display())]
    Io {
        /// Failed filesystem operation.
        operation: &'static str,
        /// Affected filesystem path.
        path: PathBuf,
        /// Underlying operation failure.
        source: io::Error,
    },
    /// The configured `PocketIC` server process could not be spawned.
    #[error("failed to spawn PocketIC server {}: {source}", .server_binary.display())]
    ServerSpawn {
        /// Selected server executable.
        server_binary: PathBuf,
        /// Underlying operation failure.
        source: io::Error,
    },
    /// The managed `PocketIC` server exited during construction or command execution.
    #[error("PocketIC server {} exited with {status} after {elapsed:?}", .server_binary.display())]
    ServerExited {
        /// Selected server executable.
        server_binary: PathBuf,
        /// Observed server exit status.
        status: ExitStatus,
        /// Time spent before the server exited.
        elapsed: Duration,
    },
    /// The managed server did not publish a usable port before the deadline.
    #[error("PocketIC server {} was not ready within {timeout:?}", .server_binary.display())]
    ReadinessTimeout {
        /// Selected server executable.
        server_binary: PathBuf,
        /// Caller-selected deadline budget.
        timeout: Duration,
    },
    /// The managed server published an invalid or oversized port-file value.
    #[error("PocketIC server {} published invalid port {value:?}", .server_binary.display())]
    InvalidServerPort {
        /// Selected server executable.
        server_binary: PathBuf,
        /// Rejected port-file contents.
        value: String,
    },
    /// `PocketIC` instance creation did not finish before the startup deadline.
    #[error("PocketIC instance creation exceeded {timeout:?}")]
    InstanceCreationTimeout {
        /// Caller-selected deadline budget.
        timeout: Duration,
    },
    /// Upstream `PocketIC` construction failed before returning an instance.
    #[error("PocketIC startup failed: {source}")]
    BuilderFailed {
        /// Underlying operation failure.
        source: ClientError,
    },
    /// The command completed, but the owned server could not be cleaned up.
    #[error("PocketIC server cleanup failed after command exited with {command_status}")]
    ServerCleanup {
        /// Exit status of the completed command.
        command_status: ExitStatus,
    },
}

impl PocketIcStartupConfig {
    /// Select the shared environment contract without discovery or downloads.
    ///
    /// `IC_TESTKIT_POCKET_IC_URL` takes precedence over `POCKET_IC_BIN`. An
    /// explicitly empty or invalid selected value fails rather than falling
    /// back. URL mode never launches a version probe or claims server ownership.
    /// Binary mode resolves the explicit path and checks `--version` against
    /// Testkit's supported protocol range using the shared bounded capture
    /// engine. This is version qualification, not executable-byte admission;
    /// prepare and verify the binary with `ic-testkit-server setup` / `check`.
    /// The probe has its own `timeout`; subsequent startup has the same budget.
    ///
    /// # Errors
    /// Rejects missing or invalid configuration and failed or incompatible version probes.
    pub fn from_env(timeout: Duration) -> Result<Self, PocketIcStartupError> {
        Self::from_environment(timeout, |name| std::env::var_os(name))
    }

    fn from_environment(
        timeout: Duration,
        mut variable: impl FnMut(&str) -> Option<std::ffi::OsString>,
    ) -> Result<Self, PocketIcStartupError> {
        if let Some(value) = variable("IC_TESTKIT_POCKET_IC_URL") {
            let server_url = value
                .into_string()
                .ok()
                .filter(|value| !value.is_empty())
                .ok_or_else(|| {
                    PocketIcStartupError::new(PocketIcStartupFailure::InvalidEnvironment {
                        variable: "IC_TESTKIT_POCKET_IC_URL",
                    })
                })?;
            let config = Self::Connect {
                server_url: (&server_url).into(),
                timeout,
            };
            config.validate()?;
            parse_server_url(&server_url)?;
            return Ok(config);
        }
        let value = variable("POCKET_IC_BIN")
            .ok_or_else(|| PocketIcStartupError::new(PocketIcStartupFailure::NotConfigured))?;
        if value.is_empty() {
            return Err(PocketIcStartupError::new(
                PocketIcStartupFailure::InvalidEnvironment {
                    variable: "POCKET_IC_BIN",
                },
            ));
        }
        let path = PathBuf::from(value);
        let binary = fs::canonicalize(&path).map_err(|source| {
            PocketIcStartupError::new(PocketIcStartupFailure::Io {
                operation: "resolve configured PocketIC executable",
                path,
                source,
            })
        })?;
        let config = Self::Spawn {
            server_binary: (&binary).into(),
            timeout,
            server_hard_ttl: None,
            server_idle_ttl: None,
            server_output_files: None,
        };
        config.validate()?;
        let evidence = ic_host_process::tool::capture_group_command(
            Command::new(&binary).arg("--version"),
            ic_host_process::tool::OutputLimits {
                stdout: ic_host_process::tool::OutputLimit::Terminate(SERVER_OUTPUT_LIMIT),
                stderr: ic_host_process::tool::OutputLimit::Terminate(SERVER_OUTPUT_LIMIT),
                timeout: Some(timeout),
            },
        )
        .and_then(ic_host_process::tool::CompletedExecution::require_success)
        .map_err(|source| {
            PocketIcStartupError::new(PocketIcStartupFailure::ServerVersionProbe { source })
        })?;
        let expected = super::server::pocket_ic_server_requirement();
        if !super::supports_pocket_ic_server(&evidence.stdout) {
            return Err(PocketIcStartupError::new(
                PocketIcStartupFailure::ServerVersionMismatch {
                    expected,
                    observed: evidence.stdout,
                },
            ));
        }
        Ok(config)
    }

    /// Complete startup deadline for either source.
    #[must_use]
    pub const fn timeout(&self) -> Duration {
        match self {
            Self::Spawn { timeout, .. } | Self::Connect { timeout, .. } => *timeout,
        }
    }

    /// Construct an instance on the caller's Tokio runtime.
    ///
    /// Preserves deadline, readiness, typed client/exit failures and bounded output.
    /// Requires a Tokio runtime with its time driver enabled.
    /// Readiness and instance waits yield; process spawning and small file reads
    /// remain synchronous, as does exact owned-child cleanup. The deadline cannot
    /// preempt synchronous work inside an upstream future poll (such as copying
    /// a read-only state directory); it is enforced when that poll returns.
    ///
    /// Dropping the future cancels local construction and terminates/reaps its
    /// owned server, including during readiness. Cancellation has no returned error;
    /// select output files to retain diagnostics. A borrowed server is untouched.
    /// If a borrowed server created an instance before timeout or a lost reply, local
    /// cancellation cannot identify or delete that instance; the caller owns this
    /// reconciliation. The deadline covers construction, not subsequent deletion.
    ///
    /// ```no_run
    /// use ic_testkit::pocket_ic::PocketIcBuilder;
    /// use ic_testkit::pic::PocketIcStartupConfig;
    /// use std::{path::Path, time::Duration};
    /// # async fn run(binary: &Path) -> Result<(), Box<dyn std::error::Error>> {
    /// let owned = PocketIcStartupConfig::Spawn { server_binary: binary.into(), timeout: Duration::from_secs(60), server_hard_ttl: None, server_idle_ttl: None, server_output_files: None }
    ///     .build_instance(PocketIcBuilder::new(std::time::Duration::from_secs(300)).with_application_subnet()).await?;
    /// let canister = owned.instance().create_canister().await?;
    /// assert!(owned.instance().canister_exists(canister).await?);
    /// let (instance, server) = owned.into_parts();
    /// instance.drop().await?;
    /// drop(server);
    /// # Ok(())
    /// # }
    /// ```
    ///
    /// # Errors
    /// Returns configuration, startup, deadline or instance-creation failures with cleanup evidence.
    pub async fn build_instance(
        self,
        builder: PocketIcBuilder,
    ) -> Result<PocketIcInstance<PocketIc>, PocketIcStartupError> {
        let startup = StartupDeadline::new(&self)?;
        let (server, server_url) = match self {
            Self::Connect { server_url, .. } => (None, server_url),
            spawn @ Self::Spawn { .. } => {
                let mut server = spawn.spawn_server(startup.started)?;
                let server_url = loop {
                    match server.poll_ready(&startup) {
                        Ok(Some(url)) => break url,
                        Ok(None) => {
                            tokio::time::sleep(startup.wait_interval(Instant::now(), true)).await;
                        }
                        Err(error) => return Err(finalize_failure(error, None, Some(server))),
                    }
                };
                (Some(server), server_url)
            }
        };
        build_bounded(builder, &server_url, &startup, server).await
    }

    /// Start a caller-owned managed server without constructing an instance.
    ///
    /// This requires a configuration created by [`Self::Spawn`]. Readiness is
    /// bounded by [`Self::timeout`]. No hard TTL is passed by default; an
    /// explicit `server_hard_ttl` value is passed to the child.
    /// `server_idle_ttl` independently selects the operation-idle
    /// lifetime; otherwise `PocketIC` retains its own idle default.
    /// Readiness requires a nonzero decimal port followed by a newline in a
    /// regular UTF-8 file of at most 64 bytes; oversized files fail with bounded
    /// diagnostics. Non-regular port files fail with [`PocketIcStartupFailure::Io`]
    /// and [`io::ErrorKind::InvalidData`] without waiting for a FIFO writer.
    /// The returned handle terminates the child on drop; use its URL with
    /// [`Self::Connect`] to construct bounded instances.
    ///
    /// # Errors
    /// Rejects invalid configuration and server startup or readiness failures.
    pub fn start_managed_server(self) -> Result<PocketIcManagedServer, PocketIcStartupError> {
        if matches!(self, Self::Connect { .. }) {
            self.validate()?;
            return Err(PocketIcStartupError::new(
                PocketIcStartupFailure::InvalidConfiguration {
                    message: "starting a managed PocketIC server requires a spawn configuration"
                        .to_owned(),
                },
            ));
        }
        let startup = StartupDeadline::new(&self)?;
        let (server, url) = self.spawn_server(startup.started)?.wait_ready(&startup)?;
        Ok(PocketIcManagedServer { server, url })
    }

    // Only validated spawn configurations reach this projection. Keeping it here
    // gives instance and server-only construction the same spawn settings.
    fn spawn_server(self, started: Instant) -> Result<ManagedServer, PocketIcStartupError> {
        let Self::Spawn {
            server_binary,
            server_hard_ttl,
            server_idle_ttl,
            server_output_files,
            ..
        } = self
        else {
            unreachable!("connect configurations do not spawn a server")
        };
        ManagedServer::spawn(
            server_binary,
            server_hard_ttl,
            server_idle_ttl,
            server_output_files,
            started,
        )
    }

    /// Run a command with `IC_TESTKIT_POCKET_IC_URL` set to this server.
    ///
    /// Spawn mode retains the managed server until command completion; connect
    /// mode borrows the external server and never terminates it. The command's
    /// IO and other environment selections remain caller-owned. Cancellation
    /// is polled after bounded startup and every 20 ms while the command runs.
    /// It returns an [`io::ErrorKind::Interrupted`] error after cleanup.
    /// If the owned server exits while the command is pending, the command is
    /// terminated and the server's status and bounded diagnostics are returned
    /// as [`PocketIcStartupFailure::ServerExited`]. External servers are not monitored.
    ///
    /// On Unix the command starts in a new owned process group. Completion,
    /// cancellation and observation failures terminate remaining group members
    /// before reaping the leader, using the same lifecycle engine as managed
    /// servers. This does not impose a command deadline or sandbox descendants
    /// that deliberately leave the owned group. Other hosts own the direct child.
    ///
    /// # Errors
    /// Returns startup, command execution, cancellation or server cleanup failures.
    pub fn run_command(
        self,
        command: &mut Command,
        mut cancelled: impl FnMut() -> bool,
    ) -> Result<ExitStatus, PocketIcStartupError> {
        self.validate()?;
        let program = PathBuf::from(command.get_program());
        let command_error = |source| {
            PocketIcStartupError::new(PocketIcStartupFailure::CommandRun {
                program: program.clone(),
                source,
            })
        };
        if cancelled() {
            return Err(command_error(io::Error::from(io::ErrorKind::Interrupted)));
        }
        let (server, url) = match self {
            Self::Connect { server_url, .. } => (None, server_url),
            spawn @ Self::Spawn { .. } => {
                let server = spawn.start_managed_server()?;
                let url = server.url().to_owned();
                (Some(server.server), url)
            }
        };
        let mut server = server;
        if cancelled() {
            return Err(finalize_failure(
                command_error(io::Error::from(io::ErrorKind::Interrupted)),
                None,
                server,
            ));
        }
        command.env("IC_TESTKIT_POCKET_IC_URL", url);
        let mut owned_child = match OwnedChild::spawn(command, CleanupPolicy::KillAndWait) {
            Ok(child) => child,
            Err(source) => {
                return Err(finalize_failure(
                    PocketIcStartupError::new(PocketIcStartupFailure::Io {
                        operation: "spawn command with PocketIC server",
                        path: PathBuf::from(command.get_program()),
                        source,
                    }),
                    None,
                    server,
                ));
            }
        };
        let result = loop {
            if cancelled() {
                break Err(command_error(io::Error::from(io::ErrorKind::Interrupted)));
            }
            match owned_child.try_wait() {
                Ok(Some(status)) => break Ok(status),
                Ok(None) => {
                    if let Some(managed) = server.as_mut()
                        && let Err(error) = managed.check_running()
                    {
                        break Err(error);
                    }
                    thread::sleep(STARTUP_POLL_INTERVAL);
                }
                Err(source) => break Err(command_error(source)),
            }
        };
        match result {
            Err(error) => Err(finalize_failure(error, Some(&mut owned_child), server)),
            Ok(status) => {
                if let Some(server) = server {
                    let (output, cleanup) = server.terminate_and_capture();
                    if let Some(cleanup) = cleanup {
                        return Err(PocketIcStartupError::new(
                            PocketIcStartupFailure::ServerCleanup {
                                command_status: status,
                            },
                        )
                        .with_cleanup(None, Some((output, Some(cleanup)))));
                    }
                }
                Ok(status)
            }
        }
    }

    fn validate(&self) -> Result<(), PocketIcStartupError> {
        if self.timeout().is_zero() {
            return Err(PocketIcStartupError::new(
                PocketIcStartupFailure::InvalidConfiguration {
                    message: "PocketIC startup timeout must be greater than zero".to_owned(),
                },
            ));
        }
        if let Self::Spawn {
            server_hard_ttl,
            server_idle_ttl,
            ..
        } = self
        {
            for (ttl, message) in [
                (
                    server_hard_ttl,
                    "PocketIC server hard TTL must be at least one second",
                ),
                (
                    server_idle_ttl,
                    "PocketIC server idle TTL must be at least one second",
                ),
            ] {
                if ttl.is_some_and(|ttl| ttl.as_secs() == 0) {
                    return Err(PocketIcStartupError::new(
                        PocketIcStartupFailure::InvalidConfiguration {
                            message: message.to_owned(),
                        },
                    ));
                }
            }
        }
        Ok(())
    }
}

impl PocketIcManagedServer {
    /// OS process ID of the owned server child, for caller-managed monitoring.
    ///
    /// This identifies the server process, not its descendants, and does not
    /// establish that it is still running. The OS may reuse the ID after the
    /// child exits and is reaped. Dropping this handle terminates and waits for
    /// the child; retaining the ID does not retain server ownership.
    #[must_use]
    pub fn process_id(&self) -> u32 {
        self.server.child.id()
    }

    /// Loopback URL published by the managed server.
    #[must_use]
    pub fn url(&self) -> &str {
        &self.url
    }

    /// Current bounded stdout and stderr captured from the managed server.
    ///
    /// This reads at most the first 16 KiB from each retained output file,
    /// renders it as lossy UTF-8, and adds an omitted-byte suffix when truncated.
    /// The diagnostic read is bounded; files can grow while the server runs.
    #[must_use]
    pub fn output(&self) -> PocketIcManagedServerOutput {
        self.server.files.capture()
    }
}

impl PocketIcManagedServerOutput {
    /// Bounded lossy UTF-8 standard output.
    #[must_use]
    pub fn stdout(&self) -> &str {
        &self.stdout
    }

    /// Bounded lossy UTF-8 standard error.
    #[must_use]
    pub fn stderr(&self) -> &str {
        &self.stderr
    }
}

struct StartupDeadline {
    started: Instant,
    timeout: Duration,
    deadline: Instant,
}

impl StartupDeadline {
    fn new(config: &PocketIcStartupConfig) -> Result<Self, PocketIcStartupError> {
        config.validate()?;
        let started = Instant::now();
        let timeout = config.timeout();
        Ok(Self {
            started,
            timeout,
            deadline: startup_deadline(started, timeout)?,
        })
    }

    fn wait_interval(&self, now: Instant, monitor_server: bool) -> Duration {
        let remaining = self.deadline.saturating_duration_since(now);
        if monitor_server {
            remaining.min(STARTUP_POLL_INTERVAL)
        } else {
            remaining
        }
    }
}

fn startup_deadline(started: Instant, timeout: Duration) -> Result<Instant, PocketIcStartupError> {
    started.checked_add(timeout).ok_or_else(|| {
        PocketIcStartupError::new(PocketIcStartupFailure::InvalidConfiguration {
            message: "PocketIC startup timeout exceeds the platform clock range".to_owned(),
        })
    })
}

/// Instance construction owns server custody until admission or failure.
struct InstanceStartup<'a> {
    deadline: &'a StartupDeadline,
    server_url: &'a str,
    server: Option<ManagedServer>,
}

impl InstanceStartup<'_> {
    fn check_deadline(&mut self) -> Result<Instant, PocketIcStartupError> {
        let now = Instant::now();
        if now >= self.deadline.deadline {
            return Err(finalize_failure(
                PocketIcStartupError::new(PocketIcStartupFailure::InstanceCreationTimeout {
                    timeout: self.deadline.timeout,
                }),
                None,
                self.server.take(),
            ));
        }
        Ok(now)
    }

    fn admit<T>(&mut self, instance: T) -> Result<PocketIcInstance<T>, PocketIcStartupError> {
        self.check_deadline()?;
        check_server(&mut self.server)?;
        Ok(PocketIcInstance {
            instance,
            server: self.server.take().map(|server| PocketIcManagedServer {
                server,
                url: self.server_url.to_owned(),
            }),
        })
    }

    fn fail(&mut self, error: PocketIcStartupError) -> PocketIcStartupError {
        finalize_failure(error, None, self.server.take())
    }
}

fn parse_server_url(server_url: &str) -> Result<reqwest::Url, PocketIcStartupError> {
    server_url.parse::<reqwest::Url>().map_err(|error| {
        PocketIcStartupError::new(PocketIcStartupFailure::InvalidServerUrl {
            server_url: server_url.to_owned(),
            message: error.to_string(),
        })
    })
}

fn check_server(server: &mut Option<ManagedServer>) -> Result<(), PocketIcStartupError> {
    if let Some(managed) = server.as_mut()
        && let Err(error) = managed.check_running()
    {
        return Err(finalize_failure(error, None, server.take()));
    }
    Ok(())
}

async fn build_bounded(
    builder: PocketIcBuilder,
    server_url: &str,
    startup: &StartupDeadline,
    server: Option<ManagedServer>,
) -> Result<PocketIcInstance<PocketIc>, PocketIcStartupError> {
    let mut startup = InstanceStartup {
        deadline: startup,
        server_url,
        server,
    };
    let builder = parse_server_url(server_url)
        .map(|url| builder.with_server_url(url))
        .map_err(|error| startup.fail(error))?;
    let mut construction = std::pin::pin!(builder.build());
    // Drop this local before the pending upstream future on cancellation.
    let mut startup = startup;
    loop {
        let now = startup.check_deadline()?;
        check_server(&mut startup.server)?;
        let next_check = now
            + startup
                .deadline
                .wait_interval(now, startup.server.is_some());
        match tokio::time::timeout_at(next_check.into(), construction.as_mut()).await {
            Ok(Ok(instance)) => {
                return startup.admit(instance);
            }
            Ok(Err(source)) => {
                return Err(startup.fail(PocketIcStartupError::new(
                    PocketIcStartupFailure::BuilderFailed { source },
                )));
            }
            Err(_) => {}
        }
    }
}

/// Finalize an owned operation once, preserving the primary failure.
fn finalize_failure(
    error: PocketIcStartupError,
    command: Option<&mut OwnedChild>,
    server: Option<ManagedServer>,
) -> PocketIcStartupError {
    error.with_cleanup(
        command.and_then(|child| child.terminate().err()),
        server.map(ManagedServer::terminate_and_capture),
    )
}

struct ManagedServer {
    child: OwnedChild,
    binary: PathBuf,
    files: StartupFiles,
    started: Instant,
}

enum PortFileState {
    Pending,
    Ready(u16),
    Invalid(String),
}

impl ManagedServer {
    fn wait_ready(
        mut self,
        startup: &StartupDeadline,
    ) -> Result<(Self, String), PocketIcStartupError> {
        loop {
            match self.poll_ready(startup) {
                Ok(Some(url)) => return Ok((self, url)),
                Ok(None) => thread::sleep(startup.wait_interval(Instant::now(), true)),
                Err(error) => return Err(finalize_failure(error, None, Some(self))),
            }
        }
    }

    fn spawn(
        binary: PathBuf,
        hard_ttl: Option<Duration>,
        idle_ttl: Option<Duration>,
        output_files: Option<(PathBuf, PathBuf)>,
        started: Instant,
    ) -> Result<Self, PocketIcStartupError> {
        let (files, stdout, stderr) = StartupFiles::create(output_files)?;
        let mut command = Command::new(&binary);
        if let Some(hard_ttl) = hard_ttl {
            command
                .arg("--hard-ttl")
                .arg(hard_ttl.as_secs().to_string());
        }
        if let Some(idle_ttl) = idle_ttl {
            command.arg("--ttl").arg(idle_ttl.as_secs().to_string());
        }
        command
            .arg("--port-file")
            .arg(&files.port)
            .stdout(Stdio::from(stdout))
            .stderr(Stdio::from(stderr));
        let child =
            OwnedChild::spawn(&mut command, CleanupPolicy::KillAndWait).map_err(|source| {
                PocketIcStartupError::new(PocketIcStartupFailure::ServerSpawn {
                    server_binary: binary.clone(),
                    source,
                })
            })?;
        Ok(Self {
            child,
            binary,
            files,
            started,
        })
    }

    fn poll_ready(
        &mut self,
        startup: &StartupDeadline,
    ) -> Result<Option<String>, PocketIcStartupError> {
        self.check_running()?;
        if Instant::now() >= startup.deadline {
            return Err(PocketIcStartupError::new(
                PocketIcStartupFailure::ReadinessTimeout {
                    server_binary: self.binary.clone(),
                    timeout: startup.timeout,
                },
            ));
        }
        match self.files.read_port()? {
            PortFileState::Pending => Ok(None),
            PortFileState::Ready(port) => Ok(Some(format!("http://127.0.0.1:{port}/"))),
            PortFileState::Invalid(value) => Err(PocketIcStartupError::new(
                PocketIcStartupFailure::InvalidServerPort {
                    server_binary: self.binary.clone(),
                    value,
                },
            )),
        }
    }

    fn check_running(&mut self) -> Result<(), PocketIcStartupError> {
        if let Some(status) = self.try_wait()? {
            return Err(self.exit_failure(status));
        }
        Ok(())
    }

    fn try_wait(&mut self) -> Result<Option<ExitStatus>, PocketIcStartupError> {
        self.child.try_wait().map_err(|source| {
            PocketIcStartupError::new(PocketIcStartupFailure::Io {
                operation: "inspect PocketIC server child",
                path: self.binary.clone(),
                source,
            })
        })
    }

    fn exit_failure(&self, status: ExitStatus) -> PocketIcStartupError {
        PocketIcStartupError::new(PocketIcStartupFailure::ServerExited {
            server_binary: self.binary.clone(),
            status,
            elapsed: self.started.elapsed(),
        })
    }

    fn terminate_and_capture(self) -> (PocketIcManagedServerOutput, Option<CleanupError>) {
        let Self {
            mut child, files, ..
        } = self;
        let cleanup = child.terminate().err();
        // Finish the child's Drop cleanup before reading the final diagnostics.
        drop(child);
        (files.capture(), cleanup)
    }
}

struct StartupFiles {
    #[cfg_attr(
        not(test),
        expect(
            dead_code,
            reason = "retains directory ownership until cleanup or drop"
        )
    )]
    directory: OwnedDirectory,
    port: PathBuf,
    stdout: PathBuf,
    stderr: PathBuf,
}

impl StartupFiles {
    fn read_port(&self) -> Result<PortFileState, PocketIcStartupError> {
        let port_path = &self.port;
        let contents =
            match ic_host_fs::read::read_file_prefix(port_path, SERVER_PORT_FILE_LIMIT + 1)
                .map_err(|error| match error {
                    ic_host_artifacts::artifact::ArtifactError::Io(source) => source,
                    error => io::Error::new(io::ErrorKind::InvalidData, error),
                })
                .and_then(|(bytes, _)| {
                    String::from_utf8(bytes)
                        .map_err(|error| io::Error::new(io::ErrorKind::InvalidData, error))
                }) {
                Ok(contents) => contents,
                Err(error) if error.kind() == io::ErrorKind::NotFound => {
                    return Ok(PortFileState::Pending);
                }
                Err(source) => {
                    return Err(PocketIcStartupError::new(PocketIcStartupFailure::Io {
                        operation: "read PocketIC server port file",
                        path: port_path.clone(),
                        source,
                    }));
                }
            };
        if contents.len() > SERVER_PORT_FILE_LIMIT {
            return Ok(PortFileState::Invalid(format!(
                "{} (port file exceeds {SERVER_PORT_FILE_LIMIT} bytes)",
                contents.trim()
            )));
        }
        if !contents.contains('\n') {
            return Ok(PortFileState::Pending);
        }
        let value = contents.trim();
        match value.parse::<u16>() {
            Ok(port) if port != 0 => Ok(PortFileState::Ready(port)),
            _ => Ok(PortFileState::Invalid(value.to_owned())),
        }
    }

    fn capture(&self) -> PocketIcManagedServerOutput {
        PocketIcManagedServerOutput {
            stdout: read_bounded_lossy(&self.stdout),
            stderr: read_bounded_lossy(&self.stderr),
        }
    }

    fn create(
        output_files: Option<(PathBuf, PathBuf)>,
    ) -> Result<(Self, File, File), PocketIcStartupError> {
        let output_files = output_files
            .map(|(stdout, stderr)| {
                Ok::<_, PocketIcStartupError>((
                    std::path::absolute(&stdout)
                        .map_err(|source| startup_file_error("resolve", &stdout, source))?,
                    std::path::absolute(&stderr)
                        .map_err(|source| startup_file_error("resolve", &stderr, source))?,
                ))
            })
            .transpose()?;
        let directory = OwnedDirectory::new(
            create_unique_directory(
                &std::env::temp_dir(),
                "ic-testkit-pocket-ic-startup",
                0o700,
                None,
            )
            .map_err(|source| startup_file_error("create", &std::env::temp_dir(), source))?,
        );
        let base = directory.path();
        let (stdout_path, stderr_path) =
            output_files.unwrap_or_else(|| (base.join("stdout"), base.join("stderr")));
        let files = Self {
            port: base.join("port"),
            stdout: stdout_path,
            stderr: stderr_path,
            directory,
        };
        let stdout = create_new_file(&files.stdout)
            .map_err(|source| startup_file_error("create", &files.stdout, source))?;
        let stderr = create_new_file(&files.stderr)
            .map_err(|source| startup_file_error("create", &files.stderr, source))?;
        Ok((files, stdout, stderr))
    }
}

fn create_new_file(path: &Path) -> io::Result<File> {
    let mut options = OpenOptions::new();
    options.write(true).create_new(true);
    #[cfg(unix)]
    options.mode(0o600);
    options.open(path)
}

fn startup_file_error(
    operation: &'static str,
    path: &Path,
    source: io::Error,
) -> PocketIcStartupError {
    PocketIcStartupError::new(PocketIcStartupFailure::Io {
        operation,
        path: path.to_owned(),
        source,
    })
}

fn read_bounded_lossy(path: &Path) -> String {
    let Ok((bytes, length)) = ic_host_fs::read::read_file_prefix(path, SERVER_OUTPUT_LIMIT) else {
        return String::new();
    };
    let mut output = String::from_utf8_lossy(&bytes).into_owned();
    let omitted = length.saturating_sub(bytes.len() as u64);
    if omitted > 0 {
        let _ = write!(output, "\n<truncated {omitted} bytes>");
    }
    output
}

impl std::fmt::Display for PocketIcStartupError {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        std::fmt::Display::fmt(self.failure.as_ref(), formatter)?;
        if !self.output.stdout.is_empty() {
            write!(formatter, "; server stdout: {}", self.output.stdout)?;
        }
        if !self.output.stderr.is_empty() {
            write!(formatter, "; server stderr: {}", self.output.stderr)?;
        }
        for (owner, cleanup) in [
            ("command", self.command_cleanup()),
            ("server", self.server_cleanup()),
        ] {
            if let Some(error) = cleanup {
                write!(formatter, "; {owner} cleanup also failed: {error}")?;
            }
        }
        Ok(())
    }
}

impl std::error::Error for PocketIcStartupError {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        std::error::Error::source(self.failure.as_ref())
    }
}

#[cfg(test)]
mod tests;
