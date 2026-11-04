use crate::testing::fs::unique_temp_directory;
use std::{
    fs,
    time::{Duration, SystemTime, UNIX_EPOCH},
};

#[test]
fn acquisition_completion_preserves_published_entries_and_cleans_failed_staging() {
    use ic_host_fs::owned_directory::OwnedDirectory;
    let root = unique_temp_directory("cache-acquisition-completion");
    for (name, succeeds, published) in [
        ("complete", true, false),
        ("incomplete", false, false),
        ("published", false, true),
    ] {
        let path = root.join(name);
        fs::create_dir(&path).unwrap();
        fs::write(path.join("output"), b"owned output").unwrap();
        let mut directory = OwnedDirectory::new(path.clone());
        if published {
            directory.disarm();
        }
        let result = super::finish_cache_build(
            if succeeds {
                Ok(())
            } else {
                Err("build failure")
            },
            directory,
            |_, _, _| panic!("unexpected cleanup failure"),
        );
        assert_eq!(result.is_ok(), succeeds);
        assert_eq!(path.exists(), succeeds || published);
    }
    fs::remove_dir_all(root).unwrap();
}

#[test]
fn acquisition_cleanup_failure_retains_both_causes_and_the_recovery_path() {
    use ic_host_fs::owned_directory::OwnedDirectory;
    let root = unique_temp_directory("cache-acquisition-cleanup-failure");
    let blocker = root.join("not-a-directory");
    fs::write(&blocker, b"preserve").unwrap();
    let path = blocker.join("staging");
    let result: Result<(), _> = super::finish_cache_build(
        Err(("original failure", None)),
        OwnedDirectory::new(path.clone()),
        |(cause, _), path, source| (cause, Some((path, source))),
    );
    let (cause, cleanup) = result.unwrap_err();
    let (recovery, source) = cleanup.unwrap();
    assert_eq!(cause, "original failure");
    assert_eq!(recovery, path);
    assert_eq!(source.kind(), std::io::ErrorKind::NotADirectory);
    assert_eq!(fs::read(blocker).unwrap(), b"preserve");
    fs::remove_dir_all(root).unwrap();
}

#[cfg(unix)]
#[test]
fn cache_lock_admission_preserves_bytes_and_returns_an_unlocked_close_on_exec_file() {
    let root = unique_temp_directory("cache-lock-admission");
    let path = root.join("nested/lock");
    let file = super::open_cache_lock_file(&path).unwrap();
    assert!(super::try_lock_cache_file(&path).unwrap().is_some());
    let flags = rustix::io::fcntl_getfd(&file).unwrap();
    assert!(flags.contains(rustix::io::FdFlags::CLOEXEC));
    drop(file);
    fs::write(&path, b"retained lock bytes").unwrap();
    drop(super::lock_cache_file(&path).unwrap());
    assert_eq!(fs::read(&path).unwrap(), b"retained lock bytes");
    fs::remove_dir_all(root).unwrap();
}

#[cfg(unix)]
#[test]
fn cache_lock_callers_refuse_redirected_final_entries() {
    use std::os::unix::fs::symlink;

    let root = unique_temp_directory("cache-lock-redirected");
    let target = root.join("target");
    fs::write(&target, b"target bytes").unwrap();
    let redirected = root.join("redirected");
    symlink(&target, &redirected).unwrap();
    for result in [
        super::open_cache_lock_file(&redirected),
        super::lock_cache_file(&redirected).map(|(file, _)| file),
        super::lock_cache_file_with_wait_observer(&redirected, Duration::from_millis(5), |_| {
            panic!("redirected lock reached acquisition")
        })
        .map(|(file, _)| file),
    ] {
        let error = result.unwrap_err();
        assert_eq!(error.path, redirected);
        assert!(matches!(
                error.source.get_ref().and_then(|cause| cause
                    .downcast_ref::<ic_host_fs::durable::RegularFileLockError>(
                )),
                Some(ic_host_fs::durable::RegularFileLockError::NotRegular)
            ));
    }
    symlink(&target, root.join(super::RETENTION_LOCK_FILE)).unwrap();
    assert!(super::RetainedCacheEntry::acquire(&root).is_err());
    assert_eq!(fs::read(&target).unwrap(), b"target bytes");
    fs::remove_dir_all(root).unwrap();
}

#[cfg(unix)]
#[test]
fn nonblocking_cache_locks_preserve_contention_and_reject_redirected_files() {
    use std::os::unix::fs::symlink;
    const CHILD_ENV: &str = "IC_TESTKIT_NONBLOCKING_CACHE_LOCK_CHILD";

    if ic_host_process::testing::run_isolated(
        CHILD_ENV,
        "nonblocking cache lock test",
        ic_host_process::tool::OutputLimits {
            stdout: ic_host_process::tool::OutputLimit::Terminate(64 * 1024),
            stderr: ic_host_process::tool::OutputLimit::Terminate(64 * 1024),
            timeout: Some(std::time::Duration::from_secs(120)),
        },
    ) {
        return;
    }

    let root = unique_temp_directory("nonblocking-cache-lock-admission");
    let lock_path = root.join("lock");
    fs::write(&lock_path, b"existing lock bytes").unwrap();
    let held = super::try_lock_cache_file(&lock_path).unwrap().unwrap();
    assert!(super::try_lock_cache_file(&lock_path).unwrap().is_none());
    assert_eq!(fs::read(&lock_path).unwrap(), b"existing lock bytes");
    drop(held);
    assert!(super::try_lock_cache_file(&lock_path).unwrap().is_some());

    let redirected = root.join("redirected-lock");
    symlink(&lock_path, &redirected).unwrap();
    let error = super::try_lock_cache_file(&redirected).unwrap_err();
    assert_eq!(error.path, redirected);
    assert!(matches!(
        error
            .source
            .get_ref()
            .and_then(|cause| cause.downcast_ref::<ic_host_fs::durable::RegularFileLockError>()),
        Some(ic_host_fs::durable::RegularFileLockError::NotRegular)
    ));
    assert_eq!(fs::read(&lock_path).unwrap(), b"existing lock bytes");
    fs::remove_dir_all(root).unwrap();
}

#[cfg(unix)]
#[test]
fn final_retention_owner_releases_lock_with_a_duplicate_descriptor_open() {
    let root = unique_temp_directory("retention-duplicate-descriptor");
    let retained = super::RetainedCacheEntry::acquire(&root).unwrap();
    // A process spawn can duplicate this descriptor before close-on-exec.
    #[expect(
        clippy::used_underscore_binding,
        reason = "inspect the RAII guard to verify inherited-descriptor retention"
    )]
    let inherited = retained._lock.file().try_clone().unwrap();
    let clone = retained.clone();
    let independently_retained = super::RetainedCacheEntry::acquire(&root).unwrap();
    let lock_path = root.join(super::RETENTION_LOCK_FILE);
    let available = || super::try_lock_cache_file(&lock_path).unwrap().is_some();
    drop(retained);
    assert!(!available(), "a record clone still retains the entry");
    drop(clone);
    assert!(
        !available(),
        "an independent acquisition still retains the entry"
    );
    drop(independently_retained);
    assert!(
        available(),
        "descriptor duplication must not extend record ownership"
    );
    drop(inherited);
    fs::remove_dir_all(root).unwrap();
}

#[test]
fn last_use_markers_preserve_timestamps_and_bounded_fallbacks() {
    let root = unique_temp_directory("bounded-last-use-marker");
    let marker = root.join(super::LAST_USED_FILE);
    let modified = || fs::metadata(&root).unwrap().modified().unwrap();
    assert_eq!(super::cache_entry_last_used(&root).unwrap(), modified());
    for timestamp in [
        UNIX_EPOCH + Duration::from_nanos(123),
        SystemTime::now() + Duration::from_secs(3600),
    ] {
        super::write_last_used(&root, timestamp).unwrap();
        assert_eq!(super::cache_entry_last_used(&root).unwrap(), timestamp);
    }
    let overflowing_timestamp = u128::MAX.to_string();
    for invalid in [
        b"invalid".as_slice(),
        overflowing_timestamp.as_bytes(),
        &[0xff],
    ] {
        fs::write(&marker, invalid).unwrap();
        assert_eq!(super::cache_entry_last_used(&root).unwrap(), modified());
    }
    fs::File::create(&marker)
        .unwrap()
        .set_len(1024 * 1024 * 1024)
        .unwrap();
    assert_eq!(super::cache_entry_last_used(&root).unwrap(), modified());
    fs::remove_file(&marker).unwrap();
    fs::create_dir(&marker).unwrap();
    assert_eq!(super::cache_entry_last_used(&root).unwrap(), modified());
    fs::remove_dir_all(root).unwrap();
}

#[test]
fn maintenance_markers_preserve_policy_intervals_and_read_errors() {
    let root = unique_temp_directory("bounded-maintenance-marker");
    let marker = root.join(super::LAST_MAINTENANCE_FILE);
    let identity = super::CacheRetentionLimits::new().maintenance_identity();
    let interval = Duration::from_secs(3600);
    let due = || super::cache_maintenance_due(&root, interval, &identity).unwrap();
    assert!(due());
    super::record_cache_maintenance(&root, &identity).unwrap();
    assert!(!due());
    assert!(super::cache_maintenance_due(&root, interval, "other-policy").unwrap());
    assert!(super::cache_maintenance_due(&root, Duration::ZERO, &identity).unwrap());

    let now = SystemTime::now().duration_since(UNIX_EPOCH).unwrap();
    for suffix in ["", "\r\n"] {
        fs::write(&marker, format!("{}\r\n{identity}{suffix}", now.as_nanos())).unwrap();
        assert!(!due());
    }
    for timestamp in [
        "invalid".to_owned(),
        "0".to_owned(),
        (now + Duration::from_secs(3600)).as_nanos().to_string(),
    ] {
        fs::write(&marker, format!("{timestamp}\n{identity}\n")).unwrap();
        assert!(due());
    }
    super::record_cache_maintenance(&root, &identity).unwrap();
    fs::OpenOptions::new()
        .write(true)
        .open(&marker)
        .unwrap()
        .set_len(1024 * 1024 * 1024)
        .unwrap();
    assert!(due());

    fs::write(&marker, [0xff]).unwrap();
    assert!(super::cache_maintenance_due(&root, Duration::ZERO, &identity).unwrap());
    let error = super::cache_maintenance_due(&root, interval, &identity).unwrap_err();
    assert_eq!(error.operation, "read cache maintenance time");
    assert_eq!(error.path, marker);
    assert_eq!(error.source.kind(), std::io::ErrorKind::InvalidData);
    fs::remove_file(&marker).unwrap();
    fs::create_dir(&marker).unwrap();
    assert!(super::cache_maintenance_due(&root, interval, &identity).is_err());
    assert!(super::cache_maintenance_due(&root, Duration::ZERO, &identity).unwrap());
    fs::remove_dir_all(root).unwrap();
}

#[test]
fn cache_directory_tags_preserve_valid_standard_signatures() {
    let root = unique_temp_directory("cache-tag-signatures");
    let tag = root.join("CACHEDIR.TAG");
    let signature = ic_host_fs::cache_tag::SIGNATURE;
    for contents in [
        signature.to_owned(),
        format!("{signature}\r\n# Created by another application\r\n"),
        format!("{signature}\n# {}\n", "comment".repeat(100_000)),
    ] {
        fs::write(&tag, &contents).unwrap();
        super::ensure_cache_directory_tag(&root).unwrap();
        assert_eq!(fs::read_to_string(&tag).unwrap(), contents);
    }
    for invalid in ["", &signature[..42], "Signature: incorrect"] {
        fs::write(&tag, invalid).unwrap();
        super::ensure_cache_directory_tag(&root).unwrap();
        assert_eq!(
            fs::read_to_string(&tag).unwrap(),
            format!("{signature}{}", super::CACHE_DIRECTORY_TAG_SUFFIX)
        );
    }
    fs::remove_dir_all(root).unwrap();
}

#[test]
#[cfg(unix)]
fn cache_directory_tag_replaces_symlinks_without_changing_referents() {
    let root = unique_temp_directory("cache-tag-symlink");
    let referent = root.join("other-application-tag");
    let contents = "Signature: 8a477f597d28d172789f06886806bc55\n# Preserve this file\n";
    fs::write(&referent, contents).unwrap();
    let tag = root.join("CACHEDIR.TAG");
    std::os::unix::fs::symlink(&referent, &tag).unwrap();
    super::ensure_cache_directory_tag(&root).unwrap();
    assert!(fs::symlink_metadata(&tag).unwrap().file_type().is_file());
    assert_eq!(fs::read_to_string(&referent).unwrap(), contents);
    assert_eq!(
        fs::read_to_string(&tag).unwrap(),
        format!(
            "{}{}",
            ic_host_fs::cache_tag::SIGNATURE,
            super::CACHE_DIRECTORY_TAG_SUFFIX
        )
    );

    fs::remove_file(&tag).unwrap();
    fs::create_dir(&tag).unwrap();
    let error = super::ensure_cache_directory_tag(&root).unwrap_err();
    assert!(matches!(
        error
            .source
            .get_ref()
            .unwrap()
            .downcast_ref::<ic_host_fs::durable::NamedWriteError<std::io::Error>>()
            .unwrap(),
        ic_host_fs::durable::NamedWriteError::BeforePublication {
            cleanup_error: None,
            ..
        }
    ));
    assert_eq!(error.operation, "write cache directory tag");
    assert_eq!(error.path, tag);
    assert!(tag.is_dir());
    fs::remove_dir_all(root).unwrap();
}
