//! Opt-in HTTP, Unix executable and process-observation fixtures for tests.
//!
//! Callers own fixture content, paths, namespace custody, transport policy and
//! assertions. Executable helpers own writer-process lifetime; no tool is admitted.

use std::time::{Duration, Instant};

#[cfg(unix)]
use std::{
    fs,
    io::{self, Write as _},
    os::unix::fs::PermissionsExt as _,
    path::Path,
    process::{Command, Stdio},
};

#[cfg(unix)]
use crate::child::{CleanupPolicy, OwnedChild};

#[cfg(unix)]
pub mod crash_barrier;
pub mod http;
pub mod net;

// A zero or elapsed deadline must never become a native socket timeout.
fn remaining_until(deadline: Instant) -> Option<Duration> {
    deadline
        .checked_duration_since(Instant::now())
        .filter(|remaining| !remaining.is_zero())
}

/// Observe one Unix test process through the native `ps` state field.
///
/// Missing processes return `None`; zombies retain their `Z` state so callers
/// can distinguish stopped execution from completed reaping. No PID is owned or
/// signalled. Output is bounded; callers own polling intervals and deadlines.
///
/// # Errors
/// Preserves process/IO failures and refuses failed or malformed observations.
#[cfg(unix)]
pub fn process_state(pid: u32) -> io::Result<Option<char>> {
    use crate::tool::{OutputLimit, OutputLimits, capture_command};
    let output = capture_command(
        Command::new("/bin/ps").args(["-p", &pid.to_string(), "-o", "stat="]),
        OutputLimits {
            stdout: OutputLimit::Terminate(4096),
            stderr: OutputLimit::Terminate(4096),
            timeout: None,
        },
    )
    .map_err(io::Error::other)?;
    if output.status.code() == Some(1) && output.stdout.is_empty() && output.stderr.is_empty() {
        return Ok(None);
    }
    if !output.status.success() {
        return Err(io::Error::other(format!(
            "process-state inspection failed: {}",
            output.status
        )));
    }
    let state = std::str::from_utf8(&output.stdout)
        .map_err(|error| io::Error::new(io::ErrorKind::InvalidData, error))?;
    Ok(state.trim().chars().next())
}

/// Create or replace a caller-owned executable script with mode 0755.
///
/// A parallel fork can inherit a parent's writable executable descriptor until
/// exec, causing ETXTBSY after the parent closes it. This parent holds only a
/// pipe: a private writer opens the target, receives all bytes, and is reaped
/// before permissions are set or the caller can execute the script.
/// No parent directories are created and no path/symlink admission is supplied.
/// Use a trusted, exclusively owned fixture namespace. Failed writes may leave
/// partial bytes for the caller to inspect. There is no deadline or retry.
///
/// # Errors
/// Returns spawn, pipe, write, wait, failed writer exit or filesystem errors.
/// The writer is waited after a write failure; that original failure remains
/// primary. The owned process group is cleaned up if waiting fails or unwinds.
#[cfg(unix)]
pub fn write_executable_script(path: &Path, contents: impl AsRef<[u8]>) -> io::Result<()> {
    let mut writer = OwnedChild::spawn(
        Command::new("/bin/sh")
            .args(["-c", "cat > \"$1\"", "write-test-script"])
            .arg(path)
            .stdin(Stdio::piped()),
        CleanupPolicy::KillAndWait,
    )?;
    let written = writer
        .take_stdin()
        .ok_or_else(|| io::Error::other("script writer stdin missing"))?
        .write_all(contents.as_ref());
    let status = writer.wait();
    written?;
    let status = status?;
    if !status.success() {
        return Err(io::Error::other(format!("script writer failed: {status}")));
    }
    let mut permissions = fs::metadata(path)?.permissions();
    permissions.set_mode(0o755);
    fs::set_permissions(path, permissions)
}

#[cfg(all(test, unix))]
mod tests;
