//! Focused `PocketIC` test-harness utilities for Internet Computer canisters.
//!
//! Import native `PocketIC` clients and DTOs from `ic_testkit::pocket_ic`.
//! `pic` adds optional typed Wire calls, extension traits for
//! installation, diagnostics and snapshots, startup errors, caller-owned
//! managed-server startup and bounded readiness polling. It does not provide a
//! simulator wrapper or a host-wide runtime lock.
//!
//! The crate also provides:
//!
//! - marker parsing, aggregation, comparison, and reports in [`benchmark`];
//! - canister-side marker emission in `performance` (Wasm only);
//! - deterministic test principals through [`fake_principal`].
//!
//! The `pic` helpers are unavailable
//! when compiling for `wasm32`; parsing and analysis are native-only. Benchmark
//! data types, checked marker formatting and marker emission remain
//! available to canister code.

pub mod benchmark;

#[cfg(not(target_arch = "wasm32"))]
pub mod pocket_ic;

#[cfg(not(target_arch = "wasm32"))]
pub mod pic;

#[cfg(target_arch = "wasm32")]
pub mod performance;

use ic_platform_types::principal::ExternalPrincipal as Principal;

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
