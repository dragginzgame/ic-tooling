//! Build one caller-selected cdylib through Cargo and capture its reported output.
//!
//! Run with WORKSPACE PACKAGE. Exclude source/config/tool writers for the whole
//! invocation. This example owns its shared target lock through capture; all
//! cooperating producers using that target must use the same lock.

use clap::{Arg, Command, value_parser};
use ic_build::{
    artifacts::CapturedArtifact,
    cargo::{
        execution::{
            CargoExecutionPolicy, CargoInvocation, CargoRecipe, execute_cargo, select_cdylib_wasm,
        },
        observation::{CargoMetadataSpec, decode_metadata, schema},
        selection::CargoSelection,
    },
};
use ic_host_process::{
    child::{CleanupPolicy, OwnedChild},
    tool::{OutputLimit, OutputLimits, OutputStream, capture_command},
};
use std::{error::Error, io::Write as _, path::PathBuf};

fn main() -> Result<(), Box<dyn Error>> {
    let arguments = Command::new("wasm_build")
        .about("Build and capture a caller-selected Wasm library")
        .version(env!("CARGO_PKG_VERSION"))
        .arg(
            Arg::new("workspace")
                .required(true)
                .value_parser(value_parser!(PathBuf)),
        )
        .arg(Arg::new("package").required(true))
        .get_matches();
    let workspace = arguments
        .get_one::<PathBuf>("workspace")
        .unwrap()
        .canonicalize()?;
    let requested = arguments.get_one::<String>("package").unwrap();
    let cargo = std::env::var_os("CARGO").unwrap_or_else(|| "cargo".into());
    let mut environment: std::collections::BTreeMap<_, _> = std::env::vars_os().collect();
    environment
        .entry("RUSTC".into())
        .or_insert_with(|| "rustc".into());
    environment.insert(
        "CARGO_TARGET_DIR".into(),
        workspace.join("target/canister-builds").into_os_string(),
    );
    let selection = CargoSelection {
        profile: Some("release".into()),
        locked: true,
        offline: true,
        ..Default::default()
    };
    let mut metadata_command = std::process::Command::new(&cargo);
    metadata_command.env_clear().envs(&environment);
    let mut metadata_command = CargoMetadataSpec {
        manifest_path: &workspace.join("Cargo.toml"),
        include_dependencies: false,
        filter_platform: Some("wasm32-unknown-unknown"),
        locked: true,
        offline: true,
        features: &selection.features,
    }
    .command(metadata_command)?;
    let metadata = capture_command(
        &mut metadata_command,
        OutputLimits {
            stdout: OutputLimit::Terminate(16 * 1024 * 1024),
            stderr: OutputLimit::Terminate(1024 * 1024),
            timeout: None,
        },
    )?
    .require_success()?
    .require_complete()?;
    let metadata = decode_metadata(&metadata.stdout)?;
    let mut matches = metadata
        .packages
        .into_iter()
        .filter(|package| package.name.as_str() == requested);
    let package = matches
        .next()
        .ok_or("package missing from Cargo metadata")?;
    if matches.next().is_some() {
        return Err("ambiguous Cargo package".into());
    }
    let manifest = package.manifest_path.into_std_path_buf();
    let invocation = CargoInvocation {
        cargo_program: cargo,
        working_directory: workspace.clone(),
        environment,
        target: "wasm32-unknown-unknown".into(),
        selection,
        packages: vec![package.name.to_string()],
        manifest_path: Some(workspace.join("Cargo.toml")),
        recipe: CargoRecipe::Library,
        configurations: Vec::new(),
        render_diagnostics: true,
        keep_going: false,
    };
    let lock = ic_host_fs::durable::open_regular_lock_file_with_parents(
        &workspace.join("target/canister-build.lock"),
    )?;
    ic_host_fs::durable::lock_exclusive(&lock)?;
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
        OwnedChild::spawn_direct,
        || false,
        |stream, bytes| {
            if stream == OutputStream::Stderr {
                std::io::stderr().write_all(bytes)
            } else {
                Ok(())
            }
        },
        |message| {
            if let schema::Message::CompilerMessage(diagnostic) = &message
                && let Some(rendered) = &diagnostic.message.rendered
            {
                std::io::stderr().write_all(rendered.as_bytes())?;
            }
            messages.push(message);
            Ok(())
        },
    )?;
    observed.execution?.require_success()?;
    observed.messages?;
    let facts = observed
        .facts?
        .ok_or("successful build omitted tool observations")?;
    let output = select_cdylib_wasm(&messages, &manifest, &invocation.recipe)?;
    let parent = workspace.join("target/canister-captures");
    std::fs::create_dir_all(&parent)?;
    let captured = CapturedArtifact::capture(&output, &parent, MAXIMUM_WASM_BYTES)?;
    drop(lock);
    let bytes = ic_host_fs::read::read_file(captured.path(), MAXIMUM_WASM_BYTES)?;
    println!(
        "target {}; Cargo {}; compiler {}",
        facts.target,
        String::from_utf8_lossy(&facts.cargo_identity).trim(),
        String::from_utf8_lossy(&facts.rustc_identity).trim()
    );
    println!("read {} captured Wasm bytes", bytes.len());
    Ok(())
}

// This example owns its output policy; the library imposes no Wasm byte budget.
const MAXIMUM_WASM_BYTES: usize = 128 * 1024 * 1024;
