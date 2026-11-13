//! Installation policy over the upstream asynchronous client.
use pocket_ic::{CanisterId as Principal, ErrorCode, RejectResponse, nonblocking::PocketIc};
use std::{num::NonZeroUsize, time::Duration};

/// Inputs for one generic canister installation.
pub struct InstallSpec {
    /// Module bytes.
    pub wasm: Vec<u8>,
    /// Caller-encoded initialization argument.
    pub init_bytes: Vec<u8>,
    /// Extra cycles added after creation.
    pub cycles: u128,
    /// Management-call sender.
    pub install_sender: Option<Principal>,
}

/// Caller-selected rate-limit retry policy.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct RetryPolicy {
    /// Maximum attempts, including the initial call.
    pub max_attempts: NonZeroUsize,
    /// Simulated cooldown before retrying.
    pub cooldown: Duration,
}

/// Create, fund and install using upstream defaults and failure semantics.
///
/// Upstream transport, creation, funding and install panics propagate unchanged.
pub async fn create_and_install(pic: &PocketIc, spec: InstallSpec) -> Principal {
    let canister_id = pic.create_canister().await;
    if spec.cycles > 0 {
        pic.add_cycles(canister_id, spec.cycles).await;
    }
    pic.install_canister(canister_id, spec.wasm, spec.init_bytes, spec.install_sender)
        .await;
    canister_id
}

/// Advance simulated time when nonzero, then tick exactly twice.
pub async fn wait_out_install_code_rate_limit(pic: &PocketIc, cooldown: Duration) {
    super::time::advance_and_tick(pic, cooldown, 2).await;
}

/// Retry only install-code rate-limit rejections, never transport panics.
///
/// # Errors
/// Returns the original non-rate-limit rejection or the final rate-limit rejection.
pub async fn retry_install_code<T>(
    pic: &PocketIc,
    policy: RetryPolicy,
    op: impl AsyncFnMut() -> Result<T, RejectResponse>,
) -> Result<T, RejectResponse> {
    retry_install_code_with(policy, op, async || {
        wait_out_install_code_rate_limit(pic, policy.cooldown).await;
    })
    .await
}

async fn retry_install_code_with<T>(
    policy: RetryPolicy,
    mut op: impl AsyncFnMut() -> Result<T, RejectResponse>,
    mut wait: impl AsyncFnMut(),
) -> Result<T, RejectResponse> {
    let mut retries = 1..policy.max_attempts.get();
    loop {
        match op().await {
            Err(error)
                if error.error_code == ErrorCode::CanisterInstallCodeRateLimited
                    && retries.next().is_some() =>
            {
                wait().await;
            }
            result => return result,
        }
    }
}

#[cfg(test)]
mod tests;
