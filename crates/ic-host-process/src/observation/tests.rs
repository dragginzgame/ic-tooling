#[test]
#[cfg(target_os = "linux")]
fn affinity_preserves_current_thread_selection_and_invalid_pid_refusal() {
    let current = super::affinity_cpus(0).unwrap();
    assert!(current > 0);
    assert_eq!(
        super::affinity_cpus(u32::try_from(super::tid()).unwrap()),
        Some(current)
    );
    assert_eq!(super::affinity_cpus(u32::MAX), None);
}

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
