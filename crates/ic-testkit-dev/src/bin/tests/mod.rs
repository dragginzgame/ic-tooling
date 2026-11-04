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
    let versions = workspace_versions(
        root,
        root,
        OutputLimits {
            stdout: OutputLimit::Terminate(1024 * 1024),
            stderr: OutputLimit::Terminate(16 * 1024),
            timeout: None,
        },
    )
    .unwrap();
    let packages = archive_specs(&versions, "foundation\nintegration\n").unwrap();
    assert_eq!(packages[0].version, "0.73.0");
    assert_eq!(packages[1].version, "0.74.0");
    assert!(packages.iter().all(|package| package.patch_registry));
    assert!(archive_specs(&versions, "missing-member").is_err());
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
