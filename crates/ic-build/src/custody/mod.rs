//! Private target custody and descriptor-relative capture admission.

#[cfg(test)]
mod tests;

use std::{
    fs::File,
    io,
    os::unix::fs::PermissionsExt,
    path::{Component, Path, PathBuf},
};

use ic_host_fs::{Access, Directory, FilePolicy};
use tempfile::TempDir;

///
/// Target
/// Holds the private build directory until artifact capture and cleanup finish.
///

pub(crate) struct Target {
    directory: Directory,
    scratch: TempDir,
}

///
/// CreationFailure
/// Retains the creation error and any failure to clean up the private directory.
///

#[derive(Debug)]
pub(crate) struct CreationFailure {
    pub(crate) cause: io::Error,
    pub(crate) cleanup: Option<(PathBuf, io::Error)>,
}

impl Target {
    pub(crate) fn create(parent: &Path) -> Result<Self, CreationFailure> {
        let scratch = tempfile::Builder::new()
            .prefix("owned-build-")
            .permissions(std::fs::Permissions::from_mode(0o700))
            .tempdir_in(parent)
            .map_err(|cause| CreationFailure {
                cause,
                cleanup: None,
            })?;
        let admitted = Directory::open(scratch.path(), 0o077)
            .map_err(io_error)
            .and_then(|directory| directory.ok_or_else(|| io::ErrorKind::NotFound.into()));
        let directory = match admitted {
            Ok(directory) => directory,
            Err(cause) => {
                let path = scratch.path().to_owned();
                let cleanup = scratch.close().err().map(|error| (path, error));

                return Err(CreationFailure { cause, cleanup });
            }
        };

        Ok(Self { directory, scratch })
    }

    pub(crate) fn path(&self) -> &Path {
        self.scratch.path()
    }

    pub(crate) fn open_artifact(&self, path: &Path) -> io::Result<File> {
        let relative = path.strip_prefix(self.path()).map_err(|_| {
            io::Error::new(
                io::ErrorKind::InvalidInput,
                "artifact is outside private target",
            )
        })?;
        let mut parts = relative.components().peekable();
        let mut directory = self.directory.clone();
        while let Some(part) = parts.next() {
            let Component::Normal(name) = part else {
                return Err(io::Error::new(
                    io::ErrorKind::InvalidInput,
                    "non-normal artifact component",
                ));
            };
            if parts.peek().is_none() {
                return directory
                    .open_file(
                        name,
                        Access::Read,
                        FilePolicy {
                            forbidden: 0,
                            single_link: false,
                        },
                    )
                    .map_err(io_error)?
                    .ok_or_else(|| io::ErrorKind::NotFound.into());
            }
            directory = directory
                .open_directory(name, 0)
                .map_err(io_error)?
                .ok_or_else(|| io::Error::from(io::ErrorKind::NotFound))?;
        }

        Err(io::Error::new(
            io::ErrorKind::InvalidInput,
            "empty artifact path",
        ))
    }

    pub(crate) fn close(self) -> Result<(), (PathBuf, io::Error)> {
        let path = self.scratch.path().to_owned();

        self.scratch.close().map_err(|error| (path, error))
    }
}

fn io_error(error: ic_host_fs::Error) -> io::Error {
    match error {
        ic_host_fs::Error::Io(error) => error,
        error => io::Error::other(error),
    }
}
