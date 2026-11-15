//! Build one Wasm library into owned bytes with complete process and Cargo evidence.
//!
//! This is an ordinary, offline developer-build operation, not a release proof or
//! sandbox for hostile build scripts. The caller owns source/tool stability and
//! the scratch parent. See `build` for the cleanup and cancellation contract.

mod bytes;
mod custody;
mod operation;
mod process;
mod protocol;

pub use operation::{Artifact, BuildFailure, BuildRequest, Failure, Limits, build};

#[cfg(test)]
mod tests;
