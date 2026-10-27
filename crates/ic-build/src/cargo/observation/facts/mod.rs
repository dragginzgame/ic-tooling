//! Generic tool and recipe observations, independent of artifact acquisition.

use crate::cargo::selection::CargoSelection;
use std::{collections::BTreeMap, ffi::OsString};

/// Tool and recipe observations from resolved inputs or an observed execution.
///
/// These facts alone do not establish successful compilation or bind an external
/// execution. `cargo::execution::execute_cargo` binds observations to its own
/// successful invocation. Consumers retain source custody and provenance
/// redaction. Environment values and probe bytes may contain secrets; Debug
/// reports only their sizes. Reading these observations performs no I/O.
#[derive(Clone, Eq, PartialEq)]
pub struct CargoBuildFacts {
    /// Selected profile, features and lock/network policy.
    pub selection: CargoSelection,
    /// Selected compilation target.
    pub target: String,
    /// Complete selected environment. Cache resolution excludes Build's owned
    /// target-directory override; custom execution retains its exact environment.
    pub environment: BTreeMap<OsString, OsString>,
    /// Exact successful Cargo probe stdout followed by stderr, as fingerprinted.
    pub cargo_identity: Vec<u8>,
    /// Exact successful compiler probe stdout followed by stderr, as fingerprinted.
    pub rustc_identity: Vec<u8>,
}

impl std::fmt::Debug for CargoBuildFacts {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("CargoBuildFacts")
            .field("selection", &self.selection)
            .field("target", &self.target)
            .field("environment_entries", &self.environment.len())
            .field("cargo_identity_bytes", &self.cargo_identity.len())
            .field("rustc_identity_bytes", &self.rustc_identity.len())
            .finish()
    }
}
