//! Native clocks and thread identity. No sampling, retention or application policy.
#![allow(
    unsafe_code,
    reason = "reviewed clock_gettime and native thread identity calls imported from Profiler"
)]

/// Host monotonic nanoseconds, falling back to a process-local Instant epoch.
#[must_use]
#[allow(
    clippy::cast_sign_loss,
    reason = "successful native monotonic clocks have nonnegative seconds and nanoseconds"
)]
pub fn monotonic_ns() -> u64 {
    static START: std::sync::OnceLock<std::time::Instant> = std::sync::OnceLock::new();
    #[cfg(unix)]
    {
        let mut value = libc::timespec {
            tv_sec: 0,
            tv_nsec: 0,
        };
        // SAFETY: points to an initialized timespec; clock_gettime does not retain it.
        if unsafe { libc::clock_gettime(libc::CLOCK_MONOTONIC, &raw mut value) } == 0 {
            return (value.tv_sec as u64)
                .saturating_mul(1_000_000_000)
                .saturating_add(value.tv_nsec as u64);
        }
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
#[allow(
    clippy::cast_sign_loss,
    reason = "successful native CPU clocks have nonnegative seconds and nanoseconds"
)]
pub fn thread_cpu_ns() -> u64 {
    #[cfg(unix)]
    {
        let mut value = libc::timespec {
            tv_sec: 0,
            tv_nsec: 0,
        };
        // SAFETY: valid writable timespec; this clock describes only the current thread.
        if unsafe { libc::clock_gettime(libc::CLOCK_THREAD_CPUTIME_ID, &raw mut value) } == 0 {
            return (value.tv_sec as u64)
                .saturating_mul(1_000_000_000)
                .saturating_add(value.tv_nsec as u64);
        }
    }
    0
}

/// Current native thread identity; zero on unsupported hosts or query failure.
#[must_use]
pub fn tid() -> u64 {
    #[cfg(target_os = "linux")]
    {
        // SAFETY: gettid has no pointer arguments.
        u64::try_from(unsafe { libc::syscall(libc::SYS_gettid) }).unwrap_or_default()
    }
    #[cfg(target_os = "macos")]
    {
        let mut id = 0;
        // SAFETY: a null thread selects the current thread; id is writable.
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
