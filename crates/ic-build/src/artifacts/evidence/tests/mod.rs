use super::*;
use crate::artifacts::test_support::fake_wasm_build_spec;
use ic_host_artifacts::artifact::hash_reader;

#[test]
fn export_preserves_exact_bytes_and_refuses_limits_and_changed_source() {
    let (root, spec) = fake_wasm_build_spec("wasm-evidence");
    let evidence = WasmBuildEvidence::acquire(
        spec.clone(),
        ic_host_process::child::OwnedChild::spawn_direct,
    )
    .unwrap();
    let record = evidence.record();
    let inputs = resolve_cargo_build_inputs(&spec).unwrap();
    assert_eq!(record.artifacts().len(), 1);
    assert_eq!(record.fingerprint(), inputs.fingerprint());
    assert_eq!(record.recipe_identity(), inputs.recipe_identity());
    assert_eq!(record.facts(), inputs.facts());
    let retained_path = record.artifacts()[0].clone();
    let pruned = crate::artifacts::prune_wasm_build_cache(
        spec.target_dir(),
        crate::artifacts::CacheRetentionLimits::new().with_max_size_bytes(0),
    )
    .unwrap();
    assert_eq!(pruned.entries_removed(), 0);
    assert_eq!(fs::read(&retained_path).unwrap(), b"\0asm\x01\0\0\0");
    let destination = root.join("evidence.wasm");
    fs::write(&destination, b"previous").unwrap();
    assert!(evidence.export_artifact(0, &destination, 7).is_err());
    assert_eq!(fs::read(&destination).unwrap(), b"previous");
    assert!(evidence.export_artifact(1, &destination, 8).is_err());
    let receipt = evidence.export_artifact(0, &destination, 8).unwrap();
    assert_eq!(receipt.build_fingerprint, inputs.fingerprint().to_string());
    assert_eq!(receipt.input_digest, inputs.input_digest().to_string());
    assert_eq!(
        receipt.validation_digest,
        inputs.validation_digest().to_string()
    );
    assert_eq!(fs::read(&destination).unwrap(), b"\0asm\x01\0\0\0");
    assert_eq!(
        receipt.wasm_sha256,
        hash_reader(fs::File::open(&destination).unwrap(), 8)
            .unwrap()
            .sha256
            .to_string()
    );
    assert!(
        evidence
            .export_artifact(0, &evidence.record().artifacts()[0], 8)
            .is_err()
    );
    let reused = WasmBuildEvidence::acquire(
        spec.clone(),
        ic_host_process::child::OwnedChild::spawn_direct,
    )
    .unwrap();
    let second = reused.export_artifact(0, &destination, 8).unwrap();
    assert_eq!(receipt.build_fingerprint, second.build_fingerprint);
    let blocked = root.join("blocked");
    fs::create_dir(&blocked).unwrap();
    let error = reused.export_artifact(0, &blocked, 8).unwrap_err();
    let recovery = error
        .get_ref()
        .unwrap()
        .downcast_ref::<crate::artifacts::CaptureRetentionError>()
        .unwrap();
    assert_eq!(
        fs::read(&recovery.retained_path).unwrap(),
        b"\0asm\x01\0\0\0"
    );
    drop(reused);
    fs::write(root.join("fixture/src/lib.rs"), "pub fn changed() {}\n").unwrap();
    assert_eq!(
        evidence.verify_current().unwrap_err().kind(),
        io::ErrorKind::InvalidData
    );
    assert!(evidence.export_artifact(0, &destination, 8).is_err());
    assert_eq!(fs::read(&destination).unwrap(), b"\0asm\x01\0\0\0");
    drop(evidence);
    let pruned = crate::artifacts::prune_wasm_build_cache(
        spec.target_dir(),
        crate::artifacts::CacheRetentionLimits::new().with_max_size_bytes(0),
    )
    .unwrap();
    assert_eq!(pruned.entries_removed(), 1);
    assert!(!retained_path.exists());
    fs::remove_dir_all(root).unwrap();
}

#[test]
fn raw_input_changes_and_mismatched_recipe_fingerprints_refuse_evidence() {
    let (root, spec) = fake_wasm_build_spec("evidence-raw-inputs");
    let mut evidence = WasmBuildEvidence::acquire(
        spec.clone(),
        ic_host_process::child::OwnedChild::spawn_direct,
    )
    .unwrap();
    let original = fs::read(root.join("Cargo.toml")).unwrap();
    let mut changed = original.clone();
    changed.extend_from_slice(b"\n# unrelated workspace edit\n");
    fs::write(root.join("Cargo.toml"), changed).unwrap();
    let current = resolve_cargo_build_inputs(&spec).unwrap();
    assert_eq!(current.fingerprint(), evidence.fingerprint);
    assert_ne!(current.validation_digest(), evidence.validation_digest);
    assert!(evidence.verify_current().is_err());
    fs::write(root.join("Cargo.toml"), original).unwrap();
    fs::create_dir_all(root.join(".cargo")).unwrap();
    let config = root.join(".cargo/config.toml");
    fs::write(&config, "[build]\njobs = 1\n").unwrap();
    assert_eq!(
        evidence.verify_current().unwrap_err().kind(),
        io::ErrorKind::InvalidData
    );
    fs::remove_file(config).unwrap();
    evidence.verify_current().unwrap();
    let alternate = spec.with_environment(crate::testing::cargo::environment_with([(
        "RECIPE_MARKER",
        "changed",
    )]));
    let other =
        build_wasm_canisters_cached(&alternate, ic_host_process::child::OwnedChild::spawn_direct)
            .unwrap();
    assert_eq!(
        other.record().input_digest(),
        evidence.build.record().input_digest()
    );
    assert_ne!(
        other.record().fingerprint(),
        evidence.build.record().fingerprint()
    );
    evidence.build = other;
    assert!(evidence.verify_current().is_err());
    drop(evidence);
    fs::remove_dir_all(root).unwrap();
}
