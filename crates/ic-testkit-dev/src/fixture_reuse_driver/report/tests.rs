use super::{MeasuredRun, RssMeasurements, distribution, summarize};
use crate::fixture_reuse_driver::{Error, arguments::parse_arguments};
use serde_json::{Map, Value, json};
use std::path::PathBuf;

#[test]
fn empty_runs_are_rejected_before_summary_publication() {
    assert!(matches!(summarize(&[]), Err(Error::InvalidData(_))));
}

fn worker_report() -> Map<String, Value> {
    serde_json::to_value(crate::fixture_reuse_report::worker_report_fixture(1))
        .unwrap()
        .as_object()
        .unwrap()
        .clone()
}

#[test]
fn summary_preserves_null_phases_raw_data_and_percentiles() {
    let options = parse_arguments(
        [
            "--server",
            "/unused",
            "--modes",
            "fresh",
            "--workers",
            "1",
            "--iterations",
            "1",
            "--state-bytes",
            "0",
        ]
        .map(Into::into),
    )
    .unwrap();
    let run = MeasuredRun::from_worker(
        serde_json::from_value(Value::Object(worker_report())).unwrap(),
        &options.benchmark_cases()[0]
            .worker_arguments(&options, PathBuf::new(), PathBuf::new())
            .workload,
        RssMeasurements {
            sampled_peak_tree_rss_bytes: 1024,
            peak_tree_process_count: 2,
            rss_samples: 3,
        },
    )
    .unwrap();
    let encoded = serde_json::to_value(&run).unwrap();
    assert_eq!(encoded["server_startup_ms"], 6.0);
    assert_eq!(encoded["sampled_peak_tree_rss_bytes"], 1024);
    let mut second = serde_json::from_value::<crate::fixture_reuse_report::WorkerReport>(
        Value::Object(worker_report()),
    )
    .unwrap();
    second.samples[0].body_ms = 3.0;
    second.samples[0].total_ms = 6.0;
    second.wall_ms = 6.0;
    second.fixture_wall_ms = 7.0;
    let second = MeasuredRun::from_worker(
        second,
        &options.benchmark_cases()[0]
            .worker_arguments(&options, PathBuf::new(), PathBuf::new())
            .workload,
        RssMeasurements {
            sampled_peak_tree_rss_bytes: 2048,
            peak_tree_process_count: 2,
            rss_samples: 3,
        },
    )
    .unwrap();
    let summary = summarize(&[run, second]).unwrap();
    let encoded = serde_json::to_value(&summary).unwrap();
    assert_eq!(encoded["phases_ms"]["restore_ms"], Value::Null);
    assert_eq!(
        encoded["body_ms"],
        json!({"count": 2, "mean": 2.0, "p50": 2.0, "p95": 3.0})
    );
    assert!(summary.restore().is_none());
    assert_eq!(summary.phases_ms.build_ms.as_ref().unwrap().count, 2);
    assert_eq!(summary.wait().count, 2);
    // Preserve the former sorted-map encoding, including nullable phases.
    assert_eq!(
        serde_json::to_string(&summary.phases_ms).unwrap(),
        serde_json::to_string(&encoded["phases_ms"]).unwrap(),
    );
    assert_eq!(summary.body_ms.count, 2);
    assert_eq!(summary.body_ms.mean, 2.0);
    assert_eq!(summary.iteration_ms.mean, 5.0);
    assert_eq!(summary.work_wall_ms.mean, 5.0);
    assert_eq!(summary.fixture_wall_ms.mean, 6.0);
    assert_eq!(summary.sampled_peak_tree_rss_bytes.mean, 1536.0);
    let stats = distribution(vec![4.0, 1.0, 3.0, 2.0]).unwrap().unwrap();
    assert_eq!(stats.mean, 2.5);
    assert_eq!(stats.p50, 2.5);
    assert_eq!(stats.p95, 4.0);
    assert_eq!(
        distribution((1..=20).map(f64::from)).unwrap().unwrap().p95,
        19.0
    );
}

#[test]
fn summary_refuses_nonfinite_derived_rates_with_the_statistics_cause() {
    let options = parse_arguments(
        [
            "--server",
            "/unused",
            "--modes",
            "fresh",
            "--workers",
            "1",
            "--iterations",
            "1",
            "--state-bytes",
            "0",
        ]
        .map(Into::into),
    )
    .unwrap();
    let mut metrics: crate::fixture_reuse_report::WorkerReport =
        serde_json::from_value(Value::Object(worker_report())).unwrap();
    metrics.wall_ms = f64::from_bits(1);
    let run = MeasuredRun::from_worker(
        metrics,
        &options.benchmark_cases()[0]
            .worker_arguments(&options, PathBuf::new(), PathBuf::new())
            .workload,
        RssMeasurements::default(),
    )
    .unwrap();
    let Err(error) = summarize(&[run]) else {
        panic!("infinite derived rate must refuse");
    };
    assert!(matches!(
        error,
        Error::Statistics(crate::statistics::SampleStatisticsError::NonFiniteSample { .. })
    ));
    assert!(
        std::error::Error::source(&error)
            .unwrap()
            .is::<crate::statistics::SampleStatisticsError>()
    );
    assert!(matches!(
        distribution(vec![f64::MAX; 3]),
        Err(Error::Statistics(
            crate::statistics::SampleStatisticsError::NonFiniteMean
        ))
    ));
}

#[test]
fn worker_boundary_rejects_mismatched_incomplete_and_invalid_reports() {
    let options = parse_arguments(
        [
            "--server",
            "/unused",
            "--modes",
            "fresh",
            "--workers",
            "1",
            "--iterations",
            "1",
            "--state-bytes",
            "0",
        ]
        .map(Into::into),
    )
    .unwrap();
    let case = options.benchmark_cases().remove(0);
    let request = case.worker_arguments(&options, PathBuf::new(), PathBuf::new());
    let admit = |raw| {
        let metrics = serde_json::from_value(Value::Object(raw))?;
        MeasuredRun::from_worker(metrics, &request.workload, RssMeasurements::default())
    };
    let mut raw = worker_report();
    raw.insert("mode".into(), json!("pooled"));
    assert!(matches!(admit(raw), Err(Error::InvalidData(_))));
    for mode in [json!("unknown"), json!("Fresh"), json!(null), json!(1)] {
        let mut raw = worker_report();
        raw.insert("mode".into(), mode);
        assert!(matches!(admit(raw), Err(Error::Json(_))));
    }
    for field in [
        "workers",
        "capacity",
        "iterations",
        "state_bytes_per_canister",
    ] {
        let mut raw = worker_report();
        raw.insert(field.into(), json!(2));
        assert!(matches!(admit(raw), Err(Error::InvalidData(_))));
    }
    let mut raw = worker_report();
    raw.insert("samples".into(), json!([]));
    assert!(matches!(admit(raw), Err(Error::InvalidData(_))));
    for phase in [
        "wait_ms",
        "build_ms",
        "restore_ms",
        "reset_ms",
        "readiness_ms",
        "validation_ms",
        "acquisition_ms",
    ] {
        let mut raw = worker_report();
        raw["samples"][0]["phases"]
            .as_object_mut()
            .unwrap()
            .remove(phase);
        assert!(admit(raw).is_err());
    }
    let mut raw = worker_report();
    raw["samples"][0]["phases"]["unexpected"] = json!(0);
    assert!(admit(raw).is_err());
    let mut raw = worker_report();
    raw["samples"][0]["phases"]["wait_ms"] = Value::Null;
    assert!(admit(raw).is_err());
    let mut raw = worker_report();
    raw.insert("rss_samples".into(), json!(1));
    assert!(matches!(admit(raw), Err(Error::Json(_))));
    for field in [
        "server_startup_ms",
        "preparation_wall_ms",
        "pool_teardown_ms",
        "server_teardown_ms",
    ] {
        let mut raw = worker_report();
        raw.remove(field);
        assert!(matches!(admit(raw), Err(Error::Json(_))));
        let mut raw = worker_report();
        raw.insert(field.into(), json!(-1));
        assert!(matches!(admit(raw), Err(Error::InvalidData(_))));
    }
    let mut raw = worker_report();
    raw.remove("cold_preparation");
    assert!(matches!(admit(raw), Err(Error::Json(_))));
    let mut raw = worker_report();
    raw["cold_preparation"] = json!([raw["samples"][0]["phases"]]);
    raw["cold_preparation"][0]["build_ms"] = json!(-1);
    assert!(matches!(admit(raw), Err(Error::InvalidData(_))));
    let mut raw = worker_report();
    raw["samples"][0]["unexpected"] = json!(0);
    assert!(matches!(admit(raw), Err(Error::Json(_))));
    let mut raw = worker_report();
    raw["samples"][0]["body_ms"] = json!(-1);
    assert!(matches!(admit(raw), Err(Error::InvalidData(_))));
}
