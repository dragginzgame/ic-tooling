//! Ordered controller snapshots using upstream management operations.
use ic_platform::{
    candid_codec::{
        self, Encode,
        arguments::DecodeArgs,
        decoder::{self, Arity, Limits},
    },
    management_canister_types::{
        ReadCanisterSnapshotDataArgs, ReadCanisterSnapshotDataResult,
        ReadCanisterSnapshotMetadataArgs, ReadCanisterSnapshotMetadataResult, SnapshotDataKind,
        SnapshotDataOffset, UploadCanisterSnapshotDataArgs, UploadCanisterSnapshotMetadataArgs,
        UploadCanisterSnapshotMetadataResult,
    },
};
use pocket_ic::common::rest::RawEffectivePrincipal;
use pocket_ic::{CanisterId as Principal, RejectResponse, nonblocking::PocketIc};
use std::collections::BTreeMap;

// Keep management frames within 2 MiB, streaming 1 MiB data windows with room
// for Candid envelopes. These bound message buffers, not total snapshot size.
const MESSAGE_BYTES: usize = 2 * 1024 * 1024;
const DATA_BYTES: u64 = 1024 * 1024;

/// Local admission failure preparing a coherent snapshot. No partial data is accepted.
#[derive(Debug, thiserror::Error)]
pub enum SnapshotPreparationError {
    /// The owned codec refused a management request.
    #[error("cannot encode {method}: {reason}")]
    Encoding {
        /// Management method.
        method: &'static str,
        /// Codec refusal without request contents.
        reason: String,
    },
    /// The owned codec refused a management reply.
    #[error("cannot decode {method}: {reason}")]
    Decoding {
        /// Management method.
        method: &'static str,
        /// Codec refusal without response contents.
        reason: String,
    },
    /// Required snapshot state was not reported.
    #[error("snapshot metadata is missing {field}")]
    MissingMetadata {
        /// Required metadata field.
        field: &'static str,
    },
    /// The server returned an incomplete or overlong data range.
    #[error("snapshot range returned {actual} bytes, expected {expected}")]
    DataLength {
        /// Requested range length.
        expected: u64,
        /// Observed range length.
        actual: usize,
    },
}

enum PreparationFailure {
    Management(Box<RejectResponse>),
    Local(SnapshotPreparationError),
}

impl From<SnapshotPreparationError> for PreparationFailure {
    fn from(error: SnapshotPreparationError) -> Self {
        Self::Local(error)
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
struct ControllerSnapshot {
    snapshot_id: Vec<u8>,
    sender: Option<Principal>,
}

/// Deterministically ordered snapshots and the exact senders that captured them.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ControllerSnapshots(Vec<(Principal, ControllerSnapshot)>);

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
        /// Rejections deleting known temporary, uploaded and earlier snapshots.
        cleanup_failures: Vec<SnapshotCleanupFailure>,
    },
    /// Coherent capture refused metadata or data after known snapshots were created.
    #[error("failed to prepare snapshot for {canister_id} with sender {sender:?}: {source}; {} cleanup rejections", .cleanup_failures.len())]
    PreparationFailed {
        /// Failed target.
        canister_id: Principal,
        /// Exact selected sender.
        sender: Option<Principal>,
        /// Local preparation refusal.
        source: Box<SnapshotPreparationError>,
        /// Rejections deleting known source, uploaded and earlier snapshots.
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
/// Callers stop/drain targets and disable automatic progress. Each source snapshot
/// is copied through metadata/data management calls into an uploaded snapshot,
/// preserving global timer and low-memory-hook state with the heap. Capture needs
/// two free snapshot slots per target, transfer cycles and streamed data buffers;
/// it never rewinds simulated time. Warm restoration only loads the prepared snapshot.
/// Cancellation and transport panics can leave partial captures for reconciliation.
///
/// # Errors
/// Refuses duplicates before effects; returns capture/preparation failures with rollback failures.
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
    let mut snapshots = Vec::with_capacity(ordered.len());
    for (canister_id, sender) in ordered {
        let capture = async {
            let original = pic
                .take_canister_snapshot(canister_id, sender, None)
                .await
                .map_err(|error| PreparationFailure::Management(Box::new(error)))?;
            snapshots.push((
                canister_id,
                ControllerSnapshot {
                    snapshot_id: original.id.clone(),
                    sender,
                },
            ));
            prepare_snapshot(pic, canister_id, sender, original.id, &mut snapshots).await
        }
        .await;
        if let Err(source) = capture {
            let cleanup_failures = cleanup_captured_snapshots(snapshots, |id, sender, snapshot| {
                pic.delete_canister_snapshot(id, sender, snapshot)
            })
            .await;
            return Err(match source {
                PreparationFailure::Management(rejection) => {
                    ControllerSnapshotError::CaptureFailed {
                        canister_id,
                        sender,
                        rejection,
                        cleanup_failures,
                    }
                }
                PreparationFailure::Local(source) => ControllerSnapshotError::PreparationFailed {
                    canister_id,
                    sender,
                    source: Box::new(source),
                    cleanup_failures,
                },
            });
        }
    }
    Ok(ControllerSnapshots(snapshots))
}

async fn management<I: Encode, R: for<'a> DecodeArgs<'a>>(
    pic: &PocketIc,
    canister_id: Principal,
    sender: Option<Principal>,
    method: &'static str,
    input: I,
) -> Result<R, PreparationFailure> {
    let bytes = candid_codec::encode_one(&input, MESSAGE_BYTES).map_err(|error| {
        SnapshotPreparationError::Encoding {
            method,
            reason: format!("{error:?}"),
        }
    })?;
    let response = pic
        .update_call_with_effective_principal(
            Principal::management_canister(),
            RawEffectivePrincipal::CanisterId(canister_id.as_slice().to_vec()),
            sender.unwrap_or(Principal::anonymous()),
            method,
            bytes,
        )
        .await
        .map_err(|error| PreparationFailure::Management(Box::new(error)))?;
    decoder::decode_args(
        &response,
        Limits {
            bytes: MESSAGE_BYTES,
            header: MESSAGE_BYTES,
            types: 128,
            work: MESSAGE_BYTES * 64,
            allocation: MESSAGE_BYTES * 64,
        },
        Arity::Exact,
    )
    .map_err(|error| {
        SnapshotPreparationError::Decoding {
            method,
            reason: format!("{error:?}"),
        }
        .into()
    })
}

async fn prepare_snapshot(
    pic: &PocketIc,
    canister_id: Principal,
    sender: Option<Principal>,
    source_id: Vec<u8>,
    known: &mut Vec<(Principal, ControllerSnapshot)>,
) -> Result<(), PreparationFailure> {
    let native_id = ic_platform_types::Principal::from_slice(canister_id.as_slice());
    let (metadata,): (ReadCanisterSnapshotMetadataResult,) = management(
        pic,
        canister_id,
        sender,
        "read_canister_snapshot_metadata",
        ReadCanisterSnapshotMetadataArgs {
            canister_id: native_id,
            snapshot_id: source_id.clone(),
        },
    )
    .await?;
    let globals = metadata
        .globals
        .into_iter()
        .collect::<Option<Vec<_>>>()
        .ok_or(SnapshotPreparationError::MissingMetadata { field: "globals" })?;
    let global_timer = metadata
        .global_timer
        .ok_or(SnapshotPreparationError::MissingMetadata {
            field: "global_timer",
        })?;
    let hook = metadata.on_low_wasm_memory_hook_status.ok_or(
        SnapshotPreparationError::MissingMetadata {
            field: "on_low_wasm_memory_hook_status",
        },
    )?;
    let (uploaded,): (UploadCanisterSnapshotMetadataResult,) = management(
        pic,
        canister_id,
        sender,
        "upload_canister_snapshot_metadata",
        UploadCanisterSnapshotMetadataArgs {
            canister_id: native_id,
            replace_snapshot: None,
            wasm_module_size: metadata.wasm_module_size,
            globals,
            wasm_memory_size: metadata.wasm_memory_size,
            stable_memory_size: metadata.stable_memory_size,
            certified_data: metadata.certified_data,
            global_timer: Some(global_timer),
            on_low_wasm_memory_hook_status: Some(hook),
        },
    )
    .await?;
    let uploaded_id = uploaded.snapshot_id;
    known.push((
        canister_id,
        ControllerSnapshot {
            snapshot_id: uploaded_id.clone(),
            sender,
        },
    ));
    copy_ranges(
        pic,
        canister_id,
        sender,
        &source_id,
        &uploaded_id,
        [
            metadata.wasm_module_size,
            metadata.wasm_memory_size,
            metadata.stable_memory_size,
        ],
    )
    .await?;
    for chunk in metadata.wasm_chunk_store {
        let (data,): (ReadCanisterSnapshotDataResult,) = management(
            pic,
            canister_id,
            sender,
            "read_canister_snapshot_data",
            ReadCanisterSnapshotDataArgs {
                canister_id: native_id,
                snapshot_id: source_id.clone(),
                kind: SnapshotDataKind::WasmChunk { hash: chunk.hash },
            },
        )
        .await?;
        let (): () = management(
            pic,
            canister_id,
            sender,
            "upload_canister_snapshot_data",
            UploadCanisterSnapshotDataArgs {
                canister_id: native_id,
                snapshot_id: uploaded_id.clone(),
                kind: SnapshotDataOffset::WasmChunk,
                chunk: data.chunk,
            },
        )
        .await?;
    }
    pic.delete_canister_snapshot(canister_id, sender, source_id)
        .await
        .map_err(|error| PreparationFailure::Management(Box::new(error)))?;
    // The source and its prepared replacement are always the final two known IDs.
    known.remove(known.len() - 2);
    Ok(())
}

async fn copy_ranges(
    pic: &PocketIc,
    canister_id: Principal,
    sender: Option<Principal>,
    source_id: &[u8],
    uploaded_id: &[u8],
    lengths: [u64; 3],
) -> Result<(), PreparationFailure> {
    let native_id = ic_platform_types::Principal::from_slice(canister_id.as_slice());
    for (region, length) in lengths.into_iter().enumerate() {
        let mut offset = 0;
        while offset < length {
            let size = DATA_BYTES.min(length - offset);
            let (kind, destination) = match region {
                0 => (
                    SnapshotDataKind::WasmModule { offset, size },
                    SnapshotDataOffset::WasmModule { offset },
                ),
                1 => (
                    SnapshotDataKind::WasmMemory { offset, size },
                    SnapshotDataOffset::WasmMemory { offset },
                ),
                _ => (
                    SnapshotDataKind::StableMemory { offset, size },
                    SnapshotDataOffset::StableMemory { offset },
                ),
            };
            let (data,): (ReadCanisterSnapshotDataResult,) = management(
                pic,
                canister_id,
                sender,
                "read_canister_snapshot_data",
                ReadCanisterSnapshotDataArgs {
                    canister_id: native_id,
                    snapshot_id: source_id.to_vec(),
                    kind,
                },
            )
            .await?;
            if u64::try_from(data.chunk.len()).ok() != Some(size) {
                return Err(SnapshotPreparationError::DataLength {
                    expected: size,
                    actual: data.chunk.len(),
                }
                .into());
            }
            let (): () = management(
                pic,
                canister_id,
                sender,
                "upload_canister_snapshot_data",
                UploadCanisterSnapshotDataArgs {
                    canister_id: native_id,
                    snapshot_id: uploaded_id.to_vec(),
                    kind: destination,
                    chunk: data.chunk,
                },
            )
            .await?;
            offset += size;
        }
    }
    Ok(())
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
    pub const fn len(&self) -> usize {
        self.0.len()
    }
    /// Whether no canisters were captured.
    #[must_use]
    pub const fn is_empty(&self) -> bool {
        self.0.is_empty()
    }
    /// Captured ids in deterministic order.
    pub fn canister_ids(&self) -> impl Iterator<Item = Principal> + '_ {
        self.0.iter().map(|(id, _)| *id)
    }
}

async fn cleanup_captured_snapshots<F, Fut>(
    snapshots: Vec<(Principal, ControllerSnapshot)>,
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
