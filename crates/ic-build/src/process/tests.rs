use std::{
    io::{self, Write},
    process::Command,
    sync::atomic::AtomicBool,
    time::Duration,
};

use super::run;
use crate::{Failure, tests::limits};

#[test]
fn drains_real_concurrent_pipes_and_reaps() {
    let mut command = Command::new("/bin/sh");
    command.args([
        "-c",
        "(head -c 131072 /dev/zero >&2) & head -c 131072 /dev/zero; wait",
    ]);
    let mut stdout = 0;
    let mut sink = Vec::new();
    let report = run(
        &mut command,
        &limits(),
        &AtomicBool::new(false),
        &mut sink,
        |bytes| stdout += bytes.len(),
    );
    assert!(report.failure.is_none(), "{:?}", report.failure);
    assert!(report.status.unwrap().success());
    assert!(report.cleanup.is_empty());
    assert_eq!(stdout, 131072);
    assert_eq!(sink.len(), 131072);
}

#[test]
fn inherited_pipe_deadline_and_cancellation_end_group() {
    let mut policy = limits();
    policy.elapsed = Duration::from_millis(50);
    let mut command = Command::new("/bin/sh");
    command.args(["-c", "sleep 30 & exit 0"]);
    let report = run(
        &mut command,
        &policy,
        &AtomicBool::new(false),
        &mut io::sink(),
        |_| {},
    );
    assert!(matches!(report.failure, Some(Failure::Deadline)));
    assert!(report.cleanup.is_empty());
    let mut command = Command::new("/bin/sh");
    command.args(["-c", "sleep 30"]);
    let report = run(
        &mut command,
        &limits(),
        &AtomicBool::new(true),
        &mut io::sink(),
        |_| {},
    );
    assert!(matches!(report.failure, Some(Failure::Cancelled)));
    assert!(report.status.is_some());
    assert!(report.cleanup.is_empty());
}

///
/// Broken
/// Refuses diagnostic writes with a broken pipe.
///

struct Broken;

impl Write for Broken {
    fn write(&mut self, _: &[u8]) -> io::Result<usize> {
        Err(io::ErrorKind::BrokenPipe.into())
    }

    fn flush(&mut self) -> io::Result<()> {
        Ok(())
    }
}

#[test]
fn stdout_stderr_bounds_and_sink_refusal_are_not_success() {
    for stderr in [false, true] {
        let mut policy = limits();
        policy.stdout_bytes = 4;
        policy.stderr_bytes = 4;
        let mut command = Command::new("/bin/sh");
        command.args([
            "-c",
            if stderr {
                "printf overflow >&2"
            } else {
                "printf overflow"
            },
        ]);
        let report = run(
            &mut command,
            &policy,
            &AtomicBool::new(false),
            &mut io::sink(),
            |_| {},
        );
        assert!(matches!(
            report.failure,
            Some(Failure::OutputLimit | Failure::DiagnosticLimit)
        ));
        assert!(report.cleanup.is_empty());
    }
    let mut command = Command::new("/bin/sh");
    command.args(["-c", "printf evidence >&2"]);
    let report = run(
        &mut command,
        &limits(),
        &AtomicBool::new(false),
        &mut Broken,
        |_| {},
    );
    assert!(matches!(report.failure, Some(Failure::Sink(_))));
    assert_eq!(report.diagnostics, b"evidence");
}

///
/// CancelOnReady
/// Requests cancellation when the child emits its readiness diagnostic.
///

struct CancelOnReady<'a>(&'a AtomicBool);
impl Write for CancelOnReady<'_> {
    fn write(&mut self, bytes: &[u8]) -> io::Result<usize> {
        self.0.store(true, std::sync::atomic::Ordering::Relaxed);

        Ok(bytes.len())
    }

    fn flush(&mut self) -> io::Result<()> {
        Ok(())
    }
}

#[test]
fn cancellation_after_child_readiness_terminates_owned_execution() {
    let cancelled = AtomicBool::new(false);
    let mut command = Command::new("/bin/sh");
    command.args(["-c", "printf ready >&2; sleep 30"]);
    let report = run(
        &mut command,
        &limits(),
        &cancelled,
        &mut CancelOnReady(&cancelled),
        |_| {},
    );
    assert!(matches!(report.failure, Some(Failure::Cancelled)));
    assert_eq!(report.diagnostics, b"ready");
    assert!(!report.status.unwrap().success());
    assert!(report.cleanup.is_empty());
}
