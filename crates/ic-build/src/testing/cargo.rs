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
    name: &str,
    manifest_tail: &str,
    source: &str,
) -> io::Result<()> {
    fs::create_dir_all(directory.join("src"))?;
    fs::write(
        directory.join("Cargo.toml"),
        format!(
            "[package]\nname = \"{name}\"\nversion = \"0.0.0\"\nedition = \"2024\"\nrust-version = \"{}\"\n{manifest_tail}",
            env!("CARGO_PKG_RUST_VERSION")
        ),
    )?;
    fs::write(directory.join("src/lib.rs"), source)
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

/// Generate a fixture lockfile offline and return its selected Cargo executable.
///
/// # Panics
/// Panics if the fixture command cannot start or Cargo rejects its manifest.
#[expect(
    clippy::must_use_candidate,
    reason = "lockfile generation is the operation; reusing the selected Cargo path is optional"
)]
pub fn generate_fixture_lockfile(root: &Path) -> std::ffi::OsString {
    let cargo = std::env::var_os("CARGO").unwrap_or_else(|| "cargo".into());
    assert!(
        std::process::Command::new(&cargo)
            .current_dir(root)
            .env("RUSTUP_AUTO_INSTALL", "0")
            .args(["generate-lockfile", "--offline"])
            .status()
            .unwrap()
            .success()
    );
    cargo
}
