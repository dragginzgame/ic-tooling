//! Typed CLI translation; admission and publication remain library-owned.

use super::{Operation, batch};
use ic_host_fs::{
    durable::{PublicationMode, WriteOptions, write_bytes},
    read::{hash_file_no_follow, read_file_no_follow},
};
use std::{
    error::Error,
    io::{self, Write as _},
    path::Path,
};

/// Translate an admitted command into the existing FS operations.
pub fn run(operation: Operation) -> Result<(), Box<dyn Error>> {
    match operation {
        Operation::HashFile { path, max_bytes } => {
            let identity = hash_file_no_follow(&path, max_bytes as u64)?;
            writeln!(io::stdout().lock(), "{}", identity.sha256)?;
        }
        Operation::HashFiles0 {
            max_bytes,
            max_list_bytes,
            max_files,
        } => batch::run(max_bytes, max_list_bytes, max_files)?,
        Operation::CreatePrivate {
            source,
            destination,
            max_bytes,
        } => write_private(&source, &destination, max_bytes, PublicationMode::CreateNew)?,
        Operation::ReplacePrivate {
            source,
            destination,
            max_bytes,
        } => write_private(&source, &destination, max_bytes, PublicationMode::Replace)?,
    }
    Ok(())
}

fn write_private(
    source: &Path,
    destination: &Path,
    max_bytes: usize,
    mode: PublicationMode,
) -> Result<(), Box<dyn Error>> {
    let bytes = read_file_no_follow(source, max_bytes)?;
    write_bytes(
        destination,
        WriteOptions {
            mode,
            permissions: 0o600,
        },
        &bytes,
    )?;
    Ok(())
}
