//! Owned execution with bounded synchronous delivery and separately reaped launches.
//!
//! Callers prepare commands and own decoding, retention and retry policy. This
//! Unix implementation supports owned groups and direct foreground children;
//! borrowed PIDs never confer signal authority. Background launches retain reaping
//! custody independently of completion observers. No sampling or tool setup is owned.

#[cfg(test)]
mod tests;

mod handoff;
#[cfg(target_os = "macos")]
mod macos;

pub use handoff::{Completion, launch};

use std::{
    io::{self, Read, Write},
    os::{fd::AsFd, unix::process::CommandExt},
    process::{Child, Command, ExitStatus, Stdio},
    sync::atomic::{AtomicBool, Ordering},
    time::{Duration, Instant},
};

use rustix::{
    fs::{OFlags, fcntl_getfl, fcntl_setfl},
    process::{Pid, Signal, WaitId, WaitIdOptions, getpgrp, kill_process_group, waitid},
};

/// Signal authority chosen before spawning; no authority over arbitrary PIDs.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum Ownership {
    /// Create and clean up a new cooperative process group, retaining its leader
    /// unreaped until signalling ends. Descendants must not escape the group.
    Group,
    /// Join the caller's process group and signal only the owned child. Descendant
    /// teardown is not promised, including descendants retaining captured pipes.
    Direct,
}

/// Standard input policy; finite pipe input is owned until execution closes.
#[derive(Clone, Eq, PartialEq)]
pub enum Input {
    /// Read EOF from the null device.
    Null,
    /// Retain the caller's input, including its controlling terminal.
    Inherit,
    /// Send these exact already-admitted bytes, then close stdin. No extra copy
    /// or producer thread is created; a failure may leave a prefix delivered.
    Bytes(Vec<u8>),
}

impl std::fmt::Debug for Input {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Null => f.write_str("Null"),
            Self::Inherit => f.write_str("Inherit"),
            Self::Bytes(bytes) => f.debug_struct("Bytes").field("len", &bytes.len()).finish(),
        }
    }
}

/// Standard error policy, distinct from captured standard output.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum Stderr {
    /// Deliver at most this many bytes to the observer; overflow terminates.
    Capture(usize),
    /// Inherit stderr. No byte budget or retained diagnostic claim applies.
    Inherit,
}

/// Complete communication and ownership policy for one execution.
#[derive(Clone, Debug)]
pub struct Policy {
    /// Child or group termination authority.
    pub ownership: Ownership,
    /// Null, inherited or finite owned pipe input. Debug output omits input bytes.
    pub input: Input,
    /// Total stdout bytes admitted before delivery to the observer.
    pub stdout_bytes: usize,
    /// Captured bounded or inherited stderr.
    pub stderr: Stderr,
    /// Elapsed communication deadline after spawn, including waiting for pipe EOF.
    /// Blocking observer/kernel calls can exceed it; this is not a hard wall limit.
    pub elapsed: Duration,
}

/// A captured pipe; output limits and observer failures identify their stream.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum Stream {
    /// Standard output.
    Stdout,
    /// Standard error.
    Stderr,
}

/// Primary spawn or communication failure, separate from status and cleanup.
#[derive(Debug, thiserror::Error)]
pub enum Failure {
    /// Original spawn, pipe or wait observation error.
    #[error("process I/O failed: {0}")]
    Io(#[source] io::Error),
    /// Caller cancellation was observed.
    #[error("execution cancelled")]
    Cancelled,
    /// Communication elapsed deadline expired.
    #[error("execution deadline expired")]
    Deadline,
    /// A captured stream exceeded its total byte budget before observer delivery.
    #[error("{0:?} exceeded its byte budget")]
    Limit(Stream),
    /// Caller observation failed; earlier delivered bytes may remain retained.
    #[error("{stream:?} observer failed: {cause}")]
    Observer {
        /// Failing stream.
        stream: Stream,
        /// Original observer error.
        #[source]
        cause: io::Error,
    },
}

/// Evidence from execution and explicit close. Nonzero exit is retained as status,
/// not converted to success or hidden inside a communication failure.
#[derive(Debug)]
pub struct Report {
    /// Reaped child status, if available (including termination by cleanup).
    pub status: Option<ExitStatus>,
    /// Spawn/communication refusal; `None` alone does not imply tool success.
    pub failure: Option<Failure>,
    /// Signalling/reaping errors, including failed group cleanup with fallback.
    pub cleanup: Vec<io::Error>,
}

struct Session {
    child: Option<Child>,
    ownership: Ownership,
}

impl Session {
    fn close(&mut self) -> (Option<ExitStatus>, Vec<io::Error>) {
        self.close_with(
            |child| match kill_process_group(Pid::from_child(child), Signal::KILL) {
                Ok(()) | Err(rustix::io::Errno::SRCH) => Ok(()),
                #[cfg(target_os = "macos")]
                Err(rustix::io::Errno::PERM) if macos::exited_group(child) => Ok(()),
                Err(error) => Err(error.into()),
            },
            Child::kill,
            Child::wait,
        )
    }

    // Keep the syscall seam private: tests inject failures without signalling
    // borrowed PIDs, changing process credentials or leaking a real child.
    fn close_with(
        &mut self,
        group: impl FnOnce(&Child) -> io::Result<()>,
        direct: impl FnOnce(&mut Child) -> io::Result<()>,
        reap: impl FnOnce(&mut Child) -> io::Result<ExitStatus>,
    ) -> (Option<ExitStatus>, Vec<io::Error>) {
        let Some(mut child) = self.child.take() else {
            return (None, vec![]);
        };
        let mut errors = vec![];
        if self.ownership == Ownership::Group {
            if let Err(error) = group(&child) {
                errors.push(error);

                if let Err(error) = direct(&mut child) {
                    errors.push(error);
                }
            }
        } else if let Err(error) = direct(&mut child) {
            errors.push(error);
        }
        let status = match reap(&mut child) {
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

/// Execute and close one owned child, fairly servicing its pipes.
///
/// Each loop writes at most one bounded stdin chunk and reads at most one bounded
/// chunk per captured stream. Pipe input is nonblocking and closes at EOF; empty
/// input closes immediately. Its owned buffer length is the exact input ceiling:
/// callers admit the bytes before invoking this owner. Never put credentials in
/// ordinary request input merely because this executor can carry bytes.
/// Budgets are checked
/// before observer delivery; a chunk crossing a limit is not delivered. The owner
/// retains the leader unreaped until signalling finishes. Explicit close runs on
/// success and failure; Drop is fallback for observer panic, with no cleanup
/// evidence promised during unwinding. No automatic retry occurs.
///
/// The function overwrites stdin/stdout/stderr and process-group configuration.
/// Executable, arguments, environment and working directory stay caller-owned.
/// Inherited terminal behavior needs the caller's foreground group/terminal.
/// `Direct` closes no descendants; a retained pipe can therefore hit the deadline.
/// Observers must return promptly. Callers own sink flushing and byte retention.
/// All errors are returned as evidence, not as a success-shaped partial buffer.
/// A stdin error or child exit before the complete input is sent refuses execution.
/// Cancellation/deadline/refusal can leave input partially delivered; closing the
/// child does not undo its work or grant permission to retry.
#[must_use]
pub fn run(
    command: &mut Command,
    policy: Policy,
    cancelled: &AtomicBool,
    mut observe: impl FnMut(Stream, &[u8]) -> io::Result<()>,
) -> Report {
    command
        .stdin(match &policy.input {
            Input::Null => Stdio::null(),
            Input::Inherit => Stdio::inherit(),
            Input::Bytes(_) => Stdio::piped(),
        })
        .stdout(Stdio::piped())
        .stderr(match policy.stderr {
            Stderr::Capture(_) => Stdio::piped(),
            Stderr::Inherit => Stdio::inherit(),
        })
        .process_group(match policy.ownership {
            Ownership::Group => 0,
            Ownership::Direct => getpgrp().as_raw_pid(),
        });
    let child = match command.spawn() {
        Ok(child) => child,
        Err(error) => {
            return Report {
                status: None,
                failure: Some(Failure::Io(error)),
                cleanup: vec![],
            };
        }
    };
    let mut session = Session {
        child: Some(child),
        ownership: policy.ownership,
    };
    let failure = communicate(&mut session, policy, cancelled, &mut observe).err();
    let (status, cleanup) = session.close();

    Report {
        status,
        failure,
        cleanup,
    }
}

fn nonblocking(fd: &impl AsFd) -> io::Result<()> {
    fcntl_setfl(fd, fcntl_getfl(fd)? | OFlags::NONBLOCK)?;

    Ok(())
}

fn communicate(
    session: &mut Session,
    policy: Policy,
    cancelled: &AtomicBool,
    observe: &mut impl FnMut(Stream, &[u8]) -> io::Result<()>,
) -> Result<(), Failure> {
    let child = session
        .child
        .as_mut()
        .ok_or_else(|| Failure::Io(io::Error::other("missing child owner")))?;
    let mut stdout = child
        .stdout
        .take()
        .ok_or_else(|| Failure::Io(io::Error::other("missing stdout")))?;
    let mut stderr = child.stderr.take();
    let input = match policy.input {
        Input::Bytes(bytes) => bytes,
        Input::Null | Input::Inherit => Vec::new(),
    };
    let mut stdin = child.stdin.take();
    if let Some(pipe) = &stdin {
        nonblocking(pipe).map_err(Failure::Io)?;
    }
    if input.is_empty() {
        // Empty pipe input still has a meaningful EOF, including for programs
        // waiting for EOF before emitting any stdout/stderr.
        stdin.take();
    }
    let mut written = 0_usize;
    nonblocking(&stdout).map_err(Failure::Io)?;
    if let Some(pipe) = &stderr {
        nonblocking(pipe).map_err(Failure::Io)?;
    }
    let started = Instant::now();
    let mut eof = [false, stderr.is_none()];
    let mut count = [0_usize; 2];
    let mut buffer = [0_u8; 8192];

    loop {
        if cancelled.load(Ordering::Relaxed) {
            return Err(Failure::Cancelled);
        }
        if started.elapsed() >= policy.elapsed {
            return Err(Failure::Deadline);
        }
        let mut progress = false;
        if let Some(pipe) = &mut stdin {
            let end = written.saturating_add(8192).min(input.len());

            match pipe.write(&input[written..end]) {
                Ok(0) => return Err(Failure::Io(io::ErrorKind::WriteZero.into())),
                Ok(n) => {
                    written += n;
                    progress = true;

                    if written == input.len() {
                        stdin.take();
                    }
                }
                Err(error)
                    if matches!(
                        error.kind(),
                        io::ErrorKind::WouldBlock | io::ErrorKind::Interrupted
                    ) => {}
                Err(error) => return Err(Failure::Io(error)),
            }
        }
        let stderr_cap = match policy.stderr {
            Stderr::Capture(cap) => cap,
            Stderr::Inherit => 0,
        };
        for (index, stream, cap, pipe) in [
            (
                0,
                Stream::Stdout,
                policy.stdout_bytes,
                Some(&mut stdout as &mut dyn Read),
            ),
            (
                1,
                Stream::Stderr,
                stderr_cap,
                stderr.as_mut().map(|p| p as &mut dyn Read),
            ),
        ] {
            if eof[index] {
                continue;
            }
            let Some(pipe) = pipe else { continue };

            match pipe.read(&mut buffer) {
                Ok(0) => eof[index] = true,
                Ok(n) => {
                    progress = true;
                    count[index] = count[index]
                        .checked_add(n)
                        .filter(|total| *total <= cap)
                        .ok_or(Failure::Limit(stream))?;
                    observe(stream, &buffer[..n])
                        .map_err(|cause| Failure::Observer { stream, cause })?;
                }
                Err(error)
                    if matches!(
                        error.kind(),
                        io::ErrorKind::WouldBlock | io::ErrorKind::Interrupted
                    ) => {}
                Err(error) => return Err(Failure::Io(error)),
            }
        }
        let exited = match waitid(
            WaitId::Pid(Pid::from_child(child)),
            WaitIdOptions::EXITED | WaitIdOptions::NOHANG | WaitIdOptions::NOWAIT,
        ) {
            Ok(status) => status.is_some(),
            Err(error) if error == rustix::io::Errno::INTR => false,
            Err(error) => return Err(Failure::Io(error.into())),
        };
        if exited && eof.iter().all(|value| *value) {
            if written != input.len() {
                return Err(Failure::Io(io::ErrorKind::BrokenPipe.into()));
            }

            return Ok(());
        }

        if !progress {
            std::thread::sleep(Duration::from_millis(2));
        }
    }
}
