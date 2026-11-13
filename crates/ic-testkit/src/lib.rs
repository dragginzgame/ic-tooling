//! Focused `PocketIC` test-harness utilities for Internet Computer canisters.
//!
//! Import native clients and DTOs through `pocket_ic`, the selected upstream crate.
//! `pic` adds optional typed Wire calls and free functions for
//! installation, diagnostics and snapshots, startup errors, caller-owned
//! managed-server startup and bounded readiness polling. It does not provide a
//! simulator wrapper or a host-wide runtime lock.
//!
//! Deterministic test principals through [`fake_principal`] are available on
//! every target. The client and `pic` helpers are native-only.
#![forbid(unsafe_code)]

/// The upstream `PocketIC` client selected by Testkit, with unchanged type identity.
///
/// Consumers can use this path without declaring a separate `PocketIC` dependency.
/// The upstream API and failure behavior apply; this does not restore the retired fork.
///
/// ```no_run
/// use ic_testkit::{pic::PocketIcStartupConfig, pocket_ic::PocketIcBuilder};
/// use std::time::Duration;
///
/// # async fn example() -> Result<(), Box<dyn std::error::Error>> {
/// let config = PocketIcStartupConfig::from_env(Duration::from_secs(30))?;
/// let owner = config.build_instance(
///     PocketIcBuilder::new()
///         .with_max_request_time_ms(Some(30_000))
///         .with_application_subnet(),
/// ).await?;
/// let client: &ic_testkit::pocket_ic::nonblocking::PocketIc = owner.instance();
/// let canister: ic_testkit::pocket_ic::CanisterId = client.create_canister().await;
/// assert!(client.canister_exists(canister).await);
/// owner.close().await;
/// # Ok(())
/// # }
/// ```
#[cfg(not(target_family = "wasm"))]
pub use pocket_ic;

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
