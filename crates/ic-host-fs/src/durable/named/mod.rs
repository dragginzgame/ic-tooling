//! Shared durable publication errors, retaining producer and cleanup causes.

use std::{fmt, io};

/// Failure from a named or streamed producer and durable publication, retaining cleanup evidence.
#[derive(Debug, thiserror::Error)]
pub enum NamedWriteError<E> {
    /// The producer or its output validation failed; no publication was attempted.
    Producer {
        /// Original consumer error, including any bounded process evidence.
        source: E,
        /// Failure to remove the owned staging entry; replacements are left untouched.
        cleanup_error: Option<io::Error>,
    },
    /// Filesystem admission, synchronization or rename failed before publication.
    BeforePublication {
        /// Original filesystem error or an invalid-data identity rejection.
        source: io::Error,
        /// Failure to remove the owned staging entry; replacements are left untouched.
        cleanup_error: Option<io::Error>,
    },
    /// Publication succeeded, but staging cleanup or final directory sync failed.
    AfterPublication {
        /// Original cleanup or synchronization error. The new output is visible;
        /// final directory durability is not established. Reconcile before retrying.
        source: io::Error,
    },
}

impl<E> NamedWriteError<E> {
    pub(super) const fn before(source: io::Error) -> Self {
        Self::BeforePublication {
            source,
            cleanup_error: None,
        }
    }
}

impl<E: fmt::Display> fmt::Display for NamedWriteError<E> {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Producer { source, .. } => write!(formatter, "staged producer failed: {source}"),
            Self::BeforePublication { source, .. } => {
                write!(formatter, "publication failed before rename: {source}")
            }
            Self::AfterPublication { source } => write!(
                formatter,
                "output published but completion failed: {source}"
            ),
        }?;
        if let Self::Producer {
            cleanup_error: Some(cleanup),
            ..
        }
        | Self::BeforePublication {
            cleanup_error: Some(cleanup),
            ..
        } = self
        {
            write!(formatter, "; staging cleanup failed: {cleanup}")?;
        }
        Ok(())
    }
}

#[cfg(all(test, unix))]
mod tests;
