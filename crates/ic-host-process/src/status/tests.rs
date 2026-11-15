use super::*;
use std::{os::unix::process::ExitStatusExt as _, process::Command};

#[test]
fn observed_exits_and_signals_keep_their_numeric_identity() {
    for code in [0, 37, 255] {
        let status = Command::new("/bin/sh")
            .args(["-c", &format!("exit {code}")])
            .status()
            .unwrap();
        assert_eq!(shell_status_code(status), Some(code));
    }
    let status = Command::new("/bin/sh")
        .args(["-c", "kill -TERM $$"])
        .status()
        .unwrap();
    assert_eq!(shell_status_code(status), Some(143));
}

#[test]
fn unclassified_status_leaves_the_fallback_to_each_caller() {
    let stopped = ExitStatus::from_raw((19 << 8) | 0x7f);
    assert_eq!(shell_status_code(stopped), None);
    assert_eq!(shell_status_code(stopped).unwrap_or(1), 1);
    assert_eq!(shell_status_code(stopped).unwrap_or(129), 129);
}
