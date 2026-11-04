use std::num::NonZeroUsize;

use super::*;
use crate::pic::CachedPocketIcBaselinePool;

fn build_fixture() -> io::Result<StandaloneCanisterFixture> {
    panic!("constructing an empty pool must not invoke its builder")
}

#[test]
fn runtime_capacity_preserves_lazy_fixture_construction() {
    for capacity in [1, 2, 4, 8].map(|value| NonZeroUsize::new(value).unwrap()) {
        let recipe = StandaloneCanisterRecipe::new(
            SnapshotRestoreFunding::TopUpTo { minimum_cycles: 1 },
            build_fixture,
        );
        let pool = CachedPocketIcBaselinePool::new(capacity, recipe);
        assert_eq!(pool.capacity(), capacity);
    }
}

#[test]
fn builder_errors_preserve_the_cause_and_release_the_slot_for_another_attempt() {
    let recipe = StandaloneCanisterRecipe::new(SnapshotRestoreFunding::Preserve, || {
        Err(io::Error::new(
            io::ErrorKind::PermissionDenied,
            std::fmt::Error,
        ))
    });
    let pool = CachedPocketIcBaselinePool::new(NonZeroUsize::new(1).unwrap(), recipe);
    for _ in 0..2 {
        let Err(super::super::BaselinePoolError::Preparation {
            error:
                super::super::BaselinePoolPreparationError::Recipe {
                    stage: super::super::BaselinePreparationStage::Build,
                    source,
                },
            timings,
        }) = pool.acquire()
        else {
            panic!("builder failure must return at the build stage");
        };
        assert_eq!(source.kind(), io::ErrorKind::PermissionDenied);
        assert!(source.get_ref().unwrap().is::<std::fmt::Error>());
        assert!(timings.build().is_some());
        assert!(timings.restore().is_none());
        assert!(timings.total() >= timings.build().unwrap());
    }
}
