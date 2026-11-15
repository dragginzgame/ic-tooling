//! One synchronous owned group, with bounded fair pipe draining and explicit close.

use std::io::{self, Read, Write};
use std::os::unix::process::CommandExt;
use std::process::{Child, Command, ExitStatus, Stdio};
use std::sync::atomic::{AtomicBool, Ordering};
use std::thread;
use std::time::{Duration, Instant};

use rustix::fs::{OFlags, fcntl_getfl, fcntl_setfl};
use rustix::process::{Pid, Signal, WaitId, WaitIdOptions, kill_process_group, waitid};

use crate::{Failure, Limits};

pub(crate) struct Report {
    pub(crate) status: Option<ExitStatus>,
    pub(crate) failure: Option<Failure>,
    pub(crate) cleanup: Vec<io::Error>,
    pub(crate) diagnostics: Vec<u8>,
}

struct Session {
    child: Option<Child>,
}

impl Session {
    // Do not reap until group signalling ends: the unreaped leader pins the ID.
    fn close(&mut self) -> (Option<ExitStatus>, Vec<io::Error>) {
        let Some(mut child) = self.child.take() else {
            return (None, Vec::new());
        };
        let mut errors = Vec::new();
        if let Err(error) = kill_process_group(Pid::from_child(&child), Signal::KILL)
            && error != rustix::io::Errno::SRCH
        {
            errors.push(error.into());
            // Preserve the group failure even when direct-child fallback succeeds.
            if let Err(error) = child.kill() {
                errors.push(error);
            }
        }
        let status = match child.wait() {
            Ok(status) => Some(status),
            Err(error) => {
                errors.push(error);
                None
            }
        };
        (status, errors)
    }
}

impl Drop for Session {
    fn drop(&mut self) {
        let _ = self.close();
    }
}

pub(crate) fn run(
    command: &mut Command,
    limits: &Limits,
    cancelled: &AtomicBool,
    sink: &mut dyn Write,
    mut stdout_chunk: impl FnMut(&[u8]),
) -> Report {
    let mut report = Report {
        status: None,
        failure: None,
        cleanup: Vec::new(),
        diagnostics: Vec::new(),
    };
    command
        .stdin(Stdio::null())
        .stdout(Stdio::piped())
        .stderr(Stdio::piped())
        .process_group(0);
    let child = match command.spawn() {
        Ok(child) => child,
        Err(error) => {
            report.failure = Some(Failure::Io(error));
            return report;
        }
    };
    let mut session = Session { child: Some(child) };
    let result = communicate(
        &mut session,
        limits,
        cancelled,
        sink,
        &mut stdout_chunk,
        &mut report.diagnostics,
    );
    report.failure = result.err();
    (report.status, report.cleanup) = session.close();
    report
}

fn communicate(
    session: &mut Session,
    limits: &Limits,
    cancelled: &AtomicBool,
    sink: &mut dyn Write,
    stdout_chunk: &mut impl FnMut(&[u8]),
    diagnostics: &mut Vec<u8>,
) -> Result<(), Failure> {
    let child = session
        .child
        .as_mut()
        .ok_or(Failure::InvalidRequest("missing process owner"))?;
    let mut stdout = child
        .stdout
        .take()
        .ok_or(Failure::InvalidRequest("missing stdout"))?;
    let mut stderr = child
        .stderr
        .take()
        .ok_or(Failure::InvalidRequest("missing stderr"))?;
    fcntl_setfl(
        &stdout,
        fcntl_getfl(&stdout)
            .map_err(io::Error::from)
            .map_err(Failure::Io)?
            | OFlags::NONBLOCK,
    )
    .map_err(io::Error::from)
    .map_err(Failure::Io)?;
    fcntl_setfl(
        &stderr,
        fcntl_getfl(&stderr)
            .map_err(io::Error::from)
            .map_err(Failure::Io)?
            | OFlags::NONBLOCK,
    )
    .map_err(io::Error::from)
    .map_err(Failure::Io)?;
    let started = Instant::now();
    let mut stdout_eof = false;
    let mut stderr_eof = false;
    let mut stdout_count = 0_usize;
    let mut stderr_count = 0_usize;
    let mut buffer = [0_u8; 8192];
    loop {
        if cancelled.load(Ordering::Relaxed) {
            return Err(Failure::Cancelled);
        }
        if started.elapsed() >= limits.elapsed {
            return Err(Failure::Deadline);
        }
        let mut progress = false;
        if !stdout_eof
            && let Some(count) = read_chunk(&mut stdout, &mut buffer).map_err(Failure::Io)?
        {
            progress = count > 0;
            stdout_eof = count == 0;
            stdout_count = stdout_count
                .checked_add(count)
                .ok_or(Failure::OutputLimit)?;
            if stdout_count > limits.stdout_bytes {
                return Err(Failure::OutputLimit);
            }
            stdout_chunk(&buffer[..count]);
        }
        if !stderr_eof
            && let Some(count) = read_chunk(&mut stderr, &mut buffer).map_err(Failure::Io)?
        {
            progress |= count > 0;
            stderr_eof = count == 0;
            stderr_count = stderr_count
                .checked_add(count)
                .ok_or(Failure::DiagnosticLimit)?;
            if stderr_count > limits.stderr_bytes {
                return Err(Failure::DiagnosticLimit);
            }
            diagnostics
                .try_reserve(count)
                .map_err(|_| Failure::Allocation)?;
            diagnostics.extend_from_slice(&buffer[..count]);
            sink.write_all(&buffer[..count]).map_err(Failure::Sink)?;
        }
        let exited = match waitid(
            WaitId::Pid(Pid::from_child(child)),
            WaitIdOptions::EXITED | WaitIdOptions::NOHANG | WaitIdOptions::NOWAIT,
        ) {
            Ok(status) => status.is_some(),
            Err(error) if error == rustix::io::Errno::INTR => false,
            Err(error) => return Err(Failure::Io(error.into())),
        };
        if exited && stdout_eof && stderr_eof {
            sink.flush().map_err(Failure::Sink)?;
            return Ok(());
        }
        if !progress {
            thread::sleep(Duration::from_millis(2));
        }
    }
}

fn read_chunk(reader: &mut impl Read, buffer: &mut [u8]) -> io::Result<Option<usize>> {
    match reader.read(buffer) {
        Ok(count) => Ok(Some(count)),
        Err(error)
            if matches!(
                error.kind(),
                io::ErrorKind::WouldBlock | io::ErrorKind::Interrupted
            ) =>
        {
            Ok(None)
        }
        Err(error) => Err(error),
    }
}

#[cfg(test)]
mod tests;
