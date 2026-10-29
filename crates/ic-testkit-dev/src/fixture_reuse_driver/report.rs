//! Validates workload reports at the process boundary and projects summaries.
//! The workload remains the encoder/owner of raw measurements; no data migration.

use std::collections::BTreeMap;

use crate::{
    fixture_reuse_protocol::WorkerArguments,
    fixture_reuse_report::{Phases, WorkerReport},
    statistics::{SampleStatistics, summarize_samples},
};
use serde::Serialize;

use super::Error;

/// Fixed provenance schema captured before worker execution.
#[derive(Serialize)]
pub(super) struct Provenance {
    pub(super) host_profile: String,
    pub(super) iterations: usize,
    pub(super) logical_cpus: u64,
    pub(super) modes: Vec<&'static str>,
    pub(super) platform: String,
    pub(super) pool_capacities: Vec<usize>,
    pub(super) repeats: usize,
    pub(super) revision: String,
    pub(super) rss_command_stderr_bytes: usize,
    pub(super) rss_command_stdout_bytes: usize,
    pub(super) rss_sample_interval_ms: u128,
    pub(super) rss_sampler: &'static str,
    pub(super) rustc: String,
    pub(super) server: String,
    pub(super) server_version: String,
    pub(super) state_bytes_per_canister: u32,
    pub(super) wasm_profile: String,
    pub(super) wasm_sha256: String,
    pub(super) workers: usize,
    pub(super) working_tree_dirty: bool,
}

/// Borrow collected measurements when encoding the final report.
#[derive(Serialize)]
pub(super) struct DriverReport<'a> {
    pub(super) format: &'static str,
    pub(super) provenance: &'a Provenance,
    pub(super) runs: BTreeMap<&'a str, &'a [MeasuredRun]>,
    pub(super) summary: BTreeMap<&'a str, &'a Summary>,
}

#[derive(Serialize)]
pub(super) struct MeasuredRun {
    #[serde(flatten)]
    metrics: WorkerReport,
    #[serde(flatten)]
    pub(super) rss: RssMeasurements,
}

#[derive(Default, Serialize)]
pub(super) struct RssMeasurements {
    pub(super) sampled_peak_tree_rss_bytes: u64,
    pub(super) peak_tree_process_count: usize,
    pub(super) rss_samples: usize,
}

impl MeasuredRun {
    pub(super) fn from_worker(
        metrics: WorkerReport,
        request: &WorkerArguments,
        rss: RssMeasurements,
    ) -> Result<Self, Error> {
        let invalid_duration = |value: f64| !value.is_finite() || value < 0.0;
        if metrics.mode != request.mode
            || metrics.capacity != request.capacity.get()
            || metrics.iterations != request.iterations.get()
            || metrics.workers != request.workers.get()
            || metrics.state_bytes_per_canister != request.state_bytes
        {
            return Err(Error::InvalidData(
                "worker report does not match requested workload",
            ));
        }
        if metrics.samples.len() != metrics.iterations
            || metrics.samples.is_empty()
            || !metrics.wall_ms.is_finite()
            || metrics.wall_ms <= 0.0
            || !metrics.fixture_wall_ms.is_finite()
            || metrics.fixture_wall_ms < metrics.wall_ms
        {
            return Err(Error::InvalidData("invalid worker task count or wall time"));
        }
        for value in [
            metrics.server_startup_ms,
            metrics.preparation_wall_ms,
            metrics.pool_teardown_ms,
            metrics.server_teardown_ms,
        ] {
            if invalid_duration(value) {
                return Err(Error::InvalidData("invalid worker lifecycle duration"));
            }
        }
        for phases in metrics
            .cold_preparation
            .iter()
            .chain(metrics.samples.iter().map(|sample| &sample.phases))
        {
            if phases.values().into_iter().flatten().any(invalid_duration) {
                return Err(Error::InvalidData("invalid worker phase duration"));
            }
        }
        for (iteration, sample) in metrics.samples.iter().enumerate() {
            if sample.iteration != iteration
                || [sample.body_ms, sample.release_ms, sample.total_ms]
                    .into_iter()
                    .any(invalid_duration)
            {
                return Err(Error::InvalidData(
                    "invalid worker task identity, phases or durations",
                ));
            }
        }
        Ok(Self { metrics, rss })
    }
}

// Complete floating-point sample statistics belong to this private report owner.
fn distribution(values: impl IntoIterator<Item = f64>) -> Result<Option<SampleStatistics>, Error> {
    let mut values = values.into_iter().collect::<Vec<_>>();
    summarize_samples(&mut values).map_err(Error::Statistics)
}

fn required_distribution(values: impl IntoIterator<Item = f64>) -> Result<SampleStatistics, Error> {
    distribution(values)?.ok_or(Error::InvalidData("summary lacks required measurements"))
}

// Keep the former map's alphabetical field order in serialized reports.
#[derive(Serialize)]
#[expect(
    clippy::struct_field_names,
    reason = "serialized field names explicitly carry their millisecond units"
)]
struct PhaseSummary {
    acquisition_ms: SampleStatistics,
    build_ms: Option<SampleStatistics>,
    readiness_ms: Option<SampleStatistics>,
    reset_ms: Option<SampleStatistics>,
    restore_ms: Option<SampleStatistics>,
    validation_ms: Option<SampleStatistics>,
    wait_ms: SampleStatistics,
}

#[derive(Serialize)]
pub(super) struct Summary {
    phases_ms: PhaseSummary,
    body_ms: SampleStatistics,
    release_ms: SampleStatistics,
    iteration_ms: SampleStatistics,
    pub(super) work_wall_ms: SampleStatistics,
    pub(super) fixture_wall_ms: SampleStatistics,
    pub(super) iterations_per_second: SampleStatistics,
    pub(super) sampled_peak_tree_rss_bytes: SampleStatistics,
}

impl Summary {
    pub(super) const fn wait(&self) -> &SampleStatistics {
        &self.phases_ms.wait_ms
    }

    pub(super) const fn restore(&self) -> Option<&SampleStatistics> {
        self.phases_ms.restore_ms.as_ref()
    }
}

#[allow(clippy::cast_precision_loss)]
pub(super) fn summarize(runs: &[MeasuredRun]) -> Result<Summary, Error> {
    let samples = || runs.iter().flat_map(|run| &run.metrics.samples);
    let phase_values = |select: fn(&Phases) -> Option<f64>| {
        samples().filter_map(move |sample| select(&sample.phases))
    };
    // Fields select their measurements directly; collection has no positional schema.
    let phases_ms = PhaseSummary {
        acquisition_ms: required_distribution(phase_values(|phases| Some(phases.acquisition_ms)))?,
        build_ms: distribution(phase_values(|phases| phases.build_ms))?,
        readiness_ms: distribution(phase_values(|phases| phases.readiness_ms))?,
        reset_ms: distribution(phase_values(|phases| phases.reset_ms))?,
        restore_ms: distribution(phase_values(|phases| phases.restore_ms))?,
        validation_ms: distribution(phase_values(|phases| phases.validation_ms))?,
        wait_ms: required_distribution(phase_values(|phases| Some(phases.wait_ms)))?,
    };
    Ok(Summary {
        phases_ms,
        body_ms: required_distribution(samples().map(|sample| sample.body_ms))?,
        release_ms: required_distribution(samples().map(|sample| sample.release_ms))?,
        iteration_ms: required_distribution(samples().map(|sample| sample.total_ms))?,
        work_wall_ms: required_distribution(runs.iter().map(|run| run.metrics.wall_ms))?,
        fixture_wall_ms: required_distribution(runs.iter().map(|run| run.metrics.fixture_wall_ms))?,
        iterations_per_second: required_distribution(
            runs.iter()
                .map(|run| run.metrics.iterations as f64 * 1000.0 / run.metrics.wall_ms),
        )?,
        sampled_peak_tree_rss_bytes: required_distribution(
            runs.iter()
                .map(|run| run.rss.sampled_peak_tree_rss_bytes as f64),
        )?,
    })
}

#[cfg(test)]
mod tests {
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
            &options.benchmark_cases()[0].worker_arguments(
                &options,
                PathBuf::new(),
                PathBuf::new(),
            ),
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
            &options.benchmark_cases()[0].worker_arguments(
                &options,
                PathBuf::new(),
                PathBuf::new(),
            ),
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
            &options.benchmark_cases()[0].worker_arguments(
                &options,
                PathBuf::new(),
                PathBuf::new(),
            ),
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
            MeasuredRun::from_worker(metrics, &request, RssMeasurements::default())
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
}
