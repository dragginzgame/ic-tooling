use super::*;
use ic_host_process::tool::{ExecutionFailure, OutputLimit, OutputStream, ToolError};

const fn limits() -> OutputLimits {
    OutputLimits {
        stdout: OutputLimit::Truncate(1024),
        stderr: OutputLimit::Truncate(1024),
        timeout: None,
    }
}

#[test]
fn failed_probe_retains_exit_status_and_both_streams() {
    let error = bytes(
        Command::new("sh").args(["-c", "printf observation; printf diagnostic >&2; exit 7"]),
        limits(),
    )
    .unwrap_err();
    let ToolError::Execution(cause) = error
        .get_ref()
        .unwrap()
        .downcast_ref::<ToolError>()
        .unwrap()
    else {
        panic!("expected typed execution failure");
    };
    assert!(matches!(cause.failure, ExecutionFailure::ExitStatus));
    assert_eq!(cause.evidence.status.unwrap().code(), Some(7));
    assert_eq!(cause.evidence.stdout, b"observation");
    assert_eq!(cause.evidence.stderr, b"diagnostic");
    assert!(!error.to_string().contains("diagnostic"));
}

#[test]
fn incomplete_probe_still_refuses_before_status_or_text_admission() {
    let error = text(
        Command::new("sh").args(["-c", "printf long; exit 7"]),
        OutputLimits {
            stdout: OutputLimit::Truncate(1),
            ..limits()
        },
    )
    .unwrap_err();
    let ToolError::Execution(cause) = error
        .get_ref()
        .unwrap()
        .downcast_ref::<ToolError>()
        .unwrap()
    else {
        panic!("expected typed output failure");
    };
    assert!(matches!(
        cause.failure,
        ExecutionFailure::OutputLimit {
            stream: OutputStream::Stdout
        }
    ));
    assert_eq!(cause.evidence.status.unwrap().code(), Some(7));
    assert_eq!(cause.evidence.stdout, b"l");
}
