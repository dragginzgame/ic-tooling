//! Private fixture benchmark programs for native Linux and macOS hosts.
//! Builds and measures the existing workload; library APIs own no benchmark policy.

mod fixture_reuse_driver;
mod fixture_reuse_protocol;
mod fixture_reuse_report;
mod statistics;
mod workload;

use std::process::ExitCode;

use fixture_reuse_driver::Error;

fn clap_exit(error: &clap::Error) -> ExitCode {
    let _ = error.print();
    ExitCode::from(u8::try_from(error.exit_code()).unwrap_or(2))
}

/// Run the operator-facing benchmark driver and project its exit status.
#[must_use]
pub fn driver_main() -> ExitCode {
    match fixture_reuse_driver::run() {
        Ok(()) => ExitCode::SUCCESS,
        Err(Error::Arguments(error)) => clap_exit(&error),
        Err(error) => {
            eprintln!("error: {error}");
            match error {
                Error::InvalidArgument(_) => ExitCode::from(2),
                Error::Interrupted => ExitCode::from(130),
                Error::Tool(error)
                    if matches!(&error, ic_host_process::tool::ToolError::Execution(execution)
                        if matches!(execution.failure, ic_host_process::tool::ExecutionFailure::Cancelled)) =>
                {
                    ExitCode::from(130)
                }
                _ => ExitCode::FAILURE,
            }
        }
    }
}

/// Run the private benchmark worker with Clap-owned argument diagnostics.
#[must_use]
pub fn worker_main() -> ExitCode {
    let arguments = match fixture_reuse_protocol::WorkerArguments::try_from(
        std::env::args_os().skip(1).collect::<Vec<_>>(),
    ) {
        Ok(arguments) => arguments,
        Err(error) => return clap_exit(&error),
    };
    match workload::run(arguments) {
        Ok(()) => ExitCode::SUCCESS,
        Err(error) => {
            eprintln!("error: {error}");
            ExitCode::FAILURE
        }
    }
}

/// Project the private probe's explicit native frame contract for `PocketIC` calls.
#[must_use]
pub const fn probe_wire_spec(
    method: ic_testkit_perf_probe::protocol::Method,
) -> ic_testkit::pic::WireCallSpec {
    ic_testkit::pic::WireCallSpec {
        method: method.name,
        method_id: method.id,
        request_limits: method.request_limits,
        response_limits: method.response_limits,
    }
}
