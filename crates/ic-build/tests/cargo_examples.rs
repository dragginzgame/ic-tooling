//! Actual Cargo library/example selection without a persistent build cache.

use ic_build::{
    artifacts::CapturedArtifact,
    cargo::{
        execution::{
            CargoExecutionPolicy, CargoInvocation, CargoRecipe, execute_cargo, select_cdylib_wasm,
        },
        selection::{CargoFeatureSelection, CargoSelection},
    },
};
use ic_host_process::{
    child::{CleanupPolicy, OwnedChild},
    tool::{OutputLimit, OutputLimits},
};
use std::fs;
fn workspace() -> std::path::PathBuf {
    let root = ic_build::testing::fs::unique_temp_directory("cargo-example");
    let workspace = root.join("workspace");
    ic_build::testing::cargo::write_fixture_package(
        &workspace,
        "example-fixture",
        concat!(
            "[workspace]\n[lib]\ncrate-type=[\"cdylib\"]\n[features]\nprobe=[]\n",
            "[[example]]\nname=\"probe\"\ncrate-type=[\"cdylib\"]\nrequired-features=[\"probe\"]\n",
            "[[example]]\nname=\"other-probe\"\ncrate-type=[\"cdylib\"]\n",
        ),
        "#[unsafe(no_mangle)] pub extern \"C\" fn library() {}\n",
    )
    .unwrap();
    fs::create_dir(workspace.join(".cargo")).unwrap();
    fs::write(
        workspace.join(".cargo/config.toml"),
        "[build]\ntarget-dir=\"configured\"\n",
    )
    .unwrap();
    fs::create_dir_all(workspace.join("examples")).unwrap();
    fs::write(
        workspace.join("examples/probe.rs"),
        "#[unsafe(no_mangle)] pub extern \"C\" fn probe() {}\n",
    )
    .unwrap();
    fs::write(
        workspace.join("examples/other-probe.rs"),
        "#[unsafe(no_mangle)] pub extern \"C\" fn other() {}\n",
    )
    .unwrap();
    ic_build::testing::cargo::generate_fixture_lockfile(&workspace);
    root
}

#[test]
fn named_examples_and_library_use_reported_targets_and_owned_captures() {
    let root = workspace();
    fs::create_dir(root.join("captures")).unwrap();
    let mut retained = Vec::new();
    let mut paths = Vec::new();
    for recipe in [
        CargoRecipe::Example("probe".into()),
        CargoRecipe::Example("other-probe".into()),
        CargoRecipe::Library,
    ] {
        let mut environment = ic_build::testing::cargo::environment();
        environment
            .entry("RUSTC".into())
            .or_insert_with(|| "rustc".into());
        environment.insert(
            "CARGO_TARGET_DIR".into(),
            root.join("cargo").into_os_string(),
        );
        let invocation = CargoInvocation {
            cargo_program: std::env::var_os("CARGO").unwrap_or_else(|| "cargo".into()),
            working_directory: root.join("workspace"),
            environment,
            target: "wasm32-unknown-unknown".into(),
            selection: CargoSelection {
                features: CargoFeatureSelection {
                    features: ["probe".into()].into(),
                    ..Default::default()
                },
                locked: true,
                offline: true,
                ..Default::default()
            },
            packages: vec!["example-fixture".into()],
            manifest_path: Some(root.join("workspace/Cargo.toml")),
            recipe,
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
                message_bytes: 1024 * 1024,
                probe_cleanup: CleanupPolicy::KillAndWait,
            },
            OwnedChild::spawn_direct,
            || false,
            |_, _| Ok(()),
            |message| {
                messages.push(message);
                Ok(())
            },
        )
        .unwrap();
        observed.execution.unwrap().require_success().unwrap();
        observed.messages.unwrap();
        observed.facts.unwrap().unwrap();
        let path = select_cdylib_wasm(
            &messages,
            &root.join("workspace/Cargo.toml"),
            &invocation.recipe,
        )
        .unwrap();
        assert!(path.starts_with(root.join("cargo")));
        let bytes = fs::read(&path).unwrap();
        let captured =
            CapturedArtifact::capture(&path, &root.join("captures"), bytes.len()).unwrap();
        fs::write(&path, b"later mutable producer output").unwrap();
        paths.push(path);
        retained.push((bytes, captured));
    }
    assert_ne!(paths[0], paths[1]);
    assert_ne!(paths[0], paths[2]);
    assert_ne!(paths[1], paths[2]);
    for (bytes, captured) in &retained {
        assert_eq!(fs::read(captured.path()).unwrap(), *bytes);
    }
    drop(retained);
    fs::remove_dir_all(root).unwrap();
}
