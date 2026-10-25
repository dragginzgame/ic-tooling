//! Private filesystem fixtures.

/// Re-exec one libtest case without unrelated threads inheriting its descriptors.
#[cfg(unix)]
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
        "isolated {context} failed:\n{}\n{}",
        String::from_utf8_lossy(&output.stdout),
        String::from_utf8_lossy(&output.stderr)
    );
    true
}

pub fn fixture() -> crate::testing::FixtureDirectory {
    crate::testing::FixtureDirectory::new(&std::env::temp_dir(), "ic-host-fs-test")
        .expect("create test fixture")
}

/// Create a real FIFO without opening either endpoint; callers select its mode.
#[cfg(unix)]
pub fn create_fifo(path: &std::path::Path, permissions: Option<u32>) -> std::io::Result<()> {
    // rustix's mkfifoat is unavailable on Apple hosts. Use the native POSIX
    // utility consistently so every special-file fixture exercises a real FIFO.
    let mut command = std::process::Command::new("/usr/bin/mkfifo");
    command.env_clear();
    if let Some(permissions) = permissions {
        command.args(["-m", &format!("{permissions:o}")]);
    }
    let status = command.arg(path).status()?;
    if status.success() {
        Ok(())
    } else {
        Err(std::io::Error::other(format!(
            "FIFO fixture creation failed: {status}"
        )))
    }
}
