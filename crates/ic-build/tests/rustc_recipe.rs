//! Real Cargo flag precedence, dependency wiring and retained compiler evidence.
use ic_build::{
    artifacts::{WasmBuildSpec, build_wasm_canisters_cached, resolve_cargo_build_inputs},
    cargo::{
        environment::ConfigurationLimits,
        rustc_recipe::{CfgConflict, RustcRecipe, RustcRecipeLimits},
        selection::CargoSelection,
    },
};
use ic_host_process::child::OwnedChild;
use serde_json::Value;
use std::{
    collections::BTreeMap,
    fs,
    path::{Path, PathBuf},
    process::Command,
};

const LIMITS: RustcRecipeLimits = RustcRecipeLimits {
    record_bytes: 128 * 1024,
    records: 32,
    total_bytes: 1024 * 1024,
};
const CONFIGURATION: ConfigurationLimits = ConfigurationLimits {
    file_bytes: 1024 * 1024,
    total_bytes: 4 * 1024 * 1024,
    files: 64,
    depth: 16,
};

fn fixture(config: &str, marker: bool) -> PathBuf {
    let root = ic_build::testing::fs::unique_temp_directory("rustc-recipe");
    fs::create_dir_all(root.join(".cargo")).unwrap();
    fs::create_dir_all(root.join("src")).unwrap();
    fs::create_dir_all(root.join("dependency/src")).unwrap();
    fs::write(root.join(".cargo/config.toml"), config).unwrap();
    fs::write(root.join("Cargo.toml"), "[package]\nname='recipe-fixture'\nversion='0.0.0'\nedition='2024'\n[workspace]\n[lib]\ncrate-type=['cdylib']\n[dependencies]\nrecipe-dependency={path='dependency'}\n[profile.release]\nopt-level='z'\nlto='fat'\ncodegen-units=1\n").unwrap();
    fs::write(
        root.join("dependency/Cargo.toml"),
        "[package]\nname='recipe-dependency'\nversion='0.0.0'\nedition='2024'\n",
    )
    .unwrap();
    let guard = "#[cfg(all(target_arch=\"wasm32\",not(cpubits=\"32\")))] compile_error!(\"missing target requirement\");\n#[cfg(all(not(target_arch=\"wasm32\"),cpubits=\"32\"))] compile_error!(\"target cfg leaked to native\");\n";
    fs::write(
        root.join("dependency/src/lib.rs"),
        format!("{guard}\npub fn value()->u32 {{ 7 }}"),
    )
    .unwrap();
    fs::write(root.join("build.rs"), format!("{guard}\nfn main() {{}}")).unwrap();
    fs::write(root.join("src/lib.rs"), format!("{guard}\n{}\n#[unsafe(no_mangle)] pub extern \"C\" fn value()->u32 {{recipe_dependency::value()}}", if marker { "#[cfg(not(caller_marker))] compile_error!(\"caller flags dropped\");" } else { "" })).unwrap();
    assert!(
        Command::new(std::env::var_os("CARGO").unwrap())
            .current_dir(&root)
            .args(["generate-lockfile", "--offline"])
            .status()
            .unwrap()
            .success()
    );
    root
}

fn recipe() -> RustcRecipe {
    RustcRecipe::new(
        Path::new(env!("CARGO_BIN_EXE_ic-build-rustc-wrapper")),
        "wasm32-unknown-unknown",
        BTreeMap::from([("cpubits".into(), "32".into())]),
        "fixture/qualified-generation",
        LIMITS,
    )
    .unwrap()
}

fn spec(root: &Path, cfg: RustcRecipe) -> WasmBuildSpec {
    WasmBuildSpec::new(
        root,
        &root.join("target"),
        &["recipe-fixture"],
        ic_build::testing::cargo::environment(),
        CONFIGURATION,
    )
    .with_isolated_builds()
    .with_rustc_recipe(cfg)
    .with_cargo_selection(CargoSelection {
        locked: true,
        offline: true,
        ..Default::default()
    })
}

fn args(record: &Value) -> Vec<&str> {
    record["arguments"]
        .as_array()
        .unwrap()
        .iter()
        .map(|arg| arg.as_str().unwrap())
        .collect()
}

type FlagCase = (
    &'static str,
    Option<&'static str>,
    Option<&'static str>,
    bool,
);
const FLAG_CASES: &[FlagCase] = &[
    ("", None, None, false),
    (
        "[build]\nrustflags=['--cfg=caller_marker']\n",
        None,
        None,
        true,
    ),
    (
        "[target.wasm32-unknown-unknown]\nrustflags=['--cfg=caller_marker']\n",
        None,
        None,
        true,
    ),
    (
        "[target.wasm32-unknown-unknown]\nrustflags=['--cfg=config_ignored']\n",
        Some("--cfg=caller_marker"),
        None,
        true,
    ),
    (
        "[build]\nrustflags=['--cfg=config_ignored']\n",
        Some("--cfg=ordinary_ignored"),
        Some("--cfg=caller_marker\x1f--cfg=caller=\"with space\""),
        true,
    ),
    (
        "[target.wasm32-unknown-unknown]\nrustflags=['--cfg=cpubits=\"32\"']\n",
        None,
        None,
        false,
    ),
    (
        "[target.wasm32-unknown-unknown]\nrustflags=['--cfg=cpubits=\"64\"']\n",
        Some("--cfg=cpubits=\"64\""),
        Some(""),
        false,
    ),
];

#[test]
fn actual_cargo_precedence_preserves_flags_and_covers_dependencies_and_native_scripts() {
    for &(config, ordinary, encoded, marker) in FLAG_CASES {
        let root = fixture(config, marker);
        let mut environment = ic_build::testing::cargo::environment();
        if let Some(value) = ordinary {
            environment.insert("RUSTFLAGS".into(), value.into());
        }
        if let Some(value) = encoded {
            environment.insert("CARGO_ENCODED_RUSTFLAGS".into(), value.into());
        }
        // A later full environment replacement must not remove the required policy.
        let spec = spec(&root, recipe()).with_environment(environment);
        let build = build_wasm_canisters_cached(&spec, OwnedChild::spawn_direct).unwrap();
        let records = build
            .record()
            .rustc_recipe_records(LIMITS.records, LIMITS.total_bytes)
            .unwrap();
        let target: Vec<_> = records
            .iter()
            .filter(|record| args(record).contains(&"wasm32-unknown-unknown"))
            .collect();
        assert!(
            target.len() >= 2,
            "root and dependency must both be covered"
        );
        for record in &target {
            let args = args(record);
            assert_eq!(
                args.iter()
                    .filter(|v| **v == "--cfg=cpubits=\"32\"")
                    .count(),
                1
            );
            if marker {
                assert!(args.contains(&"--cfg=caller_marker"));
            }
            if ordinary.is_some() || encoded.is_some() {
                assert!(!args.contains(&"--cfg=config_ignored"));
            }
            if encoded.is_some_and(|v| !v.is_empty()) {
                assert!(!args.contains(&"--cfg=ordinary_ignored"));
                assert!(args.contains(&"--cfg=caller=\"with space\""));
            }
            assert!(
                record["compiler_identity"]
                    .as_str()
                    .unwrap()
                    .contains("rustc ")
            );
        }
        let native: Vec<_> = records
            .iter()
            .filter(|record| args(record).contains(&"build_script_build"))
            .collect();
        assert_ne!(native.len(), 0);
        assert!(
            native
                .iter()
                .all(|record| !args(record).contains(&"--cfg=cpubits=\"32\""))
        );
        assert_reused(&spec, &records);
        println!(
            "compiler recipe evidence: {}",
            build.record().exact_cache_path().display()
        );
    }
}

#[test]
fn effective_profile_and_environment_changes_select_distinct_cache_entries() {
    let root = fixture("", false);
    let mut selected = spec(&root, recipe()).with_cargo_selection(CargoSelection {
        profile: Some("release".into()),
        locked: true,
        offline: true,
        ..Default::default()
    });
    let output = build_wasm_canisters_cached(&selected, OwnedChild::spawn_direct).unwrap();
    let records = output
        .record()
        .rustc_recipe_records(LIMITS.records, LIMITS.total_bytes)
        .unwrap();
    let root = records
        .iter()
        .find(|record| args(record).contains(&"recipe_fixture"))
        .unwrap();
    for setting in ["opt-level=z", "lto=fat", "codegen-units=1"] {
        assert!(
            args(root).windows(2).any(|pair| pair == ["-C", setting]),
            "missing effective compiler setting {setting}"
        );
    }
    let mut environment = selected.environment().clone();
    environment.insert("CARGO_PROFILE_RELEASE_OPT_LEVEL".into(), "3".into());
    selected = selected.with_environment(environment);
    assert_ne!(
        resolve_cargo_build_inputs(&selected).unwrap().fingerprint(),
        output.record().fingerprint()
    );
    let changed = build_wasm_canisters_cached(&selected, OwnedChild::spawn_direct).unwrap();
    assert!(!changed.is_reused());
    assert_ne!(
        changed.record().exact_cache_path(),
        output.record().exact_cache_path()
    );
}

fn assert_reused(spec: &WasmBuildSpec, records: &[Value]) {
    let reused =
        build_wasm_canisters_cached(spec, |_| panic!("warm exact cache must not spawn Cargo"))
            .unwrap();
    assert!(reused.is_reused());
    assert_eq!(
        records,
        reused
            .record()
            .rustc_recipe_records(LIMITS.records, LIMITS.total_bytes)
            .unwrap()
    );
}

#[test]
fn conflicts_refuse_and_only_qualified_overrides_replace_the_conflicting_key() {
    let root = fixture(
        "[target.wasm32-unknown-unknown]\nrustflags=['--cfg', 'cpubits=\"64\"', '--cfg=caller_marker']\n",
        true,
    );
    let rejected = spec(&root, recipe());
    let failure = build_wasm_canisters_cached(&rejected, OwnedChild::spawn_direct).unwrap_err();
    assert!(
        matches!(
            failure,
            ic_build::artifacts::WasmBuildError::CommandFailed { .. }
        ),
        "{failure:?}"
    );
    let cfg = recipe()
        .with_conflict(CfgConflict::Replace {
            qualification: "fixture/override-admission".into(),
        })
        .unwrap();
    let accepted = spec(&root, cfg);
    let output = build_wasm_canisters_cached(&accepted, OwnedChild::spawn_direct).unwrap();
    assert_ne!(
        resolve_cargo_build_inputs(&rejected).unwrap().fingerprint(),
        output.record().fingerprint()
    );
    for record in output
        .record()
        .rustc_recipe_records(LIMITS.records, LIMITS.total_bytes)
        .unwrap()
    {
        assert!(!args(&record).contains(&"cpubits=\"64\""));
        assert_eq!(record["policy"]["override"], "fixture/override-admission");
    }
}

#[test]
fn wrapper_target_and_shared_cache_ambiguity_refuse_before_execution() {
    let root = fixture("", false);
    let base = spec(&root, recipe());
    for bad in [
        base.clone().with_target("x86_64-unknown-linux-gnu"),
        base.clone()
            .with_shared_incremental_target(root.join("shared")),
        base.clone()
            .with_environment([("RUSTC_WRAPPER", "other-wrapper")]),
    ] {
        assert!(bad.cargo_build_command().is_err());
    }
    for configuration in [
        "[build]\nrustc-wrapper='other-wrapper'\n",
        "[build]\nbuild-dir='shared-state'\n",
        "[env]\nIC_BUILD_RUSTC_RECIPE={value='overridden',force=true}\n",
        "[env]\nCARGO_TARGET_DIR={value='shared-state',force=true}\n",
    ] {
        fs::write(root.join(".cargo/config.toml"), configuration).unwrap();
        assert!(resolve_cargo_build_inputs(&base).is_err());
    }
    assert!(
        recipe()
            .with_conflict(CfgConflict::Replace {
                qualification: String::new()
            })
            .is_err()
    );
}
