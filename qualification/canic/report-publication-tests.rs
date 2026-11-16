use super::*;
use std::fs;
use std::os::unix::fs::{PermissionsExt, symlink};

fn snapshot() -> Fleet {
    Fleet {
        configuration: None,
        started_at_ns: 0,
        finished_at_ns: 0,
        scope: "local deployment journal only",
        canisters: vec![],
        operations: vec![],
        issues: vec!["<untrusted>&report".into()],
        managers: vec![],
        metrics: vec![],
        stores: vec![],
        wallet: None,
    }
}

fn directory() -> std::path::PathBuf {
    ic_host_fs::directory::create_unique_directory(
        &std::env::temp_dir(),
        "canic-report",
        0o700,
        NonZeroUsize::new(32),
    )
    .unwrap()
}

#[test]
fn report_bytes_modes_and_existing_evidence_are_preserved() {
    let root = directory();
    fs::set_permissions(&root, fs::Permissions::from_mode(0o755)).unwrap();
    fs::create_dir(root.join("control")).unwrap();
    let ordinary_mode = fs::metadata(root.join("control"))
        .unwrap()
        .permissions()
        .mode()
        & 0o777;
    let path = root.join("new/deep/report.html");
    let snapshot = snapshot();
    publish(&snapshot, Some(&path), false, false, 0, &[]).unwrap();
    let expected = render(&snapshot, "").into_bytes();
    assert_eq!(fs::read(&path).unwrap(), expected);
    assert_eq!(
        fs::metadata(&path).unwrap().permissions().mode() & 0o777,
        0o600
    );
    assert_eq!(
        fs::metadata(&root).unwrap().permissions().mode() & 0o777,
        0o755
    );
    for name in ["new", "new/deep"] {
        assert_eq!(
            fs::metadata(root.join(name)).unwrap().permissions().mode() & 0o777,
            ordinary_mode
        );
    }
    assert!(publish(&snapshot, Some(&path), false, false, 0, &[]).is_err());
    assert_eq!(fs::read(&path).unwrap(), expected);
    assert_eq!(fs::read_dir(root.join("new/deep")).unwrap().count(), 1);
    fs::remove_dir_all(root).unwrap();
}

#[test]
fn report_refusals_preserve_foreign_entries_and_parent_authority() {
    let root = directory();
    let foreign = root.join("foreign");
    fs::write(&foreign, b"retain").unwrap();
    symlink("foreign", root.join("link.html")).unwrap();
    symlink("missing", root.join("dangling.html")).unwrap();
    fs::hard_link(&foreign, root.join("hard.html")).unwrap();
    fs::create_dir(root.join("occupied.html")).unwrap();
    fs::create_dir(root.join("real")).unwrap();
    symlink("real", root.join("alias")).unwrap();
    for name in [
        "link.html",
        "dangling.html",
        "hard.html",
        "occupied.html",
        "alias/child/report.html",
    ] {
        assert!(
            publish(&snapshot(), Some(&root.join(name)), false, false, 0, &[]).is_err(),
            "{name}"
        );
    }
    assert_eq!(fs::read(&foreign).unwrap(), b"retain");
    assert_eq!(fs::read(root.join("hard.html")).unwrap(), b"retain");
    assert_eq!(
        fs::read_link(root.join("link.html")).unwrap(),
        Path::new("foreign")
    );
    assert_eq!(
        fs::read_link(root.join("dangling.html")).unwrap(),
        Path::new("missing")
    );
    assert!(root.join("occupied.html").is_dir());
    for name in ["missing", "real/child"] {
        assert!(!root.join(name).exists());
    }
    assert_eq!(fs::read_dir(&root).unwrap().count(), 7);
    fs::remove_dir_all(root).unwrap();
}

#[test]
#[ignore = "replacement-only explicit path policy"]
fn replacement_report_traversal_refuses_before_effects() {
    let root = directory();
    assert!(
        publish(
            &snapshot(),
            Some(&root.join("not-created/../escape.html")),
            false,
            false,
            0,
            &[],
        )
        .is_err()
    );
    assert_eq!(fs::read_dir(&root).unwrap().count(), 0);
    fs::remove_dir_all(root).unwrap();
}
