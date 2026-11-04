use std::{
    ffi::OsString,
    path::PathBuf,
    process::{Command, ExitCode},
    sync::atomic::{AtomicI32, Ordering},
    time::Duration,
};

use clap::{CommandFactory, Parser, builder::TypedValueParser as _, error::ErrorKind};

use ic_host_process::{
    signal::{SignalActionError, SignalActions},
    status::shell_status_code,
};
use ic_testkit::pic::{PocketIcStartupConfig, server_assets};

static INTERRUPTED: AtomicI32 = AtomicI32::new(0);

#[derive(Parser)]
#[command(
    name = "ic-testkit-server",
    about = "Explicit PocketIC setup, offline admission and command-scoped ownership",
    version,
    arg_required_else_help = true,
    no_binary_name = true
)]
enum Arguments {
    Setup {
        #[arg(long, value_name = "DIRECTORY", default_value = server_assets::DEFAULT_DIRECTORY, allow_hyphen_values = true)]
        directory: PathBuf,
    },
    Check {
        #[arg(long, value_name = "DIRECTORY", default_value = server_assets::DEFAULT_DIRECTORY, allow_hyphen_values = true)]
        directory: PathBuf,
    },
    Run(Box<RunArguments>),
}

#[derive(clap::Args)]
struct RunArguments {
    #[arg(long, value_name = "DIRECTORY", allow_hyphen_values = true)]
    directory: Option<PathBuf>,
    #[arg(long, value_name = "SECONDS", value_parser = clap::value_parser!(u64).range(1..).map(Duration::from_secs))]
    ttl: Option<Duration>,
    #[arg(long, value_name = "SECONDS", value_parser = clap::value_parser!(u64).range(1..).map(Duration::from_secs))]
    idle_ttl: Option<Duration>,
    #[arg(long = "startup-timeout", value_name = "SECONDS", default_value = "30", value_parser = clap::value_parser!(u64).range(1..).map(Duration::from_secs))]
    timeout: Duration,
    #[arg(
        long,
        value_name = "NEW-FILE",
        allow_hyphen_values = true,
        requires = "server_stderr"
    )]
    server_stdout: Option<PathBuf>,
    #[arg(
        long,
        value_name = "NEW-FILE",
        allow_hyphen_values = true,
        requires = "server_stdout"
    )]
    server_stderr: Option<PathBuf>,
    #[arg(last = true, required = true, num_args = 1.., value_name = "COMMAND")]
    command: Vec<OsString>,
}

extern "C" fn interrupted(signal: libc::c_int) {
    INTERRUPTED.store(signal, Ordering::Relaxed);
}

#[expect(
    unsafe_code,
    reason = "bounded native operation with the local SAFETY contract below"
)]
fn install_signals() -> Result<SignalActions, SignalActionError> {
    // SAFETY: this standalone CLI exclusively owns these dispositions through
    // guard drop. The static handler only stores a lock-free atomic value.
    unsafe { SignalActions::install(&[libc::SIGINT, libc::SIGTERM, libc::SIGHUP], interrupted) }
}

fn parse(arguments: impl Iterator<Item = OsString>) -> Result<Arguments, clap::Error> {
    let arguments = Arguments::try_parse_from(arguments)?;
    // Only the executable, rather than every child argument, must be nonempty.
    if let Arguments::Run(run) = &arguments
        && run.command[0].is_empty()
    {
        return Err(Arguments::command().error(
            ErrorKind::ValueValidation,
            "command executable must not be empty",
        ));
    }
    Ok(arguments)
}

pub fn main() -> ExitCode {
    let arguments = match parse(std::env::args_os().skip(1)) {
        Ok(arguments) => arguments,
        Err(error) => {
            let _ = error.print();
            return ExitCode::from(u8::try_from(error.exit_code()).unwrap_or(2));
        }
    };
    let arguments = match arguments {
        Arguments::Setup { directory } => {
            return match install_signals() {
                Ok(_signals) => report_provisioning(server_assets::setup(&directory, || {
                    INTERRUPTED.load(Ordering::Relaxed) != 0
                })),
                Err(error) => report_provisioning(Err(error.into())),
            };
        }
        Arguments::Check { directory } => {
            return report_provisioning(server_assets::check(&directory));
        }
        Arguments::Run(arguments) => arguments,
    };
    let RunArguments {
        directory,
        timeout,
        ttl,
        idle_ttl,
        server_stdout,
        server_stderr,
        command,
    } = *arguments;
    let output_files = server_stdout.zip(server_stderr);
    let mut values = command.into_iter();
    let mut command = Command::new(values.next().expect("Clap requires a child command"));
    command.args(values);
    let result = (|| {
        let _signals = install_signals()?;
        let environment_selected = std::env::var_os("IC_TESTKIT_POCKET_IC_URL").is_some()
            || std::env::var_os("POCKET_IC_BIN").is_some();
        let mut config = match (directory, environment_selected) {
            (Some(_), true) => {
                return Err(
                    "--directory conflicts with an explicit server URL or POCKET_IC_BIN".into(),
                );
            }
            (None, true) => PocketIcStartupConfig::from_env(timeout)?,
            (directory, false) => {
                let binary = server_assets::check(
                    &directory.unwrap_or_else(|| PathBuf::from(server_assets::DEFAULT_DIRECTORY)),
                )?;
                command.env("POCKET_IC_BIN", &binary);
                PocketIcStartupConfig::Spawn {
                    server_binary: binary,
                    timeout,
                    server_hard_ttl: None,
                    server_idle_ttl: None,
                    server_output_files: None,
                }
            }
        };
        match &mut config {
            PocketIcStartupConfig::Spawn {
                server_hard_ttl,
                server_idle_ttl,
                server_output_files,
                ..
            } => {
                *server_hard_ttl = ttl;
                *server_idle_ttl = idle_ttl;
                *server_output_files = output_files;
            }
            PocketIcStartupConfig::Connect { .. } => {
                if ttl.is_some() || idle_ttl.is_some() || output_files.is_some() {
                    return Err("--ttl, --idle-ttl and output files require an owned server".into());
                }
            }
        }
        Ok::<_, Box<dyn std::error::Error>>(
            config.run_command(&mut command, || INTERRUPTED.load(Ordering::Relaxed) != 0)?,
        )
    })();
    if let Err(error) = &result {
        eprintln!("{error}");
    }
    if let Some(status) = interruption_exit_code() {
        return status;
    }
    match result {
        Ok(status) => command_exit_code(status),
        Err(error) => error
            .downcast_ref::<ic_testkit::pic::PocketIcStartupError>()
            .map_or(ExitCode::FAILURE, |startup| {
                failure_exit_code(startup.failure())
            }),
    }
}

fn interruption_exit_code() -> Option<ExitCode> {
    let signal = INTERRUPTED.load(Ordering::Relaxed);
    (signal != 0).then(|| ExitCode::from(u8::try_from(128 + signal).unwrap_or(1)))
}

fn report_provisioning(result: Result<PathBuf, Box<dyn std::error::Error>>) -> ExitCode {
    use std::io::Write as _;
    use std::os::unix::ffi::OsStrExt as _;
    if let Err(error) = &result {
        eprintln!("{error}");
    }
    if let Some(status) = interruption_exit_code() {
        return status;
    }
    result.map_or(ExitCode::FAILURE, |path| {
        let stdout = std::io::stdout();
        let mut stdout = stdout.lock();
        if stdout
            .write_all(path.as_os_str().as_bytes())
            .and_then(|()| stdout.write_all(b"\n"))
            .is_err()
        {
            ExitCode::FAILURE
        } else {
            ExitCode::SUCCESS
        }
    })
}

fn command_exit_code(status: std::process::ExitStatus) -> ExitCode {
    ExitCode::from(u8::try_from(shell_status_code(status).unwrap_or(129)).unwrap_or(1))
}

fn failure_exit_code(failure: &ic_testkit::pic::PocketIcStartupFailure) -> ExitCode {
    if let ic_testkit::pic::PocketIcStartupFailure::ServerCleanup { command_status } = failure
        && !command_status.success()
    {
        command_exit_code(*command_status)
    } else {
        ExitCode::FAILURE
    }
}

#[cfg(test)]
mod tests;
