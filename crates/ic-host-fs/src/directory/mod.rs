//! Descriptor-relative admission; no application formats or implicit hierarchy creation.
#[cfg(test)]
mod tests;

mod preparation;

pub use preparation::PrepareFailure;
use std::{
    ffi::{OsStr, OsString},
    fs::{File, Metadata},
    io,
    os::unix::{ffi::OsStrExt, fs::MetadataExt},
    path::Path,
    sync::Arc,
};

use rustix::fs::{self, AtFlags, FileType, Mode, OFlags};
use thiserror::Error;

/// Original native cause or explicit filesystem admission refusal.
#[derive(Debug, Error)]
pub enum Error {
    /// Native I/O cause, preserved without string conversion.
    #[error("filesystem I/O refused: {0}")]
    Io(#[from] io::Error),
    /// A symlink or non-directory component was refused by no-follow admission.
    #[error("filesystem link/type refused: {0}")]
    Refused(#[source] io::Error),
    /// A child name was not one normal component.
    #[error("name must be one normal component")]
    Name,
    /// Observed permissions violated caller policy.
    #[error("mode {mode:o} contains forbidden permissions {forbidden:o}")]
    Permissions {
        /// Observed permission bits.
        mode: u32,
        /// Caller-forbidden bits.
        forbidden: u32,
    },
    /// Expected a regular file or directory.
    #[error("unexpected entry kind")]
    Kind,
    /// A single-link file policy was violated.
    #[error("file has multiple hard links")]
    Aliased,
    /// Complete enumeration exceeded its caller-supplied ceiling.
    #[error("directory entry limit exceeded")]
    EntryLimit,
    /// An expected file entry was replaced/disappeared, or retained content changed.
    #[error("file identity changed")]
    Changed,
    /// Byte admission refused; no completed content identity exists.
    #[error(transparent)]
    Bytes(#[from] ic_host_artifacts::Error),
}
impl From<rustix::io::Errno> for Error {
    fn from(error: rustix::io::Errno) -> Self {
        if matches!(error, rustix::io::Errno::LOOP | rustix::io::Errno::NOTDIR) {
            Self::Refused(error.into())
        } else {
            Self::Io(error.into())
        }
    }
}

/// Native object identity; keep its original descriptor alive when comparing it.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct FileId {
    device: u64,
    inode: u64,
}
impl FileId {
    /// Project identity from an already admitted metadata observation.
    #[must_use]
    pub fn from_metadata(metadata: &Metadata) -> Self {
        Self {
            device: metadata.dev(),
            inode: metadata.ino(),
        }
    }

    /// Observe identity through a held file.
    /// # Errors
    /// Preserves the original metadata I/O cause.
    pub fn from_file(file: &File) -> io::Result<Self> {
        Ok(Self::from_metadata(&file.metadata()?))
    }
}

/// Existing-file authority; update never creates or truncates a file.
#[derive(Clone, Copy, Debug)]
pub enum Access {
    /// Read-only access.
    Read,
    /// Read/write access for an explicitly retained recovery protocol.
    Update,
}
/// Admission policy for existing regular files.
#[derive(Clone, Copy, Debug)]
pub struct FilePolicy {
    /// Forbidden Unix permission bits.
    pub forbidden: u32,
    /// Require exactly one hard link.
    pub single_link: bool,
}
/// Observed entry kind; links are never interpreted as their targets.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum Kind {
    /// Regular file.
    File,
    /// Directory.
    Directory,
    /// Link or other native entry.
    Other,
}
/// One observed directory entry, not a held file admission.
#[derive(Debug)]
pub struct Entry {
    /// Single-component native filename.
    pub name: OsString,
    /// No-follow observed kind.
    pub kind: Kind,
}
/// Held directory identity. Clones share custody without reopening its pathname.
#[derive(Clone, Debug)]
pub struct Directory {
    pub(crate) file: Arc<File>,
}

impl Directory {
    /// Open an existing root, refusing a final symlink and forbidden permissions.
    /// Ancestors are resolved once by the OS; later operations use this held identity.
    /// # Errors
    /// Refuses unsafe types/permissions and original I/O errors. Missing is `None`.
    pub fn open(path: &Path, forbidden: u32) -> Result<Option<Self>, Error> {
        let descriptor = optional(fs::open(path, directory_flags(), Mode::empty()))?;
        descriptor
            .map(|fd| Self::admit(File::from(fd), forbidden))
            .transpose()
    }

    fn admit(file: File, forbidden: u32) -> Result<Self, Error> {
        permissions(&file.metadata()?, forbidden)?;
        Ok(Self {
            file: Arc::new(file),
        })
    }

    /// Open one existing child directory, without creation or symlink traversal.
    /// # Errors
    /// Refuses names, types, permissions and original I/O. Missing is `None`.
    pub fn open_directory(&self, child: &OsStr, forbidden: u32) -> Result<Option<Self>, Error> {
        name(child)?;
        let descriptor = optional(fs::openat(
            &*self.file,
            child,
            directory_flags(),
            Mode::empty(),
        ))?;
        descriptor
            .map(|fd| Self::admit(File::from(fd), forbidden))
            .transpose()
    }

    /// Create one new child, sync its parent and admit it. Existing entries refuse.
    /// On later failure the created child is retained, never implicitly removed.
    /// # Errors
    /// Refuses invalid names/modes, occupied names, sync or admission failures.
    pub fn create_directory(
        &self,
        child: &OsStr,
        mode: u32,
        forbidden: u32,
    ) -> Result<Self, Error> {
        name(child)?;
        let permissions = valid_mode(mode)?;
        if mode & forbidden != 0 {
            return Err(Error::Permissions { mode, forbidden });
        }
        fs::mkdirat(&*self.file, child, permissions)?;
        self.sync()?;
        self.open_directory(child, forbidden)?.ok_or(Error::Changed)
    }

    /// Admit an existing regular file under explicit access and permission policy.
    /// # Errors
    /// Refuses links, special files, permission/alias violations and original I/O.
    pub fn open_file(
        &self,
        child: &OsStr,
        access: Access,
        policy: FilePolicy,
    ) -> Result<Option<File>, Error> {
        name(child)?;
        let flags = match access {
            Access::Read => OFlags::RDONLY,
            Access::Update => OFlags::RDWR,
        } | OFlags::NOFOLLOW
            | OFlags::CLOEXEC
            | OFlags::NONBLOCK;
        let Some(fd) = optional(fs::openat(&*self.file, child, flags, Mode::empty()))? else {
            return Ok(None);
        };
        admit_file(File::from(fd), policy).map(Some)
    }

    /// Create a retained recovery file, never replace it. No automatic deletion.
    /// The caller owns ordered writes, sync and recovery of partial effects. This
    /// writable handle is deliberately separate from publication staging.
    /// # Errors
    /// Refuses invalid names/modes, occupied names and native creation failures.
    pub fn create_file(&self, child: &OsStr, mode: u32) -> Result<File, Error> {
        name(child)?;
        let permissions = valid_mode(mode)?;
        Ok(File::from(fs::openat(
            &*self.file,
            child,
            OFlags::RDWR | OFlags::CREATE | OFlags::EXCL | OFlags::NOFOLLOW | OFlags::CLOEXEC,
            permissions,
        )?))
    }

    /// Remove one entry only after comparing it with the caller's held regular file.
    /// The parent is not synchronized; call [`Self::sync`] when durability is required.
    /// Callers exclude concurrent namespace mutation: comparison and unlink are
    /// separate syscalls, not an atomic fence against hostile writers.
    /// # Errors
    /// Refuses missing/replaced entries, links, special files and original I/O causes.
    pub fn remove_file(&self, child: &OsStr, expected: &File) -> Result<(), Error> {
        self.check_file(child, expected)?;
        fs::unlinkat(&*self.file, child, AtFlags::empty())?;
        Ok(())
    }

    pub(crate) fn check_file(&self, child: &OsStr, expected: &File) -> Result<(), Error> {
        let named = self
            .open_file(
                child,
                Access::Read,
                FilePolicy {
                    forbidden: 0,
                    single_link: false,
                },
            )?
            .ok_or(Error::Changed)?;
        if FileId::from_file(&named)? != FileId::from_file(expected)? {
            return Err(Error::Changed);
        }
        Ok(())
    }

    /// Enumerate a complete bounded set; metadata checks do not follow symlinks.
    /// # Errors
    /// Refuses the whole scan on overflow, allocation, enumeration or metadata error.
    pub fn entries(&self, limit: usize) -> Result<Vec<Entry>, Error> {
        let mut entries = Vec::new();
        for entry in fs::Dir::read_from(&*self.file)? {
            let entry = entry?;
            let bytes = entry.file_name().to_bytes();
            if bytes == b"." || bytes == b".." {
                continue;
            }
            if entries.len() == limit {
                return Err(Error::EntryLimit);
            }
            let name = OsStr::from_bytes(bytes).to_owned();
            let stat = fs::statat(&*self.file, &name, AtFlags::SYMLINK_NOFOLLOW)?;
            let kind = match FileType::from_raw_mode(stat.st_mode) {
                FileType::RegularFile => Kind::File,
                FileType::Directory => Kind::Directory,
                _ => Kind::Other,
            };
            entries
                .try_reserve(1)
                .map_err(|_| Error::Io(io::Error::other("directory allocation refused")))?;
            entries.push(Entry { name, kind });
        }
        Ok(entries)
    }

    /// Observe the held directory identity without reopening a mutable path.
    /// # Errors
    /// Preserves original descriptor metadata failure.
    pub fn identity(&self) -> io::Result<FileId> {
        FileId::from_file(&self.file)
    }

    /// Sync this held directory; success is the platform filesystem's guarantee.
    /// # Errors
    /// Preserves the original synchronization cause.
    pub fn sync(&self) -> io::Result<()> {
        self.file.sync_all()
    }
}

pub(crate) fn name(value: &OsStr) -> Result<(), Error> {
    let bytes = value.as_bytes();
    if bytes.is_empty()
        || matches!(bytes, b"." | b"..")
        || bytes.contains(&b'/')
        || bytes.contains(&0)
    {
        Err(Error::Name)
    } else {
        Ok(())
    }
}

pub(crate) fn valid_mode(mode: u32) -> Result<Mode, Error> {
    if mode & !0o777 != 0 {
        Err(Error::Permissions {
            mode,
            forbidden: !0o777,
        })
    } else {
        // The admitted nine permission bits fit both macOS and Linux mode_t.
        Ok(Mode::from_bits_truncate(mode as _))
    }
}

pub(crate) fn admit_file(file: File, policy: FilePolicy) -> Result<File, Error> {
    let metadata = file.metadata()?;
    if !metadata.is_file() {
        return Err(Error::Kind);
    }
    permissions(&metadata, policy.forbidden)?;
    if policy.single_link && metadata.nlink() != 1 {
        return Err(Error::Aliased);
    }
    Ok(file)
}

fn directory_flags() -> OFlags {
    OFlags::RDONLY | OFlags::DIRECTORY | OFlags::NOFOLLOW | OFlags::CLOEXEC
}

fn permissions(metadata: &Metadata, forbidden: u32) -> Result<(), Error> {
    let mode = metadata.mode() & 0o7777;
    if mode & forbidden != 0 {
        Err(Error::Permissions { mode, forbidden })
    } else {
        Ok(())
    }
}

fn optional<T>(result: Result<T, rustix::io::Errno>) -> Result<Option<T>, Error> {
    match result {
        Ok(value) => Ok(Some(value)),
        Err(rustix::io::Errno::NOENT) => Ok(None),
        Err(error) => Err(error.into()),
    }
}
