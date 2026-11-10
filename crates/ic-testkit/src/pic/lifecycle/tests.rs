use super::*;
use pocket_ic::RejectCode;
use std::cell::Cell;
fn rejection(error_code: ErrorCode) -> RejectResponse {
    RejectResponse {
        reject_code: RejectCode::SysTransient,
        reject_message: "rejected".into(),
        error_code,
        certified: false,
    }
}
#[tokio::test]
async fn retry_budget_includes_first_attempt_and_waits_only_between_attempts() {
    for maximum in [1, 3] {
        let attempts = Cell::new(0);
        let waits = Cell::new(0);
        let result = retry_install_code_with(
            RetryPolicy {
                max_attempts: NonZeroUsize::new(maximum).unwrap(),
                cooldown: Duration::from_secs(1),
            },
            async || {
                attempts.set(attempts.get() + 1);
                Err::<(), _>(rejection(ErrorCode::CanisterInstallCodeRateLimited))
            },
            async || waits.set(waits.get() + 1),
        )
        .await;
        assert_eq!(
            result.unwrap_err().error_code,
            ErrorCode::CanisterInstallCodeRateLimited
        );
        assert_eq!(attempts.get(), maximum);
        assert_eq!(waits.get(), maximum - 1);
    }
}
#[tokio::test]
async fn non_rate_limit_rejection_is_not_retried() {
    let result = retry_install_code_with(
        RetryPolicy {
            max_attempts: NonZeroUsize::new(3).unwrap(),
            cooldown: Duration::ZERO,
        },
        async || Err::<(), _>(rejection(ErrorCode::CanisterRejectedMessage)),
        async || panic!("must not retry"),
    )
    .await;
    assert_eq!(
        result.unwrap_err().error_code,
        ErrorCode::CanisterRejectedMessage
    );
}
#[tokio::test]
async fn successful_retry_returns_original_value() {
    let attempts = Cell::new(0);
    let result = retry_install_code_with(
        RetryPolicy {
            max_attempts: NonZeroUsize::new(3).unwrap(),
            cooldown: Duration::ZERO,
        },
        async || {
            attempts.set(attempts.get() + 1);
            if attempts.get() == 1 {
                Err(rejection(ErrorCode::CanisterInstallCodeRateLimited))
            } else {
                Ok(42)
            }
        },
        async || assert_eq!(attempts.get(), 1),
    )
    .await;
    assert_eq!(result.unwrap(), 42);
    assert_eq!(attempts.get(), 2);
}
