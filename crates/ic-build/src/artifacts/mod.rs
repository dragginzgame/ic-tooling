//! Invocation-owned artifact capture, explicit output retention.
//!
//! Cargo executes through `crate::cargo::execution`; its artifact paths are mutable.
//! Capture outputs before another build can replace them. Retain the capture owner
//! through consumption or explicitly transfer it to a caller-owned destination.
//! Source/target custody and repeated-build policy remain caller-owned.

mod staging;

pub use staging::{CaptureRetentionError, CapturedArtifact, produce_and_retain_artifact};
