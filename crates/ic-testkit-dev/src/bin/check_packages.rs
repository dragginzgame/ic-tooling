//! Qualify exact per-package archives through IC Build's owning engine.

use std::{
    collections::BTreeMap,
    error::Error,
    fmt::Write as _,
    io::Write as _,
    path::Path,
    process::{Command, ExitCode},
};

use ic_build::cargo::observation::{CargoMetadataSpec, decode_metadata};
use ic_build::cargo::packages::{
    PackageArchiveSpec, PackageCommandError, PackageQualificationLimits, capture_package_command,
    qualify_package_archives_with,
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
    let features = CargoFeatureSelection {
        all_features: true,
        ..CargoFeatureSelection::default()
    };
    qualify_package_archives_with(
        workspace,
        &target.join("package"),
        &archives,
        &packages,
        &features,
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
        |root| qualify_archive_tests(root, &target.join("archive-build-cache"), limits.process),
    )?;
    Ok(())
}

fn qualify_archive_tests(
    root: &Path,
    cache: &Path,
    limits: OutputLimits,
) -> Result<(), Box<dyn Error>> {
    // This cache contains only regenerable compiler outputs. Keep its lock
    // across invalidation and both test commands; ordinary workspace artifacts
    // and retained archive attempts are outside the cleanup namespace.
    let _lock = admit_compilation_cache(cache)?;
    std::fs::write(
        root.join("compilation-target"),
        cache.as_os_str().as_encoded_bytes(),
    )?;
    for (stage, args) in [
        ("rebuild-packages", &["clean", "--workspace"][..]),
        (
            "test-targets",
            &["test", "--workspace", "--all-targets"][..],
        ),
        ("test-docs", &["test", "--workspace", "--doc"][..]),
    ] {
        let mut command = Command::new(std::env::var_os("CARGO").unwrap_or_else(|| "cargo".into()));
        command
            .current_dir(root)
            .env_remove("CARGO_TARGET_DIR")
            .args(args)
            .args(["--offline", "--locked", "--manifest-path"])
            .arg(root.join("Cargo.toml"))
            .arg("--target-dir")
            .arg(cache);
        // Workspace-only cleaning rebuilds all extracted packages, including
        // same-version payloads with fixed tar mtimes, retaining registry builds.
        if stage != "rebuild-packages" {
            command.args(["--all-features", "--message-format=json"]);
        }
        std::fs::write(root.join("current-stage"), stage)?;
        capture_package_command(
            &mut command,
            stage,
            &root.join(format!("{stage}.json")),
            &root.join(format!("{stage}.stderr")),
            limits,
        )?;
    }
    Ok(())
}

fn admit_compilation_cache(cache: &Path) -> Result<std::fs::File, Box<dyn Error>> {
    const OWNER: &[u8] =
        b"Signature: 8a477f597d28d172789f06886806bc55\n# ic-tooling-package-qualification/1\n";
    ic_host_fs::path::validate_physical_directory_allow_missing(cache)?;
    let lock =
        ic_host_fs::durable::open_regular_lock_file_with_parents(&cache.with_extension("lock"))?;
    ic_host_fs::durable::lock_exclusive(&lock)?;
    if !cache.exists() {
        std::fs::create_dir(cache)?;
    }
    let marker = cache.join("CACHEDIR.TAG");
    if let Some(owner) = ic_host_fs::read::read_optional_file_no_follow(&marker, OWNER.len())? {
        if owner != OWNER {
            return Err("unrecognized archive compilation cache owner".into());
        }
    } else {
        if std::fs::read_dir(cache)?.next().is_some() {
            return Err("refusing to adopt a nonempty archive compilation cache".into());
        }
        std::fs::OpenOptions::new()
            .write(true)
            .create_new(true)
            .open(marker)?
            .write_all(OWNER)?;
    }
    Ok(lock)
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
