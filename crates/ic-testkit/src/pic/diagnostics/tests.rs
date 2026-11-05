use std::cell::Cell;

use crate::pocket_ic::CanisterLogRecord;
use ic_host_artifacts::batch::BatchLabelError;
use ic_platform_types::principal::ExternalPrincipal as Principal;

use super::{
    CanisterDiagnosticsReport, CanisterDiagnosticsRequest, CanisterLogRenderLimits,
    LabeledCanisterDiagnosticsRequest, PocketIcDiagnosticsExt, render_log_records,
};

struct Reporting {
    calls: Cell<usize>,
    request: Cell<Option<CanisterDiagnosticsRequest>>,
}

impl PocketIcDiagnosticsExt for Reporting {
    fn collect_canister_diagnostics(
        &self,
        request: CanisterDiagnosticsRequest,
    ) -> impl std::future::Future<Output = CanisterDiagnosticsReport> {
        self.calls.set(self.calls.get() + 1);
        self.request.set(Some(request));
        std::future::ready(CanisterDiagnosticsReport {
            request,
            status: Err(crate::pocket_ic::ClientError::Protocol(
                "synthetic status failure".into(),
            )),
            logs: Err(crate::pocket_ic::ClientError::Protocol(
                "synthetic log failure".into(),
            )),
        })
    }
}

#[tokio::test]
async fn explicit_reporting_preserves_requests_and_writer_failures() {
    let collector = Reporting {
        calls: Cell::new(0),
        request: Cell::new(None),
    };
    let request = CanisterDiagnosticsRequest {
        canister_id: Principal::from_slice(&[1]),
        status_sender: Principal::from_slice(&[2]),
        log_sender: Principal::from_slice(&[3]),
        log_limits: CanisterLogRenderLimits::default(),
    };
    let mut output = Vec::new();
    collector
        .write_canister_diagnostics(request, "install", &mut output)
        .await
        .unwrap();
    let text = String::from_utf8(output).unwrap();
    assert!(text.contains("synthetic status failure") && text.contains("synthetic log failure"));
    let error = collector
        .write_canister_diagnostics(request, "install", &mut std::io::Cursor::new([0_u8; 0]))
        .await
        .unwrap_err();
    assert_eq!(error.kind(), std::io::ErrorKind::WriteZero);
    assert_eq!(collector.calls.get(), 2);
    assert_eq!(collector.request.get(), Some(request));
}

#[tokio::test]
async fn labeled_batch_retains_order_and_continues_after_entry_failure() {
    let collector = Reporting {
        calls: Cell::new(0),
        request: Cell::new(None),
    };
    let first = CanisterDiagnosticsRequest {
        canister_id: Principal::from_slice(&[1]),
        status_sender: Principal::from_slice(&[2]),
        log_sender: Principal::from_slice(&[3]),
        log_limits: CanisterLogRenderLimits::default(),
    };
    let second = CanisterDiagnosticsRequest {
        canister_id: Principal::from_slice(&[4]),
        status_sender: Principal::from_slice(&[5]),
        log_sender: Principal::from_slice(&[6]),
        log_limits: CanisterLogRenderLimits::default(),
    };
    let report = collector
        .collect_canister_diagnostics_batch(&[
            LabeledCanisterDiagnosticsRequest {
                label: "root".into(),
                request: first,
            },
            LabeledCanisterDiagnosticsRequest {
                label: "worker".into(),
                request: second,
            },
        ])
        .await
        .expect("valid labeled diagnostics batch");

    assert_eq!(collector.calls.get(), 2);
    assert_eq!(report.entries.len(), 2);
    assert_eq!(report.entries[0].label, "root");
    assert_eq!(report.entries[0].report.request, first);
    assert_eq!(report.entries[1].label, "worker");
    assert_eq!(report.entries[1].report.request, second);
    assert!(
        report
            .entries
            .iter()
            .all(|entry| entry.entry_elapsed <= report.total)
    );
    assert_eq!(report.failures().count(), 2);
    assert!(!report.is_success());
    let compact = report.to_string();
    assert!(compact.contains("label=\"root\""));
    assert!(compact.contains("label=\"worker\""));
    assert!(compact.contains("synthetic status failure"));
}

#[tokio::test]
async fn diagnostic_batch_rejects_invalid_labels_before_collection() {
    let collector = Reporting {
        calls: Cell::new(0),
        request: Cell::new(None),
    };
    let request = CanisterDiagnosticsRequest {
        canister_id: Principal::from_slice(&[1]),
        status_sender: Principal::from_slice(&[2]),
        log_sender: Principal::from_slice(&[3]),
        log_limits: CanisterLogRenderLimits::default(),
    };
    let empty = collector
        .collect_canister_diagnostics_batch(&[LabeledCanisterDiagnosticsRequest {
            label: String::new(),
            request,
        }])
        .await
        .expect_err("empty label must reject diagnostics batch");
    assert_eq!(empty, BatchLabelError::Empty { index: 0 });
    assert_eq!(collector.calls.get(), 0);

    let duplicate = collector
        .collect_canister_diagnostics_batch(&[
            LabeledCanisterDiagnosticsRequest {
                label: "same".into(),
                request,
            },
            LabeledCanisterDiagnosticsRequest {
                label: "same".into(),
                request,
            },
        ])
        .await
        .expect_err("duplicate labels must reject diagnostics batch");
    assert_eq!(
        duplicate,
        BatchLabelError::Duplicate {
            label: "same".to_owned(),
            first_index: 0,
            duplicate_index: 1,
        }
    );
    assert_eq!(collector.calls.get(), 0);
}

#[test]
fn log_rendering_is_bounded_lossy_utf8_and_reports_truncation() {
    let logs = render_log_records(
        vec![
            CanisterLogRecord {
                idx: 7,
                timestamp_nanos: 11,
                content: vec![b'f', 0x80, b'o'],
            },
            CanisterLogRecord {
                idx: 8,
                timestamp_nanos: 12,
                content: b"bar".to_vec(),
            },
        ],
        CanisterLogRenderLimits {
            record_limit: 1,
            byte_limit: 2,
        },
    );

    assert_eq!(logs.total_records(), 2);
    assert_eq!(logs.total_content_bytes(), 6);
    assert_eq!(logs.omitted_records(), 1);
    assert_eq!(logs.omitted_content_bytes(), 4);
    assert!(logs.was_truncated());
    assert_eq!(logs.records().len(), 1);
    assert_eq!(logs.records()[0].content(), "f�");
    assert_eq!(logs.records()[0].original_content_bytes(), 3);
    assert_eq!(logs.records()[0].omitted_content_bytes(), 1);
    assert!(logs.records()[0].was_truncated());
    let rendered = logs.to_string();
    assert!(rendered.contains("f�"));
    assert!(rendered.contains("truncated omitted_records=1 omitted_content_bytes=4"));
}

#[test]
fn zero_log_bounds_retain_only_aggregate_truncation() {
    for (record_limit, byte_limit) in [(0, 0), (0, 16), (16, 0)] {
        let logs = render_log_records(
            vec![CanisterLogRecord {
                idx: 1,
                timestamp_nanos: 2,
                content: b"hello".to_vec(),
            }],
            CanisterLogRenderLimits {
                record_limit,
                byte_limit,
            },
        );

        assert_eq!(logs.records(), []);
        assert_eq!(logs.total_records(), 1);
        assert_eq!(logs.total_content_bytes(), 5);
        assert_eq!(logs.omitted_records(), 1);
        assert_eq!(logs.omitted_content_bytes(), 5);
        assert!(logs.was_truncated());
        assert_eq!(
            logs.to_string(),
            "<no retained records>; truncated omitted_records=1 omitted_content_bytes=5"
        );
    }
}

#[test]
fn logs_within_bounds_preserve_empty_records_and_raw_byte_totals() {
    let limits = CanisterLogRenderLimits {
        record_limit: 3,
        byte_limit: 3,
    };
    let empty = render_log_records(vec![], limits);
    assert_eq!(empty.total_records(), 0);
    assert_eq!(empty.total_content_bytes(), 0);
    assert_eq!(empty.omitted_records(), 0);
    assert_eq!(empty.omitted_content_bytes(), 0);
    assert!(!empty.was_truncated());
    assert_eq!(empty.to_string(), "<empty>");

    let logs = render_log_records(
        vec![
            CanisterLogRecord {
                idx: 1,
                timestamp_nanos: 2,
                content: vec![],
            },
            CanisterLogRecord {
                idx: 3,
                timestamp_nanos: 4,
                content: vec![0x80, b'a'],
            },
            CanisterLogRecord {
                idx: 5,
                timestamp_nanos: 6,
                content: vec![],
            },
        ],
        limits,
    );
    assert_eq!(logs.total_records(), 3);
    assert_eq!(logs.records().len(), 3);
    assert_eq!(logs.total_content_bytes(), 2);
    assert_eq!(logs.records()[0].content(), "");
    assert_eq!(logs.records()[1].content(), "�a");
    assert_eq!(logs.records()[2].content(), "");
    assert_eq!(logs.omitted_records(), 0);
    assert_eq!(logs.omitted_content_bytes(), 0);
    assert!(!logs.was_truncated());
}
