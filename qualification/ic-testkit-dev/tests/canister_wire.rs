//! Installed Wire calls, state, caller identity and typed failure qualification.
use ic_build::{
    cargo::execution::{
        CargoExecutionPolicy, CargoInvocation, CargoRecipe, execute_cargo, select_cdylib_wasm,
    },
    cargo::{
        observation::{decode_workspace_root, workspace_root_command},
        selection::CargoSelection,
    },
};
use ic_host_fs::read::read_file;
use ic_host_process::{
    child::{CleanupPolicy, OwnedChild},
    tool::{CompletedExecution, OutputLimit, OutputLimits, OutputStream, capture_command},
};
use ic_testkit::pic::{InstallSpec, StandaloneCanisterFixture};
use ic_testkit_perf_probe::protocol::{
    self, BENCHMARK_ONCE, BENCHMARK_TRAP, INSPECT_STATE, MUTATE_STATE, PING, QUERY_CALLER,
    SEED_STATE, State, UPDATE_CALLER,
};
use ic_testkit_wire::{WireCallError, query_wire, update_wire};
use pocket_ic::CanisterId as Principal;

use std::{io::Write as _, process::Command};
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
    // This test owns the shared incremental target through byte consumption.
    let target = workspace.join("target/pic-wasm");
    let lock = ic_host_fs::durable::open_regular_lock_file_with_parents(
        &workspace.join("target/pic-wasm.lock"),
    )
    .expect("open probe target lock");
    ic_host_fs::durable::lock_exclusive(&lock).expect("lock probe target");
    let mut environment: std::collections::BTreeMap<_, _> = std::env::vars_os().collect();
    environment
        .entry("RUSTC".into())
        .or_insert_with(|| "rustc".into());
    environment.insert("CARGO_TARGET_DIR".into(), target.into_os_string());
    let invocation = CargoInvocation {
        cargo_program: std::env::var_os("CARGO").unwrap_or_else(|| "cargo".into()),
        working_directory: workspace.clone(),
        environment,
        target: "wasm32-unknown-unknown".into(),
        selection: CargoSelection {
            locked: true,
            offline: true,
            ..CargoSelection::default()
        },
        packages: vec![PERF_PROBE_PACKAGE.into()],
        manifest_path: Some(workspace.join("Cargo.toml")),
        recipe: CargoRecipe::Library,
        configurations: Vec::new(),
        render_diagnostics: true,
        keep_going: false,
    };
    let mut messages = Vec::new();
    let observed = execute_cargo(
        &invocation,
        CargoExecutionPolicy {
            output: OutputLimits {
                stdout: OutputLimit::Truncate(0),
                stderr: OutputLimit::Truncate(1024 * 1024),
                timeout: None,
            },
            message_bytes: 16 * 1024 * 1024,
            probe_cleanup: CleanupPolicy::KillAndWait,
        },
        |command| OwnedChild::spawn(command, CleanupPolicy::KillAndWait),
        || false,
        |stream, bytes| {
            if stream == OutputStream::Stderr {
                std::io::stderr().write_all(bytes)
            } else {
                Ok(())
            }
        },
        |message| {
            if let ic_build::cargo::observation::schema::Message::CompilerMessage(diagnostic) =
                &message
                && let Some(rendered) = &diagnostic.message.rendered
            {
                std::io::stderr().write_all(rendered.as_bytes())?;
            }
            messages.push(message);
            Ok(())
        },
    )
    .expect("execute probe Cargo build");
    observed
        .execution
        .expect("capture build process")
        .require_success()
        .expect("build perf probe Wasm");
    observed.messages.expect("complete Cargo protocol");
    observed
        .facts
        .expect("unchanged recipe and tools")
        .expect("successful build facts");
    let wasm_path = select_cdylib_wasm(
        &messages,
        &workspace.join("ic-testkit-perf-probe/Cargo.toml"),
        &invocation.recipe,
    )
    .expect("select exact probe Wasm from Cargo messages");
    let wasm = read_file(&wasm_path, 128 * 1024 * 1024)
        .expect("read bounded Wasm while holding target custody");
    drop(lock);
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
        // Wire 0.20 admits redundant Postcard groups on the actual Wasm server.
        let mut nonminimal =
            ic_wire::native::encode_request(SEED_STATE.id, &(), SEED_STATE.request_limits).unwrap();
        nonminimal.extend_from_slice(&[u8::try_from(size).unwrap() | 0x80, 0]);
        pic.update_call(id, caller, SEED_STATE.name, nonminimal)
            .await
            .expect("nonminimal seed request follows upstream Postcard admission");
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

// The fixture owns the native control frame for this installed Wire contract.
const fn probe_wire_spec(
    method: ic_testkit_perf_probe::protocol::Method,
) -> ic_testkit_wire::WireCallSpec {
    ic_testkit_wire::WireCallSpec {
        method: method.name,
        method_id: method.id,
        request_limits: method.request_limits,
        response_limits: method.response_limits,
    }
}
