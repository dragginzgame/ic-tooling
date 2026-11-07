use super::{BaselinePoolTimings, add_timing};
use std::time::Duration;

#[test]
fn preparation_and_recovery_keep_display_and_unboxed_primary_sources() {
    use std::{error::Error as _, io};
    let preparation = |message| super::BaselinePoolPreparationError::Recipe {
        stage: super::BaselinePreparationStage::Build,
        source: io::Error::other(message),
    };
    let original = preparation("original");
    assert_eq!(original.to_string(), "baseline build failed: original");
    assert!(original.source().unwrap().is::<io::Error>());
    let error = super::BaselinePoolError::RecoveryFailed {
        original: Box::new(original),
        rebuild: Box::new(preparation("replacement")),
        timings: Box::default(),
    };
    assert_eq!(
        error.to_string(),
        "baseline preparation failed (baseline build failed: original); rebuilding the slot also failed: baseline build failed: replacement"
    );
    assert!(
        error
            .source()
            .unwrap()
            .is::<super::BaselinePoolPreparationError<io::Error>>()
    );
    let closed: super::BaselinePoolError<io::Error> = super::BaselinePoolError::Closed {
        timings: Box::new(BaselinePoolTimings {
            wait: Duration::from_millis(2),
            total: Duration::from_millis(2),
            ..BaselinePoolTimings::default()
        }),
    };
    assert_eq!(closed.to_string(), "baseline pool is shut down");
    assert!(closed.source().is_none());
    assert_eq!(closed.timings().wait, Duration::from_millis(2));
}

#[tokio::test]
async fn phase_timing_preserves_absence_zero_and_saturation() {
    let mut timings = BaselinePoolTimings::default();
    assert_eq!(timings.restore, None);
    add_timing(&mut timings.restore, Duration::ZERO);
    assert_eq!(timings.restore, Some(Duration::ZERO));
    add_timing(&mut timings.restore, Duration::from_secs(2));
    assert_eq!(timings.restore, Some(Duration::from_secs(2)));
    add_timing(&mut timings.restore, Duration::MAX);
    assert_eq!(timings.restore, Some(Duration::MAX));
    assert_eq!(timings.build, None);
}
