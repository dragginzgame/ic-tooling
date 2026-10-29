//! Installed regression for snapshot heap/system scheduling coherence (#263).
mod support;

use ic_host_artifacts::wasm_text::{CompileLimits, compile};
use ic_platform::{
    candid::contract::{self, DecoderConfig},
    management_canister_types::{
        CanisterTimer, ReadCanisterSnapshotMetadataArgs, ReadCanisterSnapshotMetadataResult,
        SnapshotSource,
    },
    pocket_ic::{CanisterSettings, PocketIc, common::rest::RawEffectivePrincipal},
    principal::ExternalPrincipal as Principal,
};
use ic_testkit::pic::{CachedPocketIcBaseline, CanisterSnapshotTarget, SnapshotRestoreFunding};
use std::time::Duration;

fn inspect(pic: &PocketIc, id: Principal) -> [u64; 5] {
    let bytes = pic
        .query_call(id, Principal::anonymous(), "inspect", vec![])
        .unwrap();
    assert_eq!(bytes.len(), 40);
    std::array::from_fn(|i| u64::from_le_bytes(bytes[i * 8..i * 8 + 8].try_into().unwrap()))
}

fn metadata(
    pic: &PocketIc,
    id: Principal,
    snapshot_id: Vec<u8>,
) -> ReadCanisterSnapshotMetadataResult {
    let bytes = pic
        .update_call_with_effective_principal(
            Principal::management_canister(),
            RawEffectivePrincipal::CanisterId(id.as_slice().to_vec()),
            Principal::anonymous(),
            "read_canister_snapshot_metadata",
            contract::encode_one(ReadCanisterSnapshotMetadataArgs {
                canister_id: id,
                snapshot_id,
            })
            .unwrap(),
        )
        .unwrap();
    let (reply,) = contract::decode_args_with_config(&bytes, &DecoderConfig::new()).unwrap();
    reply
}

fn install_timer_canister(pic: &PocketIc) -> (Principal, Vec<u8>) {
    let id = pic.create_canister();
    pic.update_canister_settings(
        id,
        None,
        CanisterSettings {
            wasm_memory_limit: Some((20_u64 * 65536).into()),
            wasm_memory_threshold: Some((2_u64 * 65536).into()),
            ..CanisterSettings::default()
        },
    )
    .unwrap();
    let wasm = compile(
        include_str!("fixtures/snapshot_timer.wat"),
        CompileLimits {
            source_bytes: 16 * 1024,
            module_bytes: 16 * 1024,
        },
    )
    .unwrap();
    pic.install_canister(id, wasm, vec![], None);
    let chunk = pic
        .upload_chunk(id, None, b"retained snapshot chunk".to_vec())
        .unwrap();
    (id, chunk)
}

#[test]
fn repeated_baseline_restore_keeps_heap_timer_hook_and_memory_coherent() {
    let owned = support::pocket_ic();
    let pic = owned.instance();
    let (id, chunk) = install_timer_canister(pic);
    let captured = inspect(pic, id);
    assert_eq!(&captured[1..], &[0, 7, 42, 42]);
    pic.stop_canister(id, None).unwrap();
    let baseline = CachedPocketIcBaseline::capture(
        owned,
        [CanisterSnapshotTarget {
            canister_id: id,
            sender: None,
        }],
        id,
    )
    .unwrap();
    let pic = baseline.pocket_ic();
    let snapshots = pic.list_canister_snapshots(id, None).unwrap();
    assert_eq!(
        snapshots.len(),
        1,
        "temporary native snapshot must be deleted"
    );
    let snapshot = metadata(pic, id, snapshots[0].id.clone());
    let source = snapshot.source;
    assert!(matches!(snapshot.global_timer, Some(CanisterTimer::Active(t)) if t == captured[0]));
    let hook = snapshot.on_low_wasm_memory_hook_status.unwrap();
    pic.start_canister(id, None).unwrap();
    for _ in 0..3 {
        pic.update_call(id, Principal::anonymous(), "rearm", vec![])
            .unwrap();
        assert_eq!(&inspect(pic, id)[2..], &[99, 99, 99]);
        pic.clear_chunk_store(id, None).unwrap();
        pic.stop_canister(id, None).unwrap();
        let changed = pic.take_canister_snapshot(id, None, None).unwrap();
        let changed_metadata = metadata(pic, id, changed.id.clone());
        assert_ne!(
            std::mem::discriminant(&changed_metadata.on_low_wasm_memory_hook_status.unwrap()),
            std::mem::discriminant(&hook)
        );
        pic.delete_canister_snapshot(id, None, changed.id).unwrap();
        let time = pic.get_time();
        baseline.restore(SnapshotRestoreFunding::Preserve).unwrap();
        assert!(
            pic.get_time() >= time,
            "restoration must not rewind simulated time"
        );
        let current = pic.take_canister_snapshot(id, None, None).unwrap();
        let restored = metadata(pic, id, current.id.clone());
        assert!(
            matches!(restored.global_timer, Some(CanisterTimer::Active(t)) if t == captured[0])
        );
        assert_eq!(
            std::mem::discriminant(&restored.on_low_wasm_memory_hook_status.unwrap()),
            std::mem::discriminant(&hook)
        );
        pic.delete_canister_snapshot(id, None, current.id).unwrap();
        pic.start_canister(id, None).unwrap();
        assert_eq!(inspect(pic, id), captured);
        let chunks = pic.stored_chunks(id, None).unwrap();
        assert_eq!(chunks.len(), 1);
        assert_eq!(chunks[0], chunk);
    }
    assert!(matches!(source, Some(SnapshotSource::MetadataUpload(_))));
    pic.advance_time(Duration::from_secs(61));
    pic.tick();
    assert_eq!(
        inspect(pic, id)[1],
        1,
        "restored deadline must actually execute"
    );
    pic.stop_canister(id, None).unwrap();
    baseline.restore(SnapshotRestoreFunding::Preserve).unwrap();
    pic.start_canister(id, None).unwrap();
    pic.tick();
    assert_eq!(
        inspect(pic, id)[1],
        1,
        "expired captured timer must execute after restart"
    );
}
