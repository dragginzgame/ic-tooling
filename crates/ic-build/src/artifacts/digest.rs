use ic_host_artifacts::artifact::{Sha256Digest, visit_reader};
use sha2::{Digest, Sha256};
use std::{
    borrow::Cow,
    collections::{BTreeMap, BTreeSet},
    ffi::{OsStr, OsString},
    fs::{self, File},
    io,
    path::{Path, PathBuf},
};

#[cfg(unix)]
use std::os::unix::{ffi::OsStrExt as _, fs::MetadataExt as _};
#[cfg(windows)]
use std::os::windows::ffi::OsStrExt as _;

#[derive(Debug, thiserror::Error)]
#[error("failed to atomically copy {} to {}: {source}", source_path.display(), destination_path.display())]
struct AtomicCopyErrorContext {
    source_path: PathBuf,
    destination_path: PathBuf,
    source: io::Error,
}

/// SHA-256 digest of one deterministic artifact-input set.
#[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct InputDigest([u8; 32]);

impl InputDigest {
    pub(super) const fn from_bytes(bytes: [u8; 32]) -> Self {
        Self(bytes)
    }
    /// Borrow the raw SHA-256 bytes.
    #[must_use]
    pub const fn as_bytes(&self) -> &[u8; 32] {
        &self.0
    }

    /// Render the digest as lowercase hexadecimal.
    #[must_use]
    pub fn to_hex(self) -> String {
        self.to_string()
    }
}

impl std::fmt::Display for InputDigest {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        Sha256Digest::from_bytes(self.0).fmt(formatter)
    }
}

pub(super) struct InputHasher {
    state: Sha256,
    read_buffer: Vec<u8>,
}

impl InputHasher {
    pub(super) fn new(domain: &str) -> Self {
        let mut hasher = Self {
            state: Sha256::new(),
            read_buffer: Vec::new(),
        };
        hasher.field("domain", domain.as_bytes());
        hasher
    }

    pub(super) fn field(&mut self, label: &str, value: &[u8]) {
        self.field_header(
            label,
            u64::try_from(value.len()).expect("input value length must fit in u64"),
        );
        self.state.update(value);
    }

    fn field_header(&mut self, label: &str, value_len: u64) {
        self.state.update(
            u64::try_from(label.len())
                .expect("input label length must fit in u64")
                .to_le_bytes(),
        );
        self.state.update(label.as_bytes());
        self.state.update(value_len.to_le_bytes());
    }

    fn file_field(&mut self, label: &str, path: &Path) -> io::Result<u64> {
        let mut file = File::open(path)?;
        let expected_len = file.metadata()?.len();
        self.field_header(label, expected_len);

        let mut actual_len = 0_u64;
        // Small sources need only their declared length; large artifacts use bounded reads.
        // Even empty files need a nonempty read buffer to detect growth.
        let buffer_len = usize::try_from(expected_len.clamp(1, 64 * 1024))
            .expect("bounded artifact buffer length must fit in usize");
        // A tree hashes many files with one hasher. Retain its bounded scratch
        // space, reading only this file's window and hashing only returned bytes.
        if self.read_buffer.len() < buffer_len {
            // Geometric growth near the read limit would retain almost twice
            // the scratch space required by any file in this hashing pass.
            self.read_buffer
                .reserve_exact(buffer_len - self.read_buffer.len());
            self.read_buffer.resize(buffer_len, 0);
        }
        let changed_size = |actual_len| {
            io::Error::new(
                io::ErrorKind::InvalidData,
                format!(
                    "file changed size while hashing: expected {expected_len} bytes, read {actual_len}"
                ),
            )
        };
        visit_reader::<io::Error>(
            &mut file,
            &mut self.read_buffer[..buffer_len],
            u64::MAX,
            |chunk| {
                actual_len = actual_len.saturating_add(
                    u64::try_from(chunk.len()).expect("artifact read length must fit in u64"),
                );
                if actual_len > expected_len {
                    return Err(changed_size(actual_len));
                }
                self.state.update(chunk);
                Ok(())
            },
        )?;
        if actual_len != expected_len {
            return Err(changed_size(actual_len));
        }
        Ok(actual_len)
    }

    pub(super) fn finish(self) -> InputDigest {
        InputDigest(self.state.finalize().into())
    }
}

pub(super) fn digest_bytes(domain: &str, value: &[u8]) -> InputDigest {
    let mut hasher = InputHasher::new(domain);
    hasher.field("content", value);
    hasher.finish()
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub(super) struct FileDigest {
    pub(super) bytes: u64,
    pub(super) digest: InputDigest,
}

pub(super) fn digest_file(domain: &str, path: &Path) -> io::Result<FileDigest> {
    let mut hasher = InputHasher::new(domain);
    let bytes = hasher.file_field("content", path)?;
    Ok(FileDigest {
        bytes,
        digest: hasher.finish(),
    })
}

/// Read a UTF-8 stamp without allocating or reading an oversized sidecar in full.
/// An oversized stamp is stale; other read and decoding errors reach the caller.
pub(super) fn read_stamp_with_limit(path: &Path, maximum_len: usize) -> io::Result<Option<String>> {
    read_file_with_limit(path, maximum_len)?
        .map(|contents| {
            String::from_utf8(contents)
                .map_err(|error| io::Error::new(io::ErrorKind::InvalidData, error))
        })
        .transpose()
}

/// Read at most the format's maximum length plus one byte to detect oversized files.
pub(super) fn read_file_with_limit(path: &Path, maximum_len: usize) -> io::Result<Option<Vec<u8>>> {
    use ic_host_artifacts::artifact::ArtifactError;
    use ic_host_fs::read::read_file;

    // The shared library owns bounded reading and allocation. Cache policy owns
    // the meaning of overflow: an oversized stamp or manifest is a cache miss.
    match read_file(path, maximum_len) {
        Ok(contents) => Ok(Some(contents)),
        Err(ArtifactError::LimitExceeded { .. }) => Ok(None),
        Err(ArtifactError::NotRegularFile) => Err(io::Error::new(
            io::ErrorKind::InvalidData,
            ArtifactError::NotRegularFile,
        )),
        Err(error) => Err(error.into()),
    }
}

/// Only reuse an independent, caller-owned writable destination. The caller
/// coordinates other writers and supplies a digest from a verified cache entry.
fn destination_matches_digest(domain: &str, destination: &Path, expected: &FileDigest) -> bool {
    destination_is_reusable(destination, expected.bytes)
        && digest_file(domain, destination).is_ok_and(|actual| actual == *expected)
}

/// Cold publication always copies; only previously verified outputs may preserve
/// matching caller-owned destinations and their metadata.
pub(super) fn materialize_verified_output(
    domain: &str,
    source: &Path,
    destination: &Path,
    verified: Option<&FileDigest>,
) -> io::Result<()> {
    if verified.is_some_and(|info| destination_matches_digest(domain, destination, info)) {
        return Ok(());
    }
    copy_file_atomic(source, destination).map(|_| ())
}

pub(super) fn destination_matches_bytes(destination: &Path, expected: &[u8]) -> bool {
    destination_is_reusable(
        destination,
        u64::try_from(expected.len()).expect("artifact byte length must fit in u64"),
    ) && read_file_with_limit(destination, expected.len())
        .is_ok_and(|actual| actual.as_deref() == Some(expected))
}

fn destination_is_reusable(destination: &Path, expected_bytes: u64) -> bool {
    #[cfg(unix)]
    {
        let Ok(metadata) = fs::symlink_metadata(destination) else {
            return false;
        };
        let effective_uid = rustix::process::geteuid().as_raw();
        // Detach links and normalize foreign-owned, restricted or executable
        // files, even when their bytes match a retained artifact.
        if !metadata.file_type().is_file()
            || metadata.nlink() != 1
            || metadata.uid() != effective_uid
            || metadata.mode() & 0o600 != 0o600
            || metadata.mode() & 0o7111 != 0
            || metadata.len() != expected_bytes
        {
            return false;
        }
        true
    }
    #[cfg(not(unix))]
    {
        // Preserve replacement where a portable single-link check is unavailable.
        let _ = (destination, expected_bytes);
        false
    }
}

// A cache root may not have been created yet. Other failures must not silently
// remove a declared exclusion and change which inputs contribute to identity.
fn resolve_excluded_roots(paths: &[PathBuf]) -> io::Result<Vec<PathBuf>> {
    paths
        .iter()
        .filter_map(|path| match path.canonicalize() {
            Ok(path) => Some(Ok(path)),
            Err(error) if error.kind() == io::ErrorKind::NotFound => None,
            Err(error) => Some(Err(io::Error::new(
                error.kind(),
                format!(
                    "failed to resolve excluded root {}: {error}",
                    path.display()
                ),
            ))),
        })
        .collect()
}

fn ordered_labeled_paths<L: AsRef<Path>, P>(
    paths: impl IntoIterator<Item = (L, P)>,
) -> Vec<(L, P)> {
    let mut paths = paths.into_iter().collect::<Vec<_>>();
    paths.sort_by(|(left, _), (right, _)| {
        os_bytes(left.as_ref().as_os_str()).cmp(&os_bytes(right.as_ref().as_os_str()))
    });
    paths
}

pub(super) fn digest_labeled_paths<L: AsRef<Path>, P: AsRef<Path>>(
    domain: &str,
    paths: impl IntoIterator<Item = (L, P)>,
    excluded_roots: &[PathBuf],
) -> io::Result<InputDigest> {
    let paths = ordered_labeled_paths(paths);

    let excluded_roots = resolve_excluded_roots(excluded_roots)?;
    let mut visited_directories = BTreeSet::new();
    let mut hasher = InputHasher::new(domain);
    for (label, path) in paths {
        hash_path(
            &mut hasher,
            label.as_ref(),
            path.as_ref(),
            &excluded_roots,
            &mut visited_directories,
            true,
            None,
        )?;
    }
    Ok(hasher.finish())
}

#[derive(Default)]
pub(super) struct LabeledPathDigestCache {
    entries: BTreeMap<PathBuf, Vec<LabeledPathDigestCacheEntry>>,
}

struct LabeledPathDigestCacheEntry {
    domain: String,
    label: PathBuf,
    canonical_root: PathBuf,
    excluded_roots: Vec<PathBuf>,
    traversed_external_path: bool,
    digest: InputDigest,
}

struct HashPathTrace {
    canonical_root: PathBuf,
    traversed_external_path: bool,
}

pub(super) fn digest_labeled_paths_composable<'a>(
    domain: &str,
    paths: impl IntoIterator<Item = (&'a Path, &'a Path)>,
    excluded_roots: &[PathBuf],
    cache: &mut LabeledPathDigestCache,
) -> io::Result<InputDigest> {
    let paths = ordered_labeled_paths(paths);
    let excluded_roots = resolve_excluded_roots(excluded_roots)?;
    let mut hasher = InputHasher::new(&format!("{domain}/composable-v1"));
    for (label, path) in paths {
        let digest = cache.digest_root(domain, label, path, &excluded_roots)?;
        hasher.field("input-label", &os_bytes(label.as_os_str()));
        hasher.field("input-digest", digest.as_bytes());
    }
    Ok(hasher.finish())
}

impl LabeledPathDigestCache {
    fn digest_root(
        &mut self,
        domain: &str,
        label: &Path,
        path: &Path,
        excluded_roots: &[PathBuf],
    ) -> io::Result<InputDigest> {
        let canonical_root = path.canonicalize()?;
        if let Some(entry) = self.entries.get(path).and_then(|entries| {
            entries.iter().find(|entry| {
                entry.domain == domain
                    && entry.label == label
                    && entry.excluded_roots.iter().eq(effective_root_exclusions(
                        &entry.canonical_root,
                        excluded_roots,
                        entry.traversed_external_path,
                    ))
            })
        }) {
            return Ok(entry.digest);
        }
        let mut hasher = InputHasher::new(&format!("{domain}/root-v1"));
        let mut trace = HashPathTrace {
            canonical_root: canonical_root.clone(),
            traversed_external_path: false,
        };
        hash_path(
            &mut hasher,
            label,
            path,
            excluded_roots,
            &mut BTreeSet::new(),
            true,
            Some(&mut trace),
        )?;
        let digest = hasher.finish();
        self.entries
            .entry(path.to_owned())
            .or_default()
            .push(LabeledPathDigestCacheEntry {
                domain: domain.to_owned(),
                label: label.to_owned(),
                canonical_root,
                excluded_roots: effective_root_exclusions(
                    &trace.canonical_root,
                    excluded_roots,
                    trace.traversed_external_path,
                )
                .cloned()
                .collect(),
                traversed_external_path: trace.traversed_external_path,
                digest,
            });
        Ok(digest)
    }
}

fn effective_root_exclusions<'a>(
    canonical_root: &'a Path,
    excluded_roots: &'a [PathBuf],
    traversed_external_path: bool,
) -> impl Iterator<Item = &'a PathBuf> {
    excluded_roots.iter().filter(move |excluded| {
        traversed_external_path
            || excluded.starts_with(canonical_root)
            || canonical_root.starts_with(excluded)
    })
}

fn hash_path(
    hasher: &mut InputHasher,
    label: &Path,
    path: &Path,
    excluded_roots: &[PathBuf],
    visited_directories: &mut BTreeSet<PathBuf>,
    declared_root: bool,
    trace: Option<&mut HashPathTrace>,
) -> io::Result<()> {
    let mut traversal = InputTreeWalk {
        excluded_roots,
        excluded_names: &[],
        visited_directories,
        trace,
        reject_links: false,
    };
    traversal.visit(label, path, declared_root, &mut |entry| {
        match entry {
            InputTreeEntry::File { label, path } => {
                hasher.field("file-path", &os_bytes(label.as_os_str()));
                hasher.file_field("file-content", path)?;
            }
            InputTreeEntry::Directory {
                label, repeated, ..
            } => {
                hasher.field("directory", &os_bytes(label.as_os_str()));
                if repeated {
                    hasher.field("directory-already-visited", &os_bytes(label.as_os_str()));
                }
            }
            InputTreeEntry::Missing { path } => {
                return Err(io::Error::new(
                    io::ErrorKind::NotFound,
                    path.display().to_string(),
                ));
            }
        }
        Ok(())
    })
}

/// One filesystem observation emitted by the shared input traversal.
pub(super) enum InputTreeEntry<'a> {
    File {
        label: &'a Path,
        path: &'a Path,
    },
    Directory {
        label: &'a Path,
        path: &'a Path,
        repeated: bool,
    },
    Missing {
        path: &'a Path,
    },
}

/// Path context shared by fingerprint and standalone-evidence failures.
#[derive(Debug, thiserror::Error)]
#[error("{}: {source}", .path.display())]
pub(super) struct InputPathError {
    pub(super) path: PathBuf,
    pub(super) source: io::Error,
}

/// One traversal owner; cache fingerprints retain their original follow-link policy.
pub(super) struct InputTreeWalk<'a> {
    excluded_roots: &'a [PathBuf],
    excluded_names: &'a [OsString],
    visited_directories: &'a mut BTreeSet<PathBuf>,
    trace: Option<&'a mut HashPathTrace>,
    reject_links: bool,
}

impl<'a> InputTreeWalk<'a> {
    pub(super) const fn evidence(
        excluded_roots: &'a [PathBuf],
        excluded_names: &'a [OsString],
        visited_directories: &'a mut BTreeSet<PathBuf>,
    ) -> Self {
        Self {
            excluded_roots,
            excluded_names,
            visited_directories,
            trace: None,
            reject_links: true,
        }
    }

    pub(super) fn visit(
        &mut self,
        label: &Path,
        path: &Path,
        declared_root: bool,
        visitor: &mut impl FnMut(InputTreeEntry<'_>) -> io::Result<()>,
    ) -> io::Result<()> {
        let context = |source: io::Error| {
            io::Error::new(
                source.kind(),
                InputPathError {
                    path: path.to_owned(),
                    source,
                },
            )
        };
        // Standalone evidence records absence and refuses links. Existing cache
        // hashing still canonicalizes first and follows links exactly as before.
        let inspected = if self.reject_links {
            match fs::symlink_metadata(path) {
                Ok(metadata) if metadata.file_type().is_symlink() => {
                    return Err(context(io::Error::new(
                        io::ErrorKind::InvalidInput,
                        "input is a symbolic link",
                    )));
                }
                Ok(metadata) => Some(metadata),
                Err(error) if error.kind() == io::ErrorKind::NotFound => {
                    let resolved =
                        super::cache_fs::canonicalize_allow_missing(path).map_err(context)?;
                    if self
                        .excluded_roots
                        .iter()
                        .any(|root| resolved.starts_with(root))
                    {
                        if declared_root {
                            return Err(context(io::Error::new(
                                io::ErrorKind::InvalidInput,
                                "declared missing input is inside an excluded root",
                            )));
                        }
                        return Ok(());
                    }
                    return visitor(InputTreeEntry::Missing { path }).map_err(context);
                }
                Err(error) => return Err(context(error)),
            }
        } else {
            None
        };
        let canonical = path.canonicalize().map_err(context)?;
        if let Some(trace) = &mut self.trace
            && !canonical.starts_with(&trace.canonical_root)
        {
            trace.traversed_external_path = true;
        }
        if self
            .excluded_roots
            .iter()
            .any(|root| canonical.starts_with(root))
        {
            if declared_root {
                return Err(context(io::Error::new(
                    io::ErrorKind::InvalidInput,
                    "declared input is inside an excluded root",
                )));
            }
            return Ok(());
        }
        let metadata = match inspected {
            Some(metadata) => metadata,
            None => fs::metadata(path).map_err(context)?,
        };
        if metadata.is_file() {
            return visitor(InputTreeEntry::File { label, path }).map_err(context);
        }
        if !metadata.is_dir() {
            return Err(context(io::Error::new(
                io::ErrorKind::InvalidInput,
                "input is not a regular file or directory",
            )));
        }
        // Cache fingerprints preserve cycle/alias markers. Standalone evidence
        // refuses child links, so each declared spelling can be fully scanned;
        // an alias must not be recorded as an empty, complete directory.
        let repeated = !self.visited_directories.insert(canonical) && !self.reject_links;
        visitor(InputTreeEntry::Directory {
            label,
            path,
            repeated,
        })
        .map_err(context)?;
        if repeated {
            return Ok(());
        }
        let mut entries = fs::read_dir(path)
            .map_err(context)?
            .map(|entry| entry.map(|entry| entry.file_name()))
            .collect::<Result<Vec<_>, _>>()
            .map_err(context)?;
        #[cfg(unix)]
        entries.sort_unstable_by(|left, right| os_bytes(left).cmp(&os_bytes(right)));
        #[cfg(not(unix))]
        entries.sort_by_cached_key(|name| os_bytes(name).into_owned());
        for name in entries {
            if self.excluded_names.contains(&name) {
                continue;
            }
            self.visit(&label.join(&name), &path.join(&name), false, visitor)?;
        }
        Ok(())
    }
}

pub(super) fn copy_file_atomic(source: &Path, destination: &Path) -> io::Result<u64> {
    let result = (|| {
        let mut source_file = File::open(source)?;
        ic_host_fs::durable::copy_from(
            &mut source_file,
            destination,
            ic_host_fs::durable::WriteOptions {
                mode: ic_host_fs::durable::PublicationMode::Replace,
                permissions: 0o666,
            },
            None,
        )
        .map_err(io::Error::other)
    })();
    result.map_err(|source_error| {
        io::Error::new(
            source_error.kind(),
            AtomicCopyErrorContext {
                source_path: source.to_owned(),
                destination_path: destination.to_owned(),
                source: source_error,
            },
        )
    })
}

#[cfg(unix)]
pub(super) fn os_bytes(value: &OsStr) -> Cow<'_, [u8]> {
    Cow::Borrowed(value.as_bytes())
}

#[cfg(windows)]
pub(super) fn os_bytes(value: &OsStr) -> Cow<'_, [u8]> {
    Cow::Owned(value.encode_wide().flat_map(u16::to_le_bytes).collect())
}

#[cfg(not(any(unix, windows)))]
pub(super) fn os_bytes(value: &OsStr) -> Cow<'_, [u8]> {
    Cow::Owned(value.to_string_lossy().as_bytes().to_vec())
}

#[cfg(test)]
mod tests;
