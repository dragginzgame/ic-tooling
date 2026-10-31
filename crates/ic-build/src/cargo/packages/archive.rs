//! Unix archive qualification over Host descriptor-based traversal.

use super::external_lock::admit_lock;
use super::source_inventory::{SourceFile, sources};
use crate::cargo::observation::{
    metadata_arguments,
    schema::{Metadata, Package},
};
use crate::cargo::selection::CargoFeatureSelection;
use ic_host_artifacts::artifact::Sha256Digest;
use ic_host_process::tool::{CompletedExecution, capture_command};
use ic_host_process::tool::{OutputLimits, ToolError};
#[cfg(test)]
use serde_json::{Value, json};
use std::{
    collections::{BTreeMap, BTreeSet},
    fmt::Write as _,
    fs,
    path::Path,
    process::Command,
};
use std::{error::Error, fmt, io, path::PathBuf};

pub(super) type Cause = Box<dyn Error>;

/// One consumer-selected package and its expected archive identity.
pub struct PackageArchiveSpec<'a> {
    /// Cargo package name.
    pub name: &'a str,
    /// Exact expected Cargo version; coordinated versions remain caller policy.
    pub version: &'a str,
    /// Patch registry references to this extracted package during qualification only.
    pub patch_registry: bool,
}

impl PackageArchiveSpec<'_> {
    pub(super) fn directory_name(&self) -> String {
        format!("{}-{}", self.name, self.version)
    }
}

/// Caller-selected bounds for process output and source admission.
#[derive(Clone, Copy)]
pub struct PackageQualificationLimits {
    /// Cargo/extractor output and optional per-command elapsed-time limit.
    pub process: OutputLimits,
    /// Maximum bytes in each admitted manifest, lock, archive or source file.
    pub file_bytes: usize,
    /// Maximum total directory and regular-file entries across extracted packages.
    pub source_entries: usize,
    /// Maximum directory depth beneath each extracted package root.
    pub source_depth: usize,
}

/// Command failure preserves execution separately from diagnostic-retention failures.
#[derive(Debug)]
pub struct PackageCommandError {
    /// Qualification stage that invoked the command.
    pub stage: String,
    /// Location selected for the command's stdout record.
    pub stdout_path: PathBuf,
    /// Location selected for the command's stderr record.
    pub stderr_path: PathBuf,
    /// Original process/capture failure, or `None` if only log retention failed.
    pub execution: Option<ToolError>,
    /// Independent failures writing the retained stdout/stderr records.
    pub retention: Vec<io::Error>,
}
impl fmt::Display for PackageCommandError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "package stage {}: ", self.stage)?;
        if let Some(error) = &self.execution {
            error.fmt(f)?;
        } else {
            f.write_str("diagnostic retention failed")?;
        }
        write!(
            f,
            "\nstdout log destination: {}\nstderr log destination: {}",
            self.stdout_path.display(),
            self.stderr_path.display()
        )?;
        for error in &self.retention {
            write!(f, "\nlog retention failed: {error}")?;
        }
        Ok(())
    }
}
impl Error for PackageCommandError {
    fn source(&self) -> Option<&(dyn Error + 'static)> {
        self.execution
            .as_ref()
            .map(|error| error as &dyn Error)
            .or_else(|| self.retention.first().map(|error| error as &dyn Error))
    }
}

/// Failed qualification retains its original cause and evidence directory.
#[derive(Debug, thiserror::Error)]
#[error("{cause}")]
pub struct PackageQualificationError {
    /// The caller-selected directory, retained even on failure.
    pub evidence: PathBuf,
    /// Original I/O, encoding, process or admission failure.
    #[source]
    pub cause: Cause,
}

/// Package a selection and test its exact extracted graph with frozen external selections.
///
/// Available on Unix; the pure [`super::verify_external_lock_subset`] remains portable.
///
/// `evidence` must be an existing empty private directory with trusted ancestors.
/// It and all archives/logs are retained on success or failure. `archive_root`
/// must match the selected Cargo packaging target. Cargo and extraction commands
/// are supplied by the caller, including their environment and executable custody.
/// The extractor receives a Cargo-produced archive and the evidence destination;
/// it must safely extract that archive without selecting other inputs or effects.
/// This API does not admit untrusted third-party tar files or sandbox tools.
/// Exclude concurrent writers to original inputs, archives and extracted sources.
///
/// Each attempt uses its own `evidence/target`: fixed archive mtimes must never
/// permit another same-version payload's artifacts to satisfy this verification.
/// Only the temporary verification workspace receives path patches. Original
/// manifests/lock, archive bytes, extracted source bytes/modes and admitted
/// verification manifest/lock are compared after all-target and documentation tests.
/// Commands execute once, offline; resolution may update only the verification lock
/// before admission. Every external package/checksum must already be frozen in the
/// original lock. No install, publication, source mutation, cleanup or retry occurs.
/// `features` selects the same graph for packaging, extracted metadata and tests.
/// Named features use Cargo's workspace `package/feature` syntax when needed.
///
/// # Errors
/// Retains exact process failure (including status/diagnostics), missing inputs,
/// bounded decoding, graph/lock refusals and final input drift in the evidence root.
#[expect(
    clippy::too_many_arguments,
    reason = "tools, paths, graph selection and budgets remain explicit caller inputs"
)]
pub fn qualify_package_archives(
    workspace: &Path,
    archive_root: &Path,
    evidence: &Path,
    packages: &[PackageArchiveSpec<'_>],
    features: &CargoFeatureSelection,
    limits: PackageQualificationLimits,
    mut cargo: impl FnMut() -> Command,
    mut extractor: impl FnMut(&Path, &Path) -> Command,
) -> Result<(), PackageQualificationError> {
    PreparedQualification::prepare(
        workspace,
        archive_root,
        evidence,
        packages,
        features,
        limits,
    )
    .and_then(|prepared| prepared.run(&mut cargo, &mut extractor))
    .map_err(|cause| PackageQualificationError {
        evidence: evidence.to_owned(),
        cause,
    })
}

// Paths and selection are admitted before the attempt writes evidence or runs commands.
struct PreparedQualification<'a> {
    workspace: PathBuf,
    archive_root: PathBuf,
    evidence: PathBuf,
    packages: &'a [PackageArchiveSpec<'a>],
    features: &'a CargoFeatureSelection,
    limits: PackageQualificationLimits,
}

impl<'a> PreparedQualification<'a> {
    fn prepare(
        workspace: &Path,
        archive_root: &Path,
        evidence: &Path,
        packages: &'a [PackageArchiveSpec<'a>],
        features: &'a CargoFeatureSelection,
        limits: PackageQualificationLimits,
    ) -> Result<Self, Cause> {
        if packages.is_empty() || limits.file_bytes == 0 || limits.source_entries == 0 {
            return Err("empty package selection or admission budget".into());
        }
        let mut names = BTreeSet::new();
        for package in packages {
            if !identifier(package.name)
                || !identifier(package.version)
                || !names.insert(package.name)
            {
                return Err("invalid or duplicate package archive identity".into());
            }
        }
        let workspace = workspace.canonicalize()?;
        let archive_root = archive_root.canonicalize()?;
        let evidence = evidence.canonicalize()?;
        for path in [&workspace, &archive_root, &evidence] {
            if path
                .as_os_str()
                .as_encoded_bytes()
                .iter()
                .any(|b| matches!(b, b'\r' | b'\n'))
            {
                return Err("invalid directory namespace".into());
            }
        }
        if fs::read_dir(&evidence)?.next().is_some() {
            return Err("evidence directory is not empty".into());
        }
        Ok(Self {
            workspace,
            archive_root,
            evidence,
            packages,
            features,
            limits,
        })
    }

    #[expect(
        clippy::too_many_lines,
        reason = "ordered archive qualification retains one before/after custody boundary"
    )]
    fn run(
        self,
        cargo: &mut impl FnMut() -> Command,
        extractor: &mut impl FnMut(&Path, &Path) -> Command,
    ) -> Result<(), Cause> {
        let Self {
            workspace,
            archive_root,
            evidence,
            packages,
            features,
            limits,
        } = self;
        let lock = workspace.join("Cargo.lock");
        let entry_lock = read(&lock, limits.file_bytes)?;
        fs::write(evidence.join("entry-Cargo.lock"), &entry_lock)?;
        let mut command = cargo();
        command
            .current_dir(&workspace)
            .args(metadata_arguments(false, None))
            .args(["--offline", "--locked"]);
        let metadata = execute(&mut command, &evidence, "entry-metadata", limits.process)?;
        let metadata = serde_json::from_slice(&metadata)?;
        let mut originals = BTreeMap::new();
        originals.insert(
            workspace.join("Cargo.toml"),
            read(&workspace.join("Cargo.toml"), limits.file_bytes)?,
        );
        for spec in packages {
            let selected = package(&metadata, spec.name)?;
            if selected.version.to_string() != spec.version {
                return Err("entry package version differs from selection".into());
            }
            let path = selected.manifest_path.clone().into_std_path_buf();
            originals.insert(path.clone(), read(&path, limits.file_bytes)?);
        }
        for (index, bytes) in originals.values().enumerate() {
            fs::write(evidence.join(format!("entry-manifest-{index}")), bytes)?;
        }
        let mut command = cargo();
        command.current_dir(&workspace).args([
            "package",
            "--offline",
            "--locked",
            "--allow-dirty",
            "--no-verify",
        ]);
        for spec in packages {
            command.args(["-p", spec.name]);
        }
        command.args(features.arguments());
        execute(&mut command, &evidence, "package", limits.process)?;
        let mut archives = BTreeMap::new();
        for (index, spec) in packages.iter().enumerate() {
            let path = archive_root.join(format!("{}.crate", spec.directory_name()));
            archives.insert(path.clone(), hash(&path, limits.file_bytes)?);
            execute(
                &mut extractor(&path, &evidence),
                &evidence,
                &format!("extract-{index}"),
                limits.process,
            )?;
        }
        let mut archive_hashes = String::new();
        for digest in archives.values() {
            writeln!(archive_hashes, "{digest}")?;
        }
        fs::write(evidence.join("archive.sha256"), archive_hashes)?;
        let sources = sources(&evidence, packages, limits)?;
        fs::write(
            evidence.join("entry-sources.json"),
            serde_json::to_vec(&sources)?,
        )?;
        let manifest = verification_manifest(packages)?;
        let manifest_path = evidence.join("Cargo.toml");
        let payload_lock = evidence.join("Cargo.lock");
        fs::write(&manifest_path, &manifest)?;
        fs::write(&payload_lock, &entry_lock)?;
        let mut verification_cargo = || {
            let mut command = cargo();
            command
                .current_dir(&evidence)
                .env("CARGO_TARGET_DIR", evidence.join("target"));
            command
        };
        let mut command = verification_cargo();
        command
            .args(metadata_arguments(true, None))
            .args(["--offline", "--manifest-path"])
            .arg(&manifest_path)
            .args(features.arguments());
        let metadata = execute(&mut command, &evidence, "metadata", limits.process)?;
        admit_graph(&serde_json::from_slice(&metadata)?, &evidence, packages)?;
        let admitted_lock = read(&payload_lock, limits.file_bytes)?;
        admit_lock(&entry_lock, &admitted_lock)?;
        fs::write(evidence.join("admitted-Cargo.toml"), &manifest)?;
        fs::write(evidence.join("admitted-Cargo.lock"), &admitted_lock)?;
        let baseline = QualificationBaseline {
            original_lock: (lock, entry_lock),
            originals,
            archives,
            sources,
            verification_manifest: manifest,
            admitted_lock,
        };
        for (label, selection) in [("test-targets", "--all-targets"), ("test-docs", "--doc")] {
            let mut command = verification_cargo();
            command
                .args(["test", "--offline", "--locked", "--manifest-path"])
                .arg(&manifest_path)
                .args(["--workspace", selection])
                .args(features.arguments());
            execute(&mut command, &evidence, label, limits.process)?;
        }
        baseline.verify(&evidence, packages, limits)
    }
}

// The entry and admitted observations belong to one qualification attempt.
// Revalidation remains explicit and ordered after all qualification commands.
struct QualificationBaseline {
    original_lock: (PathBuf, Vec<u8>),
    originals: BTreeMap<PathBuf, Vec<u8>>,
    archives: BTreeMap<PathBuf, Sha256Digest>,
    sources: BTreeMap<PathBuf, SourceFile>,
    verification_manifest: String,
    admitted_lock: Vec<u8>,
}

impl QualificationBaseline {
    fn verify(
        self,
        evidence: &Path,
        packages: &[PackageArchiveSpec<'_>],
        limits: PackageQualificationLimits,
    ) -> Result<(), Cause> {
        if read(&self.original_lock.0, limits.file_bytes)? != self.original_lock.1 {
            return Err("original lock changed".into());
        }
        for (path, bytes) in self.originals {
            if read(&path, limits.file_bytes)? != bytes {
                return Err("original manifest changed".into());
            }
        }
        for (path, digest) in self.archives {
            if hash(&path, limits.file_bytes)? != digest {
                return Err("package archive changed".into());
            }
        }
        let final_sources = self::sources(evidence, packages, limits)?;
        fs::write(
            evidence.join("final-sources.json"),
            serde_json::to_vec(&final_sources)?,
        )?;
        if final_sources != self.sources {
            return Err("extracted package sources changed".into());
        }
        if read(&evidence.join("Cargo.toml"), limits.file_bytes)?
            != self.verification_manifest.as_bytes()
            || read(&evidence.join("Cargo.lock"), limits.file_bytes)? != self.admitted_lock
        {
            return Err("admitted verification manifest or lock changed".into());
        }
        Ok(())
    }
}

fn verification_manifest(packages: &[PackageArchiveSpec<'_>]) -> Result<String, toml::ser::Error> {
    let members = packages
        .iter()
        .map(PackageArchiveSpec::directory_name)
        .collect::<Vec<_>>();
    let patches: toml::Table = packages
        .iter()
        .filter(|p| p.patch_registry)
        .map(|p| {
            let entry = toml::toml! { path = (p.directory_name()) };
            (p.name.to_owned(), toml::Value::Table(entry))
        })
        .collect();
    toml::to_string(&toml::toml! {
        [workspace]
        resolver = "3"
        members = (members)
        [patch]
        crates-io = (patches)
    })
}

fn identifier(value: &str) -> bool {
    !value.is_empty()
        && value
            .bytes()
            .all(|b| b.is_ascii_alphanumeric() || matches!(b, b'-' | b'_' | b'.' | b'+'))
}

fn read(path: &Path, limit: usize) -> Result<Vec<u8>, Cause> {
    Ok(ic_host_fs::read::read_file_no_follow(path, limit)?)
}
fn hash(path: &Path, limit: usize) -> Result<Sha256Digest, Cause> {
    Ok(ic_host_fs::read::hash_file_no_follow(path, u64::try_from(limit)?)?.sha256)
}

fn execute(
    command: &mut Command,
    evidence: &Path,
    label: &str,
    limits: OutputLimits,
) -> Result<Vec<u8>, Cause> {
    // Retain intent before a potentially long command; output remains bounded capture.
    fs::write(evidence.join("current-stage"), label)?;
    Ok(capture_package_command(
        command,
        label,
        &evidence.join(format!("{label}.json")),
        &evidence.join(format!("{label}.stderr")),
        limits,
    )
    .map_err(|error| error as Cause)?
    .stdout)
}

/// Capture a successful, complete package command and retain both output streams.
///
/// Callers select the stage, existing output directories and capture bounds.
/// Both log writes are attempted even when execution or the first write fails.
/// This does not create directories or update a qualification stage marker.
///
/// # Errors
/// Returns the original execution failure together with any independent log-write
/// failures, or a retention-only failure when a successful command cannot be logged.
pub fn capture_package_command(
    command: &mut Command,
    stage: &str,
    stdout_path: &Path,
    stderr_path: &Path,
    limits: OutputLimits,
) -> Result<CompletedExecution, Box<PackageCommandError>> {
    let result = capture_command(command, limits)
        .and_then(CompletedExecution::require_success)
        .and_then(CompletedExecution::require_complete);
    let retention = retain_command_output(&result, stdout_path, stderr_path);
    match result {
        Ok(output) if retention.is_empty() => Ok(output),
        other => Err(Box::new(PackageCommandError {
            stage: stage.to_owned(),
            stdout_path: stdout_path.to_owned(),
            stderr_path: stderr_path.to_owned(),
            execution: other.err(),
            retention,
        })),
    }
}

fn retain_command_output(
    result: &Result<CompletedExecution, ToolError>,
    stdout_path: &Path,
    stderr_path: &Path,
) -> Vec<io::Error> {
    let (stdout, stderr): (&[u8], &[u8]) = match result {
        Ok(output) => (&output.stdout, &output.stderr),
        Err(error) => error.evidence().map_or((&[][..], &[][..]), |evidence| {
            (evidence.stdout.as_slice(), evidence.stderr.as_slice())
        }),
    };
    [
        fs::write(stdout_path, stdout),
        fs::write(stderr_path, stderr),
    ]
    .into_iter()
    .filter_map(Result::err)
    .collect()
}

fn package<'a>(metadata: &'a Metadata, name: &str) -> Result<&'a Package, Cause> {
    let mut matching = metadata.packages.iter().filter(|p| p.name == name);
    match (matching.next(), matching.next()) {
        (Some(package), None) => Ok(package),
        _ => Err(format!("expected one extracted package: {name}").into()),
    }
}

fn admit_graph(
    metadata: &Metadata,
    root: &Path,
    packages: &[PackageArchiveSpec<'_>],
) -> Result<(), Cause> {
    let members = &metadata.workspace_members;
    let mut selected = BTreeSet::new();
    for spec in packages {
        let package = package(metadata, spec.name)?;
        let manifest = root
            .join(spec.directory_name())
            .join("Cargo.toml")
            .canonicalize()?;
        if package.version.to_string() != spec.version
            || package.source.is_some()
            || package.manifest_path.as_std_path() != manifest
            || !members.contains(&package.id)
        {
            return Err(format!(
                "resolved package is not the exact extracted payload: {}",
                spec.name
            )
            .into());
        }
        selected.insert(&package.id);
    }
    if members.len() != packages.len() || selected.len() != members.len() {
        return Err("unexpected extracted workspace member".into());
    }
    Ok(())
}

#[cfg(test)]
mod tests;
