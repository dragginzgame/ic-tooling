//! One advisory lock owner; recovery-file content remains caller-owned.
use std::fs::File;
use std::io;

use rustix::fs::{FlockOperation, flock};

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
