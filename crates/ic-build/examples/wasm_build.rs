//! Build one caller-selected cdylib and consume its bytes while retaining its record.
//!
//! Run with WORKSPACE PACKAGE. The caller must exclude source/config/tool writers
//! for the whole invocation. This example captures the caller's environment;
//! applications choose their own complete environment and process custody.

use clap::{Arg, Command, value_parser};
use ic_build::{
    artifacts::{WasmBuildEvidence, WasmBuildSpec},
    cargo::selection::CargoSelection,
};
use ic_host_process::child::OwnedChild;
use std::{error::Error, path::PathBuf};

fn main() -> Result<(), Box<dyn Error>> {
    let arguments = Command::new("wasm_build")
        .about("Build and read a caller-selected Wasm library")
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
    let package = arguments.get_one::<String>("package").unwrap();
    let environment = std::env::vars_os().filter(|(name, _)| name != "CARGO_TARGET_DIR");
    let spec = WasmBuildSpec::new(
        &workspace,
        &workspace.join("target/canister-builds"),
        &[package.as_str()],
        environment,
        CONFIGURATION_LIMITS,
    )
    .with_cargo_selection(CargoSelection {
        profile: Some("release".into()),
        locked: true,
        ..Default::default()
    });
    let evidence = WasmBuildEvidence::acquire(spec, OwnedChild::spawn_direct)?;
    let record = evidence.record();
    let [artifact] = record.artifacts() else {
        return Err("expected exactly one Wasm artifact".into());
    };
    let facts = record.facts();
    println!(
        "target {}; recipe {}; Cargo {}; compiler {}",
        facts.target,
        record.recipe_identity(),
        String::from_utf8_lossy(&facts.cargo_identity).trim(),
        String::from_utf8_lossy(&facts.rustc_identity).trim(),
    );
    let bytes = std::fs::read(artifact)?;
    evidence.verify_current()?;
    println!("read {} retained Wasm bytes", bytes.len());
    Ok(())
}

// This caller admits small configuration trees; libraries do not select this policy.
const CONFIGURATION_LIMITS: ic_build::cargo::environment::ConfigurationLimits =
    ic_build::cargo::environment::ConfigurationLimits {
        file_bytes: 1024 * 1024,
        total_bytes: 16 * 1024 * 1024,
        files: 128,
        depth: 32,
    };
