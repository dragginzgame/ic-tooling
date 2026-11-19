use std::{
    fs,
    os::unix::fs::{PermissionsExt, symlink},
};

use super::Target;

#[test]
fn capture_uses_no_follow_components_and_close_removes_private_scratch() {
    let parent = tempfile::tempdir().unwrap();
    let target = Target::create(parent.path()).unwrap();
    assert_eq!(
        fs::metadata(target.path()).unwrap().permissions().mode() & 0o077,
        0
    );
    fs::create_dir(target.path().join("nested")).unwrap();
    fs::write(target.path().join("nested/artifact.wasm"), b"owned").unwrap();
    assert_eq!(
        target
            .open_artifact(&target.path().join("nested/artifact.wasm"))
            .unwrap()
            .metadata()
            .unwrap()
            .len(),
        5
    );
    fs::write(parent.path().join("foreign"), b"foreign").unwrap();
    symlink(
        parent.path().join("foreign"),
        target.path().join("link.wasm"),
    )
    .unwrap();
    symlink(parent.path(), target.path().join("linked-directory")).unwrap();
    for relative in ["link.wasm", "linked-directory/foreign"] {
        let error = target
            .open_artifact(&target.path().join(relative))
            .unwrap_err();
        assert!(matches!(
            error
                .get_ref()
                .and_then(|cause| cause.downcast_ref::<ic_host_fs::Error>()),
            Some(ic_host_fs::Error::Refused(_))
        ));
    }
    let error = target
        .open_artifact(&parent.path().join("foreign"))
        .unwrap_err();
    assert_eq!(error.kind(), std::io::ErrorKind::InvalidInput);
    let error = target
        .open_artifact(&target.path().join("nested"))
        .unwrap_err();
    assert!(matches!(
        error
            .get_ref()
            .and_then(|cause| cause.downcast_ref::<ic_host_fs::Error>()),
        Some(ic_host_fs::Error::Kind)
    ));
    let path = target.path().to_owned();
    target.close().unwrap();
    assert!(!path.exists());
    assert_eq!(fs::read(parent.path().join("foreign")).unwrap(), b"foreign");
}
