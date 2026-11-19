use std::{
    ffi::OsStr,
    fs,
    io::{self, Read, Seek, SeekFrom},
    os::unix::fs::{MetadataExt, PermissionsExt, symlink},
    sync::{Arc, Barrier},
    thread,
};

use super::{Attempt, Commit, Destination, Phase, Policy, Retention, Sealed};
use crate::{Access, Directory, Error, FileId, FilePolicy};
use ic_host_artifacts::{Identity, Length};

fn policy(destination: Destination) -> Policy {
    Policy {
        destination,
        permissions: 0o600,
        retention: Retention::Discard,
    }
}

fn sealed(root: &Directory, staging: &str, destination: Destination, bytes: &[u8]) -> Sealed {
    root.stage(
        OsStr::new(staging),
        OsStr::new("final"),
        policy(destination),
    )
    .unwrap()
    .write_from(bytes, Length::Exactly(bytes.len() as u64))
    .unwrap()
    .seal()
    .unwrap()
}

#[test]
fn parent_rename_preserves_custody_and_replace_does_not_follow_final_symlink() {
    let scratch = tempfile::tempdir().unwrap();
    let path = scratch.path().join("root");
    fs::create_dir(&path).unwrap();
    let root = Directory::open(&path, 0).unwrap().unwrap();
    let stage = sealed(&root, "stage", Destination::Replace, b"new");
    let moved = scratch.path().join("moved");
    fs::rename(&path, &moved).unwrap();
    fs::create_dir(&path).unwrap();
    let foreign = scratch.path().join("foreign");
    fs::write(&foreign, b"original").unwrap();
    std::os::unix::fs::symlink(&foreign, moved.join("final")).unwrap();
    let Commit::Committed(receipt) = stage.commit() else {
        panic!("publication failed");
    };
    assert_eq!(receipt.content.bytes, 3);
    assert_eq!(fs::read(moved.join("final")).unwrap(), b"new");
    assert_eq!(fs::read(foreign).unwrap(), b"original");
    assert!(!path.join("final").exists());
}

#[test]
fn concurrent_create_only_has_exactly_one_winner_and_preserves_loser_ownership() {
    let scratch = tempfile::tempdir().unwrap();
    let root = Directory::open(scratch.path(), 0).unwrap().unwrap();
    let a = sealed(&root, "a", Destination::CreateNew, b"a");
    let b = sealed(&root, "b", Destination::CreateNew, b"b");
    let barrier = Arc::new(Barrier::new(2));
    let threads: Vec<_> = [a, b]
        .into_iter()
        .map(|stage| {
            let barrier = barrier.clone();

            thread::spawn(move || {
                barrier.wait();

                match stage.commit() {
                    Commit::Committed(_) => true,
                    Commit::NotPublished {
                        cause: Error::Io(error),
                        stage,
                    } => {
                        assert_eq!(error.kind(), io::ErrorKind::AlreadyExists);
                        stage.abort().unwrap();

                        false
                    }
                    outcome => panic!("unexpected {outcome:?}"),
                }
            })
        })
        .collect();
    assert_eq!(
        threads
            .into_iter()
            .map(|t| usize::from(t.join().unwrap()))
            .sum::<usize>(),
        1
    );
    assert_eq!(fs::read_dir(scratch.path()).unwrap().count(), 1);
}

#[test]
fn foreign_stage_is_neither_published_nor_removed() {
    let scratch = tempfile::tempdir().unwrap();
    let root = Directory::open(scratch.path(), 0).unwrap().unwrap();
    let stage = sealed(&root, "stage", Destination::CreateNew, b"ours");
    fs::rename(scratch.path().join("stage"), scratch.path().join("held")).unwrap();
    fs::write(scratch.path().join("stage"), b"foreign").unwrap();
    let Commit::NotPublished {
        cause: Error::Changed,
        stage,
    } = stage.commit()
    else {
        panic!("accepted replacement");
    };
    assert!(matches!(stage.abort(), Err(Error::Changed)));
    assert_eq!(fs::read(scratch.path().join("stage")).unwrap(), b"foreign");
    assert!(!scratch.path().join("final").exists());
}

#[test]
fn partial_write_short_and_growth_never_gain_seal_authority() {
    struct Broken(bool);
    impl Read for Broken {
        fn read(&mut self, out: &mut [u8]) -> io::Result<usize> {
            if self.0 {
                Err(io::Error::from_raw_os_error(5))
            } else {
                self.0 = true;
                out[0] = b'x';

                Ok(1)
            }
        }
    }
    let scratch = tempfile::tempdir().unwrap();
    let root = Directory::open(scratch.path(), 0).unwrap().unwrap();
    let attempt = root
        .stage(
            OsStr::new("stage"),
            OsStr::new("final"),
            Policy {
                retention: Retention::Retain,
                ..policy(Destination::CreateNew)
            },
        )
        .unwrap();
    let error = attempt
        .write_from(Broken(false), Length::Exactly(2))
        .unwrap_err();
    assert!(matches!(
        error.cause,
        Error::Bytes(ic_host_artifacts::Error::Io(_))
    ));
    assert_eq!(fs::read(scratch.path().join("stage")).unwrap(), b"x");
    for (name, bytes) in [("short", b"".as_slice()), ("growing", b"xx".as_slice())] {
        let error = root
            .stage(
                OsStr::new(name),
                OsStr::new("final"),
                policy(Destination::CreateNew),
            )
            .unwrap()
            .write_from(bytes, Length::Exactly(1))
            .unwrap_err();
        assert!(matches!(
            (name, error.cause),
            (
                "short",
                Error::Bytes(ic_host_artifacts::Error::Short {
                    expected: 1,
                    actual: 0
                })
            ) | (
                "growing",
                Error::Bytes(ic_host_artifacts::Error::Limit { limit: 1 })
            )
        ));
        assert!(error.cleanup.is_none());
        assert!(error.recovery.is_none());
        assert!(!scratch.path().join(name).exists());
    }
    assert!(!scratch.path().join("final").exists());
}

#[test]
fn sync_failures_report_before_and_after_visibility_without_retry() {
    let scratch = tempfile::tempdir().unwrap();
    let root = Directory::open(scratch.path(), 0).unwrap().unwrap();
    let written = root
        .stage(
            OsStr::new("stage"),
            OsStr::new("final"),
            policy(Destination::CreateNew),
        )
        .unwrap()
        .write_from(&b"data"[..], Length::Exactly(4))
        .unwrap();
    let failed = written
        .seal_with(|_| Err(io::Error::from_raw_os_error(5)))
        .unwrap_err();
    assert!(matches!(failed.cause, Error::Io(_)));
    assert!(!scratch.path().join("final").exists());
    let outcome = sealed(&root, "stage", Destination::CreateNew, b"data")
        .commit_with(|_| Err(io::Error::from_raw_os_error(5)), Attempt::remove);
    let Commit::PublishedUnconfirmed {
        phase: Phase::DirectorySync,
        receipt,
        cause: Error::Io(error),
        ..
    } = outcome
    else {
        panic!("lost publication phase");
    };
    assert_eq!(error.raw_os_error(), Some(5));
    assert_eq!(receipt.content.bytes, 4);
    assert_eq!(fs::read(scratch.path().join("final")).unwrap(), b"data");
    assert!(!scratch.path().join("stage").exists());
}

#[test]
fn failed_writer_preserves_primary_and_foreign_cleanup_refusals() {
    struct ReplacingReader {
        parent: std::path::PathBuf,
    }
    impl Read for ReplacingReader {
        fn read(&mut self, _: &mut [u8]) -> io::Result<usize> {
            fs::rename(self.parent.join("stage"), self.parent.join("original"))?;
            fs::write(self.parent.join("stage"), b"foreign")?;

            Err(io::Error::from_raw_os_error(5))
        }
    }
    let scratch = tempfile::tempdir().unwrap();
    let root = Directory::open(scratch.path(), 0).unwrap().unwrap();
    let stage = root
        .stage(
            OsStr::new("stage"),
            OsStr::new("final"),
            policy(Destination::CreateNew),
        )
        .unwrap();
    let refusal = stage
        .write_from(
            ReplacingReader {
                parent: scratch.path().into(),
            },
            Length::Exactly(1),
        )
        .unwrap_err();
    let Error::Bytes(ic_host_artifacts::Error::Io(original)) = refusal.cause else {
        panic!("lost primary");
    };
    assert_eq!(original.raw_os_error(), Some(5));
    assert!(matches!(refusal.cleanup, Some(Error::Changed)));
    assert_eq!(fs::read(scratch.path().join("stage")).unwrap(), b"foreign");
    assert!(!scratch.path().join("final").exists());
}

// SHA-256("data"), independent of the byte admission implementation.
const DATA: Identity = Identity {
    bytes: 4,
    sha256: [
        0x3a, 0x6e, 0xb0, 0x79, 0x0f, 0x39, 0xac, 0x87, 0xc9, 0x4f, 0x38, 0x56, 0xb2, 0xdd, 0x2c,
        0x5d, 0x11, 0x0e, 0x68, 0x11, 0x60, 0x22, 0x61, 0xa9, 0xa9, 0x23, 0xd3, 0xbb, 0x23, 0xad,
        0xc8, 0xb7,
    ],
};

fn retained(root: &Directory) -> std::fs::File {
    let sealed = root
        .stage(
            OsStr::new("stage"),
            OsStr::new("final"),
            Policy {
                retention: Retention::Retain,
                ..policy(Destination::CreateNew)
            },
        )
        .unwrap()
        .write_from(&b"data"[..], Length::Exactly(4))
        .unwrap()
        .seal()
        .unwrap();
    drop(sealed);

    root.open_file(
        OsStr::new("stage"),
        Access::Read,
        FilePolicy {
            forbidden: 0o077,
            single_link: true,
        },
    )
    .unwrap()
    .unwrap()
}

#[test]
fn retained_reopen_publishes_original_inode_bytes_and_mode_after_parent_rename() {
    let scratch = tempfile::tempdir().unwrap();
    let path = scratch.path().join("root");
    fs::create_dir(&path).unwrap();
    let root = Directory::open(&path, 0).unwrap().unwrap();
    let original = retained(&root);
    let id = FileId::from_file(&original).unwrap();
    drop(original);
    drop(root);
    let root = Directory::open(&path, 0).unwrap().unwrap();
    let mut original = root
        .open_file(
            OsStr::new("stage"),
            Access::Read,
            FilePolicy {
                forbidden: 0o077,
                single_link: true,
            },
        )
        .unwrap()
        .unwrap();
    original.seek(SeekFrom::Start(2)).unwrap();
    let written = root
        .admit_retained(
            OsStr::new("stage"),
            OsStr::new("final"),
            &original,
            DATA,
            0o077,
        )
        .unwrap();
    assert_eq!(original.stream_position().unwrap(), 2);
    let moved = scratch.path().join("moved");
    fs::rename(&path, &moved).unwrap();
    fs::create_dir(&path).unwrap();
    let Commit::Committed(receipt) = written.seal().unwrap().commit() else {
        panic!("retained publication failed");
    };
    assert_eq!(receipt.content, DATA);
    let metadata = fs::metadata(moved.join("final")).unwrap();
    assert_eq!(FileId::from_metadata(&metadata), id);
    assert_eq!(metadata.mode() & 0o777, 0o600);
    assert_eq!(metadata.nlink(), 1);
    assert_eq!(fs::read(moved.join("final")).unwrap(), b"data");
    assert!(!moved.join("stage").exists());
    assert!(!path.join("final").exists());
}

#[test]
fn retained_admission_refuses_incomplete_or_mismatched_bytes_without_effects() {
    let scratch = tempfile::tempdir().unwrap();
    let root = Directory::open(scratch.path(), 0).unwrap().unwrap();
    let original = retained(&root);
    let id = FileId::from_file(&original).unwrap();

    for bytes in [b"dat".as_slice(), b"data!", b"evil"] {
        fs::write(scratch.path().join("stage"), bytes).unwrap();
        let error = root
            .admit_retained(
                OsStr::new("stage"),
                OsStr::new("final"),
                &original,
                DATA,
                0o077,
            )
            .unwrap_err();
        match bytes.len() {
            3 => assert!(matches!(
                error,
                Error::Bytes(ic_host_artifacts::Error::Short { .. })
            )),
            5 => assert!(matches!(
                error,
                Error::Bytes(ic_host_artifacts::Error::Limit { .. })
            )),
            _ => assert!(matches!(error, Error::Changed)),
        }
        assert_eq!(fs::read(scratch.path().join("stage")).unwrap(), bytes);
        assert_eq!(
            FileId::from_metadata(&fs::metadata(scratch.path().join("stage")).unwrap()),
            id
        );
        assert!(!scratch.path().join("final").exists());
    }
}

#[test]
fn retained_admission_refuses_unsafe_names_entries_and_foreign_identity() {
    let scratch = tempfile::tempdir().unwrap();
    let root = Directory::open(scratch.path(), 0).unwrap().unwrap();
    let original = retained(&root);
    for (stage, destination) in [
        ("../stage", "final"),
        ("stage", "../final"),
        ("stage", "stage"),
        ("", "final"),
    ] {
        assert!(matches!(
            root.admit_retained(
                OsStr::new(stage),
                OsStr::new(destination),
                &original,
                DATA,
                0o077
            ),
            Err(Error::Name)
        ));
    }
    let stage = scratch.path().join("stage");
    fs::set_permissions(&stage, fs::Permissions::from_mode(0o644)).unwrap();
    assert!(matches!(
        root.admit_retained(
            OsStr::new("stage"),
            OsStr::new("final"),
            &original,
            DATA,
            0o077
        ),
        Err(Error::Permissions { .. })
    ));
    assert_eq!(fs::metadata(&stage).unwrap().mode() & 0o777, 0o644);
    fs::set_permissions(&stage, fs::Permissions::from_mode(0o600)).unwrap();
    fs::hard_link(&stage, scratch.path().join("alias")).unwrap();
    assert!(matches!(
        root.admit_retained(
            OsStr::new("stage"),
            OsStr::new("final"),
            &original,
            DATA,
            0o077
        ),
        Err(Error::Aliased)
    ));
    fs::remove_file(scratch.path().join("alias")).unwrap();
    fs::rename(&stage, scratch.path().join("original")).unwrap();
    symlink("original", &stage).unwrap();
    assert!(matches!(
        root.admit_retained(
            OsStr::new("stage"),
            OsStr::new("final"),
            &original,
            DATA,
            0o077
        ),
        Err(Error::Refused(_))
    ));
    assert!(
        fs::symlink_metadata(&stage)
            .unwrap()
            .file_type()
            .is_symlink()
    );
    fs::remove_file(&stage).unwrap();
    fs::create_dir(&stage).unwrap();
    assert!(matches!(
        root.admit_retained(
            OsStr::new("stage"),
            OsStr::new("final"),
            &original,
            DATA,
            0o077
        ),
        Err(Error::Kind)
    ));
    fs::remove_dir(&stage).unwrap();
    assert!(matches!(
        root.admit_retained(
            OsStr::new("stage"),
            OsStr::new("final"),
            &original,
            DATA,
            0o077
        ),
        Err(Error::Changed)
    ));
    fs::write(&stage, b"data").unwrap();
    fs::set_permissions(&stage, fs::Permissions::from_mode(0o600)).unwrap();
    assert!(matches!(
        root.admit_retained(
            OsStr::new("stage"),
            OsStr::new("final"),
            &original,
            DATA,
            0o077
        ),
        Err(Error::Changed)
    ));
    assert_eq!(fs::read(&stage).unwrap(), b"data");
    assert_eq!(fs::read(scratch.path().join("original")).unwrap(), b"data");
    assert!(!scratch.path().join("final").exists());
}

#[test]
fn retained_drop_and_occupied_destination_preserve_recovery_evidence() {
    let scratch = tempfile::tempdir().unwrap();
    let root = Directory::open(scratch.path(), 0).unwrap().unwrap();
    let original = retained(&root);
    let admit = || {
        root.admit_retained(
            OsStr::new("stage"),
            OsStr::new("final"),
            &original,
            DATA,
            0o077,
        )
        .unwrap()
    };
    drop(admit());
    drop(admit().seal().unwrap());
    fs::write(scratch.path().join("final"), b"occupied").unwrap();
    let Commit::NotPublished {
        cause: Error::Io(error),
        stage,
    } = admit().seal().unwrap().commit()
    else {
        panic!("replaced destination");
    };
    assert_eq!(error.kind(), io::ErrorKind::AlreadyExists);
    drop(stage);
    assert_eq!(fs::read(scratch.path().join("stage")).unwrap(), b"data");
    assert_eq!(fs::read(scratch.path().join("final")).unwrap(), b"occupied");
    assert_eq!(
        FileId::from_metadata(&fs::metadata(scratch.path().join("stage")).unwrap()),
        FileId::from_file(&original).unwrap()
    );
    // Removal requires a separate explicit request, never an automatic retry.
    admit().abort().unwrap();
    assert!(!scratch.path().join("stage").exists());
    assert_eq!(fs::read(scratch.path().join("final")).unwrap(), b"occupied");
}

#[test]
fn retained_stage_substitution_refuses_seal_and_commit_without_cleanup() {
    for after_seal in [false, true] {
        let scratch = tempfile::tempdir().unwrap();
        let root = Directory::open(scratch.path(), 0).unwrap().unwrap();
        let original = retained(&root);
        let written = root
            .admit_retained(
                OsStr::new("stage"),
                OsStr::new("final"),
                &original,
                DATA,
                0o077,
            )
            .unwrap();
        let replace = || {
            fs::rename(
                scratch.path().join("stage"),
                scratch.path().join("original"),
            )
            .unwrap();
            fs::write(scratch.path().join("stage"), b"foreign").unwrap();
        };
        if after_seal {
            let sealed = written.seal().unwrap();
            replace();
            let Commit::NotPublished {
                cause: Error::Changed,
                stage,
            } = sealed.commit()
            else {
                panic!("published substitute");
            };
            drop(stage);
        } else {
            replace();
            let failed = written.seal().unwrap_err();
            assert!(matches!(failed.cause, Error::Changed));
            assert!(failed.cleanup.is_none());
            assert_eq!(failed.recovery.as_deref(), Some(OsStr::new("stage")));
        }
        assert_eq!(fs::read(scratch.path().join("stage")).unwrap(), b"foreign");
        assert_eq!(fs::read(scratch.path().join("original")).unwrap(), b"data");
        assert!(!scratch.path().join("final").exists());
    }
}

#[test]
fn retained_sync_and_cleanup_failures_keep_publication_phase_and_original_causes() {
    let scratch = tempfile::tempdir().unwrap();
    let root = Directory::open(scratch.path(), 0).unwrap().unwrap();
    let original = retained(&root);
    let admit = || {
        root.admit_retained(
            OsStr::new("stage"),
            OsStr::new("final"),
            &original,
            DATA,
            0o077,
        )
        .unwrap()
    };
    let failure = admit()
        .seal_with(|_| Err(io::Error::from_raw_os_error(5)))
        .unwrap_err();
    assert!(matches!(failure.cause, Error::Io(ref error) if error.raw_os_error() == Some(5)));
    assert!(failure.cleanup.is_none());
    assert_eq!(failure.recovery.as_deref(), Some(OsStr::new("stage")));
    assert!(!scratch.path().join("final").exists());
    let outcome = admit().seal().unwrap().commit_with(
        |_| Err(io::Error::from_raw_os_error(5)),
        |_| Err(io::Error::from_raw_os_error(13).into()),
    );
    let Commit::PublishedUnconfirmed {
        receipt,
        phase: Phase::StageRemoval,
        cause: Error::Io(error),
        sync_error: Some(sync_error),
    } = outcome
    else {
        panic!("lost post-visibility failures");
    };
    assert_eq!(receipt.content, DATA);
    assert_eq!(error.raw_os_error(), Some(13));
    assert_eq!(sync_error.raw_os_error(), Some(5));
    assert_eq!(fs::read(scratch.path().join("stage")).unwrap(), b"data");
    assert_eq!(fs::read(scratch.path().join("final")).unwrap(), b"data");
    assert_eq!(
        FileId::from_metadata(&fs::metadata(scratch.path().join("final")).unwrap()),
        FileId::from_file(&original).unwrap()
    );
    // A visible hard-link pair is reconciliation input, not a fresh recovery stage.
    assert!(matches!(
        root.admit_retained(
            OsStr::new("stage"),
            OsStr::new("final"),
            &original,
            DATA,
            0o077
        ),
        Err(Error::Aliased)
    ));
}
