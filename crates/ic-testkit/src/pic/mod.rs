//! `PocketIC` host-test harness extensions.
//!
//! Import the client and DTOs directly from the upstream `pocket_ic` dependency.
//! This module owns Testkit's harness methods and policy types.
//!
//! Installation, snapshot and diagnostic helpers are free functions taking the client.
//! Import the call functions, startup configuration and polling helpers explicitly.
//! Upstream owns native simulator operations and failure semantics.
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
mod lifecycle;
mod snapshot;
mod standalone;
mod standalone_recipe;
mod startup;
mod time;
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
    LabeledCanisterDiagnosticsRequest, collect_canister_diagnostics,
    collect_canister_diagnostics_batch, write_canister_diagnostics,
};
pub use lifecycle::{
    InstallSpec, RetryPolicy, create_and_install, retry_install_code,
    wait_out_install_code_rate_limit,
};
pub use snapshot::{
    CanisterSnapshotTarget, ControllerSnapshotError, ControllerSnapshots, SnapshotCleanupFailure,
    SnapshotRestoreFunding, capture_snapshots, restore_snapshots,
};
pub use standalone::StandaloneCanisterFixture;
pub use standalone_recipe::StandaloneCanisterRecipe;
pub use startup::{
    PocketIcInstance, PocketIcManagedServer, PocketIcManagedServerOutput, PocketIcStartupConfig,
    PocketIcStartupError, PocketIcStartupFailure,
};
pub use time::{TickUntilError, advance_and_tick, tick_until};
#[cfg(feature = "wire")]
pub use wire::{WireCallContext, WireCallError, WireCallSpec, query_wire, update_wire};
