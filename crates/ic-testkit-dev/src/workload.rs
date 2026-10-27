//! Workload for the fixture-reuse driver; not an ordinary test.
//! See docs/fixture-reuse-benchmark.md for the workload and measurement boundaries.

use crate::{
    fixture_reuse_protocol::{Mode, WorkerArguments},
    probe_wire_spec,
};
use std::{
    error::Error,
    fs, io,
    num::NonZeroUsize,
    sync::{
        Arc, Barrier,
        atomic::{AtomicBool, Ordering},
    },
    thread,
    time::{Duration, Instant},
};

use crate::fixture_reuse_report::{Phases, Sample, WorkerReport};
use ic_platform::pocket_ic::{PocketIc, PocketIcBuilder};
use ic_platform::principal::ExternalPrincipal as Principal;
use ic_testkit::pic::{
    BaselinePoolDisposition, BaselinePoolTimings, BaselinePreparationStage, CachedPocketIcBaseline,
    CachedPocketIcBaselinePool, CanisterSnapshotTarget, FailureDisposition, PocketIcBaselineRecipe,
    PocketIcInstance, PocketIcStartupConfig, PreparedBaseline, ResetPolicies, ResetReceipt,
    ResetRequirements, SnapshotRestoreFunding, query_wire, update_wire,
};
use ic_testkit_perf_probe::protocol::{InspectState, MutateState, SeedState, State};

type Result<T> = std::result::Result<T, Box<dyn Error + Send + Sync>>;
const TIMEOUT: Duration = Duration::from_secs(30);
static SHUTDOWN: AtomicBool = AtomicBool::new(false);

#[cfg(unix)]
extern "C" fn request_shutdown(_signal: libc::c_int) {
    SHUTDOWN.store(true, Ordering::Relaxed);
}

#[cfg(unix)]
#[expect(
    unsafe_code,
    reason = "scoped standalone-worker dispositions with a lock-free handler"
)]
fn install_shutdown_handler() -> Result<ic_host_process::signal::SignalActions> {
    // SAFETY: the standalone worker owns these dispositions until after its
    // server and in-flight work are dropped. The handler only updates an atomic.
    Ok(unsafe {
        ic_host_process::signal::SignalActions::install(
            &[libc::SIGINT, libc::SIGTERM, libc::SIGHUP],
            request_shutdown,
        )
    }?)
}

fn check_shutdown() -> io::Result<()> {
    if SHUTDOWN.load(Ordering::Relaxed) {
        Err(io::Error::from(io::ErrorKind::Interrupted))
    } else {
        Ok(())
    }
}

#[derive(Clone)]
struct Recipe {
    requirements: ResetRequirements,
    server_url: String,
    wasm: Arc<[u8]>,
    state_bytes: u32,
}

#[derive(Debug, thiserror::Error)]
#[error("{0}")]
struct RecipeError(#[source] Box<dyn Error + Send + Sync>);

fn recipe_error(error: impl Error + Send + Sync + 'static) -> RecipeError {
    RecipeError(Box::new(error))
}

impl Recipe {
    // Fresh construction intentionally does not pay for snapshot capture.
    fn create(
        &self,
    ) -> std::result::Result<(PocketIcInstance<PocketIc>, [Principal; 2]), RecipeError> {
        let pic = PocketIcStartupConfig::Connect {
            server_url: (&self.server_url).into(),
            timeout: TIMEOUT,
        }
        .build_instance(
            PocketIcBuilder::new()
                .with_application_subnet()
                .with_max_request_time_ms(Some(30_000)),
        )
        .map_err(recipe_error)?;
        let ids = [
            pic.instance().create_canister(),
            pic.instance().create_canister(),
        ];
        for id in ids {
            pic.instance().add_cycles(id, 5_000_000_000_000);
            pic.instance()
                .install_canister(id, self.wasm.to_vec(), vec![], None);
            update_wire::<u32, ()>(
                pic.instance(),
                id,
                Principal::anonymous(),
                probe_wire_spec(SeedState::NAME, SeedState::ID),
                &self.state_bytes,
            )
            .map_err(recipe_error)?;
        }
        Ok((pic, ids))
    }

    fn validate_state(
        &self,
        pic: &PocketIc,
        ids: &[Principal; 2],
        mutated: bool,
    ) -> std::result::Result<(), RecipeError> {
        let first = (self.state_bytes > 0).then_some(if mutated { 255 } else { 42 });
        let expected = State {
            mutations: u64::from(mutated),
            bytes: u64::from(self.state_bytes),
            first,
        };
        for id in ids {
            let state = query_wire::<_, State>(
                pic,
                *id,
                Principal::anonymous(),
                probe_wire_spec(InspectState::NAME, InspectState::ID),
                &(),
            )
            .map_err(recipe_error)?;
            if state != expected {
                return Err(recipe_error(io::Error::other(format!(
                    "fixture state mismatch: expected {expected:?}, got {state:?}"
                ))));
            }
        }
        Ok(())
    }

    fn exercise(&self, pic: &PocketIc, ids: &[Principal; 2]) -> Result<()> {
        for id in ids {
            update_wire::<_, ()>(
                pic,
                *id,
                Principal::anonymous(),
                probe_wire_spec(MutateState::NAME, MutateState::ID),
                &(),
            )?;
        }
        self.validate_state(pic, ids, true)?;
        Ok(())
    }
}

impl PocketIcBaselineRecipe for Recipe {
    type Metadata = [Principal; 2];
    type Error = RecipeError;

    fn reset_requirements(&self) -> &ResetRequirements {
        &self.requirements
    }

    fn build(&self) -> std::result::Result<CachedPocketIcBaseline<Self::Metadata>, Self::Error> {
        let (pic, ids) = self.create()?;
        CachedPocketIcBaseline::capture(
            pic,
            ids.map(|id| CanisterSnapshotTarget {
                canister_id: id,
                sender: None,
            }),
            ids,
        )
        .map_err(recipe_error)
    }

    fn reset_non_snapshot_state(
        &self,
        _baseline: &CachedPocketIcBaseline<Self::Metadata>,
    ) -> std::result::Result<ResetReceipt, Self::Error> {
        // This workload mutates only captured canister state. Time and cycle
        // balances are deliberately not asserted to rewind.
        Ok(ResetReceipt::default())
    }

    fn drive_to_readiness(
        &self,
        _baseline: &CachedPocketIcBaseline<Self::Metadata>,
    ) -> std::result::Result<(), Self::Error> {
        Ok(())
    }

    fn validate(
        &self,
        baseline: &CachedPocketIcBaseline<Self::Metadata>,
        _preparation: &PreparedBaseline,
    ) -> std::result::Result<(), Self::Error> {
        self.validate_state(baseline.pocket_ic(), baseline.metadata(), false)?;
        Ok(())
    }

    fn classify_failure(
        &self,
        _stage: BaselinePreparationStage,
        _error: &(dyn std::error::Error + 'static),
    ) -> FailureDisposition {
        // Recovery would conceal a failed measurement. Stop this benchmark.
        FailureDisposition::Fatal
    }
}

impl From<BaselinePoolTimings> for Phases {
    fn from(timings: BaselinePoolTimings) -> Self {
        Self {
            wait_ms: ms(timings.wait()),
            build_ms: timings.build().map(ms),
            restore_ms: timings.restore().map(ms),
            reset_ms: timings.reset().map(ms),
            readiness_ms: timings.readiness().map(ms),
            validation_ms: timings.validation().map(ms),
            acquisition_ms: ms(timings.total()),
        }
    }
}

fn ms(duration: Duration) -> f64 {
    duration.as_secs_f64() * 1000.0
}

fn fresh_sample(recipe: &Recipe, iteration: usize) -> Result<Sample> {
    let started = Instant::now();
    let (pic, ids) = recipe.create()?;
    let build_ms = Some(ms(started.elapsed()));
    let validation_started = Instant::now();
    recipe.validate_state(pic.instance(), &ids, false)?;
    let phases = Phases {
        build_ms,
        validation_ms: Some(ms(validation_started.elapsed())),
        acquisition_ms: ms(started.elapsed()),
        ..Phases::default()
    };
    measure_sample(iteration, started, phases, pic, |owned| {
        recipe.exercise(owned.instance(), &ids)
    })
}

fn pooled_sample(
    pool: &CachedPocketIcBaselinePool<Recipe>,
    recipe: &Recipe,
    iteration: usize,
) -> Result<Sample> {
    let started = Instant::now();
    let (fixture, outcome) = pool.acquire()?;
    if !matches!(outcome.disposition, BaselinePoolDisposition::Restored) {
        return Err(io::Error::other("measured pooled acquisition was not a warm restore").into());
    }
    let phases = outcome.timings.into();
    measure_sample(iteration, started, phases, fixture, |owned| {
        recipe.exercise(owned.pocket_ic(), owned.metadata())
    })
}

// Both modes measure the same body and release boundaries. The caller supplies
// the acquisition phases and retains its distinct fresh/warm admission policy.
fn measure_sample<T>(
    iteration: usize,
    started: Instant,
    phases: Phases,
    fixture: T,
    exercise: impl FnOnce(&T) -> Result<()>,
) -> Result<Sample> {
    let body = Instant::now();
    exercise(&fixture)?;
    let body_ms = ms(body.elapsed());
    let release = Instant::now();
    drop(fixture);
    Ok(Sample {
        iteration,
        phases,
        body_ms,
        release_ms: ms(release.elapsed()),
        total_ms: ms(started.elapsed()),
    })
}

fn run_tasks(
    iterations: NonZeroUsize,
    workers: NonZeroUsize,
    sample: impl Fn(usize) -> Result<Sample> + Sync,
) -> Result<(Vec<Sample>, f64)> {
    let barrier = Barrier::new(workers.get() + 1);
    thread::scope(|scope| -> Result<_> {
        let handles = (0..workers.get())
            .map(|worker| {
                let barrier = &barrier;
                let sample = &sample;
                scope.spawn(move || -> Result<Vec<Sample>> {
                    barrier.wait();
                    (worker..iterations.get())
                        .step_by(workers.get())
                        .map(|iteration| {
                            check_shutdown()?;
                            sample(iteration)
                        })
                        .collect()
                })
            })
            .collect::<Vec<_>>();
        let started = Instant::now();
        barrier.wait();
        let mut samples = Vec::new();
        for handle in handles {
            samples.extend(
                handle
                    .join()
                    .map_err(|_| io::Error::other("benchmark worker panicked"))??,
            );
        }
        Ok((samples, ms(started.elapsed())))
    })
}

pub fn run(arguments: WorkerArguments) -> Result<()> {
    // Install before creating any owned server. A stop arriving earlier may
    // terminate this resource-free process; later stops must unwind ownership.
    #[cfg(unix)]
    let _shutdown_handler = install_shutdown_handler()?;
    check_shutdown()?;
    let WorkerArguments {
        mode,
        capacity,
        iterations,
        workers,
        state_bytes,
        wasm,
        server,
    } = arguments;
    let startup = Instant::now();
    let server = PocketIcStartupConfig::Spawn {
        server_binary: (&server).into(),
        timeout: TIMEOUT,
        server_hard_ttl: None,
        server_idle_ttl: None,
        server_output_files: None,
    }
    .start_managed_server()?;
    let server_startup_ms = ms(startup.elapsed());
    let recipe = Recipe {
        requirements: ResetRequirements {
            funding: SnapshotRestoreFunding::Preserve,
            policies: ResetPolicies::default(),
        },
        server_url: server.url().to_owned(),
        wasm: fs::read(&wasm)?.into(),
        state_bytes,
    };
    let pool = CachedPocketIcBaselinePool::new(capacity, recipe.clone());
    let fixture_started = Instant::now();
    let preparation = Instant::now();
    let mut cold = Vec::new();
    if mode == Mode::Pooled {
        // Hold every lease until all slots exist; do not accidentally measure
        // an initial cold build as a warm acquisition.
        let mut held = Vec::new();
        for _ in 0..capacity.get() {
            check_shutdown()?;
            let (fixture, outcome) = pool.acquire()?;
            if !matches!(outcome.disposition, BaselinePoolDisposition::Built) {
                return Err(io::Error::other("initial slot was not built").into());
            }
            cold.push(Phases::from(outcome.timings));
            // The first timed restore must prove it undoes a real mutation too.
            recipe.exercise(fixture.pocket_ic(), fixture.metadata())?;
            held.push(fixture);
        }
        drop(held);
    }
    let preparation_wall_ms = ms(preparation.elapsed());
    let (mut samples, wall_ms) = match mode {
        Mode::Fresh => run_tasks(iterations, workers, |iteration| {
            fresh_sample(&recipe, iteration)
        }),
        Mode::Pooled => run_tasks(iterations, workers, |iteration| {
            pooled_sample(&pool, &recipe, iteration)
        }),
    }?;
    check_shutdown()?;
    samples.sort_by_key(|sample| sample.iteration);
    let teardown = Instant::now();
    drop(pool);
    let pool_teardown_ms = ms(teardown.elapsed());
    let fixture_wall_ms = ms(fixture_started.elapsed());
    let teardown = Instant::now();
    drop(server);
    let server_teardown_ms = ms(teardown.elapsed());
    let report = WorkerReport {
        mode,
        capacity: if mode == Mode::Fresh {
            workers.get()
        } else {
            capacity.get()
        },
        iterations: iterations.get(),
        workers: workers.get(),
        state_bytes_per_canister: state_bytes,
        wall_ms,
        fixture_wall_ms,
        samples,
        server_startup_ms,
        cold_preparation: cold,
        preparation_wall_ms,
        pool_teardown_ms,
        server_teardown_ms,
    };
    println!("{}", serde_json::to_string(&report)?);
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn recipe_error_contract_preserves_the_erased_producer() {
        let error = recipe_error(io::Error::new(io::ErrorKind::InvalidData, "recipe"));
        assert_eq!(error.to_string(), "recipe");
        let source = error.source().unwrap().downcast_ref::<io::Error>().unwrap();
        assert!(std::ptr::eq(source, error.0.downcast_ref().unwrap()));
        assert_eq!(source.kind(), io::ErrorKind::InvalidData);
    }
}
