use super::*;
#[cfg(unix)]
use crate::testing::cargo::write_fixture_package;

#[test]
fn every_compiler_environment_declaration_is_reported_without_values() {
    let root = unique_temp_directory("compiler-env-facts");
    let config = root.join("config.toml");
    for name in [
        "RUSTC",
        "RUSTDOC",
        "RUSTC_WRAPPER",
        "RUSTC_WORKSPACE_WRAPPER",
        "CARGO_BUILD_RUSTC",
        "CARGO_BUILD_RUSTDOC",
        "CARGO_BUILD_RUSTC_WRAPPER",
        "CARGO_BUILD_RUSTC_WORKSPACE_WRAPPER",
    ] {
        assert!(is_compiler_override(name));
        for value in [
            "''",
            "'private-executable'",
            "{value='private-executable',force=false,relative=false}",
            "{value='private-executable',force=true,relative=true}",
            "{value='',force=true,relative=false}",
        ] {
            let contents = format!("[env]\n{name}={value}\nAPI_TOKEN='private-token'\n");
            fs::write(&config, &contents).unwrap();
            let observed =
                observe_configurations(std::slice::from_ref(&config), CONFIGURATION_LIMITS)
                    .unwrap();
            assert_eq!(
                observed[0].compiler_overrides,
                [CompilerOverride::Environment(name)]
            );
            assert!(!format!("{:?}", observed[0].compiler_overrides).contains("private-"));
            assert_eq!(fs::read_to_string(&config).unwrap(), contents);
        }
    }
    fs::remove_dir_all(root).unwrap();
}

#[test]
fn build_fields_report_presence_even_for_empty_or_invalid_values() {
    let root = unique_temp_directory("compiler-build-facts");
    let config = root.join("config.toml");
    for field in [
        "rustc",
        "rustdoc",
        "rustc-wrapper",
        "rustc-workspace-wrapper",
    ] {
        for value in [
            "''",
            "'private-executable'",
            "17",
            "{value='private-executable'}",
        ] {
            fs::write(&config, format!("[build]\n{field}={value}\n")).unwrap();
            let observed =
                observe_configurations(std::slice::from_ref(&config), CONFIGURATION_LIMITS)
                    .unwrap();
            // Observation is a refusal fact, not Cargo value/type admission.
            assert_eq!(
                observed[0].compiler_overrides,
                [CompilerOverride::Build(field)]
            );
        }
    }
    fs::remove_dir_all(root).unwrap();
}

#[test]
fn invalid_selection_tables_refuse_instead_of_reporting_absence() {
    let root = unique_temp_directory("compiler-invalid-tables");
    let config = root.join("config.toml");
    for (contents, expected) in [
        (
            "build='private-executable'",
            "Cargo configuration `build` must be a table",
        ),
        (
            "env='private-executable'",
            "Cargo configuration `env` must be a table",
        ),
    ] {
        fs::write(&config, contents).unwrap();
        assert!(
            matches!(observe_configurations(std::slice::from_ref(&config), CONFIGURATION_LIMITS),
            Err(ConfigurationError::Invalid {path, message}) if path == config && message == expected)
        );
    }
    fs::remove_dir_all(root).unwrap();
}

#[test]
fn ordinary_settings_do_not_claim_a_compiler_override() {
    let root = unique_temp_directory("compiler-default-facts");
    let config = root.join("config.toml");
    fs::write(&config, "[build]\njobs=1\nrustflags=['--cfg','fixture']\n[env]\nRUSTDOCFLAGS='--cfg fixture'\nRUSTC_BOOTSTRAP='0'\nAPI_TOKEN='private-token'\n").unwrap();
    let observe = || {
        observe_configurations(std::slice::from_ref(&config), CONFIGURATION_LIMITS)
            .unwrap()
            .remove(0)
    };
    let before = observe();
    let after = observe();
    assert_eq!(before.compiler_overrides, []);
    assert_eq!(before.sha256, after.sha256);
    assert_eq!(before.recipe_sha256, after.recipe_sha256);
    fs::remove_dir_all(root).unwrap();
}

#[test]
fn included_environment_declarations_are_not_merged_away() {
    let root = unique_temp_directory("compiler-included-env");
    let config = root.join("config.toml");
    let included = root.join("included.toml");
    fs::write(
        &config,
        "include=['included.toml']\n[build]\nrustc=''\n[env]\nRUSTC=''\n",
    )
    .unwrap();
    fs::write(&included, "[env]\nCARGO_BUILD_RUSTC_WRAPPER={value='private-wrapper',relative=true}\nRUSTC={value='private-compiler',force=true}\n").unwrap();
    let observed =
        observe_configurations(std::slice::from_ref(&config), CONFIGURATION_LIMITS).unwrap();
    assert_eq!(observed[0].path, config);
    assert_eq!(
        observed[0].compiler_overrides,
        [
            CompilerOverride::Build("rustc"),
            CompilerOverride::Environment("RUSTC")
        ]
    );
    assert_eq!(observed[1].path, included);
    assert_eq!(
        observed[1].compiler_overrides,
        [
            CompilerOverride::Environment("RUSTC"),
            CompilerOverride::Environment("CARGO_BUILD_RUSTC_WRAPPER")
        ]
    );
    fs::remove_dir_all(root).unwrap();
}

#[cfg(unix)]
#[test]
fn cargo_precedence_and_empty_wrappers_do_not_erase_refusal_facts() {
    use ic_host_process::tool::{OutputLimit, OutputLimits};
    use std::{process::Command, time::Duration};

    let root = unique_temp_directory("compiler-cargo-precedence");
    write_fixture_package(
        &root,
        "compiler_selection",
        "[workspace]\n",
        "pub fn proof() {}\n",
    )
    .unwrap();
    let directory = root.join(".cargo");
    fs::create_dir(&directory).unwrap();
    let marker = root.join("wrapper-events");
    let wrappers = [
        "included-wrapper",
        "configured-wrapper",
        "environment-wrapper",
    ]
    .map(|name| {
        write_executable(
            root.join(name),
            "#!/bin/sh\nprintf '%s\\n' \"$0\" >> \"$WRAPPER_EVENTS\"\nexec \"$@\"\n",
        )
    });
    let config = directory.join("config.toml");
    fs::write(
        directory.join("included.toml"),
        wrapper_configuration(&wrappers[0]),
    )
    .unwrap();
    fs::write(
        &config,
        format!(
            "include=['included.toml']\n{}",
            wrapper_configuration(&wrappers[1])
        ),
    )
    .unwrap();

    for (index, (variable, value, expected)) in [
        (None, None, Some(&wrappers[1])),
        (Some("RUSTC_WRAPPER"), Some(OsStr::new("")), None),
        (
            Some("RUSTC_WRAPPER"),
            Some(wrappers[2].as_os_str()),
            Some(&wrappers[2]),
        ),
        (
            Some("CARGO_BUILD_RUSTC_WRAPPER"),
            Some(wrappers[2].as_os_str()),
            Some(&wrappers[2]),
        ),
    ]
    .into_iter()
    .enumerate()
    {
        let mut command = Command::new(env!("CARGO"));
        command
            .current_dir(&root)
            .env_clear()
            .envs(crate::testing::cargo::environment())
            .args(["check", "--offline", "--quiet", "--lib"])
            .env("CARGO_HOME", root.join("cargo-home"))
            .env("CARGO_TARGET_DIR", root.join(format!("target-{index}")))
            .env("RUSTUP_AUTO_INSTALL", "0")
            .env("WRAPPER_EVENTS", &marker);
        for name in COMPILER_VARIABLES {
            command.env_remove(name);
        }
        if let (Some(name), Some(value)) = (variable, value) {
            command.env(name, value);
        }
        fs::write(&marker, "").unwrap();
        crate::tools::probe::bytes(
            &mut command,
            OutputLimits {
                stdout: OutputLimit::Terminate(4096),
                stderr: OutputLimit::Terminate(16 * 1024),
                timeout: Some(Duration::from_secs(60)),
            },
        )
        .unwrap();
        let events = fs::read_to_string(&marker).unwrap();
        if let Some(expected) = expected {
            assert_ne!(events, "");
            assert!(events.lines().all(|event| Path::new(event) == expected));
        } else {
            assert_eq!(events, "");
        }
        let observed =
            observe_configurations(std::slice::from_ref(&config), CONFIGURATION_LIMITS).unwrap();
        assert_eq!(observed.len(), 2);
        for file in observed {
            assert_eq!(
                file.compiler_overrides,
                [CompilerOverride::Build("rustc-wrapper")]
            );
        }
    }
    fs::remove_dir_all(root).unwrap();
}

#[cfg(unix)]
#[test]
fn configuration_hashes_do_not_bind_same_version_bytes_or_executable_aliases() {
    use ic_host_process::tool::{OutputLimit, OutputLimits};
    use std::{os::unix::fs::symlink, process::Command, time::Duration};
    let root = unique_temp_directory("compiler-executable-custody");
    let first = root.join("first");
    let second = root.join("second");
    for (path, comment) in [(&first, "first bytes"), (&second, "second bytes")] {
        write_executable(
            path.clone(),
            &format!("#!/bin/sh\nprintf 'compiler 1\\n'\n# {comment}\n"),
        );
    }
    let selected = root.join("selected");
    symlink(&first, &selected).unwrap();
    let config = root.join("config.toml");
    fs::write(
        &config,
        format!(
            "[build]\nrustc={}\n",
            Value::String(selected.to_str().unwrap().into())
        ),
    )
    .unwrap();
    let observe = || {
        observe_configurations(std::slice::from_ref(&config), CONFIGURATION_LIMITS)
            .unwrap()
            .remove(0)
    };
    let version = || {
        crate::tools::probe::bytes(
            Command::new(&selected).arg("--version"),
            OutputLimits {
                stdout: OutputLimit::Terminate(4096),
                stderr: OutputLimit::Terminate(4096),
                timeout: Some(Duration::from_secs(5)),
            },
        )
        .unwrap()
    };
    let before = observe();
    let original_version = version();
    let original_bytes = fs::read(&selected).unwrap();
    fs::write(
        &first,
        "#!/bin/sh\nprintf 'compiler 1\\n'\n# replaced bytes\n",
    )
    .unwrap();
    assert_ne!(fs::read(&selected).unwrap(), original_bytes);
    for repoint in [false, true] {
        if repoint {
            fs::remove_file(&selected).unwrap();
            symlink(&second, &selected).unwrap();
        }
        let after = observe();
        assert_eq!(version(), original_version);
        assert_eq!(after.sha256, before.sha256);
        assert_eq!(after.recipe_sha256, before.recipe_sha256);
        assert_eq!(after.compiler_overrides, [CompilerOverride::Build("rustc")]);
    }
    fs::remove_dir_all(root).unwrap();
}

#[cfg(unix)]
fn write_executable(path: PathBuf, contents: &str) -> PathBuf {
    use std::os::unix::fs::PermissionsExt as _;
    fs::write(&path, contents).unwrap();
    fs::set_permissions(&path, fs::Permissions::from_mode(0o755)).unwrap();
    path
}

#[cfg(unix)]
fn wrapper_configuration(wrapper: &Path) -> String {
    format!(
        "[build]\nrustc-wrapper={}\n",
        Value::String(wrapper.to_str().unwrap().into())
    )
}
