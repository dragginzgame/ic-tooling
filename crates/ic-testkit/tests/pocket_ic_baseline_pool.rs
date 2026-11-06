//! Integration checks for pocket ic baseline pool behavior.
use ic_testkit::pocket_ic::ClientError;
mod support;

use std::{
    num::NonZeroUsize,
    sync::{
        Arc, Mutex, OnceLock,
        atomic::{AtomicBool, AtomicUsize, Ordering},
        mpsc,
    },
    time::Duration,
};

use ic_platform_types::principal::ExternalPrincipal as Principal;
use ic_testkit::pic::{
    BaselinePoolDisposition, BaselinePoolError, BaselinePoolOutcome, BaselinePoolPreparationError,
    BaselinePreparationStage, CachedPocketIcBaseline, CachedPocketIcBaselinePool,
    CachedPocketIcBaselinePoolGuard, CanisterSnapshotTarget, ControllerSnapshotError,
    ExtraCanisterPolicy, FailureDisposition, PocketIcBaselineRecipe, PocketIcStartupConfig,
    PocketIcStartupError, PreparedBaseline, RebuildReason, ResetPolicies, ResetReceipt,
    ResetRequirements, SnapshotRestoreFunding, TimeResetPolicy, is_dead_pocket_ic_transport_error,
};
use ic_testkit::pocket_ic::PocketIcBuilder;

const EMPTY_WASM: &[u8] = b"\0asm\x01\0\0\0";
const OPERATION_TIMEOUT: Duration = Duration::from_secs(30);

#[tokio::test(flavor = "multi_thread", worker_threads = 2)]
async fn pool_shutdown_accepts_static_leases_and_waits_before_deletion() {
    use std::future::Future as _;
    static POOL: OnceLock<CachedPocketIcBaselinePool<TwoCanisterRecipe>> = OnceLock::new();
    let pool = POOL.get_or_init(|| {
        CachedPocketIcBaselinePool::new(
            NonZeroUsize::new(2).unwrap(),
            TwoCanisterRecipe::new(RecipeControls::default()),
        )
    });
    let (first, _): (
        CachedPocketIcBaselinePoolGuard<'static, TwoCanisterRecipe>,
        _,
    ) = pool.acquire().await.unwrap();
    let (second, _) = pool.acquire().await.unwrap();
    let urls = [
        first.pocket_ic().get_server_url(),
        second.pocket_ic().get_server_url(),
    ];
    let mut queued = Box::pin(pool.acquire());
    assert!(
        std::future::poll_fn(|cx| std::task::Poll::Ready(queued.as_mut().poll(cx)))
            .await
            .is_pending()
    );
    let mut shutdown = Box::pin(pool.shutdown());
    assert!(
        std::future::poll_fn(|cx| std::task::Poll::Ready(shutdown.as_mut().poll(cx)))
            .await
            .is_pending()
    );
    assert!(matches!(
        pool.acquire().await,
        Err(BaselinePoolError::Closed { .. })
    ));
    drop(first);
    assert!(matches!(
        queued.await,
        Err(BaselinePoolError::Closed { .. })
    ));
    second.pocket_ic().topology().await.unwrap(); // held work remains usable.
    drop(second);
    assert!(
        tokio::time::timeout(OPERATION_TIMEOUT, shutdown)
            .await
            .unwrap()
            .is_empty()
    );
    for url in urls {
        assert!(
            ic_testkit::pocket_ic::PocketIc::list_instances(url)
                .await
                .is_err()
        );
    }
    assert!(pool.shutdown().await.is_empty());
}

#[tokio::test(flavor = "multi_thread", worker_threads = 2)]
async fn pool_shutdown_returns_every_borrowed_server_deletion_failure() {
    let server = PocketIcStartupConfig::Spawn {
        server_binary: std::env::var_os("POCKET_IC_BIN")
            .expect("admit the installed server first")
            .into(),
        timeout: OPERATION_TIMEOUT,
        server_hard_ttl: Some(Duration::from_secs(60)),
        server_idle_ttl: None,
        server_output_files: None,
    }
    .start_managed_server()
    .unwrap();
    let controls = RecipeControls::default();
    let pool = CachedPocketIcBaselinePool::new(
        NonZeroUsize::new(3).unwrap(),
        TwoCanisterRecipe::new(controls.clone()),
    );
    *controls.first_server_url.lock().unwrap() = Some(server.url().to_owned());
    let (first, _) = pool.acquire().await.unwrap();
    *controls.first_server_url.lock().unwrap() = Some(server.url().to_owned());
    let (second, _) = pool.acquire().await.unwrap();
    assert_ne!(
        first.pocket_ic().instance_id,
        second.pocket_ic().instance_id
    );
    drop((first, second));
    drop(server); // only this test's dedicated borrowed server is stopped.
    let failures = tokio::time::timeout(OPERATION_TIMEOUT, pool.shutdown())
        .await
        .unwrap();
    assert_eq!(
        failures.len(),
        2,
        "failure of the first delete cannot skip the second"
    );
    assert!(
        failures
            .iter()
            .all(|error| is_dead_pocket_ic_transport_error(error))
    );
    assert_eq!(controls.builds.load(Ordering::SeqCst), 2);
    assert!(
        pool.shutdown().await.is_empty(),
        "failed deletes are not replayed"
    );
}

#[derive(Clone, Default)]
struct RecipeControls {
    builds: Arc<AtomicUsize>,
    built_validations: Arc<AtomicUsize>,
    restored_validations: Arc<AtomicUsize>,
    fail_next_build: Arc<AtomicBool>,
    fail_next_restore: Arc<AtomicBool>,
    fatal_restore: Arc<AtomicBool>,
    fail_next_reset: Arc<AtomicBool>,
    omit_next_coverage: Arc<AtomicBool>,
    mismatch_next_coverage: Arc<AtomicBool>,
    fail_next_readiness: Arc<AtomicBool>,
    fail_next_validation: Arc<AtomicBool>,
    panic_next_restore: Arc<AtomicBool>,
    tracked_extra_canister: Arc<Mutex<Option<Principal>>>,
    first_server_url: Arc<Mutex<Option<String>>>,
}

struct TwoCanisterRecipe {
    requirements: ResetRequirements,
    controls: RecipeControls,
    guarded_domain: Option<GuardedDomain>,
}

#[derive(Clone, Copy, Eq, PartialEq)]
enum GuardedDomain {
    Time,
    Cycles,
    ExtraCanisters,
}

struct TwoCanisterMetadata {
    canister_ids: [Principal; 2],
    baseline_time_nanos: u64,
    baseline_cycles: [u128; 2],
}

async fn lifecycle(
    call: impl AsyncFnOnce() -> Result<(), ic_testkit::pocket_ic::ClientError>,
) -> Result<(), TestRecipeError> {
    call().await.map_err(TestRecipeError::Lifecycle)
}

#[derive(Debug)]
enum TestRecipeError {
    Lifecycle(ClientError),
    Snapshot(ControllerSnapshotError),
    Startup(PocketIcStartupError),
    DomainMutation(&'static str),
    Synthetic(&'static str),
}

impl TwoCanisterRecipe {
    fn new(controls: RecipeControls) -> Self {
        Self {
            requirements: ResetRequirements {
                funding: SnapshotRestoreFunding::Preserve,
                policies: ResetPolicies::default(),
            },
            controls,
            guarded_domain: None,
        }
    }

    fn guarding_domain(controls: RecipeControls, domain: GuardedDomain) -> Self {
        let policies = ResetPolicies {
            pocket_ic_time: (domain == GuardedDomain::Time)
                .then_some(TimeResetPolicy::RebuildOnMutation),
            extra_canisters: (domain == GuardedDomain::ExtraCanisters)
                .then_some(ExtraCanisterPolicy::RebuildOnChange),
            ..ResetPolicies::default()
        };
        Self {
            requirements: ResetRequirements {
                funding: SnapshotRestoreFunding::Preserve,
                policies,
            },
            controls,
            guarded_domain: Some(domain),
        }
    }
}

impl PocketIcBaselineRecipe for TwoCanisterRecipe {
    type Error = TestRecipeError;
    type Metadata = TwoCanisterMetadata;

    fn reset_requirements(&self) -> &ResetRequirements {
        &self.requirements
    }

    async fn build(&self) -> Result<CachedPocketIcBaseline<Self::Metadata>, Self::Error> {
        self.controls.builds.fetch_add(1, Ordering::SeqCst);
        if self.controls.fail_next_build.swap(false, Ordering::SeqCst) {
            return Err(TestRecipeError::Synthetic("requested build failure"));
        }
        *self
            .controls
            .tracked_extra_canister
            .lock()
            .unwrap_or_else(std::sync::PoisonError::into_inner) = None;

        let first_server_url = self
            .controls
            .first_server_url
            .lock()
            .unwrap_or_else(std::sync::PoisonError::into_inner)
            .take();
        let pocket_ic = if let Some(server_url) = first_server_url {
            PocketIcStartupConfig::Connect {
                server_url,
                timeout: OPERATION_TIMEOUT,
            }
            .build_instance(PocketIcBuilder::new().with_application_subnet())
            .await
            .map_err(TestRecipeError::Startup)?
        } else {
            support::pocket_ic().await
        };
        let canister_ids = [
            pocket_ic.instance().create_canister().await.unwrap(),
            pocket_ic.instance().create_canister().await.unwrap(),
        ];
        for canister_id in canister_ids {
            pocket_ic
                .instance()
                .install_canister(canister_id, EMPTY_WASM.to_vec(), vec![], None)
                .await
                .unwrap();
            lifecycle(async || pocket_ic.instance().stop_canister(canister_id, None).await).await?;
        }

        let mut baseline = CachedPocketIcBaseline::capture(
            pocket_ic,
            canister_ids.map(|id| CanisterSnapshotTarget {
                canister_id: id,
                sender: None,
            }),
            TwoCanisterMetadata {
                canister_ids,
                baseline_time_nanos: 0,
                baseline_cycles: [0; 2],
            },
        )
        .await
        .map_err(TestRecipeError::from)?;
        for id in canister_ids {
            lifecycle(async || baseline.pocket_ic().start_canister(id, None).await).await?;
        }
        let baseline_time_nanos = baseline
            .pocket_ic()
            .get_time()
            .await
            .unwrap()
            .as_nanos_since_unix_epoch();
        let baseline_cycles = [
            baseline
                .pocket_ic()
                .cycle_balance(canister_ids[0])
                .await
                .unwrap(),
            baseline
                .pocket_ic()
                .cycle_balance(canister_ids[1])
                .await
                .unwrap(),
        ];
        let metadata = baseline.metadata_mut();
        metadata.baseline_time_nanos = baseline_time_nanos;
        metadata.baseline_cycles = baseline_cycles;
        Ok(baseline)
    }

    async fn prepare_restore(
        &self,
        baseline: &CachedPocketIcBaseline<Self::Metadata>,
    ) -> Result<(), Self::Error> {
        assert!(
            !self
                .controls
                .panic_next_restore
                .swap(false, Ordering::SeqCst),
            "requested recipe-hook panic"
        );
        if self.guarded_domain == Some(GuardedDomain::Cycles) {
            let [first, second] = baseline.metadata().canister_ids;
            let current_cycles = [
                baseline.pocket_ic().cycle_balance(first).await.unwrap(),
                baseline.pocket_ic().cycle_balance(second).await.unwrap(),
            ];
            if current_cycles != baseline.metadata().baseline_cycles {
                return Err(TestRecipeError::DomainMutation("canister-cycles"));
            }
        }
        if self
            .controls
            .fail_next_restore
            .swap(false, Ordering::SeqCst)
        {
            return Err(TestRecipeError::Synthetic("requested restore failure"));
        }

        for id in baseline.metadata().canister_ids {
            lifecycle(async || baseline.pocket_ic().stop_canister(id, None).await).await?;
        }
        Ok(())
    }

    async fn reset_non_snapshot_state(
        &self,
        baseline: &CachedPocketIcBaseline<Self::Metadata>,
    ) -> Result<ResetReceipt, Self::Error> {
        if self.controls.fail_next_reset.swap(false, Ordering::SeqCst) {
            return Err(TestRecipeError::Synthetic("requested reset failure"));
        }
        match self.guarded_domain {
            None | Some(GuardedDomain::Cycles) => Ok(ResetReceipt::default()),
            Some(GuardedDomain::Time) => {
                if baseline
                    .pocket_ic()
                    .get_time()
                    .await
                    .unwrap()
                    .as_nanos_since_unix_epoch()
                    != baseline.metadata().baseline_time_nanos
                {
                    return Err(TestRecipeError::DomainMutation("pocket-ic-time"));
                }
                Ok(ResetReceipt {
                    policies: ResetPolicies {
                        pocket_ic_time: Some(TimeResetPolicy::RebuildOnMutation),
                        ..ResetPolicies::default()
                    },
                })
            }
            Some(GuardedDomain::ExtraCanisters) => {
                let extra_canister = *self
                    .controls
                    .tracked_extra_canister
                    .lock()
                    .unwrap_or_else(std::sync::PoisonError::into_inner);
                if let Some(canister_id) = extra_canister
                    && baseline
                        .pocket_ic()
                        .canister_status(canister_id, None)
                        .await
                        .is_ok()
                {
                    return Err(TestRecipeError::DomainMutation("extra-canister"));
                }
                Ok(ResetReceipt {
                    policies: ResetPolicies {
                        extra_canisters: if self
                            .controls
                            .omit_next_coverage
                            .swap(false, Ordering::SeqCst)
                        {
                            None
                        } else if self
                            .controls
                            .mismatch_next_coverage
                            .swap(false, Ordering::SeqCst)
                        {
                            Some(ExtraCanisterPolicy::RemoveTracked)
                        } else {
                            Some(ExtraCanisterPolicy::RebuildOnChange)
                        },
                        ..ResetPolicies::default()
                    },
                })
            }
        }
    }

    async fn drive_to_readiness(
        &self,
        baseline: &CachedPocketIcBaseline<Self::Metadata>,
    ) -> Result<(), Self::Error> {
        if self
            .controls
            .fail_next_readiness
            .swap(false, Ordering::SeqCst)
        {
            return Err(TestRecipeError::Synthetic("requested readiness failure"));
        }
        for id in baseline.metadata().canister_ids {
            lifecycle(async || baseline.pocket_ic().start_canister(id, None).await).await?;
        }
        Ok(())
    }

    async fn validate(
        &self,
        baseline: &CachedPocketIcBaseline<Self::Metadata>,
        preparation: &PreparedBaseline,
    ) -> Result<(), Self::Error> {
        if self
            .controls
            .fail_next_validation
            .swap(false, Ordering::SeqCst)
        {
            return Err(TestRecipeError::Synthetic("requested validation failure"));
        }
        match preparation {
            PreparedBaseline::Built => {
                self.controls
                    .built_validations
                    .fetch_add(1, Ordering::SeqCst);
            }
            PreparedBaseline::Restored { .. } => {
                self.controls
                    .restored_validations
                    .fetch_add(1, Ordering::SeqCst);
            }
            _ => {}
        }
        for canister_id in baseline.metadata().canister_ids {
            let status = baseline
                .pocket_ic()
                .canister_status(canister_id, None)
                .await
                .map_err(|_| TestRecipeError::Synthetic("canister status failed"))?;
            if status.module_hash.is_none() {
                return Err(TestRecipeError::Synthetic(
                    "baseline canister has no installed module",
                ));
            }
        }

        Ok(())
    }

    fn classify_failure(
        &self,
        stage: BaselinePreparationStage,
        error: &(dyn std::error::Error + 'static),
    ) -> FailureDisposition {
        if self.controls.fatal_restore.load(Ordering::SeqCst) {
            FailureDisposition::Fatal
        } else if is_dead_pocket_ic_transport_error(error) {
            FailureDisposition::Rebuild(RebuildReason::DeadPocketIcTransport)
        } else if let Some(TestRecipeError::DomainMutation(code)) =
            error.downcast_ref::<TestRecipeError>()
        {
            FailureDisposition::Rebuild(RebuildReason::RecipeClassified {
                code: (*code).to_string(),
            })
        } else {
            FailureDisposition::Rebuild(stage.default_rebuild_reason())
        }
    }
}

impl From<ControllerSnapshotError> for TestRecipeError {
    fn from(error: ControllerSnapshotError) -> Self {
        Self::Snapshot(error)
    }
}

impl std::fmt::Display for TestRecipeError {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Snapshot(error) => error.fmt(formatter),
            Self::Startup(error) => error.fmt(formatter),
            Self::Lifecycle(error) => error.fmt(formatter),
            Self::DomainMutation(domain) => write!(formatter, "{domain} mutated while leased"),
            Self::Synthetic(message) => formatter.write_str(message),
        }
    }
}

impl std::error::Error for TestRecipeError {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        match self {
            Self::Snapshot(error) => Some(error),
            Self::Startup(error) => Some(error),
            Self::Lifecycle(error) => Some(error),
            Self::DomainMutation(_) | Self::Synthetic(_) => None,
        }
    }
}

#[tokio::test(flavor = "multi_thread", worker_threads = 2)]
async fn multi_canister_pool_restores_and_explicitly_rebuilds_one_slot() {
    let controls = RecipeControls::default();
    let pool = CachedPocketIcBaselinePool::new(
        NonZeroUsize::new(1).unwrap(),
        TwoCanisterRecipe::new(controls.clone()),
    );
    assert_eq!(pool.capacity().get(), 1);

    let (baseline, outcome) = pool.acquire().await.expect("first baseline should build");
    let timings = outcome.timings;
    assert!(timings.build().is_some());
    assert!(timings.restore().is_none());
    assert!(timings.validation().is_some());
    assert!(matches!(
        outcome.disposition,
        BaselinePoolDisposition::Built
    ));
    let first_instance = baseline.pocket_ic().instance_id;
    let canister_ids = baseline.metadata().canister_ids;
    baseline
        .pocket_ic()
        .uninstall_canister(canister_ids[0], None)
        .await
        .expect("test should mutate one captured canister");
    drop(baseline);

    let (mut baseline, outcome) = pool.acquire().await.expect("baseline should restore");
    let timings = outcome.timings;
    assert!(timings.build().is_none());
    assert!(timings.restore().is_some());
    assert!(timings.reset().is_some());
    assert!(timings.readiness().is_some());
    assert!(timings.validation().is_some());
    assert!(matches!(
        &outcome.disposition,
        BaselinePoolDisposition::Restored
    ));
    assert_eq!(controls.built_validations.load(Ordering::SeqCst), 1);
    assert_eq!(controls.restored_validations.load(Ordering::SeqCst), 1);
    assert_eq!(baseline.pocket_ic().instance_id, first_instance);
    for canister_id in canister_ids {
        assert!(
            baseline
                .pocket_ic()
                .canister_status(canister_id, None)
                .await
                .expect("restored canister status should remain readable")
                .module_hash
                .is_some(),
            "snapshot restore should recover every installed module",
        );
    }
    baseline.invalidate(RebuildReason::ExplicitLeaseInvalidation);
    baseline.invalidate(RebuildReason::RecipeClassified {
        code: "caller-selected".into(),
    });
    drop(baseline);

    let (baseline, outcome) = pool
        .acquire()
        .await
        .expect("invalid slot should rebuild once");
    assert!(matches!(
        outcome.disposition,
        BaselinePoolDisposition::Rebuilt(RebuildReason::RecipeClassified { code }) if code == "caller-selected"
    ));
    assert_eq!(controls.builds.load(Ordering::SeqCst), 2);
    assert_eq!(controls.built_validations.load(Ordering::SeqCst), 2);
    drop(baseline);
}

#[tokio::test(flavor = "multi_thread", worker_threads = 2)]
async fn cached_baseline_rejects_an_empty_capture() {
    assert!(matches!(
        CachedPocketIcBaseline::capture(support::pocket_ic().await, [], ()).await,
        Err(ControllerSnapshotError::EmptyCanisterSet),
    ));
}

#[tokio::test(flavor = "multi_thread", worker_threads = 2)]
async fn fatal_restore_discards_the_slot_and_finalizes_failure_timings() {
    let controls = RecipeControls::default();
    let pool = CachedPocketIcBaselinePool::new(
        NonZeroUsize::new(1).unwrap(),
        TwoCanisterRecipe::new(controls.clone()),
    );
    drop(pool.acquire().await.expect("first baseline should build").0);
    controls.fail_next_restore.store(true, Ordering::SeqCst);
    controls.fatal_restore.store(true, Ordering::SeqCst);
    let Err(error) = pool.acquire().await else {
        panic!("fatal restore failure must be returned without recovery");
    };
    assert!(matches!(&error, BaselinePoolError::Preparation { .. }));
    let timings = error.timings();
    assert!(timings.restore().is_some());
    assert!(timings.stale_teardown().is_some());
    assert!(timings.build().is_none());
    assert!(timings.total() >= timings.restore().unwrap());
    assert_eq!(controls.builds.load(Ordering::SeqCst), 1);

    let (baseline, outcome) = pool
        .acquire()
        .await
        .expect("discarded slot should build fresh");
    assert!(matches!(
        outcome.disposition,
        BaselinePoolDisposition::Built
    ));
    assert_eq!(controls.builds.load(Ordering::SeqCst), 2);
    drop(baseline);
}

#[tokio::test(flavor = "multi_thread", worker_threads = 2)]
async fn reused_slot_failure_rebuilds_once_and_preserves_a_failed_recovery() {
    let controls = RecipeControls::default();
    let pool = CachedPocketIcBaselinePool::new(
        NonZeroUsize::new(1).unwrap(),
        TwoCanisterRecipe::new(controls.clone()),
    );
    drop(pool.acquire().await.expect("first baseline should build").0);

    controls.fail_next_restore.store(true, Ordering::SeqCst);
    let (baseline, outcome) = pool
        .acquire()
        .await
        .expect("restore failure should rebuild");
    assert!(matches!(
        outcome.disposition,
        BaselinePoolDisposition::Rebuilt(RebuildReason::SnapshotRestoreFailure)
    ));
    drop(baseline);

    controls.fail_next_restore.store(true, Ordering::SeqCst);
    controls.fail_next_build.store(true, Ordering::SeqCst);
    let Err(error) = pool.acquire().await else {
        panic!("a failed restore plus failed rebuild must be returned");
    };
    let timings = error.timings();
    assert!(timings.restore().is_some());
    assert!(timings.stale_teardown().is_some());
    assert!(timings.build().is_some());
    assert!(timings.total() >= timings.restore().unwrap());
    assert!(matches!(
        &error,
        BaselinePoolError::RecoveryFailed {
            original,
            rebuild,
            ..
        } if matches!(
            **original,
            BaselinePoolPreparationError::Recipe { .. }
        ) && matches!(
            **rebuild,
            BaselinePoolPreparationError::Recipe { .. }
        )
    ));
}

#[tokio::test(flavor = "multi_thread", worker_threads = 2)]
async fn canonical_restore_failure_preserves_the_snapshot_cause_and_failed_rebuild() {
    let controls = RecipeControls::default();
    let pool = CachedPocketIcBaselinePool::new(
        NonZeroUsize::new(1).unwrap(),
        TwoCanisterRecipe::new(controls.clone()),
    );
    let (baseline, _) = pool.acquire().await.unwrap();
    let canister = baseline.metadata().canister_ids[0];
    baseline
        .pocket_ic()
        .stop_canister(canister, None)
        .await
        .unwrap();
    let snapshots = baseline
        .pocket_ic()
        .list_canister_snapshots(canister, None)
        .await
        .unwrap();
    baseline
        .pocket_ic()
        .delete_canister_snapshot(canister, None, snapshots[0].id.clone())
        .await
        .unwrap();
    drop(baseline);
    controls.fail_next_build.store(true, Ordering::SeqCst);
    let Err(BaselinePoolError::RecoveryFailed {
        original,
        rebuild,
        timings,
    }) = pool.acquire().await
    else {
        panic!("failed canonical restore and failed recovery must both be retained");
    };
    assert!(matches!(*original, BaselinePoolPreparationError::Restore(
        ControllerSnapshotError::RestoreFailed { canister_id, .. }
    ) if canister_id == canister));
    assert!(matches!(
        *rebuild,
        BaselinePoolPreparationError::Recipe {
            stage: BaselinePreparationStage::Build,
            source: TestRecipeError::Synthetic(_),
        }
    ));
    assert!(timings.restore().is_some());
    assert!(timings.stale_teardown().is_some());
    assert!(timings.build().is_some());
}

#[tokio::test(flavor = "multi_thread", worker_threads = 2)]
async fn failed_initial_build_reports_partial_timings() {
    let controls = RecipeControls::default();
    controls.fail_next_build.store(true, Ordering::SeqCst);
    let pool = CachedPocketIcBaselinePool::new(
        NonZeroUsize::new(1).unwrap(),
        TwoCanisterRecipe::new(controls),
    );

    let Err(error) = pool.acquire().await else {
        panic!("the requested initial build failure must be returned");
    };
    let timings = error.timings();
    assert!(timings.build().is_some());
    assert!(timings.restore().is_none());
    assert!(timings.validation().is_none());
    assert!(timings.total() >= timings.build().unwrap());
    assert!(matches!(
        error,
        BaselinePoolError::Preparation {
            error: BaselinePoolPreparationError::Recipe {
                stage: BaselinePreparationStage::Build,
                ..
            },
            ..
        }
    ));
}

#[tokio::test(flavor = "multi_thread", worker_threads = 2)]
async fn incomplete_or_mismatched_reset_evidence_rebuilds_before_warm_validation() {
    let controls = RecipeControls::default();
    let pool = guarded_pool(controls.clone(), GuardedDomain::ExtraCanisters);
    drop(
        pool.acquire()
            .await
            .expect("initial baseline should build")
            .0,
    );
    for fault in [
        &controls.omit_next_coverage,
        &controls.mismatch_next_coverage,
    ] {
        let builds = controls.builds.load(Ordering::SeqCst);
        let validations = controls.restored_validations.load(Ordering::SeqCst);
        fault.store(true, Ordering::SeqCst);
        let (baseline, outcome) = pool
            .acquire()
            .await
            .expect("invalid reset evidence should rebuild");
        assert_eq!(
            outcome.disposition,
            BaselinePoolDisposition::Rebuilt(RebuildReason::ResetCoverageMismatch)
        );
        assert_eq!(
            controls.restored_validations.load(Ordering::SeqCst),
            validations
        );
        assert_eq!(controls.builds.load(Ordering::SeqCst), builds + 1);
        drop(baseline);
        let (baseline, outcome) = pool
            .acquire()
            .await
            .expect("subsequent evidence should admit warm reuse");
        assert_eq!(outcome.disposition, BaselinePoolDisposition::Restored);
        assert_eq!(
            controls.restored_validations.load(Ordering::SeqCst),
            validations + 1
        );
        drop(baseline);
    }
}

#[tokio::test(flavor = "multi_thread", worker_threads = 2)]
async fn reset_readiness_and_validation_failures_have_distinct_rebuild_reasons() {
    let controls = RecipeControls::default();
    let pool = CachedPocketIcBaselinePool::new(
        NonZeroUsize::new(1).unwrap(),
        TwoCanisterRecipe::new(controls.clone()),
    );
    drop(pool.acquire().await.expect("first baseline should build").0);

    for (flag, expected) in [
        (&controls.fail_next_reset, RebuildReason::ResetFailure),
        (
            &controls.fail_next_readiness,
            RebuildReason::ReadinessFailure,
        ),
        (
            &controls.fail_next_validation,
            RebuildReason::InvariantValidationFailure,
        ),
    ] {
        flag.store(true, Ordering::SeqCst);
        let (baseline, outcome) = pool
            .acquire()
            .await
            .expect("classified preparation failure should rebuild once");
        assert!(matches!(
            outcome.disposition,
            BaselinePoolDisposition::Rebuilt(reason) if reason == expected
        ));
        drop(baseline);
    }

    assert_eq!(controls.builds.load(Ordering::SeqCst), 4);
}

#[tokio::test(flavor = "multi_thread", worker_threads = 2)]
#[ignore = "launches and kills a dedicated PocketIC server; run explicitly in isolation"]
async fn killed_dedicated_server_rebuilds_on_a_fresh_server() {
    let server = PocketIcStartupConfig::Spawn {
        server_binary: (std::env::var_os("POCKET_IC_BIN")
            .expect("prepare POCKET_IC_BIN for the dedicated server probe"))
        .into(),
        timeout: OPERATION_TIMEOUT,
        server_hard_ttl: Some(Duration::from_secs(60)),
        server_idle_ttl: None,
        server_output_files: None,
    }
    .start_managed_server()
    .expect("dedicated server should start");

    let controls = RecipeControls::default();
    *controls
        .first_server_url
        .lock()
        .unwrap_or_else(std::sync::PoisonError::into_inner) = Some(server.url().to_owned());
    let pool = CachedPocketIcBaselinePool::new(
        NonZeroUsize::new(1).unwrap(),
        TwoCanisterRecipe::new(controls.clone()),
    );
    drop(
        pool.acquire()
            .await
            .expect("dedicated-server baseline should build")
            .0,
    );

    drop(server);

    let (baseline, outcome) = pool
        .acquire()
        .await
        .expect("dead dedicated transport should rebuild on the default server");
    assert!(matches!(
        outcome.disposition,
        BaselinePoolDisposition::Rebuilt(RebuildReason::DeadPocketIcTransport)
    ));
    assert_eq!(controls.builds.load(Ordering::SeqCst), 2);
    drop(baseline);
}

#[tokio::test(flavor = "multi_thread", worker_threads = 2)]
async fn guarded_recipe_rebuilds_after_time_cycle_and_extra_canister_mutations() {
    {
        let controls = RecipeControls::default();
        let pool = guarded_pool(controls.clone(), GuardedDomain::Time);
        let (baseline, _) = pool
            .acquire()
            .await
            .expect("time-guarded baseline should build");
        baseline
            .pocket_ic()
            .advance_time(Duration::from_secs(1))
            .await
            .unwrap();
        drop(baseline);

        let (baseline, outcome) = pool
            .acquire()
            .await
            .expect("time mutation should rebuild the guarded slot");
        assert_rebuilt_for_domain(&outcome, "pocket-ic-time");
        assert_eq!(controls.builds.load(Ordering::SeqCst), 2);
        drop(baseline);
    }

    {
        let controls = RecipeControls::default();
        let pool = guarded_pool(controls.clone(), GuardedDomain::Cycles);
        let (baseline, _) = pool
            .acquire()
            .await
            .expect("cycle-guarded baseline should build");
        let canister_id = baseline.metadata().canister_ids[0];
        let _ = baseline
            .pocket_ic()
            .add_cycles(canister_id, 1_000_000)
            .await
            .unwrap();
        drop(baseline);

        let (baseline, outcome) = pool
            .acquire()
            .await
            .expect("cycle mutation should rebuild the guarded slot");
        assert_rebuilt_for_domain(&outcome, "canister-cycles");
        assert_eq!(controls.builds.load(Ordering::SeqCst), 2);
        drop(baseline);
    }

    {
        let controls = RecipeControls::default();
        let pool = guarded_pool(controls.clone(), GuardedDomain::ExtraCanisters);
        let (baseline, _) = pool
            .acquire()
            .await
            .expect("extra-canister-guarded baseline should build");
        let extra_canister = baseline.pocket_ic().create_canister().await.unwrap();
        *controls
            .tracked_extra_canister
            .lock()
            .unwrap_or_else(std::sync::PoisonError::into_inner) = Some(extra_canister);
        drop(baseline);

        let (baseline, outcome) = pool
            .acquire()
            .await
            .expect("extra canister should rebuild the guarded slot");
        assert_rebuilt_for_domain(&outcome, "extra-canister");
        assert_eq!(controls.builds.load(Ordering::SeqCst), 2);
        drop(baseline);
    }
}

#[tokio::test(flavor = "multi_thread", worker_threads = 2)]
async fn panic_while_leased_invalidates_without_hiding_the_panic() {
    let controls = RecipeControls::default();
    let pool = CachedPocketIcBaselinePool::new(
        NonZeroUsize::new(1).unwrap(),
        TwoCanisterRecipe::new(controls.clone()),
    );
    drop(pool.acquire().await.expect("first baseline should build").0);

    let panic = support::catch_async_panic(async || {
        let (_baseline, _outcome) = pool
            .acquire()
            .await
            .expect("baseline should restore before panic");
        panic!("synthetic test panic");
    });
    assert!(panic.is_err(), "the caller panic must keep unwinding");

    let (baseline, outcome) = pool
        .acquire()
        .await
        .expect("slot invalidated by unwind should rebuild");
    assert!(matches!(
        outcome.disposition,
        BaselinePoolDisposition::Rebuilt(RebuildReason::UnwindWhileLeased)
    ));
    assert_eq!(controls.builds.load(Ordering::SeqCst), 2);
    drop(baseline);
}

#[tokio::test(flavor = "multi_thread", worker_threads = 2)]
async fn recipe_hook_panic_invalidates_without_becoming_a_cache_miss() {
    let controls = RecipeControls::default();
    let pool = CachedPocketIcBaselinePool::new(
        NonZeroUsize::new(1).unwrap(),
        TwoCanisterRecipe::new(controls.clone()),
    );
    drop(pool.acquire().await.expect("first baseline should build").0);

    controls.panic_next_restore.store(true, Ordering::SeqCst);
    let panic = support::catch_async_panic(async || {
        let _ = pool.acquire().await;
    });
    assert!(panic.is_err(), "the recipe panic must keep unwinding");
    assert_eq!(
        controls.builds.load(Ordering::SeqCst),
        1,
        "the panicking acquisition must not silently rebuild",
    );

    let (baseline, outcome) = pool
        .acquire()
        .await
        .expect("slot invalidated by recipe panic should rebuild later");
    assert!(matches!(
        outcome.disposition,
        BaselinePoolDisposition::Rebuilt(RebuildReason::UnwindWhileLeased)
    ));
    assert_eq!(controls.builds.load(Ordering::SeqCst), 2);
    drop(baseline);
}

#[tokio::test(flavor = "multi_thread", worker_threads = 2)]
async fn runtime_capacity_allows_two_independent_baseline_leases() {
    let pool = Arc::new(CachedPocketIcBaselinePool::new(
        NonZeroUsize::new(2).unwrap(),
        TwoCanisterRecipe::new(RecipeControls::default()),
    ));
    let (first, first_outcome) = pool.acquire().await.expect("first slot should build");
    assert!(matches!(
        first_outcome.disposition,
        BaselinePoolDisposition::Built
    ));

    let worker_pool = Arc::clone(&pool);
    let (acquired_tx, acquired_rx) = mpsc::channel();
    let worker = support::spawn_async(async move || {
        let (second, outcome) = worker_pool
            .acquire()
            .await
            .expect("second slot should build");
        acquired_tx
            .send((
                (
                    second.pocket_ic().get_server_url(),
                    second.pocket_ic().instance_id,
                ),
                outcome,
            ))
            .expect("capacity result receiver should remain live");
    });

    let (second_instance, second_outcome) = acquired_rx
        .recv_timeout(OPERATION_TIMEOUT)
        .expect("capacity two should not wait for the first lease");
    assert!(matches!(
        second_outcome.disposition,
        BaselinePoolDisposition::Built
    ));
    assert_ne!(
        (
            first.pocket_ic().get_server_url(),
            first.pocket_ic().instance_id
        ),
        second_instance
    );
    drop(first);
    worker.join().expect("capacity worker should exit cleanly");
}

#[tokio::test(flavor = "multi_thread", worker_threads = 2)]
async fn capacity_one_reports_time_waiting_for_the_held_slot() {
    let pool = Arc::new(CachedPocketIcBaselinePool::new(
        NonZeroUsize::new(1).unwrap(),
        TwoCanisterRecipe::new(RecipeControls::default()),
    ));
    let (first, _) = pool.acquire().await.expect("first slot should build");

    let worker_pool = Arc::clone(&pool);
    let (attempting_tx, attempting_rx) = mpsc::channel();
    let (acquired_tx, acquired_rx) = mpsc::channel();
    let worker = support::spawn_async(async move || {
        attempting_tx
            .send(())
            .expect("wait-timing coordinator should remain live");
        let (baseline, outcome) = worker_pool
            .acquire()
            .await
            .expect("held slot should restore");
        acquired_tx
            .send((baseline.pocket_ic().instance_id, outcome))
            .expect("wait-timing receiver should remain live");
    });

    attempting_rx
        .recv_timeout(OPERATION_TIMEOUT)
        .expect("worker should begin its acquisition");
    assert!(
        acquired_rx
            .recv_timeout(Duration::from_millis(100))
            .is_err(),
        "capacity one must not release a second lease while the first is held",
    );
    let first_instance = first.pocket_ic().instance_id;
    drop(first);

    let (second_instance, outcome) = acquired_rx
        .recv_timeout(OPERATION_TIMEOUT)
        .expect("waiting acquisition should complete after release");
    assert_eq!(second_instance, first_instance);
    assert!(matches!(
        &outcome.disposition,
        BaselinePoolDisposition::Restored
    ));
    assert!(
        outcome.timings.wait() >= Duration::from_millis(50),
        "reported queue wait should include the observed capacity block",
    );
    worker
        .join()
        .expect("wait-timing worker should exit cleanly");
}

#[tokio::test(flavor = "multi_thread", worker_threads = 2)]
async fn one_slot_survives_one_hundred_consecutive_restores() {
    let controls = RecipeControls::default();
    let pool = CachedPocketIcBaselinePool::new(
        NonZeroUsize::new(1).unwrap(),
        TwoCanisterRecipe::new(controls.clone()),
    );
    drop(
        pool.acquire()
            .await
            .expect("stress baseline should build")
            .0,
    );

    for iteration in 0..100 {
        let (baseline, outcome) = pool
            .acquire()
            .await
            .unwrap_or_else(|error| panic!("stress restore {iteration} should succeed: {error}"));
        assert!(matches!(
            outcome.disposition,
            BaselinePoolDisposition::Restored
        ));

        let canister_id = baseline.metadata().canister_ids[iteration % 2];
        baseline
            .pocket_ic()
            .uninstall_canister(canister_id, None)
            .await
            .unwrap_or_else(|error| panic!("stress mutation {iteration} failed: {error}"));
        drop(baseline);
    }

    assert_eq!(
        controls.builds.load(Ordering::SeqCst),
        1,
        "successful restores should not reconstruct the slot",
    );
}

fn assert_rebuilt_for_domain(outcome: &BaselinePoolOutcome, expected: &str) {
    assert!(
        matches!(
            &outcome.disposition,
            BaselinePoolDisposition::Rebuilt(RebuildReason::RecipeClassified { code }) if code == expected
        ),
        "expected rebuild for {expected}, got {outcome:?}"
    );
}

fn guarded_pool(
    controls: RecipeControls,
    domain: GuardedDomain,
) -> CachedPocketIcBaselinePool<TwoCanisterRecipe> {
    CachedPocketIcBaselinePool::new(
        NonZeroUsize::new(1).unwrap(),
        TwoCanisterRecipe::guarding_domain(controls, domain),
    )
}
