//! A worker retains child custody independently of completion observation.
#[cfg(test)]
mod tests;

use std::{
    io,
    os::unix::process::CommandExt,
    process::{Child, Command, ExitStatus, Stdio},
    sync::mpsc::{self, Receiver, Sender, TryRecvError},
};

/// Completion observation without termination authority.
///
/// Dropping this value does not stop the child or its reaper. The worker lives
/// only within this host process; host/session exit can still affect the child.
/// No readiness, successful-start or retry permission is implied.
pub struct Completion {
    receiver: Receiver<io::Result<ExitStatus>>,
    observed: Option<io::Result<ExitStatus>>,
}

impl Completion {
    /// Observe completion without blocking, retaining the original result.
    ///
    /// Repeated observations borrow the same result and never wait on a PID.
    /// A wait/worker error does not prove the child was reaped or had no effect.
    pub fn try_result(&mut self) -> Option<&io::Result<ExitStatus>> {
        if self.observed.is_none() {
            self.observed = match self.receiver.try_recv() {
                Ok(result) => Some(result),
                Err(TryRecvError::Empty) => None,
                Err(TryRecvError::Disconnected) => Some(Err(disconnected())),
            };
        }
        self.observed.as_ref()
    }

    /// Wait for completion, consuming the observer; this has no elapsed deadline.
    ///
    /// # Errors
    /// Returns the original wait error, or uncertain completion if the worker
    /// disconnected. An error does not authorize relaunch or prove reaping.
    pub fn wait(self) -> io::Result<ExitStatus> {
        self.observed
            .unwrap_or_else(|| self.receiver.recv().unwrap_or_else(|_| Err(disconnected())))
    }
}

fn disconnected() -> io::Error {
    io::Error::other("launch worker disconnected; child outcome is unknown")
}

/// Launch once with null stdin/stdout and inherited stderr, then reap in a worker.
///
/// The child joins the caller's process group; this is not daemonization. Arguments,
/// environment and working directory remain caller-owned. No pipes are captured,
/// no deadline or signal is applied, and dropping the completion observer leaves
/// reaping with the worker. Callers own launch concurrency and desktop policy.
/// Success means a child was spawned, not that an application became ready.
///
/// # Errors
/// Worker setup or child spawn failure retains its original I/O error. A broken
/// startup channel instead reports an unknown spawn outcome; do not retry blindly.
pub fn launch(command: Command) -> io::Result<Completion> {
    launch_with(command, |job| {
        std::thread::Builder::new()
            .name("ic-process-reap".into())
            .spawn(job)
            .map(drop)
    })
}

fn launch_with(
    mut command: Command,
    start: impl FnOnce(Box<dyn FnOnce() + Send>) -> io::Result<()>,
) -> io::Result<Completion> {
    command
        .stdin(Stdio::null())
        .stdout(Stdio::null())
        .stderr(Stdio::inherit())
        .process_group(rustix::process::getpgrp().as_raw_pid());
    let (startup_tx, startup_rx) = mpsc::channel();
    let (done_tx, done_rx) = mpsc::channel();
    start(Box::new(move || {
        work(|| command.spawn(), Child::wait, &startup_tx, &done_tx);
    }))?;
    startup_rx.recv().map_err(|_| disconnected())??;
    Ok(Completion {
        receiver: done_rx,
        observed: None,
    })
}

// Private seams exercise transfer and OS failures without leaking real children.
fn work<C>(
    spawn: impl FnOnce() -> io::Result<C>,
    mut wait: impl FnMut(&mut C) -> io::Result<ExitStatus>,
    startup: &Sender<io::Result<()>>,
    done: &Sender<io::Result<ExitStatus>>,
) {
    let mut child = match spawn() {
        Ok(child) => child,
        Err(error) => {
            let _ = startup.send(Err(error));
            return;
        }
    };
    // Lost observers never take waiting custody away from this worker.
    let _ = startup.send(Ok(()));
    let result = loop {
        match wait(&mut child) {
            Err(error) if error.kind() == io::ErrorKind::Interrupted => continue,
            result => break result,
        }
    };
    let _ = done.send(result);
}
