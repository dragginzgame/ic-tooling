use super::*;
use pocket_ic::{ErrorCode, RejectCode};
#[tokio::test]
async fn cleanup_preserves_exact_id_sender_and_all_management_rejections() {
    let ids = [1, 2, 3, 4].map(|x| Principal::from_slice(&[x]));
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
        .collect();
    let mut calls = Vec::new();
    let failures = cleanup_captured_snapshots(snapshots, |id, caller, snapshot| {
        calls.push((id, caller, snapshot));
        std::future::ready(if id == ids[1] || id == ids[2] {
            Err(RejectResponse {
                reject_code: RejectCode::CanisterReject,
                reject_message: format!("refused {id}"),
                error_code: ErrorCode::CanisterRejectedMessage,
                certified: false,
            })
        } else {
            Ok(())
        })
    })
    .await;
    assert_eq!(calls, ids.map(|id| (id, sender, vec![7])));
    assert_eq!(failures.len(), 2);
    for (failure, id) in failures.iter().zip([ids[1], ids[2]]) {
        assert_eq!(failure.canister_id, id);
        assert_eq!(failure.sender, sender);
        assert_eq!(failure.snapshot_id, [7]);
        assert_eq!(failure.rejection.reject_message, format!("refused {id}"));
    }
}
