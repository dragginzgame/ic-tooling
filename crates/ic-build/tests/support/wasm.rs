//! Shared real-Wasm workspace for integration targets.

use std::{io, path::Path};

/// Write and lock an independent, dependency-free cdylib workspace.
pub fn write_workspace(root: &Path) -> io::Result<()> {
    ic_build::testing::cargo::write_fixture_package(
        root,
        "fixture",
        "[workspace]\n[lib]\ncrate-type = [\"cdylib\"]\n",
        "pub fn value() -> u32 { 7 }\n",
    )?;
    ic_build::testing::cargo::generate_fixture_lockfile(root);
    Ok(())
}

/// Construct a real-Wasm test recipe with this fixture adapter's selected environment.
pub fn spec(root: &Path, target: &Path, packages: &[&str]) -> ic_build::artifacts::WasmBuildSpec {
    ic_build::artifacts::WasmBuildSpec::new(
        root,
        target,
        packages,
        ic_build::testing::cargo::environment(),
        CONFIGURATION_LIMITS,
    )
}

// This caller admits small configuration trees; libraries do not select this policy.
pub const CONFIGURATION_LIMITS: ic_build::cargo::environment::ConfigurationLimits =
    ic_build::cargo::environment::ConfigurationLimits {
        file_bytes: 1024 * 1024,
        total_bytes: 16 * 1024 * 1024,
        files: 128,
        depth: 32,
    };
