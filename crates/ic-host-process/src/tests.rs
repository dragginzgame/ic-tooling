use super::*;

fn policy(ownership: Ownership) -> Policy {
    Policy {
        ownership,
        input: Input::Null,
        stdout_bytes: 131072,
        stderr: Stderr::Capture(131072),
        elapsed: Duration::from_secs(5),
    }
}

#[test]
fn simultaneous_streams_have_independent_exact_budgets_and_natural_status() {
    let mut command = Command::new("/bin/sh");
    command.args([
        "-c",
        "(head -c 131072 /dev/zero >&2) & head -c 131072 /dev/zero; wait; exit 7",
    ]);
    let mut counts = [0, 0];
    let report = run(
        &mut command,
        policy(Ownership::Group),
        &AtomicBool::new(false),
        |stream, bytes| {
            counts[usize::from(stream == Stream::Stderr)] += bytes.len();
            Ok(())
        },
    );
    assert!(report.failure.is_none(), "{report:?}");
    assert!(report.cleanup.is_empty());
    assert_eq!(report.status.unwrap().code(), Some(7));
    assert_eq!(counts, [131072; 2]);
}

#[test]
fn overflow_and_observer_failure_keep_original_status_and_partial_evidence() {
    for stream in [Stream::Stdout, Stream::Stderr] {
        let mut command = Command::new("/bin/sh");
        command.args([
            "-c",
            if stream == Stream::Stdout {
                "printf 12345; exit 9"
            } else {
                "printf 12345 >&2; exit 9"
            },
        ]);
        let mut limits = policy(Ownership::Group);
        limits.stdout_bytes = 4;
        limits.stderr = Stderr::Capture(4);
        let mut retained = vec![];
        let report = run(&mut command, limits, &AtomicBool::new(false), |_, bytes| {
            retained.extend_from_slice(bytes);
            Ok(())
        });
        assert!(matches!(report.failure, Some(Failure::Limit(value)) if value == stream));
        assert!(retained.len() <= 4);
        assert!(report.status.is_some());
        assert!(report.cleanup.is_empty());
    }
    // Wait for the leader to exit before the observer refuses. Cleanup must not
    // replace its original natural nonzero status with the observer failure.
    let mut command = Command::new("/bin/sh");
    command.args(["-c", "printf '%s:retained' \"$$\"; exit 13"]);
    let mut retained = vec![];
    let report = run(
        &mut command,
        policy(Ownership::Group),
        &AtomicBool::new(false),
        |_, bytes| {
            retained.extend_from_slice(bytes);
            if !retained.ends_with(b":retained") {
                return Ok(());
            }
            let text = std::str::from_utf8(&retained).unwrap();
            let pid = Pid::from_raw(text.split(':').next().unwrap().parse().unwrap()).unwrap();
            let started = Instant::now();
            while waitid(
                WaitId::Pid(pid),
                WaitIdOptions::EXITED | WaitIdOptions::NOHANG | WaitIdOptions::NOWAIT,
            )
            .unwrap()
            .is_none()
            {
                assert!(started.elapsed() < Duration::from_secs(5));
                std::thread::sleep(Duration::from_millis(1));
            }
            Err(io::ErrorKind::BrokenPipe.into())
        },
    );
    assert!(retained.ends_with(b":retained"));
    assert!(matches!(report.failure, Some(Failure::Observer { .. })));
    assert_eq!(report.status.unwrap().code(), Some(13));
    assert!(report.cleanup.is_empty());
}

#[test]
fn direct_child_joins_parent_group_and_inherited_stderr_is_not_captured() {
    let mut command = Command::new("/bin/sh");
    command.args(["-c", "ps -o pgid= -p $$; printf inherited >&2"]);
    let mut limits = policy(Ownership::Direct);
    limits.input = Input::Inherit;
    limits.stderr = Stderr::Inherit;
    let mut output = vec![];
    let report = run(
        &mut command,
        limits,
        &AtomicBool::new(false),
        |stream, bytes| {
            assert_eq!(stream, Stream::Stdout);
            output.extend_from_slice(bytes);
            Ok(())
        },
    );
    assert!(report.failure.is_none(), "{report:?}");
    assert!(report.status.unwrap().success());
    assert!(report.cleanup.is_empty());
    assert_eq!(
        String::from_utf8(output)
            .unwrap()
            .trim()
            .parse::<i32>()
            .unwrap(),
        getpgrp().as_raw_pid()
    );
}

#[test]
fn readiness_cancellation_and_retained_pipe_deadline_close_only_owned_execution() {
    for ownership in [Ownership::Direct, Ownership::Group] {
        let cancelled = AtomicBool::new(false);
        let mut command = Command::new("/bin/sh");
        command.args(["-c", "printf ready; exec sleep 30"]);
        let report = run(&mut command, policy(ownership), &cancelled, |_, bytes| {
            assert_eq!(bytes, b"ready");
            cancelled.store(true, Ordering::Relaxed);
            Ok(())
        });
        assert!(matches!(report.failure, Some(Failure::Cancelled)));
        assert!(!report.status.unwrap().success());
        assert!(report.cleanup.is_empty());
    }
    let mut command = Command::new("/bin/sh");
    command.args(["-c", "sleep 30 & exit 23"]);
    let mut limits = policy(Ownership::Group);
    limits.elapsed = Duration::from_millis(100);
    let report = run(&mut command, limits, &AtomicBool::new(false), |_, _| Ok(()));
    assert!(matches!(report.failure, Some(Failure::Deadline)));
    assert_eq!(report.status.unwrap().code(), Some(23));
    assert!(report.cleanup.is_empty());
}

#[test]
fn spawn_failure_has_no_status_and_close_is_locally_idempotent() {
    let report = run(
        &mut Command::new("/no/such/tool"),
        policy(Ownership::Direct),
        &AtomicBool::new(false),
        |_, _| Ok(()),
    );
    assert!(matches!(report.failure, Some(Failure::Io(_))));
    assert!(report.status.is_none());
    assert!(report.cleanup.is_empty());
    let child = Command::new("/bin/sh")
        .args(["-c", "exit 0"])
        .spawn()
        .unwrap();
    let mut session = Session {
        child: Some(child),
        ownership: Ownership::Direct,
    };
    let (status, errors) = session.close();
    assert!(status.is_some());
    assert!(errors.is_empty());
    let (status, errors) = session.close();
    assert!(status.is_none());
    assert!(errors.is_empty());
}

#[test]
#[ignore = "requires a controlling PTY with fixture-answer on stdin"]
fn foreground_terminal_input_survives_direct_child_ownership() {
    use std::io::IsTerminal as _;
    assert!(
        io::stdin().is_terminal(),
        "run through the prepared PTY fixture"
    );
    let mut command = Command::new("/bin/sh");
    command.args(["-c", "test -t 0 || exit 41; test -t 2 || exit 42; read answer; test \"$answer\" = fixture-answer || exit 43; printf foreground-ok"]);
    let mut limits = policy(Ownership::Direct);
    limits.input = Input::Inherit;
    limits.stderr = Stderr::Inherit;
    let mut output = vec![];
    let report = run(
        &mut command,
        limits,
        &AtomicBool::new(false),
        |stream, bytes| {
            assert_eq!(stream, Stream::Stdout);
            output.extend_from_slice(bytes);
            Ok(())
        },
    );
    assert!(report.failure.is_none(), "{report:?}");
    assert!(report.status.unwrap().success(), "{report:?}");
    assert!(report.cleanup.is_empty());
    assert_eq!(output, b"foreground-ok");
}

#[test]
fn cleanup_failures_preserve_order_status_and_end_local_signal_authority() {
    for fail_reap in [false, true] {
        let child = Command::new("/bin/sh")
            .args(["-c", "exit 17"])
            .spawn()
            .unwrap();
        let mut owner = Session {
            child: Some(child),
            ownership: Ownership::Group,
        };
        // Wait before close so even the deliberately failed signalling branch
        // has a real exited child to reap. Do not change credentials or process IDs.
        let pid = Pid::from_child(owner.child.as_ref().unwrap());
        let start = Instant::now();
        while waitid(
            WaitId::Pid(pid),
            WaitIdOptions::EXITED | WaitIdOptions::NOHANG | WaitIdOptions::NOWAIT,
        )
        .unwrap()
        .is_none()
        {
            assert!(start.elapsed() < Duration::from_secs(5));
            std::thread::sleep(Duration::from_millis(1));
        }
        let direct_called = std::cell::Cell::new(false);
        let (status, errors) = owner.close_with(
            |_| Err(io::Error::from_raw_os_error(1)),
            |_| {
                direct_called.set(true);
                Err(io::Error::from_raw_os_error(5))
            },
            |child| {
                let status = child.wait()?; // Always reap the fixture for real.
                if fail_reap {
                    Err(io::Error::from_raw_os_error(10))
                } else {
                    Ok(status)
                }
            },
        );
        assert!(
            direct_called.get(),
            "group refusal must attempt direct fallback"
        );
        assert_eq!(
            errors
                .iter()
                .map(io::Error::raw_os_error)
                .collect::<Vec<_>>(),
            if fail_reap {
                vec![Some(1), Some(5), Some(10)]
            } else {
                vec![Some(1), Some(5)]
            }
        );
        if fail_reap {
            assert!(status.is_none());
        } else {
            assert_eq!(status.unwrap().code(), Some(17));
        }
        let (again, errors) = owner.close_with(
            |_| panic!("closed group must not be signalled"),
            |_| panic!("closed child must not be signalled"),
            |_| panic!("closed child must not be waited again"),
        );
        assert!(again.is_none());
        assert!(errors.is_empty());
    }
}

#[test]
fn group_signal_failure_remains_visible_after_successful_direct_fallback() {
    let child = Command::new("/bin/sh")
        .args(["-c", "exec sleep 30"])
        .spawn()
        .unwrap();
    let mut owner = Session {
        child: Some(child),
        ownership: Ownership::Group,
    };
    let (status, errors) = owner.close_with(
        |_| Err(io::Error::from_raw_os_error(1)),
        Child::kill,
        Child::wait,
    );
    assert!(!status.unwrap().success());
    assert_eq!(errors.len(), 1);
    assert_eq!(errors[0].raw_os_error(), Some(1));
}

#[cfg(target_os = "linux")]
#[test]
fn group_descendant_stops_but_direct_descendant_can_finish_after_close() {
    // Observe identity/lifetime through procfs; never signal a borrowed PID. The
    // descendant has a bounded self-expiry even if this test fails before release.
    fn identity(pid: u32) -> Option<(String, char)> {
        let stat = std::fs::read_to_string(format!("/proc/{pid}/stat")).ok()?;
        let fields: Vec<_> = stat.rsplit_once(") ")?.1.split_whitespace().collect();
        Some((fields.get(19)?.to_string(), fields.first()?.chars().next()?))
    }
    for ownership in [Ownership::Group, Ownership::Direct] {
        let dir = tempfile::tempdir().unwrap();
        let release = dir.path().join("release");
        let witness = dir.path().join("survived");
        let mut command = Command::new("/bin/sh");
        command.args(["-c", "(n=0; while test ! -e \"$1\"; do n=$((n+1)); test $n -lt 500 || exit 0; sleep 0.01; done; printf survived > \"$2\") & printf '%s\\n' $!; exec sleep 30", "fixture"])
            .arg(&release).arg(&witness);
        let cancelled = AtomicBool::new(false);
        let mut output = vec![];
        let mut descendant = None;
        let report = run(&mut command, policy(ownership), &cancelled, |_, bytes| {
            output.extend_from_slice(bytes);
            if output.ends_with(b"\n") {
                let pid: u32 = std::str::from_utf8(&output)
                    .unwrap()
                    .trim()
                    .parse()
                    .unwrap();
                let (start, state) = identity(pid).expect("descendant must exist at readiness");
                assert!(!matches!(state, 'Z' | 'X'));
                descendant = Some((pid, start));
                cancelled.store(true, Ordering::Relaxed);
            }
            Ok(())
        });
        assert!(matches!(report.failure, Some(Failure::Cancelled)));
        assert!(report.cleanup.is_empty());
        let (pid, start) = descendant.unwrap();
        let deadline = Instant::now();
        if ownership == Ownership::Group {
            while let Some((current, state)) = identity(pid) {
                if current != start || matches!(state, 'Z' | 'X') {
                    break;
                }
                assert!(
                    deadline.elapsed() < Duration::from_secs(5),
                    "owned descendant still running"
                );
                std::thread::sleep(Duration::from_millis(1));
            }
        } else {
            // Only now let the descendant produce evidence. Completion therefore
            // proves survival beyond close, not merely a pre-cancellation write.
            std::fs::write(&release, b"go").unwrap();
            while !witness.exists() {
                assert!(
                    deadline.elapsed() < Duration::from_secs(5),
                    "direct close killed descendant"
                );
                std::thread::sleep(Duration::from_millis(1));
            }
            assert_eq!(std::fs::read(&witness).unwrap(), b"survived");
        }
    }
}
