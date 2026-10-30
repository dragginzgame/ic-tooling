//! Native clocks and thread identity. No sampling, retention or application policy.

#[cfg(unix)]
use rustix::time::{ClockId, DynamicClockId, clock_gettime_dynamic};

#[cfg(unix)]
#[allow(
    clippy::cast_sign_loss,
    reason = "successful native clocks have nonnegative seconds and nanoseconds"
)]
fn clock_ns(clock: ClockId) -> Option<u64> {
    let value = clock_gettime_dynamic(DynamicClockId::Known(clock)).ok()?;
    Some(
        (value.tv_sec as u64)
            .saturating_mul(1_000_000_000)
            .saturating_add(value.tv_nsec as u64),
    )
}

/// Host monotonic nanoseconds, falling back to a process-local Instant epoch.
#[must_use]
pub fn monotonic_ns() -> u64 {
    static START: std::sync::OnceLock<std::time::Instant> = std::sync::OnceLock::new();
    #[cfg(unix)]
    if let Some(value) = clock_ns(ClockId::Monotonic) {
        return value;
    }
    u64::try_from(
        START
            .get_or_init(std::time::Instant::now)
            .elapsed()
            .as_nanos(),
    )
    .unwrap_or(u64::MAX)
}

/// CPU nanoseconds of the current thread; zero when the native clock is unavailable.
#[must_use]
pub fn thread_cpu_ns() -> u64 {
    #[cfg(unix)]
    if let Some(value) = clock_ns(ClockId::ThreadCPUTime) {
        return value;
    }
    0
}

/// Current native thread identity; zero on unsupported hosts or query failure.
#[must_use]
pub fn tid() -> u64 {
    #[cfg(target_os = "linux")]
    {
        u64::try_from(rustix::thread::gettid().as_raw_pid()).unwrap_or_default()
    }
    #[cfg(target_os = "macos")]
    {
        let mut id = 0;
        // SAFETY: a null thread selects the current thread; id is writable.
        #[expect(
            unsafe_code,
            reason = "Darwin thread ID has no matching Rustix interface"
        )]
        if unsafe { libc::pthread_threadid_np(0, &raw mut id) } == 0 {
            id
        } else {
            0
        }
    }
    #[cfg(not(any(target_os = "linux", target_os = "macos")))]
    {
        0
    }
}
