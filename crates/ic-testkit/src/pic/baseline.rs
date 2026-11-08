use super::{
    CanisterSnapshotTarget, ControllerSnapshotError, ControllerSnapshots, PocketIcInstance,
    SnapshotRestoreFunding,
};
use crate::pocket_ic::PocketIc;
use ic_platform_types::principal::ExternalPrincipal as Principal;

/// One owned `PocketIC` instance/server record with snapshots and caller metadata.
///
/// The value contains no synchronization. A [`super::PocketIcBaselineRecipe`]
/// can own it as its fixture in a [`super::CachedPocketIcBaselinePool`].
pub struct CachedPocketIcBaseline<T> {
    instance: PocketIcInstance,
    snapshots: ControllerSnapshots,
    metadata: T,
}

impl<T> CachedPocketIcBaseline<T> {
    /// Capture a cached baseline with an explicit sender for every canister.
    ///
    /// Empty inputs are rejected before capture. Capture is ordered and transactional as documented by
    /// [`crate::pocket_ic::PocketIc::capture_snapshots`]. No sender fallback is performed.
    /// The caller stops/drains every target before capture and restarts only
    /// after the entire capture succeeds. Automatic progress must be disabled.
    ///
    /// # Errors
    /// Returns a controller snapshot error if target admission or snapshot capture fails.
    pub async fn capture<I>(
        instance: PocketIcInstance,
        targets: I,
        metadata: T,
    ) -> Result<Self, ControllerSnapshotError>
    where
        I: IntoIterator<Item = CanisterSnapshotTarget>,
    {
        let mut targets = targets.into_iter().peekable();
        let capture = if targets.peek().is_none() {
            Err(ControllerSnapshotError::EmptyCanisterSet)
        } else {
            instance.instance().capture_snapshots(targets).await
        };
        let snapshots = match capture {
            Ok(snapshots) => snapshots,
            Err(error) => {
                if let Err(cleanup) = instance.close().await {
                    tracing::warn!("PocketIC deletion after capture failure failed: {cleanup}");
                }
                return Err(error);
            }
        };
        Ok(Self {
            instance,
            snapshots,
            metadata,
        })
    }

    /// Restore with exactly the captured senders and selected cycle funding.
    /// Success means the complete captured set was restored.
    /// Stop/drain the entire set first; restart after complete restore/reset.
    /// Absolute captured timers and hook state are restored; current time is not.
    ///
    /// # Errors
    /// Returns a controller snapshot error if funding or snapshot restoration fails.
    #[expect(
        clippy::future_not_send,
        reason = "baseline metadata may be local; pool recipes separately require Sync metadata"
    )]
    pub async fn restore(
        &self,
        funding: SnapshotRestoreFunding,
    ) -> Result<(), ControllerSnapshotError> {
        self.instance
            .instance()
            .restore_snapshots(&self.snapshots, funding)
            .await
    }

    /// Explicitly delete the instance before releasing its managed server.
    /// Cancellation on a borrowed server requires caller reconciliation.
    /// # Errors
    /// Returns the client deletion failure, without retrying it.
    pub async fn close(self) -> Result<(), crate::pocket_ic::ClientError> {
        self.instance.close().await
    }

    /// Borrow the owned `PocketIC` instance behind this cached baseline.
    #[must_use]
    pub const fn pocket_ic(&self) -> &PocketIc {
        self.instance.instance()
    }

    /// Return the number of canisters captured by this baseline.
    #[must_use]
    pub fn snapshot_count(&self) -> usize {
        self.snapshots.len()
    }

    /// Iterate over captured canister ids in deterministic principal order.
    pub fn snapshot_canister_ids(&self) -> impl Iterator<Item = Principal> + '_ {
        self.snapshots.canister_ids()
    }

    /// Borrow the captured metadata associated with this cached baseline.
    #[must_use]
    pub const fn metadata(&self) -> &T {
        &self.metadata
    }

    /// Mutably borrow the captured metadata associated with this cached baseline.
    #[must_use]
    pub const fn metadata_mut(&mut self) -> &mut T {
        &mut self.metadata
    }
}
