//! Integration checks for pocket ic concurrency behavior.
mod support;

use std::{
    sync::{
        Arc,
        mpsc::{self, Receiver, RecvTimeoutError, Sender},
    },
    thread,
    time::{Duration, Instant},
};

use ic_testkit::pic::{
    PocketIcInstance, PocketIcStartupConfig, PocketIcStartupFailure, StandaloneCanisterFixture,
};
use pocket_ic::CanisterId as Principal;
use pocket_ic::PocketIcBuilder;

const READY_TIMEOUT: Duration = Duration::from_secs(60);
const OPERATION_TIMEOUT: Duration = Duration::from_secs(30);

#[derive(Clone, Copy)]
enum Command {
    Initialize,
    Probe,
    Drop,
}

#[derive(Debug)]
enum Event {
    Ready {
        worker: usize,
    },
    Initialized {
        worker: usize,
        canister_id: Principal,
        balance: u128,
    },
    Probed {
        worker: usize,
    },
    Dropped {
        worker: usize,
    },
}

#[tokio::test(flavor = "multi_thread", worker_threads = 2)]
async fn upstream_supports_two_overlapping_instances() {
    assert_two_instances_overlap(async move || {
        support::startup_config()
            .build_instance(
                pocket_ic::PocketIcBuilder::new()
                    .with_max_request_time_ms(Some(300_000))
                    .with_application_subnet(),
            )
            .await
            .expect("build explicitly configured upstream instance")
    });
}

#[tokio::test(flavor = "multi_thread", worker_threads = 2)]
async fn startup_configuration_returns_a_typed_startup_error() {
    let missing_binary = std::env::temp_dir().join(format!(
        "ic-testkit-missing-pocket-ic-{}",
        std::process::id()
    ));
    let result = PocketIcStartupConfig::Spawn {
        server_binary: (&missing_binary).into(),
        timeout: Duration::from_secs(1),
        server_hard_ttl: None,
        server_idle_ttl: None,
        server_output_files: None,
    }
    .build_instance(
        PocketIcBuilder::new()
            .with_max_request_time_ms(Some(300_000))
            .with_application_subnet(),
    )
    .await;

    let error = result.err().unwrap();
    assert!(
        matches!(error.failure(), PocketIcStartupFailure::ServerSpawn { server_binary, .. } if server_binary == &missing_binary)
    );
}

#[tokio::test(flavor = "multi_thread", worker_threads = 2)]
async fn standalone_accepts_a_caller_built_instance_and_preserves_it_in_parts() {
    let caller_built = support::startup_config()
        .build_instance(
            PocketIcBuilder::new()
                .with_max_request_time_ms(Some(300_000))
                .with_application_subnet()
                .with_ii_subnet(),
        )
        .await
        .expect("build explicitly configured custom topology");
    let fixture =
        StandaloneCanisterFixture::install(caller_built, support::empty_install_spec()).await;
    let (owned, canister_id) = fixture.into_parts();
    let pocket_ic = owned.instance();

    pocket_ic
        .canister_status(canister_id, None)
        .await
        .expect("PocketIC returned by into_parts should remain usable");
}

fn assert_two_instances_overlap<F>(build: F)
where
    F: AsyncFn() -> PocketIcInstance + Send + Sync + 'static,
{
    let build = Arc::new(build);
    let (event_tx, event_rx) = mpsc::channel();
    let mut start_txs = Vec::new();
    let mut command_txs = Vec::new();
    let mut workers = Vec::new();

    for worker in 0..2 {
        let (start_tx, start_rx) = mpsc::channel();
        let (command_tx, command_rx) = mpsc::channel();
        let worker_events = event_tx.clone();
        let worker_build = Arc::clone(&build);

        start_txs.push(start_tx);
        command_txs.push(command_tx);
        workers.push(support::spawn_async(async move || {
            run_worker(
                worker,
                worker_build.as_ref(),
                start_rx,
                command_rx,
                worker_events,
            )
            .await;
        }));
    }
    drop(event_tx);

    for start in start_txs {
        start.send(()).expect("worker should await start release");
    }

    let result = coordinate_workers(&event_rx, &command_txs);

    // Closing command channels is the cooperative cancellation path. A ready
    // worker drops its retained instance, allowing a peer blocked by an
    // accidentally reintroduced ownership lock to progress and then exit.
    drop(command_txs);

    let join_result = workers
        .into_iter()
        .map(thread::JoinHandle::join)
        .collect::<Vec<_>>();

    if let Err(message) = result {
        panic!("{message}");
    }
    for joined in join_result {
        joined.expect("PocketIC concurrency worker should exit cleanly");
    }
}

fn coordinate_workers(
    events: &Receiver<Event>,
    commands: &[Sender<Command>],
) -> Result<(), String> {
    let ready_deadline = Instant::now() + READY_TIMEOUT;
    let mut ready = [false; 2];
    while !ready.iter().all(|is_ready| *is_ready) {
        let event = recv_until(
            events,
            ready_deadline,
            "both PocketIC instances to be ready",
        )?;
        let Event::Ready { worker } = event else {
            return Err(format!(
                "expected Ready while acquiring instances, got {event:?}"
            ));
        };
        ready[worker] = true;
    }

    for command in commands {
        command
            .send(Command::Initialize)
            .map_err(|_| "worker exited before initialization".to_string())?;
    }

    let mut states = [None, None];
    while states.iter().any(Option::is_none) {
        match recv_operation(events, "both PocketIC instances to initialize")? {
            Event::Initialized {
                worker,
                canister_id,
                balance,
            } => states[worker] = Some((canister_id, balance)),
            event => return Err(format!("expected Initialized, got {event:?}")),
        }
    }

    let (first_id, first_balance) = states[0].expect("worker zero initialized");
    let (second_id, second_balance) = states[1].expect("worker one initialized");
    if first_id != second_id {
        return Err(format!(
            "expected equivalent fresh instances to allocate the same first canister id, got {first_id} and {second_id}"
        ));
    }
    if first_balance == second_balance {
        return Err("independent instances unexpectedly reported identical test state".to_string());
    }

    commands[0]
        .send(Command::Drop)
        .map_err(|_| "first worker exited before drop".to_string())?;
    match recv_operation(events, "the first PocketIC instance to drop")? {
        Event::Dropped { worker: 0 } => {}
        event => return Err(format!("expected worker zero to drop, got {event:?}")),
    }

    commands[1]
        .send(Command::Probe)
        .map_err(|_| "second worker exited before post-drop probe".to_string())?;
    match recv_operation(events, "the surviving PocketIC instance to respond")? {
        Event::Probed { worker: 1 } => {}
        event => return Err(format!("expected worker one probe, got {event:?}")),
    }

    commands[1]
        .send(Command::Drop)
        .map_err(|_| "second worker exited before drop".to_string())?;
    match recv_operation(events, "the second PocketIC instance to drop")? {
        Event::Dropped { worker: 1 } => Ok(()),
        event => Err(format!("expected worker one to drop, got {event:?}")),
    }
}

async fn run_worker<F>(
    worker: usize,
    build: &F,
    start: Receiver<()>,
    commands: Receiver<Command>,
    events: Sender<Event>,
) where
    F: AsyncFn() -> PocketIcInstance,
{
    if start.recv().is_err() {
        return;
    }

    let owned = build().await;
    let pocket_ic = owned.instance();
    if events.send(Event::Ready { worker }).is_err() {
        return;
    }

    let mut canister_id = None;
    while let Ok(command) = commands.recv() {
        match command {
            Command::Initialize => {
                let id = support::install_empty_canister(pocket_ic).await;
                let balance = pocket_ic
                    .add_cycles(id, (worker as u128 + 1) * 1_000_000)
                    .await;
                canister_id = Some(id);
                if events
                    .send(Event::Initialized {
                        worker,
                        canister_id: id,
                        balance,
                    })
                    .is_err()
                {
                    return;
                }
            }
            Command::Probe => {
                let id = canister_id.expect("worker must initialize before probing");
                pocket_ic
                    .canister_status(id, None)
                    .await
                    .expect("surviving PocketIC instance should remain usable");
                if events.send(Event::Probed { worker }).is_err() {
                    return;
                }
            }
            Command::Drop => {
                owned.close().await;
                let _ = events.send(Event::Dropped { worker });
                return;
            }
        }
    }
}

fn recv_until(events: &Receiver<Event>, deadline: Instant, context: &str) -> Result<Event, String> {
    let remaining = deadline.saturating_duration_since(Instant::now());
    match events.recv_timeout(remaining) {
        Ok(event) => Ok(event),
        Err(RecvTimeoutError::Timeout) => Err(format!("timed out waiting for {context}")),
        Err(RecvTimeoutError::Disconnected) => {
            Err(format!("workers disconnected while waiting for {context}"))
        }
    }
}

fn recv_operation(events: &Receiver<Event>, context: &str) -> Result<Event, String> {
    match events.recv_timeout(OPERATION_TIMEOUT) {
        Ok(event) => Ok(event),
        Err(RecvTimeoutError::Timeout) => Err(format!("timed out waiting for {context}")),
        Err(RecvTimeoutError::Disconnected) => {
            Err(format!("workers disconnected while waiting for {context}"))
        }
    }
}
