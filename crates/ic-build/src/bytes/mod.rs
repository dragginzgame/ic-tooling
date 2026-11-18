//! Build cancellation adapter around the shared byte-admission owner.

#[cfg(test)]
mod tests;

use std::{
    io::{self, Read},
    sync::atomic::{AtomicBool, Ordering},
};

use ic_host_artifacts::{Error, Length};

use crate::Failure;

#[derive(Debug, thiserror::Error)]
#[error("build capture cancelled")]
struct Cancelled;

struct Reader<'a, R> {
    inner: R,
    cancelled: &'a AtomicBool,
}
impl<R: Read> Read for Reader<'_, R> {
    fn read(&mut self, output: &mut [u8]) -> io::Result<usize> {
        if self.cancelled.load(Ordering::Relaxed) {
            return Err(io::Error::other(Cancelled));
        }
        self.inner.read(output)
    }
}

pub(crate) fn capture(
    reader: impl Read,
    limit: usize,
    cancelled: &AtomicBool,
) -> Result<(Vec<u8>, [u8; 32]), Failure> {
    ic_host_artifacts::read(
        Reader {
            inner: reader,
            cancelled,
        },
        Length::AtMost(limit as u64),
    )
    .map(|(bytes, identity)| (bytes, identity.sha256))
    .map_err(|error| match error {
        Error::Io(error) if error.get_ref().is_some_and(|cause| cause.is::<Cancelled>()) => {
            Failure::Cancelled
        }
        Error::Io(error) => Failure::Io(error),
        Error::Limit { .. } | Error::Short { .. } => Failure::ArtifactLimit,
        Error::Allocation => Failure::Allocation,
    })
}
