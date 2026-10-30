//! Cargo fixture scaffolding shared by unit and integration targets.

use std::{fs, io, path::Path};

/// Valid protocol scaffold; individual tests retain their own invalid mutations.
///
/// # Panics
/// Panics if the supplied paths cannot be represented as JSON.
#[must_use]
pub fn artifact_message(
    package: &str,
    target: &serde_json::Value,
    filenames: &[&Path],
    executable: Option<&Path>,
) -> serde_json::Value {
    serde_json::json!({
        "reason": "compiler-artifact", "package_id": package, "target": target,
        "profile": {"opt_level": "0", "debuginfo": 0, "debug_assertions": true,
            "overflow_checks": true, "test": false},
        "features": [], "filenames": filenames, "executable": executable, "fresh": false
    })
}

/// Write a minimal fixture workspace; return original directory/write errors.
///
/// # Errors
/// Returns directory creation or manifest write errors.
pub fn write_workspace(root: &Path, members: &[&str]) -> io::Result<()> {
    fs::create_dir_all(root)?;
    let members = toml::Value::Array(
        members
            .iter()
            .map(|member| toml::Value::String((*member).to_owned()))
            .collect(),
    );
    fs::write(
        root.join("Cargo.toml"),
        format!("[workspace]\nmembers = {members}\nresolver = \"2\"\n"),
    )
}

/// Write a fixture library and caller-selected manifest fields; preserve I/O errors.
///
/// # Errors
/// Returns directory creation, manifest or source write errors.
pub fn write_fixture_package(
    directory: &Path,
    manifest_tail: &str,
    source: &str,
) -> io::Result<()> {
    fs::create_dir_all(directory.join("src"))?;
    fs::write(
        directory.join("Cargo.toml"),
        format!(
            "[package]\nname = \"fixture\"\nversion = \"0.0.0\"\nedition = \"2024\"\n{manifest_tail}"
        ),
    )?;
    fs::write(directory.join("src/lib.rs"), source)
}

/// A selected external dependency containing two independent Cargo workspaces.
///
/// # Errors
/// Returns directory creation, manifest or source write errors.
pub fn write_nested_workspace_fixture(root: &Path) -> io::Result<()> {
    for (directory, name, tail) in [
        (
            "driver",
            "driver",
            "[workspace]\n[dependencies]\nadapter = { path = \"../adapter\" }\n",
        ),
        (
            "adapter",
            "adapter",
            "include = [\"src/**\", \"README.md\", \"LICENSE\"]\n",
        ),
        ("adapter/consumer", "consumer", "[workspace]\n"),
        (
            "adapter/embedded-consumer",
            "embedded-consumer",
            "[workspace]\n",
        ),
    ] {
        let directory = root.join(directory);
        fs::create_dir_all(directory.join("src"))?;
        fs::write(
            directory.join("Cargo.toml"),
            format!(
                "[package]\nname = \"{name}\"\nversion = \"0.0.0\"\nedition = \"2024\"\n{tail}"
            ),
        )?;
        fs::write(directory.join("src/lib.rs"), "pub fn value() {}\n")?;
    }
    Ok(())
}

/// Synthetic Cargo artifact record using the fixture's actual package/target ID.
/// The shell variable must contain a JSON-safe generated fixture output directory.
///
/// # Panics
/// Panics if Cargo metadata fails, is malformed or omits the fixture's cdylib target.
#[must_use]
pub fn wasm_artifact_message_script(root: &Path, variable: &str) -> String {
    let output =
        std::process::Command::new(std::env::var_os("CARGO").unwrap_or_else(|| "cargo".into()))
            .current_dir(root)
            .args([
                "metadata",
                "--offline",
                "--no-deps",
                "--format-version",
                "1",
            ])
            .output()
            .expect("observe fixture Cargo metadata");
    assert!(
        output.status.success(),
        "fixture metadata: {}",
        String::from_utf8_lossy(&output.stderr)
    );
    let metadata: serde_json::Value = serde_json::from_slice(&output.stdout).unwrap();
    let package = metadata["packages"]
        .as_array()
        .unwrap()
        .iter()
        .find(|package| package["name"] == "fixture")
        .unwrap();
    let target = package["targets"]
        .as_array()
        .unwrap()
        .iter()
        .find(|target| {
            target["kind"]
                .as_array()
                .unwrap()
                .iter()
                .any(|kind| kind == "cdylib")
        })
        .unwrap();
    let message = artifact_message(
        package["id"].as_str().unwrap(),
        target,
        &[Path::new(
            "@TARGET@/wasm32-unknown-unknown/debug/fixture.wasm",
        )],
        None,
    )
    .to_string();
    let quoted = format!("'{}'", message.replace('\'', "'\"'\"'"));
    let quoted = quoted.replace("@TARGET@", &format!("'\"${variable}\"'"));
    format!("printf '%s\\n' {quoted}\n")
}

/// Test-selected compiler group cleanup, independent of progress reporting.
///
/// # Errors
/// Returns the native child spawn error.
pub fn spawn_cargo(
    command: &mut std::process::Command,
) -> std::io::Result<ic_host_process::child::OwnedChild> {
    ic_host_process::child::OwnedChild::spawn(
        command,
        ic_host_process::child::CleanupPolicy::KillAndWait,
    )
}

/// This test adapter selects the toolchain/runtime inputs needed by its Cargo fixtures.
#[must_use]
pub fn environment() -> std::collections::BTreeMap<std::ffi::OsString, std::ffi::OsString> {
    [
        "PATH",
        "HOME",
        "USERPROFILE",
        "CARGO",
        "RUSTC",
        "CARGO_HOME",
        "RUSTUP_HOME",
        "RUSTUP_TOOLCHAIN",
        "RUSTC_WRAPPER",
        "RUSTC_WORKSPACE_WRAPPER",
        "CARGO_BUILD_RUSTC",
        "RUSTFLAGS",
        "CARGO_ENCODED_RUSTFLAGS",
        "CARGO_NET_OFFLINE",
        "TMPDIR",
        "LD_LIBRARY_PATH",
        "DYLD_LIBRARY_PATH",
    ]
    .into_iter()
    .filter_map(|name| std::env::var_os(name).map(|value| (name.into(), value)))
    .collect()
}

/// Build one complete fixture environment with caller-selected replacements.
pub fn environment_with<I, K, V>(
    extra: I,
) -> std::collections::BTreeMap<std::ffi::OsString, std::ffi::OsString>
where
    I: IntoIterator<Item = (K, V)>,
    K: Into<std::ffi::OsString>,
    V: Into<std::ffi::OsString>,
{
    let mut selected = environment();
    selected.extend(
        extra
            .into_iter()
            .map(|(key, value)| (key.into(), value.into())),
    );
    selected
}
