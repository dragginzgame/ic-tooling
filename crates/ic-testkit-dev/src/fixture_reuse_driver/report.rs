//! Validates workload reports at the process boundary and projects summaries.
//! The workload remains the encoder/owner of raw measurements; no data migration.

use std::collections::BTreeMap;

use crate::{
    fixture_reuse_protocol::Workload,
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
        request: &Workload,
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

#[expect(
    clippy::cast_precision_loss,
    reason = "finite-sample reports intentionally project exact counters into approximate rates"
)]
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
mod tests;
