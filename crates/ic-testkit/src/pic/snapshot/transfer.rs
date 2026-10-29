//! Convert a native snapshot to metadata-upload semantics without shared files.

use super::{PocketIc, PocketIcCallFailure, Principal, transport};
use ic_platform::{
    candid::contract::{self, ArgumentDecoder, CandidType, DecoderConfig},
    management_canister_types::{
        ReadCanisterSnapshotDataArgs, ReadCanisterSnapshotDataResult,
        ReadCanisterSnapshotMetadataArgs, ReadCanisterSnapshotMetadataResult, SnapshotDataKind,
        SnapshotDataOffset, UploadCanisterSnapshotDataArgs, UploadCanisterSnapshotMetadataArgs,
        UploadCanisterSnapshotMetadataResult,
    },
    pocket_ic::common::rest::RawEffectivePrincipal,
};

// One MiB leaves room for Candid framing within the management message limit.
// Total work follows the already captured canister state; no whole heap buffer,
// filesystem, application-specific timer reset or additional clock policy.
const CHUNK_BYTES: u64 = 1024 * 1024;

pub(super) enum Failure {
    Call(PocketIcCallFailure),
    Codec(contract::Error),
    Invalid(&'static str),
}

impl From<PocketIcCallFailure> for Failure {
    fn from(error: PocketIcCallFailure) -> Self {
        Self::Call(error)
    }
}

fn call<A: CandidType, R: for<'de> ArgumentDecoder<'de>>(
    pic: &PocketIc,
    canister_id: Principal,
    sender: Option<Principal>,
    method: &str,
    args: A,
) -> Result<R, Failure> {
    let args = contract::encode_args((args,)).map_err(Failure::Codec)?;
    let reply = transport::capture_call(|| {
        pic.update_call_with_effective_principal(
            Principal::management_canister(),
            RawEffectivePrincipal::CanisterId(canister_id.as_slice().to_vec()),
            sender.unwrap_or_else(Principal::anonymous),
            method,
            args,
        )
    })?;
    decode_reply(&reply)
}

fn decode_reply<R: for<'de> ArgumentDecoder<'de>>(reply: &[u8]) -> Result<R, Failure> {
    if reply.len() > 2_000_000 {
        return Err(Failure::Invalid(
            "snapshot reply exceeds the management message limit",
        ));
    }
    let mut config = DecoderConfig::new();
    config.set_full_error_message(false);
    contract::decode_args_with_config(reply, &config).map_err(Failure::Codec)
}

pub(super) fn upload(
    pic: &PocketIc,
    canister_id: Principal,
    sender: Option<Principal>,
    original: &[u8],
    pending: &mut Vec<Vec<u8>>,
) -> Result<Vec<u8>, Failure> {
    let (metadata,): (ReadCanisterSnapshotMetadataResult,) = call(
        pic,
        canister_id,
        sender,
        "read_canister_snapshot_metadata",
        ReadCanisterSnapshotMetadataArgs {
            canister_id,
            snapshot_id: original.to_vec(),
        },
    )?;
    require_complete_system_state(&metadata)?;
    let (uploaded,): (UploadCanisterSnapshotMetadataResult,) = call(
        pic,
        canister_id,
        sender,
        "upload_canister_snapshot_metadata",
        UploadCanisterSnapshotMetadataArgs {
            canister_id,
            replace_snapshot: None,
            wasm_module_size: metadata.wasm_module_size,
            // None denotes globals outside the exported/mutable snapshot set.
            globals: metadata.globals.into_iter().flatten().collect(),
            wasm_memory_size: metadata.wasm_memory_size,
            stable_memory_size: metadata.stable_memory_size,
            certified_data: metadata.certified_data,
            global_timer: metadata.global_timer,
            on_low_wasm_memory_hook_status: metadata.on_low_wasm_memory_hook_status,
        },
    )?;
    let uploaded = uploaded.snapshot_id;
    pending.push(uploaded.clone());
    copy_region(
        pic,
        canister_id,
        sender,
        original,
        &uploaded,
        metadata.wasm_module_size,
        |offset, size| SnapshotDataKind::WasmModule { offset, size },
        |offset| SnapshotDataOffset::WasmModule { offset },
    )?;
    copy_region(
        pic,
        canister_id,
        sender,
        original,
        &uploaded,
        metadata.wasm_memory_size,
        |offset, size| SnapshotDataKind::WasmMemory { offset, size },
        |offset| SnapshotDataOffset::WasmMemory { offset },
    )?;
    copy_region(
        pic,
        canister_id,
        sender,
        original,
        &uploaded,
        metadata.stable_memory_size,
        |offset, size| SnapshotDataKind::StableMemory { offset, size },
        |offset| SnapshotDataOffset::StableMemory { offset },
    )?;
    for chunk in metadata.wasm_chunk_store {
        let (data,): (ReadCanisterSnapshotDataResult,) = call(
            pic,
            canister_id,
            sender,
            "read_canister_snapshot_data",
            ReadCanisterSnapshotDataArgs {
                canister_id,
                snapshot_id: original.to_vec(),
                kind: SnapshotDataKind::WasmChunk {
                    hash: chunk.hash.clone(),
                },
            },
        )?;
        if data.chunk.len() as u64 > CHUNK_BYTES
            || ic_host_artifacts::artifact::Sha256Digest::compute(&data.chunk)
                .as_bytes()
                .as_slice()
                != chunk.hash.as_slice()
        {
            return Err(Failure::Invalid(
                "snapshot chunk store entry exceeds the protocol limit or has the wrong hash",
            ));
        }
        let (): () = call(
            pic,
            canister_id,
            sender,
            "upload_canister_snapshot_data",
            UploadCanisterSnapshotDataArgs {
                canister_id,
                snapshot_id: uploaded.clone(),
                kind: SnapshotDataOffset::WasmChunk,
                chunk: data.chunk,
            },
        )?;
    }
    Ok(uploaded)
}

const fn require_complete_system_state(
    metadata: &ReadCanisterSnapshotMetadataResult,
) -> Result<(), Failure> {
    if metadata.global_timer.is_none() || metadata.on_low_wasm_memory_hook_status.is_none() {
        return Err(Failure::Invalid(
            "snapshot omitted timer or low-memory hook state",
        ));
    }
    Ok(())
}

#[expect(
    clippy::too_many_arguments,
    reason = "one bounded region transfer keeps caller identity and exact read/write selections explicit"
)]
fn copy_region(
    pic: &PocketIc,
    canister_id: Principal,
    sender: Option<Principal>,
    original: &[u8],
    uploaded: &[u8],
    bytes: u64,
    read_kind: impl Fn(u64, u64) -> SnapshotDataKind,
    write_kind: impl Fn(u64) -> SnapshotDataOffset,
) -> Result<(), Failure> {
    let mut offset = 0;
    while offset < bytes {
        let size = CHUNK_BYTES.min(bytes - offset);
        let (data,): (ReadCanisterSnapshotDataResult,) = call(
            pic,
            canister_id,
            sender,
            "read_canister_snapshot_data",
            ReadCanisterSnapshotDataArgs {
                canister_id,
                snapshot_id: original.to_vec(),
                kind: read_kind(offset, size),
            },
        )?;
        if data.chunk.len() as u64 != size {
            return Err(Failure::Invalid(
                "snapshot returned an incomplete data region",
            ));
        }
        let (): () = call(
            pic,
            canister_id,
            sender,
            "upload_canister_snapshot_data",
            UploadCanisterSnapshotDataArgs {
                canister_id,
                snapshot_id: uploaded.to_vec(),
                kind: write_kind(offset),
                chunk: data.chunk,
            },
        )?;
        offset += size;
    }
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::{Failure, decode_reply, require_complete_system_state};
    use ic_platform::candid::contract;
    use ic_platform::management_canister_types::{
        CanisterTimer, OnLowWasmMemoryHookStatus, ReadCanisterSnapshotMetadataResult,
    };

    #[test]
    fn omitted_timer_or_hook_is_refused_instead_of_silently_preserved_on_restore() {
        let mut metadata = ReadCanisterSnapshotMetadataResult {
            source: None,
            taken_at_timestamp: 0,
            wasm_module_size: 0,
            globals: vec![],
            wasm_memory_size: 0,
            stable_memory_size: 0,
            wasm_chunk_store: vec![],
            canister_version: 0,
            certified_data: vec![],
            global_timer: Some(CanisterTimer::Inactive),
            on_low_wasm_memory_hook_status: Some(OnLowWasmMemoryHookStatus::ConditionNotSatisfied),
        };
        assert!(require_complete_system_state(&metadata).is_ok());
        metadata.global_timer = None;
        assert!(matches!(
            require_complete_system_state(&metadata),
            Err(Failure::Invalid(_))
        ));
        metadata.global_timer = Some(CanisterTimer::Inactive);
        metadata.on_low_wasm_memory_hook_status = None;
        assert!(matches!(
            require_complete_system_state(&metadata),
            Err(Failure::Invalid(_))
        ));
    }

    #[test]
    fn snapshot_replies_refuse_oversize_malformed_and_wrong_shapes() {
        assert!(matches!(
            decode_reply::<()>(&vec![0; 2_000_001]),
            Err(Failure::Invalid(_))
        ));
        assert!(matches!(
            decode_reply::<()>(b"malformed"),
            Err(Failure::Codec(_))
        ));
        let empty = contract::encode_args(()).unwrap();
        assert!(decode_reply::<()>(&empty).is_ok());
        assert!(matches!(
            decode_reply::<(u64,)>(&empty),
            Err(Failure::Codec(_))
        ));
    }
}
