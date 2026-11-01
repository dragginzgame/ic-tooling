//! Actual Cargo packaging, extracted graph admission and retained source evidence.
#![cfg(unix)]

mod support;

use ic_build::cargo::packages::{
    PackageArchiveSpec, PackageQualificationLimits, qualify_existing_package_archives,
    qualify_package_archives,
};
use ic_build::cargo::selection::CargoFeatureSelection;
use ic_host_artifacts::artifact::Sha256Digest;
use ic_host_process::tool::{OutputLimit, OutputLimits};
use std::{
    fs,
    os::unix::fs::PermissionsExt as _,
    path::{Path, PathBuf},
    process::Command,
};

#[test]
fn real_package_qualification_retains_exact_graph_and_source_inventory() {
    let root = ic_build::testing::fs::unique_temp_directory("package-qualification");
    fs::create_dir(root.join("src")).unwrap();
    fs::write(root.join("Cargo.toml"), "[workspace]\n[package]\nname='qualification-fixture'\nversion='0.1.0'\nedition='2024'\n[features]\noptional-check=[]\n[dependencies]\nitoa='1.0'\n").unwrap();
    fs::write(root.join("src/lib.rs"), "pub fn value() -> String { itoa::Buffer::new().format(42).to_owned() }\n#[cfg(feature = \"optional-check\")]\n#[test]\nfn formats_value() { assert_eq!(value(), \"42\"); }\n").unwrap();
    let cargo = || {
        let mut command = Command::new(env!("CARGO"));
        command
            .env("RUSTUP_AUTO_INSTALL", "0")
            .env("CARGO_NET_OFFLINE", "true")
            .env("CARGO_TARGET_DIR", root.join("target"));
        command
    };
    let prepared = cargo()
        .current_dir(&root)
        .args(["generate-lockfile", "--offline"])
        .output()
        .unwrap();
    assert!(
        prepared.status.success(),
        "{}",
        String::from_utf8_lossy(&prepared.stderr)
    );
    let entry_lock = fs::read(root.join("Cargo.lock")).unwrap();
    let evidence = ic_build::testing::fs::unique_temp_directory("package-qualification-evidence");
    fs::create_dir_all(root.join("target/package")).unwrap();
    fs::set_permissions(&evidence, fs::Permissions::from_mode(0o700)).unwrap();
    qualify_package_archives(
        &root,
        &root.join("target/package"),
        &evidence,
        &[PackageArchiveSpec {
            name: "qualification-fixture",
            version: "0.1.0",
            patch_registry: true,
        }],
        &CargoFeatureSelection {
            all_features: true,
            ..CargoFeatureSelection::default()
        },
        PackageQualificationLimits {
            process: OutputLimits {
                stdout: OutputLimit::Terminate(4 * 1024 * 1024),
                stderr: OutputLimit::Terminate(1024 * 1024),
                timeout: None,
            },
            file_bytes: 4 * 1024 * 1024,
            source_entries: 100,
            source_depth: 10,
        },
        cargo,
        |archive, destination| {
            let mut command = Command::new("tar");
            command.arg("-xzf").arg(archive).arg("-C").arg(destination);
            command
        },
    )
    .unwrap();
    assert!(
        fs::read_to_string(evidence.join("test-targets.json"))
            .unwrap()
            .contains("formats_value ... ok")
    );
    assert_eq!(fs::read(root.join("Cargo.lock")).unwrap(), entry_lock);
    let archive = fs::read(root.join("target/package/qualification-fixture-0.1.0.crate")).unwrap();
    assert_eq!(
        fs::read(evidence.join("archive.sha256")).unwrap(),
        format!("{}\n", Sha256Digest::compute(&archive)).as_bytes()
    );
    let original = fs::read(evidence.join("entry-sources.json")).unwrap();
    assert_eq!(
        original,
        fs::read(evidence.join("final-sources.json")).unwrap()
    );
    let inventory: serde_json::Value = serde_json::from_slice(&original).unwrap();
    assert_eq!(
        inventory["qualification-fixture-0.1.0/src/lib.rs"]["sha256"],
        Sha256Digest::compute(&fs::read(root.join("src/lib.rs")).unwrap()).to_string()
    );
    assert!(inventory["qualification-fixture-0.1.0/src/lib.rs"]["mode"].is_number());
    for name in ["test-targets.json", "test-docs.json", "admitted-Cargo.lock"] {
        assert!(evidence.join(name).is_file());
    }
}

fn custom_fixture() -> PathBuf {
    let root = ic_build::testing::fs::unique_temp_directory("custom-package-qualification");
    fs::create_dir(root.join("src")).unwrap();
    fs::write(
        root.join("Cargo.toml"),
        "[workspace]\n[package]\nname='custom-fixture'\nversion='0.1.0'\nedition='2024'\n[dependencies]\nitoa='1.0'\n",
    )
    .unwrap();
    fs::write(root.join("src/lib.rs"), "pub fn value() -> u8 { 42 }\n#[cfg(test)] compile_error!(\"default tests must not run\");\n").unwrap();
    let cargo = || cargo_for(&root);
    for args in [
        vec!["generate-lockfile", "--offline"],
        vec![
            "package",
            "--offline",
            "--locked",
            "--allow-dirty",
            "--no-verify",
        ],
    ] {
        let output = cargo().current_dir(&root).args(args).output().unwrap();
        assert!(
            output.status.success(),
            "{}",
            String::from_utf8_lossy(&output.stderr)
        );
    }
    root
}

fn cargo_for(root: &Path) -> Command {
    let mut command = Command::new(env!("CARGO"));
    command
        .env("RUSTUP_AUTO_INSTALL", "0")
        .env("CARGO_NET_OFFLINE", "true")
        .env("CARGO_TARGET_DIR", root.join("target"));
    command
}

#[test]
fn custom_qualification_uses_existing_archives_and_preserves_custody() {
    let root = custom_fixture();
    let cargo = || cargo_for(&root);
    let archive_root = root.join("target/package");
    let archive = archive_root.join("custom-fixture-0.1.0.crate");
    let archive_bytes = fs::read(&archive).unwrap();
    let lock_bytes = fs::read(root.join("Cargo.lock")).unwrap();
    for case in [
        "success",
        "callback-failure",
        "source-drift",
        "archive-drift",
        "original-lock-drift",
        "graph-drift",
        "missing-archive",
    ] {
        let evidence = root.join(case);
        fs::create_dir(&evidence).unwrap();
        fs::set_permissions(&evidence, fs::Permissions::from_mode(0o700)).unwrap();
        if case == "missing-archive" {
            fs::remove_file(&archive).unwrap();
        }
        let invoked = std::cell::Cell::new(false);
        let result = qualify_existing_package_archives(
            &root,
            &archive_root,
            &evidence,
            &[PackageArchiveSpec {
                name: "custom-fixture",
                version: "0.1.0",
                patch_registry: true,
            }],
            &CargoFeatureSelection::default(),
            PackageQualificationLimits {
                process: OutputLimits {
                    stdout: OutputLimit::Terminate(4 * 1024 * 1024),
                    stderr: OutputLimit::Terminate(1024 * 1024),
                    timeout: Some(std::time::Duration::from_secs(60)),
                },
                file_bytes: 4 * 1024 * 1024,
                source_entries: 100,
                source_depth: 10,
            },
            cargo,
            |archive, destination| {
                let mut command = Command::new("tar");
                command.arg("-xzf").arg(archive).arg("-C").arg(destination);
                command
            },
            |workspace| {
                invoked.set(true);
                assert_eq!(fs::read(&archive)?, archive_bytes);
                match case {
                    "callback-failure" => {
                        return Err(std::io::Error::other("consumer failure").into());
                    }
                    "source-drift" => {
                        fs::write(workspace.join("custom-fixture-0.1.0/src/lib.rs"), "changed")?;
                    }
                    "archive-drift" => fs::write(&archive, "changed")?,
                    "original-lock-drift" => fs::write(root.join("Cargo.lock"), "changed")?,
                    "graph-drift" => fs::write(workspace.join("Cargo.lock"), "changed")?,
                    _ => {
                        let output = cargo()
                            .current_dir(workspace)
                            .args(["check", "--workspace", "--lib", "--locked", "--offline"])
                            .output()?;
                        if !output.status.success() {
                            return Err(String::from_utf8_lossy(&output.stderr)
                                .into_owned()
                                .into());
                        }
                    }
                }
                fs::write(workspace.join("consumer-check"), b"ran")?;
                Ok(())
            },
        );
        if case == "success" {
            result.unwrap();
            assert!(evidence.join("final-sources.json").is_file());
        } else {
            let error = result.unwrap_err();
            assert_eq!(error.evidence, evidence);
            if case == "callback-failure" {
                assert_eq!(
                    error
                        .cause
                        .downcast_ref::<std::io::Error>()
                        .unwrap()
                        .to_string(),
                    "consumer failure"
                );
            }
        }
        assert_eq!(invoked.get(), case != "missing-archive");
        assert!(!evidence.join("test-targets.json").exists());
        assert!(!evidence.join("test-docs.json").exists());
        fs::write(&archive, &archive_bytes).unwrap();
        fs::write(root.join("Cargo.lock"), &lock_bytes).unwrap();
    }
}
