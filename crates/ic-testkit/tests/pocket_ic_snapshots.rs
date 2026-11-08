//! Integration checks for pocket ic snapshots behavior.
use ic_testkit::pocket_ic::ClientError;
mod support;

use ic_platform_types::principal::ExternalPrincipal as Principal;
use ic_testkit::pic::{CanisterSnapshotTarget, ControllerSnapshotError, SnapshotRestoreFunding};
use ic_testkit::pocket_ic::CanisterSettings;

#[tokio::test]
async fn explicit_snapshot_senders_support_mixed_controller_sets_without_fallback() {
    let owned = support::pocket_ic().await;
    let pocket_ic = owned.instance();
    let explicit_controller = Principal::from_slice(&[42]);
    let anonymous_canister = pocket_ic.create_canister().await.unwrap();
    let explicit_canister = pocket_ic
        .create_canister_with_settings(
            None,
            Some(CanisterSettings {
                controllers: Some(vec![explicit_controller]),
                ..CanisterSettings::default()
            }),
        )
        .await
        .unwrap();
    for (canister_id, sender) in [
        (anonymous_canister, None),
        (explicit_canister, Some(explicit_controller)),
    ] {
        pocket_ic
            .install_canister(canister_id, b"\0asm\x01\0\0\0".to_vec(), vec![], sender)
            .await
            .unwrap();
        pocket_ic
            .stop_canister(canister_id, sender)
            .await
            .expect("stop mixed-controller canister before snapshot capture");
    }

    let error = pocket_ic
        .capture_snapshots([CanisterSnapshotTarget {
            canister_id: explicit_canister,
            sender: None,
        }])
        .await
        .expect_err("incorrect explicit sender must fail without fallback");
    match error {
        ControllerSnapshotError::CaptureFailed {
            canister_id,
            sender,
            source,
            cleanup_failures,
        } => {
            assert_eq!(canister_id, explicit_canister);
            assert_eq!(sender, None);
            let ClientError::Canister(response) = *source else {
                panic!("expected a complete rejection");
            };
            assert_ne!(response.reject_message, "");
            assert_eq!(cleanup_failures.len(), 0);
        }
        other => panic!("unexpected snapshot error: {other}"),
    }

    let snapshots = pocket_ic
        .capture_snapshots([
            CanisterSnapshotTarget {
                canister_id: explicit_canister,
                sender: Some(explicit_controller),
            },
            CanisterSnapshotTarget {
                canister_id: anonymous_canister,
                sender: None,
            },
        ])
        .await
        .expect("capture mixed-controller snapshots with exact senders");

    assert_eq!(snapshots.len(), 2);
    let mut expected = vec![anonymous_canister, explicit_canister];
    expected.sort();
    assert_eq!(snapshots.canister_ids().collect::<Vec<_>>(), expected);
    pocket_ic
        .restore_snapshots(&snapshots, SnapshotRestoreFunding::Preserve)
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
    let error = pocket_ic
        .restore_snapshots(&snapshots, SnapshotRestoreFunding::Preserve)
        .await
        .expect_err("restore must retain the captured sender after controller changes");
    match error {
        ControllerSnapshotError::RestoreFailed {
            canister_id,
            sender,
            source,
        } => {
            assert_eq!(canister_id, explicit_canister);
            assert_eq!(sender, Some(explicit_controller));
            let ClientError::Canister(response) = *source else {
                panic!("expected a complete rejection");
            };
            assert_ne!(response.reject_message, "");
        }
        other => panic!("unexpected restore failure: {other}"),
    }
}

#[tokio::test]
async fn duplicate_snapshot_inputs_are_rejected_before_any_capture() {
    let owned = support::pocket_ic().await;
    let pocket_ic = owned.instance();
    let canister_id = pocket_ic.create_canister().await.unwrap();
    pocket_ic
        .install_canister(canister_id, b"\0asm\x01\0\0\0".to_vec(), vec![], None)
        .await
        .unwrap();
    pocket_ic.stop_canister(canister_id, None).await.unwrap();
    let duplicate = Principal::from_slice(&[42]);

    let error = pocket_ic
        .capture_snapshots([
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
        ])
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
        pocket_ic.create_canister().await.unwrap(),
        pocket_ic.create_canister().await.unwrap(),
    ];
    for canister_id in missing {
        pocket_ic.stop_canister(canister_id, None).await.unwrap();
        pocket_ic.delete_canister(canister_id, None).await.unwrap();
    }
    missing.sort();
    let reversed = [missing[1], missing[0]];
    let error = pocket_ic
        .capture_snapshots(reversed.map(|id| CanisterSnapshotTarget {
            canister_id: id,
            sender: None,
        }))
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
    let captured_first = pocket_ic.create_canister().await.unwrap();
    pocket_ic
        .install_canister(captured_first, b"\0asm\x01\0\0\0".to_vec(), vec![], None)
        .await
        .unwrap();
    pocket_ic
        .stop_canister(captured_first, None)
        .await
        .expect("stop canister before snapshot capture");

    let missing_canister = pocket_ic.create_canister().await.unwrap();
    pocket_ic
        .stop_canister(missing_canister, None)
        .await
        .expect("stop canister before deletion");
    pocket_ic
        .delete_canister(missing_canister, None)
        .await
        .expect("delete canister used for capture failure");
    assert!(captured_first < missing_canister);

    let error = pocket_ic
        .capture_snapshots(
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
            source,
            cleanup_failures,
        } => {
            assert_eq!(canister_id, missing_canister);
            assert_eq!(sender, None);
            let ClientError::Canister(response) = *source else {
                panic!("expected a complete rejection");
            };
            assert_ne!(response.reject_message, "");
            assert_eq!(cleanup_failures.len(), 0);
        }
        other => panic!("unexpected snapshot error: {other}"),
    }

    let remaining = pocket_ic
        .list_canister_snapshots(captured_first, None)
        .await
        .expect("list snapshots after rollback");
    assert_eq!(remaining, [] as [ic_testkit::pocket_ic::Snapshot; 0]);
}

#[tokio::test]
async fn upload_refusal_removes_only_owned_temporary_snapshots_and_can_recover() {
    let owned = support::pocket_ic().await;
    let pic = owned.instance();
    let id = pic.create_canister().await.unwrap();
    pic.install_canister(id, b"\0asm\x01\0\0\0".to_vec(), vec![], None)
        .await
        .unwrap();
    pic.stop_canister(id, None).await.unwrap();
    // PocketIC 16.1's ten-snapshot limit leaves room for the temporary take,
    // then refuses metadata upload. Existing caller snapshots must survive.
    let mut original = Vec::new();
    for _ in 0..9 {
        original.push(pic.take_canister_snapshot(id, None, None).await.unwrap().id);
    }
    let target = CanisterSnapshotTarget {
        canister_id: id,
        sender: None,
    };
    let error = pic.capture_snapshots([target]).await.unwrap_err();
    assert!(
        matches!(error, ControllerSnapshotError::CaptureFailed { source, cleanup_failures, .. }
        if matches!(*source, ClientError::Canister(_)) && cleanup_failures.is_empty())
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
    let snapshots = pic.capture_snapshots([target]).await.unwrap();
    pic.restore_snapshots(&snapshots, SnapshotRestoreFunding::Preserve)
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
    let canister_id = pocket_ic.create_canister().await.unwrap();
    pocket_ic
        .install_canister(canister_id, b"\0asm\x01\0\0\0".to_vec(), vec![], None)
        .await
        .unwrap();
    pocket_ic
        .stop_canister(canister_id, None)
        .await
        .expect("stop canister before snapshot capture");

    let snapshots = pocket_ic
        .capture_snapshots([CanisterSnapshotTarget {
            canister_id,
            sender: None,
        }])
        .await
        .expect("capture snapshot set");
    let balance_before_restore = pocket_ic.cycle_balance(canister_id).await.unwrap();

    pocket_ic
        .restore_snapshots(&snapshots, SnapshotRestoreFunding::Preserve)
        .await
        .expect("restore without funding");
    let balance_after_preserved_restore = pocket_ic.cycle_balance(canister_id).await.unwrap();
    assert!(
        balance_after_preserved_restore <= balance_before_restore,
        "preserve funding must not add cycles"
    );

    let minimum_cycles = balance_after_preserved_restore + 10_000_000_000_000;
    pocket_ic
        .restore_snapshots(
            &snapshots,
            SnapshotRestoreFunding::TopUpTo { minimum_cycles },
        )
        .await
        .expect("restore with explicit funding");
    assert!(
        pocket_ic.cycle_balance(canister_id).await.unwrap() > balance_after_preserved_restore,
        "explicit top-up should increase the post-restore balance"
    );

    let balance_before_satisfied_restore = pocket_ic.cycle_balance(canister_id).await.unwrap();
    pocket_ic
        .restore_snapshots(
            &snapshots,
            SnapshotRestoreFunding::TopUpTo { minimum_cycles: 0 },
        )
        .await
        .expect("restore with an already satisfied funding minimum");
    assert!(
        pocket_ic.cycle_balance(canister_id).await.unwrap() <= balance_before_satisfied_restore,
        "an already satisfied funding minimum must not add cycles"
    );
}
