#[test]
fn cleanup_preserves_causes_and_senders_and_continues_after_each_failure() {
    use super::{ControllerSnapshot, SnapshotCleanupFailure, cleanup_captured_snapshots};
    use crate::pocket_ic::{ClientError, ErrorCode, RejectCode, RejectResponse};
    use ic_platform_types::principal::ExternalPrincipal as Principal;

    let ids = [1, 2, 3, 4].map(|byte| Principal::from_slice(&[byte]));
    let sender = Some(Principal::anonymous());
    let snapshots = ids
        .map(|id| {
            (
                id,
                ControllerSnapshot {
                    snapshot_id: vec![7],
                    sender,
                },
            )
        })
        .into_iter()
        .collect::<std::collections::BTreeMap<_, _>>();
    let rejection = RejectResponse {
        reject_code: RejectCode::CanisterReject,
        reject_message: "cleanup rejected".into(),
        error_code: ErrorCode::CanisterCalledTrap,
        certified: false,
    };
    let mut calls = Vec::new();
    let failures = cleanup_captured_snapshots(snapshots, |id, selected_sender, snapshot_id| {
        assert_eq!(snapshot_id, [7]);
        calls.push((id, selected_sender));
        if id == ids[1] {
            return Err(ClientError::Canister(rejection.clone()));
        }
        if id == ids[2] {
            return Err(ClientError::Protocol("synthetic cleanup failure".into()));
        }
        Ok(())
    });
    assert_eq!(calls, ids.map(|id| (id, sender)));
    let [
        SnapshotCleanupFailure {
            canister_id: rejected_id,
            sender: rejected_sender,
            source: rejected,
            snapshot_id: rejected_snapshot,
        },
        SnapshotCleanupFailure {
            canister_id: panicked_id,
            sender: panicked_sender,
            source: panicked,
            snapshot_id: panicked_snapshot,
        },
    ] = failures.as_slice()
    else {
        panic!("expected both cleanup failures: {failures:?}");
    };
    assert_eq!(*rejected_id, ids[1]);
    assert_eq!(rejected_snapshot, &[7]);
    assert_eq!(panicked_snapshot, &[7]);
    assert_eq!(*rejected_sender, sender);
    let ClientError::Canister(response) = rejected.as_ref() else {
        panic!("expected rejection: {rejected:?}");
    };
    assert_eq!(response, &rejection);
    assert_eq!(*panicked_id, ids[2]);
    assert_eq!(*panicked_sender, sender);
    let ClientError::Protocol(source) = panicked.as_ref() else {
        panic!("expected operation failure: {panicked:?}");
    };
    assert_eq!(source, "synthetic cleanup failure");
}
