//! Held filesystem identity, explicit publication and retained recovery-file access.
//!
//! Callers exclude concurrent namespace/content mutation. Checks detect observed
//! replacement; they do not make files immutable or atomically fence hostile writers.
mod directory;
mod lock;
mod publication;

pub use directory::{Access, Directory, Entry, Error, FileId, FilePolicy, Kind};
pub use lock::Lock;
pub use publication::{
    AttemptFailure, Commit, Destination, Phase, Policy, Receipt, Retention, Sealed, Stage, Written,
};
