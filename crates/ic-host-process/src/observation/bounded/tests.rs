use super::*;
use crate::tests::support::Fixture;
fn stat(pid: u32, start: u64) -> String {
    let mut f = vec!["0".to_string(); 22];
    f[0] = "S".into();
    f[1] = "1".into();
    f[11] = "8".into();
    f[12] = "9".into();
    f[19] = start.to_string();
    format!("{pid} (cargo (worker)) {}", f.join(" "))
}
#[test]
fn raw_samples_preserve_identity_and_refuse_malformed_numbers() {
    let p = parse_process(10, &stat(10, 42)).unwrap();
    assert_eq!(
        (p.name.as_str(), p.state, p.cpu_ticks, p.start_ticks),
        ("cargo (worker)", 'S', 17, 42)
    );
    assert!(parse_process(11, &stat(10, 42)).is_none());
    assert!(parse_process(10, "10 (worker) S 1").is_none());
}
#[test]
fn tree_limits_and_missing_task_evidence_are_explicit() {
    let f = Fixture::new();
    fs::create_dir_all(f.path().join("10/task/10")).unwrap();
    fs::write(f.path().join("10/stat"), stat(10, 42)).unwrap();
    fs::write(f.path().join("10/task/10/children"), "").unwrap();
    let limits = TreeLimits {
        processes: 1,
        threads: 1,
        children_per_thread: 1,
    };
    let (p, complete) = process_tree(f.path(), 10, limits);
    assert!(complete);
    assert_eq!(p.len(), 1);
    assert!(
        !process_tree(
            f.path(),
            10,
            TreeLimits {
                threads: 0,
                ..limits
            }
        )
        .1
    );
    assert!(
        !process_tree(
            f.path(),
            10,
            TreeLimits {
                processes: 0,
                ..limits
            }
        )
        .1
    );
    fs::write(f.path().join("10/task/10/children"), "11").unwrap();
    assert!(!process_tree(f.path(), 10, limits).1);
}
#[cfg(target_os = "linux")]
#[test]
fn lock_observation_matches_exact_inode_and_ignores_waiters() {
    let dev = rustix::fs::makedev(8, 1);
    assert_eq!(
        parse_kernel_holder("1: FLOCK ADVISORY WRITE 42 08:01:99 0 EOF", dev, 99),
        KernelLock::Held { pid: 42 }
    );
    assert_eq!(
        parse_kernel_holder("1: -> FLOCK ADVISORY WRITE 42 08:01:99 0 EOF", dev, 99),
        KernelLock::NotObserved
    );
    assert_eq!(
        parse_kernel_holder("1: FLOCK ADVISORY WRITE 42 08:01:98 0 EOF", dev, 99),
        KernelLock::NotObserved
    );
}
