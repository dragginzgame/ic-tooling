//! Private fixture paths shared by unit and integration targets.

use std::path::PathBuf;

pub fn unique_temp_directory(label: &str) -> PathBuf {
    ic_host_fs::testing::create_fixture_directory(
        &std::env::temp_dir(),
        &format!("ic-testkit-{label}"),
    )
    .expect("create test directory")
}

/// Re-exec a libtest case so parallel forks cannot prolong its lock descriptors.
pub fn run_isolated(driver: &str, context: &str) -> bool {
    if std::env::var_os(driver).is_some() {
        return false;
    }
    let thread = std::thread::current();
    let name = thread.name().expect("libtest names each test thread");
    let output = std::process::Command::new(std::env::current_exe().unwrap())
        .args(["--exact", name, "--test-threads=1"])
        .env(driver, "1")
        .output()
        .unwrap();
    assert!(
        output.status.success(),
        "isolated {context} failed: {}{}",
        String::from_utf8_lossy(&output.stdout),
        String::from_utf8_lossy(&output.stderr)
    );
    true
}
