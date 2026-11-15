//! Explicit aggregate byte admission across bounded stream reads.
use super::{ArtifactError, read_reader};
use std::io::Read;

/// A caller-selected aggregate allowance; failures also consume bounded work.
#[derive(Debug)]
pub struct ReadBudget {
    maximum: u64,
    remaining: u64,
}
impl ReadBudget {
    /// Start one independent inspection allowance.
    #[must_use]
    pub const fn new(maximum: u64) -> Self {
        Self {
            maximum,
            remaining: maximum,
        }
    }
    /// Remaining allowance after successful and failed reads.
    #[must_use]
    pub const fn remaining(&self) -> u64 {
        self.remaining
    }
    /// Read one stream after caller-supplied metadata observation.
    /// Metadata is not a content snapshot; streaming admission detects growth.
    /// A partial IO/allocation failure conservatively charges the attempted ceiling plus one.
    /// # Errors
    /// Distinguishes per-stream, aggregate, accounting and underlying read failures.
    pub fn read(
        &mut self,
        reader: impl Read,
        metadata_length: u64,
        maximum: u64,
    ) -> Result<Vec<u8>, BudgetReadError> {
        if metadata_length > maximum {
            return Err(BudgetReadError::FileLimit {
                actual: metadata_length,
                maximum,
            });
        }
        if metadata_length > self.remaining {
            return Err(BudgetReadError::AggregateLimit {
                maximum: self.maximum,
            });
        }
        usize::try_from(metadata_length).map_err(|_| BudgetReadError::Accounting)?;
        let ceiling = maximum.min(self.remaining);
        let host_ceiling = usize::try_from(ceiling).unwrap_or(usize::MAX);
        match read_reader(reader, host_ceiling) {
            Ok(bytes) => {
                self.remaining -= bytes.len() as u64;
                Ok(bytes)
            }
            Err(ArtifactError::LimitExceeded { limit }) => {
                let actual = limit.checked_add(1).ok_or(BudgetReadError::Accounting)?;
                if actual > ceiling && ceiling == self.remaining {
                    self.remaining = 0;
                    Err(BudgetReadError::AggregateLimit {
                        maximum: self.maximum,
                    })
                } else {
                    self.remaining = self.remaining.saturating_sub(ceiling.saturating_add(1));
                    if actual > maximum {
                        Err(BudgetReadError::FileLimit { actual, maximum })
                    } else {
                        Err(BudgetReadError::Accounting)
                    }
                }
            }
            Err(source) => {
                self.remaining = self.remaining.saturating_sub(ceiling.saturating_add(1));
                Err(BudgetReadError::Read(source))
            }
        }
    }
}
/// Mechanical bounded-read refusal without product paths, schemas or recovery policy.
#[derive(Debug)]
pub enum BudgetReadError {
    /// Observed per-stream size exceeded its allowance.
    FileLimit {
        /// Observed byte length or first byte beyond the allowance.
        actual: u64,
        /// Caller-selected per-stream allowance.
        maximum: u64,
    },
    /// Inspection aggregate allowance was exhausted.
    AggregateLimit {
        /// Original caller-selected inspection allowance.
        maximum: u64,
    },
    /// Byte accounting cannot be represented on this host.
    Accounting,
    /// Bounded allocation or stream reading failed.
    Read(ArtifactError),
}
#[cfg(test)]
mod tests;

impl std::fmt::Display for BudgetReadError {
    fn fmt(&self, out: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::FileLimit { actual, maximum } => {
                write!(out, "stream has {actual} bytes, exceeding {maximum}")
            }
            Self::AggregateLimit { maximum } => write!(out, "inspection exceeds {maximum} bytes"),
            Self::Accounting => out.write_str("byte accounting exceeds the host range"),
            Self::Read(source) => std::fmt::Display::fmt(source, out),
        }
    }
}
impl std::error::Error for BudgetReadError {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        match self {
            Self::Read(source) => Some(source),
            _ => None,
        }
    }
}
