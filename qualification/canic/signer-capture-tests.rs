use super::*;
use ic_host_process_next::Failure;

fn shell(script: &str, limit: usize, elapsed: Duration) -> Result<Vec<u8>, CaptureFailure> {
    capture(
        Command::new("/bin/sh").args(["-c", script]),
        limit,
        limit,
        elapsed,
    )
}

#[test]
fn exact_limits_and_nonzero_status_keep_independent_evidence() {
    let elapsed = Duration::from_secs(2);
    assert_eq!(
        shell("printf 1234; printf 5678 >&2", 4, elapsed).unwrap(),
        b"1234"
    );
    let failure = shell("printf out; printf err >&2; exit 7", 4, elapsed).unwrap_err();
    assert!(failure.report.failure.is_none());
    assert_eq!(failure.report.status.unwrap().code(), Some(7));
    assert!(failure.report.cleanup.is_empty());
    assert_eq!(failure.stdout, b"out");
    assert_eq!(failure.stderr, b"err");
}

#[test]
fn overflow_identifies_the_stream_and_never_returns_complete_bytes() {
    for (script, expected) in [
        ("printf 12345", Stream::Stdout),
        ("printf 12345 >&2", Stream::Stderr),
    ] {
        let failure = shell(script, 4, Duration::from_secs(2)).unwrap_err();
        assert!(
            matches!(failure.report.failure, Some(Failure::Limit(stream)) if stream == expected)
        );
        assert!(failure.report.status.is_some());
        assert!(failure.report.cleanup.is_empty());
        assert!(failure.stdout.len() <= 4 && failure.stderr.len() <= 4);
    }
}

#[test]
fn deadline_and_spawn_failure_remain_distinguishable() {
    let failure = shell("sleep 2 & wait", 4, Duration::from_millis(50)).unwrap_err();
    assert!(matches!(failure.report.failure, Some(Failure::Deadline)));
    assert!(failure.report.status.is_some());
    assert!(failure.report.cleanup.is_empty());
    let missing = std::path::PathBuf::from(std::env::var_os("CANIC_SIGNER_EVIDENCE").unwrap())
        .join("missing-executable");
    assert!(!missing.exists());
    let failure = capture(&mut Command::new(missing), 4, 4, Duration::from_secs(2)).unwrap_err();
    assert!(
        matches!(failure.report.failure, Some(Failure::Io(ref error)) if error.kind() == io::ErrorKind::NotFound)
    );
    assert!(failure.report.status.is_none());
    assert!(failure.report.cleanup.is_empty());
    assert!(failure.stdout.is_empty() && failure.stderr.is_empty());
}
