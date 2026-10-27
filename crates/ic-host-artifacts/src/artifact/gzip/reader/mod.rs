//! Pull-based traversal and mandatory single-member completion.

#[cfg(test)]
mod tests;

use std::{
    fmt,
    io::{self, BufRead, Read},
};

/// A bounded gzip traversal or its consumer failed.
#[derive(Debug)]
pub enum GzipReadError<E> {
    /// The buffered source failed; preserves the original native error.
    Input(io::Error),
    /// Gzip headers, compressed data, CRC or length were invalid/incomplete.
    Decode(io::Error),
    /// Compressed input exceeded the inclusive allowance.
    InputLimit {
        /// Maximum compressed bytes exposed to the decoder.
        limit: u64,
    },
    /// Expanded bytes exceeded the inclusive allowance.
    DecodedLimit {
        /// Maximum expanded bytes exposed to the consumer or drained.
        limit: u64,
    },
    /// A byte or another member follows the complete first member.
    TrailingData,
    /// The consumer failed before a complete result could be admitted.
    Consumer(E),
}

impl<E: fmt::Display> fmt::Display for GzipReadError<E> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Input(source) => write!(f, "gzip input failed: {source}"),
            Self::Decode(source) => write!(f, "gzip decoding failed: {source}"),
            Self::InputLimit { limit } => write!(f, "gzip input exceeds {limit} bytes"),
            Self::DecodedLimit { limit } => write!(f, "gzip payload exceeds {limit} bytes"),
            Self::TrailingData => f.write_str("bytes follow the single gzip member"),
            Self::Consumer(source) => write!(f, "gzip consumer failed: {source}"),
        }
    }
}

impl<E: std::error::Error + 'static> std::error::Error for GzipReadError<E> {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        match self {
            Self::Input(source) | Self::Decode(source) => Some(source),
            Self::Consumer(source) => Some(source),
            _ => None,
        }
    }
}

/// Let a synchronous consumer pull bytes from one bounded gzip member.
///
/// The consumer borrows a decoded [`Read`] interface, never a backend type.
/// Its successful return is provisional: this function drains the remaining
/// bounded payload, validates the footer, and probes for trailing input before
/// returning its result. Even ignored decoder errors prevent success. Consumer
/// failure returns without draining; an observed stream failure takes precedence
/// over the consumer error and retains its original native cause.
///
/// Limits are inclusive. Compressed accounting counts bytes exposed to/consumed
/// by the decoder, including headers/footer; the caller's `BufRead` may already
/// buffer additional bytes. At an exact limit, one extra byte is observed to
/// distinguish real EOF from overflow. Expanded overflow is similarly probed
/// without exposing that byte to the consumer. No complete input or output is
/// allocated: decoding and completion use fixed-size traversal storage plus the
/// backend's working memory. Only interrupted reads are retried.
///
/// The supplied reader defines the complete logical compressed input. A
/// caller-created truncated view cannot establish EOF of its underlying file
/// or transport. Supply that complete source and caller-owned read deadlines.
/// This function opens
/// no paths and chooses no staging, publication, rollback or digest policy.
/// Consumer observations/effects remain provisional until completion succeeds;
/// earlier effects cannot be rolled back by this library.
///
/// # Errors
/// Returns distinct source IO, gzip integrity, compressed/expanded limit,
/// trailing-data and typed consumer errors. No validated result is returned on
/// failure, including when the consumer stops at a parser's logical EOF.
pub fn with_gzip_reader<R: BufRead, T, E>(
    input: R,
    max_compressed_bytes: u64,
    max_decoded_bytes: u64,
    consume: impl FnOnce(&mut dyn Read) -> Result<T, E>,
) -> Result<T, GzipReadError<E>> {
    let mut reader = Payload {
        decoder: flate2::bufread::GzDecoder::new(Input {
            inner: input,
            remaining: max_compressed_bytes,
            limit: max_compressed_bytes,
            failure: None,
        }),
        remaining: max_decoded_bytes,
        limit: max_decoded_bytes,
        failure: None,
    };
    let result = consume(&mut reader);
    if let Some(error) = reader.take_failure() {
        return Err(error);
    }
    let result = result.map_err(GzipReadError::Consumer)?;
    let mut scratch = [0; 16 * 1024];
    loop {
        match reader.read(&mut scratch) {
            Ok(0) => break,
            Ok(_) => {}
            Err(error) => {
                return Err(reader
                    .take_failure()
                    .unwrap_or(GzipReadError::Decode(error)));
            }
        }
    }
    loop {
        match reader.decoder.get_mut().fill_buf() {
            Ok([]) => return Ok(result),
            Ok(_) => return Err(GzipReadError::TrailingData),
            Err(error) if error.kind() == io::ErrorKind::Interrupted => {}
            Err(error) => return Err(reader.take_failure().unwrap_or(GzipReadError::Input(error))),
        }
    }
}

enum Failure {
    Input(io::Error),
    Decode(io::Error),
    InputLimit(u64),
    DecodedLimit(u64),
}

impl Failure {
    fn into_error<E>(self) -> GzipReadError<E> {
        match self {
            Self::Input(source) => GzipReadError::Input(source),
            Self::Decode(source) => GzipReadError::Decode(source),
            Self::InputLimit(limit) => GzipReadError::InputLimit { limit },
            Self::DecodedLimit(limit) => GzipReadError::DecodedLimit { limit },
        }
    }
}

fn failed_read() -> io::Error {
    // The original typed cause stays with the owner even if a parser wraps or
    // ignores this IO error. No source/decode identity depends on error prose.
    io::Error::new(io::ErrorKind::InvalidData, "bounded gzip traversal failed")
}

struct Input<R> {
    inner: R,
    remaining: u64,
    limit: u64,
    failure: Option<Failure>,
}

impl<R: BufRead> Read for Input<R> {
    fn read(&mut self, buffer: &mut [u8]) -> io::Result<usize> {
        if buffer.is_empty() {
            return Ok(0);
        }
        let available = self.fill_buf()?;
        let count = available.len().min(buffer.len());
        buffer[..count].copy_from_slice(&available[..count]);
        self.consume(count);
        Ok(count)
    }
}

impl<R: BufRead> BufRead for Input<R> {
    fn fill_buf(&mut self) -> io::Result<&[u8]> {
        if self.failure.is_some() {
            return Err(failed_read());
        }
        let available = match self.inner.fill_buf() {
            Ok(bytes) => bytes,
            Err(error) if error.kind() == io::ErrorKind::Interrupted => return Err(error),
            Err(error) => {
                self.failure = Some(Failure::Input(error));
                return Err(failed_read());
            }
        };
        if self.remaining == 0 && !available.is_empty() {
            self.failure = Some(Failure::InputLimit(self.limit));
            return Err(failed_read());
        }
        let count = available
            .len()
            .min(usize::try_from(self.remaining).unwrap_or(usize::MAX));
        Ok(&available[..count])
    }

    fn consume(&mut self, amount: usize) {
        self.inner.consume(amount);
        self.remaining -= amount as u64;
    }
}

struct Payload<R: BufRead> {
    decoder: flate2::bufread::GzDecoder<Input<R>>,
    remaining: u64,
    limit: u64,
    failure: Option<Failure>,
}

impl<R: BufRead> Payload<R> {
    fn take_failure<E>(&mut self) -> Option<GzipReadError<E>> {
        self.failure
            .take()
            .or_else(|| self.decoder.get_mut().failure.take())
            .map(Failure::into_error)
    }
}

impl<R: BufRead> Read for Payload<R> {
    fn read(&mut self, buffer: &mut [u8]) -> io::Result<usize> {
        if buffer.is_empty() {
            return Ok(0);
        }
        if self.failure.is_some() || self.decoder.get_mut().failure.is_some() {
            return Err(failed_read());
        }
        loop {
            let mut probe = [0];
            let allowance = buffer
                .len()
                .min(usize::try_from(self.remaining).unwrap_or(usize::MAX));
            let output = if allowance == 0 {
                &mut probe[..]
            } else {
                &mut buffer[..allowance]
            };
            match self.decoder.read(output) {
                Ok(count) if count > allowance => {
                    self.failure = Some(Failure::DecodedLimit(self.limit));
                    return Err(failed_read());
                }
                Ok(count) => {
                    self.remaining -= count as u64;
                    return Ok(count);
                }
                Err(error) if error.kind() == io::ErrorKind::Interrupted => {}
                Err(error) => {
                    self.failure = Some(
                        self.decoder
                            .get_mut()
                            .failure
                            .take()
                            .unwrap_or(Failure::Decode(error)),
                    );
                    return Err(failed_read());
                }
            }
        }
    }
}
