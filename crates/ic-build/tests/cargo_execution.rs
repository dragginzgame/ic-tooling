//! Real Cargo messages bind exact outputs and build-script exports before capture.
use ic_build::{
    artifacts::CapturedArtifact,
    cargo::{
        execution::{
            CargoExecutionPolicy, CargoInvocation, CargoRecipe, execute_cargo, select_cdylib_wasm,
        },
        observation::schema::Message,
        selection::CargoSelection,
    },
};
use ic_host_process::{
    child::{CleanupPolicy, OwnedChild},
    tool::{OutputLimit, OutputLimits},
};
use std::{fs, path::Path, process::Command};

#[test]
fn reported_custom_libraries_and_exports_survive_shared_target_collisions() {
    let root = ic_build::testing::fs::unique_temp_directory("cargo-execution");
    fs::create_dir_all(root.join("captures")).unwrap();
    let target = root.join("target");
    let cargo = std::env::var_os("CARGO").unwrap_or_else(|| "cargo".into());
    let mut retained = Vec::new();
    for (workspace, marker) in [("first", 1), ("second", 2)] {
        let directory = root.join(workspace);
        let manifest = write_library_fixture(&directory, marker, &cargo);
        let mut environment = ic_build::testing::cargo::environment();
        environment
            .entry("RUSTC".into())
            .or_insert_with(|| "rustc".into());
        environment.insert("CARGO_TARGET_DIR".into(), target.clone().into_os_string());
        let invocation = CargoInvocation {
            cargo_program: cargo.clone(),
            working_directory: directory.clone(),
            environment,
            target: "wasm32-unknown-unknown".into(),
            selection: CargoSelection {
                locked: true,
                offline: true,
                ..CargoSelection::default()
            },
            packages: Vec::new(),
            manifest_path: Some(manifest.clone()),
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
        let evidence = observed.execution.unwrap();
        assert!(
            evidence.status.success(),
            "{}",
            String::from_utf8_lossy(&evidence.stderr)
        );
        observed.messages.unwrap();
        let facts = observed.facts.unwrap().unwrap();
        assert_eq!(facts.environment, invocation.environment);
        assert_eq!(facts.selection, invocation.selection);
        assert_eq!(facts.target, invocation.target);
        let script = messages
            .iter()
            .find_map(|message| {
                if let Message::BuildScriptExecuted(script) = message {
                    Some(script)
                } else {
                    None
                }
            })
            .unwrap();
        assert!(
            script
                .env
                .iter()
                .any(|(name, value)| name == "CONSUMER_INPUT" && Path::new(value) == directory)
        );
        let path = select_cdylib_wasm(&messages, &manifest, &invocation.recipe).unwrap();
        assert_eq!(path.file_name().unwrap(), "custom_library.wasm");
        let bytes = fs::read(&path).unwrap();
        let capture =
            CapturedArtifact::capture(&path, &root.join("captures"), 128 * 1024 * 1024).unwrap();
        retained.push((path, bytes, capture));
    }
    assert_eq!(retained[0].0, retained[1].0);
    assert_ne!(retained[0].1, retained[1].1);
    for (_, bytes, capture) in &retained {
        assert_eq!(&fs::read(capture.path()).unwrap(), bytes);
    }
    assert_eq!(fs::read(&retained[0].0).unwrap(), retained[1].1);
    drop(retained);
    fs::remove_dir_all(root).unwrap();
}

#[test]
fn warning_policy_preserves_artifacts_and_refuses_fresh_and_cached_rendered_warnings() {
    let root = ic_build::testing::fs::unique_temp_directory("cargo-warning-policy");
    let cargo = std::env::var_os("CARGO").unwrap_or_else(|| "cargo".into());
    for warning in [false, true] {
        for rendered in [false, true] {
            for (warning_level, compiler_deny) in [("warn", false), ("deny", false), ("warn", true)]
            {
                let directory = root.join(format!(
                    "{warning}-{rendered}-{warning_level}-{compiler_deny}"
                ));
                let manifest = write_library_fixture(&directory, 7, &cargo);
                if warning {
                    fs::write(directory.join("src/lib.rs"), "#[unsafe(no_mangle)] pub extern \"C\" fn marker() -> u32 { let warning_marker = 1; 7 }\n").unwrap();
                }
                let mut environment = ic_build::testing::cargo::environment();
                environment.remove(std::ffi::OsStr::new("RUSTFLAGS"));
                environment.remove(std::ffi::OsStr::new("CARGO_ENCODED_RUSTFLAGS"));
                environment
                    .entry("RUSTC".into())
                    .or_insert_with(|| "rustc".into());
                environment.insert(
                    "CARGO_TARGET_DIR".into(),
                    directory.join("target").into_os_string(),
                );
                environment.insert("CARGO_BUILD_WARNINGS".into(), warning_level.into());
                if compiler_deny {
                    environment.insert("RUSTFLAGS".into(), "-D warnings".into());
                }
                let invocation = CargoInvocation {
                    cargo_program: cargo.clone(),
                    working_directory: directory.clone(),
                    environment,
                    target: "wasm32-unknown-unknown".into(),
                    selection: CargoSelection {
                        locked: true,
                        offline: true,
                        ..Default::default()
                    },
                    packages: Vec::new(),
                    manifest_path: Some(manifest.clone()),
                    recipe: CargoRecipe::Library,
                    configurations: Vec::new(),
                    render_diagnostics: rendered,
                    keep_going: false,
                };
                let mut original = None;
                for warm in [false, true] {
                    let mut messages = Vec::new();
                    let observed = execute_cargo(
                        &invocation,
                        CargoExecutionPolicy {
                            output: OutputLimits {
                                stdout: OutputLimit::Truncate(1024 * 1024),
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
                    let execution = observed.execution.unwrap();
                    observed.messages.unwrap();
                    // Cargo 1.100 does not enforce build.warnings=deny for plain
                    // JSON. Compiler-level denial remains strict in either mode.
                    let succeeds =
                        !warning || (!compiler_deny && (!rendered || warning_level == "warn"));
                    assert_eq!(
                        execution.status.success(),
                        succeeds,
                        "{directory:?}, warm={warm}: {}",
                        String::from_utf8_lossy(&execution.stderr)
                    );
                    let facts = observed.facts.unwrap();
                    assert_eq!(facts.is_some(), succeeds);
                    if let Some(facts) = facts {
                        assert_eq!(facts.environment, invocation.environment);
                        assert_eq!(facts.selection, invocation.selection);
                        assert_eq!(facts.target, invocation.target);
                    }
                    let artifact = messages.iter().find_map(|message| match message {
                        Message::CompilerArtifact(artifact)
                            if artifact.target.name == "custom_library" =>
                        {
                            Some(artifact)
                        }
                        _ => None,
                    });
                    if warning && compiler_deny {
                        assert!(artifact.is_none());
                        continue;
                    }
                    let artifact = artifact.unwrap();
                    assert_eq!(artifact.fresh, warm);
                    if warning {
                        if rendered {
                            assert!(
                                String::from_utf8_lossy(&execution.stderr)
                                    .contains("warning_marker")
                            );
                        } else {
                            assert!(messages.iter().any(|message| matches!(message,
                                Message::CompilerMessage(diagnostic) if diagnostic.message.message.contains("warning_marker"))));
                        }
                    }
                    assert!(messages.iter().any(|message| matches!(message,
                        Message::BuildScriptExecuted(script) if script.env.iter().any(|(name, value)| name == "CONSUMER_INPUT" && Path::new(value) == directory))));
                    let selected = select_cdylib_wasm(&messages, &manifest, &invocation.recipe);
                    if !succeeds {
                        // Cargo can emit artifacts before warning policy fails.
                        // Preserve the evidence without admitting those outputs.
                        assert!(selected.is_err());
                        continue;
                    }
                    let path = selected.unwrap();
                    let selected = (
                        artifact.package_id.clone(),
                        artifact.target.clone(),
                        path.clone(),
                        fs::read(path).unwrap(),
                    );
                    if let Some(original) = &original {
                        assert_eq!(&selected, original);
                    } else {
                        original = Some(selected);
                    }
                }
            }
        }
    }
    fs::remove_dir_all(root).unwrap();
}

fn write_library_fixture(
    directory: &Path,
    marker: u32,
    cargo: &std::ffi::OsStr,
) -> std::path::PathBuf {
    fs::create_dir_all(directory.join("src")).unwrap();
    let manifest = directory.join("Cargo.toml");
    fs::write(&manifest, "[workspace]\n[package]\nname='collision'\nversion='0.1.0'\nedition='2024'\n[lib]\nname='custom_library'\ncrate-type=['cdylib']\n").unwrap();
    fs::write(
        directory.join("src/lib.rs"),
        format!("#[unsafe(no_mangle)] pub extern \"C\" fn marker() -> u32 {{ {marker} }}\n"),
    )
    .unwrap();
    fs::write(directory.join("build.rs"), "fn main() { println!(\"cargo:rustc-env=CONSUMER_INPUT={}\", std::env::var(\"CARGO_MANIFEST_DIR\").unwrap()); }\n").unwrap();
    assert!(
        Command::new(cargo)
            .current_dir(directory)
            .args(["generate-lockfile", "--offline"])
            .status()
            .unwrap()
            .success()
    );
    manifest
}
