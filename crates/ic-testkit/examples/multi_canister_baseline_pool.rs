//! Complete two-canister recipe for `CachedPocketIcBaselinePool`.
//!
//! Run with:
//!
//! ```text
//! cargo run -p ic-testkit --example multi_canister_baseline_pool
//! ```
use ic_testkit::pocket_ic::ClientError;

use std::{error::Error, num::NonZeroUsize, time::Duration};

use ic_platform::principal::ExternalPrincipal as Principal;
use ic_testkit::pic::{
    BaselinePoolDisposition, BaselinePreparationStage, CachedPocketIcBaseline,
    CachedPocketIcBaselinePool, CanisterSnapshotTarget, ControllerSnapshotError,
    FailureDisposition, PocketIcBaselineRecipe, PocketIcStartupConfig, PocketIcStartupError,
    PreparedBaseline, RebuildReason, ResetPolicies, ResetReceipt, ResetRequirements,
    SnapshotRestoreFunding, is_dead_pocket_ic_transport_error,
};
use ic_testkit::pocket_ic::PocketIcBuilder;

const EMPTY_WASM: &[u8] = b"\0asm\x01\0\0\0";

struct TwoCanisterRecipe {
    requirements: ResetRequirements,
}

struct TopologyMetadata {
    canister_ids: [Principal; 2],
}

async fn lifecycle(
    call: impl AsyncFnOnce() -> Result<(), ic_testkit::pocket_ic::ClientError>,
) -> Result<(), RecipeError> {
    call().await.map_err(RecipeError::Lifecycle)
}

#[derive(Debug, thiserror::Error)]
enum RecipeError {
    #[error("{0}")]
    Lifecycle(#[source] ClientError),
    #[error("{0}")]
    Startup(#[source] PocketIcStartupError),
    #[error("{0}")]
    Snapshot(#[source] ControllerSnapshotError),
    #[error("{0}")]
    Validation(&'static str),
}

impl TwoCanisterRecipe {
    fn new() -> Self {
        Self {
            requirements: ResetRequirements {
                funding: SnapshotRestoreFunding::Preserve,
                policies: ResetPolicies::default(),
            },
        }
    }
}

impl PocketIcBaselineRecipe for TwoCanisterRecipe {
    type Metadata = TopologyMetadata;
    type Error = RecipeError;

    fn reset_requirements(&self) -> &ResetRequirements {
        &self.requirements
    }

    async fn build(&self) -> Result<CachedPocketIcBaseline<Self::Metadata>, Self::Error> {
        let pocket_ic = PocketIcStartupConfig::from_env(Duration::from_secs(30))?
            .build_instance(PocketIcBuilder::new().with_application_subnet())
            .await?;
        let canister_ids = [
            pocket_ic.instance().create_canister().await.unwrap(),
            pocket_ic.instance().create_canister().await.unwrap(),
        ];
        for canister_id in canister_ids {
            pocket_ic
                .instance()
                .install_canister(canister_id, EMPTY_WASM.to_vec(), vec![], None)
                .await
                .unwrap();
            lifecycle(async || pocket_ic.instance().stop_canister(canister_id, None).await).await?;
        }

        let baseline = CachedPocketIcBaseline::capture(
            pocket_ic,
            canister_ids.map(|id| CanisterSnapshotTarget {
                canister_id: id,
                sender: None,
            }),
            TopologyMetadata { canister_ids },
        )
        .await
        .map_err(RecipeError::Snapshot)?;
        self.drive_to_readiness(&baseline).await?;
        Ok(baseline)
    }

    async fn prepare_restore(
        &self,
        baseline: &CachedPocketIcBaseline<Self::Metadata>,
    ) -> Result<(), Self::Error> {
        for id in baseline.metadata().canister_ids {
            lifecycle(async || baseline.pocket_ic().stop_canister(id, None).await).await?;
        }
        Ok(())
    }

    fn reset_non_snapshot_state(
        &self,
        _baseline: &CachedPocketIcBaseline<Self::Metadata>,
    ) -> impl std::future::Future<Output = Result<ResetReceipt, Self::Error>> + Send {
        // This recipe is suitable only for tests whose relevant mutations are
        // fully contained in the two captured canisters.
        std::future::ready(Ok(ResetReceipt::default()))
    }

    async fn drive_to_readiness(
        &self,
        baseline: &CachedPocketIcBaseline<Self::Metadata>,
    ) -> Result<(), Self::Error> {
        for id in baseline.metadata().canister_ids {
            lifecycle(async || baseline.pocket_ic().start_canister(id, None).await).await?;
        }
        Ok(())
    }

    async fn validate(
        &self,
        baseline: &CachedPocketIcBaseline<Self::Metadata>,
        _preparation: &PreparedBaseline,
    ) -> Result<(), Self::Error> {
        for canister_id in baseline.metadata().canister_ids {
            let status = baseline
                .pocket_ic()
                .canister_status(canister_id, None)
                .await
                .map_err(|_| RecipeError::Validation("canister status failed"))?;
            if status.module_hash.is_none() {
                return Err(RecipeError::Validation(
                    "captured canister has no installed module",
                ));
            }
        }

        Ok(())
    }

    fn classify_failure(
        &self,
        stage: BaselinePreparationStage,
        error: &(dyn std::error::Error + 'static),
    ) -> FailureDisposition {
        if is_dead_pocket_ic_transport_error(error) {
            FailureDisposition::Rebuild(RebuildReason::DeadPocketIcTransport)
        } else {
            FailureDisposition::Rebuild(stage.default_rebuild_reason())
        }
    }
}

impl From<PocketIcStartupError> for RecipeError {
    fn from(error: PocketIcStartupError) -> Self {
        Self::Startup(error)
    }
}

impl From<ControllerSnapshotError> for RecipeError {
    fn from(error: ControllerSnapshotError) -> Self {
        Self::Snapshot(error)
    }
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let pool = CachedPocketIcBaselinePool::new(
        NonZeroUsize::new(1).expect("one is nonzero"),
        TwoCanisterRecipe::new(),
    );

    let (baseline, first) = pool.acquire().await?;
    assert!(matches!(first.disposition, BaselinePoolDisposition::Built));
    println!("first acquisition: {first:?}");
    drop(baseline);

    let (baseline, second) = pool.acquire().await?;
    assert!(matches!(
        second.disposition,
        BaselinePoolDisposition::Restored
    ));
    println!("second acquisition: {second:?}");
    drop(baseline);
    let failures = pool.close().await;
    for error in &failures {
        eprintln!("instance deletion failed: {error}");
    }
    if let Some(error) = failures.into_iter().next() {
        return Err(error.into());
    }
    Ok(())
}
