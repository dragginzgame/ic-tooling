//! Shared mechanics for installing repository-pinned host tools.
//!
//! Tool owners supply archive authority and executable admission. This module owns
//! verified local-archive staging and durable publication. Callers own acquisition,
//! extraction, versions and trust rules.

#[cfg(test)]
mod tests;

use ic_host_fs::{
    durable::{NamedWriteError, PublicationMode, WriteOptions, write_validated_with},
    owned_directory::OwnedDirectory,
};
#[cfg(unix)]
use std::os::unix::fs::PermissionsExt;
use std::{
    fs::{self, File, OpenOptions},
    io,
    path::{Path, PathBuf},
    sync::atomic::{AtomicU64, Ordering},
};

const TEMP_ATTEMPTS: usize = 64;
static TEMP_SEQUENCE: AtomicU64 = AtomicU64::new(0);

/// Caller-admitted local archive and selected members. Acquisition/retry policy stays with callers.
pub struct InstallSpec<'a> {
    /// Single-component executable name.
    pub tool: &'a str,
    /// Exact local archive bytes to verify before extraction.
    pub archive: &'a Path,
    /// Expected archive SHA-256.
    pub archive_sha256: &'static str,
    /// Relative archive member containing the executable.
    pub member: &'a str,
    /// Optional relative runtime-library member, published under lib/ with its basename.
    pub runtime_library: Option<&'a str>,
}

///
/// `InstallError`
///
/// Mechanical failures translated into each tool's existing diagnostic type.
///
/// Failure before or after a validated tool publication.
#[derive(Debug, thiserror::Error)]
pub enum InstallError {
    /// Archive bytes differ from the caller's pin.
    #[error("archive hash mismatch at {path}: expected {expected}, observed {actual}")]
    ArchiveHashMismatch {
        /// Selected archive.
        path: PathBuf,
        /// Observed hash.
        actual: String,
        /// Caller pin.
        expected: &'static str,
    },
    /// Staging changed exact executable or library bytes.
    #[error("executable hash mismatch at {path}: expected {expected}, observed {actual}")]
    ExecutableHashMismatch {
        /// Selected staged member.
        path: PathBuf,
        /// Observed hash.
        actual: String,
        /// Admitted hash.
        expected: String,
    },
    /// Filesystem failure, retaining the underlying publication evidence.
    #[error("{operation} at {path}: {source}")]
    Io {
        /// Failed operation.
        operation: &'static str,
        /// Affected path.
        path: PathBuf,
        /// Original cause.
        source: io::Error,
    },
    /// All bounded scratch names were occupied.
    #[error("temporary installation names exhausted beneath {root}")]
    TempDirectoryExhausted {
        /// Caller-selected scratch root.
        root: PathBuf,
    },
}

/// Admit both candidate and staged executable before publishing, then admit the installed path.
/// # Errors
/// Archive hash, staging, publication or caller admission/extraction failures.
pub fn install<T, E: From<InstallError> + std::error::Error + Send + Sync + 'static>(
    spec: &InstallSpec<'_>,
    destination: &Path,
    scratch_root: &Path,
    extract: impl FnOnce(&Path, &Path) -> Result<(), E>,
    admit: impl Fn(&Path) -> Result<T, E>,
) -> Result<T, E> {
    for member in [Some(spec.tool), Some(spec.member), spec.runtime_library]
        .into_iter()
        .flatten()
    {
        if member.is_empty()
            || !Path::new(member)
                .components()
                .all(|part| matches!(part, std::path::Component::Normal(_)))
        {
            return Err(io_error(
                "validate archive member",
                Path::new(member),
                io::Error::new(
                    io::ErrorKind::InvalidInput,
                    "archive member must be a relative normal path",
                ),
            )
            .into());
        }
    }
    if Path::new(spec.tool).components().count() != 1 {
        return Err(io_error(
            "validate executable name",
            Path::new(spec.tool),
            io::Error::from(io::ErrorKind::InvalidInput),
        )
        .into());
    }
    verify_archive(spec.archive, spec.archive_sha256)?;
    let temp = installation_scratch(scratch_root, spec.tool)?;
    extract(spec.archive, temp.path())?;
    let candidate = temp.path().join(spec.member);
    admit(&candidate)?;
    if let Some(library) = spec.runtime_library {
        publish_bundle(
            spec.tool,
            &candidate,
            &temp.path().join(library),
            destination,
            |path| admit(path).map(|_| ()),
        )?;
    } else {
        publish_executable(&candidate, destination, |path| admit(path).map(|_| ()))?;
    }
    admit(destination)
}

fn verify_archive(archive: &Path, expected: &'static str) -> Result<(), InstallError> {
    let actual = sha256_file(archive)?;
    if actual != expected {
        return Err(InstallError::ArchiveHashMismatch {
            path: archive.to_path_buf(),
            actual,
            expected,
        });
    }
    Ok(())
}

/// Publish exact candidate bytes only after closing the writer and admitting the staged path.
/// # Errors
/// Staging, identity or admission failures; publication errors retain their original cause.
pub fn publish_executable<E: From<InstallError> + std::error::Error + Send + Sync + 'static>(
    candidate: &Path,
    destination: &Path,
    admit: impl FnOnce(&Path) -> Result<(), E>,
) -> Result<(), E> {
    let expected = sha256_file(candidate)?;
    write_validated_with(
        destination,
        WriteOptions {
            mode: PublicationMode::Replace,
            permissions: 0o755,
        },
        |output| {
            let mut source = File::open(candidate)
                .map_err(|source| io_error("open admitted tool executable", candidate, source))?;
            io::copy(&mut source, output)
                .map_err(|source| io_error("write staged tool executable", destination, source))?;
            #[cfg(unix)]
            output
                .set_permissions(fs::Permissions::from_mode(0o755))
                .map_err(|source| {
                    io_error(
                        "set staged tool executable permissions",
                        destination,
                        source,
                    )
                })?;
            Ok(())
        },
        |stage, &()| {
            let actual = sha256_file(stage)?;
            if actual != expected {
                return Err(InstallError::ExecutableHashMismatch {
                    path: stage.to_path_buf(),
                    actual,
                    expected,
                }
                .into());
            }
            admit(stage)
        },
    )
    .map_err(|error| match error {
        NamedWriteError::Producer {
            source,
            cleanup_error: None,
        } => source,
        error => {
            let kind = match &error {
                NamedWriteError::Producer { .. } => io::ErrorKind::Other,
                NamedWriteError::BeforePublication { source, .. }
                | NamedWriteError::AfterPublication { source } => source.kind(),
            };
            io_error(
                "publish admitted tool executable",
                destination,
                io::Error::new(kind, error),
            )
            .into()
        }
    })
}

fn stage_executable(candidate: &Path, stage: &Path) -> Result<(), InstallError> {
    let mut source = File::open(candidate)
        .map_err(|source| io_error("open admitted tool executable", candidate, source))?;
    let mut output = OpenOptions::new()
        .create_new(true)
        .write(true)
        .open(stage)
        .map_err(|source| io_error("create staged tool executable", stage, source))?;
    io::copy(&mut source, &mut output)
        .map_err(|source| io_error("write staged tool executable", stage, source))?;
    #[cfg(unix)]
    fs::set_permissions(stage, fs::Permissions::from_mode(0o755))
        .map_err(|source| io_error("set staged tool executable permissions", stage, source))?;
    output
        .sync_all()
        .map_err(|source| io_error("sync staged tool executable", stage, source))?;
    Ok(())
}

/// Qualify a complete immutable bin/lib layout, then atomically select it.
/// Failed admission leaves the previous installation intact. Published bundles
/// remain available to processes holding their canonical executable paths.
#[cfg(unix)]
fn publish_bundle<E: From<InstallError>>(
    tool: &str,
    candidate: &Path,
    library: &Path,
    destination: &Path,
    admit: impl Fn(&Path) -> Result<(), E>,
) -> Result<(), E> {
    let parent = destination.parent().ok_or_else(|| {
        io_error(
            "select tool installation directory",
            destination,
            io::Error::new(io::ErrorKind::InvalidInput, "destination has no parent"),
        )
    })?;
    fs::create_dir_all(parent)
        .map_err(|source| io_error("create tool installation directory", parent, source))?;
    let parent = fs::canonicalize(parent)
        .map_err(|source| io_error("resolve tool installation directory", parent, source))?;
    let bundle = parent.join(format!(
        ".ic-build-{tool}-bundle-{}-{}",
        std::process::id(),
        TEMP_SEQUENCE.fetch_add(1, Ordering::Relaxed)
    ));
    fs::create_dir(&bundle).map_err(|source| io_error("create tool bundle", &bundle, source))?;
    let link = bundle.join("selected");
    let mut published = false;
    let result = (|| {
        let bin = bundle.join("bin");
        let lib = bundle.join("lib");
        for directory in [&bin, &lib] {
            fs::create_dir(directory)
                .map_err(|source| io_error("create tool bundle directory", directory, source))?;
        }
        let executable = bin.join(tool);
        let library_name = library.file_name().ok_or_else(|| {
            io_error(
                "select library name",
                library,
                io::Error::from(io::ErrorKind::InvalidInput),
            )
        })?;
        let staged_library = lib.join(library_name);
        stage_bundle_member(candidate, &executable)?;
        stage_bundle_member(library, &staged_library)?;
        admit(&executable)?;
        for directory in [&bin, &lib, &bundle] {
            File::open(directory)
                .and_then(|file| file.sync_all())
                .map_err(|source| io_error("sync tool bundle directory", directory, source))?;
        }
        std::os::unix::fs::symlink(&executable, &link)
            .map_err(|source| io_error("stage tool bundle selection", &link, source))?;
        File::open(&bundle)
            .and_then(|file| file.sync_all())
            .map_err(|source| io_error("sync tool bundle selection", &bundle, source))?;
        fs::rename(&link, destination)
            .map_err(|source| io_error("publish tool bundle", destination, source))?;
        published = true;
        File::open(&parent)
            .and_then(|file| file.sync_all())
            .map_err(|source| {
                E::from(io_error(
                    "sync tool installation directory",
                    &parent,
                    source,
                ))
            })
    })();
    if !published {
        let _ = fs::remove_dir_all(&bundle);
    }
    result
}

fn stage_bundle_member(source: &Path, stage: &Path) -> Result<(), InstallError> {
    let metadata = fs::symlink_metadata(source)
        .map_err(|error| io_error("inspect tool bundle member", source, error))?;
    if !metadata.is_file() {
        return Err(io_error(
            "inspect tool bundle member",
            source,
            io::Error::new(
                io::ErrorKind::InvalidData,
                "bundle member must be a regular file",
            ),
        ));
    }
    let expected = sha256_file(source)?;
    stage_executable(source, stage)?;
    let actual = sha256_file(stage)?;
    if actual != expected {
        return Err(InstallError::ExecutableHashMismatch {
            path: stage.to_path_buf(),
            actual,
            expected,
        });
    }
    Ok(())
}

/// Stream a file hash without buffering its complete contents.
/// # Errors
/// I/O errors while opening or streaming the selected file.
pub fn sha256_file(path: &Path) -> Result<String, InstallError> {
    // Distribution owners retain their exact digest pins. Hashing streams in
    // constant space and accepts every representable file size.
    ic_host_fs::read::hash_file(path, u64::MAX)
        .map(|identity| identity.sha256.to_string())
        .map_err(|source| io_error("hash admitted tool file", path, source.into()))
}

fn io_error(operation: &'static str, path: &Path, source: io::Error) -> InstallError {
    InstallError::Io {
        operation,
        path: path.to_path_buf(),
        source,
    }
}

fn installation_scratch(root: &Path, tool: &str) -> Result<OwnedDirectory, InstallError> {
    for _ in 0..TEMP_ATTEMPTS {
        let path = root.join(format!(
            "ic-build-{tool}-install-{}-{}",
            std::process::id(),
            TEMP_SEQUENCE.fetch_add(1, Ordering::Relaxed)
        ));
        match fs::create_dir(&path) {
            Ok(()) => return Ok(OwnedDirectory::new(path)),
            Err(source) if source.kind() == io::ErrorKind::AlreadyExists => {}
            Err(source) => {
                return Err(io_error(
                    "create temporary tool installation directory",
                    &path,
                    source,
                ));
            }
        }
    }
    Err(InstallError::TempDirectoryExhausted {
        root: root.to_path_buf(),
    })
}

#[cfg(not(unix))]
fn publish_bundle<E: From<InstallError>>(
    _tool: &str,
    _candidate: &Path,
    _library: &Path,
    destination: &Path,
    _admit: impl Fn(&Path) -> Result<(), E>,
) -> Result<(), E> {
    Err(io_error(
        "publish tool bundle",
        destination,
        io::Error::from(io::ErrorKind::Unsupported),
    )
    .into())
}
