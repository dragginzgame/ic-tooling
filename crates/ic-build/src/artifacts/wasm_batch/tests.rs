use crate::cargo::selection::CargoSelection;
use crate::testing::cargo::spawn_cargo;
use ic_host_artifacts::batch::BatchLabelError;
#[cfg(unix)]
use ic_host_process::testing::write_executable_script;

use super::{LabeledWasmBuildSpec, WasmBuildBatchProgressEvent, build_wasm_canisters_cached_batch};
use crate::artifacts::{
    CacheBatchEntry, WasmBuildError, WasmBuildPhase, WasmBuildProgressConfig, WasmBuildSpec,
};
use std::{path::Path, time::Duration};

#[cfg(unix)]
use crate::artifacts::support_tests::fake_wasm_build_spec;
use crate::testing::fs::unique_temp_directory;
#[cfg(unix)]
use std::fs;

#[test]
fn empty_independent_batch_succeeds_without_work() {
    let report =
        build_wasm_canisters_cached_batch(&[], None, spawn_cargo).expect("empty labeled batch");
    assert!(report.batch.is_success());
    assert_eq!(report.batch.outcomes().count(), 0);
    let metrics = report.metrics();
    assert_eq!(metrics.specifications(), 0);
    assert_eq!(metrics.succeeded(), 0);
    assert_eq!(metrics.failed(), 0);
    assert_eq!(metrics.built(), 0);
    assert_eq!(metrics.reused(), 0);
}

#[cfg(unix)]
#[test]
fn library_output_validation_survives_batch_resolution() {
    let (root, valid) = fake_wasm_build_spec("batch-library-output-validation");
    fs::create_dir_all(root.join("ordinary/src")).unwrap();
    fs::write(
        root.join("ordinary/Cargo.toml"),
        "[package]\nname = \"ordinary\"\nversion = \"0.0.0\"\nedition = \"2024\"\n",
    )
    .unwrap();
    fs::write(root.join("ordinary/src/lib.rs"), "pub fn ordinary() {}\n").unwrap();
    fs::write(
        root.join("Cargo.toml"),
        "[workspace]\nmembers = [\"fixture\", \"ordinary\"]\nresolver = \"2\"\n",
    )
    .unwrap();
    let invalid = WasmBuildSpec::new(
        &root,
        &root.join("exact"),
        &["ordinary"],
        crate::testing::cargo::environment(),
        CONFIGURATION_LIMITS,
    )
    .with_cargo_program(root.join("cargo.sh"));
    let specs = [
        LabeledWasmBuildSpec::new("ordinary", invalid),
        LabeledWasmBuildSpec::new("canister", valid),
    ];
    let check = |report: super::WasmBuildBatchReport| {
        assert_eq!(report.batch.outcomes().count(), 1);
        assert_eq!(report.batch.failures().count(), 1);
        let failure = report.batch.failures().next().unwrap();
        assert!(std::ptr::eq(
            failure,
            &raw const report.batch.entries()[failure.index()]
        ));
        let success = report.batch.outcomes().next().unwrap();
        assert!(std::ptr::eq(
            success,
            &raw const report.batch.entries()[success.index()]
        ));
        assert_eq!(failure.label(), "ordinary");
        assert!(matches!(
            failure.error().unwrap(),
            WasmBuildError::InvalidSpec { .. }
        ));
        assert_eq!(
            failure.failure_details().unwrap().phase(),
            WasmBuildPhase::InputDiscovery
        );
        assert_eq!(
            failure.failure_details().unwrap().timings().cargo_build,
            None
        );
    };
    check(build_wasm_canisters_cached_batch(&specs, None, spawn_cargo).unwrap());

    fs::remove_dir_all(root).unwrap();
}

#[test]
fn batch_retains_every_indexed_failure() {
    let root = unique_temp_directory("wasm-batch-indexed-failures");
    let target = root.join("target");
    let cargo = root.join("must-not-run-cargo");
    let specs = [
        LabeledWasmBuildSpec::new(
            "root",
            WasmBuildSpec::new(
                &root,
                &target,
                &[],
                crate::testing::cargo::environment(),
                CONFIGURATION_LIMITS,
            )
            .with_cargo_program(&cargo),
        ),
        LabeledWasmBuildSpec::new(
            "worker",
            WasmBuildSpec::new(
                &root,
                &target,
                &["fixture"],
                crate::testing::cargo::environment(),
                CONFIGURATION_LIMITS,
            )
            .with_cargo_selection(CargoSelection {
                profile: Some(String::new()),
                ..Default::default()
            })
            .with_cargo_program(&cargo),
        ),
    ];

    let report =
        build_wasm_canisters_cached_batch(&specs, None, spawn_cargo).expect("valid labeled batch");

    assert!(!report.batch.is_success());
    assert_eq!(report.batch.entries().len(), 2);
    assert_eq!(report.batch.entries()[0].label(), "root");
    assert_eq!(report.batch.entries()[1].label(), "worker");
    assert_eq!(
        report
            .batch
            .failures()
            .map(CacheBatchEntry::label)
            .collect::<Vec<_>>(),
        ["root", "worker"]
    );
    assert!(
        report
            .batch
            .failures()
            .all(|failure| failure.entry_elapsed() <= report.batch.total())
    );
    assert!(report.batch.failures().all(|failure| {
        failure.failure_details().unwrap().phase() == WasmBuildPhase::Specification
            && failure.failure_details().unwrap().timings().total <= failure.entry_elapsed()
    }));
    assert_eq!(report.batch.outcomes().count(), 0);
    let metrics = report.metrics();
    assert_eq!(metrics.specifications(), 2);
    assert_eq!(metrics.succeeded(), 0);
    assert_eq!(metrics.failed(), 2);
    assert_eq!(metrics.built(), 0);
    assert_eq!(metrics.reused(), 0);
    assert!(!target.exists());
    std::fs::remove_dir_all(root).expect("remove indexed failure fixture");
}

#[cfg(unix)]
#[test]
fn batch_failure_retains_partial_metadata_timing() {
    let root = unique_temp_directory("wasm-batch-failure-timings");
    let cargo = root.join("cargo.sh");
    write_executable_script(
        &cargo,
        b"#!/bin/sh\nif [ \"$1\" = \"--version\" ]; then echo 'cargo 1.0.0'; exit 0; fi\nsleep 0.02\necho 'synthetic metadata failure' >&2\nexit 23\n",
    )
    .expect("write executable test script");
    let specs = [LabeledWasmBuildSpec::new(
        "metadata-failure",
        WasmBuildSpec::new(
            &root,
            &root.join("target"),
            &["fixture"],
            crate::testing::cargo::environment(),
            CONFIGURATION_LIMITS,
        )
        .with_cargo_program(&cargo),
    )];

    let report =
        build_wasm_canisters_cached_batch(&specs, None, spawn_cargo).expect("valid labeled batch");
    let failure = report.batch.failures().next().expect("metadata failure");

    assert_eq!(
        failure.failure_details().unwrap().phase(),
        WasmBuildPhase::CargoMetadata
    );
    assert!(
        failure
            .failure_details()
            .unwrap()
            .timings()
            .input_resolution
            .tool_identity
            > Duration::ZERO
    );
    assert!(
        failure
            .failure_details()
            .unwrap()
            .timings()
            .input_resolution
            .cargo_metadata
            > Duration::ZERO
    );
    assert_eq!(
        failure.failure_details().unwrap().timings().cargo_build,
        None
    );
    assert!(failure.failure_details().unwrap().timings().total <= failure.entry_elapsed());

    std::fs::remove_dir_all(root).expect("remove failure timing fixture");
}

#[cfg(unix)]
#[test]
fn malformed_shared_metadata_is_reported_for_every_entry() {
    let (root, spec) = fake_wasm_build_spec("batch-malformed-metadata");
    let cargo = root.join("cargo.sh");
    write_executable_script(
        &cargo,
        b"#!/bin/sh\ncase \"$1\" in\n--version) echo 'cargo 1.0.0' ;;\nmetadata) echo '{}' ;;\n*) exit 97 ;;\nesac\n",
    )
    .expect("write executable test script");
    let specs = [
        LabeledWasmBuildSpec::new("first", spec.clone()),
        LabeledWasmBuildSpec::new("second", spec),
    ];
    let report = build_wasm_canisters_cached_batch(
        &specs,
        Some((WasmBuildProgressConfig::new(), &mut |_| {})),
        spawn_cargo,
    )
    .expect("valid labeled batch");

    assert_eq!(report.batch.outcomes().count(), 0);
    let failures = report.batch.failures().collect::<Vec<_>>();
    assert_eq!(failures.len(), 2);
    for (index, failure) in failures.iter().enumerate() {
        assert_eq!(failure.index(), index);
        assert_eq!(failure.label(), specs[index].label());
        assert_eq!(
            failure.failure_details().unwrap().phase(),
            WasmBuildPhase::CargoMetadata
        );
        assert!(matches!(
            failure.error().unwrap(),
            WasmBuildError::InvalidMetadata { .. }
        ));
        assert_eq!(
            failure.failure_details().unwrap().timings().cargo_build,
            None
        );
    }
    std::fs::remove_dir_all(root).expect("remove malformed metadata fixture");
}

#[test]
fn batch_rejects_invalid_labels_before_progress_or_build_work() {
    let invalid = WasmBuildSpec::new(
        Path::new("."),
        Path::new("target"),
        &[],
        crate::testing::cargo::environment(),
        CONFIGURATION_LIMITS,
    );
    let empty = [LabeledWasmBuildSpec::new("", invalid.clone())];
    let mut progress_events = 0;
    let empty_error = build_wasm_canisters_cached_batch(
        &empty,
        Some((WasmBuildProgressConfig::new(), &mut |_| {
            progress_events += 1;
        })),
        spawn_cargo,
    )
    .expect_err("empty label must reject the batch");
    assert_eq!(empty_error, BatchLabelError::Empty { index: 0 });
    assert_eq!(progress_events, 0);

    let duplicate = [
        LabeledWasmBuildSpec::new("same", invalid.clone()),
        LabeledWasmBuildSpec::new("same", invalid),
    ];
    assert_eq!(
        build_wasm_canisters_cached_batch(&duplicate, None, spawn_cargo,)
            .expect_err("duplicate labels must reject the batch"),
        BatchLabelError::Duplicate {
            label: "same".to_owned(),
            first_index: 0,
            duplicate_index: 1,
        }
    );
}

#[cfg(unix)]
#[test]
fn batch_progress_retains_the_caller_label() {
    let (root, spec) = fake_wasm_build_spec("batch-heartbeat-label");
    crate::artifacts::support_tests::write_fake_wasm_cargo(
        &root,
        &root,
        crate::artifacts::support_tests::FakeWasmCargo {
            before_build: "sleep 0.05",
            ..Default::default()
        },
    );
    let specs = [LabeledWasmBuildSpec::new("root", spec)];
    let mut labels = Vec::new();
    let report = build_wasm_canisters_cached_batch(
        &specs,
        Some((
            WasmBuildProgressConfig::new().with_heartbeat_interval(Duration::from_millis(5)),
            &mut |event| {
                let WasmBuildBatchProgressEvent::BuildProgress { label, .. } = event;
                labels.push(label);
            },
        )),
        spawn_cargo,
    )
    .unwrap();
    assert!(report.batch.is_success());
    assert_ne!(labels, [] as [String; 0]);
    assert!(labels.iter().all(|label| label == "root"));
    drop(report);
    fs::remove_dir_all(root).unwrap();
}

#[cfg(unix)]
#[test]
fn unsafe_shared_target_does_not_fail_other_compatible_batch_entries() {
    let (root, spec) = fake_wasm_build_spec("batch-source-boundary");
    let unsafe_target = root.join("fixture/src/generated-target");
    fs::create_dir_all(&unsafe_target).expect("create unsafe shared target");
    let sentinel = unsafe_target.join("source-sentinel");
    fs::write(&sentinel, b"preserve").expect("write source sentinel");
    let specs = [
        LabeledWasmBuildSpec::new(
            "unsafe",
            spec.clone().with_shared_incremental_target(&unsafe_target),
        ),
        LabeledWasmBuildSpec::new("safe", spec),
    ];
    let report =
        build_wasm_canisters_cached_batch(&specs, None, spawn_cargo).expect("valid labeled batch");
    assert_eq!(report.batch.outcomes().count(), 1);
    let failures = report.batch.failures().collect::<Vec<_>>();
    assert_eq!(failures.len(), 1);
    assert_eq!(failures[0].label(), "unsafe");
    assert_eq!(
        failures[0].failure_details().unwrap().phase(),
        WasmBuildPhase::InputDiscovery
    );
    assert!(matches!(
        failures[0].error().unwrap(),
        WasmBuildError::InvalidSpec { .. }
    ));
    assert_eq!(
        failures[0].failure_details().unwrap().timings().cargo_build,
        None
    );
    assert_eq!(
        fs::read(&sentinel).expect("read preserved source"),
        b"preserve"
    );
    drop(report);
    fs::remove_dir_all(root).expect("remove batch source boundary fixture");
}

#[cfg(unix)]
#[test]
fn missing_or_unhashable_inputs_do_not_fail_other_compatible_batch_entries() {
    for (input, phase) in [
        ("missing-input", WasmBuildPhase::InputDiscovery),
        ("unhashable-input", WasmBuildPhase::ContentHashing),
    ] {
        for invalid_first in [true, false] {
            for observed in [true, false] {
                let (root, spec) = fake_wasm_build_spec("batch-hash-isolation");
                let unhashable = root.join("unhashable-input");
                fs::create_dir(&unhashable).unwrap();
                std::os::unix::fs::symlink("missing-target", unhashable.join("link")).unwrap();
                let good = LabeledWasmBuildSpec::new("good", spec.clone());
                let bad = LabeledWasmBuildSpec::new("bad", spec.with_additional_inputs([input]));
                let specs = if invalid_first {
                    [bad, good]
                } else {
                    [good, bad]
                };
                let report = if observed {
                    build_wasm_canisters_cached_batch(
                        &specs,
                        Some((WasmBuildProgressConfig::new(), &mut |_| {})),
                        spawn_cargo,
                    )
                } else {
                    build_wasm_canisters_cached_batch(&specs, None, spawn_cargo)
                }
                .expect("valid batch labels");
                assert_eq!(report.batch.outcomes().count(), 1);
                let failures = report.batch.failures().collect::<Vec<_>>();
                assert_eq!(failures.len(), 1);
                assert_eq!(failures[0].label(), "bad");
                assert_eq!(failures[0].failure_details().unwrap().phase(), phase);
                for entry in report.batch.into_entries() {
                    let failed = entry.label() == "bad";
                    assert_eq!(entry.result().is_err(), failed);
                    assert_eq!(entry.failure_details().is_some(), failed);
                    let (_, label, result, _) = entry.into_parts();
                    assert_eq!(result.is_err(), label == "bad");
                    if let Err((error, details)) = result {
                        assert!(matches!(error, WasmBuildError::Io { .. }));
                        assert_eq!(details.phase(), phase);
                    }
                }
                std::fs::remove_dir_all(root).expect("remove batch hashing fixture");
            }
        }
    }
}

#[cfg(unix)]
#[test]
fn missing_tools_keep_their_phase_in_silent_and_observed_batches() {
    let (root, spec) = fake_wasm_build_spec("canonical-tool-failure-phases");
    let missing = root.join("missing-tool");
    for (spec, expected) in [
        (
            spec.clone().with_cargo_program(&missing),
            WasmBuildPhase::CargoIdentity,
        ),
        (
            spec.with_environment(crate::testing::cargo::environment_with([(
                "RUSTC",
                missing.as_os_str(),
            )])),
            WasmBuildPhase::RustcIdentity,
        ),
    ] {
        for observed in [false, true] {
            let specs = [LabeledWasmBuildSpec::new("tool-failure", spec.clone())];
            let mut events = Vec::new();
            let mut observe = |event| events.push(event);
            let progress = observed.then_some((
                WasmBuildProgressConfig::new(),
                &mut observe as &mut dyn FnMut(_),
            ));
            let report = build_wasm_canisters_cached_batch(&specs, progress, spawn_cargo).unwrap();
            let entry = &report.batch.entries()[0];
            assert_eq!(entry.failure_details().unwrap().phase(), expected);
            assert!(
                matches!(entry.error(), Some(WasmBuildError::CommandSpawn { phase, .. }) if *phase == expected)
            );
            if !observed {
                assert_eq!(events, [] as [WasmBuildBatchProgressEvent; 0]);
            }
            assert_eq!(report.batch.outcomes().count(), 0);
        }
    }
    fs::remove_dir_all(root).unwrap();
}

// This caller admits small configuration trees; libraries do not select this policy.
const CONFIGURATION_LIMITS: crate::cargo::environment::ConfigurationLimits =
    crate::cargo::environment::ConfigurationLimits {
        file_bytes: 1024 * 1024,
        total_bytes: 16 * 1024 * 1024,
        files: 128,
        depth: 32,
    };
