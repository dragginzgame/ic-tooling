use super::{
    CanisterSnapshotTarget, ControllerSnapshotError, ControllerSnapshots, PocketIcInstance,
    SnapshotRestoreFunding,
};
use pocket_ic::CanisterId as Principal;
use pocket_ic::nonblocking::PocketIc;

/// One owned `PocketIC` instance/server record with snapshots and caller metadata.
///
/// The caller owns capture, stop/drain, restoration and instance lifetime.
/// This value supplies no pooling, reset recipe or automatic rebuild.
pub struct CachedPocketIcBaseline<T> {
    instance: PocketIcInstance,
    snapshots: ControllerSnapshots,
    metadata: T,
}

impl<T> CachedPocketIcBaseline<T> {
    /// Capture a cached baseline with an explicit sender for every canister.
    ///
    /// Empty inputs are rejected before capture. Capture is ordered and transactional as documented by
    /// [`super::capture_snapshots`]. No sender fallback is performed.
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
            super::capture_snapshots(instance.instance(), targets).await
        };
        let snapshots = match capture {
            Ok(snapshots) => snapshots,
            Err(error) => {
                instance.close().await;
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
    /// Loads the prepared snapshot's heap, global timer and low-memory-hook state.
    /// Current time and non-snapshot state remain caller-owned.
    ///
    /// # Errors
    /// Returns a controller snapshot error if funding or snapshot restoration fails.
    pub async fn restore(
        &self,
        funding: SnapshotRestoreFunding,
    ) -> Result<(), ControllerSnapshotError> {
        super::restore_snapshots(self.instance.instance(), &self.snapshots, funding).await
    }

    /// Explicitly delete the instance before releasing its managed server.
    /// Cancellation on a borrowed server requires caller reconciliation.
    /// Upstream deletion is best effort and does not report its failure.
    pub async fn close(self) {
        self.instance.close().await;
    }

    /// Borrow the owned `PocketIC` instance behind this cached baseline.
    #[must_use]
    pub const fn pocket_ic(&self) -> &PocketIc {
        self.instance.instance()
    }

    /// Return the number of canisters captured by this baseline.
    #[must_use]
    pub const fn snapshot_count(&self) -> usize {
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
