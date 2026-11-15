use super::*;
use crate::{
    durable::{PublicationMode, WriteOptions, write_at_validated_with},
    testing::FixtureDirectory,
};
#[cfg(target_os = "linux")]
use std::os::unix::ffi::OsStringExt;
use std::{
    fs,
    io::Write,
    os::unix::fs::{PermissionsExt as _, symlink},
};

const OBSERVE: DirectoryOptions = DirectoryOptions {
    create_permissions: None,
    forbidden_permissions: 0o022,
};
const CREATE: DirectoryOptions = DirectoryOptions {
    create_permissions: Some(0o700),
    ..OBSERVE
};

fn write(entry: &FileEntry, bytes: &[u8]) {
    let parent = entry.parent().open_syncable().unwrap();
    write_at_validated_with(
        parent.as_fd(),
        entry.file_name(),
        WriteOptions {
            mode: PublicationMode::Replace,
            permissions: 0o600,
        },
        |file| file.write_all(bytes),
        |()| Ok(()),
    )
    .unwrap();
}

#[test]
fn managed_round_trip_uses_owner_only_modes() {
    let fixture = FixtureDirectory::new(&std::env::temp_dir(), "capability-round-trip").unwrap();
    let root_path = fixture.path().join("root");
    let root = Directory::open_root(&root_path, CREATE).unwrap().unwrap();
    let entry = root
        .resolve_parent(Path::new("nns/ic/report"), CREATE)
        .unwrap()
        .unwrap();
    write(&entry, b"evidence");
    let file = entry.open_regular_file().unwrap().unwrap();
    assert_eq!(root.permissions().unwrap(), 0o700);
    assert_eq!(file.metadata().unwrap().permissions().mode() & 0o777, 0o600);
    assert_eq!(crate::read::read_opened_file(file, 8).unwrap(), b"evidence");
}

#[test]
fn missing_root_and_file_are_observed_without_creation() {
    let fixture = FixtureDirectory::new(&std::env::temp_dir(), "capability-missing").unwrap();
    let missing = fixture.path().join("missing");
    assert!(Directory::open_root(&missing, OBSERVE).unwrap().is_none());
    assert!(!missing.exists());
    let root = Directory::open_root(fixture.path(), OBSERVE)
        .unwrap()
        .unwrap();
    assert!(
        root.resolve_parent(Path::new("missing/file"), OBSERVE)
            .unwrap()
            .is_none()
    );
    let entry = root
        .resolve_parent(Path::new("file"), OBSERVE)
        .unwrap()
        .unwrap();
    assert!(entry.open_regular_file().unwrap().is_none());
    assert!(!entry.remove_file_and_sync().unwrap());
}

#[test]
fn managed_path_rejects_escape() {
    let fixture = FixtureDirectory::new(&std::env::temp_dir(), "capability-escape").unwrap();
    let root = Directory::open_root(fixture.path(), OBSERVE)
        .unwrap()
        .unwrap();
    for path in ["", ".", "../file", "child/../file", "/outside"] {
        assert!(matches!(
            root.resolve_parent(Path::new(path), CREATE),
            Err(CapabilityError::Refused {
                reason: Refusal::InvalidPath,
                ..
            })
        ));
    }
    assert!(matches!(
        Directory::open_root(&fixture.path().join("../outside"), CREATE),
        Err(CapabilityError::Refused {
            reason: Refusal::InvalidPath,
            ..
        })
    ));
    assert!(!fixture.path().join("child").exists());
}

#[test]
fn managed_path_rejects_symlinked_parent_and_file() {
    let fixture = FixtureDirectory::new(&std::env::temp_dir(), "capability-links").unwrap();
    let root_path = fixture.path().join("root");
    let root = Directory::open_root(&root_path, CREATE).unwrap().unwrap();
    let outside = fixture.path().join("outside");
    fs::create_dir(&outside).unwrap();
    fs::write(outside.join("target"), b"outside").unwrap();
    symlink(&outside, root_path.join("linked-dir")).unwrap();
    symlink(outside.join("target"), root_path.join("linked-file")).unwrap();
    symlink("absent", root_path.join("dangling")).unwrap();
    assert!(matches!(
        root.open_directory(OsStr::new("dangling"), OBSERVE),
        Err(CapabilityError::Refused {
            reason: Refusal::SymbolicLink,
            ..
        })
    ));
    assert!(matches!(
        root.resolve_parent(Path::new("linked-dir/file"), CREATE),
        Err(CapabilityError::Refused {
            reason: Refusal::SymbolicLink,
            ..
        })
    ));
    let entry = root
        .resolve_parent(Path::new("linked-file"), OBSERVE)
        .unwrap()
        .unwrap();
    assert!(matches!(
        entry.open_regular_file(),
        Err(CapabilityError::Refused {
            reason: Refusal::SymbolicLink,
            ..
        })
    ));
    assert!(
        matches!(entry.create_new_file(0o600), Err(CapabilityError::Io { operation: Operation::CreateFile, source, .. }) if source.kind() == io::ErrorKind::AlreadyExists)
    );
    let alias = fixture.path().join("root-alias");
    symlink(&root_path, &alias).unwrap();
    assert!(matches!(
        Directory::open_root(&alias, CREATE),
        Err(CapabilityError::Refused {
            reason: Refusal::SymbolicLink,
            ..
        })
    ));
    assert!(!outside.join("file").exists());
}

#[test]
fn managed_publication_stays_with_the_admitted_parent_after_path_replacement() {
    let fixture = FixtureDirectory::new(&std::env::temp_dir(), "capability-parent-move").unwrap();
    let root = Directory::open_root(fixture.path(), OBSERVE)
        .unwrap()
        .unwrap();
    let entry = root
        .resolve_parent(Path::new("parent/report"), CREATE)
        .unwrap()
        .unwrap();
    write(&entry, b"complete");
    fs::rename(fixture.path().join("parent"), fixture.path().join("moved")).unwrap();
    fs::create_dir(fixture.path().join("parent")).unwrap();
    fs::write(
        fixture.path().join("parent/report"),
        b"replacement evidence",
    )
    .unwrap();
    write(&entry, b"new complete");
    assert_eq!(
        fs::read(fixture.path().join("moved/report")).unwrap(),
        b"new complete"
    );
    assert_eq!(
        fs::read(fixture.path().join("parent/report")).unwrap(),
        b"replacement evidence"
    );
    assert!(entry.remove_file_and_sync().unwrap());
    assert!(!fixture.path().join("moved/report").exists());
    assert_eq!(
        fs::read(fixture.path().join("parent/report")).unwrap(),
        b"replacement evidence"
    );
}

#[test]
fn readable_cache_tree_supports_load_and_publication_without_permission_repair() {
    let fixture = FixtureDirectory::new(&std::env::temp_dir(), "capability-readable").unwrap();
    fs::set_permissions(fixture.path(), fs::Permissions::from_mode(0o755)).unwrap();
    let root = Directory::open_root(fixture.path(), OBSERVE)
        .unwrap()
        .unwrap();
    let entry = root
        .resolve_parent(Path::new("report"), OBSERVE)
        .unwrap()
        .unwrap();
    fs::write(fixture.path().join("report"), b"evidence").unwrap();
    fs::set_permissions(
        fixture.path().join("report"),
        fs::Permissions::from_mode(0o644),
    )
    .unwrap();
    write(&entry, b"replacement");
    assert_eq!(root.permissions().unwrap(), 0o755);
    assert_eq!(
        fs::metadata(fixture.path().join("report"))
            .unwrap()
            .permissions()
            .mode()
            & 0o777,
        0o600
    );
}

#[test]
fn directory_permission_settings_remain_explicit() {
    let fixture =
        FixtureDirectory::new(&std::env::temp_dir(), "capability-permission-policy").unwrap();
    fs::create_dir(fixture.path().join("shared")).unwrap();
    fs::set_permissions(
        fixture.path().join("shared"),
        fs::Permissions::from_mode(0o775),
    )
    .unwrap();
    let root = Directory::open_root(fixture.path(), OBSERVE)
        .unwrap()
        .unwrap();
    assert!(matches!(
        root.resolve_parent(Path::new("shared/file"), OBSERVE),
        Err(CapabilityError::Permissions {
            actual: 0o775,
            forbidden: 0o022,
            ..
        })
    ));
    let allow_shared = DirectoryOptions {
        forbidden_permissions: 0,
        ..OBSERVE
    };
    assert!(
        root.resolve_parent(Path::new("shared/file"), allow_shared)
            .unwrap()
            .is_some()
    );
    assert_eq!(
        fs::metadata(fixture.path().join("shared"))
            .unwrap()
            .permissions()
            .mode()
            & 0o777,
        0o775
    );
}

#[test]
fn exclusive_creation_retains_existing_bytes_and_native_cause() {
    let fixture = FixtureDirectory::new(&std::env::temp_dir(), "capability-exclusive").unwrap();
    let root = Directory::open_root(fixture.path(), OBSERVE)
        .unwrap()
        .unwrap();
    let entry = root
        .resolve_parent(Path::new("lock"), OBSERVE)
        .unwrap()
        .unwrap();
    entry
        .create_new_file(0o600)
        .unwrap()
        .write_all(b"intent")
        .unwrap();
    assert!(
        matches!(entry.create_new_file(0o600), Err(CapabilityError::Io { operation: Operation::CreateFile, source, .. }) if source.kind() == io::ErrorKind::AlreadyExists)
    );
    assert_eq!(fs::read(fixture.path().join("lock")).unwrap(), b"intent");
}

#[test]
fn directory_creation_rechecks_already_existing_entries() {
    let fixture = FixtureDirectory::new(&std::env::temp_dir(), "capability-create-race").unwrap();
    let root = Directory::open_root(fixture.path(), OBSERVE)
        .unwrap()
        .unwrap();
    fs::create_dir(fixture.path().join("directory")).unwrap();
    fs::set_permissions(
        fixture.path().join("directory"),
        fs::Permissions::from_mode(0o700),
    )
    .unwrap();
    assert_eq!(
        root.create_component(OsStr::new("directory"), 0o700, 0o022)
            .unwrap()
            .permissions()
            .unwrap(),
        0o700
    );
    fs::write(fixture.path().join("file"), b"existing").unwrap();
    symlink("directory", fixture.path().join("link")).unwrap();
    for (name, expected) in [
        ("file", Refusal::NotDirectory),
        ("link", Refusal::SymbolicLink),
    ] {
        assert!(
            matches!(root.create_component(OsStr::new(name), 0o700, 0o022), Err(CapabilityError::Refused { reason, .. }) if reason == expected)
        );
    }
    fs::set_permissions(
        fixture.path().join("directory"),
        fs::Permissions::from_mode(0o777),
    )
    .unwrap();
    assert!(matches!(
        root.create_component(OsStr::new("directory"), 0o700, 0o022),
        Err(CapabilityError::Permissions { .. })
    ));
    assert_eq!(fs::read(fixture.path().join("file")).unwrap(), b"existing");
}

#[test]
fn removal_preserves_completed_effect_and_original_sync_cause() {
    let fixture = FixtureDirectory::new(&std::env::temp_dir(), "capability-remove-sync").unwrap();
    let root = Directory::open_root(fixture.path(), OBSERVE)
        .unwrap()
        .unwrap();
    let entry = root
        .resolve_parent(Path::new("file"), OBSERVE)
        .unwrap()
        .unwrap();
    write(&entry, b"evidence");
    let failure = entry
        .remove_file_and_sync_with(|parent| {
            Err(super::failure(
                Operation::SyncDirectory,
                &parent.display,
                io::Error::from_raw_os_error(rustix::io::Errno::IO.raw_os_error()),
            ))
        })
        .unwrap_err();
    assert!(failure.removed);
    assert!(!fixture.path().join("file").exists());
    assert!(
        matches!(failure.source, CapabilityError::Io { operation: Operation::SyncDirectory, source, .. } if source.raw_os_error() == Some(rustix::io::Errno::IO.raw_os_error()))
    );
    fs::create_dir(fixture.path().join("file")).unwrap();
    let failure = entry
        .remove_file_and_sync_with(|_| panic!("unlink failed; sync must not run"))
        .unwrap_err();
    assert!(!failure.removed);
    assert!(matches!(
        failure.source,
        CapabilityError::Io {
            operation: Operation::RemoveFile,
            ..
        }
    ));
    assert!(fixture.path().join("file").is_dir());
}

#[test]
fn entries_are_bounded_without_candidate_or_unicode_policy() {
    let fixture = FixtureDirectory::new(&std::env::temp_dir(), "capability-entry-budget").unwrap();
    let root = Directory::open_root(fixture.path(), OBSERVE)
        .unwrap()
        .unwrap();
    assert!(root.entries(0).unwrap().next().is_none());
    #[cfg(target_os = "linux")]
    let expected = OsString::from_vec(b"non-utf8-\xff".to_vec());
    #[cfg(not(target_os = "linux"))]
    let expected = OsString::from("data");
    fs::write(fixture.path().join(&expected), b"data").unwrap();
    assert!(matches!(
        root.entries(0).unwrap().next(),
        Some(Err(CapabilityError::EntryLimit { maximum: 0, .. }))
    ));
    let exact = root
        .entries(1)
        .unwrap()
        .collect::<Result<Vec<_>, _>>()
        .unwrap();
    assert_eq!(
        exact,
        vec![DirectoryEntry {
            name: expected,
            kind: EntryKind::File
        }]
    );
    fs::create_dir(fixture.path().join("directory")).unwrap();
    let mut bounded = root.entries(1).unwrap();
    assert!(bounded.next().unwrap().is_ok());
    assert!(matches!(
        bounded.next(),
        Some(Err(CapabilityError::EntryLimit { maximum: 1, .. }))
    ));
    assert!(bounded.next().is_none());
    assert!(bounded.next().is_none());
}

#[test]
fn special_files_are_refused_without_waiting_for_a_fifo_writer() {
    let fixture = FixtureDirectory::new(&std::env::temp_dir(), "capability-fifo").unwrap();
    let fifo = fixture.path().join("fifo");
    crate::testing::create_fifo(&fifo, None).unwrap();
    let root = Directory::open_root(fixture.path(), OBSERVE)
        .unwrap()
        .unwrap();
    let entry = root
        .resolve_parent(Path::new("fifo"), OBSERVE)
        .unwrap()
        .unwrap();
    assert!(matches!(
        entry.open_regular_file(),
        Err(CapabilityError::Refused {
            reason: Refusal::NotRegularFile,
            ..
        })
    ));
    let entries = root
        .entries(1)
        .unwrap()
        .collect::<Result<Vec<_>, _>>()
        .unwrap();
    assert_eq!(entries[0].kind, EntryKind::Other);
}

#[test]
fn configured_regular_file_admission_preserves_modes_and_absence() {
    let fixture = FixtureDirectory::new(&std::env::temp_dir(), "capability-file-modes").unwrap();
    let root = Directory::open_root(fixture.path(), OBSERVE)
        .unwrap()
        .unwrap();
    let entry = root
        .resolve_parent(Path::new("file"), OBSERVE)
        .unwrap()
        .unwrap();
    assert!(
        entry
            .open_regular_file_with_permissions(0o022)
            .unwrap()
            .is_none()
    );
    fs::write(fixture.path().join("file"), b"data").unwrap();
    fs::set_permissions(
        fixture.path().join("file"),
        fs::Permissions::from_mode(0o620),
    )
    .unwrap();
    assert!(matches!(
        entry.open_regular_file_with_permissions(0o022),
        Err(CapabilityError::Permissions { actual: 0o620, .. })
    ));
    assert!(
        entry
            .open_regular_file_with_permissions(0)
            .unwrap()
            .is_some()
    );
    assert_eq!(
        fs::metadata(fixture.path().join("file"))
            .unwrap()
            .permissions()
            .mode()
            & 0o777,
        0o620
    );
    assert!(entry.open_regular_file_with_permissions(0o1000).is_err());
    fs::remove_file(fixture.path().join("file")).unwrap();
    symlink("absent", fixture.path().join("file")).unwrap();
    assert!(matches!(
        entry.open_regular_file_with_permissions(0),
        Err(CapabilityError::Refused {
            reason: Refusal::SymbolicLink,
            ..
        })
    ));
}

#[test]
fn update_open_preserves_absence_bytes_identity_and_caller_write_position() {
    use crate::identity::FileIdentity;
    use std::io::{Read as _, Seek as _, SeekFrom};

    let fixture = FixtureDirectory::new(&std::env::temp_dir(), "capability-update").unwrap();
    let root = Directory::open_root(fixture.path(), OBSERVE)
        .unwrap()
        .unwrap();
    let entry = root
        .resolve_parent(Path::new("ledger"), OBSERVE)
        .unwrap()
        .unwrap();
    assert!(entry.open_regular_file_for_update(0o077).unwrap().is_none());
    assert!(!fixture.path().join("ledger").exists());
    write(&entry, b"intent");
    let identity = FileIdentity::from_file(&entry.open_regular_file().unwrap().unwrap()).unwrap();
    let mut file = entry.open_regular_file_for_update(0o077).unwrap().unwrap();
    assert_eq!(FileIdentity::from_file(&file).unwrap(), identity);
    assert_eq!(file.stream_position().unwrap(), 0);
    assert_eq!(fs::read(fixture.path().join("ledger")).unwrap(), b"intent");
    let flags = rustix::fs::fcntl_getfl(&file).unwrap();
    assert!(flags.contains(rustix::fs::OFlags::RDWR));
    assert!(flags.contains(rustix::fs::OFlags::NONBLOCK));
    assert!(!flags.contains(rustix::fs::OFlags::APPEND));
    assert!(
        rustix::io::fcntl_getfd(&file)
            .unwrap()
            .contains(rustix::io::FdFlags::CLOEXEC)
    );
    file.seek(SeekFrom::End(0)).unwrap();
    file.write_all(b"/next").unwrap();
    file.sync_all().unwrap();
    file.rewind().unwrap();
    let mut bytes = Vec::new();
    file.read_to_end(&mut bytes).unwrap();
    assert_eq!(bytes, b"intent/next");
    assert_eq!(FileIdentity::from_file(&file).unwrap(), identity);
    // Reading APIs retain read-only access even with permission admission.
    for mut read_only in [
        entry.open_regular_file().unwrap().unwrap(),
        entry
            .open_regular_file_with_permissions(0o077)
            .unwrap()
            .unwrap(),
    ] {
        assert!(read_only.write_all(b"X").is_err());
    }
    assert_eq!(fs::read(fixture.path().join("ledger")).unwrap(), bytes);
}

#[test]
fn update_open_keeps_held_parent_after_ancestor_and_root_replacement() {
    let fixture =
        FixtureDirectory::new(&std::env::temp_dir(), "capability-update-renamed").unwrap();
    let root_path = fixture.path().join("root");
    let root = Directory::open_root(&root_path, CREATE).unwrap().unwrap();
    let entry = root
        .resolve_parent(Path::new("child/ledger"), CREATE)
        .unwrap()
        .unwrap();
    write(&entry, b"original");
    fs::rename(root_path.join("child"), root_path.join("retained")).unwrap();
    fs::create_dir(root_path.join("child")).unwrap();
    fs::write(root_path.join("child/ledger"), b"child replacement").unwrap();
    let moved = fixture.path().join("moved");
    fs::rename(&root_path, &moved).unwrap();
    fs::create_dir_all(root_path.join("child")).unwrap();
    fs::write(root_path.join("child/ledger"), b"root replacement").unwrap();
    let mut file = entry.open_regular_file_for_update(0o077).unwrap().unwrap();
    file.write_all(b"updated!").unwrap();
    file.sync_all().unwrap();
    assert_eq!(
        fs::read(moved.join("retained/ledger")).unwrap(),
        b"updated!"
    );
    assert_eq!(
        fs::read(moved.join("child/ledger")).unwrap(),
        b"child replacement"
    );
    assert_eq!(
        fs::read(root_path.join("child/ledger")).unwrap(),
        b"root replacement"
    );
}

#[test]
fn update_open_refuses_permissions_without_repair_or_content_change() {
    let fixture = FixtureDirectory::new(&std::env::temp_dir(), "capability-update-modes").unwrap();
    let root = Directory::open_root(fixture.path(), OBSERVE)
        .unwrap()
        .unwrap();
    let entry = root
        .resolve_parent(Path::new("ledger"), OBSERVE)
        .unwrap()
        .unwrap();
    write(&entry, b"intent");
    fs::set_permissions(
        fixture.path().join("ledger"),
        fs::Permissions::from_mode(0o620),
    )
    .unwrap();
    assert!(matches!(
        entry.open_regular_file_for_update(0o022),
        Err(CapabilityError::Permissions {
            actual: 0o620,
            forbidden: 0o022,
            ..
        })
    ));
    assert!(entry.open_regular_file_for_update(0).unwrap().is_some());
    assert!(
        matches!(entry.open_regular_file_for_update(0o1000), Err(CapabilityError::Io { operation: Operation::Open, source, .. }) if source.kind() == io::ErrorKind::InvalidInput)
    );
    assert_eq!(
        fs::metadata(fixture.path().join("ledger"))
            .unwrap()
            .permissions()
            .mode()
            & 0o777,
        0o620
    );
    assert_eq!(fs::read(fixture.path().join("ledger")).unwrap(), b"intent");
}

#[test]
fn update_open_refuses_links_directories_fifos_and_sockets() {
    use std::os::unix::net::UnixListener;

    let fixture = FixtureDirectory::new(&std::env::temp_dir(), "capability-update-types").unwrap();
    let root = Directory::open_root(fixture.path(), OBSERVE)
        .unwrap()
        .unwrap();
    fs::write(fixture.path().join("original"), b"original").unwrap();
    symlink("original", fixture.path().join("link")).unwrap();
    symlink("missing", fixture.path().join("dangling")).unwrap();
    fs::create_dir(fixture.path().join("directory")).unwrap();
    crate::testing::create_fifo(&fixture.path().join("fifo"), None).unwrap();
    let _socket = UnixListener::bind(fixture.path().join("socket")).unwrap();
    for (name, expected) in [
        ("link", Refusal::SymbolicLink),
        ("dangling", Refusal::SymbolicLink),
        ("directory", Refusal::NotRegularFile),
        ("fifo", Refusal::NotRegularFile),
        ("socket", Refusal::NotRegularFile),
    ] {
        let entry = root
            .resolve_parent(Path::new(name), OBSERVE)
            .unwrap()
            .unwrap();
        assert!(
            matches!(entry.open_regular_file_for_update(0), Err(CapabilityError::Refused { reason, .. }) if reason == expected),
            "{name}"
        );
    }
    assert_eq!(
        fs::read(fixture.path().join("original")).unwrap(),
        b"original"
    );
}

#[test]
fn update_descriptor_does_not_retarget_and_link_policy_stays_with_caller() {
    use std::os::unix::fs::MetadataExt as _;

    let fixture =
        FixtureDirectory::new(&std::env::temp_dir(), "capability-update-identity").unwrap();
    let root = Directory::open_root(fixture.path(), OBSERVE)
        .unwrap()
        .unwrap();
    let entry = root
        .resolve_parent(Path::new("ledger"), OBSERVE)
        .unwrap()
        .unwrap();
    write(&entry, b"original");
    fs::hard_link(fixture.path().join("ledger"), fixture.path().join("alias")).unwrap();
    let mut file = entry.open_regular_file_for_update(0o077).unwrap().unwrap();
    assert_eq!(file.metadata().unwrap().nlink(), 2);
    fs::rename(
        fixture.path().join("ledger"),
        fixture.path().join("retained"),
    )
    .unwrap();
    fs::write(fixture.path().join("ledger"), b"replacement").unwrap();
    file.write_all(b"updated!").unwrap();
    assert_eq!(
        fs::read(fixture.path().join("retained")).unwrap(),
        b"updated!"
    );
    assert_eq!(fs::read(fixture.path().join("alias")).unwrap(), b"updated!");
    assert_eq!(
        fs::read(fixture.path().join("ledger")).unwrap(),
        b"replacement"
    );
}
