use crate::{Directory, Error, FileId, FilePolicy, Lock};
use std::ffi::OsStr;
use std::fs;
use std::io;
use std::os::unix::fs::{PermissionsExt, symlink};

const POLICY: FilePolicy = FilePolicy {
    forbidden: 0,
    single_link: false,
};

#[test]
fn existing_inode_bytes_and_permissions_survive_contention_and_release() {
    let scratch = tempfile::tempdir().unwrap();
    let path = scratch.path().join("lock");
    fs::write(&path, b"retained sidecar").unwrap();
    fs::set_permissions(&path, fs::Permissions::from_mode(0o644)).unwrap();
    let original = FileId::from_metadata(&fs::metadata(&path).unwrap());
    let parent = Directory::open(scratch.path(), 0).unwrap().unwrap();
    let held = parent.lock(OsStr::new("lock"), 0o600, POLICY).unwrap();
    assert_eq!(FileId::from_file(held.file()).unwrap(), original);
    assert!(
        matches!(parent.lock(OsStr::new("lock"), 0o600, POLICY), Err(Error::Io(cause)) if cause.kind() == io::ErrorKind::WouldBlock)
    );
    let file = fs::OpenOptions::new()
        .read(true)
        .write(true)
        .open(&path)
        .unwrap();
    assert_eq!(
        Lock::acquire(file).unwrap_err().kind(),
        io::ErrorKind::WouldBlock
    );
    held.release().unwrap();
    parent
        .lock(OsStr::new("lock"), 0o600, POLICY)
        .unwrap()
        .release()
        .unwrap();
    assert_eq!(
        FileId::from_metadata(&fs::metadata(&path).unwrap()),
        original
    );
    assert_eq!(fs::read(&path).unwrap(), b"retained sidecar");
    assert_eq!(
        fs::metadata(&path).unwrap().permissions().mode() & 0o777,
        0o644
    );
}

#[test]
fn held_parent_creation_survives_rename_and_drop_releases() {
    let scratch = tempfile::tempdir().unwrap();
    let before = scratch.path().join("before");
    let moved = scratch.path().join("moved");
    fs::create_dir(&before).unwrap();
    let parent = Directory::open(&before, 0).unwrap().unwrap();
    fs::rename(&before, &moved).unwrap();
    fs::create_dir(&before).unwrap();
    let held = parent.lock(OsStr::new("lock"), 0o600, POLICY).unwrap();
    assert!(moved.join("lock").is_file());
    assert!(!before.join("lock").exists());
    assert_eq!(
        fs::metadata(moved.join("lock"))
            .unwrap()
            .permissions()
            .mode()
            & 0o177,
        0
    );
    drop(held);
    parent
        .lock(OsStr::new("lock"), 0o600, POLICY)
        .unwrap()
        .release()
        .unwrap();
    assert!(moved.join("lock").exists());
}

#[test]
fn unsafe_entries_and_policy_refuse_without_repair() {
    let scratch = tempfile::tempdir().unwrap();
    let parent = Directory::open(scratch.path(), 0).unwrap().unwrap();
    let private = FilePolicy {
        forbidden: 0o077,
        single_link: true,
    };
    assert!(matches!(
        parent.lock(OsStr::new("../escape"), 0o600, POLICY),
        Err(Error::Name)
    ));
    assert!(matches!(
        parent.lock(OsStr::new("invalid"), 0o644, private),
        Err(Error::Permissions { .. })
    ));
    assert!(!scratch.path().join("invalid").exists());
    fs::write(scratch.path().join("public"), b"foreign").unwrap();
    fs::set_permissions(
        scratch.path().join("public"),
        fs::Permissions::from_mode(0o644),
    )
    .unwrap();
    assert!(matches!(
        parent.lock(OsStr::new("public"), 0o600, private),
        Err(Error::Permissions { .. })
    ));
    symlink("public", scratch.path().join("link")).unwrap();
    assert!(matches!(
        parent.lock(OsStr::new("link"), 0o600, POLICY),
        Err(Error::Kind)
    ));
    rustix::fs::mkfifoat(
        &*parent.file,
        "fifo",
        rustix::fs::Mode::RUSR | rustix::fs::Mode::WUSR,
    )
    .unwrap();
    assert!(matches!(
        parent.lock(OsStr::new("fifo"), 0o600, POLICY),
        Err(Error::Kind)
    ));
    fs::hard_link(scratch.path().join("public"), scratch.path().join("alias")).unwrap();
    assert!(matches!(
        parent.lock(
            OsStr::new("alias"),
            0o600,
            FilePolicy {
                forbidden: 0,
                single_link: true
            }
        ),
        Err(Error::Aliased)
    ));
    parent
        .lock(OsStr::new("alias"), 0o600, POLICY)
        .unwrap()
        .release()
        .unwrap();
    assert_eq!(fs::read(scratch.path().join("public")).unwrap(), b"foreign");
    assert_eq!(
        fs::metadata(scratch.path().join("public"))
            .unwrap()
            .permissions()
            .mode()
            & 0o777,
        0o644
    );
}

#[test]
fn concurrent_creation_locks_one_inode_without_replacement() {
    let scratch = tempfile::tempdir().unwrap();
    let parent = Directory::open(scratch.path(), 0).unwrap().unwrap();
    let barrier = std::sync::Barrier::new(2);
    let results = std::thread::scope(|scope| {
        let handles: Vec<_> = (0..2)
            .map(|_| {
                let parent = &parent;
                let barrier = &barrier;
                scope.spawn(move || {
                    barrier.wait();
                    parent.lock(OsStr::new("lock"), 0o600, POLICY)
                })
            })
            .collect();
        handles
            .into_iter()
            .map(|handle| handle.join().unwrap())
            .collect::<Vec<_>>()
    });
    assert_eq!(results.iter().filter(|result| result.is_ok()).count(), 1);
    let original = FileId::from_metadata(&fs::metadata(scratch.path().join("lock")).unwrap());
    for result in results {
        match result {
            Ok(lock) => {
                assert_eq!(FileId::from_file(lock.file()).unwrap(), original);
                lock.release().unwrap();
            }
            Err(Error::Io(cause)) => assert_eq!(cause.kind(), io::ErrorKind::WouldBlock),
            other => panic!("unexpected result: {other:?}"),
        }
    }
    assert_eq!(
        FileId::from_metadata(&fs::metadata(scratch.path().join("lock")).unwrap()),
        original
    );
}
