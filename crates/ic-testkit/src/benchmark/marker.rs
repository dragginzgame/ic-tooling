//! Portable benchmark marker grammar and exact counter values.

/// Default prefix for benchmark marker lines.
pub const DEFAULT_PREFIX: &str = "ICTK";

/// Whether a marker opens or closes a measured span.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum BenchmarkEventKind {
    /// Opening counter sample.
    Start,
    /// Closing counter sample.
    End,
}

/// Exact instruction and byte counters reported by a marker.
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct BenchmarkCounters {
    /// Instruction counter in the representation described by this type.
    pub instructions: u128,
    /// Heap byte counter in the representation described by this type.
    pub heap_bytes: u128,
    /// Memory byte counter in the representation described by this type.
    pub memory_bytes: u128,
}

#[cfg(not(target_arch = "wasm32"))]
impl BenchmarkCounters {
    pub(super) fn checked_delta(self, start: Self) -> Option<Self> {
        Some(Self {
            instructions: self.instructions.checked_sub(start.instructions)?,
            heap_bytes: self.heap_bytes.checked_sub(start.heap_bytes)?,
            memory_bytes: self.memory_bytes.checked_sub(start.memory_bytes)?,
        })
    }

    pub(super) fn checked_add(
        self,
        other: Self,
    ) -> Result<Self, super::analysis::BenchmarkCounter> {
        Ok(Self {
            instructions: self
                .instructions
                .checked_add(other.instructions)
                .ok_or(super::analysis::BenchmarkCounter::Instructions)?,
            heap_bytes: self
                .heap_bytes
                .checked_add(other.heap_bytes)
                .ok_or(super::analysis::BenchmarkCounter::HeapBytes)?,
            memory_bytes: self
                .memory_bytes
                .checked_add(other.memory_bytes)
                .ok_or(super::analysis::BenchmarkCounter::MemoryBytes)?,
        })
    }

    pub(super) fn min_assign(&mut self, other: Self) {
        self.instructions = self.instructions.min(other.instructions);
        self.heap_bytes = self.heap_bytes.min(other.heap_bytes);
        self.memory_bytes = self.memory_bytes.min(other.memory_bytes);
    }

    pub(super) fn max_assign(&mut self, other: Self) {
        self.instructions = self.instructions.max(other.instructions);
        self.heap_bytes = self.heap_bytes.max(other.heap_bytes);
        self.memory_bytes = self.memory_bytes.max(other.memory_bytes);
    }
}

/// A marker field cannot be represented in the line-oriented marker grammar.
#[derive(Clone, Copy, Debug, Eq, PartialEq, thiserror::Error)]
pub enum BenchmarkMarkerError {
    /// Prefix is empty or contains a pipe, CR or LF.
    #[error("invalid benchmark marker prefix")]
    InvalidPrefix,
    /// Label contains a pipe/line break or lacks a nonempty span and start/end suffix.
    #[error("invalid benchmark marker label")]
    InvalidLabel,
}

/// Format one complete marker after admitting its prefix and start/end label.
///
/// Printable Unicode and spaces remain literal. No fields are escaped or trimmed.
/// The parser must separately be configured to accept this prefix.
///
/// # Errors
/// Refuses empty prefixes, pipe/CR/LF delimiters and labels without a nonempty
/// span followed by `:start` or `:end`. Errors do not include the rejected text.
pub fn format_marker(
    prefix: &str,
    label: &str,
    counters: BenchmarkCounters,
) -> Result<String, BenchmarkMarkerError> {
    if prefix.is_empty() || prefix.contains(['|', '\r', '\n']) {
        return Err(BenchmarkMarkerError::InvalidPrefix);
    }
    if label.contains(['|', '\r', '\n']) || split_label_kind(label).is_none() {
        return Err(BenchmarkMarkerError::InvalidLabel);
    }
    Ok(format!(
        "{}|{}|{}|{}|{}",
        prefix, label, counters.instructions, counters.heap_bytes, counters.memory_bytes
    ))
}

pub(super) fn split_label_kind(label: &str) -> Option<(&str, BenchmarkEventKind)> {
    label
        .strip_suffix(":start")
        .map(|span| (span, BenchmarkEventKind::Start))
        .or_else(|| {
            label
                .strip_suffix(":end")
                .map(|span| (span, BenchmarkEventKind::End))
        })
        .filter(|(span, _)| !span.is_empty())
}
