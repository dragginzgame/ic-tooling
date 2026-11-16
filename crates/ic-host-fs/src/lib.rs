//! Held filesystem identity, explicit publication and retained recovery-file access.
//!
//! Callers exclude concurrent namespace/content mutation. Checks detect observed
//! replacement; they do not make files immutable or atomically fence hostile writers.
mod directory;
mod directory_publication;
mod lock;
mod publication;

pub use directory::{Access, Directory, Entry, Error, FileId, FilePolicy, Kind, PrepareFailure};
pub use directory_publication::{DirectoryPublication, DirectoryPublishError};
pub use lock::Lock;
pub use publication::{
    AttemptFailure, Commit, Destination, Phase, Policy, Receipt, Retention, Sealed, Stage, Written,
};
