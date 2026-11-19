use crate::{Directory, DirectoryPublishError, Error};
use std::{
    ffi::OsStr,
    fs,
    io::{self, Write},
    os::unix::fs::symlink,
};

fn tree(parent: &Directory, name: &str, bytes: &[u8]) -> Directory {
    let directory = parent.create_directory(OsStr::new(name), 0o700, 0).unwrap();
    let mut file = directory.create_file(OsStr::new("member"), 0o600).unwrap();
    file.write_all(bytes).unwrap();
    file.sync_all().unwrap();

    directory
}

#[test]
fn held_parent_and_tree_survive_parent_rename() {
    let scratch = tempfile::tempdir().unwrap();
    let original = scratch.path().join("parent");
    let moved = scratch.path().join("moved");
    fs::create_dir(&original).unwrap();
    let parent = Directory::open(&original, 0).unwrap().unwrap();
    let stage = tree(&parent, "stage", b"complete");
    fs::rename(&original, &moved).unwrap();
    fs::create_dir(&original).unwrap();
    let receipt = parent
        .publish_directory(OsStr::new("stage"), &stage, OsStr::new("final"))
        .unwrap();
    assert_eq!(receipt.parent, parent.identity().unwrap());
    assert_eq!(receipt.directory, stage.identity().unwrap());
    assert_eq!(receipt.destination, "final");
    assert_eq!(fs::read(moved.join("final/member")).unwrap(), b"complete");
    assert!(!moved.join("stage").exists());
    assert_eq!(fs::read_dir(original).unwrap().count(), 0);
    assert!(matches!(
        parent.publish_directory(OsStr::new("stage"), &stage, OsStr::new("final")),
        Err(DirectoryPublishError::NotPublished(Error::Changed))
    ));
}

#[test]
fn occupied_names_never_replace_files_directories_or_links() {
    let scratch = tempfile::tempdir().unwrap();
    let parent = Directory::open(scratch.path(), 0).unwrap().unwrap();
    let stage = tree(&parent, "stage", b"staged");
    fs::create_dir(scratch.path().join("empty")).unwrap();
    fs::write(scratch.path().join("file"), b"foreign").unwrap();
    symlink("absent", scratch.path().join("link")).unwrap();
    for destination in ["empty", "file", "link", "stage", "../escape"] {
        assert!(matches!(
            parent.publish_directory(OsStr::new("stage"), &stage, OsStr::new(destination)),
            Err(DirectoryPublishError::NotPublished(_))
        ));
        assert_eq!(
            fs::read(scratch.path().join("stage/member")).unwrap(),
            b"staged"
        );
    }
    assert!(scratch.path().join("empty").is_dir());
    assert_eq!(fs::read(scratch.path().join("file")).unwrap(), b"foreign");
    assert_eq!(
        fs::read_link(scratch.path().join("link")).unwrap(),
        std::path::Path::new("absent")
    );
    fs::rename(
        scratch.path().join("stage"),
        scratch.path().join("retained"),
    )
    .unwrap();
    symlink("retained", scratch.path().join("stage")).unwrap();
    assert!(matches!(
        parent.publish_directory(OsStr::new("stage"), &stage, OsStr::new("final")),
        Err(DirectoryPublishError::NotPublished(Error::Refused(_)))
    ));
    assert_eq!(
        fs::read(scratch.path().join("retained/member")).unwrap(),
        b"staged"
    );
    assert!(!scratch.path().join("final").exists());
}

#[test]
fn substitution_after_root_sync_refuses_without_removing_either_tree() {
    let scratch = tempfile::tempdir().unwrap();
    let parent = Directory::open(scratch.path(), 0).unwrap().unwrap();
    let stage = tree(&parent, "stage", b"original");
    let mut calls = 0;
    let result = parent.publish_directory_with(
        OsStr::new("stage"),
        &stage,
        OsStr::new("final"),
        &mut |held| {
            calls += 1;
            held.sync()?;
            fs::rename(
                scratch.path().join("stage"),
                scratch.path().join("retained"),
            )?;
            tree(&parent, "stage", b"foreign");

            Ok(())
        },
    );
    assert!(matches!(
        result,
        Err(DirectoryPublishError::NotPublished(Error::Changed))
    ));
    assert_eq!(calls, 1);
    assert_eq!(
        fs::read(scratch.path().join("retained/member")).unwrap(),
        b"original"
    );
    assert_eq!(
        fs::read(scratch.path().join("stage/member")).unwrap(),
        b"foreign"
    );
    assert!(!scratch.path().join("final").exists());
}

#[test]
fn sync_failures_distinguish_staging_from_visible_publication() {
    for fail_at in [0, 1] {
        let scratch = tempfile::tempdir().unwrap();
        let parent = Directory::open(scratch.path(), 0).unwrap().unwrap();
        let stage = tree(&parent, "stage", b"complete");
        let mut calls = 0;
        let result = parent.publish_directory_with(
            OsStr::new("stage"),
            &stage,
            OsStr::new("final"),
            &mut |held| {
                let current = calls;
                calls += 1;

                if current == fail_at {
                    Err(io::Error::from_raw_os_error(5))
                } else {
                    held.sync()
                }
            },
        );
        if fail_at == 0 {
            assert!(
                matches!(result, Err(DirectoryPublishError::NotPublished(Error::Io(cause))) if cause.raw_os_error() == Some(5))
            );
            assert!(!scratch.path().join("final").exists());
            assert_eq!(
                fs::read(scratch.path().join("stage/member")).unwrap(),
                b"complete"
            );
        } else {
            let Err(DirectoryPublishError::PublishedUnconfirmed { receipt, cause }) = result else {
                panic!("expected visible publication");
            };
            assert_eq!(cause.raw_os_error(), Some(5));
            assert_eq!(receipt.directory, stage.identity().unwrap());
            assert_eq!(receipt.destination, "final");
            assert!(!scratch.path().join("stage").exists());
            assert_eq!(
                fs::read(scratch.path().join("final/member")).unwrap(),
                b"complete"
            );
        }
        assert_eq!(calls, fail_at + 1);
    }
}

#[test]
fn competing_publishers_have_one_winner_and_retain_loser() {
    let scratch = tempfile::tempdir().unwrap();
    let parent = Directory::open(scratch.path(), 0).unwrap().unwrap();
    let left = tree(&parent, "left", b"left");
    let right = tree(&parent, "right", b"right");
    let barrier = std::sync::Barrier::new(2);
    let results = std::thread::scope(|scope| {
        let attempts: Vec<_> = [("left", left), ("right", right)]
            .into_iter()
            .map(|(name, stage)| {
                let parent = &parent;
                let barrier = &barrier;

                scope.spawn(move || {
                    barrier.wait();

                    (
                        name,
                        parent.publish_directory(OsStr::new(name), &stage, OsStr::new("final")),
                    )
                })
            })
            .collect();

        attempts
            .into_iter()
            .map(|attempt| attempt.join().unwrap())
            .collect::<Vec<_>>()
    });
    assert_eq!(
        results.iter().filter(|(_, result)| result.is_ok()).count(),
        1
    );

    for (name, result) in results {
        if result.is_ok() {
            assert!(!scratch.path().join(name).exists());
            assert_eq!(
                fs::read(scratch.path().join("final/member")).unwrap(),
                name.as_bytes()
            );
        } else {
            assert!(matches!(
                result,
                Err(DirectoryPublishError::NotPublished(_))
            ));
            assert_eq!(
                fs::read(scratch.path().join(name).join("member")).unwrap(),
                name.as_bytes()
            );
        }
    }
}
