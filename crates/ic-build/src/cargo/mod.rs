//! Explicit Cargo execution, bounded observations and package qualification.
//!
//! Responsibility: execute selected recipes, observe complete Cargo evidence and qualify package archives.
//! Does not own: recipe selection, source custody, role contracts or runtime lifecycle.
//! Boundary: consumers select recipes and feature profiles.

pub mod diagnostics;
pub mod discovery;
pub mod environment;
pub mod execution;
pub mod flags;
pub mod lock_projection;
pub(crate) mod lockfile;
pub(crate) mod messages;
#[cfg(unix)]
pub mod native;
pub mod observation;
pub mod packages;
pub mod release_metadata;
pub mod selection;
