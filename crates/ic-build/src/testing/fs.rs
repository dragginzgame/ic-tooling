//! Fixture paths shared by unit and integration targets.

use std::path::PathBuf;

/// Allocate a retained fixture beneath the current process's temporary directory.
///
/// # Panics
/// Panics if fixture directory creation fails. Callers own removal or retention.
#[must_use]
pub fn unique_temp_directory(label: &str) -> PathBuf {
    ic_host_fs::testing::create_fixture_directory(
        &std::env::temp_dir(),
        &format!("ic-testkit-{label}"),
    )
    .expect("create test directory")
}
