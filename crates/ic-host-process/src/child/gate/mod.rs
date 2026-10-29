//! Caller-released Unix command launch gate, without capture or readiness policy.
#![allow(
    unsafe_code,
    reason = "reviewed pipe, close-on-exec and post-fork descriptor operations imported from Profiler"
)]
use std::{
    ffi::{OsStr, OsString},
    fs::File,
    io,
    process::Command,
};

/// Failure to prepare the gate's descriptors. No command has been spawned.
#[derive(Debug)]
pub enum LaunchGateError {
    /// Anonymous pipe creation failed.
    Pipe(io::Error),
    /// Close-on-exec setup failed.
    Descriptor(io::Error),
}
impl std::fmt::Display for LaunchGateError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Pipe(e) | Self::Descriptor(e) => e.fmt(f),
        }
    }
}
impl std::error::Error for LaunchGateError {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        match self {
            Self::Pipe(e) | Self::Descriptor(e) => Some(e),
        }
    }
}

/// Prepare a shell launcher that waits for a newline before exec'ing exact argv.
///
/// The caller configures IO/environment, spawns once, admits its own readiness,
/// then writes a line to the returned file. Closing without a line refuses exec.
/// Original stdin is untouched. No process-group or cleanup policy is selected.
/// # Errors
/// Returns descriptor preparation failures; spawning and release are caller-owned.
pub fn gated_command(
    program: &OsStr,
    arguments: &[OsString],
    launcher_name: &OsStr,
) -> Result<(Command, File), LaunchGateError> {
    use std::os::fd::{AsRawFd, FromRawFd, OwnedFd};
    use std::os::unix::process::CommandExt;

    let mut descriptors = [-1; 2];
    // SAFETY: descriptors has space for both pipe output descriptors.
    if unsafe { libc::pipe(descriptors.as_mut_ptr()) } != 0 {
        return Err(LaunchGateError::Pipe(std::io::Error::last_os_error()));
    }
    // SAFETY: pipe succeeded and transfers ownership of two unique FDs.
    let read = unsafe { OwnedFd::from_raw_fd(descriptors[0]) };
    let write = unsafe { OwnedFd::from_raw_fd(descriptors[1]) };
    for fd in [read.as_raw_fd(), write.as_raw_fd()] {
        // SAFETY: fcntl operates on live descriptors and retains no pointers.
        if unsafe { libc::fcntl(fd, libc::F_SETFD, libc::FD_CLOEXEC) } < 0 {
            return Err(LaunchGateError::Descriptor(std::io::Error::last_os_error()));
        }
    }
    let write_fd = write.as_raw_fd();
    let mut command = Command::new("/bin/sh");
    command
        .args(["-c", "read -r _ <&3 || exit 125; exec 3<&-; exec \"$@\""])
        .arg(launcher_name)
        .arg(program)
        .args(arguments);
    // SAFETY: the post-fork closure only invokes async-signal-safe FD
    // operations. It never waits; the exec'ed shell owns the actual gate.
    unsafe {
        command.pre_exec(move || {
            let source = read.as_raw_fd();
            if libc::dup2(source, 3) < 0 || libc::fcntl(3, libc::F_SETFD, 0) < 0 {
                return Err(std::io::Error::last_os_error());
            }
            if source != 3 {
                libc::close(source);
            }
            if write_fd != 3 {
                libc::close(write_fd);
            }
            Ok(())
        });
    }
    Ok((command, std::fs::File::from(write)))
}
