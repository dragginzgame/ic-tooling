use std::{
    num::NonZeroUsize,
    ops::Deref,
    time::{Duration, Instant},
};

use super::{
    CachedPocketIcBaseline, ControllerSnapshotError, SnapshotRestoreFunding,
    bounded_pool::{BoundedSlotLease, BoundedSlotPool, Slot},
};

/// Whether validation is observing a newly built or restored baseline.
#[non_exhaustive]
#[derive(Clone, Debug, Eq, PartialEq)]
pub enum PreparedBaseline {
    /// The recipe just built this baseline.
    Built,
    /// The pool restored an existing baseline and the recipe validated its reset.
    Restored,
}

/// Recipe stage associated with a structured preparation failure.
#[non_exhaustive]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum BaselinePreparationStage {
    /// Constructing a new baseline.
    Build,
    /// Restoring captured canisters.
    RestoreCanisters,
    /// Resetting state outside the snapshots.
    ResetNonSnapshotState,
    /// Driving the restored topology to readiness.
    DriveToReadiness,
    /// Validating a newly built baseline.
    ValidateBuilt,
    /// Validating a restored baseline.
    ValidateRestored,
}

/// Why an invalid or failed slot was reconstructed.
#[non_exhaustive]
#[derive(Clone, Debug, Eq, PartialEq)]
pub enum RebuildReason {
    /// Captured snapshot restoration failed.
    SnapshotRestoreFailure,
    /// Non-snapshot reset failed.
    ResetFailure,
    /// Readiness or quiescence could not be established.
    ReadinessFailure,
    /// Final invariant validation failed.
    InvariantValidationFailure,
    /// A caller explicitly invalidated its lease.
    ExplicitLeaseInvalidation,
    /// A lease was dropped while its thread was unwinding.
    UnwindWhileLeased,
    /// Async slot preparation was cancelled before its invariants were restored.
    PreparationCancelled,
    /// Recipe-specific structured reason.
    RecipeClassified {
        /// Caller-defined rebuild reason.
        code: String,
    },
}

/// Recipe decision for a failed restored-slot preparation stage.
#[non_exhaustive]
#[derive(Clone, Debug, Eq, PartialEq)]
pub enum FailureDisposition {
    /// Return the failure without rebuilding during this acquisition.
    Fatal,
    /// Invalidate and rebuild the slot once.
    Rebuild(RebuildReason),
}

/// Timings for one baseline-pool acquisition.
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct BaselinePoolTimings {
    /// Time spent waiting for a capacity slot.
    pub wait: Duration,
    /// Time spent constructing a new baseline.
    pub build: Option<Duration>,
    /// Time spent restoring captured canisters.
    pub restore: Option<Duration>,
    /// Time spent resetting non-snapshot state.
    pub reset: Option<Duration>,
    /// Time spent driving the topology to readiness.
    pub readiness: Option<Duration>,
    /// Time spent validating final baseline invariants.
    pub validation: Option<Duration>,
    /// Time spent dropping an invalid baseline before rebuilding.
    pub stale_teardown: Option<Duration>,
    /// Complete acquisition duration.
    pub total: Duration,
}

/// One successful acquisition's slot, timings and preparation disposition.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct BaselinePoolOutcome {
    /// Diagnostic slot index.
    pub slot: usize,
    /// Acquisition phase timings.
    pub timings: BaselinePoolTimings,
    /// Whether this acquisition built, restored or rebuilt the baseline.
    pub disposition: BaselinePoolDisposition,
}

/// Whether a baseline-pool lease was built, restored, or rebuilt.
#[non_exhaustive]
#[derive(Clone, Debug, Eq, PartialEq)]
pub enum BaselinePoolDisposition {
    /// An empty slot was constructed and validated.
    Built,
    /// An existing slot was restored, reset, and validated.
    Restored,
    /// An invalid or failed slot was reconstructed and validated.
    Rebuilt(RebuildReason),
}

/// One failed recipe or contract stage while preparing a baseline slot.
#[non_exhaustive]
#[derive(Debug, thiserror::Error)]
pub enum BaselinePoolPreparationError<E> {
    /// Caller-owned recipe logic returned an error.
    #[error("baseline {stage} failed: {source}")]
    Recipe {
        /// Failed lifecycle stage.
        stage: BaselinePreparationStage,
        /// Caller-owned structured source error.
        source: E,
    },
    /// Canonical restoration of the captured snapshot set failed.
    #[error("baseline canister restore failed: {0}")]
    Restore(#[source] ControllerSnapshotError),
}

/// Failure to acquire a validated baseline-pool lease.
#[non_exhaustive]
#[derive(Debug)]
pub enum BaselinePoolError<E> {
    /// Shutdown has begun; no baseline was prepared or exposed.
    Closed {
        /// Queue wait before shutdown was observed.
        timings: Box<BaselinePoolTimings>,
    },
    /// Initial construction or a non-rebuilt preparation failed.
    Preparation {
        /// Recipe or contract failure that stopped acquisition.
        error: BaselinePoolPreparationError<E>,
        /// Phase timings recorded before acquisition failed.
        timings: Box<BaselinePoolTimings>,
    },
    /// Reused-slot preparation failed and its one rebuild attempt also failed.
    RecoveryFailed {
        /// Original restore/reset/readiness/validation failure.
        original: Box<BaselinePoolPreparationError<E>>,
        /// Failure while rebuilding or validating the replacement.
        rebuild: Box<BaselinePoolPreparationError<E>>,
        /// Combined timings for preparation, stale teardown, and rebuilding.
        timings: Box<BaselinePoolTimings>,
    },
}

// Final timings are attached only after preparation and recovery have finished.
enum PreparationFailure<E> {
    Single(BaselinePoolPreparationError<E>),
    Recovery {
        original: Box<BaselinePoolPreparationError<E>>,
        rebuild: Box<BaselinePoolPreparationError<E>>,
    },
}

impl<E> PreparationFailure<E> {
    fn with_timings(self, timings: BaselinePoolTimings) -> BaselinePoolError<E> {
        let timings = Box::new(timings);
        match self {
            Self::Single(error) => BaselinePoolError::Preparation { error, timings },
            Self::Recovery { original, rebuild } => BaselinePoolError::RecoveryFailed {
                original,
                rebuild,
                timings,
            },
        }
    }
}

/// Caller-owned build, preflight and non-snapshot reset policy for one pooled baseline.
pub trait PocketIcBaselineRecipe: Send + Sync + 'static {
    /// Metadata retained beside every baseline owned by this recipe.
    type Metadata: Send + Sync + 'static;
    /// Structured caller error shared by recipe lifecycle stages.
    type Error: std::error::Error + Send + Sync + 'static;

    /// Caller-selected funding applied immediately before canonical snapshot restore.
    fn snapshot_funding(&self) -> SnapshotRestoreFunding;

    /// Construct and capture one complete baseline.
    /// Stop/drain the complete snapshot set before capture, then start it after
    /// the complete set is captured and ready. Automatic progress stays disabled.
    ///
    /// # Errors
    /// Returns the recipe error if construction or baseline capture fails.
    fn build(
        &self,
    ) -> impl std::future::Future<
        Output = Result<CachedPocketIcBaseline<Self::Metadata>, Self::Error>,
    > + Send;

    /// Admit recipe-specific preconditions before the pool restores snapshots.
    ///
    /// Stop/drain every captured canister here unless the complete set is already
    /// stopped. Checks such as exact current cycle-balance guards also run here,
    /// before restoration can charge cycles. This cannot replace
    /// the pool's canonical restore or manufacture its completion evidence.
    ///
    /// # Errors
    /// Returns the recipe error when a precondition forbids snapshot restoration.
    fn prepare_restore(
        &self,
        _baseline: &CachedPocketIcBaseline<Self::Metadata>,
    ) -> impl std::future::Future<Output = Result<(), Self::Error>> + Send {
        async { Ok(()) }
    }

    /// Reset state not covered by canister snapshots.
    /// The recipe owns its reset schema, evidence and required/achieved checks.
    /// Return success only after those checks pass; a failure never exposes the
    /// partially reset baseline. Readiness and final validation run afterward.
    ///
    /// # Errors
    /// Returns the recipe error if the required non-snapshot reset fails.
    fn reset_non_snapshot_state(
        &self,
        baseline: &CachedPocketIcBaseline<Self::Metadata>,
    ) -> impl std::future::Future<Output = Result<(), Self::Error>> + Send;

    /// Drive the topology to the recipe's readiness boundary.
    /// Successful completion permits final invariant validation.
    /// Restart the complete stopped set here, after canonical restore and the
    /// non-snapshot reset have succeeded. Never restart a partially restored set.
    ///
    /// # Errors
    /// Returns the recipe error if the caller-defined readiness boundary is not reached.
    fn drive_to_readiness(
        &self,
        baseline: &CachedPocketIcBaseline<Self::Metadata>,
    ) -> impl std::future::Future<Output = Result<(), Self::Error>> + Send;

    /// Validate the same baseline invariants after build and restore.
    /// Success admits this pool-owned recipe's baseline for leasing.
    ///
    /// # Errors
    /// Returns the recipe error when baseline invariants are not satisfied.
    fn validate(
        &self,
        baseline: &CachedPocketIcBaseline<Self::Metadata>,
        preparation: &PreparedBaseline,
    ) -> impl std::future::Future<Output = Result<(), Self::Error>> + Send;

    /// Classify a restored-slot recipe failure as fatal or rebuildable.
    fn classify_failure(
        &self,
        stage: BaselinePreparationStage,
        _error: &(dyn std::error::Error + 'static),
    ) -> FailureDisposition {
        FailureDisposition::Rebuild(stage.default_rebuild_reason())
    }
}

/// Caller-owned runtime-capacity pool of independently restorable `PocketIC` baselines.
///
/// One pool structurally owns one [`PocketIcBaselineRecipe`]. A warm
/// acquisition restores the complete captured canister set, applies the
/// recipe's validated non-snapshot reset, reaches its readiness boundary and
/// validates final invariants before exposing a lease.
/// Each capacity slot owns an independent `PocketIC` instance.
///
/// Snapshot reuse is not a complete `PocketIC` rollback. The recipe must account
/// for time, extra canisters, pending messages, subnet state, cycles, and
/// external resources when those domains matter to its tests.
pub struct CachedPocketIcBaselinePool<R>
where
    R: PocketIcBaselineRecipe,
{
    recipe: R,
    slots: BoundedSlotPool<CachedPocketIcBaseline<R::Metadata>, RebuildReason>,
}

/// Exclusive lease of one validated pooled `PocketIC` baseline.
pub struct CachedPocketIcBaselinePoolGuard<'a, R>
where
    R: PocketIcBaselineRecipe,
{
    slot: BoundedSlotLease<'a, CachedPocketIcBaseline<R::Metadata>, RebuildReason>,
}

impl BaselinePreparationStage {
    /// Default rebuild reason used by [`PocketIcBaselineRecipe::classify_failure`].
    ///
    /// Recipes that override classification can use this for their fallback
    /// after handling a more specific error such as dead `PocketIC` transport.
    #[must_use]
    pub fn default_rebuild_reason(self) -> RebuildReason {
        match self {
            Self::RestoreCanisters => RebuildReason::SnapshotRestoreFailure,
            Self::ResetNonSnapshotState => RebuildReason::ResetFailure,
            Self::DriveToReadiness => RebuildReason::ReadinessFailure,
            Self::ValidateRestored | Self::ValidateBuilt => {
                RebuildReason::InvariantValidationFailure
            }
            Self::Build => RebuildReason::RecipeClassified {
                code: "build".to_owned(),
            },
        }
    }
}

impl std::fmt::Display for BaselinePoolTimings {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            formatter,
            "total={:?} wait={:?} build={:?} restore={:?} reset={:?} readiness={:?} validation={:?} stale_teardown={:?}",
            self.total,
            self.wait,
            self.build,
            self.restore,
            self.reset,
            self.readiness,
            self.validation,
            self.stale_teardown,
        )
    }
}

impl std::fmt::Display for BaselinePoolOutcome {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match &self.disposition {
            BaselinePoolDisposition::Built => formatter.write_str("built")?,
            BaselinePoolDisposition::Restored => formatter.write_str("restored")?,
            BaselinePoolDisposition::Rebuilt(reason) => {
                write!(
                    formatter,
                    "rebuilt slot={} reason={reason:?} {}",
                    self.slot, self.timings
                )?;
                return Ok(());
            }
        }
        write!(formatter, " slot={} {}", self.slot, self.timings)
    }
}

impl<E> BaselinePoolError<E> {
    /// Timings recorded before this acquisition failed.
    #[must_use]
    pub const fn timings(&self) -> BaselinePoolTimings {
        match self {
            Self::Closed { timings }
            | Self::Preparation { timings, .. }
            | Self::RecoveryFailed { timings, .. } => **timings,
        }
    }
}

impl<R> CachedPocketIcBaselinePool<R>
where
    R: PocketIcBaselineRecipe,
{
    /// Create a runtime-capacity pool that structurally owns one recipe.
    ///
    /// Construction is const and lazy: slots are allocated on first acquisition,
    /// and the recipe builds a baseline only when a slot needs one.
    #[must_use]
    pub const fn new(capacity: NonZeroUsize, recipe: R) -> Self {
        Self {
            recipe,
            slots: BoundedSlotPool::new(capacity),
        }
    }

    /// Consume the pool and await [`Self::shutdown`]. Upstream deletion is best effort.
    pub async fn close(self) {
        self.shutdown().await;
    }

    /// Stop admission, await all leases, and attempt every cached instance deletion.
    ///
    /// Works through a shared or static reference. Acquisitions after shutdown
    /// begins return [`BaselinePoolError::Closed`]; queued acquisitions cannot
    /// prepare another baseline. Already exposed leases remain usable until
    /// their guards are dropped. Do not await shutdown while holding a lease
    /// in the same task. Borrowed server owners must outlive this operation.
    ///
    /// Upstream deletion logs failures instead of returning them. Concurrent
    /// shutdowns serialize and completed attempts are never replayed. Completion
    /// is not an acknowledgement that a borrowed server deleted every instance.
    ///
    /// Cancellation leaves admission closed and retains unattempted baselines
    /// for a later shutdown. An in-flight deletion has an unknown outcome and
    /// requires caller reconciliation; it is not retried. Rust Drop performs
    /// no client network I/O.
    pub async fn shutdown(&self) {
        self.slots
            .shutdown(async |baseline| {
                baseline.close().await;
                Ok::<(), std::convert::Infallible>(())
            })
            .await;
    }

    /// Maximum number of simultaneously leased `PocketIC` baselines.
    #[must_use]
    pub const fn capacity(&self) -> NonZeroUsize {
        self.slots.capacity()
    }

    /// Acquire one fully built or restored and validated baseline lease.
    ///
    /// A rebuildable warm-preparation failure discards the stale slot and
    /// performs at most one build attempt. Recipe and caller panics are never
    /// converted into cache misses; the lease is invalidated while the panic
    /// continues unwinding.
    ///
    /// # Errors
    ///
    /// Returns a stage-specific recipe or contract error. If warm preparation
    /// and its one recovery build both fail, the error preserves both causes.
    #[expect(
        clippy::significant_drop_tightening,
        reason = "lease and coordinator scope deliberately cover the complete operation"
    )]
    pub async fn acquire(
        &self,
    ) -> Result<
        (CachedPocketIcBaselinePoolGuard<'_, R>, BaselinePoolOutcome),
        BaselinePoolError<R::Error>,
    > {
        let total_started = Instant::now();
        let mut slot = self.slots.acquire().await.ok_or_else(|| {
            let wait = total_started.elapsed();
            BaselinePoolError::Closed {
                timings: Box::new(BaselinePoolTimings {
                    wait,
                    total: wait,
                    ..BaselinePoolTimings::default()
                }),
            }
        })?;
        slot.begin_preparation();
        let mut timings = BaselinePoolTimings {
            wait: slot.wait(),
            ..BaselinePoolTimings::default()
        };
        let slot_index = slot.slot_index();

        let result = Box::pin(self.prepare_slot(&mut slot, &mut timings)).await;
        slot.finish_preparation();
        timings.total = total_started.elapsed();
        let disposition = result.map_err(|error| error.with_timings(timings))?;
        Ok((
            CachedPocketIcBaselinePoolGuard { slot },
            BaselinePoolOutcome {
                slot: slot_index,
                timings,
                disposition,
            },
        ))
    }

    async fn prepare_slot(
        &self,
        slot: &mut BoundedSlotLease<'_, CachedPocketIcBaseline<R::Metadata>, RebuildReason>,
        timings: &mut BaselinePoolTimings,
    ) -> Result<BaselinePoolDisposition, PreparationFailure<R::Error>> {
        let (disposition, original) = match slot.state() {
            Slot::Reusable(_) => {
                let Err(original) = self.prepare_reused(slot, timings).await else {
                    return Ok(BaselinePoolDisposition::Restored);
                };
                let FailureDisposition::Rebuild(reason) = self.failure_disposition(&original)
                else {
                    // A failed restore may have partially changed the instance.
                    // Discard it without treating a fatal failure as a rebuild.
                    Self::discard_stale_slot(slot, timings).await;
                    return Err(PreparationFailure::Single(original));
                };
                (BaselinePoolDisposition::Rebuilt(reason), Some(original))
            }
            Slot::Empty => (BaselinePoolDisposition::Built, None),
            Slot::Invalidated { reason, .. } => {
                (BaselinePoolDisposition::Rebuilt(reason.clone()), None)
            }
            Slot::Interrupted(_) => (
                BaselinePoolDisposition::Rebuilt(RebuildReason::PreparationCancelled),
                None,
            ),
            Slot::Unwound(_) => (
                BaselinePoolDisposition::Rebuilt(RebuildReason::UnwindWhileLeased),
                None,
            ),
        };

        if slot.get().is_some() {
            Self::discard_stale_slot(slot, timings).await;
        }
        let build = self.build_slot(slot, timings).await;
        if let Err(error) = build {
            return Err(match original {
                Some(original) => PreparationFailure::Recovery {
                    original: Box::new(original),
                    rebuild: Box::new(error),
                },
                None => PreparationFailure::Single(error),
            });
        }

        Ok(disposition)
    }

    async fn prepare_reused(
        &self,
        slot: &BoundedSlotLease<'_, CachedPocketIcBaseline<R::Metadata>, RebuildReason>,
        timings: &mut BaselinePoolTimings,
    ) -> Result<(), BaselinePoolPreparationError<R::Error>> {
        let baseline = slot
            .get()
            .expect("reusable baseline slot must be populated");

        let started = Instant::now();
        let restore = async {
            self.recipe
                .prepare_restore(baseline)
                .await
                .map_err(|source| BaselinePoolPreparationError::Recipe {
                    stage: BaselinePreparationStage::RestoreCanisters,
                    source,
                })?;
            baseline
                .restore(self.recipe.snapshot_funding())
                .await
                .map_err(BaselinePoolPreparationError::Restore)
        }
        .await;
        add_timing(&mut timings.restore, started.elapsed());
        restore?;

        timed_recipe_stage(
            &mut timings.reset,
            BaselinePreparationStage::ResetNonSnapshotState,
            self.recipe.reset_non_snapshot_state(baseline),
        )
        .await?;
        timed_recipe_stage(
            &mut timings.readiness,
            BaselinePreparationStage::DriveToReadiness,
            self.recipe.drive_to_readiness(baseline),
        )
        .await?;

        let preparation = PreparedBaseline::Restored;
        self.validate_baseline(baseline, &preparation, timings)
            .await?;
        Ok(())
    }

    async fn build_slot(
        &self,
        slot: &mut BoundedSlotLease<'_, CachedPocketIcBaseline<R::Metadata>, RebuildReason>,
        timings: &mut BaselinePoolTimings,
    ) -> Result<(), BaselinePoolPreparationError<R::Error>> {
        let baseline = timed_recipe_stage(
            &mut timings.build,
            BaselinePreparationStage::Build,
            self.recipe.build(),
        )
        .await?;

        if let Err(error) = self
            .validate_baseline(&baseline, &PreparedBaseline::Built, timings)
            .await
        {
            drop_baseline_safely(baseline).await;
            return Err(error);
        }
        let replaced = slot.replace(baseline);
        debug_assert!(replaced.is_none());
        Ok(())
    }

    async fn validate_baseline(
        &self,
        baseline: &CachedPocketIcBaseline<R::Metadata>,
        preparation: &PreparedBaseline,
        timings: &mut BaselinePoolTimings,
    ) -> Result<(), BaselinePoolPreparationError<R::Error>> {
        let stage = match preparation {
            PreparedBaseline::Built => BaselinePreparationStage::ValidateBuilt,
            PreparedBaseline::Restored => BaselinePreparationStage::ValidateRestored,
        };
        timed_recipe_stage(
            &mut timings.validation,
            stage,
            self.recipe.validate(baseline, preparation),
        )
        .await
    }

    fn failure_disposition(
        &self,
        error: &BaselinePoolPreparationError<R::Error>,
    ) -> FailureDisposition {
        match error {
            BaselinePoolPreparationError::Recipe { stage, source } => {
                self.recipe.classify_failure(*stage, source)
            }
            BaselinePoolPreparationError::Restore(source) => self
                .recipe
                .classify_failure(BaselinePreparationStage::RestoreCanisters, source),
        }
    }

    async fn discard_stale_slot(
        slot: &mut BoundedSlotLease<'_, CachedPocketIcBaseline<R::Metadata>, RebuildReason>,
        timings: &mut BaselinePoolTimings,
    ) {
        let started = Instant::now();
        if let Some(stale) = slot.take() {
            drop_baseline_safely(stale).await;
        }
        timings.stale_teardown = Some(started.elapsed());
    }
}

impl<R> CachedPocketIcBaselinePoolGuard<'_, R>
where
    R: PocketIcBaselineRecipe,
{
    /// Diagnostic slot index held by this lease.
    #[must_use]
    pub const fn slot(&self) -> usize {
        self.slot.slot_index()
    }

    /// Mark this slot non-reusable with a structured rebuild reason.
    pub fn invalidate(&mut self, reason: RebuildReason) {
        self.slot.invalidate(reason);
    }
}

impl<R> Deref for CachedPocketIcBaselinePoolGuard<'_, R>
where
    R: PocketIcBaselineRecipe,
{
    type Target = CachedPocketIcBaseline<R::Metadata>;

    fn deref(&self) -> &Self::Target {
        self.slot
            .get()
            .expect("leased baseline pool slot must be populated")
    }
}

async fn timed_recipe_stage<T, E>(
    timing: &mut Option<Duration>,
    stage: BaselinePreparationStage,
    operation: impl std::future::Future<Output = Result<T, E>>,
) -> Result<T, BaselinePoolPreparationError<E>> {
    let started = Instant::now();
    let result = operation.await;
    add_timing(timing, started.elapsed());
    result.map_err(|source| BaselinePoolPreparationError::Recipe { stage, source })
}

const fn add_timing(total: &mut Option<Duration>, elapsed: Duration) {
    *total = Some(match *total {
        None => elapsed,
        Some(previous) => previous.saturating_add(elapsed),
    });
}

async fn drop_baseline_safely<M>(baseline: CachedPocketIcBaseline<M>) {
    baseline.close().await;
}

impl std::fmt::Display for BaselinePreparationStage {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        formatter.write_str(match self {
            Self::Build => "build",
            Self::RestoreCanisters => "canister restore",
            Self::ResetNonSnapshotState => "non-snapshot reset",
            Self::DriveToReadiness => "readiness",
            Self::ValidateBuilt => "built-baseline validation",
            Self::ValidateRestored => "restored-baseline validation",
        })
    }
}

impl<E> std::fmt::Display for BaselinePoolError<E>
where
    E: std::fmt::Display,
{
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Closed { .. } => formatter.write_str("baseline pool is shut down"),
            Self::Preparation { error, .. } => error.fmt(formatter),
            Self::RecoveryFailed {
                original, rebuild, ..
            } => write!(
                formatter,
                "baseline preparation failed ({original}); rebuilding the slot also failed: {rebuild}",
            ),
        }
    }
}

impl<E> std::error::Error for BaselinePoolError<E>
where
    E: std::error::Error + 'static,
{
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        match self {
            Self::Closed { .. } => None,
            Self::Preparation { error, .. } => Some(error),
            Self::RecoveryFailed { original, .. } => Some(original.as_ref()),
        }
    }
}

#[cfg(test)]
mod tests;
