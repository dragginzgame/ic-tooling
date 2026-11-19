//! Explicit offline assets and bounded owned simulator processes.
//!
//! Enable `managed-server` explicitly. Callers own the Tokio runtime, application
//! fixtures, simulated progress and gateway policy. Process owns native execution.
//! Deletion completion means an upstream attempt, not observed remote cleanup.
#[cfg(test)]
mod tests;

mod assets;

pub use assets::{Asset, PreparedServer, host_asset, prepare};

use std::{
    io,
    net::{SocketAddr, TcpStream},
    os::unix::fs::PermissionsExt,
    process::Command,
    sync::{
        Arc,
        atomic::{AtomicBool, AtomicUsize, Ordering},
    },
    thread::JoinHandle,
    time::{Duration, Instant},
};

use ic_host_process::{Input, Ownership, Policy, Report, Stderr, Stream};
use pocket_ic::{PocketIcBuilder, nonblocking::PocketIc};

/// Failure before an admitted asset/server or an explicit lifecycle refusal.
#[derive(Debug, thiserror::Error)]
pub enum Error {
    /// Original native IO failure.
    #[error("simulator IO: {0}")]
    Io(#[from] io::Error),
    /// Held filesystem admission refused.
    #[error(transparent)]
    Filesystem(#[from] ic_host_fs::Error),
    /// Bounded byte admission refused.
    #[error(transparent)]
    Bytes(#[from] ic_host_artifacts::Error),
    /// Stage write/seal failed, retaining cleanup evidence.
    #[error(transparent)]
    Stage(#[from] ic_host_fs::AttemptFailure),
    /// Publication did not complete; inspect visibility and cleanup separately.
    #[error("asset publication did not complete: {0:?}")]
    Publication(Box<ic_host_fs::Commit>),
    /// No compiled-in asset is selected for this native host.
    #[error("unsupported PocketIC host")]
    UnsupportedHost,
    /// Missing, changed or untrusted prepared bytes.
    #[error("PocketIC asset refused: {0}")]
    Asset(&'static str),
    /// Limits/tool paths must be explicit and usable.
    #[error("invalid simulator limits or tool path")]
    Configuration,
    /// Decompression failed; output is bounded and execution is already closed.
    #[error("asset decompression failed")]
    Decompression(Box<Stopped>),
    /// Startup refused; the execution owner was joined before returning evidence.
    #[error("simulator startup refused: {reason}")]
    Startup {
        /// Startup observation, distinct from process/cleanup failure.
        reason: String,
        /// Original execution and bounded output evidence.
        evidence: Box<Stopped>,
    },
    /// An instance or creation future still retains the server dependency.
    #[error("simulator still has live instance owners")]
    LiveInstances,
    /// Explicit close already consumed the server, or its worker has exited.
    #[error("simulator is stopped")]
    Stopped,
    /// Instance creation timed out; server-side effects may remain.
    #[error("instance creation timed out; remote effects are unknown")]
    CreationTimeout,
    /// Process worker unwound; its Process owner performs fallback cleanup.
    #[error("simulator worker panicked")]
    WorkerPanic,
}

/// Retained bytes; Debug deliberately reports lengths rather than terminal data.
#[derive(Default)]
pub struct Output {
    /// Raw bounded stdout, not safe to print without escaping.
    pub stdout: Vec<u8>,
    /// Raw bounded stderr, not safe to print without escaping.
    pub stderr: Vec<u8>,
}
impl std::fmt::Debug for Output {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("Output")
            .field("stdout_bytes", &self.stdout.len())
            .field("stderr_bytes", &self.stderr.len())
            .finish()
    }
}

/// Joined execution evidence. Explicit stop normally records `Failure::Cancelled`;
/// do not discard unexpected failure, original status or cleanup errors.
#[derive(Debug)]
pub struct Stopped {
    /// Original execution, signalling and reaping evidence.
    pub process: Report,
    /// Bounded raw output.
    pub output: Output,
    /// Creations/deletions whose futures were abandoned or whose deadline expired,
    /// plus instance owners dropped without completing a deletion attempt.
    pub abandoned: usize,
}

/// Explicit process and startup limits; there are no implicit lifetime defaults.
#[derive(Clone, Copy, Debug)]
pub struct Limits {
    /// Readiness deadline, including waiting for the owned port file/listener.
    pub startup: Duration,
    /// Whole process communication deadline; kernel waits are not hard real-time.
    pub lifetime: Duration,
    /// Total admitted stdout before Process terminates on overflow.
    pub stdout_bytes: usize,
    /// Total admitted stderr before Process terminates on overflow.
    pub stderr_bytes: usize,
}

struct Running {
    task: Option<JoinHandle<Stopped>>,
    cancelled: Arc<AtomicBool>,
    abandoned: AtomicUsize,
    // These must outlive execution, including startup failure/unwind.
    _scratch: tempfile::TempDir,
    _prepared: PreparedServer,
}
impl Running {
    fn finished(&self) -> bool {
        self.task.as_ref().is_none_or(JoinHandle::is_finished)
    }

    fn finish(&mut self) -> Result<Stopped, Error> {
        self.cancelled.store(true, Ordering::Relaxed);
        let mut stopped = self
            .task
            .take()
            .ok_or(Error::Stopped)?
            .join()
            .map_err(|_| Error::WorkerPanic)?;
        stopped.abandoned = self.abandoned.load(Ordering::Relaxed);

        Ok(stopped)
    }
}
impl Drop for Running {
    fn drop(&mut self) {
        if self.task.is_some() {
            let _ = self.finish();
        }
    }
}

/// One explicit server owner. Live instances retain its process dependency.
/// Dropping the last owner cancels/joins as fallback; prefer explicit stop evidence.
///
/// # Example
///
/// Start and stop synchronously outside latency-sensitive runtime workers.
/// The caller selects already-prepared bytes and owns the async runtime.
///
/// ```no_run
/// use ic_testkit::{
///     pocket_ic::PocketIcBuilder,
///     server::{Deletion, Limits, PreparedServer, Server},
/// };
/// use std::{path::Path, time::Duration};
///
/// fn run(prepared: &Path) -> Result<(), Box<dyn std::error::Error>> {
///     let mut server = Server::start(
///         PreparedServer::check(prepared)?,
///         Limits {
///             startup: Duration::from_secs(30),
///             lifetime: Duration::from_secs(180),
///             stdout_bytes: 1_048_576,
///             stderr_bytes: 1_048_576,
///         },
///     )?;
///     let runtime = tokio::runtime::Builder::new_current_thread()
///         .enable_all()
///         .build()?;
///     runtime.block_on(async {
///         let timeout = Duration::from_secs(30);
///         let instance = server
///             .instance(PocketIcBuilder::new().with_application_subnet(), timeout)
///             .await?;
///         // Fixtures use instance.client(); gateways use client_mut().
///         assert_eq!(instance.close(timeout).await, Deletion::Attempted);
///         Ok::<_, ic_testkit::server::Error>(())
///     })?;
///     let stopped = server.stop()?;
///     // Assess process failure, bounded output and abandonment in caller policy.
///     // Intentional stop normally records Failure::Cancelled.
///     assert_eq!(stopped.abandoned, 0);
///     assert!(stopped.process.status.is_some() && stopped.process.cleanup.is_empty());
///     Ok(())
/// }
/// ```
pub struct Server {
    running: Option<Arc<Running>>,
    endpoint: String,
}
impl Server {
    /// Reverify prepared bytes and start one loopback server without network setup.
    /// This synchronous operation must not run on a latency-sensitive async worker.
    /// # Errors
    /// Refuses invalid limits/assets, worker creation, early exit, output overflow,
    /// malformed readiness and startup timeout; returns joined execution evidence.
    pub fn start(prepared: PreparedServer, limits: Limits) -> Result<Self, Error> {
        if limits.startup.is_zero()
            || limits.lifetime < limits.startup
            || limits.lifetime < Duration::from_secs(1)
            || limits.stdout_bytes == 0
            || limits.stderr_bytes == 0
        {
            return Err(Error::Configuration);
        }
        prepared.verify()?;
        let scratch = tempfile::Builder::new()
            .prefix("ic-testkit-server-")
            .permissions(std::fs::Permissions::from_mode(0o700))
            .tempdir()?;
        let directory = ic_host_fs::Directory::open(scratch.path(), 0o077)?
            .ok_or(Error::Asset("missing startup directory"))?;
        let mut command = Command::new(prepared.path());
        command
            .args(["--ip-addr", "127.0.0.1", "--port", "0", "--port-file"])
            .arg(scratch.path().join("port"))
            .arg("--ttl")
            .arg(limits.lifetime.as_secs().saturating_add(1).to_string())
            .arg("--hard-ttl")
            .arg(limits.lifetime.as_secs().saturating_add(1).to_string());
        let cancelled = Arc::new(AtomicBool::new(false));
        let worker_cancel = cancelled.clone();
        let task = std::thread::Builder::new()
            .name("ic-testkit-server".into())
            .spawn(move || {
                capture(
                    &mut command,
                    policy(limits.lifetime, limits.stdout_bytes, limits.stderr_bytes),
                    &worker_cancel,
                )
            })?;
        let mut running = Running {
            task: Some(task),
            cancelled,
            abandoned: AtomicUsize::new(0),
            _scratch: scratch,
            _prepared: prepared,
        };
        let started = Instant::now();
        let ready = (|| -> Result<u16, Error> {
            while started.elapsed() < limits.startup {
                if running.finished() {
                    return Err(Error::Stopped);
                }
                if let Some(file) = directory.open_file(
                    "port".as_ref(),
                    ic_host_fs::Access::Read,
                    ic_host_fs::FilePolicy {
                        forbidden: 0,
                        single_link: true,
                    },
                )? {
                    let (bytes, _) =
                        ic_host_artifacts::read(file, ic_host_artifacts::Length::AtMost(6))?;

                    if !bytes.is_empty() {
                        let port: u16 = std::str::from_utf8(&bytes)
                            .ok()
                            .and_then(|text| text.trim().parse().ok())
                            .filter(|port| *port != 0)
                            .ok_or(Error::Asset("malformed server port"))?;
                        let address = SocketAddr::from(([127, 0, 0, 1], port));
                        let remaining = limits.startup.saturating_sub(started.elapsed());

                        if !remaining.is_zero()
                            && TcpStream::connect_timeout(
                                &address,
                                remaining.min(Duration::from_millis(20)),
                            )
                            .is_ok()
                            && !running.finished()
                        {
                            return Ok(port);
                        }
                    }
                }
                std::thread::sleep(Duration::from_millis(2));
            }

            Err(Error::Io(io::Error::new(
                io::ErrorKind::TimedOut,
                "startup deadline",
            )))
        })();

        match ready {
            Ok(port) => Ok(Self {
                running: Some(Arc::new(running)),
                endpoint: format!("http://127.0.0.1:{port}/"),
            }),
            Err(reason) => Err(Error::Startup {
                reason: reason.to_string(),
                evidence: Box::new(running.finish()?),
            }),
        }
    }

    /// Selected loopback URL; it may become unreachable after expiry or stop.
    #[must_use]
    pub fn endpoint(&self) -> &str {
        &self.endpoint
    }

    /// Construct an independent upstream instance on the caller's runtime.
    /// Overrides builder discovery/endpoint and per-request timeout. No progress,
    /// funding, subnet or gateway policy is added.
    /// # Errors
    /// Refuses stopped servers/invalid deadlines or times out with uncertain effects.
    /// # Panics
    /// Upstream creation/transport panic behavior remains unchanged.
    pub async fn instance(
        &self,
        builder: PocketIcBuilder,
        timeout: Duration,
    ) -> Result<Instance, Error> {
        let milliseconds = u64::try_from(timeout.as_millis()).map_err(|_| Error::Configuration)?;
        if milliseconds == 0 {
            return Err(Error::Configuration);
        }
        let running = self.running.as_ref().ok_or(Error::Stopped)?;
        if running.finished() {
            return Err(Error::Stopped);
        }
        let mut instance = Instance {
            client: None,
            running: running.clone(),
            attempted: false,
        };
        let url = self
            .endpoint
            .parse()
            .map_err(|error| io::Error::other(format!("endpoint: {error}")))?;
        instance.client = Some(
            tokio::time::timeout(
                timeout,
                builder
                    .with_server_url(url)
                    .with_max_request_time_ms(Some(milliseconds))
                    .build_async(),
            )
            .await
            .map_err(|_| Error::CreationTimeout)?,
        );

        Ok(instance)
    }

    /// Cancel/join the owned process, only after instance owners have been closed.
    /// A process report preserves cancellation, original exit and cleanup failures.
    /// # Errors
    /// Refuses live instance/creation owners without stopping their server, repeat
    /// stop, or worker panic. Dropped unclosed instances are counted in the report.
    pub fn stop(&mut self) -> Result<Stopped, Error> {
        if Arc::strong_count(self.running.as_ref().ok_or(Error::Stopped)?) != 1 {
            return Err(Error::LiveInstances);
        }
        let running = self.running.take().ok_or(Error::Stopped)?;

        match Arc::try_unwrap(running) {
            Ok(mut owner) => owner.finish(),
            Err(owner) => {
                self.running = Some(owner);

                Err(Error::LiveInstances)
            }
        }
    }
}

/// One original upstream instance retaining its server's lifetime.
pub struct Instance {
    client: Option<PocketIc>,
    running: Arc<Running>,
    attempted: bool,
}
/// Deletion attempt evidence, never an independent observation of remote deletion.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum Deletion {
    /// Upstream deletion future completed; upstream may log rather than return errors.
    Attempted,
    /// Deadline expired; deletion effects remain uncertain.
    TimedOut,
}
impl Instance {
    /// Borrow the unchanged PocketIC client.
    /// # Panics
    /// Only if the private live-instance invariant is violated; consuming close
    /// cannot overlap a safe borrow of this owner.
    #[must_use]
    pub fn client(&self) -> &PocketIc {
        // Public instances always contain a client until consuming close begins.
        self.client.as_ref().expect("live instance")
    }

    /// Borrow the client mutably, including for original localhost gateway methods.
    /// Do not replace/move the client out through this borrow or independently
    /// attach clients whose lifetime exceeds the owner.
    /// # Panics
    /// Only if the private live-instance invariant is violated.
    #[must_use]
    pub fn client_mut(&mut self) -> &mut PocketIc {
        self.client.as_mut().expect("live instance")
    }

    /// Attempt deletion before releasing the server dependency. Cancellation/panic
    /// records abandonment; Drop does not run asynchronous deletion or claim success.
    /// # Panics
    /// Upstream transport/runtime panic behavior remains unchanged.
    pub async fn close(mut self, timeout: Duration) -> Deletion {
        let client = self.client.take().expect("live instance");

        if tokio::time::timeout(timeout, client.drop()).await.is_ok() {
            self.attempted = true;

            Deletion::Attempted
        } else {
            Deletion::TimedOut
        }
    }
}
impl Drop for Instance {
    fn drop(&mut self) {
        if !self.attempted {
            self.running.abandoned.fetch_add(1, Ordering::Relaxed);
        }
    }
}

fn policy(elapsed: Duration, stdout_bytes: usize, stderr_bytes: usize) -> Policy {
    Policy {
        ownership: Ownership::Group,
        input: Input::Null,
        stdout_bytes,
        stderr: Stderr::Capture(stderr_bytes),
        elapsed,
    }
}

fn capture(command: &mut Command, policy: Policy, cancelled: &AtomicBool) -> Stopped {
    let mut output = Output::default();
    let process = ic_host_process::run(command, policy, cancelled, |stream, bytes| {
        let target = match stream {
            Stream::Stdout => &mut output.stdout,
            Stream::Stderr => &mut output.stderr,
        };
        target.try_reserve(bytes.len()).map_err(io::Error::other)?;
        target.extend_from_slice(bytes);

        Ok(())
    });

    Stopped {
        process,
        output,
        abandoned: 0,
    }
}
