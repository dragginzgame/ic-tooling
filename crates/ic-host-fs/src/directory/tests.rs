use crate::{Access, Directory, Error, FilePolicy, Lock};
use std::{
    ffi::OsStr,
    fs, io,
    os::unix::fs::{PermissionsExt, symlink},
};

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
    let child = root
        .create_directory(OsStr::new("child"), 0o700, 0o077)
        .unwrap();
    let original = child.identity().unwrap();
    assert!(matches!(
        root.create_directory(OsStr::new("child"), 0o700, 0o077),
        Err(Error::Io(error)) if error.kind() == io::ErrorKind::AlreadyExists
    ));
    assert_eq!(
        root.open_directory(OsStr::new("child"), 0o077)
            .unwrap()
            .unwrap()
            .identity()
            .unwrap(),
        original
    );
}

#[test]
fn removal_uses_held_parent_and_refuses_replaced_file() {
    let scratch = tempfile::tempdir().unwrap();
    let original = scratch.path().join("original");
    let moved = scratch.path().join("moved");
    fs::create_dir(&original).unwrap();
    let root = Directory::open(&original, 0).unwrap().unwrap();
    let held = root.create_file(OsStr::new("producer"), 0o600).unwrap();
    fs::rename(&original, &moved).unwrap();
    fs::create_dir(&original).unwrap();
    fs::write(original.join("producer"), b"foreign parent").unwrap();
    fs::rename(moved.join("producer"), moved.join("held")).unwrap();
    fs::write(moved.join("producer"), b"foreign child").unwrap();
    assert!(matches!(
        root.remove_file(OsStr::new("producer"), &held),
        Err(Error::Changed)
    ));
    assert_eq!(fs::read(moved.join("producer")).unwrap(), b"foreign child");
    fs::remove_file(moved.join("producer")).unwrap();
    symlink("held", moved.join("producer")).unwrap();
    assert!(matches!(
        root.remove_file(OsStr::new("producer"), &held),
        Err(Error::Refused(_))
    ));
    root.remove_file(OsStr::new("held"), &held).unwrap();
    root.sync().unwrap();
    assert!(!moved.join("held").exists());
    assert_eq!(
        fs::read(original.join("producer")).unwrap(),
        b"foreign parent"
    );
}
