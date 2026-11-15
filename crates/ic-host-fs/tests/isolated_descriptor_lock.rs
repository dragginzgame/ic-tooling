//! A single libtest case keeps descriptors out of unrelated test forks.
#![cfg(unix)]

use ic_host_fs::durable::lock_exclusive_with_wait;
use ic_host_fs::testing::FixtureDirectory;
use std::{fs::File, time::Duration};

#[test]
fn descriptor_contention_reports_and_preserves_returned_ownership() {
    let fixture = FixtureDirectory::new(&std::env::temp_dir(), "isolated-descriptor-lock").unwrap();
    let path = fixture.path().join("lock");
    std::fs::write(&path, b"").unwrap();
    let held = File::open(&path).unwrap();
    rustix::fs::flock(&held, rustix::fs::FlockOperation::LockExclusive).unwrap();
    let acquired = File::open(&path).unwrap();
    let mut reports = 0;
    lock_exclusive_with_wait(&acquired, Duration::from_millis(1), |_| {
        reports += 1;
        rustix::fs::flock(&held, rustix::fs::FlockOperation::Unlock).unwrap();
        Ok(())
    })
    .unwrap();
    assert_eq!(reports, 1);
    assert_eq!(
        rustix::fs::flock(&held, rustix::fs::FlockOperation::NonBlockingLockExclusive),
        Err(rustix::io::Errno::WOULDBLOCK)
    );
    drop(acquired);
    lock_exclusive_with_wait(&held, Duration::from_millis(1), |_| {
        panic!("uncontended acquisition must not report")
    })
    .unwrap();
}
