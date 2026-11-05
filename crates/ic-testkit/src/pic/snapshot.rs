use std::collections::BTreeMap;

use crate::pocket_ic::{ClientError, PocketIc};
use ic_platform_types::principal::ExternalPrincipal as Principal;

mod transfer;

#[derive(Clone, Debug, Eq, PartialEq)]
struct ControllerSnapshot {
    snapshot_id: Vec<u8>,
    sender: Option<Principal>,
}

/// Deterministically ordered snapshots retaining each successful capture sender.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ControllerSnapshots(BTreeMap<Principal, ControllerSnapshot>);

/// One canister and exact management-call sender used for snapshot capture.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct CanisterSnapshotTarget {
    /// Canister whose snapshot will be captured.
    pub canister_id: Principal,
    /// Exact management-call sender; None selects `PocketIC`'s default.
    pub sender: Option<Principal>,
}

/// Failure to remove a snapshot while rolling back a partial capture.
#[non_exhaustive]
#[derive(Debug)]
pub struct SnapshotCleanupFailure {
    /// Canister whose snapshot could not be removed.
    pub canister_id: Principal,
    /// Exact sender selected for cleanup.
    pub sender: Option<Principal>,
    /// Exact known snapshot whose deletion failed; no other ID was substituted.
    pub snapshot_id: Vec<u8>,
    /// Complete rejection or operation failure, retaining transport classification.
    pub source: Box<ClientError>,
}

/// Caller-selected cycle funding applied immediately before snapshot restore.
#[non_exhaustive]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum SnapshotRestoreFunding {
    /// Do not add cycles before the restore operation.
    Preserve,
    /// Add cycles only when needed to reach the given minimum balance.
    TopUpTo {
        /// Minimum balance established immediately before each restore.
        minimum_cycles: u128,
    },
}

/// Structured controller-snapshot failure.
#[non_exhaustive]
#[derive(Debug, thiserror::Error)]
pub enum ControllerSnapshotError {
    /// A cached baseline requires at least one canister.
    #[error("a cached baseline requires at least one canister")]
    EmptyCanisterSet,
    /// Input contained the same canister more than once.
    #[error("duplicate canister id in snapshot set: {canister_id}")]
    DuplicateCanisterId {
        /// Repeated canister id.
        canister_id: Principal,
    },
    /// Snapshot transfer metadata or complete data admission failed.
    #[error("invalid snapshot for {canister_id}: {message}; {} partial snapshots could not be cleaned up", .cleanup_failures.len())]
    InvalidSnapshot {
        /// Exact canister selected for capture.
        canister_id: Principal,
        /// Refused snapshot property, excluding source bytes.
        message: &'static str,
        /// Known snapshot deletions that failed during rollback.
        cleanup_failures: Vec<SnapshotCleanupFailure>,
    },
    /// The external management snapshot message could not be encoded or decoded.
    #[error("snapshot codec failed for {canister_id}: {source}")]
    SnapshotCodec {
        /// Exact canister selected for capture.
        canister_id: Principal,
        /// Original external Candid codec failure.
        source: ic_platform::candid::contract::Error,
        /// Known snapshot deletions that failed during rollback.
        cleanup_failures: Vec<SnapshotCleanupFailure>,
    },
    /// Snapshot capture failed with the exact selected sender.
    #[error("failed to capture snapshot for {canister_id} with sender {sender:?}: {source}; {} partial snapshots could not be cleaned up", .cleanup_failures.len())]
    CaptureFailed {
        /// Canister whose capture failed.
        canister_id: Principal,
        /// Exact sender selected for capture.
        sender: Option<Principal>,
        /// Complete rejection or operation failure.
        source: Box<ClientError>,
        /// Failures while deleting snapshots captured earlier in the set.
        cleanup_failures: Vec<SnapshotCleanupFailure>,
    },
    /// Snapshot restore failed with the retained capture sender.
    #[error("failed to restore snapshot for {canister_id} with sender {sender:?}: {source}")]
    RestoreFailed {
        /// Canister whose restore failed.
        canister_id: Principal,
        /// Exact sender selected for restore.
        sender: Option<Principal>,
        /// Complete rejection or operation failure.
        source: Box<ClientError>,
    },
}

/// Exact-sender capture and restore of related canister snapshots.
#[expect(
    async_fn_in_trait,
    reason = "extension futures may retain caller-owned non-Send callbacks, iterators or writers; pool recipes separately require Send"
)]
pub trait PocketIcSnapshotExt {
    /// Capture snapshots with one explicit sender per canister.
    ///
    /// The caller must stop and drain the complete target set before capture,
    /// then restart it only after the complete set succeeds. This helper does
    /// not stop/start canisters or rewind simulated time. Disable automatic
    /// progress and exclude concurrent instance operations throughout the set.
    /// Input is duplicate-checked before any effects and captured in deterministic
    /// canister-id order. A later failure rolls back snapshots captured earlier
    /// in the set, including known temporary transfer snapshots. No other sender
    /// is attempted after a rejection. Upload semantics preserve the captured
    /// global timer and low-memory hook alongside the module, globals, heap,
    /// stable memory, certified data and chunk store. Transfer uses bounded
    /// management messages; total work follows the captured state size.
    ///
    /// Cancellation cannot await rollback. Use isolated server custody or reconcile
    /// any temporary snapshots explicitly when cancelling on a borrowed server.
    ///
    /// # Errors
    /// Returns a target-admission or snapshot-capture error, retaining completed work.
    async fn capture_snapshots<I>(
        &self,
        targets: I,
    ) -> Result<ControllerSnapshots, ControllerSnapshotError>
    where
        I: IntoIterator<Item = CanisterSnapshotTarget>;

    /// Restore each snapshot with its exact captured sender and selected funding.
    ///
    /// The caller must stop/drain the entire captured set before restoring and
    /// restart only after every restore and non-snapshot reset succeeds. Automatic
    /// progress and concurrent instance operations must remain disabled. Absolute
    /// timer deadlines are restored, not shifted; an expired timer may run when
    /// its canister restarts. Simulated time and lifecycle remain caller-owned.
    ///
    /// `Preserve` never adds cycles; `TopUpTo` is evaluated before each restore.
    /// `PocketIC` may charge cycles for the restore itself. Rejection or panic stops
    /// the set without retrying another sender or rewinding earlier restores.
    ///
    /// # Errors
    /// Returns a funding or restoration error, retaining partial progress.
    async fn restore_snapshots(
        &self,
        snapshots: &ControllerSnapshots,
        funding: SnapshotRestoreFunding,
    ) -> Result<(), ControllerSnapshotError>;
}

impl PocketIcSnapshotExt for PocketIc {
    async fn capture_snapshots<I>(
        &self,
        targets: I,
    ) -> Result<ControllerSnapshots, ControllerSnapshotError>
    where
        I: IntoIterator<Item = CanisterSnapshotTarget>,
    {
        capture_snapshot_set(self, targets).await
    }

    async fn restore_snapshots(
        &self,
        snapshots: &ControllerSnapshots,
        funding: SnapshotRestoreFunding,
    ) -> Result<(), ControllerSnapshotError> {
        for (canister_id, snapshot_id, sender) in snapshots.iter() {
            restore_controller_snapshot(self, canister_id, snapshot_id, funding, sender).await?;
        }
        Ok(())
    }
}

async fn capture_snapshot_set<I>(
    pocket_ic: &PocketIc,
    targets: I,
) -> Result<ControllerSnapshots, ControllerSnapshotError>
where
    I: IntoIterator<Item = CanisterSnapshotTarget>,
{
    // Validate the entire input before issuing any management calls. The map
    // also owns the deterministic canister order.
    let mut ordered_targets = BTreeMap::new();
    for target in targets {
        if ordered_targets
            .insert(target.canister_id, target.sender)
            .is_some()
        {
            return Err(ControllerSnapshotError::DuplicateCanisterId {
                canister_id: target.canister_id,
            });
        }
    }
    let mut snapshots = BTreeMap::new();
    for (canister_id, sender) in ordered_targets {
        let mut pending = Vec::new();
        let failure = match try_take_snapshot(pocket_ic, canister_id, sender, &mut pending).await {
            Ok(snapshot) => {
                snapshots.insert(canister_id, snapshot);
                continue;
            }
            Err(failure) => failure,
        };
        let cleanup_failures = cleanup_captured_snapshots(
            pending
                .into_iter()
                .map(|snapshot_id| {
                    (
                        canister_id,
                        ControllerSnapshot {
                            snapshot_id,
                            sender,
                        },
                    )
                })
                .chain(snapshots),
            |canister_id, sender, snapshot_id| {
                pocket_ic.delete_canister_snapshot(canister_id, sender, snapshot_id)
            },
        )
        .await;
        return Err(match failure {
            transfer::Failure::Call(failure) => ControllerSnapshotError::CaptureFailed {
                canister_id,
                sender,
                source: Box::new(failure),
                cleanup_failures,
            },
            transfer::Failure::Invalid(message) => ControllerSnapshotError::InvalidSnapshot {
                canister_id,
                message,
                cleanup_failures,
            },
            transfer::Failure::Codec(source) => ControllerSnapshotError::SnapshotCodec {
                canister_id,
                source,
                cleanup_failures,
            },
        });
    }
    Ok(ControllerSnapshots(snapshots))
}

impl ControllerSnapshots {
    /// Return the number of captured canisters.
    #[must_use]
    pub fn len(&self) -> usize {
        self.0.len()
    }

    /// Report whether the set contains no snapshots.
    #[must_use]
    pub fn is_empty(&self) -> bool {
        self.0.is_empty()
    }

    /// Iterate over captured canister ids in deterministic principal order.
    pub fn canister_ids(&self) -> impl Iterator<Item = Principal> + '_ {
        self.0.keys().copied()
    }

    pub(super) fn iter(&self) -> impl Iterator<Item = (Principal, &[u8], Option<Principal>)> + '_ {
        self.0.iter().map(|(canister_id, snapshot)| {
            (
                *canister_id,
                snapshot.snapshot_id.as_slice(),
                snapshot.sender,
            )
        })
    }
}

async fn try_take_snapshot(
    pocket_ic: &PocketIc,
    canister_id: Principal,
    sender: Option<Principal>,
    pending: &mut Vec<Vec<u8>>,
) -> Result<ControllerSnapshot, transfer::Failure> {
    let snapshot = pocket_ic
        .take_canister_snapshot(canister_id, sender, None)
        .await?;
    pending.push(snapshot.id.clone());
    let uploaded = transfer::upload(pocket_ic, canister_id, sender, &snapshot.id, pending).await?;
    pocket_ic
        .delete_canister_snapshot(canister_id, sender, snapshot.id)
        .await?;
    pending.clear();
    Ok(ControllerSnapshot {
        snapshot_id: uploaded,
        sender,
    })
}

async fn cleanup_captured_snapshots<F, Fut>(
    snapshots: impl IntoIterator<Item = (Principal, ControllerSnapshot)>,
    mut remove: F,
) -> Vec<SnapshotCleanupFailure>
where
    F: FnMut(Principal, Option<Principal>, Vec<u8>) -> Fut,
    Fut: std::future::Future<Output = Result<(), ClientError>>,
{
    let mut failures = Vec::new();
    for (canister_id, snapshot) in snapshots {
        let cleanup = remove(canister_id, snapshot.sender, snapshot.snapshot_id.clone()).await;
        if let Err(source) = cleanup {
            failures.push(SnapshotCleanupFailure {
                canister_id,
                sender: snapshot.sender,
                snapshot_id: snapshot.snapshot_id,
                source: Box::new(source),
            });
        }
    }
    failures
}

async fn restore_controller_snapshot(
    pocket_ic: &PocketIc,
    canister_id: Principal,
    snapshot_id: &[u8],
    funding: SnapshotRestoreFunding,
    sender: Option<Principal>,
) -> Result<(), ControllerSnapshotError> {
    async {
        apply_snapshot_restore_funding(pocket_ic, canister_id, funding).await?;
        pocket_ic
            .load_canister_snapshot(canister_id, sender, snapshot_id.to_vec())
            .await
    }
    .await
    .map_err(|source| ControllerSnapshotError::RestoreFailed {
        canister_id,
        sender,
        source: Box::new(source),
    })
}

async fn apply_snapshot_restore_funding(
    pocket_ic: &PocketIc,
    canister_id: Principal,
    funding: SnapshotRestoreFunding,
) -> Result<(), ClientError> {
    match funding {
        SnapshotRestoreFunding::Preserve => {}
        SnapshotRestoreFunding::TopUpTo { minimum_cycles } => {
            let top_up = minimum_cycles.saturating_sub(pocket_ic.cycle_balance(canister_id).await?);
            if top_up > 0 {
                pocket_ic.add_cycles(canister_id, top_up).await?;
            }
        }
    }
    Ok(())
}

#[cfg(test)]
mod tests;
