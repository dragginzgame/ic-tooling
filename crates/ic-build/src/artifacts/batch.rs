//! Ordered identity and result storage shared by both cache batch engines.

use std::time::Duration;

/// One caller-labeled result and its elapsed time in a collect-all cache batch.
#[derive(Debug)]
pub struct CacheBatchEntry<T, E> {
    pub(super) index: usize,
    pub(super) label: String,
    pub(super) result: Result<T, E>,
    pub(super) entry_elapsed: Duration,
}

impl<T, E> CacheBatchEntry<T, E> {
    /// Zero-based position in the supplied specification slice.
    #[must_use]
    pub const fn index(&self) -> usize {
        self.index
    }

    /// Caller-owned stable label.
    #[must_use]
    pub fn label(&self) -> &str {
        &self.label
    }

    /// Complete outcome or failure, including domain-specific failure details.
    ///
    /// # Errors
    /// Borrows the failure retained for this entry.
    pub const fn result(&self) -> Result<&T, &E> {
        self.result.as_ref()
    }

    /// Successful outcome, when this entry succeeded.
    #[must_use]
    pub fn outcome(&self) -> Option<&T> {
        self.result.as_ref().ok()
    }

    /// Complete failure, when this entry failed.
    #[must_use]
    pub fn failure(&self) -> Option<&E> {
        self.result.as_ref().err()
    }

    /// Complete wall-clock time retained for this entry.
    #[must_use]
    pub const fn entry_elapsed(&self) -> Duration {
        self.entry_elapsed
    }

    /// Whether this entry completed successfully.
    #[must_use]
    pub const fn is_success(&self) -> bool {
        self.result.is_ok()
    }

    /// Consume the ordered identity, complete result and wall time.
    pub fn into_parts(self) -> (usize, String, Result<T, E>, Duration) {
        (self.index, self.label, self.result, self.entry_elapsed)
    }
}
