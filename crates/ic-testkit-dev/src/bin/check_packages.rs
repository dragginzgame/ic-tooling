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
    PackageArchiveSpec, PackageCommandError, PackageQualificationLimits, capture_package_command,
    qualify_package_archives,
};
use ic_build::cargo::selection::CargoFeatureSelection;
use ic_host_process::tool::{OutputLimit, OutputLimits};

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
    let arguments = clap::Command::new("check_packages")
        .about("Qualify the selected workspace package archives through IC Build")
        .version(env!("CARGO_PKG_VERSION"))
        .arg(
            clap::Arg::new("package")
                .long("package")
                .value_name("PUBLIC_PACKAGE")
                .action(clap::ArgAction::Append),
        )
        .get_matches();
    let roster = selected_roster(
        include_str!("../../../../ci/public-packages.txt"),
        arguments
            .get_many::<String>("package")
            .into_iter()
            .flatten()
            .map(String::as_str),
    )?;
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
    let packages = archive_specs(&versions, &roster)?;
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

fn selected_roster<'a>(
    roster: &str,
    requested: impl Iterator<Item = &'a str>,
) -> Result<String, Box<dyn Error>> {
    let mut selected = Vec::new();
    for name in requested {
        if !roster.lines().any(|eligible| eligible == name) {
            return Err(format!("unknown or private archive package: {name}").into());
        }
        if selected.contains(&name) {
            return Err(format!("duplicate archive package: {name}").into());
        }
        selected.push(name);
    }
    Ok(if selected.is_empty() {
        roster.to_owned()
    } else {
        selected.join("\n")
    })
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
    let output = capture_package_command(
        &mut command,
        "workspace-selection",
        &evidence.join("selection.json"),
        &evidence.join("selection.stderr"),
        limits,
    )
    .map_err(|error| error as Box<dyn Error>)?;
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
mod tests;
