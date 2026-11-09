//! Bounded single-value Candid decoding with caller-owned resource policy.
//!
//! This module does not authenticate replies or interpret application errors.

use ic_platform::candid_codec::decoder::{self as codec, Arity, Decode};
use thiserror::Error;

#[cfg(test)]
mod tests;

/// Caller-selected bounds for one reply, independent of the codec supplier.
///
/// Work and cumulative logical allocation charges use Platform's owned codec
/// units. They are not interchangeable with the retired registry decoder quotas.
/// There are no defaults: the caller selects every resource allowance.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct DecodeLimits {
    /// Maximum complete encoded reply bytes, checked before aggregate charging.
    pub reply_bytes: usize,
    /// Maximum cumulative byte reads and traversal charges.
    pub work: usize,
    /// Maximum cumulative logical storage charges (not peak heap).
    pub allocation: usize,
    /// Maximum Candid type-table entries.
    pub type_entries: usize,
    /// Maximum encoded Candid header bytes.
    pub header_bytes: usize,
}

/// Resource admission failures are distinct from malformed or incompatible Candid.
#[derive(Debug, Error)]
pub enum DecodeError {
    /// The encoded reply exceeds the per-reply allowance.
    #[error("Candid reply exceeds its byte limit")]
    ReplyBytes,
    /// The reply cannot be admitted within the aggregate allowance.
    #[error("Candid reply exceeds the remaining aggregate byte budget")]
    AggregateBytes,
    /// Malformed, incompatible or budget-exhausting Candid.
    #[error("Candid reply could not be decoded: {0:?}")]
    Candid(codec::Error),
}

/// Decode one result using Candid's compatibility rules, charging admitted bytes.
///
/// Oversized replies do not consume aggregate allowance. Once admitted, a reply
/// consumes its bytes before decoding, so malformed replies cannot evade accounting.
/// Caller policy owns budgets; this helper never authenticates the decoded value.
/// Import per-reply limits from this module's [`DecodeLimits`].
/// Extra wire arguments are skipped under the selected work budget, like Candid's
/// decoder; trailing undecodable bytes reject. These are byte and decoder-work
/// limits, not a peak-heap, stack or elapsed-time guarantee. Typed refusals do
/// not include reply bytes.
///
/// # Errors
/// Returns byte admission failure before decoding, or the original Candid error.
pub fn decode_one<T: for<'a> Decode<'a>>(
    bytes: &[u8],
    limits: DecodeLimits,
    remaining: &mut usize,
) -> Result<T, DecodeError> {
    if bytes.len() > limits.reply_bytes {
        return Err(DecodeError::ReplyBytes);
    }
    *remaining = remaining
        .checked_sub(bytes.len())
        .ok_or(DecodeError::AggregateBytes)?;
    let limits = codec::Limits {
        bytes: limits.reply_bytes,
        work: limits.work,
        allocation: limits.allocation,
        types: limits.type_entries,
        header: limits.header_bytes,
    };
    codec::decode_args::<(T,)>(bytes, limits, Arity::Compatible)
        .map(|(value,)| value)
        .map_err(DecodeError::Candid)
}
