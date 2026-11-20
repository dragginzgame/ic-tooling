//! Immutable, completely admitted catalogs and generated held-update modules.
//!
//! Repeated method names represent distinct lifetime slots in input order. Every
//! slot is an update by construction; query methods are not part of this API.

#[cfg(test)]
mod tests;

mod control;
mod module;

use std::collections::TryReserveError;

pub use control::{ControlError, Fixture, Request};
pub use module::CompileError;

///
/// Reply
/// A borrowed update method and the exact bytes its eventual callback returns.
///

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct Reply<'a> {
    /// Nonempty UTF-8 method name without control characters.
    pub method: &'a str,
    /// Opaque bytes, including empty or non-UTF-8 data.
    pub bytes: &'a [u8],
}

///
/// Limits
/// Caller-selected aggregate admission budgets; repeated names count per slot.
///

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct Limits {
    /// Maximum number of lifetime reply slots.
    pub slots: usize,
    /// Maximum sum of UTF-8 method-name bytes across all slots.
    pub name_bytes: usize,
    /// Maximum sum of reply bytes across all slots.
    pub reply_bytes: usize,
}

///
/// Budget
/// The budget exhausted during complete catalog admission.
///

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum Budget {
    /// Number of reply slots.
    Slots,
    /// Sum of method-name bytes.
    Names,
    /// Sum of reply bytes.
    Replies,
}

///
/// Error
/// Complete-catalog refusal; no partially owned catalog escapes.
///

#[derive(Debug, thiserror::Error)]
pub enum Error {
    /// A name is empty or contains a Unicode control character.
    #[error("invalid method name at slot {slot}")]
    InvalidName {
        /// Input position, not the method contents.
        slot: usize,
    },
    /// An aggregate exceeds the selected budget, including arithmetic overflow.
    #[error("catalog exceeds {budget:?} limit {limit}")]
    Limit {
        /// Exhausted dimension.
        budget: Budget,
        /// Caller-selected maximum.
        limit: usize,
    },
    /// Slot indexing or immutable data cannot fit the fixture's Wasm32 layout.
    #[error("catalog exceeds Wasm32 representation at slot {slot}")]
    Representation {
        /// First unrepresentable input position.
        slot: usize,
    },
    /// Reserving owned catalog storage failed.
    #[error("catalog allocation failed: {0}")]
    Allocation(#[from] TryReserveError),
}

///
/// Slot
/// Stores a held method name with its exact reply bytes.
///

#[derive(Debug)]
struct Slot {
    method: String,
    bytes: Vec<u8>,
}

///
/// Catalog
/// An owned immutable catalog, preserving duplicate names, order and exact bytes.
///
/// No generated module, server or installation is created. Input budgets bound
/// retained payload/name lengths, not allocator overhead or compiler memory.
///

#[derive(Debug)]
pub struct Catalog {
    slots: Vec<Slot>,
}

impl Catalog {
    /// Admit all descriptors before copying any method or reply bytes.
    ///
    /// The empty catalog is valid. Local allocations completed before an allocation
    /// refusal are dropped; no partial catalog is returned. Compiled-module limits
    /// remain a separate generation obligation, not a promise of this admission.
    ///
    /// # Errors
    /// Returns the first invalid name, exhausted budget, Wasm32 representation
    /// refusal or original allocation failure.
    pub fn new(replies: &[Reply<'_>], limits: Limits) -> Result<Self, Error> {
        charge(0, replies.len(), limits.slots, Budget::Slots)?;
        if replies.len() > i32::MAX as usize {
            return Err(Error::Representation {
                slot: i32::MAX as usize,
            });
        }
        let mut names = 0;
        let mut bytes = 0;
        for (slot, reply) in replies.iter().enumerate() {
            if reply.method.is_empty() || reply.method.chars().any(char::is_control) {
                return Err(Error::InvalidName { slot });
            }
            names = charge(names, reply.method.len(), limits.name_bytes, Budget::Names)?;
            bytes = charge(
                bytes,
                reply.bytes.len(),
                limits.reply_bytes,
                Budget::Replies,
            )?;

            // Generated Wasm uses nonnegative i32 offsets and explicit name lengths.
            // Metadata/encoding overhead must be admitted again by the generator.
            if bytes > i32::MAX as usize || reply.method.len() > i32::MAX as usize {
                return Err(Error::Representation { slot });
            }
        }
        let mut slots = Vec::new();
        slots.try_reserve_exact(replies.len())?;
        for reply in replies {
            let mut method = String::new();
            method.try_reserve_exact(reply.method.len())?;
            method.push_str(reply.method);
            let mut bytes = Vec::new();
            bytes.try_reserve_exact(reply.bytes.len())?;
            bytes.extend_from_slice(reply.bytes);
            slots.push(Slot { method, bytes });
        }

        Ok(Self { slots })
    }

    /// Number of distinct lifetime slots, including repeated method names.
    #[must_use]
    pub fn len(&self) -> usize {
        self.slots.len()
    }

    /// Whether the catalog admits no calls.
    #[must_use]
    pub fn is_empty(&self) -> bool {
        self.slots.is_empty()
    }

    /// Inspect one immutable slot by its original input position.
    #[must_use]
    pub fn get(&self, slot: usize) -> Option<Reply<'_>> {
        self.slots.get(slot).map(|entry| Reply {
            method: &entry.method,
            bytes: &entry.bytes,
        })
    }
}

fn charge(used: usize, added: usize, limit: usize, budget: Budget) -> Result<usize, Error> {
    used.checked_add(added)
        .filter(|total| *total <= limit)
        .ok_or(Error::Limit { budget, limit })
}
