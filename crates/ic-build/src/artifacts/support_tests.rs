#[cfg(unix)]
use ic_host_process::testing::write_executable_script;

use crate::testing::fs::unique_temp_directory;
use std::path::PathBuf;

/// Real Cargo metadata and identities, with a deterministic minimal Wasm producer.
#[cfg(unix)]
pub(super) fn fake_wasm_build_spec(label: &str) -> (PathBuf, super::wasm_cache::WasmBuildSpec) {
    let root = unique_temp_directory(label);
    crate::testing::cargo::write_workspace(&root, &["fixture"]).expect("write workspace manifest");
    crate::testing::cargo::write_fixture_package(
        &root.join("fixture"),
        "fixture",
        "[lib]\ncrate-type = [\"cdylib\"]\n",
        "pub fn value() -> u8 { 1 }\n",
    )
    .expect("write fixture package");
    let wrapper = root.join("cargo.sh");
    write_fake_wasm_cargo(&root, &root, FakeWasmCargo::default());
    let spec = super::wasm_cache::WasmBuildSpec::new(
        &root,
        &root.join("exact"),
        &["fixture"],
        crate::testing::cargo::environment(),
        CONFIGURATION_LIMITS,
    )
    .with_isolated_builds()
    .with_cargo_program(wrapper);
    (root, spec)
}

/// Explicit script hooks and protocol output for this deterministic fixture.
#[cfg(unix)]
#[derive(Clone, Copy, Default)]
pub(super) struct FakeWasmCargo<'a> {
    pub(super) before_dispatch: &'a str,
    pub(super) before_build: &'a str,
    pub(super) artifact_messages: Option<&'a str>,
    pub(super) after_build: &'a str,
    pub(super) exit_status: u8,
}

#[cfg(unix)]
pub(super) fn write_fake_wasm_cargo(
    root: &std::path::Path,
    workspace: &std::path::Path,
    options: FakeWasmCargo<'_>,
) {
    let default_message;
    let artifact_message = if let Some(messages) = options.artifact_messages {
        messages
    } else {
        default_message = crate::testing::cargo::wasm_artifact_message_script(workspace, "target");
        &default_message
    };
    write_executable_script(
        &root.join("cargo.sh"),
        format!(
            r#"#!/bin/sh
set -eu
{before_dispatch}
if [ "$1" = build ]; then
    {before_build}
    target="$CARGO_TARGET_DIR"
    mkdir -p "$target/wasm32-unknown-unknown/debug"
    printf '\000asm\001\000\000\000' > "$target/wasm32-unknown-unknown/debug/fixture.wasm"
    {artifact_message}
    {after_build}
    exit {exit_status}
fi
exec "${{CARGO:-cargo}}" "$@"
"#,
            before_dispatch = options.before_dispatch,
            before_build = options.before_build,
            after_build = options.after_build,
            exit_status = options.exit_status,
        ),
    )
    .expect("write executable test script");
}

// This caller admits small configuration trees; libraries do not select this policy.
const CONFIGURATION_LIMITS: crate::cargo::environment::ConfigurationLimits =
    crate::cargo::environment::ConfigurationLimits {
        file_bytes: 1024 * 1024,
        total_bytes: 16 * 1024 * 1024,
        files: 128,
        depth: 32,
    };
