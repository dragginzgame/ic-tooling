use super::*;

#[test]
fn file_hash_preserves_pinned_identity_and_missing_file_error_kind() {
    let temp = installation_scratch(&std::env::temp_dir()).unwrap();
    let candidate = temp.path().join("candidate");
    fs::write(&candidate, b"abc").unwrap();
    assert_eq!(
        sha256_file(&candidate).unwrap(),
        "ba7816bf8f01cfea414140de5dae2223b00361a396177a9cb410ff61f20015ad"
    );
    let missing = temp.path().join("missing");
    match sha256_file(&missing) {
        Err(InstallError::Io { path, source, .. }) => {
            assert_eq!(path, missing);
            assert_eq!(source.kind(), io::ErrorKind::NotFound);
        }
        result => panic!("expected a missing-file I/O failure, got {result:?}"),
    }
}

#[test]
fn rejected_stage_preserves_destination_and_removes_temporary_copy() {
    let temp = installation_scratch(&std::env::temp_dir()).unwrap();
    let candidate = temp.path().join("candidate");
    let destination = temp.path().join("installed");
    fs::write(&candidate, b"new executable").unwrap();
    fs::write(&destination, b"previous executable").unwrap();
    let result = publish_executable(&candidate, &destination, |_stage| {
        Err(InstallError::ExecutableHashMismatch {
            path: destination.clone(),
            actual: "actual".into(),
            expected: "required".into(),
        })
    });
    std::assert_matches!(result, Err(InstallError::ExecutableHashMismatch { .. }));
    assert_eq!(fs::read(&destination).unwrap(), b"previous executable");
    assert_eq!(fs::read_dir(temp.path()).unwrap().count(), 2);
}

#[test]
fn rejected_foreign_stage_retains_original_admission_and_cleanup_causes() {
    let temp = installation_scratch(&std::env::temp_dir()).unwrap();
    let candidate = temp.path().join("candidate");
    let destination = temp.path().join("installed");
    fs::write(&candidate, b"new executable").unwrap();
    fs::write(&destination, b"previous executable").unwrap();
    let mut replacement = None;
    let result = publish_executable(&candidate, &destination, |stage| {
        fs::remove_file(stage).unwrap();
        fs::create_dir(stage).unwrap();
        replacement = Some(stage.to_path_buf());
        Err(InstallError::ExecutableHashMismatch {
            path: stage.to_path_buf(),
            actual: "replaced".into(),
            expected: "admitted".into(),
        })
    });
    let Err(InstallError::Io { source, .. }) = result else {
        panic!("expected retained publication evidence");
    };
    let cause = source
        .get_ref()
        .unwrap()
        .downcast_ref::<NamedWriteError<InstallError>>()
        .unwrap();
    std::assert_matches!(
        cause,
        NamedWriteError::Producer {
            source: InstallError::ExecutableHashMismatch { .. },
            cleanup_error: Some(_),
        }
    );
    assert!(replacement.unwrap().is_dir());
    assert_eq!(fs::read(destination).unwrap(), b"previous executable");
}

#[test]
fn archive_hash_mismatch_is_rejected_before_extraction() {
    let temp = installation_scratch(&std::env::temp_dir()).unwrap();
    let archive = temp.path().join("archive");
    fs::write(&archive, b"wrong archive").unwrap();
    std::assert_matches!(
        verify_archive(&archive, "required"),
        Err(InstallError::ArchiveHashMismatch { .. })
    );
}

#[test]
fn publication_failure_cleans_stage_and_preserves_destination_directory() {
    let temp = installation_scratch(&std::env::temp_dir()).unwrap();
    let candidate = temp.path().join("candidate");
    let destination = temp.path().join("installed");
    fs::write(&candidate, b"candidate").unwrap();
    fs::create_dir(&destination).unwrap();
    let result = publish_executable(&candidate, &destination, |_| Ok::<_, InstallError>(()));
    std::assert_matches!(result, Err(InstallError::Io { .. }));
    assert!(destination.is_dir());
    assert_eq!(fs::read_dir(temp.path()).unwrap().count(), 2);
}

#[cfg(unix)]
#[test]
fn bundle_publication_preserves_relative_library_and_previous_selection_on_rejection() {
    let temp = installation_scratch(&std::env::temp_dir()).unwrap();
    let candidate = temp.path().join("candidate");
    let library = temp.path().join("library");
    fs::write(&candidate, b"executable").unwrap();
    fs::write(&library, b"runtime library").unwrap();
    let destination = temp.path().join("bin/wasm-opt");
    fs::create_dir_all(destination.parent().unwrap()).unwrap();
    fs::write(&destination, b"previous executable").unwrap();
    let admit = |path: &Path| {
        assert_eq!(fs::read(path).unwrap(), b"executable");
        assert_eq!(
            fs::read(path.parent().unwrap().join("../lib/library")).unwrap(),
            b"runtime library"
        );
        Ok::<_, InstallError>(())
    };
    publish_bundle("wasm-opt", &candidate, &library, &destination, admit).unwrap();
    let installed = fs::canonicalize(&destination).unwrap();
    assert!(
        fs::symlink_metadata(&destination)
            .unwrap()
            .file_type()
            .is_symlink()
    );
    admit(&installed).unwrap();
    let result = publish_bundle("wasm-opt", &candidate, &library, &destination, |stage| {
        Err(InstallError::ExecutableHashMismatch {
            path: stage.to_path_buf(),
            actual: "rejected".into(),
            expected: "required".into(),
        })
    });
    std::assert_matches!(result, Err(InstallError::ExecutableHashMismatch { .. }));
    assert_eq!(fs::canonicalize(&destination).unwrap(), installed);
    assert_eq!(
        fs::read_dir(destination.parent().unwrap()).unwrap().count(),
        2
    );
}

#[cfg(unix)]
#[test]
fn missing_runtime_library_preserves_working_installation() {
    let temp = installation_scratch(&std::env::temp_dir()).unwrap();
    let candidate = temp.path().join("candidate");
    let destination = temp.path().join("installed");
    fs::write(&candidate, b"candidate").unwrap();
    fs::write(&destination, b"previous executable").unwrap();
    let result = publish_bundle(
        "wasm-opt",
        &candidate,
        &temp.path().join("missing"),
        &destination,
        |_| Ok::<_, InstallError>(()),
    );
    std::assert_matches!(result, Err(InstallError::Io { source, .. }) if source.kind() == io::ErrorKind::NotFound);
    assert_eq!(fs::read(&destination).unwrap(), b"previous executable");
    assert_eq!(fs::read_dir(temp.path()).unwrap().count(), 2);
}

#[cfg(unix)]
#[test]
fn bundle_executable_runs_with_its_relative_library_before_and_after_selection() {
    let temp = installation_scratch(&std::env::temp_dir()).unwrap();
    let candidate = temp.path().join("candidate");
    let library = temp.path().join("library");
    let destination = temp.path().join("bin/wasm-opt");
    fs::write(
        &candidate,
        b"#!/bin/sh\nread identity < \"${0%/*}/../lib/library\"\nprintf '%s\\n' \"$identity\"\n",
    )
    .unwrap();
    fs::write(&library, b"qualified runtime\n").unwrap();
    let admit = |path: &Path| {
        let output = std::process::Command::new(path).output().unwrap();
        assert!(output.status.success());
        assert_eq!(output.stdout, b"qualified runtime\n");
        Ok::<_, InstallError>(())
    };
    publish_bundle("wasm-opt", &candidate, &library, &destination, admit).unwrap();
    admit(&fs::canonicalize(&destination).unwrap()).unwrap();
}

#[test]
fn archive_admission_precedes_extraction_and_publication() {
    let temp = installation_scratch(&std::env::temp_dir()).unwrap();
    let archive = temp.path().join("archive");
    fs::write(&archive, b"abc").unwrap();
    let destination = temp.path().join("installed");
    fs::write(&destination, b"old").unwrap();
    let spec = InstallSpec {
        tool: "example",
        archive: &archive,
        archive_sha256: "wrong",
        member: "bin/example",
        runtime_library: None,
    };
    let result = install(
        &spec,
        &destination,
        temp.path(),
        |_, _| panic!("mismatching archive must never extract"),
        |_| Ok::<_, InstallError>(()),
    );
    assert!(matches!(
        result,
        Err(InstallError::ArchiveHashMismatch { .. })
    ));
    let spec = InstallSpec {
        archive_sha256: "ba7816bf8f01cfea414140de5dae2223b00361a396177a9cb410ff61f20015ad",
        ..spec
    };
    install(
        &spec,
        &destination,
        temp.path(),
        |_, dir| {
            fs::create_dir(dir.join("bin")).unwrap();
            fs::write(dir.join("bin/example"), b"new").unwrap();
            Ok::<_, InstallError>(())
        },
        |p| {
            assert_eq!(fs::read(p).unwrap(), b"new");
            Ok::<_, InstallError>(())
        },
    )
    .unwrap();
    assert_eq!(fs::read(&destination).unwrap(), b"new");
}

#[test]
fn delegated_scratch_preserves_directory_modes_errors_and_owned_cleanup() {
    let root = installation_scratch(&std::env::temp_dir()).unwrap();
    let first = installation_scratch(root.path()).unwrap();
    let second = installation_scratch(root.path()).unwrap();
    assert_ne!(first.path(), second.path());
    assert!(first.path().starts_with(root.path()));
    #[cfg(unix)]
    {
        let control = root.path().join("control");
        fs::create_dir(&control).unwrap();
        assert_eq!(
            fs::metadata(first.path()).unwrap().permissions().mode() & 0o777,
            fs::metadata(control).unwrap().permissions().mode() & 0o777
        );
    }
    let first_path = first.path().to_owned();
    fs::write(first.path().join("owned"), b"owned").unwrap();
    drop(first);
    assert!(!first_path.exists());
    assert!(second.path().is_dir());
    let missing = root.path().join("missing");
    assert!(matches!(installation_scratch(&missing),
        Err(InstallError::Io { source, .. }) if source.kind() == io::ErrorKind::NotFound));
}

#[cfg(unix)]
#[test]
fn failed_bundle_cleanup_retains_original_cause_foreign_path_and_previous_installation() {
    use std::error::Error as _;
    let temp = installation_scratch(&std::env::temp_dir()).unwrap();
    let candidate = temp.path().join("candidate");
    let library = temp.path().join("library");
    let destination = temp.path().join("installed");
    fs::write(&candidate, b"executable").unwrap();
    fs::write(&library, b"library").unwrap();
    fs::write(&destination, b"previous").unwrap();
    let displaced = std::cell::RefCell::new(None);
    let result = publish_bundle("tool", &candidate, &library, &destination, |stage| {
        let bundle = stage.parent().unwrap().parent().unwrap();
        let retained = bundle.with_extension("displaced");
        fs::rename(bundle, &retained).unwrap();
        fs::write(bundle, b"foreign replacement").unwrap();
        displaced.replace(Some(retained));
        Err(InstallError::ExecutableHashMismatch {
            path: stage.to_owned(),
            actual: "rejected".into(),
            expected: "required".into(),
        })
    });
    let Err(InstallError::Io {
        operation,
        path,
        source,
    }) = result
    else {
        panic!("expected retained primary and cleanup evidence");
    };
    assert_eq!(operation, "remove failed tool bundle");
    assert_eq!(fs::read(path).unwrap(), b"foreign replacement");
    assert!(displaced.into_inner().unwrap().is_dir());
    assert_eq!(fs::read(destination).unwrap(), b"previous");
    let error = source
        .get_ref()
        .unwrap()
        .downcast_ref::<BundleCleanupError<InstallError>>()
        .unwrap();
    assert!(matches!(
        error.source,
        InstallError::ExecutableHashMismatch { .. }
    ));
    assert!(error.source().unwrap().is::<InstallError>());
    assert!(error.cleanup.raw_os_error().is_some());
}
