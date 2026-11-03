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

/// Generic canister installation and structured install-code retry policy.
///
/// The extension creates canisters using `PocketIC` defaults. `InstallSpec::cycles`
/// is an additional top-up, not the complete initial balance.
pub trait CanisterInstallExt {
    /// Create and install one canister from a reusable specification.
    ///
    /// Retains typed client failures during creation, cycle funding, and installation.
    /// The error identifies the stage and retains an id when creation succeeded.
    /// Callers choose whether to collect diagnostics; this performs no diagnostic
    /// requests or error logging.
    ///
    /// # Errors
    /// Returns the failed creation or installation stage, retaining canister identity when available.
    fn try_create_and_install(&self, spec: InstallSpec) -> Result<Principal, CanisterInstallError>;

    /// Advance simulated time once, then tick twice for an install-code cooldown.
    /// Zero duration skips time reads/writes while retaining both ticks.
    ///
    /// # Errors
    /// Returns the first client failure without dispatching remaining ticks.
    fn wait_out_install_code_rate_limit(&self, cooldown: Duration) -> Result<(), ClientError>;

    /// Retry only while `PocketIC` reports install-code rate limiting.
    ///
    /// `RetryPolicy::max_attempts` includes the initial call. Before each
    /// retry, this advances simulated time by the configured cooldown and
    /// executes two `PocketIC` ticks. Other rejections are returned unchanged.
    ///
    /// # Errors
    /// Returns non-rate-limit failures immediately, or the final rate-limit rejection.
    /// A cooldown failure stops further installation attempts.
    fn retry_install_code<T, F>(&self, policy: RetryPolicy, op: F) -> Result<T, ClientError>
    where
        F: FnMut() -> Result<T, ClientError>;
}

impl CanisterInstallExt for PocketIc {
    /// Install one arbitrary wasm module from a generic install specification.
    fn try_create_and_install(&self, spec: InstallSpec) -> Result<Principal, CanisterInstallError> {
        let canister_id =
            self.create_canister()
                .map_err(|source| CanisterInstallError::CreateCanister {
                    label: spec.label.clone(),
                    source,
                })?;
        if spec.cycles > 0 {
            self.add_cycles(canister_id, spec.cycles)
                .map_err(|source| CanisterInstallError::AddCycles {
                    canister_id,
                    label: spec.label.clone(),
                    source,
                })?;
        }

        // Move the Wasm and init payload while retaining the label for failures.
        self.install_canister(canister_id, spec.wasm, spec.init_bytes, spec.install_sender)
            .map_err(|source| CanisterInstallError::InstallCode {
                canister_id,
                label: spec.label,
                source,
            })?;

        Ok(canister_id)
    }

    /// Wait out the `PocketIC` `install_code` cooldown window inside the same instance.
    fn wait_out_install_code_rate_limit(&self, cooldown: Duration) -> Result<(), ClientError> {
        super::time::advance_and_tick(self, cooldown, 2)
    }

    fn retry_install_code<T, F>(&self, policy: RetryPolicy, op: F) -> Result<T, ClientError>
    where
        F: FnMut() -> Result<T, ClientError>,
    {
        retry_install_code_with(policy, op, || {
            self.wait_out_install_code_rate_limit(policy.cooldown)
        })
    }
}

fn retry_install_code_with<T, F, W>(
    policy: RetryPolicy,
    mut op: F,
    mut wait_out_cooldown: W,
) -> Result<T, ClientError>
where
    F: FnMut() -> Result<T, ClientError>,
    W: FnMut() -> Result<(), ClientError>,
{
    let mut retries = 1..policy.max_attempts.get();
    loop {
        match op() {
            Err(ClientError::Canister(err))
                if err.error_code == ErrorCode::CanisterInstallCodeRateLimited
                    && retries.next().is_some() =>
            {
                wait_out_cooldown()?;
            }
            result => return result,
        }
    }
}

#[cfg(test)]
mod tests {
    use std::{
        cell::{Cell, RefCell},
        num::NonZeroUsize,
        time::Duration,
    };

    use crate::pocket_ic::{ErrorCode, RejectCode, RejectResponse};

    use super::{ClientError, RetryPolicy, retry_install_code_with};

    fn rejection(error_code: ErrorCode, message: &str) -> ClientError {
        ClientError::Canister(RejectResponse {
            reject_code: RejectCode::SysTransient,
            reject_message: message.to_string(),
            error_code,
            certified: false,
        })
    }

    #[test]
    fn retry_policy_counts_the_first_attempt() {
        for max_attempts in [1, 3] {
            let attempts = Cell::new(0);
            let waits = Cell::new(0);
            let result = retry_install_code_with(
                RetryPolicy {
                    max_attempts: NonZeroUsize::new(max_attempts).unwrap(),
                    cooldown: Duration::from_secs(1),
                },
                || {
                    attempts.set(attempts.get() + 1);
                    Err::<(), _>(rejection(
                        ErrorCode::CanisterInstallCodeRateLimited,
                        &format!("rate limit on attempt {}", attempts.get()),
                    ))
                },
                || {
                    waits.set(waits.get() + 1);
                    Ok(())
                },
            );

            assert_eq!(
                result.unwrap_err().to_string(),
                rejection(
                    ErrorCode::CanisterInstallCodeRateLimited,
                    &format!("rate limit on attempt {max_attempts}"),
                )
                .to_string()
            );
            assert_eq!(attempts.get(), max_attempts);
            assert_eq!(waits.get(), max_attempts - 1);
        }
    }

    #[test]
    fn retry_policy_waits_only_between_retryable_attempts() {
        for success_attempt in [1, 3] {
            let events = RefCell::new(Vec::new());
            let attempts = Cell::new(0);
            let result = retry_install_code_with(
                RetryPolicy {
                    max_attempts: NonZeroUsize::new(3).unwrap(),
                    cooldown: Duration::from_secs(1),
                },
                || {
                    events.borrow_mut().push("attempt");
                    attempts.set(attempts.get() + 1);
                    if attempts.get() == success_attempt {
                        Ok(42)
                    } else {
                        Err(rejection(
                            ErrorCode::CanisterInstallCodeRateLimited,
                            "rate limit",
                        ))
                    }
                },
                || {
                    events.borrow_mut().push("wait");
                    Ok(())
                },
            );
            assert_eq!(result.unwrap(), 42);
            assert_eq!(attempts.get(), success_attempt);
            assert_eq!(
                events.into_inner(),
                if success_attempt == 1 {
                    vec!["attempt"]
                } else {
                    vec!["attempt", "wait", "attempt", "wait", "attempt"]
                }
            );
        }
    }

    #[test]
    fn retry_policy_stops_on_non_rate_limit_failure() {
        let attempts = Cell::new(0);
        let not_retryable = rejection(ErrorCode::CanisterRejectedMessage, "not retryable");
        let result = retry_install_code_with(
            RetryPolicy {
                max_attempts: NonZeroUsize::new(3).unwrap(),
                cooldown: Duration::from_secs(1),
            },
            || {
                attempts.set(attempts.get() + 1);
                Err::<(), _>(rejection(
                    ErrorCode::CanisterRejectedMessage,
                    "not retryable",
                ))
            },
            || panic!("non-rate-limit failure must not wait"),
        );

        assert_eq!(result.unwrap_err().to_string(), not_retryable.to_string());
        assert_eq!(attempts.get(), 1);
    }

    #[test]
    fn client_and_cooldown_failures_stop_without_replaying_installation() {
        for fail_during_cooldown in [false, true] {
            let attempts = Cell::new(0);
            let error = retry_install_code_with(
                RetryPolicy {
                    max_attempts: NonZeroUsize::new(3).unwrap(),
                    cooldown: Duration::from_secs(1),
                },
                || {
                    attempts.set(attempts.get() + 1);
                    Err::<(), _>(if fail_during_cooldown {
                        rejection(ErrorCode::CanisterInstallCodeRateLimited, "rate limit")
                    } else {
                        ClientError::Protocol("uncertain operation".into())
                    })
                },
                || Err(ClientError::Protocol("cooldown failed".into())),
            )
            .unwrap_err();
            assert_eq!(attempts.get(), 1);
            assert!(
                matches!(error, ClientError::Protocol(message) if message == if fail_during_cooldown { "cooldown failed" } else { "uncertain operation" })
            );
        }
    }
}
