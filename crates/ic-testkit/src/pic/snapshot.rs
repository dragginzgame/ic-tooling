//! Ordered controller snapshots using upstream management operations.
use pocket_ic::{CanisterId as Principal, RejectResponse, nonblocking::PocketIc};
use std::collections::BTreeMap;

#[derive(Clone, Debug, Eq, PartialEq)]
struct ControllerSnapshot {
    snapshot_id: Vec<u8>,
    sender: Option<Principal>,
}

/// Deterministically ordered snapshots and the exact senders that captured them.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ControllerSnapshots(BTreeMap<Principal, ControllerSnapshot>);

/// One canister and caller-selected management sender.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct CanisterSnapshotTarget {
    /// Target canister.
    pub canister_id: Principal,
    /// Management sender; None selects upstream's default.
    pub sender: Option<Principal>,
}

/// A rejection while rolling back a known snapshot.
#[derive(Debug)]
pub struct SnapshotCleanupFailure {
    /// Target canister.
    pub canister_id: Principal,
    /// Exact capture sender.
    pub sender: Option<Principal>,
    /// Known snapshot id selected for deletion.
    pub snapshot_id: Vec<u8>,
    /// Original management rejection.
    pub rejection: Box<RejectResponse>,
}

/// Caller-selected cycle funding immediately before restoration.
#[non_exhaustive]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum SnapshotRestoreFunding {
    /// Never add cycles.
    Preserve,
    /// Add enough cycles to establish the selected minimum balance.
    TopUpTo {
        /// Required minimum before restore costs.
        minimum_cycles: u128,
    },
}

/// Snapshot admission or management rejection. Upstream transport panics propagate.
#[derive(Debug, thiserror::Error)]
pub enum ControllerSnapshotError {
    /// Baselines need a nonempty snapshot set.
    #[error("a cached baseline requires at least one canister")]
    EmptyCanisterSet,
    /// Duplicate target refused before any calls.
    #[error("duplicate canister id in snapshot set: {canister_id}")]
    DuplicateCanisterId {
        /// Repeated canister.
        canister_id: Principal,
    },
    /// Capture failed after earlier captures may have completed.
    #[error("failed to capture snapshot for {canister_id} with sender {sender:?}: {rejection}; {} cleanup rejections", .cleanup_failures.len())]
    CaptureFailed {
        /// Failed target.
        canister_id: Principal,
        /// Exact selected sender.
        sender: Option<Principal>,
        /// Management rejection.
        rejection: Box<RejectResponse>,
        /// Rejections deleting earlier captured snapshots.
        cleanup_failures: Vec<SnapshotCleanupFailure>,
    },
    /// Restoration failed after earlier targets may have completed.
    #[error("failed to restore snapshot for {canister_id} with sender {sender:?}: {rejection}")]
    RestoreFailed {
        /// Failed target.
        canister_id: Principal,
        /// Retained capture sender.
        sender: Option<Principal>,
        /// Original rejection.
        rejection: Box<RejectResponse>,
    },
}

/// Capture in canister order, rolling back known captures after a rejection.
///
/// Callers stop/drain targets and disable automatic progress. This uses ordinary
/// upstream take/load semantics; it does not copy snapshot data, reset simulated
/// time or promise the retired transfer's timer/low-memory-hook restoration.
/// Cancellation and transport panics can leave partial captures for reconciliation.
///
/// # Errors
/// Refuses duplicates before effects; returns a capture rejection with rollback failures.
pub async fn capture_snapshots(
    pic: &PocketIc,
    targets: impl IntoIterator<Item = CanisterSnapshotTarget>,
) -> Result<ControllerSnapshots, ControllerSnapshotError> {
    let mut ordered = BTreeMap::new();
    for target in targets {
        if ordered.insert(target.canister_id, target.sender).is_some() {
            return Err(ControllerSnapshotError::DuplicateCanisterId {
                canister_id: target.canister_id,
            });
        }
    }
    let mut snapshots = BTreeMap::new();
    for (canister_id, sender) in ordered {
        match pic.take_canister_snapshot(canister_id, sender, None).await {
            Ok(snapshot) => {
                snapshots.insert(
                    canister_id,
                    ControllerSnapshot {
                        snapshot_id: snapshot.id,
                        sender,
                    },
                );
            }
            Err(source) => {
                let cleanup_failures =
                    cleanup_captured_snapshots(snapshots, |id, sender, snapshot| {
                        pic.delete_canister_snapshot(id, sender, snapshot)
                    })
                    .await;
                return Err(ControllerSnapshotError::CaptureFailed {
                    canister_id,
                    sender,
                    rejection: Box::new(source),
                    cleanup_failures,
                });
            }
        }
    }
    Ok(ControllerSnapshots(snapshots))
}

/// Restore in deterministic order using captured senders and selected funding.
/// Callers own stopping, draining, restarting, and non-snapshot state reset.
/// # Errors
/// Returns the first management rejection without replaying earlier restores.
pub async fn restore_snapshots(
    pic: &PocketIc,
    snapshots: &ControllerSnapshots,
    funding: SnapshotRestoreFunding,
) -> Result<(), ControllerSnapshotError> {
    for (id, snapshot) in &snapshots.0 {
        if let SnapshotRestoreFunding::TopUpTo { minimum_cycles } = funding {
            let balance = pic.cycle_balance(*id).await;
            if balance < minimum_cycles {
                pic.add_cycles(*id, minimum_cycles - balance).await;
            }
        }
        pic.load_canister_snapshot(*id, snapshot.sender, snapshot.snapshot_id.clone())
            .await
            .map_err(|source| ControllerSnapshotError::RestoreFailed {
                canister_id: *id,
                sender: snapshot.sender,
                rejection: Box::new(source),
            })?;
    }
    Ok(())
}

impl ControllerSnapshots {
    /// Number of captured canisters.
    #[must_use]
    pub fn len(&self) -> usize {
        self.0.len()
    }
    /// Whether no canisters were captured.
    #[must_use]
    pub fn is_empty(&self) -> bool {
        self.0.is_empty()
    }
    /// Captured ids in deterministic order.
    pub fn canister_ids(&self) -> impl Iterator<Item = Principal> + '_ {
        self.0.keys().copied()
    }
}

async fn cleanup_captured_snapshots<F, Fut>(
    snapshots: BTreeMap<Principal, ControllerSnapshot>,
    mut remove: F,
) -> Vec<SnapshotCleanupFailure>
where
    F: FnMut(Principal, Option<Principal>, Vec<u8>) -> Fut,
    Fut: std::future::Future<Output = Result<(), RejectResponse>>,
{
    let mut failures = Vec::new();
    for (canister_id, snapshot) in snapshots {
        if let Err(source) =
            remove(canister_id, snapshot.sender, snapshot.snapshot_id.clone()).await
        {
            failures.push(SnapshotCleanupFailure {
                canister_id,
                sender: snapshot.sender,
                snapshot_id: snapshot.snapshot_id,
                rejection: Box::new(source),
            });
        }
    }
    failures
}
#[cfg(test)]
mod tests;
