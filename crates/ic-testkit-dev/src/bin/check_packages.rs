//! Qualify exact per-package archives through IC Build's owning engine.

use std::{
    collections::BTreeMap,
    error::Error,
    fmt::Write as _,
    path::Path,
    process::{Command, ExitCode},
};

use ic_build::cargo::observation::{CargoMetadataSpec, decode_metadata};
use ic_build::cargo::packages::{
    PackageArchiveSpec, PackageCommandError, PackageQualificationLimits, qualify_package_archives,
};
use ic_build::cargo::selection::CargoFeatureSelection;
use ic_host_process::tool::{CompletedExecution, OutputLimit, OutputLimits, capture_command};

fn main() -> ExitCode {
    match run() {
        Ok(()) => ExitCode::SUCCESS,
        Err(error) => {
            eprintln!("{}", failure_report(error.as_ref()));
            ExitCode::FAILURE
        }
    }
}

fn failure_report(error: &(dyn Error + 'static)) -> String {
    let mut report = format!("Package qualification failed: {error}");
    let mut cause = Some(error);
    while let Some(error) = cause {
        if let Some(command) = error.downcast_ref::<PackageCommandError>() {
            if let Some(evidence) = command
                .execution
                .as_ref()
                .and_then(|error| error.evidence())
            {
                for (label, bytes) in [("stdout", &evidence.stdout), ("stderr", &evidence.stderr)] {
                    let tail = &bytes[bytes.len().saturating_sub(8192)..];
                    let _ = writeln!(report, "\n{label} (last {} captured bytes):", tail.len());
                    for line in String::from_utf8_lossy(tail).lines() {
                        let _ = writeln!(report, "{}", ic_host_text::escape::terminal_text(line));
                    }
                }
            }
            break;
        }
        cause = error.source();
    }
    report
}

fn run() -> Result<(), Box<dyn Error>> {
    clap::Command::new("check_packages")
        .about("Qualify the selected workspace package archives through IC Build")
        .version(env!("CARGO_PKG_VERSION"))
        .get_matches();
    let workspace = Path::new(env!("CARGO_MANIFEST_DIR"))
        .parent()
        .and_then(Path::parent)
        .ok_or("development package requires the repository workspace")?;
    let target = workspace.join("target");
    std::fs::create_dir_all(target.join("package"))?;
    let evidence = ic_host_fs::directory::create_unique_directory(
        &target,
        "package-qualification",
        0o700,
        None,
    )?;
    eprintln!("Package qualification evidence: {}", evidence.display());
    let limits = PackageQualificationLimits {
        process: OutputLimits {
            stdout: OutputLimit::Terminate(64 * 1024 * 1024),
            stderr: OutputLimit::Terminate(64 * 1024 * 1024),
            timeout: None,
        },
        file_bytes: 16 * 1024 * 1024,
        source_entries: 8192,
        source_depth: 64,
    };
    let versions = workspace_versions(workspace, &evidence, limits.process)?;
    let packages = archive_specs(
        &versions,
        include_str!("../../../../ci/public-packages.txt"),
    )?;
    // Build owns an initially empty evidence directory; selection diagnostics
    // remain alongside it in the driver's retained attempt.
    let archives = evidence.join("archives");
    std::fs::create_dir(&archives)?;
    eprintln!(
        "Current archive stage: {}",
        archives.join("current-stage").display()
    );
    qualify_package_archives(
        workspace,
        &target.join("package"),
        &archives,
        &packages,
        &CargoFeatureSelection {
            all_features: true,
            ..CargoFeatureSelection::default()
        },
        limits,
        || {
            let mut command =
                Command::new(std::env::var_os("CARGO").unwrap_or_else(|| "cargo".into()));
            command.env("CARGO_TARGET_DIR", &target);
            command
        },
        |archive, destination| {
            let mut command = Command::new("tar");
            command.arg("-xzf").arg(archive).arg("-C").arg(destination);
            command
        },
    )?;
    Ok(())
}

fn workspace_versions(
    workspace: &Path,
    evidence: &Path,
    limits: OutputLimits,
) -> Result<BTreeMap<String, String>, Box<dyn Error>> {
    let mut command = CargoMetadataSpec {
        manifest_path: &workspace.join("Cargo.toml"),
        include_dependencies: false,
        filter_platform: None,
        locked: true,
        offline: true,
        features: &CargoFeatureSelection::default(),
    }
    .command(Command::new(
        std::env::var_os("CARGO").unwrap_or_else(|| "cargo".into()),
    ))?;
    let observation = capture_command(&mut command, limits)
        .and_then(CompletedExecution::require_success)
        .and_then(CompletedExecution::require_complete);
    let stdout_path = evidence.join("selection.json");
    let stderr_path = evidence.join("selection.stderr");
    let output = observation.map_err(|error| {
        let mut retention = Vec::new();
        if let Some(captured) = error.evidence() {
            for (path, bytes) in [
                (&stdout_path, &captured.stdout),
                (&stderr_path, &captured.stderr),
            ] {
                if let Err(error) = std::fs::write(path, bytes) {
                    retention.push(error);
                }
            }
        }
        PackageCommandError {
            stage: "workspace-selection".to_owned(),
            stdout_path: stdout_path.clone(),
            stderr_path: stderr_path.clone(),
            execution: Some(error),
            retention,
        }
    })?;
    std::fs::write(stdout_path, &output.stdout)?;
    std::fs::write(stderr_path, &output.stderr)?;
    let metadata = decode_metadata(&output.stdout)?;
    Ok(metadata
        .workspace_packages()
        .into_iter()
        .map(|package| (package.name.to_string(), package.version.to_string()))
        .collect())
}

fn archive_specs<'a>(
    versions: &'a BTreeMap<String, String>,
    roster: &'a str,
) -> Result<Vec<PackageArchiveSpec<'a>>, Box<dyn Error>> {
    roster
        .lines()
        .map(|name| {
            let version = versions
                .get(name)
                .ok_or_else(|| format!("public package is not a workspace member: {name}"))?;
            Ok(PackageArchiveSpec {
                name,
                version,
                patch_registry: true,
            })
        })
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;
    use ic_host_process::tool::{ExecutionError, ExecutionEvidence, ExecutionFailure, ToolError};

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
                format!(
                    "[package]\nname = \"{name}\"\nversion = \"{version}\"\nedition = \"2024\"\n"
                ),
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
}
