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
