//! A single libtest case keeps descriptors out of unrelated test forks.
#![cfg(unix)]

use ic_host_fs::durable::open_private_regular_lock_file;
use ic_host_fs::testing::FixtureDirectory;
use std::{fs, io};

#[test]
fn private_lock_preserves_existing_identity_and_caller_owned_acquisition() {
    use rustix::fs::{FlockOperation::NonBlockingLockExclusive, flock};
    use std::os::unix::fs::{MetadataExt, PermissionsExt};
    let fixture = FixtureDirectory::new(&std::env::temp_dir(), "isolated-private-lock").unwrap();
    let path = fixture.path().join("command.lock");
    fs::write(&path, b"custody evidence").unwrap();
    fs::set_permissions(&path, fs::Permissions::from_mode(0o640)).unwrap();
    let before = fs::metadata(&path).unwrap();
    let first = open_private_regular_lock_file(&path).unwrap();
    let second = open_private_regular_lock_file(&path).unwrap();
    // Admission is unlocked. A second descriptor can acquire before the first.
    flock(&second, NonBlockingLockExclusive).unwrap();
    let error = flock(&first, NonBlockingLockExclusive).unwrap_err();
    let projected = io::Error::from_raw_os_error(error.raw_os_error());
    assert_eq!(projected.kind(), io::ErrorKind::WouldBlock);
    for after in [
        first.metadata().unwrap(),
        second.metadata().unwrap(),
        fs::metadata(&path).unwrap(),
    ] {
        assert_eq!((before.dev(), before.ino()), (after.dev(), after.ino()));
        assert_eq!(before.permissions().mode(), after.permissions().mode());
    }
    assert_eq!(fs::read(&path).unwrap(), b"custody evidence");
    drop(second);
    flock(&first, NonBlockingLockExclusive).unwrap();
    drop(first);
    let reopened = open_private_regular_lock_file(&path).unwrap();
    flock(&reopened, NonBlockingLockExclusive).unwrap();
    assert_eq!(fs::read(&path).unwrap(), b"custody evidence");
}
