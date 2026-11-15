//! Opt-in filesystem fixtures with explicit allocation and panic-retaining cleanup.

use std::{
    io,
    path::{Path, PathBuf},
};

/// A fresh test directory removed on normal Drop and retained during panic.
///
/// Uses [`create_fixture_directory`] for allocation and
/// [`crate::owned_directory::OwnedDirectory`] for cleanup custody. Callers select
/// the parent/prefix and must exclude namespace replacement. Cleanup is best
/// effort; a failed cleanup leaves evidence at the original path. Panicking tests
/// retain the directory and print its path, without changing production cleanup.
#[derive(Debug)]
pub struct FixtureDirectory {
    directory: crate::owned_directory::OwnedDirectory,
}

impl FixtureDirectory {
    /// Allocate one fresh directory and take cleanup responsibility for it.
    ///
    /// # Errors
    /// Returns [`create_fixture_directory`]'s validation and allocation failures.
    pub fn new(parent: &Path, prefix: &str) -> io::Result<Self> {
        Ok(Self {
            directory: crate::owned_directory::OwnedDirectory::new(create_fixture_directory(
                parent, prefix,
            )?),
        })
    }

    /// Borrow the physical path allocated for this fixture.
    #[must_use]
    pub fn path(&self) -> &Path {
        self.directory.path()
    }
}

impl Drop for FixtureDirectory {
    fn drop(&mut self) {
        if std::thread::panicking() {
            self.directory.disarm();
            eprintln!(
                "Failed test fixture retained: {}",
                self.directory.path().display()
            );
        }
    }
}

/// Create one fresh fixture directory beneath the caller-selected existing parent.
///
/// The prefix must be one normal filename component without CR/LF. Parent aliases
/// are resolved before creation. Existing entries are skipped, never reclaimed.
/// Unix directories use mode 0700 subject to umask; other hosts use native defaults.
/// The path is anchored to the physical parent and has no automatic cleanup.
/// The caller owns retention,
/// removal and custody against concurrent renames or replacement of the namespace.
/// This is a fixture primitive, not hostile-directory confinement.
///
/// # Errors
/// Returns invalid-prefix, parent resolution and directory creation errors.
pub fn create_fixture_directory(parent: &Path, prefix: &str) -> io::Result<PathBuf> {
    // Preserve invalid-prefix admission before any parent lookup.
    crate::directory::validate_options(prefix, 0o700)?;
    let parent = parent.canonicalize()?;
    crate::directory::create_unique_directory(&parent, prefix, 0o700, None)
}

/// Create a real FIFO without opening either endpoint; callers select its mode.
///
/// Uses the native POSIX utility on Unix, including macOS. The path must be
/// absent and have a caller-owned parent; no cleanup or byte IO is performed.
/// # Errors
/// Returns utility spawn or nonzero-exit failures.
#[cfg(unix)]
pub fn create_fifo(path: &Path, permissions: Option<u32>) -> io::Result<()> {
    // rustix's mkfifoat is unavailable on Apple hosts. Use the native POSIX
    // utility consistently so every special-file fixture exercises a real FIFO.
    let mut command = std::process::Command::new("/usr/bin/mkfifo");
    command.env_clear();
    if let Some(permissions) = permissions {
        command.args(["-m", &format!("{permissions:o}")]);
    }
    let status = command.arg(path).status()?;
    if status.success() {
        Ok(())
    } else {
        Err(io::Error::other(format!(
            "FIFO fixture creation failed: {status}"
        )))
    }
}

#[cfg(test)]
mod tests;
