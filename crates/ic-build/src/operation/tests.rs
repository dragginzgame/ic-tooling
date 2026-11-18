use std::{fs, io};

use super::{Artifact, BuildFailure, Failure, complete};
use crate::custody::Target;

#[test]
fn actual_cleanup_refusal_prevents_success_and_preserves_foreign_file() {
    let parent = tempfile::tempdir().unwrap();
    let target = Target::create(parent.path()).unwrap();
    let path = target.path().to_owned();
    // Deliberately violate the stable-scratch-parent assumption to exercise the
    // OS refusal. Recursive directory removal must not unlink this foreign file.
    fs::remove_dir(&path).unwrap();
    fs::write(&path, b"foreign").unwrap();
    let cleanup = target.close();
    assert!(cleanup.is_err());
    let result = complete(
        Ok(Artifact {
            bytes: vec![1],
            digest: [0; 32],
        }),
        cleanup,
    );
    let failure = result.unwrap_err();
    assert!(matches!(failure.primary, Failure::Cleanup));
    assert_eq!(failure.cleanup.len(), 1);
    assert_eq!(failure.scratch.as_ref(), Some(&path));
    assert_eq!(fs::read(path).unwrap(), b"foreign");
}

#[test]
fn cleanup_refusal_keeps_the_primary_capture_failure() {
    let failure = complete(
        Err(BuildFailure::new(Failure::ArtifactLimit)),
        Err(("scratch".into(), io::ErrorKind::PermissionDenied.into())),
    )
    .unwrap_err();
    assert!(matches!(failure.primary, Failure::ArtifactLimit));
    assert_eq!(failure.cleanup[0].kind(), io::ErrorKind::PermissionDenied);
}
