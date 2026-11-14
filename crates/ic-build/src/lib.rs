//! Shared host-side Cargo execution, input evidence and artifact capture.
//!
//! Callers select recipes, tools, output budgets, source custody and target lifetime.
//! The engine never owns a `PocketIC` lifecycle or a consumer's release identity.
//! Retain capture owners through artifact consumption. Cargo owns incremental
//! compilation; no persistent artifact cache or default elapsed-time deadline is added.
#![forbid(unsafe_code)]

pub mod artifacts;
pub mod cargo;
pub mod provenance;
pub mod tools;

#[cfg(any(test, feature = "test-support"))]
pub mod testing;
