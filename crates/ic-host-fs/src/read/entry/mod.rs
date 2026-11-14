//! Shared descriptor entry mechanics; callers own traversal and type admission.

use rustix::{
    fd::{AsFd, OwnedFd},
    fs::{self, AtFlags, FileType, Mode, OFlags},
    path::Arg,
};
use std::io;

pub(super) fn entry_type(parent: impl AsFd, name: impl Arg) -> io::Result<FileType> {
    let metadata = fs::statat(parent, name, AtFlags::SYMLINK_NOFOLLOW).map_err(io::Error::from)?;
    Ok(FileType::from_raw_mode(metadata.st_mode))
}

#[derive(Clone, Copy)]
pub(crate) enum FileAccess {
    ReadOnly,
    #[cfg(feature = "capability")]
    ReadWrite,
}

pub(super) fn open_child(
    parent: impl AsFd,
    name: impl Arg,
    directory: bool,
) -> io::Result<OwnedFd> {
    open_child_with_access(parent, name, directory, FileAccess::ReadOnly)
}

pub(super) fn open_child_with_access(
    parent: impl AsFd,
    name: impl Arg,
    directory: bool,
    access: FileAccess,
) -> io::Result<OwnedFd> {
    let access = match access {
        FileAccess::ReadOnly => OFlags::RDONLY,
        #[cfg(feature = "capability")]
        FileAccess::ReadWrite if !directory => OFlags::RDWR,
        #[cfg(feature = "capability")]
        FileAccess::ReadWrite => OFlags::RDONLY,
    };
    let mut flags = access | OFlags::NOFOLLOW | OFlags::NONBLOCK | OFlags::CLOEXEC;
    if directory {
        flags |= OFlags::DIRECTORY;
    }
    fs::openat(parent, name, flags, Mode::empty()).map_err(io::Error::from)
}

pub(super) fn opened_type(fd: impl AsFd) -> io::Result<FileType> {
    let metadata = fs::fstat(fd).map_err(io::Error::from)?;
    Ok(FileType::from_raw_mode(metadata.st_mode))
}
