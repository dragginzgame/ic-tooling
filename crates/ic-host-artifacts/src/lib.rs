//! Bounded byte admission and exact content identity, independent of native hosts.

#[cfg(test)]
mod tests;

mod producer;

use std::io::{self, Read, Write};

use sha2::{Digest, Sha256};
use thiserror::Error;

pub use producer::{ProduceError, hash_with};

/// Explicit complete-stream admission policy; one extra byte may be observed.
#[derive(Clone, Copy, Debug)]
pub enum Length {
    /// Admit EOF at or below this ceiling.
    AtMost(u64),
    /// Require exactly this many bytes followed by EOF.
    Exactly(u64),
}

/// Identity of successfully admitted original bytes, not origin authentication.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct Identity {
    /// Complete admitted length.
    pub bytes: u64,
    /// SHA-256 of the original bytes without framing.
    pub sha256: [u8; 32],
}

/// Admission refusal; arbitrary writers may already have partial effects.
#[derive(Debug, Error)]
pub enum Error {
    /// Original reader or writer cause.
    #[error("byte I/O refused: {0}")]
    Io(#[from] io::Error),
    /// One byte beyond the allowed length was observed.
    #[error("input exceeds {limit} bytes")]
    Limit {
        /// Selected maximum length.
        limit: u64,
    },
    /// EOF arrived before an exact declaration.
    #[error("expected {expected} bytes, received {actual}")]
    Short {
        /// Declared length.
        expected: u64,
        /// Observed length.
        actual: u64,
    },
    /// Owned buffer reservation failed.
    #[error("byte buffer allocation refused")]
    Allocation,
}

/// Copy and hash with constant scratch memory; publication is the filesystem owner's job.
///
/// # Errors
/// Refuses I/O, excess input or early EOF. No successful identity is returned for
/// partial effects. Interrupted reads retry; other original I/O causes are preserved.
pub fn copy(
    mut reader: impl Read,
    mut writer: impl Write,
    length: Length,
) -> Result<Identity, Error> {
    pump(&mut reader, length, |chunk| {
        writer.write_all(chunk).map_err(Error::Io)
    })
}

/// Read complete bounded bytes with fallible buffer growth, returning their identity.
///
/// # Errors
/// Refuses input/length/allocation failures without exposing partial bytes.
pub fn read(mut reader: impl Read, length: Length) -> Result<(Vec<u8>, Identity), Error> {
    let mut bytes = Vec::new();
    let identity = pump(&mut reader, length, |chunk| {
        bytes
            .try_reserve(chunk.len())
            .map_err(|_| Error::Allocation)?;
        bytes.extend_from_slice(chunk);
        Ok(())
    })?;
    Ok((bytes, identity))
}

fn pump(
    reader: &mut impl Read,
    length: Length,
    mut admit: impl FnMut(&[u8]) -> Result<(), Error>,
) -> Result<Identity, Error> {
    let (Length::AtMost(limit) | Length::Exactly(limit)) = length;
    let mut buffer = [0_u8; 8192];
    let mut bytes = 0;
    let mut digest = Sha256::new();
    loop {
        let room = limit - bytes;
        let take = room.saturating_add(1).min(buffer.len() as u64) as usize;
        let count = match reader.read(&mut buffer[..take]) {
            Err(error) if error.kind() == io::ErrorKind::Interrupted => continue,
            result => result?,
        };
        if count == 0 {
            if matches!(length, Length::Exactly(_)) && bytes != limit {
                return Err(Error::Short {
                    expected: limit,
                    actual: bytes,
                });
            }
            return Ok(Identity {
                bytes,
                sha256: digest.finalize().into(),
            });
        }
        if count as u64 > room {
            return Err(Error::Limit { limit });
        }
        admit(&buffer[..count])?;
        digest.update(&buffer[..count]);
        bytes += count as u64;
    }
}
