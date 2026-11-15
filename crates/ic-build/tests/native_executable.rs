//! Real native Cargo selection, environment preservation and failed-build evidence.
#![cfg(unix)]

use ic_build::cargo::selection::CargoSelection;

use ic_build::cargo::native::{NativeExecutableSpec, NativeTargetKind, build_native_executable};
use ic_host_process::{child::OwnedChild, tool::ToolError};
use std::{cell::Cell, ffi::OsString, fs, process::Command};

#[test]
fn actual_cargo_reports_custom_profile_executable_and_streams_failed_build_diagnostics() {
    let (root, cargo_program) = fixture();
    let target = root.join("output space");
    let calls = Cell::new(0);
    let factory = || {
        calls.set(calls.get() + 1);
        command(&cargo_program, &root, &target)
    };
    let spec = NativeExecutableSpec {
        package: "native-fixture",
        target_name: "worker",
        target_kind: NativeTargetKind::Example,
        selection: CargoSelection {
            profile: Some("custom".into()),
            locked: true,
            ..Default::default()
        },
        max_message_bytes: 1024 * 1024,
    };
    let mut diagnostics = Vec::new();
    let built = build_native_executable(
        &spec,
        factory,
        &mut diagnostics,
        || Ok(()),
        OwnedChild::spawn_direct,
        || false,
    )
    .unwrap();
    assert_eq!(calls.get(), 2);
    assert_eq!(built.target_directory, target);
    assert!(built.executable.starts_with(&target));
    assert!(String::from_utf8_lossy(&diagnostics).contains("never used"));
    assert_eq!(
        Command::new(&built.executable).output().unwrap().stdout,
        b"factory-preserved\n"
    );
    let binary = NativeExecutableSpec {
        target_name: "native-fixture",
        target_kind: NativeTargetKind::Binary,
        ..spec.clone()
    };
    let built = build_native_executable(
        &binary,
        factory,
        &mut diagnostics,
        || Ok(()),
        OwnedChild::spawn_direct,
        || false,
    )
    .unwrap();
    assert_eq!(calls.get(), 4);
    assert_eq!(built.target_directory, target);
    assert_eq!(
        Command::new(&built.executable).output().unwrap().stdout,
        b"binary\n"
    );
    calls.set(0);
    let admissions = Cell::new(0);
    let error = build_native_executable(
        &spec,
        factory,
        &mut diagnostics,
        || {
            admissions.set(admissions.get() + 1);
            if admissions.get() == 2 {
                Err(std::io::ErrorKind::Interrupted.into())
            } else {
                Ok(())
            }
        },
        OwnedChild::spawn_direct,
        || false,
    )
    .unwrap_err();
    let failure = error.downcast_ref::<std::io::Error>().unwrap();
    assert_eq!(failure.kind(), std::io::ErrorKind::Interrupted);
    assert_eq!(calls.get(), 1);
    fs::write(
        root.join("examples/worker.rs"),
        "fn main() { this_does_not_compile(); }\n",
    )
    .unwrap();
    diagnostics.clear();
    let error = build_native_executable(
        &spec,
        factory,
        &mut diagnostics,
        || Ok(()),
        OwnedChild::spawn_direct,
        || false,
    )
    .unwrap_err();
    let execution = error
        .downcast_ref::<ToolError>()
        .expect("original Host failure stays typed");
    let status = execution.evidence().unwrap().status.unwrap();
    assert_eq!(status.code(), Some(101));
    assert!(String::from_utf8_lossy(&diagnostics).contains("this_does_not_compile"));
    fs::remove_dir_all(root).expect("remove native fixture");
}

#[test]
fn native_plain_json_preserves_diagnostics_artifacts_and_compiler_warning_denial() {
    let (root, cargo_program) = fixture();
    let target = root.join("warning-target");
    for (target_name, target_kind, warning) in [
        ("worker", NativeTargetKind::Example, true),
        ("native-fixture", NativeTargetKind::Binary, false),
    ] {
        let spec = NativeExecutableSpec {
            package: "native-fixture",
            target_name,
            target_kind,
            selection: CargoSelection {
                locked: true,
                offline: true,
                ..Default::default()
            },
            max_message_bytes: 1024 * 1024,
        };
        for (warning_level, compiler_deny) in [("warn", false), ("deny", false), ("warn", true)] {
            let mut original = None;
            for _ in 0..2 {
                let mut diagnostics = Vec::new();
                let result = build_native_executable(
                    &spec,
                    || {
                        let mut command = command(&cargo_program, &root, &target);
                        command
                            .env_remove("RUSTFLAGS")
                            .env_remove("CARGO_ENCODED_RUSTFLAGS")
                            .env("CARGO_BUILD_WARNINGS", warning_level);
                        if compiler_deny {
                            command.env("RUSTFLAGS", "-D warnings");
                        }
                        command
                    },
                    &mut diagnostics,
                    || Ok(()),
                    OwnedChild::spawn_direct,
                    || false,
                );
                if warning && compiler_deny {
                    let error = result.unwrap_err();
                    let failure = error.downcast_ref::<ToolError>().unwrap();
                    assert_eq!(
                        failure.evidence().unwrap().status.unwrap().code(),
                        Some(101)
                    );
                } else {
                    let built = result.unwrap();
                    assert_eq!(built.target_directory, target);
                    let selected = (
                        built.executable.clone(),
                        fs::read(built.executable).unwrap(),
                    );
                    if let Some(original) = &original {
                        assert_eq!(&selected, original);
                    } else {
                        original = Some(selected);
                    }
                }
                if warning {
                    assert!(String::from_utf8_lossy(&diagnostics).contains("unused_fixture"));
                }
                if let Some((path, bytes)) = &original {
                    assert_eq!(&fs::read(path).unwrap(), bytes);
                }
            }
        }
    }
    fs::remove_dir_all(root).unwrap();
}

fn fixture() -> (std::path::PathBuf, OsString) {
    let root = ic_build::testing::fs::unique_temp_directory("native-executable");
    fs::create_dir(root.join("examples")).unwrap();
    fs::write(root.join("Cargo.toml"), "[package]\nname = \"native-fixture\"\nversion = \"0.0.0\"\nedition = \"2024\"\n[workspace]\n[profile.custom]\ninherits = \"dev\"\n").unwrap();
    fs::write(
        root.join("examples/worker.rs"),
        "fn unused_fixture() {}\nfn main() { println!(\"{}\", env!(\"NATIVE_FIXTURE_VALUE\")); }\n",
    )
    .unwrap();
    fs::create_dir(root.join("src")).unwrap();
    fs::write(
        root.join("src/main.rs"),
        "fn main() { println!(\"binary\"); }\n",
    )
    .unwrap();
    let cargo_program = ic_build::testing::cargo::generate_fixture_lockfile(&root);
    (root, cargo_program)
}

fn command(cargo: &std::ffi::OsStr, root: &std::path::Path, target: &std::path::Path) -> Command {
    let mut command = Command::new(cargo);
    command
        .current_dir(root)
        .env("CARGO_TARGET_DIR", target)
        .env("CARGO_NET_OFFLINE", "true")
        .env("NATIVE_FIXTURE_VALUE", "factory-preserved");
    command
}
