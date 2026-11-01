//! Opt-in, real final-Wasm recipe qualification. No publication or tool setup.
use clap::Parser;
use ic_build::{
    artifacts::{
        ArtifactCacheOutcome, ArtifactCachePreparation, ArtifactCacheSpec, WasmBuildEvidence,
        WasmBuildSpec, prepare_artifact_cache, resolve_cargo_build_inputs,
    },
    cargo::{
        environment::ConfigurationLimits,
        rustc_recipe::{RustcRecipe, RustcRecipeLimits},
        selection::CargoSelection,
    },
};
use ic_host_artifacts::wasm::{InspectionLimits, inspect};
use ic_host_process::{
    child::OwnedChild,
    tool::{AdmittedTool, ExecutionContext, OutputLimit, OutputLimits, VersionSpec},
};
use serde_json::json;
use std::{collections::BTreeMap, ffi::OsString, fs, path::PathBuf, time::Duration};

#[derive(Parser)]
struct Args {
    #[arg(long)]
    output: PathBuf,
    #[arg(long)]
    wrapper: PathBuf,
    #[arg(long)]
    optimizer: PathBuf,
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args = Args::parse();
    let workspace = args.output.join("probe");
    let mut environment: BTreeMap<OsString, OsString> = std::env::vars_os().collect();
    environment.remove(&OsString::from("CARGO_TARGET_DIR"));
    let optimizer_environment: Vec<_> = environment.clone().into_iter().collect();
    let context = ExecutionContext {
        current_dir: &args.output,
        environment: &optimizer_environment,
    };
    let limits = OutputLimits {
        stdout: OutputLimit::Terminate(1024 * 1024),
        stderr: OutputLimit::Terminate(1024 * 1024),
        timeout: Some(Duration::from_secs(120)),
    };
    let optimizer = AdmittedTool::admit_version(
        &VersionSpec {
            executable: &args.optimizer,
            executable_bytes: 256 * 1024 * 1024,
            version_arguments: &["--version".into()],
        },
        "wasm-opt version 132 (version_132)",
        &context,
        limits,
    )?;
    let mut rows = vec![];
    for bits in ["64", "32"] {
        let rustc_recipe = RustcRecipe::new(
            &args.wrapper,
            "wasm32-unknown-unknown",
            BTreeMap::from([("cpubits".into(), bits.into())]),
            "Platform 0.16 / k256 0.14 / explicit backend comparison",
            RustcRecipeLimits {
                record_bytes: 256 * 1024,
                records: 256,
                total_bytes: 16 * 1024 * 1024,
            },
        )?;
        let spec = WasmBuildSpec::new(
            &workspace,
            &args.output.join("cache"),
            &["tooling-ecdsa-recipe-probe"],
            environment.clone(),
            ConfigurationLimits {
                file_bytes: 1024 * 1024,
                total_bytes: 16 * 1024 * 1024,
                files: 128,
                depth: 32,
            },
        )
        .with_isolated_builds()
        .with_rustc_recipe(rustc_recipe)
        .with_cargo_selection(CargoSelection {
            profile: Some("release".into()),
            locked: true,
            offline: true,
            ..Default::default()
        });
        let resolved = resolve_cargo_build_inputs(&spec)?;
        let evidence = WasmBuildEvidence::acquire(spec.clone(), OwnedChild::spawn_direct)?;
        let records = evidence
            .record()
            .rustc_recipe_records(256, 16 * 1024 * 1024)?;
        fs::write(
            args.output.join(format!("compiler-{bits}.json")),
            serde_json::to_vec_pretty(&records)?,
        )?;
        let raw_destination = args.output.join(format!("cpubits-{bits}.raw.wasm"));
        let receipt = evidence.export_artifact(0, &raw_destination, 16 * 1024 * 1024)?;
        let input = &evidence.record().artifacts()[0];
        let destination = args.output.join(format!("cpubits-{bits}.optimized.wasm"));
        // Keep the exact ordered optimizer arguments in the existing post-link
        // cache. Only the input/output paths are bound to transaction custody.
        let flags: Vec<OsString> = [
            "-Oz",
            "--strip-debug",
            "--enable-bulk-memory",
            "--enable-sign-ext",
            "--enable-multivalue",
            "--enable-nontrapping-float-to-int",
        ]
        .into_iter()
        .map(Into::into)
        .collect();
        let template: Vec<OsString> = [OsString::from("<input>")]
            .into_iter()
            .chain(flags.clone())
            .chain(["-o".into(), "<output>".into()])
            .collect();
        let post = ArtifactCacheSpec::new(
            &args.output.join("post-cache"),
            "deploy",
            "wasm-opt132/ecdsa-qualification/v1",
        )
        .with_cargo_build_inputs("cargo", &spec, &resolved)
        .with_input("wasm", input)
        .with_input("pipeline", &args.output.join("driver/src/main.rs"))
        .with_input(
            "compiler-recipes",
            &evidence
                .record()
                .exact_cache_path()
                .join(".ic-build-rustc-recipes"),
        )
        .with_tool("optimizer", optimizer.path())
        .with_arguments(&template)
        .with_identity_bytes("optimizer-version", optimizer.version_identity().as_bytes())
        .with_identity_bytes("optimizer-cwd", args.output.as_os_str().as_encoded_bytes())
        .with_environment(optimizer_environment.iter().cloned())
        .with_output("deploy", &destination);
        let optimized = match prepare_artifact_cache(&post)? {
            ArtifactCachePreparation::Reused(record) => ArtifactCacheOutcome::Reused(record),
            ArtifactCachePreparation::Build(transaction) => {
                let output = transaction.output_path("deploy")?;
                let command: Vec<OsString> = [input.into()]
                    .into_iter()
                    .chain(flags)
                    .chain(["-o".into(), output.into()])
                    .collect();
                let result = optimizer.run(&command, &context, limits)?;
                fs::write(
                    args.output.join(format!("optimizer-{bits}.stdout")),
                    &result.stdout,
                )?;
                fs::write(
                    args.output.join(format!("optimizer-{bits}.stderr")),
                    &result.stderr,
                )?;
                result.require_success()?.require_complete()?;
                transaction.commit()?
            }
        };
        evidence.verify_current()?;
        let mut measurements = vec![];
        for (stage, path) in [
            ("compiler", raw_destination),
            (
                "post-link",
                optimized.record().artifacts()[0].path().to_owned(),
            ),
        ] {
            let bytes = ic_host_fs::read::read_file(&path, 16 * 1024 * 1024)?;
            let facts = inspect(
                &bytes,
                InspectionLimits {
                    module_bytes: 16 * 1024 * 1024,
                    sections: 256,
                    exports: 256,
                    custom_sections: 256,
                },
            )?;
            measurements.push(json!({"stage": stage, "module_bytes": facts.raw_bytes, "code_section_bytes": facts.code_section_bytes}));
        }
        rows.push(json!({"cpubits": bits, "cargo_identity": String::from_utf8(evidence.record().facts().cargo_identity.clone())?, "profile": "release", "compiler_receipt": receipt,
            "optimizer_version": optimizer.version_identity(), "optimizer_sha256": optimizer.identity().sha256.to_string(),
            "optimizer_template": template.iter().map(|arg| arg.to_str().expect("literal optimizer arguments")).collect::<Vec<_>>(), "measurements": measurements}));
    }
    fs::write(
        args.output.join("recipes.json"),
        serde_json::to_vec_pretty(&rows)?,
    )?;
    println!("{}", serde_json::to_string_pretty(&rows)?);
    Ok(())
}
