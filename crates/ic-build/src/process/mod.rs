//! Cargo's streaming/diagnostic adapter over the shared owned execution core.

#[cfg(test)]
mod tests;

use std::{
    io::{self, Write},
    process::{Command, ExitStatus},
    sync::atomic::AtomicBool,
};

use ic_host_process::{Input, Ownership, Policy, Stderr, Stream};

use crate::{Failure, Limits};

pub(crate) struct Report {
    pub(crate) status: Option<ExitStatus>,
    pub(crate) failure: Option<Failure>,
    pub(crate) cleanup: Vec<io::Error>,
    pub(crate) diagnostics: Vec<u8>,
}

pub(crate) fn run(
    command: &mut Command,
    limits: &Limits,
    cancelled: &AtomicBool,
    sink: &mut dyn Write,
    mut stdout_chunk: impl FnMut(&[u8]),
) -> Report {
    let mut diagnostics = Vec::new();
    let mut allocation_failed = false;
    let report = ic_host_process::run(
        command,
        Policy {
            ownership: Ownership::Group,
            input: Input::Null,
            stdout_bytes: limits.stdout_bytes,
            stderr: Stderr::Capture(limits.stderr_bytes),
            elapsed: limits.elapsed,
        },
        cancelled,
        |stream, bytes| match stream {
            Stream::Stdout => {
                stdout_chunk(bytes);

                Ok(())
            }
            Stream::Stderr => {
                diagnostics.try_reserve(bytes.len()).map_err(|error| {
                    allocation_failed = true;

                    io::Error::other(error)
                })?;
                diagnostics.extend_from_slice(bytes);

                sink.write_all(bytes)
            }
        },
    );
    let failure = report.failure.map(|failure| match failure {
        ic_host_process::Failure::Io(error) => Failure::Io(error),
        ic_host_process::Failure::Cancelled => Failure::Cancelled,
        ic_host_process::Failure::Deadline => Failure::Deadline,
        ic_host_process::Failure::Limit(Stream::Stdout) => Failure::OutputLimit,
        ic_host_process::Failure::Limit(Stream::Stderr) => Failure::DiagnosticLimit,
        ic_host_process::Failure::Observer { cause, .. } => {
            if allocation_failed {
                Failure::Allocation
            } else {
                Failure::Sink(cause)
            }
        }
    });
    // Flushing belongs to the caller's diagnostic sink, not the execution owner.
    let failure = failure.or_else(|| sink.flush().err().map(Failure::Sink));

    Report {
        status: report.status,
        failure,
        cleanup: report.cleanup,
        diagnostics,
    }
}
