//! Native benchmark checks for the private Wasm probe.
use ic_build::{
    artifacts::{WasmBuildSpec, build_wasm_canisters_cached},
    cargo::{
        observation::{decode_workspace_root, workspace_root_command},
        selection::CargoSelection,
    },
};
use ic_host_fs::read::read_file;
use ic_host_process::{
    child::{CleanupPolicy, OwnedChild},
    tool::{CompletedExecution, OutputLimit, OutputLimits, capture_command},
};
use ic_platform::principal::ExternalPrincipal as Principal;
use ic_testkit::{
    benchmark::{
        BenchmarkEventSource, BenchmarkParserConfig, pair_benchmark_spans,
        parse_benchmark_events_from_source,
    },
    pic::{
        InstallSpec, PocketIcCallFailure, StandaloneCanisterFixture, WireCallError, query_wire,
        update_wire,
    },
};
use ic_testkit_dev::probe_wire_spec;
use ic_testkit_perf_probe::protocol::{
    self, BENCHMARK_ONCE, BENCHMARK_TRAP, INSPECT_STATE, MUTATE_STATE, PING, QUERY_CALLER,
    SEED_STATE, State, UPDATE_CALLER,
};

use std::process::Command;
const PERF_PROBE_PACKAGE: &str = "ic-testkit-perf-probe";

fn install_probe() -> StandaloneCanisterFixture {
    let mut workspace = workspace_root_command(
        env!("CARGO_MANIFEST_DIR"),
        Command::new(std::env::var_os("CARGO").unwrap_or_else(|| env!("CARGO").into())),
    )
    .expect("render Cargo workspace discovery");
    let workspace = capture_command(
        &mut workspace,
        OutputLimits {
            stdout: OutputLimit::Terminate(64 * 1024),
            stderr: OutputLimit::Terminate(64 * 1024),
            timeout: None,
        },
    )
    .and_then(CompletedExecution::require_success)
    .and_then(CompletedExecution::require_complete)
    .expect("observe complete Cargo workspace discovery");
    let workspace = decode_workspace_root(&workspace.stdout).expect("resolve Cargo workspace");
    assert!(
        workspace
            .join("crates/ic-testkit-perf-probe/Cargo.toml")
            .is_file(),
        "benchmark qualification requires the workspace perf probe"
    );
    let target_dir = ic_host_fs::owned_directory::OwnedDirectory::new(
        ic_host_fs::directory::create_unique_directory(
            &workspace.join("target"),
            "ic-testkit-perf-probe-target",
            0o700,
            None,
        )
        .expect("create private test directory"),
    );

    let env = std::env::vars_os();
    let spec = WasmBuildSpec::new(
        &workspace,
        target_dir.path(),
        &[PERF_PROBE_PACKAGE],
        env,
        CONFIGURATION_LIMITS,
    )
    .with_cargo_selection(CargoSelection {
        locked: true,
        ..CargoSelection::default()
    })
    .with_shared_incremental_target(workspace.join("target/pic-wasm"));
    let outcome = build_wasm_canisters_cached(&spec, |command| {
        OwnedChild::spawn(command, CleanupPolicy::KillAndWait)
    })
    .expect("build perf probe Wasm");
    let wasm_path = outcome
        .record()
        .artifacts()
        .first()
        .expect("perf probe build returns its retained Wasm");
    let wasm = read_file(wasm_path, 128 * 1024 * 1024).expect("read retained bounded Wasm");
    StandaloneCanisterFixture::try_install(
        ic_testkit::pic::PocketIcStartupConfig::from_env(std::time::Duration::from_secs(30))
            .expect("configure prepared PocketIC")
            .build_instance(
                ic_platform::pocket_ic::PocketIcBuilder::new().with_application_subnet(),
            )
            .expect("construct explicitly configured test instance"),
        InstallSpec {
            wasm,
            init_bytes: vec![],
            cycles: 0,
            install_sender: None,
            label: None,
        },
    )
    .expect("install standalone fixture")
}

#[test]
fn perf_probe_canister_emits_parseable_benchmark_markers() {
    let fixture = install_probe();
    let result = update_wire::<_, u64>(
        fixture.pocket_ic(),
        fixture.canister_id(),
        Principal::anonymous(),
        probe_wire_spec(BENCHMARK_ONCE),
        &(),
    )
    .expect("benchmark_once update call");

    assert_eq!(result, 1_498_500);

    let logs = fixture
        .pocket_ic()
        .fetch_canister_logs(fixture.canister_id(), Principal::anonymous())
        .expect("fetch perf probe logs");
    let log_text = logs
        .iter()
        .map(|record| String::from_utf8_lossy(&record.content))
        .collect::<Vec<_>>()
        .join("\n");
    let parsed = parse_benchmark_events_from_source(
        &log_text,
        &BenchmarkParserConfig::default(),
        BenchmarkEventSource::FetchedLog,
    );
    let spans = pair_benchmark_spans(&parsed.events);

    assert!(
        parsed.malformed_markers.is_empty(),
        "unexpected malformed markers: {:?}",
        parsed.malformed_markers
    );
    assert!(
        spans.invalid_spans.is_empty(),
        "unexpected invalid spans: {:?}",
        spans.invalid_spans
    );
    assert!(
        spans.unpaired_markers.is_empty(),
        "unexpected unpaired markers: {:?}",
        spans.unpaired_markers
    );
    assert!(
        spans
            .spans
            .iter()
            .any(|span| span.span_label == "probe/benchmark_once" && span.delta.instructions > 0),
        "expected a positive instruction delta in spans: {:?}",
        spans.spans
    );

    assert_probe_contract(&fixture);
    assert_probe_trap(&fixture);
    assert_callers(&fixture);
}

fn assert_probe_trap(fixture: &StandaloneCanisterFixture) {
    let error = update_wire::<(), ()>(
        fixture.pocket_ic(),
        fixture.canister_id(),
        Principal::anonymous(),
        probe_wire_spec(BENCHMARK_TRAP),
        &(),
    )
    .expect_err("the probe deliberately traps");
    let WireCallError::Call { source, .. } = error else {
        panic!("structured rejection required")
    };
    assert!(
        matches!(*source, PocketIcCallFailure::Rejected(response) if response.reject_message.contains("intentional perf probe trap"))
    );
    assert_eq!(
        query_wire::<_, ic_wire::Text<4>>(
            fixture.pocket_ic(),
            fixture.canister_id(),
            Principal::anonymous(),
            probe_wire_spec(PING),
            &()
        )
        .unwrap()
        .as_str(),
        "pong"
    );
    let reply = fixture
        .pocket_ic()
        .query_call(
            fixture.canister_id(),
            Principal::anonymous(),
            PING.name,
            ic_wire::native::encode_request(PING.id, &(), PING.request_limits).unwrap(),
        )
        .expect("the same canister remains callable after the trap");
    assert_eq!(
        ic_wire::native::decode_response::<ic_wire::Text<4>>(PING.id, &reply, PING.response_limits)
            .unwrap()
            .as_str(),
        "pong"
    );
    assert!(
        ic_wire::native::decode_response::<State>(
            INSPECT_STATE.id,
            &reply,
            INSPECT_STATE.response_limits
        )
        .is_err()
    );
    let request = ic_wire::native::encode_request(PING.id, &(), PING.request_limits).unwrap();
    assert!(
        ic_wire::native::decode_response::<ic_wire::Text<4>>(
            PING.id,
            &request,
            PING.response_limits
        )
        .is_err()
    );
    for malformed in [
        reply[..reply.len() - 1].to_vec(),
        [reply.as_slice(), &[0]].concat(),
        vec![0; protocol::FRAME_BYTES + 1],
    ] {
        assert!(
            ic_wire::native::decode_response::<ic_wire::Text<4>>(
                PING.id,
                &malformed,
                PING.response_limits
            )
            .is_err()
        );
    }
}

fn assert_probe_contract(fixture: &StandaloneCanisterFixture) {
    let pic = fixture.pocket_ic();
    let id = fixture.canister_id();
    let caller = Principal::anonymous();
    for size in [0_u32, 3] {
        update_wire::<_, ()>(pic, id, caller, probe_wire_spec(SEED_STATE), &size).unwrap();
        let expected = State {
            mutations: 0,
            bytes: u64::from(size),
            first: (size > 0).then_some(42),
        };
        assert_eq!(
            query_wire::<_, State>(pic, id, caller, probe_wire_spec(INSPECT_STATE), &()).unwrap(),
            expected
        );
    }
    update_wire::<_, ()>(pic, id, caller, probe_wire_spec(MUTATE_STATE), &()).unwrap();
    let request =
        ic_wire::native::encode_request(SEED_STATE.id, &99_u32, SEED_STATE.request_limits).unwrap();
    let bad = [
        vec![],
        request[..request.len() - 1].to_vec(),
        ic_wire::native::encode_request(MUTATE_STATE.id, &(), MUTATE_STATE.request_limits).unwrap(),
        ic_wire::native::encode_response(SEED_STATE.id, &(), SEED_STATE.response_limits).unwrap(),
        [request.as_slice(), &[0]].concat(),
        vec![0; protocol::FRAME_BYTES + 1],
    ];
    for bytes in bad {
        let oversized = bytes.len() > protocol::FRAME_BYTES;
        let error = pic
            .update_call(id, caller, SEED_STATE.name, bytes)
            .expect_err("invalid request refuses before seeding");
        if oversized {
            assert!(
                error
                    .reject_message
                    .contains("perf probe Wire request exceeds limit")
            );
        }
        assert_eq!(
            query_wire::<_, State>(pic, id, caller, probe_wire_spec(INSPECT_STATE), &()).unwrap(),
            State {
                mutations: 1,
                bytes: 3,
                first: Some(255)
            }
        );
    }
    pic.query_call(
        id,
        caller,
        INSPECT_STATE.name,
        ic_wire::native::encode_request(PING.id, &(), PING.request_limits).unwrap(),
    )
    .expect_err("query must check its method identity too");
}

fn assert_callers(fixture: &StandaloneCanisterFixture) {
    for caller in [Principal::anonymous(), Principal::from_slice(&[7, 3])] {
        let query = query_wire::<_, ic_wire::Bytes<29>>(
            fixture.pocket_ic(),
            fixture.canister_id(),
            caller,
            probe_wire_spec(QUERY_CALLER),
            &(),
        )
        .unwrap();
        assert_eq!(query.as_slice(), caller.as_slice());
        let update = update_wire::<_, ic_wire::Bytes<29>>(
            fixture.pocket_ic(),
            fixture.canister_id(),
            caller,
            probe_wire_spec(UPDATE_CALLER),
            &(),
        )
        .unwrap();
        assert_eq!(update.as_slice(), caller.as_slice());
    }
}

// This caller admits small configuration trees; libraries do not select this policy.
const CONFIGURATION_LIMITS: ic_build::cargo::environment::ConfigurationLimits =
    ic_build::cargo::environment::ConfigurationLimits {
        file_bytes: 1024 * 1024,
        total_bytes: 16 * 1024 * 1024,
        files: 128,
        depth: 32,
    };
