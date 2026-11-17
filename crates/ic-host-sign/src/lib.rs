//! Bounded external IC signing; key custody stays in the selected helper process.
//!
//! Platform constructs/admit envelopes. Process owns communication and cleanup.
//! Callers own approval, spending, durable custody, submission and recovery.

mod client;
pub use client::{Signer, State};
pub mod protocol;

/// Local admission/execution failure. No provider stderr or key data is exposed.
#[derive(Debug, thiserror::Error)]
pub enum Error {
    /// The supplied tool, identity profile or original approval differs.
    #[error("signer configuration refused")]
    Configuration,
    /// Invalid fields or a request/frame above the supported bound.
    #[error("signing request refused")]
    Request,
    /// Another invocation still owns execution or awaits settlement.
    #[error("previous signing invocation must be settled first")]
    Busy,
    /// Cleanup or worker completion is uncertain; this signer cannot be reused.
    #[error("signer cleanup is uncertain; further invocations refused")]
    Uncertain,
    /// The signing-completion deadline expired, including output admission.
    #[error("signing deadline expired")]
    Deadline,
    /// Cancellation was observed before a child was dispatched.
    #[error("signing cancelled before dispatch")]
    Cancelled,
    /// No invocation is awaiting settlement.
    #[error("no signing invocation to settle")]
    Idle,
    /// The helper did not complete successfully. Original failure/status and
    /// independent cleanup evidence are retained without captured payloads.
    #[error("signer process failed")]
    Execution {
        /// Original native execution evidence.
        report: ic_host_process::Report,
    },
    /// The returned frame or its exact request binding differs.
    #[error("signer output refused")]
    Output,
    /// Original public-file or local process-admission I/O failure.
    #[error("signer I/O failed: {0}")]
    Io(#[from] std::io::Error),
}
