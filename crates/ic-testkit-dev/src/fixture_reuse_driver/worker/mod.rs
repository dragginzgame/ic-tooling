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

    pub(super) const fn process(&mut self) -> &mut Child {
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
        if child.try_wait()?.is_some() {
            return Ok(());
        }
        let pid = libc::pid_t::try_from(child.id()).map_err(io::Error::other)?;
        #[expect(
            unsafe_code,
            reason = "signal only this still-owned, unreaped direct child"
        )]
        // SAFETY: after try_wait returns None this owned child has not been reaped,
        // so its PID cannot be reused before wait. Signal its handler, not its group.
        let sent = unsafe { libc::kill(pid, libc::SIGTERM) } == 0;
        if sent {
            return Ok(());
        }
        let error = io::Error::last_os_error();
        if error.raw_os_error() == Some(libc::ESRCH) {
            Ok(())
        } else {
            Err(error)
        }
    }
    #[cfg(not(unix))]
    {
        let _ = child;
        Err(io::Error::from(io::ErrorKind::Unsupported))
    }
}
