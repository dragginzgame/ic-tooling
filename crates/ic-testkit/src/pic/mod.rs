//! `PocketIC` host-test harness extensions.
//!
//! Import the client and DTOs from `ic_testkit::pocket_ic`.
//! This module owns Testkit's harness methods and policy types.
//!
//! Installation and snapshot helpers are inherent methods on that client.
//! Diagnostic helpers are inherent methods on the client as well.
//! Import the call functions, startup configuration and polling helpers explicitly.
//! Native simulator operations are maintained in the client module.
//!
//! [`PocketIcManagedServer`] can explicitly own one exact caller-selected
//! server child for a serial suite. No type serializes independent instances or
//! downloads a server implicitly. On Unix, `server_assets` owns explicit
//! authenticated provisioning and admission; the published
//! `ic-testkit-server setup` / `check` commands delegate to that implementation.

#[cfg(feature = "reply-fixtures")]
pub mod fixtures;

mod server;
pub use server::{
    POCKET_IC_SERVER_VERSION, pocket_ic_server_requirement, supports_pocket_ic_server,
};
#[cfg(unix)]
pub mod server_assets;

mod baseline;
mod baseline_pool;
mod bounded_pool;
mod diagnostics;
mod errors;
mod lifecycle;
mod snapshot;
mod standalone;
mod standalone_recipe;
mod startup;
mod time;
mod transport;
#[cfg(feature = "wire")]
mod wire;

pub use baseline::CachedPocketIcBaseline;
pub use baseline_pool::{
    BaselinePoolDisposition, BaselinePoolError, BaselinePoolOutcome, BaselinePoolPreparationError,
    BaselinePoolTimings, BaselinePreparationStage, CachedPocketIcBaselinePool,
    CachedPocketIcBaselinePoolGuard, FailureDisposition, PocketIcBaselineRecipe, PreparedBaseline,
    RebuildReason,
};
pub use diagnostics::{
    CanisterDiagnosticLogRecord, CanisterDiagnosticLogs, CanisterDiagnosticsBatchEntry,
    CanisterDiagnosticsBatchReport, CanisterDiagnosticsReport, CanisterDiagnosticsRequest,
    CanisterLogRenderLimits, DEFAULT_CANISTER_LOG_BYTE_LIMIT, DEFAULT_CANISTER_LOG_RECORD_LIMIT,
    LabeledCanisterDiagnosticsRequest,
};
pub use errors::{CanisterInstallError, StandaloneCanisterInstallError};
pub use lifecycle::{InstallSpec, RetryPolicy};
pub use snapshot::{
    CanisterSnapshotTarget, ControllerSnapshotError, ControllerSnapshots, SnapshotCleanupFailure,
    SnapshotRestoreFunding,
};
pub use standalone::StandaloneCanisterFixture;
pub use standalone_recipe::StandaloneCanisterRecipe;
pub use startup::{
    PocketIcInstance, PocketIcManagedServer, PocketIcManagedServerOutput, PocketIcStartupConfig,
    PocketIcStartupError, PocketIcStartupFailure,
};
pub use time::{TickUntilError, advance_and_tick, tick_until};
pub use transport::is_dead_pocket_ic_transport_error;
#[cfg(feature = "wire")]
pub use wire::{WireCallContext, WireCallError, WireCallSpec, query_wire, update_wire};
