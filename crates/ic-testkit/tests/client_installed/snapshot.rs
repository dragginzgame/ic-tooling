//! Installed bounded copy preserves heap, stable memory, timer, hook and chunks.
use candid::Principal;
use ic_host_artifacts::wasm_text::{CompileLimits, compile};
use ic_management_canister_types::{CanisterTimer, SnapshotSource};
use ic_testkit::pocket_ic::{
    CanisterSettings, PocketIc,
    snapshot::{
        CallLimits, ReadCanisterSnapshotMetadataArgs, SnapshotTransfer, TransferLimits,
        TransferStatus,
    },
};
use std::time::Duration;

async fn inspect(pic: &PocketIc, id: Principal) -> [u64; 5] {
    let bytes = pic
        .query_call(id, Principal::anonymous(), "inspect", vec![])
        .await
        .unwrap();
    assert_eq!(bytes.len(), 40);
    std::array::from_fn(|i| u64::from_le_bytes(bytes[i * 8..i * 8 + 8].try_into().unwrap()))
}

#[expect(
    clippy::too_many_lines,
    reason = "ordered installed snapshot restoration proof transferred from Platform"
)]
pub async fn qualify(pic: &PocketIc) {
    let sender = Principal::anonymous();
    let id = pic.create_canister().await.unwrap();
    pic.update_canister_settings(
        id,
        None,
        CanisterSettings {
            wasm_memory_limit: Some((20_u64 * 65536).into()),
            wasm_memory_threshold: Some((2_u64 * 65536).into()),
            ..CanisterSettings::default()
        },
    )
    .await
    .unwrap();
    let wasm = compile(
        include_str!("../fixtures/snapshot_timer.wat"),
        CompileLimits {
            source_bytes: 16 * 1024,
            module_bytes: 16 * 1024,
        },
    )
    .unwrap();
    pic.install_canister(id, wasm, vec![], None).await.unwrap();
    let chunk = pic
        .upload_chunk(id, None, b"retained snapshot chunk".to_vec())
        .await
        .unwrap();
    let captured = inspect(pic, id).await;
    assert_eq!(&captured[1..], &[0, 7, 42, 42]);
    pic.stop_canister(id, None).await.unwrap();
    let native = pic.take_canister_snapshot(id, None, None).await.unwrap();
    let budget = CallLimits {
        http_bytes: 4_000_000,
        candid_bytes: 2_000_000,
        decoding_quota: 20_000_000,
        skipping_quota: 20_000_000,
    };
    let metadata = pic
        .read_canister_snapshot_metadata(
            sender,
            ReadCanisterSnapshotMetadataArgs {
                canister_id: id,
                snapshot_id: native.id.clone(),
            },
            budget,
        )
        .await
        .unwrap();
    assert!(matches!(metadata.global_timer,Some(CanisterTimer::Active(t)) if t==captured[0]));
    let hook = std::mem::discriminant(metadata.on_low_wasm_memory_hook_status.as_ref().unwrap());
    let mut copy = SnapshotTransfer::new(
        id,
        sender,
        native.id.clone(),
        metadata,
        TransferLimits {
            call: budget,
            chunk_bytes: 1024 * 1024,
            total_bytes: 4 * 1024 * 1024,
        },
    )
    .unwrap();
    copy.advance(pic).await.unwrap();
    // The consumer takes custody before any data writes, as required by the API.
    let uploaded = copy.uploaded_snapshot().unwrap().to_vec();
    while !matches!(copy.status(), TransferStatus::Complete) {
        copy.advance(pic).await.unwrap();
    }
    assert!(copy.transferred_bytes() > 2 * 1024 * 1024);
    pic.delete_canister_snapshot(id, None, native.id)
        .await
        .unwrap();
    let uploaded_metadata = pic
        .read_canister_snapshot_metadata(
            sender,
            ReadCanisterSnapshotMetadataArgs {
                canister_id: id,
                snapshot_id: uploaded.clone(),
            },
            budget,
        )
        .await
        .unwrap();
    assert!(matches!(
        uploaded_metadata.source,
        Some(SnapshotSource::MetadataUpload(_))
    ));
    pic.start_canister(id, None).await.unwrap();
    pic.update_call(id, sender, "rearm", vec![]).await.unwrap();
    assert_eq!(&inspect(pic, id).await[2..], &[99, 99, 99]);
    pic.clear_chunk_store(id, None).await.unwrap();
    pic.stop_canister(id, None).await.unwrap();
    pic.load_canister_snapshot(id, None, uploaded.clone())
        .await
        .unwrap();
    let restored = pic.take_canister_snapshot(id, None, None).await.unwrap();
    let restored_metadata = pic
        .read_canister_snapshot_metadata(
            sender,
            ReadCanisterSnapshotMetadataArgs {
                canister_id: id,
                snapshot_id: restored.id.clone(),
            },
            budget,
        )
        .await
        .unwrap();
    assert!(
        matches!(restored_metadata.global_timer,Some(CanisterTimer::Active(t)) if t==captured[0])
    );
    assert_eq!(
        std::mem::discriminant(
            restored_metadata
                .on_low_wasm_memory_hook_status
                .as_ref()
                .unwrap()
        ),
        hook
    );
    pic.delete_canister_snapshot(id, None, restored.id)
        .await
        .unwrap();
    pic.start_canister(id, None).await.unwrap();
    assert_eq!(inspect(pic, id).await, captured);
    assert_eq!(pic.stored_chunks(id, None).await.unwrap(), vec![chunk]);
    pic.advance_time(Duration::from_secs(61)).await.unwrap();
    pic.tick().await.unwrap();
    assert_eq!(inspect(pic, id).await[1], 1, "restored timer must execute");
    pic.delete_canister_snapshot(id, None, uploaded)
        .await
        .unwrap();
    println!(
        "Bounded snapshot copy: module/heap/stable/globals/chunks/timer/hook restoration passed"
    );
}
