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

/// Re-execute the current libtest case with bounded output and child custody.
///
/// Returns true in the parent after the isolated case passes, and false in the
/// selected child. The caller selects a unique environment guard, context and
/// process limits. Re-execution isolates descriptor-owning parallel test threads;
/// it does not change the test's fixture or admission policy.
///
/// # Panics
/// Panics outside a named test thread or if spawning, capture, cleanup or the
/// child test fails. Captured output is bounded by the supplied limits.
#[cfg(unix)]
#[must_use]
pub fn run_isolated(driver: &str, context: &str, limits: crate::tool::OutputLimits) -> bool {
    let thread = std::thread::current();
    let name = thread.name().expect("libtest names each test thread");
    if std::env::var(driver).as_deref() == Ok(name) {
        return false;
    }
    let mut child = OwnedChild::spawn(
        Command::new(std::env::current_exe().expect("current test executable"))
            .args([
                "--exact",
                name,
                "--include-ignored",
                "--test-threads=1",
                "--nocapture",
            ])
            .env(driver, name)
            .stdin(Stdio::null())
            .stdout(Stdio::piped())
            .stderr(Stdio::piped()),
        CleanupPolicy::KillAndWait,
    )
    .expect("spawn isolated test");
    crate::tool::communicate_child(
        &mut child,
        None,
        limits,
        crate::tool::SuccessfulExit::Cleanup,
        &mut || false,
    )
    .and_then(crate::tool::CompletedExecution::require_success)
    .unwrap_or_else(|error| panic!("{}", crate::tool::format_failure_evidence(context, &error)));
    true
}
