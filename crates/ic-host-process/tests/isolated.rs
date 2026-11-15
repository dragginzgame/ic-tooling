//! Named libtest re-execution through the process fixture owner.
#![cfg(all(unix, feature = "test-support"))]

use ic_host_process::{
    testing::run_isolated,
    tool::{OutputLimit, OutputLimits},
};
use std::time::Duration;

#[test]
fn isolated_case_selects_the_current_test_and_returns_in_the_parent() {
    if run_isolated(
        "IC_HOST_PROCESS_ISOLATED_CASE",
        "helper fixture",
        OutputLimits {
            stdout: OutputLimit::Terminate(4096),
            stderr: OutputLimit::Terminate(4096),
            timeout: Some(Duration::from_secs(10)),
        },
    ) {
        assert!(std::env::var_os("IC_HOST_PROCESS_ISOLATED_CASE").is_none());
        return;
    }
    assert_eq!(
        std::env::var("IC_HOST_PROCESS_ISOLATED_CASE").unwrap(),
        std::thread::current().name().unwrap()
    );
}

#[test]
fn failed_child_preserves_the_context_status_and_bounded_output() {
    const DRIVER: &str = "IC_HOST_PROCESS_FAILED_ISOLATED_CASE";
    let thread = std::thread::current();
    if std::env::var(DRIVER).as_deref() == Ok(thread.name().unwrap()) {
        eprintln!("retained worker diagnostic");
        std::process::exit(23);
    }
    let failure = std::panic::catch_unwind(|| {
        run_isolated(
            DRIVER,
            "failed worker",
            OutputLimits {
                stdout: OutputLimit::Terminate(4096),
                stderr: OutputLimit::Terminate(4096),
                timeout: Some(Duration::from_secs(10)),
            },
        )
    })
    .expect_err("failed worker must refuse");
    let message = failure.downcast_ref::<String>().unwrap();
    assert!(message.contains("failed worker"));
    assert!(message.contains("status:"));
    assert!(message.contains("retained worker diagnostic"));
}
