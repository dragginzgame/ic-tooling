//! Driver-owned cooperative worker completion, including unwinding.

use std::{
    io,
    process::{Child, Command, ExitStatus},
};

#[cfg(unix)]
use std::os::unix::process::CommandExt as _;

pub(super) struct Worker {
    process: Option<Child>,
}

impl Worker {
    pub(super) fn spawn(command: &mut Command) -> io::Result<Self> {
        #[cfg(unix)]
        command.process_group(0);
        Ok(Self {
            process: Some(command.spawn()?),
        })
    }

    pub(super) fn try_wait(&mut self) -> io::Result<Option<ExitStatus>> {
        self.process().try_wait()
    }

    pub(super) fn id(&mut self) -> u32 {
        self.process().id()
    }

    const fn process(&mut self) -> &mut Child {
        self.process
            .as_mut()
            .expect("unfinished worker retains its child")
    }

    pub(super) fn finish(mut self, request_stop: bool) -> io::Result<ExitStatus> {
        self.wait(request_stop)
    }

    fn wait(&mut self, request_stop: bool) -> io::Result<ExitStatus> {
        let mut child = self.process.take().expect("worker is completed once");
        if request_stop && let Err(error) = request_shutdown(&mut child) {
            eprintln!("could not request worker shutdown: {error}");
        }
        // Let the worker stop scheduling and finish its own server cleanup.
        // There is no group kill or teardown deadline, including during unwind.
        child.wait()
    }
}

impl Drop for Worker {
    fn drop(&mut self) {
        if self.process.is_some()
            && let Err(error) = self.wait(true)
        {
            eprintln!("could not wait for worker completion: {error}");
        }
    }
}

fn request_shutdown(child: &mut Child) -> io::Result<()> {
    #[cfg(unix)]
    {
        use rustix::process::{Pid, Signal, kill_process};

        if child.try_wait()?.is_some() {
            return Ok(());
        }
        let pid = Pid::from_raw(i32::try_from(child.id()).map_err(io::Error::other)?)
            .ok_or_else(|| io::Error::other("worker PID must be positive"))?;
        // After try_wait returns None this owned child has not been reaped,
        // so its PID cannot be reused before wait. Signal its handler, not its group.
        match kill_process(pid, Signal::TERM) {
            Ok(()) | Err(rustix::io::Errno::SRCH) => Ok(()),
            Err(error) => Err(error.into()),
        }
    }
    #[cfg(not(unix))]
    {
        let _ = child;
        Err(io::Error::from(io::ErrorKind::Unsupported))
    }
}
