use super::cache_fs::canonicalize_allow_missing;
use super::{CacheBatchEntry, CacheBatchReport, CacheOutcomeCounts};
use ic_host_artifacts::batch::{BatchLabelError, validate_labels};

use std::{
    collections::HashMap,
    path::PathBuf,
    time::{Duration, Instant},
};

use super::wasm_cache::{
    SharedIncrementalTargetMaintenanceConfig, SharedIncrementalTargetMaintenanceOutcome,
    WasmBuildBatchInputMetrics, WasmBuildBatchInputResolver, WasmBuildError,
    WasmBuildFailureDetails, WasmBuildOutcome, WasmBuildProgressConfig, WasmBuildProgressEvent,
    WasmBuildSpec, WasmBuildTimings, build_wasm_canisters_cached_in_batch,
    shared_incremental_target,
};

/// Caller-labeled specification for one exact Wasm batch entry.
///
/// The label is report and progress identity only; it does not alter the
/// underlying exact Wasm fingerprint or cache key.
pub type LabeledWasmBuildSpec = super::batch::LabeledSpec<WasmBuildSpec>;

/// Ordered outcomes and failures from a collect-all Wasm build batch.
#[derive(Debug)]
pub struct WasmBuildBatchReport {
    /// Common ordered outcomes and elapsed time.
    pub batch: CacheBatchReport<WasmBuildOutcome, WasmBuildBatchFailure>,
    input_resolution: WasmBuildBatchInputMetrics,
}

/// Structural error that prevents a labeled Wasm batch from starting.
#[non_exhaustive]
#[derive(Clone, Debug, Eq, PartialEq, thiserror::Error)]
pub enum WasmBuildBatchContractError {
    /// Invalid input labels, rejected before source validation or command effects.
    #[error("Wasm batch {0}")]
    Labels(#[source] BatchLabelError),
}

/// Build error together with the phase and partial timings at the failure.
pub type WasmBuildBatchFailure = (WasmBuildError, WasmBuildFailureDetails);

impl CacheBatchEntry<WasmBuildOutcome, WasmBuildBatchFailure> {
    /// The build error, independently of its partial phase/timing details.
    #[must_use]
    pub fn error(&self) -> Option<&WasmBuildError> {
        self.failure().map(|(error, _)| error)
    }

    /// Structured phase and partial timings when this entry failed.
    #[must_use]
    pub fn failure_details(&self) -> Option<WasmBuildFailureDetails> {
        self.failure().map(|(_, details)| *details)
    }
}

/// Aggregate counters and successful-acquisition timings for a Wasm build batch.
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct WasmBuildBatchMetrics {
    specifications: usize,
    counts: CacheOutcomeCounts,
    input_resolution_runs: usize,
    input_resolution_reuses: usize,
    successful_timings: WasmBuildTimings,
    total: Duration,
}

/// Structured progress for an independent sequence of exact Wasm builds.
#[non_exhaustive]
#[derive(Clone, Debug, Eq, PartialEq)]
pub enum WasmBuildBatchProgressEvent {
    /// Progress forwarded from one independent build.
    BuildProgress {
        /// Zero-based position in the supplied specification slice.
        index: usize,
        /// Caller-owned stable label.
        label: String,
        /// Event emitted by that build.
        event: WasmBuildProgressEvent,
    },
}

impl WasmBuildBatchReport {
    /// Canonical entries paired with their integrated shared-target maintenance.
    ///
    /// Batch-owned maintenance contributes at most one outcome for each
    /// distinct configured shared-target path.
    pub fn shared_incremental_maintenance_outcomes(
        &self,
    ) -> impl Iterator<
        Item = (
            &CacheBatchEntry<WasmBuildOutcome, WasmBuildBatchFailure>,
            &SharedIncrementalTargetMaintenanceOutcome,
        ),
    > {
        self.batch.outcomes().filter_map(|entry| {
            entry
                .outcome()?
                .record()
                .shared_incremental_maintenance()
                .map(|outcome| (entry, outcome))
        })
    }

    /// Aggregate outcome, input-resolution reuse, and timing counters.
    #[must_use]
    pub fn metrics(&self) -> WasmBuildBatchMetrics {
        let mut metrics = WasmBuildBatchMetrics {
            specifications: self.batch.entries.len(),
            input_resolution_runs: self.input_resolution.runs,
            input_resolution_reuses: self.input_resolution.reuses,
            total: self.batch.total,
            ..WasmBuildBatchMetrics::default()
        };
        for outcome in self
            .batch
            .entries
            .iter()
            .filter_map(CacheBatchEntry::outcome)
        {
            metrics.counts.record(outcome.is_reused());
            metrics.successful_timings = metrics
                .successful_timings
                .saturating_add(outcome.record().timings());
        }
        metrics
    }
}

impl WasmBuildBatchMetrics {
    /// Number of supplied specifications.
    #[must_use]
    pub const fn specifications(self) -> usize {
        self.specifications
    }

    /// Number of successful specifications.
    #[must_use]
    pub const fn succeeded(self) -> usize {
        self.counts.succeeded()
    }

    /// Number of failed specifications.
    #[must_use]
    pub const fn failed(self) -> usize {
        self.specifications - self.succeeded()
    }

    /// Number of newly built Wasm artifact sets.
    #[must_use]
    pub const fn built(self) -> usize {
        self.counts.built
    }

    /// Number of Wasm artifact sets reused from the exact cache.
    #[must_use]
    pub const fn reused(self) -> usize {
        self.counts.reused
    }

    /// Number of workspace/toolchain input-resolution snapshots performed.
    #[must_use]
    pub const fn input_resolution_runs(self) -> usize {
        self.input_resolution_runs
    }

    /// Number of specifications resolved by reusing another batch snapshot.
    #[must_use]
    pub const fn input_resolution_reuses(self) -> usize {
        self.input_resolution_reuses
    }

    /// Sum of timings from successful acquisitions.
    #[must_use]
    pub const fn successful_timings(self) -> WasmBuildTimings {
        self.successful_timings
    }

    /// Complete wall-clock time for the sequential batch.
    #[must_use]
    pub const fn total(self) -> Duration {
        self.total
    }
}

impl std::fmt::Display for WasmBuildBatchReport {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let metrics = self.metrics();
        write!(
            formatter,
            "builds={} succeeded={} failed={} built={} reused={} input_resolution_runs={} input_resolution_reuses={} successful_timings=({}) total={:?}",
            metrics.specifications(),
            metrics.succeeded(),
            metrics.failed(),
            metrics.built(),
            metrics.reused(),
            metrics.input_resolution_runs(),
            metrics.input_resolution_reuses(),
            metrics.successful_timings(),
            metrics.total(),
        )
    }
}

/// Build every Wasm specification as an independent Cargo invocation.
///
/// Each spec owns its maintenance policy. The batch applies matching policies
/// once per canonical shared target and rejects conflicting policies per entry.
/// Pass optional progress reporting.
/// `spawn` selects Cargo child/group custody and cleanup for each cache miss,
/// independently of progress; it may override the default null stdin.
/// Specifications run sequentially and every result is retained. Each entry
/// keeps its own package set, profile arguments, feature resolution,
/// fingerprint, locks, and cache policy. Packages are never combined into one
/// Cargo command because doing so can unify shared dependency features.
///
/// # Errors
/// Returns a contract error for an empty or duplicate label before any work.
/// Individual build failures are retained in the returned report.
#[expect(
    clippy::result_large_err,
    reason = "Batch entries retain complete failure details inline without per-failure allocation."
)]
pub fn build_wasm_canisters_cached_batch(
    specs: &[LabeledWasmBuildSpec],
    mut progress: Option<(
        WasmBuildProgressConfig,
        &mut dyn FnMut(WasmBuildBatchProgressEvent),
    )>,
    mut spawn: impl FnMut(
        &mut std::process::Command,
    ) -> std::io::Result<ic_host_process::child::OwnedChild>,
) -> Result<WasmBuildBatchReport, WasmBuildBatchContractError> {
    validate_labels(specs.iter().map(LabeledWasmBuildSpec::label))
        .map_err(WasmBuildBatchContractError::Labels)?;
    let mut resolver =
        WasmBuildBatchInputResolver::new(specs.iter().map(LabeledWasmBuildSpec::spec));
    let mut report = build_wasm_batch(specs, |spec, index, maintenance| {
        let progress_config = progress.as_ref().map(|(config, _)| *config);
        let mut forward = |event| {
            if let Some((_, observer)) = progress.as_mut() {
                observer(WasmBuildBatchProgressEvent::BuildProgress {
                    index,
                    label: specs[index].label.clone(),
                    event,
                });
            }
        };
        build_wasm_canisters_cached_in_batch(
            spec,
            maintenance,
            index,
            &mut resolver,
            progress_config.map(|config| (config, &mut forward as &mut dyn FnMut(_))),
            &mut spawn,
        )
    });
    report.input_resolution = resolver.metrics();
    Ok(report)
}

fn build_wasm_batch<F>(specs: &[LabeledWasmBuildSpec], mut build: F) -> WasmBuildBatchReport
where
    F: FnMut(
        &WasmBuildSpec,
        usize,
        Option<SharedIncrementalTargetMaintenanceConfig>,
    ) -> Result<WasmBuildOutcome, (WasmBuildError, WasmBuildFailureDetails)>,
{
    let started = Instant::now();
    let mut entries = Vec::with_capacity(specs.len());
    let mut maintenance = BatchMaintenanceTracker::default();
    for (index, labeled) in specs.iter().enumerate() {
        let entry_started = Instant::now();
        let spec = &labeled.spec;
        let result = match maintenance.prepare_spec(spec) {
            Ok(configured) => build(spec, index, configured),
            Err(error) => Err((
                error,
                WasmBuildFailureDetails::specification(entry_started.elapsed()),
            )),
        };
        entries.push(CacheBatchEntry {
            index,
            label: labeled.label.clone(),
            result,
            entry_elapsed: entry_started.elapsed(),
        });
    }
    WasmBuildBatchReport {
        batch: CacheBatchReport {
            entries,
            total: started.elapsed(),
        },
        input_resolution: WasmBuildBatchInputMetrics::default(),
    }
}

#[derive(Default)]
struct BatchMaintenanceTracker {
    configured_targets: HashMap<PathBuf, SharedIncrementalTargetMaintenanceConfig>,
}

impl BatchMaintenanceTracker {
    fn prepare_spec(
        &mut self,
        spec: &WasmBuildSpec,
    ) -> Result<Option<SharedIncrementalTargetMaintenanceConfig>, WasmBuildError> {
        let Some(config) = spec.shared_incremental_target_maintenance() else {
            return Ok(None);
        };
        let Some(target_dir) = shared_incremental_target(spec) else {
            return Ok(None);
        };
        let canonical =
            canonicalize_allow_missing(&target_dir).map_err(|source| WasmBuildError::Io {
                operation: "resolve batch shared incremental target",
                path: target_dir,
                source,
            })?;
        if let Some(previous) = self.configured_targets.get(&canonical) {
            if *previous != config {
                return Err(WasmBuildError::InvalidSpec {
                    message: "shared target has conflicting maintenance policies in one batch"
                        .to_owned(),
                });
            }
            return Ok(None);
        }
        self.configured_targets.insert(canonical, config);
        Ok(Some(config))
    }
}

#[cfg(test)]
mod tests;
