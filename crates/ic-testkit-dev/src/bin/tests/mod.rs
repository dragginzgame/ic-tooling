use super::*;
use ic_host_process::tool::{ExecutionError, ExecutionEvidence, ExecutionFailure, ToolError};

#[test]
fn explicit_archive_selection_preserves_default_and_refuses_private_or_duplicate_names() {
    let roster = "ic-host-text\nic-testkit-native\n";
    assert_eq!(selected_roster(roster, std::iter::empty()).unwrap(), roster);
    assert_eq!(
        selected_roster(roster, ["ic-testkit-native"].into_iter()).unwrap(),
        "ic-testkit-native"
    );
    assert!(selected_roster(roster, ["ic-testkit-dev"].into_iter()).is_err());
    assert!(selected_roster(roster, ["unknown"].into_iter()).is_err());
    assert!(selected_roster(roster, ["ic-host-text", "ic-host-text"].into_iter()).is_err());
}

#[test]
fn archive_selection_preserves_mixed_versions_and_refuses_missing_members() {
    let fixture = ic_host_fs::testing::FixtureDirectory::new(
        &std::env::temp_dir(),
        "archive-version-selection",
    )
    .unwrap();
    let root = fixture.path();
    std::fs::write(
        root.join("Cargo.toml"),
        "[workspace]\nmembers = [\"foundation\", \"integration\"]\nresolver = \"3\"\n",
    )
    .unwrap();
    let mut lock = String::from("version = 4\n");
    for (name, version) in [("foundation", "0.73.0"), ("integration", "0.74.0")] {
        std::fs::create_dir_all(root.join(name).join("src")).unwrap();
        std::fs::write(root.join(name).join("src/lib.rs"), "").unwrap();
        std::fs::write(
            root.join(name).join("Cargo.toml"),
            format!("[package]\nname = \"{name}\"\nversion = \"{version}\"\nedition = \"2024\"\n"),
        )
        .unwrap();
        writeln!(
            lock,
            "\n[[package]]\nname = \"{name}\"\nversion = \"{version}\""
        )
        .unwrap();
    }
    std::fs::write(root.join("Cargo.lock"), lock).unwrap();
    let metadata = workspace_metadata(
        root,
        root,
        OutputLimits {
            stdout: OutputLimit::Terminate(1024 * 1024),
            stderr: OutputLimit::Terminate(16 * 1024),
            timeout: None,
        },
    )
    .unwrap();
    let versions = workspace_versions(&metadata);
    let packages = archive_specs(&versions, "foundation\nintegration\n").unwrap();
    assert_eq!(packages[0].version, "0.73.0");
    assert_eq!(packages[1].version, "0.74.0");
    assert!(packages.iter().all(|package| package.patch_registry));
    assert!(archive_specs(&versions, "missing-member").is_err());
}

fn archive_limits() -> PackageQualificationLimits {
    PackageQualificationLimits {
        process: OutputLimits {
            stdout: OutputLimit::Terminate(4 * 1024 * 1024),
            stderr: OutputLimit::Terminate(1024 * 1024),
            timeout: Some(std::time::Duration::from_secs(60)),
        },
        file_bytes: 4 * 1024 * 1024,
        source_entries: 100,
        source_depth: 10,
    }
}

fn archive_fixture(label: &str) -> ic_host_fs::testing::FixtureDirectory {
    let parent = Path::new(env!("CARGO_MANIFEST_DIR"))
        .parent()
        .unwrap()
        .parent()
        .unwrap()
        .join("target");
    ic_host_fs::testing::FixtureDirectory::new(&parent, label).unwrap()
}

#[test]
fn focused_archives_keep_local_dependencies_without_running_their_tests() {
    let fixture = archive_fixture("archive-local-closure");
    let source = fixture.path().join("source");
    std::fs::create_dir(&source).unwrap();
    std::fs::write(
        source.join("Cargo.toml"),
        "[workspace]\nmembers=['owner','support','private','unrelated']\nresolver='3'\n",
    )
    .unwrap();
    for name in ["owner", "support", "private", "unrelated"] {
        std::fs::create_dir_all(source.join(name).join("src")).unwrap();
        let extra = match name {
            "owner" => {
                "[dependencies]\nrenamed={package='support',path='../support',version='0.1.0'}\n[dev-dependencies]\nprivate={path='../private'}\n"
            }
            "private" => "publish=false\n",
            _ => "",
        };
        std::fs::write(
            source.join(name).join("Cargo.toml"),
            format!("[package]\nname='{name}'\nversion='0.1.0'\nedition='2024'\n{extra}"),
        )
        .unwrap();
        let body = if name == "owner" {
            "pub fn value()->u32 { renamed::value() }\n#[test] fn selected_owner() { assert_eq!(value(), 7); }\n"
        } else {
            "pub fn value()->u32 { 7 }\n#[test] fn unselected_support() { panic!(\"support tests must not run\"); }\n"
        };
        std::fs::write(source.join(name).join("src/lib.rs"), body).unwrap();
    }
    assert!(
        archive_cargo(&source)
            .current_dir(&source)
            .args(["generate-lockfile", "--offline"])
            .status()
            .unwrap()
            .success()
    );
    let limits = archive_limits();
    let mut metadata = workspace_metadata(&source, fixture.path(), limits.process).unwrap();
    let public = "owner\nsupport\nunrelated\n";
    let closure = archive_closure(&metadata, "owner", public).unwrap();
    assert_eq!(closure, "owner\nsupport");
    assert!(archive_closure(&metadata, "missing", public).is_err());
    let versions = workspace_versions(&metadata);
    let packages = archive_specs(&versions, &closure).unwrap();
    let evidence = fixture.path().join("evidence");
    std::fs::create_dir(&evidence).unwrap();
    std::fs::create_dir_all(source.join("target/package")).unwrap();
    let cache = fixture.path().join("cache");
    ic_build::cargo::packages::qualify_package_archives_with(
        &source,
        &source.join("target/package"),
        &evidence,
        &packages,
        &CargoFeatureSelection::default(),
        limits,
        || archive_cargo(&source),
        |archive, destination| {
            let mut command = Command::new("tar");
            command.arg("-xzf").arg(archive).arg("-C").arg(destination);
            command
        },
        |root| qualify_archive_tests(root, &cache, limits.process, Some("owner")),
    )
    .unwrap();
    let output = std::fs::read_to_string(evidence.join("test-targets.json")).unwrap();
    assert!(output.contains("selected_owner ... ok"));
    assert!(!output.contains("unselected_support"));
    let error = qualify_archive_tests(&evidence, &cache, limits.process, None).unwrap_err();
    assert!(
        error
            .downcast_ref::<PackageCommandError>()
            .is_some_and(|error| error.stage == "test-targets")
    );
    assert!(
        std::fs::read_to_string(evidence.join("test-targets.json"))
            .unwrap()
            .contains("unselected_support ... FAILED")
    );
    for kind in [DependencyKind::Normal, DependencyKind::Development] {
        let owner = metadata
            .packages
            .iter_mut()
            .find(|p| p.name == "owner")
            .unwrap();
        let private = owner
            .dependencies
            .iter_mut()
            .find(|d| d.name == "private")
            .unwrap();
        private.kind = kind;
        private.req = "=0.1.0".parse().unwrap();
        assert!(archive_closure(&metadata, "owner", public).is_err());
    }
}

#[test]
fn failure_report_preserves_stage_paths_and_bounds_escaped_capture() {
    let mut stderr = vec![b'x'; 20_000];
    stderr.extend_from_slice(b"\nerror: broken fixture\x1b[31m\n");
    let command = PackageCommandError {
        stage: "test-targets".to_owned(),
        stdout_path: "/evidence/test-targets.json".into(),
        stderr_path: "/evidence/test-targets.stderr".into(),
        execution: Some(ToolError::Execution(Box::new(ExecutionError {
            failure: ExecutionFailure::ExitStatus,
            evidence: ExecutionEvidence {
                stderr,
                ..ExecutionEvidence::default()
            },
            cleanup: None,
        }))),
        retention: Vec::new(),
    };
    let error = ic_build::cargo::packages::PackageQualificationError {
        evidence: "/evidence".into(),
        cause: Box::new(command),
    };
    let report = failure_report(&error);
    assert!(report.contains("package stage test-targets"));
    assert!(report.contains("/evidence/test-targets.stderr"));
    assert!(report.contains("error: broken fixture"));
    assert!(!report.contains('\x1b'));
    assert!(report.len() < 9000);
}

#[cfg(unix)]
fn archive_payload(value: u8) -> String {
    format!(
        "pub fn value()->u8{{{value}}}\n#[test]fn archived_value(){{assert_eq!(value(),42); assert_eq!(itoa::Buffer::new().format(value()),\"42\");}}\n"
    )
}

#[cfg(unix)]
fn archive_cargo(source: &Path) -> Command {
    let mut command = Command::new(std::env::var_os("CARGO").unwrap_or_else(|| "cargo".into()));
    command
        .env("RUSTUP_AUTO_INSTALL", "0")
        .env("CARGO_NET_OFFLINE", "true")
        .env("CARGO_TARGET_DIR", source.join("target"));
    command
}

#[cfg(unix)]
fn archive_cache_fixture() -> ic_host_fs::testing::FixtureDirectory {
    let fixture = archive_fixture("archive-cache");
    let source = fixture.path().join("source");
    std::fs::create_dir_all(source.join("src")).unwrap();
    std::fs::create_dir(source.join("target")).unwrap();
    std::fs::write(source.join("Cargo.toml"), "[workspace]\n[package]\nname='cache-fixture'\nversion='0.1.0'\nedition='2024'\n[dependencies]\nitoa='1.0'\n").unwrap();
    std::fs::write(source.join("src/lib.rs"), archive_payload(42)).unwrap();
    let lock = archive_cargo(&source)
        .current_dir(&source)
        .args(["generate-lockfile", "--offline"])
        .output()
        .unwrap();
    assert!(
        lock.status.success(),
        "{}",
        String::from_utf8_lossy(&lock.stderr)
    );
    std::fs::create_dir(source.join("target/package")).unwrap();
    fixture
}

#[cfg(unix)]
fn reject_fixed_mtime_workspace(root: &Path, cache: &Path, limits: OutputLimits) {
    let source = root.join("cache-fixture-0.1.0/src/lib.rs");
    let modified = std::fs::metadata(&source).unwrap().modified().unwrap();
    std::fs::write(&source, archive_payload(43)).unwrap();
    std::fs::File::open(&source)
        .unwrap()
        .set_modified(modified)
        .unwrap();
    let error = qualify_archive_tests(root, cache, limits, None).unwrap_err();
    assert!(failure_report(error.as_ref()).contains("test-targets"));
    assert!(
        std::fs::read_to_string(root.join("test-targets.json"))
            .unwrap()
            .contains("archived_value ... FAILED")
    );
}

#[cfg(unix)]
#[test]
fn archive_cache_reuses_dependencies_and_rebuilds_same_version_payloads() {
    let fixture = archive_cache_fixture();
    let source = fixture.path().join("source");
    let cache = fixture.path().join("compiler");
    let packages = [PackageArchiveSpec {
        name: "cache-fixture",
        version: "0.1.0",
        patch_registry: true,
    }];
    let features = CargoFeatureSelection {
        all_features: true,
        ..CargoFeatureSelection::default()
    };
    let limits = archive_limits();
    let mut mtimes = Vec::new();
    for attempt in ["cold", "warm", "changed"] {
        if attempt == "changed" {
            std::fs::write(source.join("src/lib.rs"), archive_payload(43)).unwrap();
        }
        let evidence = fixture.path().join(attempt);
        std::fs::create_dir(&evidence).unwrap();
        let started = std::time::Instant::now();
        let result = qualify_package_archives_with(
            &source,
            &source.join("target/package"),
            &evidence,
            &packages,
            &features,
            limits,
            || archive_cargo(&source),
            |archive, destination| {
                let mut command = Command::new("tar");
                command.arg("-xzf").arg(archive).arg("-C").arg(destination);
                command
            },
            |root| qualify_archive_tests(root, &cache, limits.process, None),
        );
        eprintln!("Archive compiler cache {attempt}: {:?}", started.elapsed());
        if attempt != "changed"
            && let Err(error) = &result
        {
            panic!("{}", failure_report(error));
        }
        mtimes.push(
            std::fs::metadata(evidence.join("cache-fixture-0.1.0/src/lib.rs"))
                .unwrap()
                .modified()
                .unwrap(),
        );
        let output = std::fs::read_to_string(evidence.join("test-targets.json")).unwrap();
        let artifacts: Vec<serde_json::Value> = output
            .lines()
            .filter_map(|line| serde_json::from_str::<serde_json::Value>(line).ok())
            .filter(|record| record["reason"] == "compiler-artifact")
            .collect();
        if attempt == "changed" {
            let error = result.unwrap_err();
            assert!(failure_report(&error).contains("test-targets"));
            assert!(
                std::fs::read_to_string(evidence.join("test-targets.json"))
                    .unwrap()
                    .contains("archived_value ... FAILED")
            );
            assert_eq!(
                mtimes[0], mtimes[2],
                "tar timestamps must not hide the changed payload"
            );
        } else {
            result.unwrap_or_else(|error| panic!("{}", failure_report(&error)));
            assert!(
                artifacts
                    .iter()
                    .any(|record| record["target"]["name"] == "itoa"
                        && record["fresh"] == (attempt == "warm")),
                "missing dependency freshness evidence: {output}"
            );
            assert!(evidence.join("final-sources.json").is_file());
        }
        assert!(
            artifacts
                .iter()
                .any(|record| record["target"]["name"] == "cache_fixture"
                    && record["fresh"] == false),
            "extracted package must rebuild: {output}"
        );
        assert!(evidence.join("rebuild-packages.stderr").is_file());
        if attempt == "warm" {
            // Exercise an identical path too: Cargo's mtime freshness alone
            // must not let a cached success hide changed source bytes.
            reject_fixed_mtime_workspace(&evidence, &cache, limits.process);
        }
    }
}

#[cfg(unix)]
#[test]
fn archive_cache_refuses_unowned_or_redirected_artifacts() {
    use std::os::unix::fs::symlink;
    let parent = Path::new(env!("CARGO_MANIFEST_DIR"))
        .parent()
        .unwrap()
        .parent()
        .unwrap()
        .join("target");
    let fixture =
        ic_host_fs::testing::FixtureDirectory::new(&parent, "archive-cache-custody").unwrap();
    let unowned = fixture.path().join("unowned");
    std::fs::create_dir(&unowned).unwrap();
    std::fs::write(unowned.join("keep"), b"user artifact").unwrap();
    assert!(admit_compilation_cache(&unowned).is_err());
    let redirected = fixture.path().join("redirected");
    symlink(&unowned, &redirected).unwrap();
    assert!(admit_compilation_cache(&redirected).is_err());
    let marker_link = fixture.path().join("marker-link");
    std::fs::create_dir(&marker_link).unwrap();
    symlink(unowned.join("keep"), marker_link.join("CACHEDIR.TAG")).unwrap();
    assert!(admit_compilation_cache(&marker_link).is_err());
    assert_eq!(
        std::fs::read(unowned.join("keep")).unwrap(),
        b"user artifact"
    );
}
