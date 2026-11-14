//! Invocation-owned artifact capture, input inventories and native extraction.
//!
//! Cargo executes through `crate::cargo::execution`; its artifact paths are mutable.
//! Capture outputs before another build can replace them. Retain the capture owner
//! through consumption or explicitly transfer it to a caller-owned destination.
//! Source/target custody and repeated-build policy remain caller-owned.

#[cfg(unix)]
pub mod candid;

mod digest;
mod inputs;
mod staging;

pub use digest::InputDigest;
pub use inputs::{
    InputEvidence, InputEvidenceError, InputEvidenceSpec, InputEvidenceValue, parse_cargo_depfile,
    read_cargo_depfile_inputs,
};
pub use staging::{CaptureRetentionError, CapturedArtifact, produce_and_retain_artifact};

/// Private keyed environment comparison evidence.
pub mod environment_comparison;
