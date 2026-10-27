//! Pure wire encodings, independent of transport, trust and application fields.

#[cfg(test)]
mod tests;

/// A complete canonical unsigned `u64` LEB128 value could not be admitted.
#[derive(Clone, Copy, Debug, Eq, PartialEq, thiserror::Error)]
pub enum UnsignedLeb128Error {
    /// Input ended before a terminating byte.
    #[error("truncated unsigned LEB128")]
    Truncated,
    /// The value or continuation exceeds the ten-byte `u64` representation.
    #[error("unsigned LEB128 exceeds u64")]
    Overflow,
    /// Bytes follow the first terminating byte.
    #[error("bytes follow unsigned LEB128")]
    TrailingData,
    /// The representation contains an unnecessary leading zero group.
    #[error("noncanonical unsigned LEB128")]
    NonCanonical,
}

/// Decode exactly one minimally encoded unsigned `u64` LEB128 value.
///
/// Inspects at most ten bytes and allocates nothing. Trailing data takes
/// precedence over nonminimal encoding; a tenth group outside `u64` or with a
/// continuation bit is overflow. Callers own field labels and semantic limits.
///
/// # Errors
/// Returns a typed grammar, range or complete-input admission failure.
pub fn decode_canonical_unsigned_u64(bytes: &[u8]) -> Result<u64, UnsignedLeb128Error> {
    let mut value = 0_u64;
    for (index, byte) in bytes.iter().copied().take(10).enumerate() {
        let payload = byte & 0x7f;
        if index == 9 && (payload > 1 || byte & 0x80 != 0) {
            return Err(UnsignedLeb128Error::Overflow);
        }
        value |= u64::from(payload) << (index * 7);
        if byte & 0x80 == 0 {
            if index + 1 != bytes.len() {
                return Err(UnsignedLeb128Error::TrailingData);
            }
            if index > 0 && payload == 0 {
                return Err(UnsignedLeb128Error::NonCanonical);
            }
            return Ok(value);
        }
    }
    Err(UnsignedLeb128Error::Truncated)
}
