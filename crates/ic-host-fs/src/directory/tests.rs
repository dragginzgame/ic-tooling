use crate::{Access, Directory, Error, FilePolicy, Lock};
use std::ffi::OsStr;
use std::fs;
use std::io;
use std::os::unix::fs::{PermissionsExt, symlink};

#[test]
fn admission_refuses_links_permissions_aliases_and_limits() {
    let scratch = tempfile::tempdir().unwrap();
    fs::set_permissions(scratch.path(), fs::Permissions::from_mode(0o700)).unwrap();
    let root = Directory::open(scratch.path(), 0o077).unwrap().unwrap();
    assert!(matches!(
        root.create_file(OsStr::new("../escape"), 0o600),
        Err(Error::Name)
    ));
    root.create_file(OsStr::new("file"), 0o600).unwrap();
    assert!(matches!(root.entries(0), Err(Error::EntryLimit)));
    let policy = FilePolicy {
        forbidden: 0o177,
        single_link: true,
    };
    fs::hard_link(scratch.path().join("file"), scratch.path().join("alias")).unwrap();
    assert!(matches!(
        root.open_file(OsStr::new("file"), Access::Read, policy),
        Err(Error::Aliased)
    ));
    symlink("file", scratch.path().join("link")).unwrap();
    assert!(matches!(
        root.open_file(OsStr::new("link"), Access::Read, policy),
        Err(Error::Refused(_))
    ));
    fs::set_permissions(
        scratch.path().join("file"),
        fs::Permissions::from_mode(0o644),
    )
    .unwrap();
    assert!(matches!(
        root.open_file(OsStr::new("file"), Access::Read, policy),
        Err(Error::Permissions { .. })
    ));
    assert_eq!(root.entries(3).unwrap().len(), 3);
}
#[test]
fn locks_contend_and_explicit_release_ends_clone_authority() {
    let scratch = tempfile::tempdir().unwrap();
    let root = Directory::open(scratch.path(), 0).unwrap().unwrap();
    let file = root.create_file(OsStr::new("lock"), 0o600).unwrap();
    let lock = Lock::acquire(file).unwrap();
    let _held = lock.file().try_clone().unwrap();
    let policy = FilePolicy {
        forbidden: 0o177,
        single_link: true,
    };
    let contender = || {
        root.open_file(OsStr::new("lock"), Access::Read, policy)
            .unwrap()
            .unwrap()
    };
    assert_eq!(
        Lock::acquire(contender()).unwrap_err().kind(),
        io::ErrorKind::WouldBlock
    );
    lock.release().unwrap();
    Lock::acquire(contender()).unwrap().release().unwrap();
}

#[test]
fn directory_creation_is_explicit_and_policy_refusal_precedes_effects() {
    let scratch = tempfile::tempdir().unwrap();
    let root = Directory::open(scratch.path(), 0).unwrap().unwrap();
    assert!(
        root.open_directory(OsStr::new("child"), 0o077)
            .unwrap()
            .is_none()
    );
    assert!(matches!(
        root.create_directory(OsStr::new("child"), 0o755, 0o077),
        Err(Error::Permissions { .. })
    ));
    assert!(!scratch.path().join("child").exists());
    root.create_directory(OsStr::new("child"), 0o700, 0o077)
        .unwrap();
    assert!(
        root.create_directory(OsStr::new("child"), 0o700, 0o077)
            .is_err()
    );
}
