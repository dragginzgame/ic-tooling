//! Explicit, release-bound `PocketIC` assets for Rust fixtures and the CLI.
//!
//! [`check`] is offline and never sets up a missing bundle. [`setup`] explicitly
//! provisions it. Callers retain namespace custody and select the bundle root;
//! managed startup and application safety remain separate responsibilities.

use super::POCKET_IC_SERVER_VERSION;
use ic_host_artifacts::artifact::{Sha256Digest, copy_gzip, hash_gzip, hash_reader};
use ic_host_fs::path::validate_physical_directory_allow_missing;
use ic_host_fs::{durable, read::read_file_no_follow};
use ic_host_process::{
    child::CleanupPolicy,
    tool::{
        AdmittedTool, ExecutionContext, OutputLimit, OutputLimits, VersionSpec,
        capture_group_command_with_cancellation,
    },
};
use std::{
    error::Error,
    ffi::OsString,
    fs::{self, File},
    io,
    path::{Path, PathBuf},
    process::{Command, Stdio},
    sync::atomic::{AtomicU64, Ordering},
    time::Duration,
};

type Result<T> = std::result::Result<T, Box<dyn Error>>;
const ARCHIVE_LIMIT: usize = 80 * 1024 * 1024;
const EXECUTABLE_LIMIT: usize = 512 * 1024 * 1024;
static SEQUENCE: AtomicU64 = AtomicU64::new(0);
/// Default bundle root used by `ic-testkit-server`.
pub const DEFAULT_DIRECTORY: &str = ".tools/ic-testkit-server";

struct Asset {
    host: &'static str,
    digest: Sha256Digest,
    github_id: u64,
}

fn write_evidence(path: &Path, bytes: &[u8]) -> Result<()> {
    durable::write_bytes(
        path,
        durable::WriteOptions {
            mode: durable::PublicationMode::Replace,
            permissions: 0o666,
        },
        bytes,
    )?;
    Ok(())
}

fn selected_asset() -> Result<Asset> {
    // Official digests reviewed 2026-10-09; matching GitHub asset IDs verified
    // 2026-11-08. Admission still authenticates both archive and executable bytes.
    let (host, digest, github_id) = match (std::env::consts::OS, std::env::consts::ARCH) {
        ("linux", "x86_64") => (
            "x86_64-linux",
            "131219d90dcf9bf6f3ed8ee02d8f55504ea24a6db382f5680675b7bd6b7ed3bf",
            621_918_359,
        ),
        ("macos", "x86_64") => (
            "x86_64-darwin",
            "af9ad2d781530a43556ef2d1c8f93db99a2425c6cabdc78520f61922399ed530",
            621_918_384,
        ),
        ("macos", "aarch64") => (
            "arm64-darwin",
            "9ae843fbb7ae6c6eb30137671c8629a80ef53b3a3652eb85b344847ac39f9fcd",
            621_918_361,
        ),
        _ => return Err("no reviewed PocketIC server asset for this host".into()),
    };
    Ok(Asset {
        host,
        digest: digest.parse()?,
        github_id,
    })
}

fn root_path(directory: &Path) -> Result<PathBuf> {
    if directory.as_os_str().is_empty() {
        return Err("server directory must not be empty".into());
    }
    let absolute = if directory.is_absolute() {
        directory.to_owned()
    } else {
        std::env::current_dir()?.join(directory)
    };
    if absolute
        .as_os_str()
        .as_encoded_bytes()
        .iter()
        .any(|byte| matches!(byte, b'\n' | b'\r'))
    {
        return Err("server directory and its ancestors must not contain line breaks".into());
    }
    validate_physical_directory_allow_missing(&absolute)?;
    Ok(absolute.components().collect())
}

fn bundle_path(root: &Path, asset: &Asset) -> PathBuf {
    root.join(format!("{}-{}", POCKET_IC_SERVER_VERSION, asset.host))
}

fn authenticated_archive(archive: &Path, digest: Sha256Digest) -> Result<Vec<u8>> {
    let bytes = read_file_no_follow(archive, ARCHIVE_LIMIT)?;
    hash_reader(bytes.as_slice(), ARCHIVE_LIMIT as u64)?.verify(digest)?;
    Ok(bytes)
}

fn admit(executable: &Path, digest: Sha256Digest) -> Result<()> {
    let directory = executable.parent().ok_or("missing executable parent")?;
    AdmittedTool::admit_no_follow(
        &VersionSpec {
            executable,
            executable_bytes: EXECUTABLE_LIMIT as u64,
            version_arguments: &[OsString::from("--version")],
        },
        digest,
        &format!("pocket-ic-server {POCKET_IC_SERVER_VERSION}"),
        &ExecutionContext {
            current_dir: directory,
            environment: &[],
        },
        OutputLimits {
            stdout: OutputLimit::Terminate(4096),
            stderr: OutputLimit::Terminate(4096),
            timeout: Some(Duration::from_secs(10)),
        },
    )?;
    Ok(())
}

fn check_bundle(bundle: &Path, asset: &Asset) -> Result<PathBuf> {
    let metadata = fs::symlink_metadata(bundle)?;
    if !metadata.is_dir() || metadata.is_symlink() {
        return Err("PocketIC bundle must be a physical directory".into());
    }
    let bytes = authenticated_archive(&bundle.join("archive.gz"), asset.digest)?;
    let digest = hash_gzip(&bytes, ARCHIVE_LIMIT, EXECUTABLE_LIMIT as u64)?.sha256;
    drop(bytes);
    let executable = bundle.join("pocket-ic");
    admit(&executable, digest)?;
    Ok(executable)
}

/// Admit an existing reviewed bundle without downloading or changing it.
///
/// Authenticates the bounded archive before decoding, compares the executable's
/// exact bytes, then runs bounded `--version` admission with an empty environment.
/// Returns the absolute executable path, not ongoing custody of its namespace.
/// Callers must prevent replacement between admission and managed startup.
///
/// # Errors
/// Rejects unsupported hosts, redirected paths, missing or modified bundles,
/// invalid archives, or failed executable/version admission.
pub fn check(directory: &Path) -> Result<PathBuf> {
    let asset = selected_asset()?;
    check_bundle(&bundle_path(&root_path(directory)?, &asset), &asset)
}

/// Explicitly prepare or admit the reviewed bundle, retaining failed attempts.
///
/// Downloads only when the selected bundle is absent. Cooperating setups share
/// one lock; existing bundles must pass the same offline [`check`] admission.
/// `interrupted` lets the caller request bounded downloader cancellation without
/// this library installing process-global signal handlers.
///
/// # Errors
/// Returns path, lock, download, authentication, decoding, publication or
/// executable/version failures. Retained evidence is never automatically removed.
pub fn setup(directory: &Path, mut interrupted: impl FnMut() -> bool) -> Result<PathBuf> {
    let asset = selected_asset()?;
    setup_with(directory, &asset, |archive| {
        let url = format!(
            "https://api.github.com/repos/dfinity/pocketic/releases/assets/{}",
            asset.github_id
        );
        let mut command = Command::new("curl");
        command
            .args([
                "--proto",
                "=https",
                "--proto-redir",
                "=https",
                "--tlsv1.2",
                "--fail",
                "--silent",
                "--show-error",
                "--location",
                "--header",
                "Accept: application/octet-stream",
                "--connect-timeout",
                "15",
                "--max-time",
                "300",
                "--max-filesize",
                "83886080",
                "--output",
            ])
            .arg(archive)
            .arg(url)
            .stdin(Stdio::null())
            .stdout(Stdio::piped())
            .stderr(Stdio::piped());
        let outcome = capture_group_command_with_cancellation(
            &mut command,
            OutputLimits {
                stdout: OutputLimit::Terminate(4096),
                stderr: OutputLimit::Terminate(64 * 1024),
                timeout: Some(Duration::from_secs(310)),
            },
            CleanupPolicy::KillAndWait,
            None,
            &mut interrupted,
        )
        .and_then(ic_host_process::tool::CompletedExecution::require_success);
        let stderr = match &outcome {
            Ok(completed) => Some(completed.stderr.as_slice()),
            Err(error) => error.evidence().map(|evidence| evidence.stderr.as_slice()),
        };
        if let Some(stderr) = stderr
            && let Err(record_error) = write_evidence(
                &archive
                    .parent()
                    .ok_or("missing archive parent")?
                    .join("download.stderr"),
                stderr,
            )
        {
            if outcome.is_ok() {
                return Err(record_error);
            }
            // Recording evidence must not replace the failed download's cause.
            eprintln!("could not record download stderr: {record_error}");
        }
        outcome?;
        Ok(())
    })
}

fn setup_with(
    directory: &Path,
    asset: &Asset,
    download: impl FnOnce(&Path) -> Result<()>,
) -> Result<PathBuf> {
    let root = root_path(directory)?;
    // Namespace custody remains with the caller. All cooperating setups serialize
    // through Host's admitted regular lock; checks are read-only.
    let lock = durable::open_regular_lock_file_with_parents(&root.join(".setup-v1.lock"))?;
    durable::lock_exclusive(&lock)?;
    let bundle = bundle_path(&root, asset);
    let attempt = root.join(format!(
        ".setup-v1-{}-{}",
        std::process::id(),
        SEQUENCE.fetch_add(1, Ordering::Relaxed)
    ));
    let staging_directory = match fs::symlink_metadata(&bundle) {
        Ok(_) => {
            check_bundle(&bundle, asset)?;
            None
        }
        Err(error) if error.kind() == io::ErrorKind::NotFound => {
            let held = durable::create_private_directory(&attempt)?;
            eprintln!("PocketIC setup evidence: {}", attempt.display());
            if let Err(error) = install(&attempt, asset, download) {
                if let Err(record_error) =
                    write_evidence(&attempt.join("failure.txt"), error.to_string().as_bytes())
                {
                    eprintln!("could not record setup failure: {record_error}");
                }
                return Err(error);
            }
            Some(held)
        }
        Err(error) => return Err(error.into()),
    };
    // Host owns no-replace publication and phase-aware durability failures.
    // Fresh staging must retain its original directory identity. Recovery admits
    // the installed bundle again before Host synchronizes its parent.
    durable::commit_existing_directory_with(
        &attempt,
        &bundle,
        |selected, path| -> io::Result<()> {
            if let Some(held) = &staging_directory {
                durable::admit_directory(path, held)?;
            }
            selected.sync_all()?;
            Ok(())
        },
        |_, _| Ok(()),
    )?;
    eprintln!("PocketIC setup evidence retained: {}", bundle.display());
    Ok(bundle.join("pocket-ic"))
}

fn install(
    attempt: &Path,
    asset: &Asset,
    download: impl FnOnce(&Path) -> Result<()>,
) -> Result<()> {
    let archive = attempt.join("archive.gz");
    download(&archive)?;
    let bytes = authenticated_archive(&archive, asset.digest)?;
    File::open(&archive)?.sync_all()?;
    let executable = attempt.join("pocket-ic");
    let identity = durable::write_with(
        &executable,
        durable::WriteOptions {
            mode: durable::PublicationMode::CreateNew,
            permissions: 0o755,
        },
        |file| copy_gzip(&bytes, file, ARCHIVE_LIMIT, EXECUTABLE_LIMIT as u64),
    )?;
    drop(bytes);
    // PocketIC requires its canonical basename. The whole directory is private
    // and unselected until this admission succeeds; no existing bundle is replaced.
    if let Err(error) = admit(&executable, identity.sha256) {
        if let Some(evidence) = error
            .downcast_ref::<ic_host_process::tool::ToolError>()
            .and_then(|error| error.evidence())
        {
            for (name, bytes) in [
                ("version.stdout", &evidence.stdout),
                ("version.stderr", &evidence.stderr),
            ] {
                if let Err(record_error) = write_evidence(&attempt.join(name), bytes) {
                    eprintln!("could not record version evidence: {record_error}");
                }
            }
        }
        return Err(error);
    }
    Ok(())
}

#[cfg(test)]
mod tests;
