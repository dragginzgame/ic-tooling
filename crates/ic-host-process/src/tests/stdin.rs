use crate::*;

fn piped(input: Vec<u8>) -> Policy {
    Policy {
        ownership: Ownership::Group,
        input: Input::Bytes(input),
        stdout_bytes: 524288,
        stderr: Stderr::Capture(131072),
        elapsed: Duration::from_secs(5),
    }
}

#[test]
fn large_input_and_both_output_pipes_progress_without_a_writer_thread() {
    let input: Vec<u8> = (0..262144).map(|n| (n % 256) as u8).collect();
    let expected = input.clone();
    let mut command = Command::new("/bin/sh");
    command.args([
        "-c",
        "(head -c 131072 /dev/zero >&2) & head -c 131072 /dev/zero; cat; wait",
    ]);
    let mut stdout = Vec::new();
    let mut stderr = Vec::new();
    let report = run(
        &mut command,
        piped(input),
        &AtomicBool::new(false),
        |stream, bytes| {
            match stream {
                Stream::Stdout => stdout.extend_from_slice(bytes),
                Stream::Stderr => stderr.extend_from_slice(bytes),
            }

            Ok(())
        },
    );
    assert!(report.failure.is_none(), "{report:?}");
    assert!(report.cleanup.is_empty(), "{report:?}");
    assert!(report.status.unwrap().success());
    assert_eq!(stderr, vec![0; 131072]);
    assert_eq!(&stdout[..131072], vec![0; 131072]);
    assert_eq!(&stdout[131072..], expected);
}

#[test]
fn empty_input_closes_before_eof_dependent_output_and_debug_omits_payload() {
    let mut command = Command::new("/bin/sh");
    command.args(["-c", "cat; printf eof"]);
    let mut stdout = Vec::new();
    let report = run(
        &mut command,
        piped(vec![]),
        &AtomicBool::new(false),
        |_, bytes| {
            stdout.extend_from_slice(bytes);

            Ok(())
        },
    );
    assert!(report.failure.is_none(), "{report:?}");
    assert!(report.cleanup.is_empty(), "{report:?}");
    assert!(report.status.unwrap().success());
    assert_eq!(stdout, b"eof");
    let diagnostic = format!(
        "{:?}",
        piped(b"fixture input is not diagnostic output".to_vec())
    );
    assert!(diagnostic.contains("len: 38"));
    assert!(!diagnostic.contains("fixture input"));
}

#[test]
fn early_exit_cannot_turn_partial_stdin_into_success() {
    let mut command = Command::new("/bin/sh");
    command.args(["-c", "exit 7"]);
    let report = run(
        &mut command,
        piped(vec![1; 1048576]),
        &AtomicBool::new(false),
        |_, _| Ok(()),
    );
    assert!(
        matches!(report.failure, Some(Failure::Io(ref error)) if error.kind() == io::ErrorKind::BrokenPipe),
        "{report:?}"
    );
    assert_eq!(report.status.unwrap().code(), Some(7));
    assert!(report.cleanup.is_empty(), "{report:?}");
}

#[test]
fn blocked_input_obeys_cancellation_deadline_and_owned_cleanup() {
    for cancel in [true, false] {
        let cancelled = AtomicBool::new(false);
        let mut command = Command::new("/bin/sh");
        command.args(["-c", "sleep 1 & printf ready; wait"]);
        let mut policy = piped(vec![1; 1048576]);
        policy.elapsed = Duration::from_millis(100);
        let started = Instant::now();
        let report = run(&mut command, policy, &cancelled, |_, _| {
            if cancel {
                cancelled.store(true, Ordering::Relaxed);
            }

            Ok(())
        });
        if cancel {
            assert!(
                matches!(report.failure, Some(Failure::Cancelled)),
                "{report:?}"
            );
        } else {
            assert!(
                matches!(report.failure, Some(Failure::Deadline)),
                "{report:?}"
            );
        }
        assert!(started.elapsed() < Duration::from_millis(800));
        assert!(!report.status.unwrap().success());
        assert!(report.cleanup.is_empty(), "{report:?}");
    }
}

#[test]
fn output_limit_still_refuses_while_stdin_is_incomplete() {
    let mut command = Command::new("/bin/sh");
    command.args(["-c", "head -c 131072 /dev/zero; sleep 1"]);
    let mut policy = piped(vec![1; 1048576]);
    policy.stdout_bytes = 32;
    let mut retained = Vec::new();
    let report = run(&mut command, policy, &AtomicBool::new(false), |_, bytes| {
        retained.extend_from_slice(bytes);

        Ok(())
    });
    assert!(
        matches!(report.failure, Some(Failure::Limit(Stream::Stdout))),
        "{report:?}"
    );
    assert!(retained.len() <= 32);
    assert!(report.status.is_some());
    assert!(report.cleanup.is_empty(), "{report:?}");
}
