//! Installed Wire calls, state, caller identity and typed failure qualification.
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
use ic_testkit::pic::{
    InstallSpec, StandaloneCanisterFixture, WireCallError, query_wire, update_wire,
};
use ic_testkit_perf_probe::protocol::{
    self, BENCHMARK_ONCE, BENCHMARK_TRAP, INSPECT_STATE, MUTATE_STATE, PING, QUERY_CALLER,
    SEED_STATE, State, UPDATE_CALLER,
};
use pocket_ic::CanisterId as Principal;

use std::process::Command;
const PERF_PROBE_PACKAGE: &str = "ic-testkit-perf-probe";

async fn install_probe() -> StandaloneCanisterFixture {
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
        workspace.join("ic-testkit-perf-probe/Cargo.toml").is_file(),
        "Wire qualification requires the workspace control canister"
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
    StandaloneCanisterFixture::install(
        ic_testkit::pic::PocketIcStartupConfig::from_env(std::time::Duration::from_secs(30))
            .expect("configure prepared PocketIC")
            .build_instance(
                pocket_ic::PocketIcBuilder::new()
                    .with_max_request_time_ms(Some(300_000))
                    .with_application_subnet(),
            )
            .await
            .expect("construct explicitly configured test instance"),
        InstallSpec {
            wasm,
            init_bytes: vec![],
            cycles: 0,
            install_sender: None,
        },
    )
    .await
}

#[tokio::test]
async fn perf_probe_wire_calls_preserve_state_and_typed_failures() {
    let fixture = install_probe().await;
    let result = update_wire::<_, u64>(
        fixture.pocket_ic(),
        fixture.canister_id(),
        Principal::anonymous(),
        probe_wire_spec(BENCHMARK_ONCE),
        &(),
    )
    .await
    .expect("benchmark_once update call");

    assert_eq!(result, 1_498_500);

    assert_probe_contract(&fixture).await;
    assert_probe_trap(&fixture).await;
    assert_callers(&fixture).await;
}

async fn assert_probe_trap(fixture: &StandaloneCanisterFixture) {
    let error = update_wire::<(), ()>(
        fixture.pocket_ic(),
        fixture.canister_id(),
        Principal::anonymous(),
        probe_wire_spec(BENCHMARK_TRAP),
        &(),
    )
    .await
    .expect_err("the probe deliberately traps");
    let WireCallError::Call { source, .. } = error else {
        panic!("structured rejection required")
    };
    assert!(
        source
            .reject_message
            .contains("intentional perf probe trap")
    );
    assert_eq!(
        query_wire::<_, ic_wire::Text<4>>(
            fixture.pocket_ic(),
            fixture.canister_id(),
            Principal::anonymous(),
            probe_wire_spec(PING),
            &()
        )
        .await
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
        .await
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

async fn assert_probe_contract(fixture: &StandaloneCanisterFixture) {
    let pic = fixture.pocket_ic();
    let id = fixture.canister_id();
    let caller = Principal::anonymous();
    for size in [0_u32, 3] {
        update_wire::<_, ()>(pic, id, caller, probe_wire_spec(SEED_STATE), &size)
            .await
            .unwrap();
        let expected = State {
            mutations: 0,
            bytes: u64::from(size),
            first: (size > 0).then_some(42),
        };
        assert_eq!(
            query_wire::<_, State>(pic, id, caller, probe_wire_spec(INSPECT_STATE), &())
                .await
                .unwrap(),
            expected
        );
    }
    update_wire::<_, ()>(pic, id, caller, probe_wire_spec(MUTATE_STATE), &())
        .await
        .unwrap();
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
            .await
            .expect_err("invalid request refuses before seeding");
        if oversized {
            assert!(
                error
                    .to_string()
                    .contains("perf probe Wire request exceeds limit")
            );
        }
        assert_eq!(
            query_wire::<_, State>(pic, id, caller, probe_wire_spec(INSPECT_STATE), &())
                .await
                .unwrap(),
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
    .await
    .expect_err("query must check its method identity too");
}

async fn assert_callers(fixture: &StandaloneCanisterFixture) {
    for caller in [Principal::anonymous(), Principal::from_slice(&[7, 3])] {
        let query = query_wire::<_, ic_wire::Bytes<29>>(
            fixture.pocket_ic(),
            fixture.canister_id(),
            caller,
            probe_wire_spec(QUERY_CALLER),
            &(),
        )
        .await
        .unwrap();
        assert_eq!(query.as_slice(), caller.as_slice());
        let update = update_wire::<_, ic_wire::Bytes<29>>(
            fixture.pocket_ic(),
            fixture.canister_id(),
            caller,
            probe_wire_spec(UPDATE_CALLER),
            &(),
        )
        .await
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

// The fixture owns the native control frame for this installed Wire contract.
const fn probe_wire_spec(
    method: ic_testkit_perf_probe::protocol::Method,
) -> ic_testkit::pic::WireCallSpec {
    ic_testkit::pic::WireCallSpec {
        method: method.name,
        method_id: method.id,
        request_limits: method.request_limits,
        response_limits: method.response_limits,
    }
}
