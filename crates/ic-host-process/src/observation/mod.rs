//! Raw native process observations imported from IC Profiler 70907ff.
//!
//! Consumers own workload selection, cadence, PID-identity admission, aggregation
//! and interpretation of unavailable counters. These observations are not atomic.

mod clock;
#[cfg(target_os = "macos")]
mod macos;
pub mod procfs;

pub use clock::{monotonic_ns, thread_cpu_ns, tid};
#[cfg(not(target_os = "macos"))]
use std::path::Path;

/// One observed process; this is not proof of ownership or a live process handle.
#[derive(Clone, Debug, Default)]
pub struct ProcessObservation {
    /// Observed process identifier.
    pub pid: u32,
    /// Observed parent identifier.
    pub ppid: u32,
    /// Opaque creation identity: Linux start ticks or Darwin start microseconds. Compare only on the same host.
    pub start_ticks: u64,
    /// Observed process name.
    pub name: String,
    /// Process user CPU nanoseconds.
    pub user_ns: u64,
    /// Process system CPU nanoseconds.
    pub system_ns: u64,
    /// Linux waited-child user CPU nanoseconds; zero on Darwin.
    pub waited_user_ns: u64,
    /// Linux waited-child system CPU nanoseconds; zero on Darwin.
    pub waited_system_ns: u64,
    /// Resident bytes; shared pages can occur in multiple processes.
    pub rss_bytes: u64,
    /// Linux proportional-set bytes, when requested and available.
    pub pss_bytes: Option<u64>,
    /// Linux storage read bytes, when available.
    pub read_bytes: Option<u64>,
    /// Linux storage write bytes, when available.
    pub write_bytes: Option<u64>,
    /// Linux process and waited-child minor faults; zero on Darwin.
    pub minor_faults: u64,
    /// Linux process and waited-child major faults; zero on Darwin.
    pub major_faults: u64,
    /// Observed thread count.
    pub threads: u64,
}

/// Observe base CPU/RSS/identity facts without Linux I/O or PSS reads.
#[must_use]
pub fn basic_process(pid: u32) -> Option<ProcessObservation> {
    #[cfg(target_os = "macos")]
    return macos::process(pid);
    #[cfg(not(target_os = "macos"))]
    procfs::basic_process(Path::new("/proc"), pid)
}

/// Read the native opaque creation identity, or None if unavailable.
#[must_use]
pub fn process_identity(pid: u32) -> Option<u64> {
    #[cfg(target_os = "macos")]
    return macos::identity(pid);
    #[cfg(not(target_os = "macos"))]
    procfs::process_identity(Path::new("/proc"), pid)
}

/// Observe a process, including Linux I/O and explicitly requested PSS.
#[must_use]
pub fn process(pid: u32, memory_detail: bool) -> Option<ProcessObservation> {
    #[cfg(target_os = "macos")]
    {
        let _ = memory_detail;
        basic_process(pid)
    }
    #[cfg(not(target_os = "macos"))]
    {
        let mut result = basic_process(pid)?;
        procfs::read_details(Path::new("/proc"), &mut result, memory_detail);
        Some(result)
    }
}

/// Native child-list evidence. It does not select or adopt workload descendants.
#[derive(Debug, Default)]
pub struct ChildProcesses {
    /// Observed child process IDs.
    pub pids: Vec<u32>,
    /// The interface or all thread lists were unavailable.
    pub unavailable: bool,
    /// A thread list was unreadable or malformed.
    pub incomplete: bool,
    /// The Darwin caller-selected PID capacity was reached.
    pub truncated: bool,
}

/// Observe native child IDs without changing process ownership.
/// Darwin's capacity is caller-selected; Linux inspects every task's children file.
#[must_use]
pub fn children(pid: u32, capacity: usize) -> ChildProcesses {
    #[cfg(target_os = "macos")]
    return macos::children(pid, capacity);
    #[cfg(not(target_os = "macos"))]
    {
        let _ = capacity;
        procfs::children(Path::new("/proc"), pid)
    }
}

/// Observe the Linux scheduler-affinity CPU count for a PID.
/// Returns None on another host or native query failure; fallback policy is caller-owned.
#[must_use]
#[cfg_attr(
    not(target_os = "linux"),
    allow(
        clippy::missing_const_for_fn,
        reason = "the Linux implementation queries live scheduler state and cannot be const"
    )
)]
#[allow(
    unsafe_code,
    reason = "reviewed initialized sched_getaffinity output imported from Profiler"
)]
pub fn affinity_cpus(pid: u32) -> Option<u32> {
    #[cfg(target_os = "linux")]
    {
        let pid = libc::pid_t::try_from(pid).ok()?;
        // SAFETY: cpu_set_t is a plain-data bitset; all-zero initialization is valid.
        let mut set: libc::cpu_set_t = unsafe { std::mem::zeroed() };
        // SAFETY: correct writable output buffer and ABI size; pointer is not retained.
        if unsafe { libc::sched_getaffinity(pid, std::mem::size_of_val(&set), &raw mut set) } != 0 {
            return None;
        }
        // SAFETY: reads the complete initialized bitset after a successful query.
        u32::try_from(unsafe { libc::CPU_COUNT(&set) }).ok()
    }
    #[cfg(not(target_os = "linux"))]
    {
        let _ = pid;
        None
    }
}

#[cfg(test)]
mod tests {
    #[test]
    #[cfg(unix)]
    fn native_identity_clocks_and_process_facts_are_available() {
        let pid = std::process::id();
        let before = super::monotonic_ns();
        let observation = super::process(pid, true).unwrap();
        assert_eq!(observation.pid, pid);
        assert_ne!(observation.rss_bytes, 0);
        assert_eq!(super::process_identity(pid), Some(observation.start_ticks));
        assert!(super::monotonic_ns() >= before);
        assert_ne!(super::thread_cpu_ns(), 0);
        assert_ne!(super::tid(), 0);
        #[cfg(target_os = "macos")]
        assert!(observation.pss_bytes.is_none() && observation.read_bytes.is_none());
    }
}
