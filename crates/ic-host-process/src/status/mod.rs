//! Explicit shell-style projection of observed child status; callers own CLI fallback policy.

use std::process::ExitStatus;

/// Return the numeric exit code, or `128 + signal` for Unix signal termination.
///
/// An unclassified status returns `None`. Callers choose its fallback and the
/// conversion to their CLI's supported exit-code range. This does not wait,
/// signal, reap or otherwise manage a process, and does not imply success.
#[must_use]
pub fn shell_status_code(status: ExitStatus) -> Option<i32> {
    if let Some(code) = status.code() {
        return Some(code);
    }
    #[cfg(unix)]
    {
        use std::os::unix::process::ExitStatusExt as _;
        status.signal().map(|signal| 128 + signal)
    }
    #[cfg(not(unix))]
    {
        None
    }
}

#[cfg(all(test, unix))]
mod tests;
