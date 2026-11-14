//! Integration checks for pocket ic snapshots behavior.
mod support;

use ic_testkit::pic::{CanisterSnapshotTarget, ControllerSnapshotError, SnapshotRestoreFunding};
use pocket_ic::CanisterId as Principal;
use pocket_ic::CanisterSettings;

// This fixture independently models the heap/system ownership invariant. It
// imports no timer library and traps if arming observes a different prior timer.
const TIMER_WAT: &str = r#"(module
  (import "ic0" "global_timer_set" (func $timer (param i64) (result i64)))
  (import "ic0" "time" (func $time (result i64)))
  (import "ic0" "msg_arg_data_copy" (func $args (param i32 i32 i32)))
  (import "ic0" "msg_reply_data_append" (func $append (param i32 i32)))
  (import "ic0" "msg_reply" (func $reply))
  (import "ic0" "trap" (func $trap (param i32 i32)))
  (import "ic0" "stable64_grow" (func $grow (param i64) (result i64)))
  (import "ic0" "stable64_write" (func $write (param i64 i64 i64)))
  (import "ic0" "stable64_read" (func $read (param i64 i64 i64)))
  (import "ic0" "certified_data_set" (func $certify (param i32 i32)))
  (memory (export "memory") 20)
  (global $immutable i64 (i64.const 41))
  (global $expected (mut i64) (i64.const 0))
  (global $fired (mut i64) (i64.const 0))
  (global $hooks (mut i64) (i64.const 0))
  (data (i32.const 128) "competing global timer owner")
  (func $arm (param $next i64)
    (if (i64.ne (call $timer (local.get $next)) (global.get $expected))
      (then (call $trap (i32.const 128) (i32.const 28))))
    (global.set $expected (local.get $next)))
  (func (export "canister_init")
    (drop (call $grow (i64.const 20)))
    (i32.store8 (i32.const 1200000) (i32.const 77))
    (call $write (i64.const 1200000) (i64.const 1200000) (i64.const 1))
    (call $certify (i32.const 1200000) (i32.const 1))
    (call $arm (i64.add (call $time) (i64.const 60000000000))))
  (func (export "canister_update mutate")
    (call $args (i32.const 0) (i32.const 0) (i32.const 8))
    (call $arm (i64.load (i32.const 0)))
    (drop (memory.grow (i32.const 32)))
    (i32.store8 (i32.const 1200000) (i32.const 99))
    (call $write (i64.const 1200000) (i64.const 1200000) (i64.const 1))
    (call $certify (i32.const 1200000) (i32.const 1))
    (call $reply))
  (func (export "canister_global_timer")
    (global.set $expected (i64.const 0))
    (global.set $fired (i64.add (global.get $fired) (i64.const 1)))
    (call $arm (i64.add (call $time) (i64.const 10000000000))))
  (func (export "canister_on_low_wasm_memory")
    (global.set $hooks (i64.add (global.get $hooks) (i64.const 1))))
  (func (export "canister_query state")
    (i64.store (i32.const 0) (global.get $expected))
    (i64.store (i32.const 8) (global.get $fired))
    (call $read (i64.const 16) (i64.const 1200000) (i64.const 1))
    (i32.store8 (i32.const 24) (i32.load8_u (i32.const 1200000)))
    (i64.store (i32.const 32) (global.get $hooks))
    (call $append (i32.const 0) (i32.const 40))
    (call $reply)))"#;

async fn snapshot_metadata(
    pic: &pocket_ic::nonblocking::PocketIc,
    id: Principal,
) -> ic_platform::management_canister_types::ReadCanisterSnapshotMetadataResult {
    use ic_platform::{
        candid_codec::{
            decoder::{Limits, decode_one},
            encode_one,
        },
        management_canister_types::ReadCanisterSnapshotMetadataArgs,
    };
    let snapshot = pic.take_canister_snapshot(id, None, None).await.unwrap();
    let payload = encode_one(
        &ReadCanisterSnapshotMetadataArgs {
            canister_id: ic_platform_types::Principal::from_slice(id.as_slice()),
            snapshot_id: snapshot.id.clone(),
        },
        4096,
    )
    .unwrap();
    let bytes = pic
        .update_call_with_effective_principal(
            Principal::management_canister(),
            pocket_ic::common::rest::RawEffectivePrincipal::CanisterId(id.as_slice().to_vec()),
            Principal::anonymous(),
            "read_canister_snapshot_metadata",
            payload,
        )
        .await
        .unwrap();
    let metadata = decode_one(
        &bytes,
        Limits {
            bytes: 16_384,
            header: 4096,
            types: 128,
            work: 1_000_000,
            allocation: 1_000_000,
        },
    )
    .unwrap();
    pic.delete_canister_snapshot(id, None, snapshot.id)
        .await
        .unwrap();
    metadata
}

async fn timer_state(pic: &pocket_ic::nonblocking::PocketIc, id: Principal) -> Vec<u8> {
    pic.query_call(id, Principal::anonymous(), "state", vec![])
        .await
        .unwrap()
}

async fn mutate_timer_fixture(pic: &pocket_ic::nonblocking::PocketIc, id: Principal) {
    let deadline = pic.get_time().await.as_nanos_since_unix_epoch() + 120_000_000_000;
    pic.update_call(
        id,
        Principal::anonymous(),
        "mutate",
        deadline.to_le_bytes().to_vec(),
    )
    .await
    .unwrap();
    pic.clear_chunk_store(id, None).await.unwrap();
}

async fn build_timer_fixture() -> std::io::Result<ic_testkit::pic::StandaloneCanisterFixture> {
    use ic_testkit::pic::{InstallSpec, StandaloneCanisterFixture};
    let wasm = ic_host_artifacts::wasm_text::compile(
        TIMER_WAT,
        ic_host_artifacts::wasm_text::CompileLimits {
            source_bytes: 8192,
            module_bytes: 8192,
        },
    )
    .unwrap();
    let fixture = StandaloneCanisterFixture::install(
        support::pocket_ic().await,
        InstallSpec {
            wasm,
            init_bytes: vec![],
            cycles: 50_000_000_000_000,
            install_sender: None,
        },
    )
    .await;
    fixture
        .pocket_ic()
        .update_canister_settings(
            fixture.canister_id(),
            None,
            CanisterSettings {
                wasm_memory_limit: Some((4_u64 * 1024 * 1024).into()),
                wasm_memory_threshold: Some((1024_u64 * 1024).into()),
                ..CanisterSettings::default()
            },
        )
        .await
        .unwrap();
    fixture
        .pocket_ic()
        .upload_chunk(fixture.canister_id(), None, vec![37; 262_153])
        .await
        .unwrap();
    Ok(fixture)
}

#[tokio::test]
async fn explicit_baseline_restores_heap_timer_hook_and_memory_before_rearming() {
    use ic_testkit::pic::CachedPocketIcBaseline;
    use std::time::Duration;
    let fixture = build_timer_fixture().await.unwrap();
    let (owned, id) = fixture.into_parts();
    owned.instance().stop_canister(id, None).await.unwrap();
    let captured = CachedPocketIcBaseline::capture(
        owned,
        [CanisterSnapshotTarget {
            canister_id: id,
            sender: None,
        }],
        id,
    )
    .await
    .unwrap();
    let pic = captured.pocket_ic();
    let instance = pic.instance_id;
    pic.start_canister(id, None).await.unwrap();
    let initial = timer_state(pic, id).await;
    let chunks = pic.stored_chunks(id, None).await.unwrap();
    assert_eq!(chunks.len(), 1);
    assert_eq!(initial[16], 77);
    assert_eq!(initial[24], 77);
    pic.stop_canister(id, None).await.unwrap();
    let baseline = snapshot_metadata(pic, id).await;
    assert_eq!(baseline.certified_data, [77]);
    assert!(matches!(
        baseline.global_timer,
        Some(ic_platform::management_canister_types::CanisterTimer::Active(_))
    ));
    pic.start_canister(id, None).await.unwrap();
    for iteration in 0..3 {
        mutate_timer_fixture(pic, id).await;
        pic.stop_canister(id, None).await.unwrap();
        let changed = snapshot_metadata(pic, id).await;
        assert_eq!(changed.certified_data, [99]);
        assert_eq!(
            changed.wasm_chunk_store,
            [] as [ic_platform::management_canister_types::ChunkHash; 0]
        );
        assert_ne!(
            format!("{:?}", changed.global_timer),
            format!("{:?}", baseline.global_timer)
        );
        assert_ne!(
            format!("{:?}", changed.on_low_wasm_memory_hook_status),
            format!("{:?}", baseline.on_low_wasm_memory_hook_status)
        );
        captured
            .restore(SnapshotRestoreFunding::Preserve)
            .await
            .unwrap();
        pic.start_canister(id, None).await.unwrap();
        assert_eq!(pic.instance_id, instance, "restore, never rebuild");
        assert_eq!(*captured.metadata(), id);
        assert_eq!(timer_state(pic, id).await, initial);
        pic.stop_canister(id, None).await.unwrap();
        let restored = snapshot_metadata(pic, id).await;
        assert_eq!(restored.certified_data, baseline.certified_data);
        assert_eq!(pic.stored_chunks(id, None).await.unwrap(), chunks);
        assert_eq!(
            format!("{:?}", restored.global_timer),
            format!("{:?}", baseline.global_timer)
        );
        assert_eq!(
            format!("{:?}", restored.on_low_wasm_memory_hook_status),
            format!("{:?}", baseline.on_low_wasm_memory_hook_status)
        );
        pic.start_canister(id, None).await.unwrap();
        if iteration == 2 {
            pic.advance_time(Duration::from_secs(70)).await;
            for _ in 0..4 {
                pic.tick().await;
            }
            let fired = timer_state(pic, id).await;
            assert!(u64::from_le_bytes(fired[8..16].try_into().unwrap()) > 0);
            mutate_timer_fixture(pic, id).await;
        }
    }
    captured.close().await;
}

#[tokio::test]
async fn explicit_snapshot_senders_support_mixed_controller_sets_without_fallback() {
    let owned = support::pocket_ic().await;
    let pocket_ic = owned.instance();
    let explicit_controller = Principal::from_slice(&[42]);
    let anonymous_canister = pocket_ic.create_canister().await;
    let explicit_canister = pocket_ic
        .create_canister_with_settings(
            None,
            Some(CanisterSettings {
                controllers: Some(vec![explicit_controller]),
                ..CanisterSettings::default()
            }),
        )
        .await;
    for (canister_id, sender) in [
        (anonymous_canister, None),
        (explicit_canister, Some(explicit_controller)),
    ] {
        pocket_ic
            .install_canister(canister_id, b"\0asm\x01\0\0\0".to_vec(), vec![], sender)
            .await;
        pocket_ic
            .stop_canister(canister_id, sender)
            .await
            .expect("stop mixed-controller canister before snapshot capture");
    }

    let error = ic_testkit::pic::capture_snapshots(
        pocket_ic,
        [CanisterSnapshotTarget {
            canister_id: explicit_canister,
            sender: None,
        }],
    )
    .await
    .expect_err("incorrect explicit sender must fail without fallback");
    match error {
        ControllerSnapshotError::CaptureFailed {
            canister_id,
            sender,
            rejection,
            cleanup_failures,
        } => {
            assert_eq!(canister_id, explicit_canister);
            assert_eq!(sender, None);
            let response = *rejection;
            assert_ne!(response.reject_message, "");
            assert_eq!(cleanup_failures.len(), 0);
        }
        other => panic!("unexpected snapshot error: {other}"),
    }

    let snapshots = ic_testkit::pic::capture_snapshots(
        pocket_ic,
        [
            CanisterSnapshotTarget {
                canister_id: explicit_canister,
                sender: Some(explicit_controller),
            },
            CanisterSnapshotTarget {
                canister_id: anonymous_canister,
                sender: None,
            },
        ],
    )
    .await
    .expect("capture mixed-controller snapshots with exact senders");

    assert_eq!(snapshots.len(), 2);
    let mut expected = vec![anonymous_canister, explicit_canister];
    expected.sort();
    assert_eq!(snapshots.canister_ids().collect::<Vec<_>>(), expected);
    ic_testkit::pic::restore_snapshots(pocket_ic, &snapshots, SnapshotRestoreFunding::Preserve)
        .await
        .expect("restore mixed-controller snapshots with exact captured senders");

    pocket_ic
        .set_controllers(
            explicit_canister,
            Some(explicit_controller),
            vec![Principal::anonymous()],
        )
        .await
        .expect("replace the captured controller with anonymous authority");
    let error =
        ic_testkit::pic::restore_snapshots(pocket_ic, &snapshots, SnapshotRestoreFunding::Preserve)
            .await
            .expect_err("restore must retain the captured sender after controller changes");
    match error {
        ControllerSnapshotError::RestoreFailed {
            canister_id,
            sender,
            rejection,
        } => {
            assert_eq!(canister_id, explicit_canister);
            assert_eq!(sender, Some(explicit_controller));
            let response = *rejection;
            assert_ne!(response.reject_message, "");
        }
        other => panic!("unexpected restore failure: {other}"),
    }
}

#[tokio::test]
async fn duplicate_snapshot_inputs_are_rejected_before_any_capture() {
    let owned = support::pocket_ic().await;
    let pocket_ic = owned.instance();
    let canister_id = pocket_ic.create_canister().await;
    pocket_ic
        .install_canister(canister_id, b"\0asm\x01\0\0\0".to_vec(), vec![], None)
        .await;
    pocket_ic.stop_canister(canister_id, None).await.unwrap();
    let duplicate = Principal::from_slice(&[42]);

    let error = ic_testkit::pic::capture_snapshots(
        pocket_ic,
        [
            CanisterSnapshotTarget {
                canister_id,
                sender: None,
            },
            CanisterSnapshotTarget {
                canister_id: duplicate,
                sender: None,
            },
            CanisterSnapshotTarget {
                canister_id: duplicate,
                sender: Some(Principal::anonymous()),
            },
        ],
    )
    .await
    .unwrap_err();
    assert!(
        matches!(error, ControllerSnapshotError::DuplicateCanisterId { canister_id } if canister_id == duplicate)
    );
    assert!(
        pocket_ic
            .list_canister_snapshots(canister_id, None)
            .await
            .unwrap()
            .is_empty(),
        "duplicate validation must precede all snapshot creation",
    );
}

#[tokio::test]
async fn snapshot_capture_reports_first_failure_in_canister_order() {
    let owned = support::pocket_ic().await;
    let pocket_ic = owned.instance();
    let mut missing = [
        pocket_ic.create_canister().await,
        pocket_ic.create_canister().await,
    ];
    for canister_id in missing {
        pocket_ic.stop_canister(canister_id, None).await.unwrap();
        pocket_ic.delete_canister(canister_id, None).await.unwrap();
    }
    missing.sort();
    let reversed = [missing[1], missing[0]];
    let error = ic_testkit::pic::capture_snapshots(
        pocket_ic,
        reversed.map(|id| CanisterSnapshotTarget {
            canister_id: id,
            sender: None,
        }),
    )
    .await
    .unwrap_err();
    match error {
        ControllerSnapshotError::CaptureFailed { canister_id, .. } => {
            assert_eq!(canister_id, missing[0]);
        }
        other => panic!("unexpected snapshot error: {other}"),
    }
}

#[tokio::test]
async fn failed_snapshot_set_capture_cleans_up_earlier_snapshots() {
    let owned = support::pocket_ic().await;
    let pocket_ic = owned.instance();
    let captured_first = pocket_ic.create_canister().await;
    pocket_ic
        .install_canister(captured_first, b"\0asm\x01\0\0\0".to_vec(), vec![], None)
        .await;
    pocket_ic
        .stop_canister(captured_first, None)
        .await
        .expect("stop canister before snapshot capture");

    let missing_canister = pocket_ic.create_canister().await;
    pocket_ic
        .stop_canister(missing_canister, None)
        .await
        .expect("stop canister before deletion");
    pocket_ic
        .delete_canister(missing_canister, None)
        .await
        .expect("delete canister used for capture failure");
    assert!(captured_first < missing_canister);

    let error = ic_testkit::pic::capture_snapshots(
        pocket_ic,
        [captured_first, missing_canister].map(|id| CanisterSnapshotTarget {
            canister_id: id,
            sender: None,
        }),
    )
    .await
    .expect_err("missing canister should fail the snapshot set");

    match error {
        ControllerSnapshotError::CaptureFailed {
            canister_id,
            sender,
            rejection,
            cleanup_failures,
        } => {
            assert_eq!(canister_id, missing_canister);
            assert_eq!(sender, None);
            let response = *rejection;
            assert_ne!(response.reject_message, "");
            assert_eq!(cleanup_failures.len(), 0);
        }
        other => panic!("unexpected snapshot error: {other}"),
    }

    let remaining = pocket_ic
        .list_canister_snapshots(captured_first, None)
        .await
        .expect("list snapshots after rollback");
    assert_eq!(remaining, [] as [pocket_ic::Snapshot; 0]);
}

#[tokio::test]
async fn snapshot_limit_refusal_preserves_existing_snapshots_and_can_recover() {
    let owned = support::pocket_ic().await;
    let pic = owned.instance();
    let id = pic.create_canister().await;
    pic.install_canister(id, b"\0asm\x01\0\0\0".to_vec(), vec![], None)
        .await;
    pic.stop_canister(id, None).await.unwrap();
    // Coherent preparation needs source and upload slots. Refusal must preserve
    // caller snapshots even when taking the temporary source already succeeded.
    let mut original = Vec::new();
    for _ in 0..10 {
        original.push(pic.take_canister_snapshot(id, None, None).await.unwrap().id);
    }
    let target = CanisterSnapshotTarget {
        canister_id: id,
        sender: None,
    };
    let error = ic_testkit::pic::capture_snapshots(pic, [target])
        .await
        .unwrap_err();
    assert!(
        matches!(error, ControllerSnapshotError::CaptureFailed { rejection, cleanup_failures, .. }
        if !rejection.reject_message.is_empty() && cleanup_failures.is_empty())
    );
    let mut remaining = pic
        .list_canister_snapshots(id, None)
        .await
        .unwrap()
        .into_iter()
        .map(|s| s.id)
        .collect::<Vec<_>>();
    original.sort();
    remaining.sort();
    assert_eq!(remaining, original);
    pic.delete_canister_snapshot(id, None, original.remove(0))
        .await
        .unwrap();
    ic_testkit::pic::capture_snapshots(pic, [target])
        .await
        .unwrap_err();
    assert_eq!(
        pic.list_canister_snapshots(id, None).await.unwrap().len(),
        9
    );
    pic.delete_canister_snapshot(id, None, original.remove(0))
        .await
        .unwrap();
    let snapshots = ic_testkit::pic::capture_snapshots(pic, [target])
        .await
        .unwrap();
    ic_testkit::pic::restore_snapshots(pic, &snapshots, SnapshotRestoreFunding::Preserve)
        .await
        .unwrap();
    assert_eq!(
        pic.list_canister_snapshots(id, None).await.unwrap().len(),
        9
    );
}

#[tokio::test]
async fn snapshot_restore_only_adds_cycles_when_explicitly_requested() {
    let owned = support::pocket_ic().await;
    let pocket_ic = owned.instance();
    let canister_id = pocket_ic.create_canister().await;
    pocket_ic
        .install_canister(canister_id, b"\0asm\x01\0\0\0".to_vec(), vec![], None)
        .await;
    pocket_ic
        .stop_canister(canister_id, None)
        .await
        .expect("stop canister before snapshot capture");

    let snapshots = ic_testkit::pic::capture_snapshots(
        pocket_ic,
        [CanisterSnapshotTarget {
            canister_id,
            sender: None,
        }],
    )
    .await
    .expect("capture snapshot set");
    let balance_before_restore = pocket_ic.cycle_balance(canister_id).await;

    ic_testkit::pic::restore_snapshots(pocket_ic, &snapshots, SnapshotRestoreFunding::Preserve)
        .await
        .expect("restore without funding");
    let balance_after_preserved_restore = pocket_ic.cycle_balance(canister_id).await;
    assert!(
        balance_after_preserved_restore <= balance_before_restore,
        "preserve funding must not add cycles"
    );

    let minimum_cycles = balance_after_preserved_restore + 10_000_000_000_000;
    ic_testkit::pic::restore_snapshots(
        pocket_ic,
        &snapshots,
        SnapshotRestoreFunding::TopUpTo { minimum_cycles },
    )
    .await
    .expect("restore with explicit funding");
    assert!(
        pocket_ic.cycle_balance(canister_id).await > balance_after_preserved_restore,
        "explicit top-up should increase the post-restore balance"
    );

    let balance_before_satisfied_restore = pocket_ic.cycle_balance(canister_id).await;
    ic_testkit::pic::restore_snapshots(
        pocket_ic,
        &snapshots,
        SnapshotRestoreFunding::TopUpTo { minimum_cycles: 0 },
    )
    .await
    .expect("restore with an already satisfied funding minimum");
    assert!(
        pocket_ic.cycle_balance(canister_id).await <= balance_before_satisfied_restore,
        "an already satisfied funding minimum must not add cycles"
    );
}
