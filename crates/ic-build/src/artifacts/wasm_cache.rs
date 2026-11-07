use super::phase::WasmBuildPhase;
use crate::cargo::{
    execution::{
        CargoInvocation, CargoRecipe, CargoTool, TOOL_IDENTITY_BYTES, communicate_cargo,
        wasm_artifact_paths,
    },
    messages::artifact_matches,
    observation::{facts::CargoBuildFacts, metadata_arguments, schema},
    selection::{CargoSelection, is_normal_component},
};
use ic_host_fs::owned_directory::OwnedDirectory;
use ic_host_process::{
    child::{CleanupPolicy, OwnedChild},
    tool::{
        CompletedExecution, ExecutionFailure, ExecutionOperation, OutputLimit, OutputLimits,
        ToolError, capture_command, capture_group_command_with_cancellation,
    },
};
use std::{
    collections::BTreeMap,
    ffi::{OsStr, OsString},
    fs::{self, File},
    io,
    path::{Path, PathBuf},
    process::{Command, ExitStatus, Stdio},
    sync::Arc,
    time::{Duration, Instant, SystemTime},
};

mod input_batch;
mod maintenance;
mod outputs;
mod progress;
mod resolution;
mod source_inputs;
pub(super) use input_batch::{WasmBuildBatchInputMetrics, WasmBuildBatchInputResolver};
#[cfg(test)]
use maintenance::integrated_shared_maintenance_result;
use maintenance::{
    SharedIncrementalTargetMaintenanceSchedule, maintain_shared_incremental_target_locked,
    perform_configured_shared_incremental_target_maintenance,
    perform_due_shared_incremental_target_maintenance,
    schedule_shared_incremental_target_maintenance,
};
use outputs::{ResolvedWasmOutputs, WasmArtifactTarget};
use progress::ProgressReporter;
use resolution::{ParsedCargoMetadata, digest_resolved_local_inputs, resolve_local_inputs};

use super::{
    cache_fs::{
        ArtifactCacheMaintenance, ArtifactCachePruneReport, CacheFsError, CacheRetentionLimits,
        RetainedCacheEntry, canonicalize_allow_missing,
        ensure_cache_directory_tag as ensure_cache_tag, finish_cache_build, is_sha256_directory,
        lock_cache_file, lock_cache_file_with_wait_observer, perform_scheduled_cache_maintenance,
        prune_direct_child_directories, record_cache_entry_use as record_entry_use,
        remove_unretained_entry,
    },
    digest::{
        FileDigest, InputDigest, InputHasher, LabeledPathDigestCache, copy_file_atomic,
        destination_matches_bytes, digest_file, digest_labeled_paths_composable,
        materialize_verified_output, os_bytes, read_stamp_with_limit,
    },
};

const CACHE_FORMAT_VERSION: &str = "ic-testkit-wasm-build-v1";
// Version the recipe key separately from retained artifact/stamp formats.
const BUILD_FINGERPRINT_DOMAIN: &str = "ic-build-explicit-environment-fingerprint";
const ARTIFACT_STAMP_DIGEST_LABEL: &str = "artifact-sha256:";
// Consumer policy: diagnostic prefixes, complete probe output, and no deadline.
const CARGO_DIAGNOSTIC_BYTES: usize = 1024 * 1024;
const CARGO_METADATA_BYTES: usize = 16 * 1024 * 1024;

const DEFAULT_TARGET: &str = "wasm32-unknown-unknown";

/// Complete caller-owned description of one cacheable Cargo Wasm build.
///
/// The selected package graph, sources, semantic workspace projection, Cargo
/// configuration, Rust toolchain files, target, profile arguments, explicit
/// complete child environment and additional watched
/// inputs contribute to the build fingerprint. The complete workspace
/// manifest and lockfile remain conservative mutation-validation inputs.
#[derive(Clone, Eq, PartialEq)]
pub struct WasmBuildSpec {
    configuration_limits: crate::cargo::environment::ConfigurationLimits,
    workspace_root: PathBuf,
    target_dir: PathBuf,
    packages: Vec<String>,
    example: Option<String>,
    cargo_selection: CargoSelection,
    environment: BTreeMap<OsString, OsString>,
    additional_inputs: Vec<PathBuf>,
    target: String,
    cargo_program: OsString,
    rustc_program: OsString,
    cargo_message_bytes: usize,
    cache_mode: WasmBuildCacheMode,
    pruning: Option<(CacheRetentionLimits, Duration)>,
    shared_incremental_maintenance_config: Option<SharedIncrementalTargetMaintenanceConfig>,
}

impl std::fmt::Debug for WasmBuildSpec {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        formatter
            .debug_struct("WasmBuildSpec")
            .field("configuration_limits", &self.configuration_limits)
            .field("workspace_root", &self.workspace_root)
            .field("target_dir", &self.target_dir)
            .field("packages", &self.packages)
            .field("example", &self.example)
            .field("cargo_selection", &self.cargo_selection)
            .field(
                "environment_names",
                &self.environment.keys().collect::<Vec<_>>(),
            )
            .field("additional_inputs", &self.additional_inputs)
            .field("target", &self.target)
            .field("cargo_program", &self.cargo_program)
            .field("rustc_program", &self.rustc_program)
            .field("cargo_message_bytes", &self.cargo_message_bytes)
            .field("cache_mode", &self.cache_mode)
            .field("pruning", &self.pruning)
            .field(
                "shared_incremental_maintenance_config",
                &self.shared_incremental_maintenance_config,
            )
            .finish()
    }
}

/// Failure handling for integrated shared incremental-target maintenance.
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub enum SharedIncrementalTargetMaintenanceFailureMode {
    /// Fail the Wasm acquisition when scheduled maintenance fails.
    #[default]
    Strict,
    /// Preserve the acquisition and attach a structured failed-maintenance outcome.
    BestEffort,
}

/// Scheduled shared incremental-target maintenance attached to a Wasm acquisition.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct SharedIncrementalTargetMaintenanceConfig {
    policy: CacheRetentionLimits,
    minimum_interval: Duration,
    failure_mode: SharedIncrementalTargetMaintenanceFailureMode,
}

/// Cargo-target ownership mode for one exact cached Wasm build.
#[derive(Clone, Debug, Eq, PartialEq)]
enum WasmBuildCacheMode {
    /// Build each exact fingerprint in its own content-addressed Cargo target.
    Isolated,
    /// Build misses in caller-owned shared Cargo incremental state, then cache final Wasm files.
    SharedIncremental {
        /// Mutable Cargo target directory shared across source fingerprints.
        target_dir: PathBuf,
    },
}

/// Whether a cacheable Wasm build ran Cargo or reused exact matching artifacts.
#[derive(Clone, Debug, Eq, PartialEq)]
pub enum WasmBuildOutcome {
    /// Cargo ran and a new successful stamp was published.
    Built(WasmBuildRecord),
    /// Existing artifacts and their content-addressed stamp matched exactly.
    Reused(WasmBuildRecord),
}

/// Details shared by built and reused Wasm outcomes.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct WasmBuildRecord {
    facts: CargoBuildFacts,
    fingerprint: InputDigest,
    recipe_identity: InputDigest,
    input_digest: InputDigest,
    retention: RetainedCacheEntry,
    artifacts: Arc<[PathBuf]>,
    timings: WasmBuildTimings,
    maintenance: Option<ArtifactCacheMaintenance>,
    shared_incremental_maintenance: Option<SharedIncrementalTargetMaintenanceOutcome>,
}

/// Timings for cache coordination, input resolution, and Cargo execution.
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct WasmBuildTimings {
    /// Time spent waiting for the output-directory process lock.
    pub lock_wait: Duration,
    /// Time spent waiting for a shared incremental-target lock, when configured.
    pub shared_incremental_lock_wait: Option<Duration>,
    /// Detailed tool, metadata, discovery, and hashing timings.
    pub input_resolution: WasmInputResolutionTimings,
    /// Time spent in `cargo build`, or `None` for a cache hit.
    pub cargo_build: Option<Duration>,
    /// Time spent on configured best-effort cache maintenance.
    pub cache_maintenance: Option<Duration>,
    /// Total operation duration, including lock coordination.
    pub total: Duration,
}

/// Detailed timings for exact Wasm build-input resolution.
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct WasmInputResolutionTimings {
    /// Time spent reading Cargo and rustc identities.
    pub tool_identity: Duration,
    /// Time spent running and decoding `cargo metadata`.
    pub cargo_metadata: Duration,
    /// Time spent resolving packages, configuration, and watched paths.
    pub input_discovery: Duration,
    /// Time spent reading and hashing exact input contents.
    pub content_hashing: Duration,
    /// Complete input-resolution duration.
    pub total: Duration,
}

/// Partial phase timings retained when a Wasm acquisition fails.
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct WasmBuildFailureTimings {
    /// Time spent coordinating the exact artifact cache before failure.
    pub exact_cache_coordination: Duration,
    /// Time spent coordinating a shared incremental target, when reached.
    pub shared_target_coordination: Option<Duration>,
    /// Partial Cargo/rustc identity, metadata, discovery, and hashing timings.
    pub input_resolution: WasmInputResolutionTimings,
    /// Time spent on shared-target maintenance, when reached.
    pub shared_target_maintenance: Option<Duration>,
    /// Time spent executing Cargo, including an unsuccessful execution.
    pub cargo_build: Option<Duration>,
    /// Time spent validating or publishing artifacts, when reached.
    pub artifact_publication: Option<Duration>,
    /// Time spent on exact-cache maintenance, when reached.
    pub exact_cache_maintenance: Option<Duration>,
    /// Explicit incomplete-entry cleanup time, when failure required it.
    pub cleanup: Option<Duration>,
    /// Complete failed acquisition wall time.
    pub total: Duration,
}

/// Structured phase and partial timings for one failed Wasm entry.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct WasmBuildFailureDetails {
    phase: WasmBuildPhase,
    timings: WasmBuildFailureTimings,
}

/// One exact local Cargo source or configuration input under a stable logical label.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct CargoBuildInput {
    label: PathBuf,
    path: PathBuf,
}

/// Resolved exact inputs and identity for one [`WasmBuildSpec`].
///
/// The snapshot can be resolved again after an external operation to detect
/// source, configuration, toolchain, argument, or environment changes.
/// Clones share immutable input and exclusion lists while retaining independent
/// timing values.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ResolvedCargoBuildInputs {
    facts: CargoBuildFacts,
    recipe_identity: InputDigest,
    fingerprint: InputDigest,
    input_digest: InputDigest,
    validation_digest: InputDigest,
    inputs: Arc<[CargoBuildInput]>,
    // Re-derive nested targets during guards; a former generated root can become source.
    base_exclusions: Arc<[PathBuf]>,
    declared_package_files: Arc<[PathBuf]>,
    exclusions: Arc<[PathBuf]>,
    wasm_outputs: Result<ResolvedWasmOutputs, String>,
    timings: WasmInputResolutionTimings,
}

struct DiscoveredBuildInputs {
    inputs: ResolvedLocalInputs,
    exclusions: Vec<PathBuf>,
}

struct HashedBuildInputs {
    inputs: ResolvedLocalInputs,
    exclusions: Vec<PathBuf>,
    input_digest: InputDigest,
    validation_digest: InputDigest,
}

struct ResolvedLocalInputs {
    validation_inputs: Vec<(PathBuf, PathBuf)>,
    base_exclusions: Vec<PathBuf>,
    declared_package_files: Vec<PathBuf>,
    workspace_projection: Option<InputDigest>,
    configuration_identity: InputDigest,
    wasm_targets: Result<Vec<WasmArtifactTarget>, String>,
}

impl DiscoveredBuildInputs {
    fn resolve(
        spec: &WasmBuildSpec,
        metadata: &ParsedCargoMetadata<'_>,
    ) -> Result<Self, WasmBuildError> {
        let (inputs, exclusions) = resolve_local_inputs(spec, metadata)?;
        Ok(Self { inputs, exclusions })
    }

    fn hash(
        mut self,
        cache: &mut LabeledPathDigestCache,
        declarations: &mut source_inputs::WorkspaceDeclarations,
        workspace_root: &Path,
        operations: (&'static str, &'static str),
    ) -> Result<HashedBuildInputs, WasmBuildError> {
        let (input_digest, validation_digest) = digest_resolved_local_inputs(
            &self.inputs,
            &mut self.exclusions,
            cache,
            declarations,
            workspace_root,
            operations.0,
            operations.1,
        )?;
        Ok(HashedBuildInputs {
            inputs: self.inputs,
            exclusions: self.exclusions,
            input_digest,
            validation_digest,
        })
    }
}

/// Result of explicit shared Cargo target maintenance.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct SharedIncrementalTargetMaintenance {
    target_dir: PathBuf,
    logical_size_bytes_before: u64,
    logical_size_bytes_after: u64,
    last_used_before: SystemTime,
    cleared: bool,
    lock_wait: Duration,
    maintenance: Duration,
}

/// Result of interval-limited shared Cargo target maintenance.
#[non_exhaustive]
#[derive(Clone, Debug, Eq, PartialEq)]
pub enum SharedIncrementalTargetMaintenanceOutcome {
    /// The configured shared target does not exist, so nothing was created or inspected.
    Missing {
        /// Configured target path. A missing path cannot necessarily be canonicalized.
        target_dir: PathBuf,
    },
    /// A successful matching maintenance pass is still inside the requested interval.
    Skipped {
        /// Canonical shared Cargo target directory.
        target_dir: PathBuf,
        /// Time spent waiting for another process using the shared target.
        lock_wait: Duration,
        /// Time spent checking the small cross-process schedule marker.
        schedule_check: Duration,
    },
    /// Retention was evaluated under the shared-target lock.
    Performed {
        /// Completed retention report.
        maintenance: SharedIncrementalTargetMaintenance,
        /// Time spent checking the small cross-process schedule marker.
        schedule_check: Duration,
    },
    /// Integrated best-effort maintenance failed without invalidating the Wasm acquisition.
    Failed {
        /// Canonical shared Cargo target directory.
        target_dir: PathBuf,
        /// Time spent waiting for another process using the shared target.
        lock_wait: Duration,
        /// Rendered maintenance failure retained for diagnostics.
        message: String,
    },
}

/// Observation settings for one cacheable Wasm build.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct WasmBuildProgressConfig {
    heartbeat_interval: Duration,
}

/// Phase-aware heartbeat emitted while a cacheable Wasm build is busy.
#[non_exhaustive]
#[derive(Clone, Debug, Eq, PartialEq)]
pub enum WasmBuildProgressEvent {
    /// The current phase remained active until the next heartbeat.
    Heartbeat {
        /// Phase currently making or waiting for progress.
        phase: WasmBuildPhase,
        /// Time elapsed since this phase started.
        elapsed: Duration,
    },
}

impl Default for WasmBuildProgressConfig {
    fn default() -> Self {
        Self {
            heartbeat_interval: Duration::from_secs(10),
        }
    }
}

impl WasmBuildProgressConfig {
    /// Observe acquisition progress and emit a heartbeat at least every ten quiet seconds.
    #[must_use]
    pub fn new() -> Self {
        Self::default()
    }

    /// Select the maximum quiet interval between phase-aware heartbeat events.
    ///
    /// A zero interval is rejected before any build work begins.
    #[must_use]
    pub const fn with_heartbeat_interval(mut self, interval: Duration) -> Self {
        self.heartbeat_interval = interval;
        self
    }

    /// Configured positive heartbeat interval.
    #[must_use]
    pub const fn heartbeat_interval(self) -> Duration {
        self.heartbeat_interval
    }
}

/// Structured failure from a cacheable Wasm build.
#[non_exhaustive]
#[derive(Debug, thiserror::Error)]
pub enum WasmBuildError {
    /// The caller supplied an incomplete or inconsistent specification.
    #[error("invalid Wasm build spec: {message}")]
    InvalidSpec {
        /// Diagnostic describing the rejected input or evidence.
        message: String,
    },
    /// A filesystem operation failed.
    #[error("failed to {operation} at {}: {source}", .path.display())]
    Io {
        /// Filesystem or process operation that failed.
        operation: &'static str,
        /// Exact path involved in the failed operation.
        path: PathBuf,
        /// Original underlying error, including its typed failure evidence.
        source: io::Error,
    },
    /// An external command could not be launched.
    #[error("failed to launch {phase} using `{}`: {source}", .program.to_string_lossy())]
    CommandSpawn {
        /// Build phase in which the command failed.
        phase: WasmBuildPhase,
        /// Selected executable that could not be launched.
        program: OsString,
        /// Original underlying error, including its typed failure evidence.
        source: io::Error,
    },
    /// An external command completed unsuccessfully.
    /// Cargo-build streams retain at most 1 MiB of raw bytes each, with an explicit marker
    /// if truncated; optional progress callbacks still receive all output.
    #[error("{phase} failed with {status}\nstdout:\n{stdout}\nstderr:\n{stderr}")]
    CommandFailed {
        /// Build phase in which the command failed.
        phase: WasmBuildPhase,
        /// Original child exit status.
        status: ExitStatus,
        /// Retained standard output from the original Cargo invocation.
        stdout: String,
        /// Retained standard error from the original Cargo invocation.
        stderr: String,
    },
    /// Cargo metadata did not contain the expected package graph.
    #[error("invalid Cargo metadata: {message}")]
    InvalidMetadata {
        /// Diagnostic describing the rejected input or evidence.
        message: String,
    },
    /// A discovered Cargo configuration could not be interpreted exactly.
    #[error("invalid Cargo configuration at {}: {message}", .path.display())]
    InvalidCargoConfiguration {
        /// Exact path involved in the failed operation.
        path: PathBuf,
        /// Diagnostic describing the unsupported or malformed Cargo configuration.
        message: String,
    },
    /// Cargo succeeded without producing every declared Wasm artifact.
    #[error("cargo build succeeded without producing: {}", MissingArtifactList(.paths))]
    MissingArtifacts {
        /// Expected artifact paths that were not produced.
        paths: Vec<PathBuf>,
    },
    /// Declared inputs changed while acquiring or building Wasm artifacts.
    #[error("Wasm inputs changed during artifact acquisition: {before} -> {after}")]
    InputsChangedDuringAcquisition {
        /// Input identity captured before the operation.
        before: InputDigest,
        /// Input identity observed during revalidation.
        after: InputDigest,
    },
    /// A build failed and its incomplete fingerprint directory could not be removed.
    #[error("Wasm build failed ({build_error}) and its incomplete target directory at {} could not be removed: {source}", .path.display())]
    FailedBuildCleanup {
        /// Original build failure before cleanup was attempted.
        build_error: Box<Self>,
        /// Exact path involved in the failed operation.
        path: PathBuf,
        /// Original underlying error, including its typed failure evidence.
        source: io::Error,
    },
}

struct MissingArtifactList<'a>(&'a [PathBuf]);

impl std::fmt::Display for MissingArtifactList<'_> {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for (index, path) in self.0.iter().enumerate() {
            if index != 0 {
                formatter.write_str(", ")?;
            }
            write!(formatter, "{}", path.display())?;
        }
        Ok(())
    }
}

impl WasmBuildSpec {
    /// Set the maximum bytes in one Cargo build message, including its newline.
    ///
    /// The default is 16 MiB. This decoding budget does not affect fingerprints
    /// or impose a build deadline. Overflow refuses publication after Cargo
    /// finishes; diagnostics continue streaming and the child is not terminated.
    #[must_use]
    pub const fn with_cargo_message_bytes(mut self, maximum: usize) -> Self {
        self.cargo_message_bytes = maximum;
        self
    }

    /// Describe one Cargo build targeting `wasm32-unknown-unknown`.
    ///
    /// The output subdirectory is derived from [`Self::with_cargo_selection`]:
    /// the default, `dev`, and `test` use `debug`; `release` and `bench` use
    /// `release`; custom profiles use their names. The derived directory must
    /// be one normal path component so artifacts remain inside their targets.
    /// Relative `workspace_root` and exact `target_dir` paths are resolved from
    /// the caller's working directory. Shared targets are workspace-relative.
    /// Package names are sorted and deduplicated for identity, discovery,
    /// Cargo invocation, and artifact paths.
    /// Misses share the workspace-relative `target/ic-testkit-incremental`
    /// Cargo target by default. Use [`Self::with_isolated_builds`] when compiler
    /// state must be independent, and select retention limits explicitly.
    /// By default, acquisition builds package libraries with Cargo's `--lib`
    /// flag. Use [`Self::with_example`] for one named `cdylib` example.
    /// Each default-library package must declare a `cdylib` with Cargo's default
    /// library name: package hyphens become underscores. Artifact paths use that
    /// admitted name; genuinely renamed libraries are refused.
    ///
    /// `configuration_limits` bounds file bytes, total bytes, file count and include
    /// depth during each configuration observation. Callers choose these budgets.
    ///
    /// `environment` is the complete environment for probes and builds; child
    /// inheritance is cleared. Select PATH, `HOME`/`CARGO_HOME`, Rustup and build
    /// inputs at the caller as required. CARGO and RUSTC in this map select the
    /// initial tool programs; otherwise the literal programs cargo/rustc are used.
    #[must_use]
    pub fn new<I, K, V>(
        workspace_root: &Path,
        target_dir: &Path,
        packages: &[&str],
        environment: I,
        configuration_limits: crate::cargo::environment::ConfigurationLimits,
    ) -> Self
    where
        I: IntoIterator<Item = (K, V)>,
        K: Into<OsString>,
        V: Into<OsString>,
    {
        let environment: BTreeMap<OsString, OsString> = environment
            .into_iter()
            .map(|(key, value)| (key.into(), value.into()))
            .collect();
        let mut packages = packages
            .iter()
            .map(|package| (*package).to_owned())
            .collect::<Vec<_>>();
        packages.sort();
        packages.dedup();
        Self {
            configuration_limits,
            workspace_root: workspace_root.to_owned(),
            target_dir: target_dir.to_owned(),
            packages,
            example: None,
            cargo_selection: CargoSelection::default(),
            cargo_program: environment
                .get(OsStr::new("CARGO"))
                .cloned()
                .unwrap_or_else(|| "cargo".into()),
            rustc_program: environment
                .get(OsStr::new("RUSTC"))
                .cloned()
                .unwrap_or_else(|| "rustc".into()),
            environment,
            additional_inputs: Vec::new(),
            target: DEFAULT_TARGET.to_owned(),
            cargo_message_bytes: CARGO_METADATA_BYTES,
            cache_mode: WasmBuildCacheMode::SharedIncremental {
                target_dir: PathBuf::from("target/ic-testkit-incremental"),
            },
            pruning: None,
            shared_incremental_maintenance_config: None,
        }
    }

    /// Select one named `cdylib` example instead of the default package library.
    ///
    /// Requires exactly one package. Metadata must declare that exact example;
    /// its selection contributes to identity and outputs live under `examples/`.
    /// Raw example/other-target selectors in profile arguments remain refused.
    #[must_use]
    pub fn with_example(mut self, name: impl Into<String>) -> Self {
        self.example = Some(name.into());
        self
    }

    /// Borrow the explicit example selection, if this is not a library recipe.
    #[must_use]
    pub fn example(&self) -> Option<&str> {
        self.example.as_deref()
    }

    /// Prepare this recipe's Cargo command without starting a process or cache.
    ///
    /// External execution uses `target_dir` directly; cache/maintenance choices
    /// are not executed. Resolve inputs first and borrow their admitted output
    /// paths, retain source/target custody, execute this command with caller-owned
    /// deadline/cancellation/cleanup, and validate inputs before capturing outputs.
    /// Keep the selected arguments and environment intact. Command success alone
    /// does not admit an artifact or release.
    ///
    /// # Errors
    /// Returns invalid selections or target-directory resolution errors.
    pub fn cargo_build_command(&self) -> Result<Command, WasmBuildError> {
        validate_spec(self)?;
        cargo_build_command(self, &self.target_dir)
    }

    /// Replace typed Cargo profile, feature and network/lock selections.
    ///
    /// The same selection configures metadata, builds and cache identity. Package,
    /// target, output paths and diagnostics remain owned by this specification.
    #[must_use]
    pub fn with_cargo_selection(mut self, selection: CargoSelection) -> Self {
        self.cargo_selection = selection;
        self
    }

    /// Replace the complete environment used by identity probes, metadata and builds.
    ///
    /// Inheritance is cleared. Absent names stay absent; duplicate names use the
    /// last supplied value. Consumers own capture, filtering and recapture policy.
    /// Cargo target directories remain owned by the build specification.
    #[must_use]
    pub fn with_environment<I, K, V>(mut self, environment: I) -> Self
    where
        I: IntoIterator<Item = (K, V)>,
        K: Into<OsString>,
        V: Into<OsString>,
    {
        self.environment = environment
            .into_iter()
            .map(|(key, value)| (key.into(), value.into()))
            .collect();
        self
    }

    /// Borrow the explicitly selected child environment; values may contain secrets.
    #[must_use]
    pub const fn environment(&self) -> &BTreeMap<OsString, OsString> {
        &self.environment
    }

    /// Add files or directories not discoverable through Cargo's local dependency graph.
    ///
    /// Relative paths are resolved from the workspace root. Use this for build
    /// script configuration, generated schemas, or other externally read inputs.
    #[must_use]
    pub fn with_additional_inputs<I, P>(mut self, paths: I) -> Self
    where
        I: IntoIterator<Item = P>,
        P: Into<PathBuf>,
    {
        self.additional_inputs
            .extend(paths.into_iter().map(Into::into));
        self
    }

    /// Override the Cargo compilation target.
    #[must_use]
    pub fn with_target(mut self, target: &str) -> Self {
        target.clone_into(&mut self.target);
        self
    }

    /// Override the Cargo executable used by metadata, identity, and build commands.
    #[must_use]
    pub fn with_cargo_program(mut self, program: impl Into<OsString>) -> Self {
        self.cargo_program = program.into();
        self
    }

    /// Override the Rust compiler executable used to fingerprint the toolchain.
    ///
    /// This selects the `-vV` identity probe; it does not change the compiler
    /// invoked by Cargo or infer a compiler hidden inside a Cargo shim. Supply
    /// the actual shim-selected compiler. To configure Cargo and its identity
    /// together, select `RUSTC` in the complete environment; that explicit environment
    /// selection takes precedence over this identity-only setting.
    #[must_use]
    pub fn with_rustc_identity_program(mut self, program: impl Into<OsString>) -> Self {
        self.rustc_program = program.into();
        self
    }

    /// Build cache misses in one caller-owned shared Cargo incremental target.
    ///
    /// Exact final Wasm artifacts still live in the content-addressed cache.
    /// The shared target is coordinated across processes but is never pruned
    /// or removed by IC Build after a failed build.
    #[must_use]
    pub fn with_shared_incremental_target(mut self, target_dir: impl Into<PathBuf>) -> Self {
        self.cache_mode = WasmBuildCacheMode::SharedIncremental {
            target_dir: target_dir.into(),
        };
        self
    }

    /// Build each fingerprint in a separate Cargo target instead of sharing state.
    ///
    /// Select this when the test explicitly requires isolated compiler state.
    /// Retained targets need a caller-selected prune policy to bound disk use.
    #[must_use]
    pub fn with_isolated_builds(mut self) -> Self {
        self.cache_mode = WasmBuildCacheMode::Isolated;
        self
    }

    /// Schedule caller-owned shared-target retention as part of acquisition.
    ///
    /// This option requires shared incremental mode (the default). Every
    /// acquisition coordinates through that target, including an exact hit,
    /// so a missing target can be created and receive its first schedule
    /// marker immediately. Matching recent passes only check the marker; due
    /// passes reuse the acquisition's exact Cargo input resolution before
    /// evaluating retention. The structured result is attached to the build
    /// record and emitted through observed progress. Maintenance failures fail
    /// the acquisition and do not record a successful schedule marker.
    #[must_use]
    pub const fn with_shared_incremental_target_maintenance_at_most_every(
        mut self,
        policy: CacheRetentionLimits,
        minimum_interval: Duration,
    ) -> Self {
        self.shared_incremental_maintenance_config = Some(
            SharedIncrementalTargetMaintenanceConfig::new(policy, minimum_interval),
        );
        self
    }

    /// Attach an explicit shared-target maintenance configuration.
    ///
    /// This is the configurable counterpart to
    /// [`Self::with_shared_incremental_target_maintenance_at_most_every`] and
    /// supports strict or best-effort failure handling.
    #[must_use]
    pub const fn with_shared_incremental_target_maintenance(
        mut self,
        config: SharedIncrementalTargetMaintenanceConfig,
    ) -> Self {
        self.shared_incremental_maintenance_config = Some(config);
        self
    }

    /// Apply cache retention under the build operation's existing process lock.
    ///
    /// Maintenance is best-effort: its structured result is attached to the
    /// successful build record and cannot turn ready artifacts into a build
    /// failure. The active fingerprint is protected from this pruning pass.
    #[must_use]
    pub const fn with_prune_policy(mut self, policy: CacheRetentionLimits) -> Self {
        self.pruning = Some((policy, Duration::ZERO));
        self
    }

    /// Apply exact-entry retention at most once per `minimum_interval`.
    ///
    /// The active fingerprint remains protected. A zero interval is equivalent
    /// to [`Self::with_prune_policy`]. The interval covers attempted
    /// maintenance, including a nonfatal failed attempt. This schedule never
    /// owns or scans a caller-owned shared incremental Cargo target.
    #[must_use]
    pub const fn with_prune_policy_at_most_every(
        mut self,
        policy: CacheRetentionLimits,
        minimum_interval: Duration,
    ) -> Self {
        self.pruning = Some((policy, minimum_interval));
        self
    }

    /// Workspace containing the selected Cargo packages.
    #[must_use]
    pub fn workspace_root(&self) -> &Path {
        &self.workspace_root
    }

    /// Cargo target directory containing artifacts, lock, and stamps.
    #[must_use]
    pub fn target_dir(&self) -> &Path {
        &self.target_dir
    }

    /// Selected Cargo package names in sorted order, without duplicates.
    #[must_use]
    pub fn packages(&self) -> &[String] {
        &self.packages
    }

    /// Shared incremental-target maintenance attached to this specification.
    #[must_use]
    pub const fn shared_incremental_target_maintenance(
        &self,
    ) -> Option<SharedIncrementalTargetMaintenanceConfig> {
        self.shared_incremental_maintenance_config
    }
}

impl WasmBuildOutcome {
    /// Read the common build record.
    #[must_use]
    pub const fn record(&self) -> &WasmBuildRecord {
        match self {
            Self::Built(record) | Self::Reused(record) => record,
        }
    }

    /// Report whether exact matching artifacts were reused.
    #[must_use]
    pub const fn is_reused(&self) -> bool {
        matches!(self, Self::Reused(_))
    }
}

impl WasmBuildRecord {
    /// Borrow the input observations bound to this artifact's fingerprint.
    ///
    /// Both built and reused records retain the selected recipe and tool probes.
    /// Reuse does not claim that a compiler ran during this acquisition. These
    /// facts perform no I/O; consumers own execution binding and redaction.
    #[must_use]
    pub const fn facts(&self) -> &CargoBuildFacts {
        &self.facts
    }

    /// Exact build fingerprint used by the atomic cache stamp.
    #[must_use]
    pub const fn fingerprint(&self) -> InputDigest {
        self.fingerprint
    }

    /// Source-independent comparison identity retained from the resolved snapshot.
    /// See [`ResolvedCargoBuildInputs::recipe_identity`] for coverage and limitations.
    #[must_use]
    pub const fn recipe_identity(&self) -> InputDigest {
        self.recipe_identity
    }

    /// Semantic digest of selected package sources and configuration inputs.
    #[must_use]
    pub const fn input_digest(&self) -> InputDigest {
        self.input_digest
    }

    /// Immutable content-addressed cache directory for this exact build.
    ///
    /// The directory is selected by the build fingerprint and contains the
    /// cached Wasm artifacts and their stamps. The record and its clones retain
    /// this entry against pruning until their last drop. A copied path alone
    /// does not retain ownership. Treat the contents as read-only.
    #[must_use]
    pub fn exact_cache_path(&self) -> &Path {
        self.retention.path()
    }

    /// Exact, read-only Wasm paths retained until this record and its clones drop.
    ///
    /// Keep a record alive throughout post-link processing and reading. Configured
    /// materialization destinations remain mutable and are not retained.
    #[must_use]
    pub fn artifacts(&self) -> &[PathBuf] {
        &self.artifacts
    }

    /// Phase timings captured by the cacheable build operation.
    #[must_use]
    pub const fn timings(&self) -> WasmBuildTimings {
        self.timings
    }

    /// Cache maintenance attempted under the build lock, when configured.
    #[must_use]
    pub const fn maintenance(&self) -> Option<&ArtifactCacheMaintenance> {
        self.maintenance.as_ref()
    }

    /// Scheduled caller-owned shared-target maintenance, when configured.
    #[must_use]
    pub const fn shared_incremental_maintenance(
        &self,
    ) -> Option<&SharedIncrementalTargetMaintenanceOutcome> {
        self.shared_incremental_maintenance.as_ref()
    }
}

impl WasmBuildTimings {
    pub(super) fn saturating_add(self, other: Self) -> Self {
        let mut input_resolution = self.input_resolution;
        input_resolution.include(other.input_resolution);
        Self {
            lock_wait: self.lock_wait.saturating_add(other.lock_wait),
            shared_incremental_lock_wait: super::sum_optional_durations(
                self.shared_incremental_lock_wait,
                other.shared_incremental_lock_wait,
            ),
            input_resolution,
            cargo_build: super::sum_optional_durations(self.cargo_build, other.cargo_build),
            cache_maintenance: super::sum_optional_durations(
                self.cache_maintenance,
                other.cache_maintenance,
            ),
            total: self.total.saturating_add(other.total),
        }
    }
}

impl WasmInputResolutionTimings {
    const fn include(&mut self, other: Self) {
        self.tool_identity = self.tool_identity.saturating_add(other.tool_identity);
        self.cargo_metadata = self.cargo_metadata.saturating_add(other.cargo_metadata);
        self.input_discovery = self.input_discovery.saturating_add(other.input_discovery);
        self.content_hashing = self.content_hashing.saturating_add(other.content_hashing);
        self.total = self.total.saturating_add(other.total);
    }
}

impl WasmBuildFailureDetails {
    pub(super) fn specification(total: Duration) -> Self {
        Self {
            phase: WasmBuildPhase::Specification,
            timings: WasmBuildFailureTimings {
                total,
                ..WasmBuildFailureTimings::default()
            },
        }
    }

    /// Primary acquisition phase that returned the failure.
    #[must_use]
    pub const fn phase(self) -> WasmBuildPhase {
        self.phase
    }

    /// Partial phase timings retained before the failure returned.
    #[must_use]
    pub const fn timings(self) -> WasmBuildFailureTimings {
        self.timings
    }
}

impl CargoBuildInput {
    /// Stable checkout-independent label used while hashing this input.
    #[must_use]
    pub fn label(&self) -> &Path {
        &self.label
    }

    /// Resolved file or directory read by the Cargo build.
    ///
    /// Configuration inputs preserve their lookup paths so mutation guards
    /// observe replaced symlinks as well as changed file contents.
    #[must_use]
    pub fn path(&self) -> &Path {
        &self.path
    }
}

/// Caller-owned controls for Cargo identity, compiler identity and metadata probes.
///
/// Each probe runs in an owned process group using Host's existing communication
/// and cleanup machinery. Output limits remain fixed by IC Build. These controls
/// do not impose a deadline on filesystem discovery or hashing, execute a build,
/// or alter fingerprints. Existing resolution APIs retain their unbounded defaults.
pub struct CargoInputProbePolicy<'a> {
    /// Absolute deadline shared by every probe, including subsequent revalidation.
    /// Spawning syscalls and cleanup can exceed it; `None` imposes no deadline.
    pub deadline: Option<Instant>,
    /// Termination grace and reaping policy, separate from the probe deadline.
    pub cleanup: CleanupPolicy,
    /// Cancellation predicate, polled even while a probe is silent.
    /// It must return promptly; callback panics attempt cleanup before unwinding.
    pub cancelled: &'a mut (dyn FnMut() -> bool + Send),
}

/// Exact acquisition identity, with conservative mutation evidence checked first.
#[derive(Clone, Copy, Debug)]
pub(super) struct CargoInputIdentity {
    pub(super) fingerprint: InputDigest,
    pub(super) validation_digest: InputDigest,
}

impl CargoInputIdentity {
    pub(super) fn difference(self, current: Self) -> Option<(InputDigest, InputDigest)> {
        [
            (self.validation_digest, current.validation_digest),
            (self.fingerprint, current.fingerprint),
        ]
        .into_iter()
        .find(|(before, after)| before != after)
    }
}

impl ResolvedCargoBuildInputs {
    pub(super) const fn identity(&self) -> CargoInputIdentity {
        CargoInputIdentity {
            fingerprint: self.fingerprint,
            validation_digest: self.validation_digest,
        }
    }

    /// Borrow the recipe and tool observations used by this snapshot, without I/O.
    #[must_use]
    pub const fn facts(&self) -> &CargoBuildFacts {
        &self.facts
    }

    fn assemble(
        spec: &WasmBuildSpec,
        hashed: HashedBuildInputs,
        facts: CargoBuildFacts,
        timings: impl FnOnce() -> WasmInputResolutionTimings,
    ) -> Self {
        let HashedBuildInputs {
            inputs,
            exclusions,
            input_digest,
            validation_digest,
        } = hashed;
        Self {
            fingerprint: finish_build_fingerprint(spec, &facts, input_digest),
            recipe_identity: resolution::recipe_identity(
                spec,
                &facts,
                inputs.configuration_identity,
            ),
            facts,
            input_digest,
            validation_digest,
            inputs: cargo_build_inputs(inputs.validation_inputs),
            base_exclusions: inputs.base_exclusions.into(),
            declared_package_files: inputs.declared_package_files.into(),
            exclusions: exclusions.into(),
            wasm_outputs: inputs
                .wasm_targets
                .map(|targets| ResolvedWasmOutputs::new(spec, targets)),
            // Fresh-resolution totals include path conversion and assembly.
            timings: timings(),
        }
    }

    /// Borrow declared output paths after metadata admits the selected targets.
    ///
    /// These are paths under the specification's `target_dir`, not retained or
    /// captured outputs. External producers must execute the same recipe under
    /// source/target custody and validate before consuming or capturing bytes.
    /// Generic snapshots for non-Wasm packages remain useful independently.
    ///
    /// # Errors
    /// Returns missing/non-cdylib targets or colliding retained artifact names.
    pub fn artifact_paths(&self) -> Result<&[PathBuf], WasmBuildError> {
        self.wasm_outputs()
            .map(|outputs| outputs.artifact_paths.as_ref())
    }

    fn wasm_outputs(&self) -> Result<&ResolvedWasmOutputs, WasmBuildError> {
        self.wasm_outputs
            .as_ref()
            .map_err(|message| WasmBuildError::InvalidSpec {
                message: message.clone(),
            })
    }

    /// Exact build fingerprint including Cargo inputs, tools, arguments, and environment.
    #[must_use]
    pub const fn fingerprint(&self) -> InputDigest {
        self.fingerprint
    }

    /// Source-independent recipe identity for cohort comparisons, without I/O.
    ///
    /// Covers observed Cargo/Rust compiler identities, target, selected profile,
    /// package/example selection, workspace profiles and relevant Cargo configuration
    /// and explicit build/compiler environment settings. Features are deliberately
    /// excluded: compare or report them separately through [`Self::facts`].
    /// Source bytes, dependency/lock identities, output directories and unrelated
    /// environment values are excluded. No raw environment/configuration values
    /// are exposed by this digest. Configuration locations and all declared profiles
    /// are compared conservatively, so relocation or unused profile changes can
    /// differ. Wrapper settings are covered, not wrapper bytes.
    ///
    /// This is comparison evidence, never cache admission or proof of equivalent
    /// build-script behavior. Callers still compare their lock/tool policy and
    /// own any additional environment or external-input constraints.
    #[must_use]
    pub const fn recipe_identity(&self) -> InputDigest {
        self.recipe_identity
    }

    /// Semantic digest of selected Cargo sources and workspace configuration.
    ///
    /// Unlike [`Self::validation_digest`], this may remain unchanged after an
    /// unrelated host-only workspace manifest or lockfile update.
    #[must_use]
    pub const fn input_digest(&self) -> InputDigest {
        self.input_digest
    }

    /// Conservative digest of every raw source and configuration input.
    ///
    /// This digest is used for mutation guards. It may change while
    /// [`Self::input_digest`] and [`Self::fingerprint`] remain unchanged.
    #[must_use]
    pub const fn validation_digest(&self) -> InputDigest {
        self.validation_digest
    }

    /// Stable logical labels and conservative resolved validation paths.
    #[must_use]
    pub fn inputs(&self) -> &[CargoBuildInput] {
        &self.inputs
    }

    /// Generated-state roots excluded while recursively hashing local inputs.
    ///
    /// These exclusions are derived by IC Build; callers cannot add
    /// arbitrary exclusions through this snapshot.
    /// Nested explicit Cargo workspaces contribute ordinary `target` directories
    /// unless they contain a declared input. Links remain watched, not excluded.
    #[must_use]
    pub fn exclusions(&self) -> &[PathBuf] {
        &self.exclusions
    }

    /// Timings for tool identity, metadata, discovery, and content hashing.
    #[must_use]
    pub const fn timings(&self) -> WasmInputResolutionTimings {
        self.timings
    }

    /// Resolve `spec` again and report whether its exact identity is unchanged.
    ///
    /// # Errors
    /// Returns specification, tool/metadata, filesystem or input-resolution errors.
    pub fn is_current(&self, spec: &WasmBuildSpec) -> Result<bool, WasmBuildError> {
        resolve_cargo_build_inputs(spec).map(|current| current.fingerprint == self.fingerprint)
    }

    /// Resolve again using the caller's probe deadline, cancellation and cleanup.
    ///
    /// Reusing the same policy preserves its absolute deadline across acquisitions.
    /// Filesystem discovery and hashing are outside the probe deadline.
    ///
    /// # Errors
    /// Returns the same errors as [`resolve_cargo_build_inputs_with_probe_policy`].
    pub fn is_current_with_probe_policy(
        &self,
        spec: &WasmBuildSpec,
        policy: &mut CargoInputProbePolicy<'_>,
    ) -> Result<bool, WasmBuildError> {
        resolve_cargo_build_inputs_with_probe_policy(spec, policy)
            .map(|current| current.fingerprint == self.fingerprint)
    }

    /// Rehash the already discovered Cargo source/configuration set.
    ///
    /// This is cheaper than rerunning Cargo metadata and is intended for
    /// before/after guards around external artifact transformations. Resolve a
    /// new snapshot to observe tool, argument, environment, or dependency-graph
    /// identity changes between separate acquisitions.
    ///
    /// # Errors
    /// Returns filesystem errors while rehashing the previously discovered inputs.
    pub fn is_content_current(&self) -> Result<bool, WasmBuildError> {
        self.current_validation_digest()
            .map(|current| current == self.validation_digest)
    }

    pub(super) fn current_validation_digest(&self) -> Result<InputDigest, WasmBuildError> {
        let digest = || {
            let mut exclusions = self.base_exclusions.to_vec();
            source_inputs::exclude_nested_workspace_targets(
                self.inputs.iter().map(|input| input.path.as_path()),
                &self.declared_package_files,
                &mut exclusions,
                &mut source_inputs::WorkspaceDeclarations::new(),
            )?;
            digest_labeled_paths_composable(
                "wasm-source-inputs-v1",
                self.inputs
                    .iter()
                    .map(|input| (input.label.as_path(), input.path.as_path())),
                &exclusions,
                &mut LabeledPathDigestCache::default(),
            )
        };
        digest().map_err(|source| {
            WasmBuildError::io(
                "rehash resolved Cargo build inputs",
                self.inputs
                    .first()
                    .map_or_else(PathBuf::new, |input| input.path.clone()),
                source,
            )
        })
    }
}

fn resolve_tool_identity_with_policy(
    spec: &WasmBuildSpec,
    progress: &mut ProgressReporter<'_>,
    policy: &mut Option<&mut CargoInputProbePolicy<'_>>,
) -> Result<(CargoBuildFacts, Duration), WasmBuildError> {
    let started = Instant::now();
    let facts = spec
        .cargo_invocation()
        .observe(&spec.rustc_program, |tool, command| {
            let phase = match tool {
                CargoTool::Cargo => WasmBuildPhase::CargoIdentity,
                CargoTool::Rustc => WasmBuildPhase::RustcIdentity,
            };
            progress.run_phase(phase, || {
                let output =
                    capture_probe_output_with_policy(command, phase, TOOL_IDENTITY_BYTES, policy)?;
                ensure_command_success(phase, output)
            })
        })?;
    Ok((facts, started.elapsed()))
}

// The shared probe prefix; discovery and hashing retain their single/batch
// ownership, independent failures and timing attribution.
fn observe_cargo_resolution(
    spec: &WasmBuildSpec,
    progress: &mut ProgressReporter<'_>,
    policy: &mut Option<&mut CargoInputProbePolicy<'_>>,
) -> Result<
    (
        CargoBuildFacts,
        schema::Metadata,
        WasmInputResolutionTimings,
    ),
    WasmBuildError,
> {
    let (facts, tool_identity) = resolve_tool_identity_with_policy(spec, progress, policy)?;
    let started = Instant::now();
    let metadata = progress.run_phase(WasmBuildPhase::CargoMetadata, || {
        cargo_metadata_with_policy(spec, policy)
    })?;
    Ok((
        facts,
        metadata,
        WasmInputResolutionTimings {
            tool_identity,
            cargo_metadata: started.elapsed(),
            ..WasmInputResolutionTimings::default()
        },
    ))
}

impl SharedIncrementalTargetMaintenanceConfig {
    /// Schedule one strict retention pass at most once per interval.
    #[must_use]
    pub const fn new(policy: CacheRetentionLimits, minimum_interval: Duration) -> Self {
        Self {
            policy,
            minimum_interval,
            failure_mode: SharedIncrementalTargetMaintenanceFailureMode::Strict,
        }
    }

    /// Select whether an integrated maintenance failure fails the acquisition.
    #[must_use]
    pub const fn with_failure_mode(
        mut self,
        failure_mode: SharedIncrementalTargetMaintenanceFailureMode,
    ) -> Self {
        self.failure_mode = failure_mode;
        self
    }
}

impl SharedIncrementalTargetMaintenance {
    /// Canonical shared Cargo target directory maintained under lock.
    #[must_use]
    pub fn target_dir(&self) -> &Path {
        &self.target_dir
    }

    /// Logical bytes observed before applying the policy.
    #[must_use]
    pub const fn logical_size_bytes_before(&self) -> u64 {
        self.logical_size_bytes_before
    }

    /// Logical bytes retained after applying the policy.
    #[must_use]
    pub const fn logical_size_bytes_after(&self) -> u64 {
        self.logical_size_bytes_after
    }

    /// Most recent build use observed before applying the policy.
    #[must_use]
    pub const fn last_used_before(&self) -> SystemTime {
        self.last_used_before
    }

    /// Whether a configured limit caused the mutable target contents to be cleared.
    #[must_use]
    pub const fn was_cleared(&self) -> bool {
        self.cleared
    }

    /// Time spent waiting for another process using the shared target.
    #[must_use]
    pub const fn lock_wait(&self) -> Duration {
        self.lock_wait
    }

    /// Time spent measuring and, when required, clearing the target.
    #[must_use]
    pub const fn maintenance(&self) -> Duration {
        self.maintenance
    }
}

impl std::fmt::Display for SharedIncrementalTargetMaintenance {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            formatter,
            "target={} action={} bytes={}=>{} lock={:?} maintenance={:?}",
            self.target_dir.display(),
            if self.cleared { "cleared" } else { "retained" },
            self.logical_size_bytes_before,
            self.logical_size_bytes_after,
            self.lock_wait,
            self.maintenance,
        )
    }
}

impl SharedIncrementalTargetMaintenanceOutcome {
    /// Configured or canonical target associated with this result.
    #[must_use]
    pub fn target_dir(&self) -> &Path {
        match self {
            Self::Missing { target_dir }
            | Self::Skipped { target_dir, .. }
            | Self::Failed { target_dir, .. } => target_dir,
            Self::Performed { maintenance, .. } => maintenance.target_dir(),
        }
    }

    /// Completed maintenance report, when retention was evaluated.
    #[must_use]
    pub const fn maintenance(&self) -> Option<&SharedIncrementalTargetMaintenance> {
        match self {
            Self::Performed { maintenance, .. } => Some(maintenance),
            Self::Missing { .. } | Self::Skipped { .. } | Self::Failed { .. } => None,
        }
    }

    /// Whether retention was evaluated during this call.
    #[must_use]
    pub const fn was_performed(&self) -> bool {
        matches!(self, Self::Performed { .. })
    }

    /// Time spent waiting for another process, when the target existed.
    #[must_use]
    pub const fn lock_wait(&self) -> Option<Duration> {
        match self {
            Self::Missing { .. } => None,
            Self::Skipped { lock_wait, .. } | Self::Failed { lock_wait, .. } => Some(*lock_wait),
            Self::Performed { maintenance, .. } => Some(maintenance.lock_wait()),
        }
    }

    /// Time spent checking the schedule marker, when the target existed.
    #[must_use]
    pub const fn schedule_check(&self) -> Option<Duration> {
        match self {
            Self::Missing { .. } | Self::Failed { .. } => None,
            Self::Skipped { schedule_check, .. } | Self::Performed { schedule_check, .. } => {
                Some(*schedule_check)
            }
        }
    }

    /// Rendered integrated maintenance failure, when best-effort handling preserved acquisition.
    #[must_use]
    pub fn failure_message(&self) -> Option<&str> {
        match self {
            Self::Failed { message, .. } => Some(message),
            Self::Missing { .. } | Self::Skipped { .. } | Self::Performed { .. } => None,
        }
    }
}

impl std::fmt::Display for SharedIncrementalTargetMaintenanceOutcome {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Missing { target_dir } => {
                write!(formatter, "target={} action=missing", target_dir.display())
            }
            Self::Skipped {
                target_dir,
                lock_wait,
                schedule_check,
            } => write!(
                formatter,
                "target={} action=skipped lock={lock_wait:?} schedule={schedule_check:?}",
                target_dir.display(),
            ),
            Self::Performed {
                maintenance,
                schedule_check,
            } => write!(formatter, "{maintenance} schedule={schedule_check:?}"),
            Self::Failed {
                target_dir,
                lock_wait,
                message,
            } => write!(
                formatter,
                "target={} action=failed lock={lock_wait:?} error={message}",
                target_dir.display(),
            ),
        }
    }
}

impl std::fmt::Display for WasmBuildTimings {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            formatter,
            "total={:?} lock={:?} shared_lock={:?} inputs={:?} cargo={:?} maintenance={:?}",
            self.total,
            self.lock_wait,
            self.shared_incremental_lock_wait,
            self.input_resolution.total,
            self.cargo_build,
            self.cache_maintenance,
        )
    }
}

impl std::fmt::Display for WasmBuildOutcome {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let state = if self.is_reused() { "reused" } else { "built" };
        write!(
            formatter,
            "{state} fingerprint={} artifacts={} {}",
            self.record().fingerprint,
            self.record().artifacts.len(),
            self.record().timings,
        )?;
        if let Some(maintenance) = self.record().shared_incremental_maintenance() {
            write!(formatter, " shared_maintenance=({maintenance})")?;
        }
        Ok(())
    }
}

/// Resolve the exact Cargo source, configuration, toolchain, argument, and environment identity.
///
/// This performs the same resolution used before and after cached Wasm builds
/// without running `cargo build`.
/// Input-only snapshots may describe ordinary libraries or other Cargo targets;
/// the `cdylib` name/output constraint is checked when acquiring Wasm artifacts.
/// Tool identities require complete output within 64 KiB per stream. Metadata
/// requires complete stdout within 16 MiB and stderr within 64 KiB. Probes have
/// no elapsed-time deadline; oversized output fails before parsing or hashing.
///
/// # Errors
/// Returns invalid specifications, tool/metadata probe failures, filesystem
/// errors or Cargo input discovery/resolution errors.
pub fn resolve_cargo_build_inputs(
    spec: &WasmBuildSpec,
) -> Result<ResolvedCargoBuildInputs, WasmBuildError> {
    validate_spec(spec)?;
    build_fingerprint(spec)
}

/// Resolve Cargo inputs with caller-selected probe execution controls.
///
/// Uses the same commands, environment, complete-output limits, input discovery,
/// target admission and fingerprinting as [`resolve_cargo_build_inputs`]. Only
/// probe execution changes: Host owns each process group and retains original
/// failure, stdout/stderr and any cleanup failure in its [`ToolError`], stored
/// inside [`WasmBuildError::Io`]. Spawn errors retain [`WasmBuildError::CommandSpawn`].
/// Filesystem discovery and hashing are outside the probe deadline.
///
/// # Errors
/// Returns ordinary resolution errors or caller deadline/cancellation failures.
pub fn resolve_cargo_build_inputs_with_probe_policy(
    spec: &WasmBuildSpec,
    policy: &mut CargoInputProbePolicy<'_>,
) -> Result<ResolvedCargoBuildInputs, WasmBuildError> {
    validate_spec(spec)?;
    build_fingerprint_with_policy(spec, &mut ProgressReporter::silent(), &mut Some(policy))
}

/// Apply explicit whole-target retention to caller-owned shared Cargo state.
///
/// Returns `None` without creating anything when the target does not exist.
/// Policy evaluation and any clearing occur under the same cross-process lock
/// used by shared-incremental builds. The target root, `CACHEDIR.TAG`, and
/// `.ic-testkit` lock metadata are preserved, so another process cannot enter
/// through a replacement lock while maintenance is active.
/// Every other target child is removed when a limit is exceeded; unrelated
/// data that must survive must not be colocated there. Exact Cargo input
/// resolution first rejects targets overlapping source or configuration.
///
/// This function is never called automatically by exact Wasm acquisitions.
/// Consumers retain ownership of when mutable incremental state may be lost.
///
/// # Errors
/// Returns missing configuration, unsafe source/target overlap, input-resolution,
/// lock, inspection or target-removal errors.
pub fn maintain_shared_incremental_target(
    spec: &WasmBuildSpec,
    policy: CacheRetentionLimits,
) -> Result<Option<SharedIncrementalTargetMaintenance>, WasmBuildError> {
    let target_dir = configured_shared_incremental_target(spec)?;
    if !shared_incremental_target_exists(
        &target_dir,
        "inspect shared incremental Cargo target before maintenance",
    )? {
        return Ok(None);
    }

    // Reuse the exact build resolver so destructive maintenance cannot act on
    // a target that overlaps Cargo sources, configuration, or additional
    // inputs. The target itself is excluded as generated state during hashing.
    let _ = resolve_cargo_build_inputs(spec)?;
    let (_lock, lock_wait, canonical) = lock_shared_incremental_target(&target_dir)?;
    maintain_shared_incremental_target_locked(&canonical, policy, lock_wait).map(Some)
}

/// Apply whole-target retention at most once per interval across processes.
///
/// The schedule marker is checked under the same lock used by shared Cargo
/// builds. A matching successful pass inside `minimum_interval` returns
/// [`SharedIncrementalTargetMaintenanceOutcome::Skipped`] without resolving
/// Cargo inputs or traversing the target. Missing targets are not created.
/// Changing the policy makes maintenance immediately due, and a zero interval
/// always evaluates retention.
///
/// Due maintenance performs exact Cargo input resolution before inspecting or
/// clearing the target. Failures are returned and are not recorded as a
/// successful pass, so an unsafe configuration cannot be hidden by the
/// schedule.
///
/// # Errors
/// Returns missing configuration, schedule, input-resolution, lock, inspection
/// or target-removal errors. Failed maintenance does not advance the schedule.
pub fn maintain_shared_incremental_target_at_most_every(
    spec: &WasmBuildSpec,
    policy: CacheRetentionLimits,
    minimum_interval: Duration,
) -> Result<SharedIncrementalTargetMaintenanceOutcome, WasmBuildError> {
    let target_dir = configured_shared_incremental_target(spec)?;
    if !shared_incremental_target_exists(
        &target_dir,
        "inspect shared incremental Cargo target before scheduled maintenance",
    )? {
        return Ok(SharedIncrementalTargetMaintenanceOutcome::Missing { target_dir });
    }

    let (_lock, lock_wait, canonical) = lock_shared_incremental_target(&target_dir)?;
    let schedule = schedule_shared_incremental_target_maintenance(
        &canonical,
        policy,
        minimum_interval,
        lock_wait,
    )?;
    let schedule = match schedule {
        SharedIncrementalTargetMaintenanceSchedule::Skipped(outcome) => return Ok(outcome),
        SharedIncrementalTargetMaintenanceSchedule::Due(due) => due,
    };

    // Keep the schedule decision and maintenance in one critical section so
    // concurrent test binaries cannot all perform the same expensive scan.
    let _ = resolve_cargo_build_inputs(spec)?;
    perform_due_shared_incremental_target_maintenance(&canonical, policy, lock_wait, schedule)
}

/// Build or reuse one exact set of Cargo Wasm artifacts.
///
/// The operation takes an exclusive process lock scoped to `target_dir`, then
/// fingerprints all declared inputs. A cache hit requires both a matching
/// atomic stamp and every expected nonempty Wasm output. Ordinary warm hits
/// revalidate inputs before returning and reject mutations during acquisition.
/// Failed or interrupted builds never publish a successful stamp.
///
/// `spawn` selects direct-child or owned-group custody and its cleanup policy,
/// independently of progress. Cargo starts with null stdin and piped stdout/
/// stderr; the spawner may override stdin. It must execute the supplied command
/// without changing its recipe. No signal handler or elapsed-time limit is added.
/// Probes retain their separate existing execution policy.
///
/// Use [`build_wasm_canisters_cached_with_progress`] for phase-aware heartbeats.
///
/// A verified exact entry owns the bytes for its fingerprint. Warm acquisitions
/// repair public outputs and stamps to match it; matching independent writable
/// files owned by the effective user stay in place on Unix. If the exact entry
/// is missing or invalid, acquisition runs Cargo again instead of importing
/// mutable public outputs. A retained invalid entry cannot be replaced until
/// its last acquisition record releases it. Cold publication and non-Unix
/// materialization use atomic replacement. Callers must coordinate
/// other writers to mutable public destinations and treat retained paths as read-only.
///
/// # Errors
/// Returns invalid specification, input-resolution/change, coordination-lock,
/// Cargo execution, artifact verification or publication errors.
pub fn build_wasm_canisters_cached(
    spec: &WasmBuildSpec,
    mut spawn: impl FnMut(&mut Command) -> io::Result<OwnedChild>,
) -> Result<WasmBuildOutcome, WasmBuildError> {
    build_wasm_canisters_cached_internal(
        spec,
        None,
        &mut ProgressReporter::silent(),
        None,
        &mut spawn,
    )
}

/// Build or reuse exact Wasm artifacts with caller-selected progress observation.
///
/// Execution, source checks and retention match [`build_wasm_canisters_cached`].
/// Observation never selects process custody or imposes a build deadline.
/// Observer panics clean up the active child with the caller's selected spawner.
///
/// # Errors
/// Returns invalid progress/specification, execution, verification or publication errors.
pub fn build_wasm_canisters_cached_with_progress<F>(
    spec: &WasmBuildSpec,
    config: WasmBuildProgressConfig,
    mut observer: F,
    mut spawn: impl FnMut(&mut Command) -> io::Result<OwnedChild>,
) -> Result<WasmBuildOutcome, WasmBuildError>
where
    F: FnMut(WasmBuildProgressEvent),
{
    build_wasm_canisters_cached_internal(
        spec,
        None,
        &mut ProgressReporter::new(Some((config, &mut observer)))?,
        None,
        &mut spawn,
    )
}

#[expect(
    clippy::result_large_err,
    reason = "Batch entries retain complete failure details inline without per-failure allocation."
)]
pub(super) fn build_wasm_canisters_cached_in_batch(
    spec: &WasmBuildSpec,
    maintenance: Option<SharedIncrementalTargetMaintenanceConfig>,
    index: usize,
    resolver: &mut WasmBuildBatchInputResolver<'_>,
    observation: Option<(
        WasmBuildProgressConfig,
        &mut dyn FnMut(WasmBuildProgressEvent),
    )>,
    mut spawn: &mut dyn FnMut(&mut Command) -> io::Result<OwnedChild>,
) -> Result<WasmBuildOutcome, (WasmBuildError, WasmBuildFailureDetails)> {
    let mut progress = ProgressReporter::new(observation).map_err(|error| {
        (
            error,
            WasmBuildFailureDetails::specification(Duration::ZERO),
        )
    })?;
    let started = Instant::now();
    let result = build_wasm_canisters_cached_internal(
        spec,
        maintenance,
        &mut progress,
        Some((resolver, index)),
        &mut spawn,
    );
    let total = started.elapsed();
    result.map_err(|error| (error, progress.failure_details(total)))
}

#[derive(Clone, Debug)]
enum CachePlan {
    Isolated,
    Shared {
        target_dir: PathBuf,
        maintenance: Option<SharedIncrementalTargetMaintenanceConfig>,
    },
}

#[derive(Clone, Copy)]
enum SharedAcquisition<'a> {
    Scheduled {
        target_dir: &'a Path,
        config: SharedIncrementalTargetMaintenanceConfig,
    },
    Miss {
        target_dir: &'a Path,
        resolved: &'a ResolvedCargoBuildInputs,
        first_lock_wait: Duration,
    },
}

fn build_wasm_canisters_cached_internal(
    spec: &WasmBuildSpec,
    batch_maintenance: Option<SharedIncrementalTargetMaintenanceConfig>,
    progress: &mut ProgressReporter<'_>,
    mut batch_resolution: Option<(&mut WasmBuildBatchInputResolver<'_>, usize)>,
    mut spawn: &mut dyn FnMut(&mut Command) -> io::Result<OwnedChild>,
) -> Result<WasmBuildOutcome, WasmBuildError> {
    let total_started = Instant::now();
    // The batch tracker supplies maintenance only for the first specification
    // of a distinct shared target, after refusing per-spec ownership conflicts.
    // Keep ordinary spec admission and its failure order before this projection.
    let mut plan = validate_spec(spec)?;
    if let (CachePlan::Shared { maintenance, .. }, Some(config)) = (&mut plan, batch_maintenance) {
        *maintenance = Some(config);
    }
    if let CachePlan::Shared {
        target_dir,
        maintenance: Some(config),
    } = &plan
    {
        return build_wasm_with_shared_incremental(
            spec,
            SharedAcquisition::Scheduled {
                target_dir,
                config: *config,
            },
            total_started,
            progress,
            batch_resolution.take(),
            &mut spawn,
        );
    }
    let (cache_lock, first_lock_wait) =
        lock_wasm_build_cache_with_progress(&spec.target_dir, progress)?;
    ensure_cache_directory_tag(&spec.target_dir)?;

    let resolved = resolve_initial_inputs(spec, batch_resolution.take(), progress)?;
    let cache_entry = cache_entry_directory(spec, resolved.fingerprint);
    let isolated_acquisition = WasmAcquisitionContext::isolated(&cache_entry);
    if let Some(outcome) = try_reuse_wasm_artifacts(
        spec,
        &resolved,
        first_lock_wait,
        &isolated_acquisition,
        total_started,
        progress,
    )? {
        return Ok(outcome);
    }
    match plan {
        CachePlan::Shared { target_dir, .. } => {
            drop(cache_lock);
            build_wasm_with_shared_incremental(
                spec,
                SharedAcquisition::Miss {
                    target_dir: &target_dir,
                    resolved: &resolved,
                    first_lock_wait,
                },
                total_started,
                progress,
                None,
                &mut spawn,
            )
        }
        CachePlan::Isolated => build_wasm_cache_miss(
            spec,
            &resolved,
            first_lock_wait,
            &isolated_acquisition,
            total_started,
            progress,
            &mut spawn,
        ),
    }
}

fn build_wasm_with_shared_incremental(
    spec: &WasmBuildSpec,
    acquisition: SharedAcquisition<'_>,
    total_started: Instant,
    progress: &mut ProgressReporter<'_>,
    batch_resolution: Option<(&mut WasmBuildBatchInputResolver<'_>, usize)>,
    mut spawn: &mut dyn FnMut(&mut Command) -> io::Result<OwnedChild>,
) -> Result<WasmBuildOutcome, WasmBuildError> {
    let target_dir = match acquisition {
        SharedAcquisition::Scheduled { target_dir, .. }
        | SharedAcquisition::Miss { target_dir, .. } => target_dir,
    };
    let (shared_lock, shared_lock_wait, shared_target) =
        lock_shared_incremental_target_with_progress(target_dir, progress)?;
    let (_cache_lock, second_lock_wait) =
        lock_wasm_build_cache_with_progress(&spec.target_dir, progress)?;
    ensure_cache_directory_tag(&spec.target_dir)?;

    // Scheduled maintenance resolves under both locks. An ordinary miss must
    // resolve again after changing lock order. Retain both observations' timings.
    let mut refreshed;
    let (current, first_lock_wait, maintenance_config) = match acquisition {
        SharedAcquisition::Miss {
            resolved,
            first_lock_wait,
            ..
        } => {
            refreshed = resolve_initial_inputs(spec, None, progress)?;
            refreshed.timings.include(resolved.timings);
            (&refreshed, first_lock_wait, None)
        }
        SharedAcquisition::Scheduled { config, .. } => {
            refreshed = resolve_initial_inputs(spec, batch_resolution, progress)?;
            (&refreshed, Duration::ZERO, Some(config))
        }
    };
    let lock_wait = first_lock_wait.saturating_add(second_lock_wait);
    let maintenance = maintenance_config
        .map(|config| {
            perform_configured_shared_incremental_target_maintenance(
                config,
                &shared_target,
                shared_lock_wait,
                progress,
            )
        })
        .transpose()?;
    let shared_incremental =
        WasmAcquisitionContext::shared(&shared_target, shared_lock_wait, maintenance);
    if let Some(outcome) = try_reuse_wasm_artifacts(
        spec,
        current,
        lock_wait,
        &shared_incremental,
        total_started,
        progress,
    )? {
        return Ok(outcome);
    }

    let outcome = build_wasm_cache_miss(
        spec,
        current,
        lock_wait,
        &shared_incremental,
        total_started,
        progress,
        &mut spawn,
    );
    drop(shared_lock);
    outcome
}

fn resolve_initial_inputs(
    spec: &WasmBuildSpec,
    batch_resolution: Option<(&mut WasmBuildBatchInputResolver<'_>, usize)>,
    progress: &mut ProgressReporter<'_>,
) -> Result<ResolvedCargoBuildInputs, WasmBuildError> {
    let resolved = if let Some((resolver, index)) = batch_resolution {
        resolver.resolve(index, progress)?
    } else {
        build_fingerprint_with_progress(spec, progress)?
    };
    resolved.wasm_outputs().inspect_err(|_| {
        progress.begin_phase(WasmBuildPhase::InputDiscovery);
    })?;
    Ok(resolved)
}

#[derive(Debug)]
struct WasmAcquisitionContext<'a> {
    cargo_target: AcquiredCargoTarget<'a>,
}

impl<'a> WasmAcquisitionContext<'a> {
    const fn isolated(target: &'a Path) -> Self {
        Self {
            cargo_target: AcquiredCargoTarget::Exact(target),
        }
    }

    const fn shared(
        target: &'a Path,
        lock_wait: Duration,
        maintenance: Option<SharedIncrementalTargetMaintenanceOutcome>,
    ) -> Self {
        Self {
            cargo_target: AcquiredCargoTarget::Shared {
                target,
                lock_wait,
                maintenance,
            },
        }
    }

    const fn lock_wait(&self) -> Option<Duration> {
        match &self.cargo_target {
            AcquiredCargoTarget::Exact(_) => None,
            AcquiredCargoTarget::Shared { lock_wait, .. } => Some(*lock_wait),
        }
    }

    fn maintenance(&self) -> Option<SharedIncrementalTargetMaintenanceOutcome> {
        match &self.cargo_target {
            AcquiredCargoTarget::Exact(_) => None,
            AcquiredCargoTarget::Shared { maintenance, .. } => maintenance.clone(),
        }
    }
}

fn try_reuse_wasm_artifacts(
    spec: &WasmBuildSpec,
    resolved: &ResolvedCargoBuildInputs,
    lock_wait: Duration,
    shared_incremental: &WasmAcquisitionContext<'_>,
    total_started: Instant,
    progress: &mut ProgressReporter<'_>,
) -> Result<Option<WasmBuildOutcome>, WasmBuildError> {
    let targets = &resolved.wasm_outputs()?.targets;
    let fingerprint = resolved.fingerprint;
    let cache_entry = cache_entry_directory(spec, fingerprint);
    let paths = artifact_publication_paths(spec, &cache_entry, targets);
    let Some(artifact_info) = progress.run_phase(WasmBuildPhase::ArtifactPublication, || {
        validated_artifact_set(paths, fingerprint)
    }) else {
        return Ok(None);
    };
    progress.run_phase(WasmBuildPhase::ArtifactPublication, || {
        materialize_artifacts(&artifact_info, fingerprint, true)?;
        record_cache_entry_use(&cache_entry)
    })?;
    let input_resolution = validate_reused_inputs(spec, resolved, progress)?;
    Ok(Some(WasmBuildOutcome::Reused(complete_build_record(
        spec,
        BuildRecordInput {
            resolved,
            lock_wait,
            shared_incremental_lock_wait: shared_incremental.lock_wait(),
            shared_incremental_maintenance: shared_incremental.maintenance(),
            input_resolution,
            cargo_build: None,
            active_entry: &cache_entry,
        },
        total_started,
        progress,
    )?)))
}

fn validate_reused_inputs(
    spec: &WasmBuildSpec,
    resolved: &ResolvedCargoBuildInputs,
    progress: &mut ProgressReporter<'_>,
) -> Result<WasmInputResolutionTimings, WasmBuildError> {
    let mut timings = resolved.timings;
    let verified = verify_resolved_inputs(spec, resolved, progress)?;
    timings.include(verified.timings);
    Ok(timings)
}

fn verify_resolved_inputs(
    spec: &WasmBuildSpec,
    resolved: &ResolvedCargoBuildInputs,
    progress: &mut ProgressReporter<'_>,
) -> Result<ResolvedCargoBuildInputs, WasmBuildError> {
    let verified = resolve_initial_inputs(spec, None, progress)?;
    if let Some((before, after)) = resolved.identity().difference(verified.identity()) {
        return Err(WasmBuildError::InputsChangedDuringAcquisition { before, after });
    }
    Ok(verified)
}

// Only the shared acquisition path supplies a shared target, after taking its
// coordination lock. Publication need not reinterpret the original specification.
#[derive(Debug)]
enum AcquiredCargoTarget<'a> {
    Exact(&'a Path),
    Shared {
        target: &'a Path,
        lock_wait: Duration,
        maintenance: Option<SharedIncrementalTargetMaintenanceOutcome>,
    },
}

fn build_wasm_cache_miss(
    spec: &WasmBuildSpec,
    resolved: &ResolvedCargoBuildInputs,
    lock_wait: Duration,
    shared_incremental: &WasmAcquisitionContext<'_>,
    total_started: Instant,
    progress: &mut ProgressReporter<'_>,
    mut spawn: &mut dyn FnMut(&mut Command) -> io::Result<OwnedChild>,
) -> Result<WasmBuildOutcome, WasmBuildError> {
    let targets = &resolved.wasm_outputs()?.targets;
    let fingerprint = resolved.fingerprint;
    let mut input_resolution = resolved.timings;
    let cache_entry = cache_entry_directory(spec, fingerprint);
    let incomplete_directory = progress.run_phase(WasmBuildPhase::ArtifactPublication, || {
        remove_unretained_entry(&cache_entry).map_err(wasm_cache_fs_error)?;
        create_dir_all(
            &cache_entry,
            "create content-addressed Cargo target directory",
        )?;
        Ok::<_, WasmBuildError>(OwnedDirectory::new(cache_entry.clone()))
    })?;
    let build_result = (|| {
        let cargo_target_dir = match &shared_incremental.cargo_target {
            AcquiredCargoTarget::Exact(path) => path,
            AcquiredCargoTarget::Shared { target, .. } => {
                record_cache_entry_use(target)?;
                target
            }
        };
        let build_started = Instant::now();
        progress.begin_phase(WasmBuildPhase::CargoBuild);
        let cargo_result = run_cargo_build(spec, cargo_target_dir, targets, progress, &mut spawn);
        let cargo_build = build_started.elapsed();
        progress.record_phase(WasmBuildPhase::CargoBuild, cargo_build);
        let built_artifacts = cargo_result?;
        let missing = progress.run_phase(WasmBuildPhase::ArtifactPublication, || {
            missing_artifacts(&built_artifacts)
        });
        if !missing.is_empty() {
            return Err(WasmBuildError::MissingArtifacts { paths: missing });
        }

        let verified = verify_resolved_inputs(spec, resolved, progress)?;
        input_resolution.include(verified.timings);

        let paths = artifact_publication_paths(spec, &cache_entry, targets);
        progress.run_phase(WasmBuildPhase::ArtifactPublication, || {
            copy_wasm_artifacts(&built_artifacts, &paths)?;
            let artifacts = publish_artifact_stamps(paths, fingerprint)?;
            materialize_artifacts(&artifacts, fingerprint, false)?;
            record_cache_entry_use(&cache_entry)
        })?;

        Ok(WasmBuildOutcome::Built(complete_build_record(
            spec,
            BuildRecordInput {
                resolved,
                lock_wait,
                shared_incremental_lock_wait: shared_incremental.lock_wait(),
                shared_incremental_maintenance: shared_incremental.maintenance(),
                input_resolution,
                cargo_build: Some(cargo_build),
                active_entry: &cache_entry,
            },
            total_started,
            progress,
        )?))
    })();
    finish_fingerprint_build(build_result, incomplete_directory, progress)
}

/// Prune fingerprint-specific Cargo target directories under `target_dir`.
///
/// Pruning uses the same exclusive process lock as builds. Entries older than
/// the configured age are removed first, then least-recently-used entries are
/// removed until the configured logical byte limit is met. Only direct child
/// directories with SHA-256 fingerprint names are eligible; caller-facing
/// artifacts and unrelated target contents are never removed.
/// Entries owned by live build records are skipped until their last owner drops.
///
/// # Errors
/// Returns cache-lock, inspection or filesystem removal errors.
pub fn prune_wasm_build_cache(
    target_dir: &Path,
    policy: CacheRetentionLimits,
) -> Result<ArtifactCachePruneReport, WasmBuildError> {
    let (_lock_file, _) = lock_wasm_build_cache(target_dir)?;
    ensure_cache_directory_tag(target_dir)?;

    prune_wasm_build_cache_locked(target_dir, policy, None)
}

struct BuildRecordInput<'a> {
    resolved: &'a ResolvedCargoBuildInputs,
    lock_wait: Duration,
    shared_incremental_lock_wait: Option<Duration>,
    shared_incremental_maintenance: Option<SharedIncrementalTargetMaintenanceOutcome>,
    input_resolution: WasmInputResolutionTimings,
    cargo_build: Option<Duration>,
    active_entry: &'a Path,
}

fn complete_build_record(
    spec: &WasmBuildSpec,
    input: BuildRecordInput<'_>,
    total_started: Instant,
    progress: &mut ProgressReporter<'_>,
) -> Result<WasmBuildRecord, WasmBuildError> {
    let retention = progress.run_phase(WasmBuildPhase::ArtifactPublication, || {
        RetainedCacheEntry::acquire(input.active_entry).map_err(wasm_cache_fs_error)
    })?;
    let (maintenance, cache_maintenance) =
        spec.pruning.map_or((None, None), |(policy, interval)| {
            let (outcome, elapsed) =
                progress.run_phase(WasmBuildPhase::ExactCacheMaintenance, || {
                    let cache_root = spec.target_dir.join(".ic-testkit/wasm-targets");
                    let identity = policy.maintenance_identity();
                    perform_scheduled_cache_maintenance(&cache_root, interval, &identity, || {
                        prune_wasm_build_cache_locked(
                            &spec.target_dir,
                            policy,
                            Some(input.active_entry),
                        )
                        .map_err(|error| error.to_string())
                    })
                });
            (outcome, Some(elapsed))
        });
    Ok(WasmBuildRecord {
        facts: input.resolved.facts.clone(),
        fingerprint: input.resolved.fingerprint,
        recipe_identity: input.resolved.recipe_identity,
        input_digest: input.resolved.input_digest,
        artifacts: expected_artifacts(
            spec,
            input.active_entry,
            &input.resolved.wasm_outputs()?.targets,
        )
        .into(),
        retention,
        timings: WasmBuildTimings {
            lock_wait: input.lock_wait,
            shared_incremental_lock_wait: input.shared_incremental_lock_wait,
            input_resolution: input.input_resolution,
            cargo_build: input.cargo_build,
            cache_maintenance,
            total: total_started.elapsed(),
        },
        maintenance,
        shared_incremental_maintenance: input.shared_incremental_maintenance,
    })
}

fn prune_wasm_build_cache_locked(
    target_dir: &Path,
    policy: CacheRetentionLimits,
    protected_entry: Option<&Path>,
) -> Result<ArtifactCachePruneReport, WasmBuildError> {
    let cache_root = target_dir.join(".ic-testkit/wasm-targets");
    prune_direct_child_directories(&cache_root, policy, protected_entry, is_sha256_directory)
        .map_err(wasm_cache_fs_error)
}

fn finish_fingerprint_build<T>(
    result: Result<T, WasmBuildError>,
    incomplete_directory: OwnedDirectory,
    progress: &mut ProgressReporter<'_>,
) -> Result<T, WasmBuildError> {
    let failed = result.is_err();
    let primary_phase = progress.failure_details(Duration::ZERO).phase();
    let cleanup_started = Instant::now();
    let mut cleanup_failed = false;
    let result = finish_cache_build(result, incomplete_directory, |build_error, path, source| {
        cleanup_failed = true;
        WasmBuildError::FailedBuildCleanup {
            build_error: Box::new(build_error),
            path,
            source,
        }
    });
    if failed {
        progress.record_phase(WasmBuildPhase::Cleanup, cleanup_started.elapsed());
        if !cleanup_failed {
            progress.begin_phase(primary_phase);
        }
    }
    result
}

fn lock_wasm_build_cache(target_dir: &Path) -> Result<(File, Duration), WasmBuildError> {
    lock_wasm_build_cache_with_progress(target_dir, &mut ProgressReporter::silent())
}

fn lock_wasm_build_cache_with_progress(
    target_dir: &Path,
    progress: &mut ProgressReporter<'_>,
) -> Result<(File, Duration), WasmBuildError> {
    progress.begin_phase(WasmBuildPhase::ExactCacheCoordination);
    create_dir_all(target_dir, "create Cargo target directory")?;
    let lock_path = target_dir.join(".ic-testkit/wasm-build.lock");
    lock_cache_file_with_progress(&lock_path, WasmBuildPhase::ExactCacheCoordination, progress)
}

fn lock_shared_incremental_target(
    target_dir: &Path,
) -> Result<(File, Duration, PathBuf), WasmBuildError> {
    lock_shared_incremental_target_internal(target_dir, &mut ProgressReporter::silent())
}

fn lock_shared_incremental_target_with_progress(
    target_dir: &Path,
    progress: &mut ProgressReporter<'_>,
) -> Result<(File, Duration, PathBuf), WasmBuildError> {
    let (lock, wait, canonical) = lock_shared_incremental_target_internal(target_dir, progress)?;
    Ok((lock, wait, canonical))
}

fn lock_shared_incremental_target_internal(
    target_dir: &Path,
    progress: &mut ProgressReporter<'_>,
) -> Result<(File, Duration, PathBuf), WasmBuildError> {
    progress.begin_phase(WasmBuildPhase::SharedTargetCoordination);
    create_dir_all(
        target_dir,
        "create shared incremental Cargo target directory",
    )?;
    ensure_cache_tag(target_dir).map_err(wasm_cache_fs_error)?;
    let canonical = target_dir.canonicalize().map_err(|source| {
        WasmBuildError::io(
            "resolve shared incremental Cargo target directory",
            target_dir.to_owned(),
            source,
        )
    })?;
    let lock_path = canonical.join(".ic-testkit/wasm-incremental.lock");
    let (lock, wait) = lock_cache_file_with_progress(
        &lock_path,
        WasmBuildPhase::SharedTargetCoordination,
        progress,
    )?;
    Ok((lock, wait, canonical))
}

fn lock_cache_file_with_progress(
    lock_path: &Path,
    phase: WasmBuildPhase,
    progress: &mut ProgressReporter<'_>,
) -> Result<(File, Duration), WasmBuildError> {
    let started = Instant::now();
    progress.begin_phase(phase);
    let result = progress.heartbeat_interval().map_or_else(
        || lock_cache_file(lock_path).map_err(wasm_cache_fs_error),
        |interval| {
            lock_cache_file_with_wait_observer(lock_path, interval, |elapsed| {
                progress.emit_heartbeat_if_due(phase, elapsed);
            })
            .map_err(wasm_cache_fs_error)
        },
    );
    progress.record_phase(phase, started.elapsed());
    result
}

fn ensure_cache_directory_tag(target_dir: &Path) -> Result<(), WasmBuildError> {
    ensure_cache_tag(target_dir).map_err(wasm_cache_fs_error)
}

fn record_cache_entry_use(path: &Path) -> Result<(), WasmBuildError> {
    record_entry_use(path).map_err(wasm_cache_fs_error)
}

fn wasm_cache_fs_error(error: CacheFsError) -> WasmBuildError {
    WasmBuildError::io(error.operation, error.path, error.source)
}

fn validate_spec(spec: &WasmBuildSpec) -> Result<CachePlan, WasmBuildError> {
    for (key, value) in &spec.environment {
        let key = key.as_encoded_bytes();
        if key.is_empty()
            || key.iter().any(|byte| matches!(byte, b'=' | 0))
            || value.as_encoded_bytes().contains(&0)
        {
            return Err(WasmBuildError::InvalidSpec {
                message: "invalid child environment name or value".to_owned(),
            });
        }
    }
    if spec.cargo_message_bytes == 0 {
        return Err(WasmBuildError::InvalidSpec {
            message: "Cargo message allowance must be nonzero".to_owned(),
        });
    }
    if spec.packages.is_empty() {
        return Err(WasmBuildError::InvalidSpec {
            message: "at least one Cargo package is required".to_owned(),
        });
    }
    if spec.target.is_empty() {
        return Err(WasmBuildError::InvalidSpec {
            message: "Cargo compilation target must not be empty".to_owned(),
        });
    }
    if spec
        .environment
        .contains_key(OsStr::new("CARGO_TARGET_DIR"))
    {
        return Err(WasmBuildError::InvalidSpec {
            message: "Cargo target directories are owned by the build specification; use target_dir or with_shared_incremental_target instead of command overrides".to_owned(),
        });
    }
    validate_cargo_target_selection(spec)?;
    spec.cargo_selection
        .profile_target_dir()
        .map_err(|message| WasmBuildError::InvalidSpec {
            message: message.to_owned(),
        })?;
    if matches!(
        &spec.cache_mode,
        WasmBuildCacheMode::SharedIncremental { target_dir } if target_dir.as_os_str().is_empty()
    ) {
        return Err(WasmBuildError::InvalidSpec {
            message: "shared incremental Cargo target directory must not be empty".to_owned(),
        });
    }
    match (
        shared_incremental_target(spec),
        spec.shared_incremental_maintenance_config,
    ) {
        (None, None) => Ok(CachePlan::Isolated),
        (Some(target_dir), maintenance) => Ok(CachePlan::Shared {
            target_dir,
            maintenance,
        }),
        (None, Some(_)) => Err(WasmBuildError::InvalidSpec {
            message:
                "scheduled shared-target maintenance requires a shared incremental Cargo target"
                    .to_owned(),
        }),
    }
}

fn validate_cargo_target_selection(spec: &WasmBuildSpec) -> Result<(), WasmBuildError> {
    if let Some(example) = &spec.example
        && (spec.packages.len() != 1 || !is_normal_component(Path::new(example)))
    {
        return Err(WasmBuildError::InvalidSpec {
            message: "a named Cargo example requires one package, a normal target name".to_owned(),
        });
    }
    if spec
        .packages
        .iter()
        .any(|package| !is_normal_component(Path::new(package)))
    {
        return Err(WasmBuildError::InvalidSpec {
            message: "Cargo package names must be normal path components".to_owned(),
        });
    }
    Ok(())
}

fn build_fingerprint(spec: &WasmBuildSpec) -> Result<ResolvedCargoBuildInputs, WasmBuildError> {
    build_fingerprint_with_progress(spec, &mut ProgressReporter::silent())
}

fn build_fingerprint_with_progress(
    spec: &WasmBuildSpec,
    progress: &mut ProgressReporter<'_>,
) -> Result<ResolvedCargoBuildInputs, WasmBuildError> {
    build_fingerprint_with_policy(spec, progress, &mut None)
}

fn build_fingerprint_with_policy(
    spec: &WasmBuildSpec,
    progress: &mut ProgressReporter<'_>,
    policy: &mut Option<&mut CargoInputProbePolicy<'_>>,
) -> Result<ResolvedCargoBuildInputs, WasmBuildError> {
    let total_started = Instant::now();
    let (facts, metadata, mut timings) = observe_cargo_resolution(spec, progress, policy)?;

    let discovery_started = Instant::now();
    let discovered = progress.run_phase(WasmBuildPhase::InputDiscovery, || {
        let parsed =
            ParsedCargoMetadata::parse(&metadata).map_err(|message| invalid_metadata(&message))?;
        DiscoveredBuildInputs::resolve(spec, &parsed)
    })?;
    let input_discovery = discovery_started.elapsed();

    let hashing_started = Instant::now();
    let hashed = progress.run_phase(WasmBuildPhase::ContentHashing, || {
        let mut cache = LabeledPathDigestCache::default();
        discovered.hash(
            &mut cache,
            &mut source_inputs::WorkspaceDeclarations::new(),
            &spec.workspace_root,
            ("hash Wasm build inputs", "hash semantic Wasm build inputs"),
        )
    })?;
    let content_hashing = hashing_started.elapsed();

    timings.input_discovery = input_discovery;
    timings.content_hashing = content_hashing;

    Ok(ResolvedCargoBuildInputs::assemble(
        spec,
        hashed,
        facts,
        || {
            timings.total = total_started.elapsed();
            timings
        },
    ))
}

fn cargo_build_inputs(inputs: Vec<(PathBuf, PathBuf)>) -> Arc<[CargoBuildInput]> {
    inputs
        .into_iter()
        .map(|(label, path)| CargoBuildInput { label, path })
        .collect()
}

fn finish_build_fingerprint(
    spec: &WasmBuildSpec,
    facts: &CargoBuildFacts,
    input_digest: InputDigest,
) -> InputDigest {
    let mut hasher = InputHasher::new(BUILD_FINGERPRINT_DOMAIN);
    for package in &spec.packages {
        hasher.field("package", package.as_bytes());
    }
    hasher.field("target", facts.target.as_bytes());
    if let Some(example) = &spec.example {
        hasher.field("cargo-target-selection", b"example");
        hasher.field("cargo-example", example.as_bytes());
    } else {
        hasher.field("cargo-target-selection", b"lib");
    }
    hasher.field(
        "profile-target-dir",
        facts
            .selection
            .profile_target_dir()
            .expect("Cargo profile was validated before fingerprinting")
            .as_bytes(),
    );
    for argument in &facts.selection.build_arguments() {
        hasher.field("cargo-argument", &os_bytes(argument));
    }
    for (key, value) in &facts.environment {
        hasher.field("environment-key", &os_bytes(key));
        hasher.field("environment-value", &os_bytes(value));
    }
    hasher.field("cargo-identity", &facts.cargo_identity);
    hasher.field("rustc-identity", &facts.rustc_identity);
    hasher.field("source-input-digest", input_digest.as_bytes());
    hasher.finish()
}

fn cargo_metadata_with_policy(
    spec: &WasmBuildSpec,
    policy: &mut Option<&mut CargoInputProbePolicy<'_>>,
) -> Result<schema::Metadata, WasmBuildError> {
    let mut command = Command::new(&spec.cargo_program);
    command
        .current_dir(&spec.workspace_root)
        .args(metadata_arguments(true, None));
    for argument in spec.cargo_selection.metadata_arguments() {
        command.arg(argument);
    }
    apply_command_environment(&mut command, spec);
    let output = capture_probe_output_with_policy(
        &mut command,
        WasmBuildPhase::CargoMetadata,
        CARGO_METADATA_BYTES,
        policy,
    )?;
    let output = ensure_command_success(WasmBuildPhase::CargoMetadata, output)?;
    crate::cargo::observation::decode_metadata(&output.stdout).map_err(|error| {
        WasmBuildError::InvalidMetadata {
            message: format!("Cargo metadata did not match its schema: {error}"),
        }
    })
}

fn capture_probe_output_with_policy(
    command: &mut Command,
    phase: WasmBuildPhase,
    stdout_bytes: usize,
    policy: &mut Option<&mut CargoInputProbePolicy<'_>>,
) -> Result<CompletedExecution, WasmBuildError> {
    let limits = OutputLimits {
        stdout: OutputLimit::Terminate(stdout_bytes),
        stderr: OutputLimit::Terminate(TOOL_IDENTITY_BYTES),
        timeout: None,
    };
    let result = if let Some(policy) = policy {
        capture_group_command_with_cancellation(
            command,
            limits,
            policy.cleanup,
            policy.deadline,
            &mut *policy.cancelled,
        )
    } else {
        capture_command(command, limits)
    }
    .and_then(CompletedExecution::require_complete);
    let evidence = match result {
        Ok(evidence) => evidence,
        Err(ToolError::Execution(error))
            if matches!(
                error.failure,
                ExecutionFailure::Io {
                    operation: ExecutionOperation::Spawn,
                    ..
                }
            ) && error.cleanup.is_none() =>
        {
            let ExecutionFailure::Io { source, .. } = error.failure else {
                unreachable!()
            };
            return Err(WasmBuildError::CommandSpawn {
                phase,
                program: command.get_program().to_owned(),
                source,
            });
        }
        Err(error) => {
            return Err(WasmBuildError::io(
                match phase {
                    WasmBuildPhase::CargoIdentity => "capture complete Cargo identity probe output",
                    WasmBuildPhase::RustcIdentity => "capture complete rustc identity probe output",
                    _ => "capture complete Cargo metadata probe output",
                },
                PathBuf::from(command.get_program()),
                io::Error::other(error),
            ));
        }
    };
    Ok(evidence)
}

pub(super) fn shared_incremental_target(spec: &WasmBuildSpec) -> Option<PathBuf> {
    let WasmBuildCacheMode::SharedIncremental { target_dir } = &spec.cache_mode else {
        return None;
    };
    Some(if target_dir.is_absolute() {
        target_dir.clone()
    } else {
        spec.workspace_root.join(target_dir)
    })
}

fn configured_shared_incremental_target(spec: &WasmBuildSpec) -> Result<PathBuf, WasmBuildError> {
    shared_incremental_target(spec).ok_or_else(|| WasmBuildError::InvalidSpec {
        message: "shared incremental target is not configured".to_owned(),
    })
}

fn shared_incremental_target_exists(
    target_dir: &Path,
    operation: &'static str,
) -> Result<bool, WasmBuildError> {
    match fs::symlink_metadata(target_dir) {
        Ok(metadata) if metadata.is_dir() => Ok(true),
        Ok(_) => Err(WasmBuildError::InvalidSpec {
            message: format!(
                "shared incremental Cargo target {} must be a directory",
                target_dir.display()
            ),
        }),
        Err(error) if error.kind() == io::ErrorKind::NotFound => Ok(false),
        Err(source) => Err(WasmBuildError::io(operation, target_dir, source)),
    }
}

fn apply_command_environment(command: &mut Command, spec: &WasmBuildSpec) {
    command.env_clear().envs(&spec.environment);
}

fn run_cargo_build(
    spec: &WasmBuildSpec,
    build_target_dir: &Path,
    targets: &[WasmArtifactTarget],
    progress: &mut ProgressReporter<'_>,
    mut spawn: &mut dyn FnMut(&mut Command) -> io::Result<OwnedChild>,
) -> Result<Vec<PathBuf>, WasmBuildError> {
    let command = cargo_build_command(spec, build_target_dir)?;
    communicate_cargo_build(
        spec,
        build_target_dir,
        targets,
        command,
        progress,
        &mut spawn,
    )
}

impl WasmBuildSpec {
    fn cargo_invocation(&self) -> CargoInvocation {
        CargoInvocation {
            cargo_program: self.cargo_program.clone(),
            working_directory: self.workspace_root.clone(),
            environment: self.environment.clone(),
            target: self.target.clone(),
            selection: self.cargo_selection.clone(),
            packages: self.packages.clone(),
            manifest_path: None,
            recipe: self.example.as_ref().map_or(CargoRecipe::Library, |name| {
                CargoRecipe::Example(name.clone())
            }),
            configurations: Vec::new(),
            render_diagnostics: false,
            keep_going: false,
        }
    }
}

fn cargo_build_command(
    spec: &WasmBuildSpec,
    build_target_dir: &Path,
) -> Result<Command, WasmBuildError> {
    let absolute_target_dir = canonicalize_allow_missing(build_target_dir).map_err(|source| {
        WasmBuildError::io(
            "resolve Cargo build target directory",
            build_target_dir.to_owned(),
            source,
        )
    })?;
    let mut invocation = spec.cargo_invocation();
    invocation.environment.insert(
        "CARGO_TARGET_DIR".into(),
        absolute_target_dir.into_os_string(),
    );
    invocation.command().map_err(|source| {
        WasmBuildError::io(
            "render Cargo build invocation",
            spec.workspace_root.clone(),
            source,
        )
    })
}

fn communicate_cargo_build(
    spec: &WasmBuildSpec,
    build_target_dir: &Path,
    targets: &[WasmArtifactTarget],
    mut command: Command,
    progress: &mut ProgressReporter<'_>,
    spawn: &mut dyn FnMut(&mut Command) -> io::Result<OwnedChild>,
) -> Result<Vec<PathBuf>, WasmBuildError> {
    // The caller's spawner may override stdin and selects child/group custody.
    // Progress reporting never changes either selection.
    command
        .stdin(Stdio::null())
        .stdout(Stdio::piped())
        .stderr(Stdio::piped());
    let started = Instant::now();
    let mut child = spawn(&mut command).map_err(|source| WasmBuildError::CommandSpawn {
        phase: WasmBuildPhase::CargoBuild,
        program: spec.cargo_program.clone(),
        source,
    })?;
    let mut artifacts = vec![None; targets.len()];
    let mut collect = |message| collect_wasm_artifact(message, targets, &mut artifacts);
    let observed = communicate_cargo(
        &mut child,
        OutputLimits {
            // Retain bounded diagnostics independently of heartbeats.
            // Output volume never terminates a build, and builds have no deadline.
            stdout: OutputLimit::Truncate(CARGO_DIAGNOSTIC_BYTES),
            stderr: OutputLimit::Truncate(CARGO_DIAGNOSTIC_BYTES),
            timeout: None,
        },
        spec.cargo_message_bytes,
        || {
            progress.emit_heartbeat_if_due(WasmBuildPhase::CargoBuild, started.elapsed());
            false
        },
        |_, _| Ok(()),
        &mut collect,
    );
    let result = observed.execution;
    let evidence = result.map_err(|error| {
        WasmBuildError::io(
            "communicate with cargo build",
            PathBuf::from(&spec.cargo_program),
            // Retain typed Host evidence and secondary cleanup failures.
            io::Error::other(error),
        )
    })?;
    let status = evidence.status;
    if status.success() {
        observed.messages.map_err(|source| {
            WasmBuildError::io(
                "decode Cargo build artifacts",
                PathBuf::from(&spec.cargo_program),
                source,
            )
        })?;
        return artifacts
            .into_iter()
            .collect::<Option<Vec<_>>>()
            .ok_or_else(|| WasmBuildError::MissingArtifacts {
                paths: expected_artifacts(spec, build_target_dir, targets),
            });
    }
    Err(WasmBuildError::CommandFailed {
        phase: WasmBuildPhase::CargoBuild,
        status,
        stdout: cargo_diagnostic(&evidence.stdout, evidence.stdout_truncated),
        stderr: cargo_diagnostic(&evidence.stderr, evidence.stderr_truncated),
    })
}

fn collect_wasm_artifact(
    message: schema::Message,
    targets: &[WasmArtifactTarget],
    artifacts: &mut [Option<PathBuf>],
) -> io::Result<()> {
    let schema::Message::CompilerArtifact(artifact) = message else {
        return Ok(());
    };
    // Acquisition admits unique targets before dispatching Cargo.
    let Some((_, selected)) = targets.iter().zip(artifacts).find(|(target, _)| {
        artifact_matches(&artifact, &target.package_id, &target.name, &target.kind)
    }) else {
        return Ok(());
    };
    if !artifact
        .target
        .crate_types
        .contains(&schema::CrateType::CDyLib)
    {
        return Err(io::Error::new(
            io::ErrorKind::InvalidData,
            "Cargo reported a non-cdylib Wasm target",
        ));
    }
    let mut matching = wasm_artifact_paths(&artifact);
    let path = matching
        .next()
        .filter(|path| path.is_absolute())
        .ok_or_else(|| {
            io::Error::new(
                io::ErrorKind::InvalidData,
                "Cargo omitted absolute Wasm artifact path",
            )
        })?;
    if matching.next().is_some() || selected.replace(path.to_path_buf()).is_some() {
        return Err(io::Error::new(
            io::ErrorKind::InvalidData,
            "Cargo reported ambiguous Wasm artifacts",
        ));
    }
    Ok(())
}

fn cargo_diagnostic(bytes: &[u8], truncated: bool) -> String {
    let mut text = String::from_utf8_lossy(bytes).into_owned();
    if truncated {
        text.push_str("\n[diagnostic truncated after 1 MiB]\n");
    }
    text
}

fn ensure_command_success(
    phase: WasmBuildPhase,
    output: CompletedExecution,
) -> Result<CompletedExecution, WasmBuildError> {
    if output.status.success() {
        return Ok(output);
    }
    Err(WasmBuildError::CommandFailed {
        phase,
        status: output.status,
        stdout: String::from_utf8_lossy(&output.stdout).into_owned(),
        stderr: String::from_utf8_lossy(&output.stderr).into_owned(),
    })
}

fn artifact_relative_path(spec: &WasmBuildSpec, target: &WasmArtifactTarget) -> PathBuf {
    let profile = Path::new(&spec.target).join(
        spec.cargo_selection
            .profile_target_dir()
            .expect("Cargo profile was validated before artifact discovery"),
    );
    let directory = if spec.example.is_some() {
        profile.join("examples")
    } else {
        profile
    };
    directory.join(&target.file_name)
}

fn expected_artifacts(
    spec: &WasmBuildSpec,
    target_dir: &Path,
    targets: &[WasmArtifactTarget],
) -> Vec<PathBuf> {
    targets
        .iter()
        .map(|target| target_dir.join(artifact_relative_path(spec, target)))
        .collect()
}

fn artifact_publication_paths(
    spec: &WasmBuildSpec,
    cache_entry: &Path,
    targets: &[WasmArtifactTarget],
) -> Vec<(PathBuf, PathBuf)> {
    targets
        .iter()
        .map(|target| {
            let relative = artifact_relative_path(spec, target);
            (cache_entry.join(&relative), spec.target_dir.join(relative))
        })
        .collect()
}

#[derive(Debug, Eq, PartialEq)]
struct VerifiedWasmArtifact {
    cached_path: PathBuf,
    public_path: PathBuf,
    info: FileDigest,
}

fn cache_entry_directory(spec: &WasmBuildSpec, fingerprint: InputDigest) -> PathBuf {
    spec.target_dir
        .join(".ic-testkit/wasm-targets")
        .join(fingerprint.to_hex())
}

fn validated_artifact_set(
    artifacts: Vec<(PathBuf, PathBuf)>,
    fingerprint: InputDigest,
) -> Option<Vec<VerifiedWasmArtifact>> {
    let header = artifact_stamp_header(fingerprint);
    // Every digest has the same width. Derive admission from the actual encoder.
    let stamp_len = artifact_stamp_contents(&header, fingerprint).len();
    artifacts
        .into_iter()
        .map(|(artifact, public_path)| {
            let metadata = fs::metadata(&artifact).ok()?;
            if !metadata.is_file() || metadata.len() == 0 {
                return None;
            }
            let stamp =
                read_stamp_with_limit(&artifact_stamp_path(&artifact), stamp_len).ok()??;
            // An incomplete stamp or different build cannot be a hit. Reject it
            // before reading the Wasm bytes; then verify the complete exact stamp.
            if stamp.len() != stamp_len || !stamp.starts_with(&header) {
                return None;
            }
            let info = digest_file("wasm-artifact-v1", &artifact).ok()?;
            (stamp == artifact_stamp_contents(&header, info.digest)).then_some(
                VerifiedWasmArtifact {
                    cached_path: artifact,
                    public_path,
                    info,
                },
            )
        })
        .collect()
}

fn missing_artifacts(artifacts: &[PathBuf]) -> Vec<PathBuf> {
    artifacts
        .iter()
        .filter(|path| {
            fs::metadata(path).map_or(true, |metadata| !metadata.is_file() || metadata.len() == 0)
        })
        .cloned()
        .collect()
}

fn artifact_stamp_path(artifact: &Path) -> PathBuf {
    let mut name = artifact
        .file_name()
        .map_or_else(|| OsString::from("artifact"), OsString::from);
    name.push(".ic-testkit-build");
    artifact.with_file_name(name)
}

fn artifact_stamp_header(fingerprint: InputDigest) -> String {
    format!("{CACHE_FORMAT_VERSION}\nbuild-sha256:{fingerprint}\n")
}

fn artifact_stamp_contents(header: &str, artifact_digest: InputDigest) -> String {
    format!("{header}{ARTIFACT_STAMP_DIGEST_LABEL}{artifact_digest}\n")
}

fn write_artifact_stamp(
    artifact: &Path,
    fingerprint: InputDigest,
    info: &FileDigest,
    preserve_matching: bool,
) -> Result<(), WasmBuildError> {
    let stamp_path = artifact_stamp_path(artifact);
    let stamp = artifact_stamp_contents(&artifact_stamp_header(fingerprint), info.digest);
    if preserve_matching && destination_matches_bytes(&stamp_path, stamp.as_bytes()) {
        return Ok(());
    }
    ic_host_fs::durable::write_bytes(
        &stamp_path,
        ic_host_fs::durable::WriteOptions {
            mode: ic_host_fs::durable::PublicationMode::Replace,
            permissions: 0o666,
        },
        stamp.as_bytes(),
    )
    .map_err(|source| {
        WasmBuildError::io(
            "publish Wasm build stamp",
            stamp_path,
            io::Error::other(source),
        )
    })
}

fn publish_artifact_stamps(
    artifacts: Vec<(PathBuf, PathBuf)>,
    fingerprint: InputDigest,
) -> Result<Vec<VerifiedWasmArtifact>, WasmBuildError> {
    artifacts
        .into_iter()
        .map(|(cached_path, public_path)| {
            let info = digest_file("wasm-artifact-v1", &cached_path).map_err(|source| {
                WasmBuildError::io("hash built Wasm artifact", cached_path.clone(), source)
            })?;
            write_artifact_stamp(&cached_path, fingerprint, &info, false)?;
            Ok(VerifiedWasmArtifact {
                cached_path,
                public_path,
                info,
            })
        })
        .collect()
}

fn materialize_artifacts(
    artifacts: &[VerifiedWasmArtifact],
    fingerprint: InputDigest,
    preserve_matching: bool,
) -> Result<(), WasmBuildError> {
    for artifact in artifacts {
        materialize_verified_output(
            "wasm-artifact-v1",
            &artifact.cached_path,
            &artifact.public_path,
            preserve_matching.then_some(&artifact.info),
        )
        .map_err(|source| {
            WasmBuildError::io(
                "publish Wasm artifact",
                artifact.public_path.clone(),
                source,
            )
        })?;
    }
    // Complete every copy before stamping any public output.
    for artifact in artifacts {
        write_artifact_stamp(
            &artifact.public_path,
            fingerprint,
            &artifact.info,
            preserve_matching,
        )?;
    }
    Ok(())
}

fn copy_wasm_artifacts(
    source_artifacts: &[PathBuf],
    destinations: &[(PathBuf, PathBuf)],
) -> Result<(), WasmBuildError> {
    // Cargo paths are external evidence. Refuse a mismatched inventory before any copy.
    if source_artifacts.len() != destinations.len() {
        return Err(invalid_metadata(
            "Cargo artifact count does not match admitted outputs",
        ));
    }
    for (source, (cached, _)) in source_artifacts.iter().zip(destinations) {
        if source != cached {
            copy_file_atomic(source, cached).map_err(|source| {
                WasmBuildError::io(
                    "cache shared-incremental Wasm artifact",
                    cached.clone(),
                    source,
                )
            })?;
        }
    }
    Ok(())
}

fn create_dir_all(path: &Path, operation: &'static str) -> Result<(), WasmBuildError> {
    fs::create_dir_all(path)
        .map_err(|source| WasmBuildError::io(operation, path.to_owned(), source))
}

fn add_if_present(inputs: &mut Vec<(PathBuf, PathBuf)>, label: &str, path: PathBuf) {
    if path.exists() {
        inputs.push((PathBuf::from(label), path));
    }
}

fn invalid_metadata(message: &str) -> WasmBuildError {
    WasmBuildError::InvalidMetadata {
        message: message.to_owned(),
    }
}

impl WasmBuildError {
    fn io(operation: &'static str, path: impl Into<PathBuf>, source: io::Error) -> Self {
        Self::Io {
            operation,
            path: path.into(),
            source,
        }
    }
}

#[cfg(test)]
mod tests;
