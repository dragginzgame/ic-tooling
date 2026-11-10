//! Integration checks for pocket ic diagnostics behavior.
mod support;

use ic_testkit::pic::{
    CanisterDiagnosticsRequest, CanisterLogRenderLimits, LabeledCanisterDiagnosticsRequest,
};
use pocket_ic::CanisterId as Principal;
use pocket_ic::CanisterSettings;

#[tokio::test]
async fn diagnostics_use_independent_exact_senders_and_preserve_both_outcomes() {
    let owned = support::pocket_ic().await;
    let pocket_ic = owned.instance();
    let status_sender = Principal::from_slice(&[41]);
    let log_sender = Principal::from_slice(&[42]);
    let outsider = Principal::from_slice(&[43]);
    let canister_id = pocket_ic
        .create_canister_with_settings(
            None,
            Some(CanisterSettings {
                controllers: Some(vec![status_sender, log_sender]),
                ..CanisterSettings::default()
            }),
        )
        .await;

    let request = CanisterDiagnosticsRequest {
        canister_id,
        status_sender,
        log_sender,
        log_limits: CanisterLogRenderLimits::default(),
    };
    let report = ic_testkit::pic::collect_canister_diagnostics(pocket_ic, request).await;

    assert_eq!(report.request, request);
    assert!(report.status.is_ok(), "{report}");
    assert!(report.logs.is_ok(), "{report}");
    let compact = report.to_string();
    assert!(compact.contains("status=ok(state="));
    assert!(compact.contains("logs=<empty>"));

    let batch = ic_testkit::pic::collect_canister_diagnostics_batch(
        pocket_ic,
        &[
            LabeledCanisterDiagnosticsRequest {
                label: "denied".into(),
                request: CanisterDiagnosticsRequest {
                    canister_id,
                    status_sender: outsider,
                    log_sender: outsider,
                    log_limits: CanisterLogRenderLimits::default(),
                },
            },
            LabeledCanisterDiagnosticsRequest {
                label: "controller".into(),
                request,
            },
        ],
    )
    .await
    .expect("valid labeled diagnostics batch");
    assert_eq!(batch.entries.len(), 2);
    assert_eq!(batch.entries[0].label, "denied");
    assert_eq!(batch.entries[1].label, "controller");
    assert!(
        batch
            .entries
            .iter()
            .all(|entry| entry.entry_elapsed <= batch.total)
    );
    let denied = &batch.entries[0].report;
    assert!(denied.status.is_err());
    assert!(denied.logs.is_err());
    assert!(batch.entries[1].is_success());
    assert_eq!(batch.failures().count(), 1);
}
