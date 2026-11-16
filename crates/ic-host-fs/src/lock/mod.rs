//! One advisory lock owner; recovery-file content remains caller-owned.
use std::ffi::OsStr;
use std::fs::File;
use std::io;

use crate::{
    Directory, Error, FilePolicy,
    directory::{admit_file, name, valid_mode},
};
use rustix::fs::{AtFlags, FileType, FlockOperation, Mode, OFlags, flock, openat, statat};

impl Directory {
    /// Open or create a regular sidecar and acquire its exclusive nonblocking lock.
    /// Existing bytes, inode and permissions remain unchanged. New modes are
    /// reduced by umask. Names/mode policy refuse before effects; no parents are
    /// created. There is no synchronization, cleanup, retry or truncation.
    /// The caller excludes namespace writers; advisory locking does not prevent
    /// replacement by noncooperating processes. A new sidecar may remain on failure.
    /// # Errors
    /// Preserves native contention as `Error::Io` with `WouldBlock`, other I/O,
    /// and regular-file/permission/link admission refusals.
    pub fn lock(&self, child: &OsStr, mode: u32, policy: FilePolicy) -> Result<Lock, Error> {
        name(child)?;
        valid_mode(mode)?;
        if mode & policy.forbidden != 0 {
            return Err(Error::Permissions {
                mode,
                forbidden: policy.forbidden,
            });
        }
        // Do not open an observed device or FIFO just to learn its type.
        // Descriptor admission below still checks the entry selected by open.
        match statat(&*self.file, child, AtFlags::SYMLINK_NOFOLLOW) {
            Ok(metadata) if FileType::from_raw_mode(metadata.st_mode) == FileType::RegularFile => {}
            Ok(_) => return Err(Error::Kind),
            Err(rustix::io::Errno::NOENT) => {}
            Err(cause) => return Err(cause.into()),
        }
        let file = File::from(openat(
            &*self.file,
            child,
            OFlags::RDWR | OFlags::CREATE | OFlags::NOFOLLOW | OFlags::NONBLOCK | OFlags::CLOEXEC,
            Mode::from_bits_truncate(mode),
        )?);
        let file = admit_file(file, policy)?;
        Lock::acquire(file).map_err(Error::Io)
    }
}

/// Nonblocking exclusive advisory lock; explicit release reports refusal.
#[derive(Debug)]
#[must_use = "retain lock ownership or explicitly release it"]
pub struct Lock {
    file: File,
    active: bool,
}
impl Lock {
    /// Lock the held file; does not create or replace its name.
    /// # Errors
    /// Returns the original lock failure (including WouldBlock for contention).
    pub fn acquire(file: File) -> io::Result<Self> {
        flock(&file, FlockOperation::NonBlockingLockExclusive)?;
        Ok(Self { file, active: true })
    }
    /// Original file for identity checks. A cloned descriptor does not become a
    /// separate lock owner: releasing this owner explicitly unlocks its description.
    #[must_use]
    pub fn file(&self) -> &File {
        &self.file
    }
    /// End the advisory lock, preserving the file and reporting unlock failure.
    /// # Errors
    /// Returns the original native unlock refusal.
    pub fn release(mut self) -> io::Result<()> {
        let result = flock(&self.file, FlockOperation::Unlock).map_err(io::Error::from);
        self.active = false;
        result
    }
}
impl Drop for Lock {
    fn drop(&mut self) {
        if self.active {
            let _ = flock(&self.file, FlockOperation::Unlock);
        }
    }
}

#[cfg(test)]
mod tests;
