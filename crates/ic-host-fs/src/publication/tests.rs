use std::ffi::OsStr;
use std::fs;
use std::io::{self, Read};
use std::sync::{Arc, Barrier};
use std::thread;

use super::{Commit, Destination, Phase, Policy, Retention, Sealed};
use crate::{Directory, Error};
use ic_host_artifacts::Length;

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
        assert!(
            root.stage(
                OsStr::new(name),
                OsStr::new("final"),
                policy(Destination::CreateNew)
            )
            .unwrap()
            .write_from(bytes, Length::Exactly(1))
            .is_err()
        );
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
        .commit_with(|_| Err(io::Error::from_raw_os_error(5)));
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
