//! Explicit, pinned II fixture. Consumers own tool admission and the output directory.
use std::{
    error::Error,
    fs::OpenOptions,
    io::{Read as _, Write as _},
    path::{Path, PathBuf},
    process::Command,
    sync::{
        atomic::{AtomicBool, Ordering},
        mpsc,
    },
    time::Duration,
};

use clap::Parser;
use ic_host_artifacts::{
    artifact::hash_reader,
    wasm_text::{self, CompileLimits},
};
use ic_host_fs::read::read_file_no_follow;
use ic_host_process::tool::{OutputLimit, OutputLimits, capture_command};
use ic_testkit::pocket_ic::common::rest::InstanceHttpGatewayConfig;
use ic_testkit::{
    pic::{PocketIcInstance, PocketIcStartupConfig},
    pocket_ic::{PocketIc, PocketIcBuilder},
};
use serde::Deserialize;
use serde_json::json;

type Result<T> = std::result::Result<T, Box<dyn Error + Send + Sync>>;
const PINS: &str = include_str!("../../../../testing/browser/internet-identity/inputs.json");
const TIMEOUT: Duration = Duration::from_secs(60);
static STOP: AtomicBool = AtomicBool::new(false);

#[derive(Parser)]
#[command(about = "Deploy pinned local II without dummy auth; newline/EOF stops the fixture")]
struct Arguments {
    /// Prepared `PocketIC` 16.1.0 binary (admit with ic-testkit-server check first).
    #[arg(long)]
    server: PathBuf,
    /// Prepared didc 0.6.2 executable, admitted using ci/ic-tools.tsv.
    #[arg(long)]
    didc: PathBuf,
    /// Directory populated by the explicit inputs.mjs setup command.
    #[arg(long)]
    inputs: PathBuf,
    /// New, caller-owned evidence directory. Existing directories are refused.
    #[arg(long)]
    output: PathBuf,
}

#[derive(Deserialize)]
struct Pins {
    release: String,
    revision: String,
    assets: Vec<Asset>,
}
#[derive(Deserialize)]
struct Asset {
    name: String,
    sha256: String,
}

extern "C" fn stop(_: libc::c_int) {
    STOP.store(true, Ordering::Relaxed);
}

fn main() -> Result<()> {
    let args = Arguments::parse();
    // Match the other standalone worker: handlers set a lock-free flag only.
    #[expect(
        unsafe_code,
        reason = "standalone fixture owns these signal dispositions"
    )]
    // SAFETY: the handler only writes a static atomic; guards outlive the runtime.
    let _signals = unsafe {
        ic_host_process::signal::SignalActions::install(
            &[libc::SIGTERM, libc::SIGINT, libc::SIGHUP],
            stop,
        )?
    };
    let runtime = tokio::runtime::Builder::new_multi_thread()
        .worker_threads(2)
        .enable_all()
        .build()?;
    runtime.block_on(run(args))
}

fn write_new(path: &Path, bytes: &[u8]) -> Result<()> {
    let mut file = OpenOptions::new().write(true).create_new(true).open(path)?;
    file.write_all(bytes)?;
    Ok(())
}

fn tool(didc: &Path, arguments: &[&str]) -> Result<Vec<u8>> {
    let output = capture_command(
        Command::new(didc).args(arguments),
        OutputLimits {
            stdout: OutputLimit::Terminate(1024 * 1024),
            stderr: OutputLimit::Terminate(64 * 1024),
            timeout: Some(Duration::from_secs(10)),
        },
    )?
    .require_success()?;
    Ok(output.stdout)
}

fn encode(didc: &Path, definitions: &Path, types: &str, value: &str) -> Result<Vec<u8>> {
    // Upstream .did owns the external schema; this fixture supplies only init data.
    let encoded = tool(
        didc,
        &[
            "encode",
            "--defs",
            definitions.to_str().ok_or("non-UTF8 interface path")?,
            "--types",
            types,
            value,
        ],
    )?;
    let hex = std::str::from_utf8(&encoded)?.trim();
    if hex.len() % 2 != 0 {
        return Err("didc returned invalid hex".into());
    }
    (0..hex.len())
        .step_by(2)
        .map(|i| Ok(u8::from_str_radix(&hex[i..i + 2], 16)?))
        .collect()
}

async fn run(args: Arguments) -> Result<()> {
    if tool(&args.server, &["--version"])? != b"pocket-ic-server 16.1.0\n" {
        return Err("expected admitted PocketIC 16.1.0".into());
    }
    if tool(&args.didc, &["--version"])? != b"didc 0.6.2\n" {
        return Err("expected admitted didc 0.6.2".into());
    }
    std::fs::create_dir(&args.output)?;
    let pins: Pins = serde_json::from_str(PINS)?;
    // Read, verify and take private copies before dispatch. Later input rewrites
    // cannot change what didc encodes or what gets installed.
    let mut wasm = Vec::new();
    for asset in &pins.assets {
        let bytes = read_file_no_follow(&args.inputs.join(&asset.name), 32 * 1024 * 1024)?;
        hash_reader(bytes.as_slice(), 32 * 1024 * 1024)?.verify(asset.sha256.parse()?)?;
        if matches!(asset.name.as_str(), "backend.did" | "frontend.did") {
            write_new(&args.output.join(&asset.name), &bytes)?;
        } else {
            wasm.push(bytes);
        }
    }
    let config = PocketIcStartupConfig::Spawn {
        server_binary: args.server,
        timeout: TIMEOUT,
        server_hard_ttl: Some(Duration::from_secs(600)),
        server_idle_ttl: Some(Duration::from_secs(120)),
        server_output_files: Some((
            args.output.join("server.stdout"),
            args.output.join("server.stderr"),
        )),
    };
    let builder = PocketIcBuilder::new(std::time::Duration::from_secs(60))
        .with_nns_subnet()
        .with_ii_subnet()
        .with_application_subnet()
        .with_auto_progress()
        .with_http_gateway(InstanceHttpGatewayConfig {
            ip_addr: Some("127.0.0.1".into()),
            port: Some(0),
            domains: None,
            https_config: None,
            domain_custom_provider_local_file: None,
        });
    let instance = tokio::select! {
        result = config.build_instance(builder) => result?,
        () = interrupted() => return Err("fixture startup interrupted".into()),
    };
    let result = tokio::select! {
        result = serve(&args.output, &args.didc, &pins, &wasm, &instance) => result,
        () = interrupted() => Ok(()),
    };
    // Timeout drops the ownership record and stops/reaps the managed server too.
    let cleanup = tokio::time::timeout(Duration::from_secs(5), instance.close()).await;
    if !matches!(cleanup, Ok(Ok(()))) {
        eprintln!("II fixture cleanup: {cleanup:?}");
    }
    result?;
    cleanup??;
    Ok(())
}

async fn interrupted() {
    while !STOP.load(Ordering::Relaxed) {
        tokio::time::sleep(Duration::from_millis(20)).await;
    }
}

async fn serve(
    output: &Path,
    didc: &Path,
    pins: &Pins,
    wasm: &[Vec<u8>],
    instance: &PocketIcInstance<PocketIc>,
) -> Result<()> {
    let pic = instance.instance();
    let backend = pic.create_canister().await?;
    let frontend = pic.create_canister().await?;
    let caller = pic.create_canister().await?;
    let gateway = pic.url().ok_or("missing local HTTP gateway")?;
    let port = gateway.port().ok_or("gateway needs an explicit port")?;
    let backend_origin = format!("http://{backend}.localhost:{port}");
    let frontend_origin = format!("http://{frontend}.localhost:{port}");
    let backend_args = encode(
        didc,
        &output.join("backend.did"),
        "(opt InternetIdentityInit)",
        "(opt record { is_production = opt false; dummy_auth = opt null; openid_configs = opt vec {}; captcha_config = opt record { max_unsolved_captchas = 50 : nat64; captcha_trigger = variant { Static = variant { CaptchaDisabled } } } })",
    )?;
    let frontend_args = encode(
        didc,
        &output.join("frontend.did"),
        "(InternetIdentityFrontendInit)",
        &format!(
            "(record {{ backend_canister_id = principal \"{backend}\"; backend_origin = \"{backend_origin}\"; related_origins = opt vec {{ \"{frontend_origin}\" }}; fetch_root_key = opt true; dummy_auth = opt null; dev_csp = opt true }})"
        ),
    )?;
    for (id, module, arg) in [
        (backend, wasm[0].clone(), backend_args),
        (frontend, wasm[1].clone(), frontend_args),
    ] {
        pic.add_cycles(id, 10_000_000_000_000).await?;
        pic.install_canister(id, module, arg, None).await?;
    }
    let caller_wasm = wasm_text::compile(
        include_str!("../../../../testing/browser/internet-identity/caller.wat"),
        CompileLimits {
            source_bytes: 4096,
            module_bytes: 4096,
        },
    )?;
    pic.add_cycles(caller, 2_000_000_000_000).await?;
    pic.install_canister(caller, caller_wasm, vec![], None)
        .await?;
    let root_key = pic.root_key().await?.ok_or("missing local root key")?;
    let connection = json!({
        "schema": 1, "release": pins.release, "revision": pins.revision,
        "serverUrl": pic.get_server_url().as_str(), "instanceId": pic.instance_id,
        "gateway": gateway.as_str(), "authorizeUrl": format!("{frontend_origin}/authorize"),
        "backendOrigin": backend_origin, "backendCanisterId": backend.to_text(),
        "frontendCanisterId": frontend.to_text(), "callerCanisterId": caller.to_text(),
        "rootKey": root_key,
        "backendModuleHash": pic.canister_status(backend, None).await?.module_hash,
        "frontendModuleHash": pic.canister_status(frontend, None).await?.module_hash,
        "dummyAuth": false, "fetchRootKey": true, "serverVersion": "16.1.0",
    });
    write_new(
        &output.join("connection.json"),
        &serde_json::to_vec_pretty(&connection)?,
    )?;
    let (send, receive) = mpsc::channel();
    std::thread::spawn(move || {
        let mut byte = [0];
        let _ = std::io::stdin().read(&mut byte);
        let _ = send.send(());
    });
    while !STOP.load(Ordering::Relaxed)
        && matches!(receive.try_recv(), Err(mpsc::TryRecvError::Empty))
    {
        tokio::time::sleep(Duration::from_millis(20)).await;
    }
    Ok(())
}
