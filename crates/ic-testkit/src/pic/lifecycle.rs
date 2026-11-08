use std::{num::NonZeroUsize, time::Duration};

use crate::pocket_ic::{ClientError, ErrorCode, PocketIc};
use ic_platform_types::principal::ExternalPrincipal as Principal;

use super::CanisterInstallError;

/// Inputs and diagnostic context for one generic canister installation.
pub struct InstallSpec {
    /// Wasm module installed into the newly created canister.
    pub wasm: Vec<u8>,
    /// Raw initialization bytes encoded by the caller.
    pub init_bytes: Vec<u8>,
    /// Extra cycles added after `PocketIC` creates the canister and before install.
    pub cycles: u128,
    /// Optional sender for the management-canister install operation.
    pub install_sender: Option<Principal>,
    /// Optional human-readable context included in install errors.
    pub label: Option<String>,
}

/// Retry limits and simulated cooldown for install-code operations.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct RetryPolicy {
    /// Maximum attempts, including the initial call.
    pub max_attempts: NonZeroUsize,
    /// Simulated cooldown between retryable attempts.
    pub cooldown: Duration,
}

impl PocketIc {
    /// Create and install one canister from a reusable specification.
    /// Creates the canister using `PocketIC` defaults; `InstallSpec::cycles`
    /// adds a top-up rather than setting its complete initial balance.
    ///
    /// Retains typed client failures during creation, cycle funding, and installation.
    /// The error identifies the stage and retains an id when creation succeeded.
    /// Callers choose whether to collect diagnostics; this performs no diagnostic
    /// requests or error logging.
    ///
    /// # Errors
    /// Returns the failed creation or installation stage, retaining canister identity when available.
    pub async fn try_create_and_install(
        &self,
        spec: InstallSpec,
    ) -> Result<Principal, CanisterInstallError> {
        let canister_id = self.create_canister().await.map_err(|source| {
            CanisterInstallError::CreateCanister {
                label: spec.label.clone(),
                source,
            }
        })?;
        if spec.cycles > 0 {
            self.add_cycles(canister_id, spec.cycles)
                .await
                .map_err(|source| CanisterInstallError::AddCycles {
                    canister_id,
                    label: spec.label.clone(),
                    source,
                })?;
        }

        // Move the Wasm and init payload while retaining the label for failures.
        self.install_canister(canister_id, spec.wasm, spec.init_bytes, spec.install_sender)
            .await
            .map_err(|source| CanisterInstallError::InstallCode {
                canister_id,
                label: spec.label,
                source,
            })?;

        Ok(canister_id)
    }

    /// Advance simulated time once, then tick twice for an install-code cooldown.
    /// Zero duration skips time reads/writes while retaining both ticks.
    ///
    /// # Errors
    /// Returns the first client failure without dispatching remaining ticks.
    pub async fn wait_out_install_code_rate_limit(
        &self,
        cooldown: Duration,
    ) -> Result<(), ClientError> {
        super::time::advance_and_tick(self, cooldown, 2).await
    }

    /// Retry only while `PocketIC` reports install-code rate limiting.
    ///
    /// `RetryPolicy::max_attempts` includes the initial call. Before each
    /// retry, this advances simulated time by the configured cooldown and
    /// executes two `PocketIC` ticks. Other rejections are returned unchanged.
    ///
    /// # Errors
    /// Returns non-rate-limit failures immediately, or the final rate-limit rejection.
    /// A cooldown failure stops further installation attempts.
    pub async fn retry_install_code<T, F>(
        &self,
        policy: RetryPolicy,
        op: F,
    ) -> Result<T, ClientError>
    where
        F: AsyncFnMut() -> Result<T, ClientError>,
    {
        retry_install_code_with(policy, op, async || {
            self.wait_out_install_code_rate_limit(policy.cooldown).await
        })
        .await
    }
}

async fn retry_install_code_with<T, F, W>(
    policy: RetryPolicy,
    mut op: F,
    mut wait_out_cooldown: W,
) -> Result<T, ClientError>
where
    F: AsyncFnMut() -> Result<T, ClientError>,
    W: AsyncFnMut() -> Result<(), ClientError>,
{
    let mut retries = 1..policy.max_attempts.get();
    loop {
        match op().await {
            Err(ClientError::Canister(err))
                if err.error_code == ErrorCode::CanisterInstallCodeRateLimited
                    && retries.next().is_some() =>
            {
                wait_out_cooldown().await?;
            }
            result => return result,
        }
    }
}

#[cfg(test)]
mod tests;
