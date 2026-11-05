//! Testkit's snapshot policy and rollback bookkeeping over the client transfer owner.
use crate::pocket_ic::{
    ClientError,
    snapshot::{
        CallLimits, ReadCanisterSnapshotMetadataArgs, SnapshotError, SnapshotTransfer,
        TransferLimits, TransferStatus,
    },
};

use super::{PocketIc, Principal};
use ic_platform::candid::contract;

// Preserve the existing management reply and chunk limits. The envelope budget
// includes JSON's byte-array expansion; neither this adapter nor the client
// selects application funding, timer reset, retry or filesystem policy.
const CALL_LIMITS: CallLimits = CallLimits {
    http_bytes: 16 * 1024 * 1024,
    candid_bytes: 2_000_000,
    decoding_quota: usize::MAX,
    skipping_quota: usize::MAX,
};
const TRANSFER_LIMITS: TransferLimits = TransferLimits {
    call: CALL_LIMITS,
    chunk_bytes: 1024 * 1024,
    // Work follows the already captured state, as before. Clients that need a
    // smaller aggregate budget can select the underlying transfer API directly.
    total_bytes: u64::MAX,
};

pub(super) enum Failure {
    Call(ClientError),
    Codec(contract::Error),
    Invalid(&'static str),
}

impl From<ClientError> for Failure {
    fn from(error: ClientError) -> Self {
        Self::Call(error)
    }
}

impl From<SnapshotError> for Failure {
    fn from(error: SnapshotError) -> Self {
        match error {
            SnapshotError::Call {
                source: ClientError::Candid(error),
                ..
            } => Self::Codec(error),
            SnapshotError::Call { source, .. } => Self::Call(source),
            SnapshotError::Invalid(message) => Self::Invalid(message),
        }
    }
}

pub(super) async fn upload(
    pic: &PocketIc,
    canister_id: Principal,
    sender: Option<Principal>,
    original: &[u8],
    pending: &mut Vec<Vec<u8>>,
) -> Result<Vec<u8>, Failure> {
    let sender = sender.unwrap_or_else(Principal::anonymous);
    let metadata = pic
        .read_canister_snapshot_metadata(
            sender,
            ReadCanisterSnapshotMetadataArgs {
                canister_id,
                snapshot_id: original.to_vec(),
            },
            CALL_LIMITS,
        )
        .await?;
    let mut transfer = SnapshotTransfer::new(
        canister_id,
        sender,
        original.to_vec(),
        metadata,
        TRANSFER_LIMITS,
    )?;
    // The first advance only creates the upload. Retain its exact identity
    // before any chunk can fail, so Testkit's outer capture can roll it back.
    transfer.advance(pic).await?;
    let uploaded = transfer
        .uploaded_snapshot()
        .ok_or(Failure::Invalid(
            "snapshot transfer did not return the uploaded snapshot identity",
        ))?
        .to_vec();
    pending.push(uploaded.clone());
    while !matches!(transfer.status(), TransferStatus::Complete) {
        transfer.advance(pic).await?;
    }
    Ok(uploaded)
}
