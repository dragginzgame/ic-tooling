//! Streaming reads through a borrowed Unix descriptor, without a duplicate handle.

use ic_host_artifacts::artifact::{ArtifactError, ArtifactIdentity, hash_reader};
use std::{
    io::{self, Read},
    os::fd::AsFd,
};

/// Hash bytes from an already-open descriptor at its current shared offset.
///
/// Borrows the descriptor without reopening a path, duplicating or closing it.
/// The stream uses the canonical inclusive byte limit and overflow probe. This
/// performs no type, path, permission or size admission; callers must establish
/// those properties before calling when they matter. Offset advancement is
/// visible through other handles sharing the open file description, including
/// on failure. Callers own blocking/deadline policy and content/namespace custody.
///
/// # Errors
/// Preserves native read errors and refuses bytes beyond `max_bytes`.
pub fn hash_descriptor(
    descriptor: &impl AsFd,
    max_bytes: u64,
) -> Result<ArtifactIdentity, ArtifactError> {
    hash_reader(DescriptorReader(descriptor), max_bytes)
}

struct DescriptorReader<'a, F>(&'a F);

impl<F: AsFd> Read for DescriptorReader<'_, F> {
    fn read(&mut self, buffer: &mut [u8]) -> io::Result<usize> {
        rustix::io::read(self.0, buffer).map_err(Into::into)
    }
}

#[cfg(test)]
mod tests;
