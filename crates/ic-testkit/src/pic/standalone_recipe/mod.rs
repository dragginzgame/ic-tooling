use std::io;

use pocket_ic::CanisterId as Principal;

use super::{
    CachedPocketIcBaseline, CanisterSnapshotTarget, FailureDisposition, PocketIcBaselineRecipe,
    PreparedBaseline, SnapshotRestoreFunding, StandaloneCanisterFixture,
};

/// A standalone recipe's stop or start rejection, retained inside its `io::Error`.
/// Upstream transport panics propagate without being converted into this error.
#[derive(Debug, thiserror::Error)]
pub enum StandaloneCanisterLifecycleError {
    /// Stopping the canister before capture or restoration was rejected.
    #[error("failed to stop standalone canister {canister_id}: {rejection}")]
    Stop {
        /// Target canister.
        canister_id: Principal,
        /// Complete original management rejection.
        rejection: Box<pocket_ic::RejectResponse>,
    },
    /// Starting the canister at readiness was rejected.
    #[error("failed to start standalone canister {canister_id}: {rejection}")]
    Start {
        /// Target canister.
        canister_id: Principal,
        /// Complete original management rejection.
        rejection: Box<pocket_ic::RejectResponse>,
    },
}

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
/// baseline before returning the error. Upstream transport panics propagate;
/// an interrupted preparation must rebuild before the next lease is exposed.
pub struct StandaloneCanisterRecipe<B> {
    funding: SnapshotRestoreFunding,
    build: B,
}

impl<B> StandaloneCanisterRecipe<B> {
    /// Describe a single-canister recipe without invoking its builder.
    ///
    /// Static suites can initialize the recipe and pool with [`std::sync::LazyLock`].
    #[must_use]
    pub const fn new(funding: SnapshotRestoreFunding, build: B) -> Self {
        Self { funding, build }
    }
}

impl<B, Fut> PocketIcBaselineRecipe for StandaloneCanisterRecipe<B>
where
    B: Fn() -> Fut + Send + Sync + 'static,
    Fut: std::future::Future<Output = io::Result<StandaloneCanisterFixture>> + Send,
{
    type Metadata = Principal;
    // Preserve builder errors and typed snapshot/lifecycle causes.
    type Error = io::Error;

    fn snapshot_funding(&self) -> SnapshotRestoreFunding {
        self.funding
    }

    async fn build(&self) -> Result<CachedPocketIcBaseline<Principal>, io::Error> {
        let (pocket_ic, canister_id) = (self.build)().await?.into_parts();
        if let Err(source) = pocket_ic.instance().stop_canister(canister_id, None).await {
            let error = io::Error::other(StandaloneCanisterLifecycleError::Stop {
                canister_id,
                rejection: Box::new(source),
            });
            pocket_ic.close().await;
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
            baseline.close().await;
            return Err(error);
        }
        Ok(baseline)
    }

    fn reset_non_snapshot_state(
        &self,
        _baseline: &CachedPocketIcBaseline<Principal>,
    ) -> impl std::future::Future<Output = Result<(), io::Error>> + Send {
        std::future::ready(Ok(()))
    }

    async fn prepare_restore(
        &self,
        baseline: &CachedPocketIcBaseline<Principal>,
    ) -> Result<(), io::Error> {
        baseline
            .pocket_ic()
            .stop_canister(*baseline.metadata(), None)
            .await
            .map_err(|rejection| {
                io::Error::other(StandaloneCanisterLifecycleError::Stop {
                    canister_id: *baseline.metadata(),
                    rejection: Box::new(rejection),
                })
            })
    }

    async fn drive_to_readiness(
        &self,
        baseline: &CachedPocketIcBaseline<Principal>,
    ) -> Result<(), io::Error> {
        baseline
            .pocket_ic()
            .start_canister(*baseline.metadata(), None)
            .await
            .map_err(|rejection| {
                io::Error::other(StandaloneCanisterLifecycleError::Start {
                    canister_id: *baseline.metadata(),
                    rejection: Box::new(rejection),
                })
            })
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
        _error: &(dyn std::error::Error + 'static),
    ) -> FailureDisposition {
        FailureDisposition::Fatal
    }
}

#[cfg(test)]
mod tests;
