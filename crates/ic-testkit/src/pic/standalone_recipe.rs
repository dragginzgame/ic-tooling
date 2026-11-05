use std::io;

use ic_platform_types::principal::ExternalPrincipal as Principal;

use super::{
    CachedPocketIcBaseline, CanisterSnapshotTarget, FailureDisposition, PocketIcBaselineRecipe,
    PreparedBaseline, RebuildReason, ResetPolicies, ResetReceipt, ResetRequirements,
    SnapshotRestoreFunding, StandaloneCanisterFixture, is_dead_pocket_ic_transport_error,
};

/// Snapshot-only recipe for one caller-built, installed canister.
///
/// Use with [`super::CachedPocketIcBaselinePool`]. The baseline metadata is the
/// installed canister ID; calls go through the baseline's borrowed `PocketIC`.
/// The builder owns initialization and readiness before capture and must produce
/// the same topology and seeded state for every slot of this recipe. It returns
/// `io::Result<StandaloneCanisterFixture>`; build errors retain their original
/// cause and are reported at the pool's build stage, without invoking capture.
///
/// Only the captured canister is restored. Time, other canisters, pending work
/// and external resources are not reset. Use a custom recipe or a fresh instance
/// when those mutations matter. Funding is applied immediately before restore.
/// This recipe stops/drains its single canister before capture and warm restore,
/// then starts it at readiness. Automatic progress must remain disabled.
/// Non-transport restore failures are fatal: the pool discards the affected
/// baseline before returning the error. Dead transport permits one recovery build.
pub struct StandaloneCanisterRecipe<B> {
    requirements: ResetRequirements,
    build: B,
}

impl<B> StandaloneCanisterRecipe<B> {
    /// Describe a single-canister recipe without invoking its builder.
    ///
    /// Static suites can initialize the recipe and pool with [`std::sync::LazyLock`].
    #[must_use]
    pub fn new(funding: SnapshotRestoreFunding, build: B) -> Self {
        Self {
            requirements: ResetRequirements {
                funding,
                policies: ResetPolicies::default(),
            },
            build,
        }
    }
}

impl<B, Fut> PocketIcBaselineRecipe for StandaloneCanisterRecipe<B>
where
    B: Fn() -> Fut + Send + Sync + 'static,
    Fut: std::future::Future<Output = io::Result<StandaloneCanisterFixture>> + Send,
{
    type Metadata = Principal;
    // Retain the typed snapshot/contract cause without another error vocabulary.
    type Error = io::Error;

    fn reset_requirements(&self) -> &ResetRequirements {
        &self.requirements
    }

    async fn build(&self) -> Result<CachedPocketIcBaseline<Principal>, io::Error> {
        let (pocket_ic, canister_id) = (self.build)().await?.into_parts();
        if let Err(source) = pocket_ic.instance().stop_canister(canister_id, None).await {
            let error = io::Error::other(super::ControllerSnapshotError::CaptureFailed {
                canister_id,
                sender: None,
                source: Box::new(source),
                cleanup_failures: Vec::new(),
            });
            if let Err(cleanup) = pocket_ic.close().await {
                tracing::warn!("PocketIC deletion after stop failure failed: {cleanup}");
            }
            return Err(error);
        }
        let baseline = CachedPocketIcBaseline::capture(
            pocket_ic,
            [CanisterSnapshotTarget {
                canister_id,
                sender: None,
            }],
            canister_id,
        )
        .await
        .map_err(io::Error::other)?;
        if let Err(error) = self.drive_to_readiness(&baseline).await {
            if let Err(cleanup) = baseline.close().await {
                tracing::warn!("PocketIC deletion after readiness failure failed: {cleanup}");
            }
            return Err(error);
        }
        Ok(baseline)
    }

    fn reset_non_snapshot_state(
        &self,
        _baseline: &CachedPocketIcBaseline<Principal>,
    ) -> impl std::future::Future<Output = Result<ResetReceipt, io::Error>> + Send {
        std::future::ready(Ok(ResetReceipt::default()))
    }

    async fn prepare_restore(
        &self,
        baseline: &CachedPocketIcBaseline<Principal>,
    ) -> Result<(), io::Error> {
        baseline
            .pocket_ic()
            .stop_canister(*baseline.metadata(), None)
            .await
            .map_err(io::Error::other)
    }

    async fn drive_to_readiness(
        &self,
        baseline: &CachedPocketIcBaseline<Principal>,
    ) -> Result<(), io::Error> {
        baseline
            .pocket_ic()
            .start_canister(*baseline.metadata(), None)
            .await
            .map_err(io::Error::other)
    }

    fn validate(
        &self,
        _baseline: &CachedPocketIcBaseline<Principal>,
        _preparation: &PreparedBaseline,
    ) -> impl std::future::Future<Output = Result<(), io::Error>> + Send {
        std::future::ready(Ok(()))
    }

    fn classify_failure(
        &self,
        _stage: super::BaselinePreparationStage,
        error: &(dyn std::error::Error + 'static),
    ) -> FailureDisposition {
        if is_dead_pocket_ic_transport_error(error) {
            FailureDisposition::Rebuild(RebuildReason::DeadPocketIcTransport)
        } else {
            FailureDisposition::Fatal
        }
    }
}

#[cfg(test)]
mod tests;
