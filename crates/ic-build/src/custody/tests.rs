use std::fs;
use std::os::unix::fs::PermissionsExt;
use std::os::unix::fs::symlink;

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
    assert!(
        target
            .open_artifact(&target.path().join("link.wasm"))
            .is_err()
    );
    assert!(
        target
            .open_artifact(&target.path().join("linked-directory/foreign"))
            .is_err()
    );
    assert!(
        target
            .open_artifact(&parent.path().join("foreign"))
            .is_err()
    );
    assert!(target.open_artifact(&target.path().join("nested")).is_err());
    let path = target.path().to_owned();
    target.close().unwrap();
    assert!(!path.exists());
    assert_eq!(fs::read(parent.path().join("foreign")).unwrap(), b"foreign");
}
