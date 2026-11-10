use crate::test_support::http::Peer;
use pocket_ic::nonblocking::PocketIc;
use std::{io::Write, time::Duration};

use ic_host_artifacts::batch::BatchLabelError;
use pocket_ic::CanisterId as Principal;
use pocket_ic::CanisterLogRecord;

use super::{
    CanisterDiagnosticsRequest, CanisterLogRenderLimits, LabeledCanisterDiagnosticsRequest,
    render_log_records,
};

#[tokio::test]
async fn explicit_reporting_preserves_requests_and_writer_failures() {
    let request = CanisterDiagnosticsRequest {
        canister_id: Principal::from_slice(&[1]),
        status_sender: Principal::from_slice(&[2]),
        log_sender: Principal::from_slice(&[3]),
        log_limits: CanisterLogRenderLimits::default(),
    };
    let (collector, peer) = reporting_peer(vec![request, request]);
    let mut output = Vec::new();
    crate::pic::write_canister_diagnostics(&collector, request, "install", &mut output)
        .await
        .unwrap();
    let text = String::from_utf8(output).unwrap();
    assert!(text.contains("synthetic status failure") && text.contains("synthetic log failure"));
    let error = crate::pic::write_canister_diagnostics(
        &collector,
        request,
        "install",
        &mut std::io::Cursor::new([0_u8; 0]),
    )
    .await
    .unwrap_err();
    assert_eq!(error.kind(), std::io::ErrorKind::WriteZero);
    peer.finish();
}

#[tokio::test]
async fn labeled_batch_retains_order_and_continues_after_entry_failure() {
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
    let (collector, peer) = reporting_peer(vec![first, second]);
    let report = crate::pic::collect_canister_diagnostics_batch(
        &collector,
        &[
            LabeledCanisterDiagnosticsRequest {
                label: "root".into(),
                request: first,
            },
            LabeledCanisterDiagnosticsRequest {
                label: "worker".into(),
                request: second,
            },
        ],
    )
    .await
    .expect("valid labeled diagnostics batch");

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
    peer.finish();
}

#[tokio::test]
async fn diagnostic_batch_rejects_invalid_labels_before_collection() {
    let request = CanisterDiagnosticsRequest {
        canister_id: Principal::from_slice(&[1]),
        status_sender: Principal::from_slice(&[2]),
        log_sender: Principal::from_slice(&[3]),
        log_limits: CanisterLogRenderLimits::default(),
    };
    let (collector, peer) = reporting_peer(vec![]);
    let empty = crate::pic::collect_canister_diagnostics_batch(
        &collector,
        &[LabeledCanisterDiagnosticsRequest {
            label: String::new(),
            request,
        }],
    )
    .await
    .expect_err("empty label must reject diagnostics batch");
    assert_eq!(empty, BatchLabelError::Empty { index: 0 });

    let duplicate = crate::pic::collect_canister_diagnostics_batch(
        &collector,
        &[
            LabeledCanisterDiagnosticsRequest {
                label: "same".into(),
                request,
            },
            LabeledCanisterDiagnosticsRequest {
                label: "same".into(),
                request,
            },
        ],
    )
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
    peer.finish();
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

fn reporting_peer(requests: Vec<CanisterDiagnosticsRequest>) -> (PocketIc, Peer) {
    let mut expected = requests
        .into_iter()
        .flat_map(|request| {
            [
                (
                    "canister_status",
                    request.status_sender,
                    "synthetic status failure",
                ),
                (
                    "fetch_canister_logs",
                    request.log_sender,
                    "synthetic log failure",
                ),
            ]
        })
        .collect::<Vec<_>>()
        .into_iter();
    let peer = Peer::spawn(
        Some(expected.len()),
        Duration::from_millis(30),
        ic_host_process::testing::http::HttpFixtureLimits {
            timeout: Duration::from_secs(5),
            max_header_bytes: 16_384,
            max_body_bytes: 16_384,
        },
        move |stream, request, _, _| {
            let bytes = request.into_result().unwrap();
            let request = crate::test_support::http::Request::new(&bytes);
            let call: pocket_ic::common::rest::RawCanisterCall =
                serde_json::from_slice(request.body).unwrap();
            let (method, sender, message) = expected.next().expect("no replay");
            assert_eq!(call.method, method);
            assert_eq!(call.sender, sender.as_slice());
            let body = serde_json::json!({"Err": {"reject_code":"CanisterReject", "reject_message": message, "error_code":"CanisterRejectedMessage", "certified":false}}).to_string();
            write!(stream, "HTTP/1.1 200 OK\r\nContent-Type: application/json\r\nContent-Length: {}\r\nConnection: close\r\n\r\n{body}", body.len()).unwrap();
        },
    );
    let client = PocketIc::new_from_existing_instance(peer.url().clone(), 0, Some(1000));
    (client, peer)
}

#[test]
fn log_truncation_preserves_valid_and_split_utf8_prefixes() {
    for (limit, expected) in [(1, "a"), (2, "a�"), (3, "aé"), (4, "aéb")] {
        let logs = render_log_records(
            vec![CanisterLogRecord {
                idx: 1,
                timestamp_nanos: 2,
                content: "aéb".as_bytes().to_vec(),
            }],
            CanisterLogRenderLimits {
                record_limit: 1,
                byte_limit: limit,
            },
        );
        assert_eq!(logs.records()[0].content(), expected);
        assert_eq!(logs.records()[0].original_content_bytes(), 4);
        assert_eq!(logs.omitted_content_bytes(), 4 - limit);
    }
}
