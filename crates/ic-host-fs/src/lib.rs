//! Local host filesystem reads, durable publication and descriptor locks.
//!
//! Optional Unix APIs: `capability` selects held directory/file capabilities;
//! `temporary-file` selects scoped named scratch files. Defaults select neither.
#![forbid(unsafe_code)]
#[cfg(all(unix, feature = "capability"))]
pub mod capability;
pub mod directory;
pub mod durable;
#[cfg(unix)]
pub mod identity;
pub mod path;
pub mod read;
pub mod scratch;
#[cfg(any(test, feature = "test-support"))]
pub mod testing;
#[cfg(test)]
mod tests;

pub mod owned_directory;
