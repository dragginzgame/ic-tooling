use super::*;
use ic_host_artifacts::artifact::encode_gzip;
use std::os::unix::fs::symlink;

struct Fixture {
    root: ic_host_fs::testing::FixtureDirectory,
}
impl Fixture {
    fn new() -> Self {
        let root =
            ic_host_fs::testing::FixtureDirectory::new(&std::env::temp_dir(), "testkit-provision")
                .unwrap();
        Self { root }
    }
}
fn archive(version: &str) -> (Asset, Vec<u8>) {
    let script = format!("#!/bin/sh\nprintf 'pocket-ic-server {version}\\n'\n");
    let mut bytes = Vec::new();
    encode_gzip(script.as_bytes(), &mut bytes, 1, 65536).unwrap();
    (
        Asset {
            host: "fixture",
            digest: Sha256Digest::compute(&bytes),
        },
        bytes,
    )
}

#[test]
fn authenticated_setup_reuses_offline_and_refuses_changed_bytes_before_execution() {
    if ic_host_process::testing::run_isolated(
        "IC_TESTKIT_ASSET_CASE",
        "server asset test",
        ic_host_process::tool::OutputLimits {
            stdout: ic_host_process::tool::OutputLimit::Terminate(64 * 1024),
            stderr: ic_host_process::tool::OutputLimit::Terminate(64 * 1024),
            timeout: Some(std::time::Duration::from_secs(120)),
        },
    ) {
        return;
    }
    let root = Fixture::new();
    let (asset, bytes) = archive(POCKET_IC_SERVER_VERSION);
    let executable = setup_with(root.root.path(), &asset, |path| {
        fs::write(path, &bytes)?;
        Ok(())
    })
    .unwrap();
    let original = fs::read(&executable).unwrap();
    assert_eq!(
        setup_with(root.root.path(), &asset, |_| panic!(
            "verified reuse must not download"
        ))
        .unwrap(),
        executable
    );
    let entries = fs::read_dir(root.root.path()).unwrap().count();
    assert_eq!(
        check_bundle(&bundle_path(root.root.path(), &asset), &asset).unwrap(),
        executable
    );
    assert_eq!(fs::read_dir(root.root.path()).unwrap().count(), entries);
    let marker = root.root.path().join("executed");
    fs::write(
        &executable,
        format!("#!/bin/sh\ntouch '{}'\n", marker.display()),
    )
    .unwrap();
    assert!(check_bundle(&bundle_path(root.root.path(), &asset), &asset).is_err());
    assert!(!marker.exists());
    fs::write(&executable, original).unwrap();
    assert!(check_bundle(&bundle_path(root.root.path(), &asset), &asset).is_ok());
    let saved = root.root.path().join("saved");
    fs::rename(&executable, &saved).unwrap();
    symlink(&saved, &executable).unwrap();
    assert!(check_bundle(&bundle_path(root.root.path(), &asset), &asset).is_err());
}

#[test]
fn offline_check_authenticates_before_decoding_and_rejects_invalid_gzip() {
    use ic_host_artifacts::artifact::{ArtifactError, GzipError};

    if ic_host_process::testing::run_isolated(
        "IC_TESTKIT_ASSET_CASE",
        "server asset test",
        ic_host_process::tool::OutputLimits {
            stdout: ic_host_process::tool::OutputLimit::Terminate(64 * 1024),
            stderr: ic_host_process::tool::OutputLimit::Terminate(64 * 1024),
            timeout: Some(std::time::Duration::from_secs(120)),
        },
    ) {
        return;
    }

    let root = Fixture::new();
    let (mut asset, original) = archive(POCKET_IC_SERVER_VERSION);
    let executable = setup_with(root.root.path(), &asset, |path| {
        fs::write(path, &original)?;
        Ok(())
    })
    .unwrap();
    let installed = fs::read(&executable).unwrap();
    let bundle = bundle_path(root.root.path(), &asset);
    let path = bundle.join("archive.gz");

    // Both authentication and decoding would fail; authentication wins.
    fs::write(&path, b"not gzip").unwrap();
    let error = check_bundle(&bundle, &asset).unwrap_err();
    assert!(matches!(
        error.downcast_ref::<ArtifactError>(),
        Some(ArtifactError::DigestMismatch { .. })
    ));

    let mut bad_crc = original.clone();
    let crc = bad_crc.len() - 8;
    bad_crc[crc] ^= 1;
    let mut bad_length = original.clone();
    let length = bad_length.len() - 4;
    bad_length[length] ^= 1;
    for bytes in [
        b"not gzip".to_vec(),
        original[..original.len() - 1].to_vec(),
        bad_crc,
        bad_length,
        [original.as_slice(), b"trailing"].concat(),
        [original.as_slice(), original.as_slice()].concat(),
    ] {
        fs::write(&path, &bytes).unwrap();
        asset.digest = Sha256Digest::compute(&bytes);
        let error = check_bundle(&bundle, &asset).unwrap_err();
        // Gzip refusal occurs before executable admission/version execution.
        assert!(error.downcast_ref::<GzipError>().is_some());
        assert_eq!(fs::read(&executable).unwrap(), installed);
        assert_eq!(fs::read(&path).unwrap(), bytes);
    }
    fs::write(&path, &original).unwrap();
    asset.digest = Sha256Digest::compute(&original);
    assert_eq!(check_bundle(&bundle, &asset).unwrap(), executable);
}

#[test]
fn setup_rejects_corrupt_payloads_without_execution_or_publication() {
    use ic_host_artifacts::artifact::GzipError;

    if ic_host_process::testing::run_isolated(
        "IC_TESTKIT_ASSET_CASE",
        "server asset test",
        ic_host_process::tool::OutputLimits {
            stdout: ic_host_process::tool::OutputLimit::Terminate(64 * 1024),
            stderr: ic_host_process::tool::OutputLimit::Terminate(64 * 1024),
            timeout: Some(std::time::Duration::from_secs(120)),
        },
    ) {
        return;
    }
    let root = Fixture::new();
    let marker = root.root.path().join("executed");
    let script = format!("#!/bin/sh\nprintf executed > '{}'\n", marker.display());
    let mut original = Vec::new();
    encode_gzip(script.as_bytes(), &mut original, 1, 65536).unwrap();
    let mut bad_crc = original.clone();
    let crc = bad_crc.len() - 8;
    bad_crc[crc] ^= 1;
    for bytes in [
        bad_crc,
        original[..original.len() - 1].to_vec(),
        [original.as_slice(), b"trailing"].concat(),
    ] {
        let asset = Asset {
            host: "fixture",
            digest: Sha256Digest::compute(&bytes),
        };
        let error = setup_with(root.root.path(), &asset, |path| {
            fs::write(path, &bytes)?;
            Ok(())
        })
        .unwrap_err();
        assert!(matches!(
            error.downcast_ref::<durable::NamedWriteError<GzipError>>(),
            Some(durable::NamedWriteError::Producer { .. })
        ));
        assert!(!marker.exists());
        assert!(!bundle_path(root.root.path(), &asset).exists());
        assert!(fs::read_dir(root.root.path()).unwrap().any(|entry| {
            let path = entry.unwrap().path();
            path.join("failure.txt").is_file()
                && fs::read(path.join("archive.gz")).ok().as_deref() == Some(bytes.as_slice())
        }));
    }
}

#[test]
fn failed_setup_retains_evidence_and_previous_bundles_then_can_retry() {
    if ic_host_process::testing::run_isolated(
        "IC_TESTKIT_ASSET_CASE",
        "server asset test",
        ic_host_process::tool::OutputLimits {
            stdout: ic_host_process::tool::OutputLimit::Terminate(64 * 1024),
            stderr: ic_host_process::tool::OutputLimit::Terminate(64 * 1024),
            timeout: Some(std::time::Duration::from_secs(120)),
        },
    ) {
        return;
    }
    let root = Fixture::new();
    let previous = root.root.path().join("previous-version");
    fs::create_dir(&previous).unwrap();
    fs::write(previous.join("pocket-ic"), b"previous bytes").unwrap();
    let (asset, bytes) = archive(POCKET_IC_SERVER_VERSION);
    assert!(
        setup_with(root.root.path(), &asset, |path| {
            fs::write(path, b"partial download")?;
            Err("interrupted download".into())
        })
        .is_err()
    );
    assert!(!bundle_path(root.root.path(), &asset).exists());
    let failed = fs::read_dir(root.root.path())
        .unwrap()
        .find_map(|entry| {
            let path = entry.unwrap().path();
            path.join("failure.txt").is_file().then_some(path)
        })
        .unwrap();
    assert_eq!(
        fs::read(failed.join("archive.gz")).unwrap(),
        b"partial download"
    );
    assert!(
        std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
            let _ = setup_with(root.root.path(), &asset, |path| {
                fs::write(path, b"interrupted candidate")?;
                panic!("injected interruption before admission");
            });
        }))
        .is_err()
    );
    assert!(!bundle_path(root.root.path(), &asset).exists());
    assert!(fs::read_dir(root.root.path()).unwrap().any(|entry| {
        fs::read(entry.unwrap().path().join("archive.gz"))
            .ok()
            .as_deref()
            == Some(b"interrupted candidate")
    }));
    assert!(
        setup_with(root.root.path(), &asset, |path| {
            fs::write(path, &bytes)?;
            Ok(())
        })
        .is_ok()
    );
    assert!(failed.join("failure.txt").is_file());
    assert_eq!(
        fs::read(previous.join("pocket-ic")).unwrap(),
        b"previous bytes"
    );
}

#[test]
fn digest_decode_and_version_failures_never_publish() {
    if ic_host_process::testing::run_isolated(
        "IC_TESTKIT_ASSET_CASE",
        "server asset test",
        ic_host_process::tool::OutputLimits {
            stdout: ic_host_process::tool::OutputLimit::Terminate(64 * 1024),
            stderr: ic_host_process::tool::OutputLimit::Terminate(64 * 1024),
            timeout: Some(std::time::Duration::from_secs(120)),
        },
    ) {
        return;
    }
    for mode in [0, 1, 2] {
        let root = Fixture::new();
        let (mut asset, mut bytes) = archive("15.0.0");
        if mode == 0 {
            asset.digest = Sha256Digest::compute(b"wrong");
        }
        if mode == 1 {
            bytes = b"not gzip".to_vec();
            asset.digest = Sha256Digest::compute(&bytes);
        }
        assert!(
            setup_with(root.root.path(), &asset, |path| {
                fs::write(path, &bytes)?;
                Ok(())
            })
            .is_err()
        );
        assert!(!bundle_path(root.root.path(), &asset).exists());
    }
}

#[test]
fn redirected_paths_are_refused_without_touching_the_target() {
    if ic_host_process::testing::run_isolated(
        "IC_TESTKIT_ASSET_CASE",
        "server asset test",
        ic_host_process::tool::OutputLimits {
            stdout: ic_host_process::tool::OutputLimit::Terminate(64 * 1024),
            stderr: ic_host_process::tool::OutputLimit::Terminate(64 * 1024),
            timeout: Some(std::time::Duration::from_secs(120)),
        },
    ) {
        return;
    }
    let root = Fixture::new();
    let (asset, _) = archive(POCKET_IC_SERVER_VERSION);
    let target = root.root.path().join("target");
    fs::create_dir(&target).unwrap();
    let redirected = root.root.path().join("redirected");
    symlink(&target, &redirected).unwrap();
    assert!(
        setup_with(&redirected.join("child"), &asset, |_| panic!(
            "redirected ancestor reached downloader"
        ))
        .is_err()
    );
    assert_eq!(fs::read_dir(&target).unwrap().count(), 0);
    symlink(&target, root.root.path().join(".setup-v1.lock")).unwrap();
    assert!(
        setup_with(root.root.path(), &asset, |_| panic!(
            "redirected lock reached downloader"
        ))
        .is_err()
    );
}

#[test]
fn replaced_staging_directory_is_not_published() {
    if ic_host_process::testing::run_isolated(
        "IC_TESTKIT_ASSET_CASE",
        "server asset test",
        ic_host_process::tool::OutputLimits {
            stdout: ic_host_process::tool::OutputLimit::Terminate(64 * 1024),
            stderr: ic_host_process::tool::OutputLimit::Terminate(64 * 1024),
            timeout: Some(std::time::Duration::from_secs(120)),
        },
    ) {
        return;
    }
    let root = Fixture::new();
    let (asset, bytes) = archive(POCKET_IC_SERVER_VERSION);
    let retained = root.root.path().join("retained-original-stage");
    let error = setup_with(root.root.path(), &asset, |archive| {
        fs::write(archive, &bytes)?;
        let stage = archive.parent().unwrap();
        fs::rename(stage, &retained)?;
        fs::create_dir(stage)?;
        fs::write(archive, &bytes)?;
        Ok(())
    })
    .expect_err("replacement must not inherit the held directory claim");
    assert!(matches!(
        error.downcast_ref::<durable::DirectoryCommitError<io::Error>>(),
        Some(durable::DirectoryCommitError::Prepare(source))
            if source.kind() == io::ErrorKind::InvalidInput
    ));
    assert!(!bundle_path(root.root.path(), &asset).exists());
    assert_eq!(fs::read(retained.join("archive.gz")).unwrap(), bytes);
}

#[test]
fn occupied_destination_preserves_both_bundle_and_verified_staging() {
    if ic_host_process::testing::run_isolated(
        "IC_TESTKIT_ASSET_CASE",
        "server asset test",
        ic_host_process::tool::OutputLimits {
            stdout: ic_host_process::tool::OutputLimit::Terminate(64 * 1024),
            stderr: ic_host_process::tool::OutputLimit::Terminate(64 * 1024),
            timeout: Some(std::time::Duration::from_secs(120)),
        },
    ) {
        return;
    }
    let root = Fixture::new();
    let (asset, bytes) = archive(POCKET_IC_SERVER_VERSION);
    let bundle = bundle_path(root.root.path(), &asset);
    let mut attempt = None;
    let error = setup_with(root.root.path(), &asset, |archive| {
        fs::write(archive, &bytes)?;
        attempt = archive.parent().map(Path::to_owned);
        fs::create_dir(&bundle)?;
        fs::write(bundle.join("unrelated"), b"retained destination")?;
        Ok(())
    })
    .unwrap_err();
    assert!(matches!(
        error.downcast_ref::<durable::DirectoryCommitError<io::Error>>(),
        Some(durable::DirectoryCommitError::Conflict)
    ));
    assert_eq!(
        fs::read(bundle.join("unrelated")).unwrap(),
        b"retained destination"
    );
    let attempt = attempt.unwrap();
    assert_eq!(fs::read(attempt.join("archive.gz")).unwrap(), bytes);
    assert!(attempt.join("pocket-ic").is_file());
}

#[test]
fn concurrent_setups_download_once_and_recheck_under_the_lock() {
    if ic_host_process::testing::run_isolated(
        "IC_TESTKIT_ASSET_CASE",
        "server asset test",
        ic_host_process::tool::OutputLimits {
            stdout: ic_host_process::tool::OutputLimit::Terminate(64 * 1024),
            stderr: ic_host_process::tool::OutputLimit::Terminate(64 * 1024),
            timeout: Some(std::time::Duration::from_secs(120)),
        },
    ) {
        return;
    }
    let root = Fixture::new();
    let (asset, bytes) = archive(POCKET_IC_SERVER_VERSION);
    let calls = AtomicU64::new(0);
    std::thread::scope(|scope| {
        let handles = (0..2)
            .map(|_| {
                scope.spawn(|| {
                    setup_with(root.root.path(), &asset, |path| {
                        calls.fetch_add(1, Ordering::Relaxed);
                        fs::write(path, &bytes)?;
                        Ok(())
                    })
                    .unwrap()
                })
            })
            .collect::<Vec<_>>();
        for handle in handles {
            assert_eq!(
                handle.join().unwrap(),
                bundle_path(root.root.path(), &asset).join("pocket-ic")
            );
        }
    });
    assert_eq!(calls.load(Ordering::Relaxed), 1);
}
