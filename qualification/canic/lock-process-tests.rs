use super::*;

#[test]
fn lock_failure_retains_status_bytes_and_escaped_diagnostic() {
    let error = initialize_lock(Command::new("/bin/sh").args([
        "-c",
        "printf 'out'; printf 'lock oracle\\033[31m\\n' >&2; exit 7",
    ]))
    .unwrap_err();
    let crate::Error::Supplier(source) = error else {
        panic!("wrong error owner")
    };
    let failure = source.downcast_ref::<LockFailure>().unwrap();
    assert_eq!(failure.report.status.unwrap().code(), Some(7));
    assert!(failure.report.failure.is_none() && failure.report.cleanup.is_empty());
    assert_eq!(failure.stdout, b"out");
    assert_eq!(failure.stderr, b"lock oracle\x1b[31m\n");
    let display = failure.to_string();
    assert!(display.contains("lock oracle"));
    assert!(!display.contains('\x1b'));
}

#[test]
fn lock_diagnostic_remains_bounded_and_marks_partial_capture() {
    let text = vec![b'x'; 5000];
    let diagnostic = crate::artifact::cargo_diagnostic_bytes(b"", &text, false).unwrap();
    assert_eq!(
        diagnostic,
        format!("{}\n[Cargo diagnostic truncated]", "x".repeat(4096))
    );
    assert_eq!(
        crate::artifact::cargo_diagnostic_bytes(b"", b"partial", true).unwrap(),
        "partial\n[Cargo diagnostic truncated]"
    );
}
