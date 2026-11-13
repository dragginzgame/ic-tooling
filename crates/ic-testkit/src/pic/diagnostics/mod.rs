use std::{
    fmt, io,
    time::{Duration, Instant},
};

use ic_host_artifacts::batch::{BatchLabelError, validate_labels};
use pocket_ic::CanisterId as Principal;
use pocket_ic::{CanisterLogRecord, CanisterStatusResult, RejectResponse, nonblocking::PocketIc};

/// Default maximum number of canister-log records retained in diagnostics.
pub const DEFAULT_CANISTER_LOG_RECORD_LIMIT: usize = 32;

/// Default maximum aggregate raw canister-log bytes retained in diagnostics.
pub const DEFAULT_CANISTER_LOG_BYTE_LIMIT: usize = 16 * 1024;

/// Explicit bounds applied when canister logs are converted to diagnostic text.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct CanisterLogRenderLimits {
    /// Maximum retained records; zero retains none.
    pub record_limit: usize,
    /// Maximum retained raw-content bytes; zero retains none.
    pub byte_limit: usize,
}

impl Default for CanisterLogRenderLimits {
    fn default() -> Self {
        Self {
            record_limit: DEFAULT_CANISTER_LOG_RECORD_LIMIT,
            byte_limit: DEFAULT_CANISTER_LOG_BYTE_LIMIT,
        }
    }
}

/// Exact controller-aware inputs for one best-effort diagnostic collection.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct CanisterDiagnosticsRequest {
    /// Canister to inspect.
    pub canister_id: Principal,
    /// Exact sender for the status request.
    pub status_sender: Principal,
    /// Exact sender for the log request.
    pub log_sender: Principal,
    /// Caller-selected log rendering bounds.
    pub log_limits: CanisterLogRenderLimits,
}

/// One caller-labeled exact diagnostic request in a collect-all batch.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct LabeledCanisterDiagnosticsRequest {
    /// Caller label, checked for emptiness and uniqueness before collection.
    pub label: String,
    /// Exact controller-aware diagnostic inputs.
    pub request: CanisterDiagnosticsRequest,
}

/// One canister-log record retained as bounded lossy UTF-8 text.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct CanisterDiagnosticLogRecord {
    index: u64,
    timestamp_nanos: u64,
    content: String,
    original_content_bytes: usize,
    omitted_content_bytes: usize,
}

impl CanisterDiagnosticLogRecord {
    /// Upstream record index.
    #[must_use]
    pub const fn index(&self) -> u64 {
        self.index
    }

    /// Upstream record timestamp in nanoseconds.
    #[must_use]
    pub const fn timestamp_nanos(&self) -> u64 {
        self.timestamp_nanos
    }

    /// Retained content converted with lossy UTF-8 decoding.
    #[must_use]
    pub fn content(&self) -> &str {
        &self.content
    }

    /// Original raw content length before bounding and conversion.
    #[must_use]
    pub const fn original_content_bytes(&self) -> usize {
        self.original_content_bytes
    }

    /// Raw content bytes omitted from this retained record.
    #[must_use]
    pub const fn omitted_content_bytes(&self) -> usize {
        self.omitted_content_bytes
    }

    /// Whether this record's content was truncated.
    #[must_use]
    pub const fn was_truncated(&self) -> bool {
        self.omitted_content_bytes != 0
    }
}

/// Successfully fetched canister logs after bounded text conversion.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct CanisterDiagnosticLogs {
    records: Vec<CanisterDiagnosticLogRecord>,
    total_records: usize,
    total_content_bytes: usize,
    omitted_content_bytes: usize,
}

impl CanisterDiagnosticLogs {
    /// Retained records in upstream order.
    #[must_use]
    pub fn records(&self) -> &[CanisterDiagnosticLogRecord] {
        &self.records
    }

    /// Total number of records returned by `PocketIC` before bounding.
    #[must_use]
    pub const fn total_records(&self) -> usize {
        self.total_records
    }

    /// Total raw content bytes returned by `PocketIC` before bounding.
    #[must_use]
    pub const fn total_content_bytes(&self) -> usize {
        self.total_content_bytes
    }

    /// Number of whole records omitted by the configured bounds.
    #[must_use]
    pub const fn omitted_records(&self) -> usize {
        self.total_records - self.records.len()
    }

    /// Aggregate raw content bytes omitted across retained and omitted records.
    #[must_use]
    pub const fn omitted_content_bytes(&self) -> usize {
        self.omitted_content_bytes
    }

    /// Whether either whole records or record content were truncated.
    #[must_use]
    pub const fn was_truncated(&self) -> bool {
        self.omitted_records() != 0 || self.omitted_content_bytes != 0
    }
}

impl fmt::Display for CanisterDiagnosticLogs {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        if self.records.is_empty() {
            if self.total_records == 0 {
                formatter.write_str("<empty>")?;
            } else {
                formatter.write_str("<no retained records>")?;
            }
        } else {
            for (position, record) in self.records.iter().enumerate() {
                if position != 0 {
                    formatter.write_str(", ")?;
                }
                write!(
                    formatter,
                    "[{}@{}]={:?}",
                    record.index, record.timestamp_nanos, record.content
                )?;
                if record.was_truncated() {
                    write!(
                        formatter,
                        " (truncated {} bytes)",
                        record.omitted_content_bytes
                    )?;
                }
            }
        }
        if self.was_truncated() {
            write!(
                formatter,
                "; truncated omitted_records={} omitted_content_bytes={}",
                self.omitted_records(),
                self.omitted_content_bytes
            )?;
        }
        Ok(())
    }
}

/// Independent status and log outcomes for one diagnostic collection.
#[derive(Debug)]
pub struct CanisterDiagnosticsReport {
    /// Exact request used for this report.
    pub request: CanisterDiagnosticsRequest,
    /// Status result, independent of log retrieval.
    pub status: Result<CanisterStatusResult, RejectResponse>,
    /// Log result, independent of status retrieval.
    pub logs: Result<CanisterDiagnosticLogs, RejectResponse>,
}

impl CanisterDiagnosticsReport {
    /// Whether both status and log collection succeeded.
    #[must_use]
    pub const fn is_success(&self) -> bool {
        self.status.is_ok() && self.logs.is_ok()
    }
}

/// One ordered labeled entry in a collect-all diagnostics batch.
#[derive(Debug)]
pub struct CanisterDiagnosticsBatchEntry {
    /// Caller-supplied label.
    pub label: String,
    /// Structured status and log outcomes for this entry.
    pub report: CanisterDiagnosticsReport,
    /// Complete wall-clock time for this target's diagnostic collection.
    pub entry_elapsed: Duration,
}

impl CanisterDiagnosticsBatchEntry {
    /// Whether both status and log collection succeeded for this entry.
    #[must_use]
    pub const fn is_success(&self) -> bool {
        self.report.is_success()
    }
}

/// Ordered entries from a sequential collect-all diagnostics batch.
#[derive(Debug, Default)]
pub struct CanisterDiagnosticsBatchReport {
    /// Entries in the supplied request order.
    pub entries: Vec<CanisterDiagnosticsBatchEntry>,
    /// Complete wall-clock time for the sequential collect-all batch.
    pub total: Duration,
}

impl CanisterDiagnosticsBatchReport {
    /// Entries with at least one failed status or log operation.
    pub fn failures(&self) -> impl Iterator<Item = &CanisterDiagnosticsBatchEntry> {
        self.entries.iter().filter(|entry| !entry.is_success())
    }

    /// Whether every entry collected both status and logs successfully.
    #[must_use]
    pub fn is_success(&self) -> bool {
        self.entries
            .iter()
            .all(CanisterDiagnosticsBatchEntry::is_success)
    }
}

impl fmt::Display for CanisterDiagnosticsBatchReport {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            formatter,
            "diagnostics={} total={:?}",
            self.entries.len(),
            self.total
        )?;
        for entry in &self.entries {
            write!(
                formatter,
                "; label={:?} elapsed={:?} {}",
                entry.label, entry.entry_elapsed, entry.report
            )?;
        }
        Ok(())
    }
}

impl fmt::Display for CanisterDiagnosticsReport {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            formatter,
            "canister={} status_sender={} status=",
            self.request.canister_id, self.request.status_sender
        )?;
        match &self.status {
            Ok(status) => write!(
                formatter,
                "ok(state={:?} version={} controllers={} module_hash_bytes={} memory_bytes={} cycles={})",
                status.status,
                status.version,
                status.settings.controllers.len(),
                status.module_hash.as_ref().map_or(0, Vec::len),
                status.memory_size,
                status.cycles,
            ),
            Err(failure) => write!(formatter, "<{failure}>"),
        }?;
        write!(formatter, " log_sender={} logs=", self.request.log_sender)?;
        match &self.logs {
            Err(failure) => write!(formatter, "<{failure}>")?,
            Ok(logs) => write!(formatter, "{logs}")?,
        }
        Ok(())
    }
}

/// Collect status and bounded logs using the request's exact senders.
///
/// Both calls are attempted independently. Typed client failures remain in
/// the report so diagnostics can stay subordinate to the original failure.
pub async fn collect_canister_diagnostics(
    pic: &PocketIc,
    request: CanisterDiagnosticsRequest,
) -> CanisterDiagnosticsReport {
    let status = pic
        .canister_status(request.canister_id, Some(request.status_sender))
        .await;
    let logs = pic
        .fetch_canister_logs(request.canister_id, request.log_sender)
        .await
        .map(|records| render_log_records(records, request.log_limits));

    CanisterDiagnosticsReport {
        request,
        status,
        logs,
    }
}

/// Explicitly collect and write one contextual diagnostic report.
///
/// Uses the request's exact senders and log limits, with no retry or output
/// destination selected implicitly. Unrelated collection/formatting/writer
/// panics propagate.
/// Ordinary writer errors are returned unchanged. Status/log failures remain
/// in the written report; successful output does not mean those calls passed.
/// Callers keep their primary failure authoritative and decide whether to
/// retain or ignore this secondary reporting result.
///
/// # Errors
/// Returns the writer’s I/O error when diagnostics cannot be written.
pub async fn write_canister_diagnostics(
    pic: &PocketIc,
    request: CanisterDiagnosticsRequest,
    context: &str,
    writer: &mut dyn io::Write,
) -> io::Result<()> {
    let report = collect_canister_diagnostics(pic, request).await;
    writeln!(writer, "{context}: {report}")
}

/// Collect every labeled request sequentially in its supplied order.
///
/// Every target is attempted even when an earlier entry has client failures.
/// Each entry preserves its exact request and independent status/log
/// outcomes; no anonymous retry or fallback is performed.
/// Invalid labels return Host's [`BatchLabelError`] before collection starts.
///
/// # Errors
/// Rejects invalid or duplicate labels before collecting diagnostics.
pub async fn collect_canister_diagnostics_batch(
    pic: &PocketIc,
    requests: &[LabeledCanisterDiagnosticsRequest],
) -> Result<CanisterDiagnosticsBatchReport, BatchLabelError> {
    validate_labels(requests.iter().map(|request| request.label.as_str()))?;
    let started = Instant::now();
    let mut entries = Vec::with_capacity(requests.len());
    for labeled in requests {
        let entry_started = Instant::now();
        let request = labeled.request;
        let report = collect_canister_diagnostics(pic, request).await;
        entries.push(CanisterDiagnosticsBatchEntry {
            label: labeled.label.clone(),
            report,
            entry_elapsed: entry_started.elapsed(),
        });
    }

    Ok(CanisterDiagnosticsBatchReport {
        entries,
        total: started.elapsed(),
    })
}
fn render_log_records(
    records: Vec<CanisterLogRecord>,
    limits: CanisterLogRenderLimits,
) -> CanisterDiagnosticLogs {
    let total_records = records.len();
    let mut total_content_bytes = 0usize;
    let mut rendered = Vec::with_capacity(total_records.min(limits.record_limit));
    let mut retained_bytes = 0usize;
    let mut omitted_content_bytes = 0usize;

    for record in records {
        total_content_bytes = total_content_bytes.saturating_add(record.content.len());
        let include = rendered.len() < limits.record_limit && retained_bytes < limits.byte_limit;
        let retained = if include {
            record
                .content
                .len()
                .min(limits.byte_limit.saturating_sub(retained_bytes))
        } else {
            0
        };
        let omitted = record.content.len().saturating_sub(retained);
        omitted_content_bytes = omitted_content_bytes.saturating_add(omitted);
        if !include {
            continue;
        }

        let original_content_bytes = record.content.len();
        // Reuse complete valid buffers, but do not retain a large allocation
        // merely to hold a caller-truncated prefix.
        let content = if retained == original_content_bytes {
            String::from_utf8(record.content)
                .unwrap_or_else(|error| String::from_utf8_lossy(error.as_bytes()).into_owned())
        } else {
            String::from_utf8_lossy(&record.content[..retained]).into_owned()
        };
        retained_bytes = retained_bytes.saturating_add(retained);
        rendered.push(CanisterDiagnosticLogRecord {
            index: record.idx,
            timestamp_nanos: record.timestamp_nanos,
            content,
            original_content_bytes,
            omitted_content_bytes: omitted,
        });
    }

    CanisterDiagnosticLogs {
        records: rendered,
        total_records,
        total_content_bytes,
        omitted_content_bytes,
    }
}

#[cfg(test)]
mod tests;
