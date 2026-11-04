use super::*;
use std::fs;

#[test]
fn fixture_guard_removes_successful_test_output() {
    let parent = crate::tests::fixture();
    let fixture = FixtureDirectory::new(parent.path(), "successful").unwrap();
    let path = fixture.path().to_path_buf();
    fs::write(path.join("output"), b"completed output").unwrap();
    drop(fixture);
    assert!(!path.exists());
    assert!(parent.path().is_dir());
}

#[test]
fn fixture_guard_retains_failed_test_evidence_during_unwind() {
    let parent = crate::tests::fixture();
    let fixture = FixtureDirectory::new(parent.path(), "failed").unwrap();
    let path = fixture.path().to_path_buf();
    fs::write(path.join("evidence"), b"failure evidence").unwrap();
    let panic = std::panic::catch_unwind(move || {
        let _fixture = fixture;
        panic!("intentional fixture failure");
    });
    assert!(panic.is_err());
    assert_eq!(
        fs::read(path.join("evidence")).unwrap(),
        b"failure evidence"
    );
}

#[test]
fn parallel_allocation_preserves_existing_entries_and_caller_retention() {
    let root = crate::tests::fixture();
    let marker = root.path().join("retained");
    fs::write(&marker, b"original evidence").unwrap();
    #[expect(
        clippy::needless_collect,
        reason = "spawn every worker before joining to exercise concurrent allocation"
    )]
    let paths = std::thread::scope(|scope| {
        let workers = (0..32)
            .map(|_| scope.spawn(|| create_fixture_directory(root.path(), "parallel").unwrap()))
            .collect::<Vec<_>>();
        workers
            .into_iter()
            .map(|worker| worker.join().unwrap())
            .collect::<std::collections::BTreeSet<_>>()
    });
    assert_eq!(paths.len(), 32);
    assert_eq!(fs::read(&marker).unwrap(), b"original evidence");
    for path in paths {
        assert!(path.is_dir());
        assert_eq!(path.parent(), Some(root.path()));
        #[cfg(unix)]
        {
            use std::os::unix::fs::PermissionsExt as _;
            assert_eq!(fs::metadata(&path).unwrap().permissions().mode() & 0o077, 0);
        }
    }
}

#[test]
fn invalid_prefix_and_missing_parent_preserve_the_namespace() {
    let root = crate::tests::fixture();
    for prefix in [
        "",
        ".",
        "..",
        "../escape",
        "/absolute",
        "nested/child",
        "trailing/",
        "nested/.",
        "line\nfeed",
        "carriage\rreturn",
    ] {
        assert_eq!(
            create_fixture_directory(root.path(), prefix)
                .unwrap_err()
                .kind(),
            io::ErrorKind::InvalidInput
        );
    }
    assert_eq!(
        create_fixture_directory(&root.path().join("absent"), "valid")
            .unwrap_err()
            .kind(),
        io::ErrorKind::NotFound
    );
    assert_eq!(fs::read_dir(root.path()).unwrap().count(), 0);
    assert_eq!(
        create_fixture_directory(&root.path().join("absent"), "../invalid")
            .unwrap_err()
            .kind(),
        io::ErrorKind::InvalidInput
    );
}

#[cfg(unix)]
#[test]
fn parent_alias_returns_a_physical_path() {
    let root = crate::tests::fixture();
    let parent = root.path().join("physical");
    fs::create_dir(&parent).unwrap();
    let alias = root.path().join("alias");
    std::os::unix::fs::symlink(&parent, &alias).unwrap();
    let path = create_fixture_directory(&alias, "fixture").unwrap();
    assert_eq!(path.parent(), Some(parent.as_path()));
    assert_eq!(path, path.canonicalize().unwrap());
}
