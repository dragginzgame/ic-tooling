//! Focused `PocketIC` test-harness utilities for Internet Computer canisters.
//!
//! Import native clients and DTOs directly from upstream `pocket_ic`.
//! `pic` adds optional typed Wire calls and free functions for
//! installation, diagnostics and snapshots, startup errors, caller-owned
//! managed-server startup and bounded readiness polling. It does not provide a
//! simulator wrapper or a host-wide runtime lock.
//!
//! Deterministic test principals through [`fake_principal`] are available on
//! every target. The client and `pic` helpers are native-only.

#[cfg(not(target_family = "wasm"))]
pub mod pic;

use ic_platform_types::Principal;

/// Deterministically derive a test [`Principal`] from `seed`.
///
/// Values are derived directly from a numeric seed, making fixtures stable
/// without embedding textual principal literals.
#[must_use]
pub fn fake_principal(seed: u32) -> Principal {
    let mut buf = [0u8; 29];
    buf[..4].copy_from_slice(&seed.to_be_bytes());

    Principal::from_slice(&buf)
}

#[cfg(test)]
mod tests;

#[cfg(all(test, not(target_family = "wasm")))]
mod test_support;
