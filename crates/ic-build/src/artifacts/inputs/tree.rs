//! No-follow traversal for caller-declared input inventories.

use std::{
    ffi::OsString,
    fs, io,
    path::{Path, PathBuf},
};

pub(super) enum InputTreeEntry<'a> {
    File { path: &'a Path },
    Directory { path: &'a Path },
    Missing { path: &'a Path },
}

#[derive(Debug, thiserror::Error)]
#[error("{}: {source}", .path.display())]
pub(super) struct InputPathError {
    pub(super) path: PathBuf,
    pub(super) source: io::Error,
}

pub(super) struct InputTreeWalk<'a> {
    excluded_roots: &'a [PathBuf],
    excluded_names: &'a [OsString],
}

impl<'a> InputTreeWalk<'a> {
    pub(super) const fn new(excluded_roots: &'a [PathBuf], excluded_names: &'a [OsString]) -> Self {
        Self {
            excluded_roots,
            excluded_names,
        }
    }

    pub(super) fn visit(
        &self,
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
        let metadata = match fs::symlink_metadata(path) {
            Ok(metadata) if metadata.file_type().is_symlink() => {
                return Err(context(io::Error::new(
                    io::ErrorKind::InvalidInput,
                    "input is a symbolic link",
                )));
            }
            Ok(metadata) => metadata,
            Err(error) if error.kind() == io::ErrorKind::NotFound => {
                let resolved = super::canonicalize_allow_missing(path).map_err(context)?;
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
        };
        let canonical = path.canonicalize().map_err(context)?;
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
        if metadata.is_file() {
            return visitor(InputTreeEntry::File { path }).map_err(context);
        }
        if !metadata.is_dir() {
            return Err(context(io::Error::new(
                io::ErrorKind::InvalidInput,
                "input is not a regular file or directory",
            )));
        }
        visitor(InputTreeEntry::Directory { path }).map_err(context)?;
        let mut entries = fs::read_dir(path)
            .map_err(context)?
            .map(|entry| entry.map(|entry| entry.file_name()))
            .collect::<Result<Vec<_>, _>>()
            .map_err(context)?;
        entries.sort_unstable();
        for name in entries {
            if !self.excluded_names.contains(&name) {
                self.visit(&path.join(name), false, visitor)?;
            }
        }
        Ok(())
    }
}
