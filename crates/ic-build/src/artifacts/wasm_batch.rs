use super::{CacheBatchEntry, CacheBatchReport, CacheOutcomeCounts};
use ic_host_artifacts::batch::{BatchLabelError, validate_labels};

use std::time::{Duration, Instant};

use super::wasm_cache::{
    WasmBuildBatchInputMetrics, WasmBuildBatchInputResolver, WasmBuildError,
    WasmBuildFailureDetails, WasmBuildOutcome, WasmBuildProgressEvent, WasmBuildSpec,
    WasmBuildTimings, build_wasm_canisters_cached_in_batch,
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

/// Progress forwarded from one independent build in a batch.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct WasmBuildBatchProgressEvent {
    /// Zero-based position in the supplied specification slice; use it to borrow
    /// that specification's caller-owned label.
    pub index: usize,
    /// Heartbeat emitted by that build.
    pub event: WasmBuildProgressEvent,
}

impl WasmBuildBatchReport {
    /// Aggregate outcome, input-resolution reuse, and timing counters.
    #[must_use]
    pub fn metrics(&self) -> WasmBuildBatchMetrics {
        let (counts, successful_timings) =
            self.batch.summarize(|sum: &mut WasmBuildTimings, record| {
                *sum = sum.saturating_add(record.timings());
            });
        WasmBuildBatchMetrics {
            specifications: self.batch.entries.len(),
            input_resolution_runs: self.input_resolution.runs,
            input_resolution_reuses: self.input_resolution.reuses,
            total: self.batch.total,
            counts,
            successful_timings,
        }
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
/// Shared-target maintenance is a separate caller-owned operation.
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
    mut progress: Option<(Duration, &mut dyn FnMut(WasmBuildBatchProgressEvent))>,
    mut spawn: impl FnMut(
        &mut std::process::Command,
    ) -> std::io::Result<ic_host_process::child::OwnedChild>,
) -> Result<WasmBuildBatchReport, BatchLabelError> {
    validate_labels(specs.iter().map(LabeledWasmBuildSpec::label))?;
    let mut resolver =
        WasmBuildBatchInputResolver::new(specs.iter().map(LabeledWasmBuildSpec::spec));
    let mut report = build_wasm_batch(specs, |spec, index| {
        let heartbeat_interval = progress.as_ref().map(|(interval, _)| *interval);
        let mut forward = |event| {
            if let Some((_, observer)) = progress.as_mut() {
                observer(WasmBuildBatchProgressEvent { index, event });
            }
        };
        build_wasm_canisters_cached_in_batch(
            spec,
            index,
            &mut resolver,
            heartbeat_interval.map(|interval| (interval, &mut forward as &mut dyn FnMut(_))),
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
    ) -> Result<WasmBuildOutcome, (WasmBuildError, WasmBuildFailureDetails)>,
{
    let started = Instant::now();
    let mut entries = Vec::with_capacity(specs.len());
    for (index, labeled) in specs.iter().enumerate() {
        let entry_started = Instant::now();
        let spec = &labeled.spec;
        let result = build(spec, index);
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

#[cfg(test)]
mod tests;
