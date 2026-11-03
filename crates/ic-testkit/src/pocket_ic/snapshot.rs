//! Bounded native snapshot copying. Callers own funding, custody and recovery.

use crate::pocket_ic::{ClientError, common::rest::RawMessageId};
use candid::Principal;
pub use ic_management_canister_types::{
    ReadCanisterSnapshotDataArgs, ReadCanisterSnapshotDataResult, ReadCanisterSnapshotMetadataArgs,
    ReadCanisterSnapshotMetadataResult, SnapshotDataKind, SnapshotDataOffset,
    UploadCanisterSnapshotDataArgs, UploadCanisterSnapshotMetadataArgs,
    UploadCanisterSnapshotMetadataResult,
};

/// Explicit per-call bounds, including the outer PocketIC HTTP/JSON envelope.
#[derive(Clone, Copy, Debug)]
pub struct CallLimits {
    pub http_bytes: usize,
    pub candid_bytes: usize,
    pub decoding_quota: usize,
    pub skipping_quota: usize,
}

/// Caller-selected memory/work limits. No implicit application budget is added.
#[derive(Clone, Copy, Debug)]
pub struct TransferLimits {
    pub call: CallLimits,
    pub chunk_bytes: u64,
    pub total_bytes: u64,
}

#[derive(Debug, thiserror::Error)]
pub enum SnapshotError {
    #[error("snapshot call failed: {source}")]
    Call {
        #[source]
        source: ClientError,
        /// Retained even when Candid decoding fails after accepted ingress.
        ingress: Option<RawMessageId>,
    },
    #[error("invalid snapshot transfer: {0}")]
    Invalid(&'static str),
}

/// The last attempted action. An interrupted action cannot be retried implicitly.
#[derive(Clone, Debug)]
pub enum TransferStatus {
    Ready,
    UploadingMetadata,
    Reading(SnapshotDataKind),
    Writing(SnapshotDataOffset),
    Complete,
}

/// Caller-owned progress survives errors and dropped async futures.
///
/// First advance uploads metadata only: record `uploaded_snapshot()` in the
/// caller's pending set before advancing again. Each later advance copies at most
/// one bounded chunk. On error/cancellation the status retains the attempted
/// action and further advances refuse. Reconcile explicitly using the raw methods;
/// this object never deletes snapshots, loads them, retries or performs Drop I/O.
#[derive(Debug)]
pub struct SnapshotTransfer {
    pub(crate) canister: Principal,
    pub(crate) sender: Principal,
    pub(crate) original: Vec<u8>,
    pub(crate) metadata: ReadCanisterSnapshotMetadataResult,
    pub(crate) limits: TransferLimits,
    pub(crate) uploaded: Option<Vec<u8>>,
    pub(crate) status: TransferStatus,
    pub(crate) ingress: Option<RawMessageId>,
    pub(crate) region: usize,
    pub(crate) offset: u64,
    pub(crate) transferred: u64,
}

impl SnapshotTransfer {
    pub fn new(
        canister: Principal,
        sender: Principal,
        original: Vec<u8>,
        metadata: ReadCanisterSnapshotMetadataResult,
        limits: TransferLimits,
    ) -> Result<Self, SnapshotError> {
        if limits.chunk_bytes == 0 || limits.chunk_bytes > 1024 * 1024 {
            return Err(SnapshotError::Invalid("chunk size must be 1..=1 MiB"));
        }
        if metadata.global_timer.is_none() || metadata.on_low_wasm_memory_hook_status.is_none() {
            return Err(SnapshotError::Invalid(
                "snapshot omitted timer or low-memory hook state",
            ));
        }
        let bytes = metadata
            .wasm_module_size
            .checked_add(metadata.wasm_memory_size)
            .and_then(|bytes| bytes.checked_add(metadata.stable_memory_size))
            .ok_or(SnapshotError::Invalid("snapshot region sizes overflow"))?;
        if bytes > limits.total_bytes {
            return Err(SnapshotError::Invalid("snapshot exceeds total byte budget"));
        }
        let mut hashes = std::collections::BTreeSet::new();
        for chunk in &metadata.wasm_chunk_store {
            if chunk.hash.len() != 32 || !hashes.insert(chunk.hash.clone()) {
                return Err(SnapshotError::Invalid("invalid or duplicate chunk hash"));
            }
        }
        Ok(Self {
            canister,
            sender,
            original,
            metadata,
            limits,
            uploaded: None,
            status: TransferStatus::Ready,
            ingress: None,
            region: 0,
            offset: 0,
            transferred: 0,
        })
    }

    pub fn uploaded_snapshot(&self) -> Option<&[u8]> {
        self.uploaded.as_deref()
    }
    pub fn status(&self) -> &TransferStatus {
        &self.status
    }
    pub fn ingress(&self) -> Option<&RawMessageId> {
        self.ingress.as_ref()
    }
    pub fn transferred_bytes(&self) -> u64 {
        self.transferred
    }

    /// Synchronous frontend for the same async implementation.
    pub fn advance(&mut self, client: &crate::pocket_ic::PocketIc) -> Result<(), SnapshotError> {
        client
            .runtime
            .block_on(self.advance_async(&client.pocket_ic))
    }
}

impl crate::pocket_ic::PocketIc {
    pub fn read_canister_snapshot_metadata(
        &self,
        sender: Principal,
        args: ReadCanisterSnapshotMetadataArgs,
        limits: CallLimits,
    ) -> Result<ReadCanisterSnapshotMetadataResult, SnapshotError> {
        self.runtime.block_on(
            self.pocket_ic
                .read_canister_snapshot_metadata(sender, args, limits),
        )
    }
    pub fn read_canister_snapshot_data(
        &self,
        sender: Principal,
        args: ReadCanisterSnapshotDataArgs,
        limits: CallLimits,
    ) -> Result<ReadCanisterSnapshotDataResult, SnapshotError> {
        self.runtime.block_on(
            self.pocket_ic
                .read_canister_snapshot_data(sender, args, limits),
        )
    }
    pub fn upload_canister_snapshot_metadata(
        &self,
        sender: Principal,
        args: UploadCanisterSnapshotMetadataArgs,
        limits: CallLimits,
    ) -> Result<UploadCanisterSnapshotMetadataResult, SnapshotError> {
        self.runtime.block_on(
            self.pocket_ic
                .upload_canister_snapshot_metadata(sender, args, limits),
        )
    }
    pub fn upload_canister_snapshot_data(
        &self,
        sender: Principal,
        args: UploadCanisterSnapshotDataArgs,
        limits: CallLimits,
    ) -> Result<(), SnapshotError> {
        self.runtime.block_on(
            self.pocket_ic
                .upload_canister_snapshot_data(sender, args, limits),
        )
    }
}
