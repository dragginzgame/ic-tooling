//! Complete producer hashing without retaining payload bytes or owning a sink.
use std::io::{self, Write};

use sha2::{Digest, Sha256};

use crate::{Error, Identity, Length};

/// A producer failed, or its completed byte stream failed admission.
#[derive(Debug, thiserror::Error)]
pub enum ProduceError<E> {
    /// Original producer cause; no complete identity is available.
    #[error("byte producer refused: {0}")]
    Producer(#[source] E),
    /// Length admission failed, including a write refusal the producer swallowed.
    #[error(transparent)]
    Admission(Error),
}

/// Hash one completed producer operation with constant payload memory.
///
/// The producer owns framing and must report its own failures. Its writer only
/// hashes: it does not retain bytes, publish output or flush an external sink.
/// An over-limit write accepts none of that write and permanently refuses further
/// writes. No digest snapshot or partial-success result escapes this operation.
/// A producer error takes precedence over length admission and is retained intact.
///
/// # Errors
/// Returns the original producer error, or a limit/short-input admission error.
/// Catching an over-limit write inside the producer cannot authorize success.
pub fn hash_with<E>(
    length: Length,
    produce: impl FnOnce(&mut dyn Write) -> Result<(), E>,
) -> Result<Identity, ProduceError<E>> {
    let (Length::AtMost(limit) | Length::Exactly(limit)) = length;
    let mut writer = Producer {
        digest: Sha256::new(),
        bytes: 0,
        limit,
        refused: false,
    };
    produce(&mut writer).map_err(ProduceError::Producer)?;
    if writer.refused {
        return Err(ProduceError::Admission(Error::Limit { limit }));
    }
    if matches!(length, Length::Exactly(_)) && writer.bytes != limit {
        return Err(ProduceError::Admission(Error::Short {
            expected: limit,
            actual: writer.bytes,
        }));
    }
    Ok(Identity {
        bytes: writer.bytes,
        sha256: writer.digest.finalize().into(),
    })
}

struct Producer {
    digest: Sha256,
    bytes: u64,
    limit: u64,
    refused: bool,
}

impl Write for Producer {
    fn write(&mut self, bytes: &[u8]) -> io::Result<usize> {
        if self.refused || bytes.len() as u64 > self.limit - self.bytes {
            self.refused = true;
            return Err(io::Error::other("byte producer limit exceeded"));
        }
        self.digest.update(bytes);
        self.bytes += bytes.len() as u64;
        Ok(bytes.len())
    }

    fn flush(&mut self) -> io::Result<()> {
        if self.refused {
            return Err(io::Error::other("byte producer limit exceeded"));
        }
        Ok(())
    }
}

#[cfg(test)]
mod tests;
