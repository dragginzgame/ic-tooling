use crate::{Directory, Error};
use std::{
    ffi::OsStr,
    fs, io,
    os::unix::{
        ffi::OsStrExt,
        fs::{PermissionsExt, symlink},
    },
};

#[test]
fn all_names_and_policy_are_admitted_before_creation() {
    let scratch = tempfile::tempdir().unwrap();
    let root = Directory::open(scratch.path(), 0).unwrap().unwrap();
    for invalid in [b"".as_slice(), b".", b"..", b"a/b", b"nul\0name"] {
        let error = root
            .prepare_directories(
                &[OsStr::new("new"), OsStr::from_bytes(invalid)],
                0o700,
                0o077,
            )
            .unwrap_err();
        assert_eq!(error.component, Some(1));
        assert!(matches!(error.source, Error::Name));
        assert!(!scratch.path().join("new").exists());
    }
    let error = root
        .prepare_directories(&[OsStr::new("new")], 0o755, 0o077)
        .unwrap_err();
    assert_eq!(error.component, None);
    assert!(matches!(error.source, Error::Permissions { .. }));
    assert!(!scratch.path().join("new").exists());
    assert_eq!(
        root.prepare_directories(&[], 0o700, 0o077)
            .unwrap()
            .identity()
            .unwrap(),
        root.identity().unwrap()
    );
}

#[test]
fn preparation_returns_held_leaf_under_renamed_root() {
    let scratch = tempfile::tempdir().unwrap();
    let original = scratch.path().join("original");
    let moved = scratch.path().join("moved");
    fs::create_dir(&original).unwrap();
    let root = Directory::open(&original, 0).unwrap().unwrap();
    fs::rename(&original, &moved).unwrap();
    fs::create_dir(&original).unwrap();
    let names = [OsStr::new("one"), OsStr::new("two")];
    let leaf = root.prepare_directories(&names, 0o700, 0o077).unwrap();
    leaf.create_file(OsStr::new("held"), 0o600).unwrap();
    assert!(moved.join("one/two/held").is_file());
    assert!(!original.join("one").exists());
    assert_eq!(
        fs::metadata(moved.join("one/two"))
            .unwrap()
            .permissions()
            .mode()
            & 0o077,
        0
    );
    assert_eq!(
        root.prepare_directories(&names, 0o700, 0o077)
            .unwrap()
            .identity()
            .unwrap(),
        leaf.identity().unwrap()
    );
}

#[test]
fn occupied_components_and_permissions_are_never_repaired() {
    let scratch = tempfile::tempdir().unwrap();
    let root = Directory::open(scratch.path(), 0).unwrap().unwrap();
    fs::write(scratch.path().join("file"), b"foreign").unwrap();
    fs::create_dir(scratch.path().join("public")).unwrap();
    fs::set_permissions(
        scratch.path().join("public"),
        fs::Permissions::from_mode(0o755),
    )
    .unwrap();
    symlink("public", scratch.path().join("link")).unwrap();
    for child in ["file", "public", "link"] {
        let error = root
            .prepare_directories(&[OsStr::new(child), OsStr::new("new")], 0o700, 0o077)
            .unwrap_err();
        assert_eq!(error.component, Some(0));
        assert!(matches!(
            error.source,
            Error::Refused(_) | Error::Permissions { .. }
        ));
    }
    assert_eq!(fs::read(scratch.path().join("file")).unwrap(), b"foreign");
    assert_eq!(
        fs::metadata(scratch.path().join("public"))
            .unwrap()
            .permissions()
            .mode()
            & 0o777,
        0o755
    );
    assert!(!scratch.path().join("public/new").exists());
}

#[test]
fn sync_failures_retain_prefix_and_original_cause() {
    for fail_at in [0, 1, 2, 3] {
        let scratch = tempfile::tempdir().unwrap();
        let root = Directory::open(scratch.path(), 0).unwrap().unwrap();
        let mut calls = 0;
        let error = root
            .prepare_with(
                &[OsStr::new("one"), OsStr::new("two"), OsStr::new("three")],
                0o700,
                0o077,
                &mut |directory| {
                    let current = calls;
                    calls += 1;

                    if current == fail_at {
                        Err(io::Error::from_raw_os_error(5))
                    } else {
                        directory.sync()
                    }
                },
            )
            .unwrap_err();
        assert_eq!(error.component, Some(fail_at / 2));
        assert!(matches!(error.source, Error::Io(cause) if cause.raw_os_error() == Some(5)));
        assert_eq!(calls, fail_at + 1);
        assert!(scratch.path().join("one").is_dir());
        assert_eq!(scratch.path().join("one/two").is_dir(), fail_at >= 2);
        assert!(!scratch.path().join("one/two/three").exists());
    }
}
