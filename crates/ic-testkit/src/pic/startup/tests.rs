use crate::pocket_ic::ClientError;
mod instances;

use std::{
    fs,
    path::PathBuf,
    time::{Duration, Instant},
};

use super::{
    OwnedDirectory, PocketIcStartupConfig, PocketIcStartupError, PocketIcStartupFailure,
    StartupFiles, create_unique_directory,
};
use crate::pocket_ic::PocketIcBuilder;

#[cfg(unix)]
use ic_host_process::testing::write_executable_script;
#[cfg(unix)]
use std::{
    io::Write as _,
    os::unix::fs::{OpenOptionsExt as _, PermissionsExt as _},
    process::Command,
    sync::mpsc,
};

#[test]
fn completed_instances_must_still_meet_the_startup_deadline() {
    let started = Instant::now();
    let startup = super::StartupDeadline {
        started,
        timeout: Duration::from_secs(60),
        deadline: started + Duration::from_secs(60),
    };
    let mut server = None;
    let instance = startup
        .admit_instance(7_u8, "http://localhost/", &mut server)
        .unwrap();
    assert_eq!(*instance.instance(), 7);
    assert!(server.is_none());
    let expired = super::StartupDeadline {
        deadline: started,
        ..startup
    };
    let error = expired
        .admit_instance(7_u8, "http://localhost/", &mut server)
        .err()
        .expect("completed instance after the deadline must refuse admission");
    assert!(
        matches!(error.failure(), PocketIcStartupFailure::InstanceCreationTimeout { timeout } if *timeout == Duration::from_secs(60))
    );
    assert!(server.is_none());
}

#[test]
fn original_failure_output_and_both_typed_cleanup_reports_survive_projection() {
    use crate::pocket_ic::ClientError;
    use std::error::Error as _;
    for failure in [
        PocketIcStartupFailure::ReadinessTimeout {
            server_binary: "server".into(),
            timeout: Duration::from_secs(1),
        },
        PocketIcStartupFailure::InstanceCreationTimeout {
            timeout: Duration::from_secs(1),
        },
        PocketIcStartupFailure::InvalidServerPort {
            server_binary: "server".into(),
            value: "invalid".into(),
        },
        PocketIcStartupFailure::BuilderFailed {
            source: ClientError::Protocol("original-client-failure".into()),
        },
        PocketIcStartupFailure::Io {
            operation: "original-io",
            path: "port".into(),
            source: std::io::Error::from_raw_os_error(9),
        },
    ] {
        let primary_text = failure.to_string();
        let output = super::PocketIcManagedServerOutput {
            stdout: "original-stdout".into(),
            stderr: "original-stderr".into(),
        };
        let command = super::CleanupError {
            status: None,
            group_error: None,
            term_error: Some(std::io::Error::from_raw_os_error(4)),
            kill_error: Some(std::io::Error::from_raw_os_error(5)),
            wait_error: None,
        };
        let server = super::CleanupError {
            status: None,
            group_error: Some(std::io::Error::from_raw_os_error(1)),
            term_error: None,
            kill_error: None,
            wait_error: Some(std::io::Error::from_raw_os_error(10)),
        };
        let error = PocketIcStartupError::new(failure)
            .with_cleanup(Some(command), Some((output, Some(server))));
        assert_eq!(error.failure().to_string(), primary_text);
        assert_eq!(error.output().stdout(), "original-stdout");
        assert_eq!(error.output().stderr(), "original-stderr");
        let command = error.command_cleanup().unwrap();
        let server = error.server_cleanup().unwrap();
        assert_eq!(command.kill_error.as_ref().unwrap().raw_os_error(), Some(5));
        assert_eq!(command.term_error.as_ref().unwrap().raw_os_error(), Some(4));
        assert_eq!(server.group_error.as_ref().unwrap().raw_os_error(), Some(1));
        assert_eq!(server.wait_error.as_ref().unwrap().raw_os_error(), Some(10));
        if matches!(error.failure(), PocketIcStartupFailure::Io { .. }) {
            assert_eq!(
                error
                    .source()
                    .unwrap()
                    .downcast_ref::<std::io::Error>()
                    .unwrap()
                    .raw_os_error(),
                Some(9)
            );
        }
        if let PocketIcStartupFailure::BuilderFailed { source } = error.failure() {
            assert!(std::ptr::eq(
                error
                    .source()
                    .unwrap()
                    .downcast_ref::<ClientError>()
                    .unwrap(),
                source
            ));
        }
        let displayed = error.to_string();
        assert!(displayed.starts_with(&primary_text));
        for marker in [
            "original-stdout",
            "original-stderr",
            "command cleanup also failed",
            "server cleanup also failed",
        ] {
            assert!(displayed.contains(marker));
        }
    }
    let error = PocketIcStartupError::new(PocketIcStartupFailure::NotConfigured);
    assert_eq!(error.to_string(), error.failure().to_string());
    assert!(error.command_cleanup().is_none());
    assert!(error.server_cleanup().is_none());
}

#[cfg(unix)]
#[test]
fn port_io_and_command_spawn_failures_finalize_the_owned_server() {
    for port_io in [true, false] {
        let port = if port_io {
            "mkfifo \"$2\""
        } else {
            "printf '34567\n' > \"$2\""
        };
        let script = TestServerScript::new(
            "owned-io-failure",
            &format!(
                "#!/bin/sh\nprintf '%s\n%s\n' \"$$\" \"$2\"\nprintf 'original-server-diagnostic' >&2\n{port}\nexec sleep 30\n"
            ),
        );
        let config = script.config(Duration::from_secs(2));
        let error = if port_io {
            config.start_managed_server().err().unwrap()
        } else {
            config
                .run_command(&mut Command::new("/missing/ic-testkit-command"), || false)
                .unwrap_err()
        };
        let PocketIcStartupFailure::Io { source, .. } = error.failure() else {
            panic!("expected original IO failure: {error:?}");
        };
        assert_eq!(
            source.kind(),
            if port_io {
                std::io::ErrorKind::InvalidData
            } else {
                std::io::ErrorKind::NotFound
            }
        );
        assert_eq!(error.output().stderr(), "original-server-diagnostic");
        let mut lines = error.output().stdout().lines();
        let pid = lines.next().unwrap().parse().unwrap();
        let port = PathBuf::from(lines.next().unwrap());
        assert_eq!(process_state(pid), None);
        assert!(!port.parent().unwrap().exists());
        assert!(error.server_cleanup().is_none());
    }
}

#[cfg(unix)]
#[test]
fn command_cancellation_before_startup_has_no_spawn_effects() {
    let error = PocketIcStartupConfig::Spawn {
        server_binary: ("/missing/server").into(),
        timeout: Duration::from_secs(1),
        server_hard_ttl: None,
        server_idle_ttl: None,
        server_output_files: None,
    }
    .run_command(&mut Command::new("/missing/command"), || true)
    .unwrap_err();
    assert!(
        matches!(error.failure(), PocketIcStartupFailure::CommandRun { source, .. } if source.kind() == std::io::ErrorKind::Interrupted)
    );
    assert_eq!(error.output().stdout(), "");
    assert_eq!(error.output().stderr(), "");
    assert!(error.command_cleanup().is_none());
    assert!(error.server_cleanup().is_none());
}

#[cfg(unix)]
#[test]
fn cancellation_callback_panic_still_reaps_the_owned_command() {
    let script = TestServerScript::new(
        "cancel-panic",
        "#!/bin/sh\nprintf '%s' \"$$\" > \"$1\"\nexec sleep 30\n",
    );
    let pid_file = script.path().with_extension("pid");
    let result = std::panic::catch_unwind(|| {
        PocketIcStartupConfig::Connect {
            server_url: ("http://127.0.0.1:12345/").into(),
            timeout: Duration::from_secs(1),
        }
        .run_command(Command::new(script.path()).arg(&pid_file), || {
            assert!(
                !fs::read_to_string(&pid_file).is_ok_and(|value| !value.is_empty()),
                "caller cancellation failed"
            );
            false
        })
    });
    assert!(result.is_err());
    let pid = fs::read_to_string(&pid_file).unwrap().parse().unwrap();
    assert!(process_state(pid).is_none_or(|state| state == 'Z'));
    fs::remove_file(pid_file).unwrap();
}

#[cfg(unix)]
#[test]
fn environment_selection_prefers_urls_and_fails_closed() {
    use std::os::unix::ffi::OsStringExt as _;

    let timeout = Duration::from_secs(1);
    let config = PocketIcStartupConfig::from_environment(timeout, |name| {
        Some(
            if name == "IC_TESTKIT_POCKET_IC_URL" {
                "http://127.0.0.1:12345/"
            } else {
                "/missing/server"
            }
            .into(),
        )
    })
    .unwrap();
    assert!(
        matches!(config, PocketIcStartupConfig::Connect { ref server_url, .. } if server_url == "http://127.0.0.1:12345/")
    );
    assert!(matches!(
        PocketIcStartupConfig::from_environment(timeout, |_| None)
            .as_ref()
            .map_err(PocketIcStartupError::failure),
        Err(PocketIcStartupFailure::NotConfigured)
    ));
    assert!(matches!(
        PocketIcStartupConfig::from_environment(timeout, |_| Some("".into()))
            .as_ref()
            .map_err(PocketIcStartupError::failure),
        Err(PocketIcStartupFailure::InvalidEnvironment {
            variable: "IC_TESTKIT_POCKET_IC_URL"
        })
    ));
    assert!(matches!(
        PocketIcStartupConfig::from_environment(timeout, |_| Some("bad URL".into()))
            .as_ref()
            .map_err(PocketIcStartupError::failure),
        Err(PocketIcStartupFailure::InvalidServerUrl { .. })
    ));
    assert!(matches!(
        PocketIcStartupConfig::from_environment(timeout, |_| Some(std::ffi::OsString::from_vec(
            vec![0xff]
        )))
        .as_ref()
        .map_err(PocketIcStartupError::failure),
        Err(PocketIcStartupFailure::InvalidEnvironment { .. })
    ));
}

#[cfg(unix)]
#[test]
fn version_capture_cleans_wrapper_descendants_on_exit_and_timeout() {
    for timeout in [false, true] {
        let qualified = format!(
            "printf 'pocket-ic-server {}\\n'",
            crate::pic::POCKET_IC_SERVER_VERSION
        );
        let ending = if timeout { "wait" } else { &qualified };
        let script = TestServerScript::new(
            "version-group",
            &format!(
                "#!/bin/sh\nsleep 30 >/dev/null 2>&1 &\nprintf '%s' \"$!\" > \"$0.pid\"\n{ending}\n"
            ),
        );
        let result = PocketIcStartupConfig::from_environment(Duration::from_secs(1), |name| {
            (name == "POCKET_IC_BIN").then(|| script.path().into_os_string())
        });
        if timeout {
            assert!(matches!(
                result.as_ref().map_err(PocketIcStartupError::failure),
                Err(PocketIcStartupFailure::ServerVersionProbe { .. })
            ));
        } else {
            assert!(result.is_ok());
        }
        let pid_file = script.path().with_extension("pid");
        let pid = fs::read_to_string(&pid_file).unwrap().parse().unwrap();
        let deadline = Instant::now() + Duration::from_secs(3);
        while process_state(pid).is_some_and(|state| state != 'Z') {
            assert!(
                Instant::now() < deadline,
                "version wrapper descendant remained alive"
            );
            std::thread::sleep(Duration::from_millis(20));
        }
        fs::remove_file(pid_file).unwrap();
    }
}

#[cfg(unix)]
#[tokio::test]
async fn owned_builder_failure_retains_server_output_and_reaps_child() {
    let script = TestServerScript::new(
        "builder-panic",
        "#!/bin/sh\nprintf '%s' \"$$\" > \"$0.pid\"\nprintf 'builder stdout'\nprintf 'builder stderr' >&2\nprintf '34567\\n' > \"$2\"\nexec sleep 30\n",
    );
    let error = script
        .config(Duration::from_secs(2))
        .build_instance(PocketIcBuilder::new(std::time::Duration::from_secs(300)))
        .await
        .err()
        .unwrap();
    let PocketIcStartupFailure::BuilderFailed { source } = error.failure() else {
        panic!("expected builder failure, got {error:?}");
    };
    assert_ne!(source.to_string(), "");
    assert!(std::ptr::eq(
        std::error::Error::source(&error)
            .unwrap()
            .downcast_ref::<ClientError>()
            .unwrap(),
        source
    ));
    assert_eq!(error.output().stdout(), "builder stdout");
    assert_eq!(error.output().stderr(), "builder stderr");
    assert!(error.server_cleanup().is_none());
    let pid_file = script.path().with_extension("pid");
    let pid = fs::read_to_string(&pid_file).unwrap().parse().unwrap();
    assert_eq!(process_state(pid), None);
    fs::remove_file(pid_file).unwrap();
}

#[cfg(unix)]
#[test]
fn environment_binary_selection_uses_bounded_shared_version_capture() {
    let qualified = format!(
        "printf 'pocket-ic-server {}\\n'",
        crate::pic::POCKET_IC_SERVER_VERSION
    );
    let failed = format!("{qualified}; exit 23");
    for (label, body, expected) in [
        ("qualified", qualified.as_str(), 0),
        ("compatible-16.0", "printf 'pocket-ic-server 16.0.0\\n'", 0),
        ("compatible-patch", "printf 'pocket-ic-server 16.0.1\\n'", 0),
        ("wrong-version", "printf 'pocket-ic-server 15.0.0\\n'", 1),
        ("future-major", "printf 'pocket-ic-server 17.0.0\\n'", 1),
        ("malformed-version", "printf 'pocket-ic-server 16.1\\n'", 1),
        ("failed-version", failed.as_str(), 2),
        ("invalid-utf8", "printf '\\377'", 1),
        ("version-timeout", "exec sleep 30", 2),
    ] {
        let script = TestServerScript::new(
            label,
            &format!("#!/bin/sh\n[ \"$1\" = --version ] || exit 99\n{body}\n"),
        );
        // A successful process launch needs scheduling headroom on loaded CI
        // hosts; only the deliberately stalled probe needs a short deadline.
        let timeout = if label == "version-timeout" {
            Duration::from_millis(200)
        } else {
            Duration::from_secs(5)
        };
        let result = PocketIcStartupConfig::from_environment(timeout, |name| {
            (name == "POCKET_IC_BIN").then(|| script.path().into_os_string())
        });
        match (
            expected,
            result.as_ref().map_err(PocketIcStartupError::failure),
        ) {
            (0, Ok(config)) => {
                let binary = script.path().canonicalize().unwrap();
                assert!(
                    matches!(config, PocketIcStartupConfig::Spawn { server_binary, .. } if server_binary == &binary)
                );
            }
            (1, Err(PocketIcStartupFailure::ServerVersionMismatch { .. }))
            | (2, Err(PocketIcStartupFailure::ServerVersionProbe { .. })) => {}
            (_, result) => panic!("unexpected {label} result: {result:?}"),
        }
    }
}

#[cfg(unix)]
fn process_state(pid: u32) -> Option<char> {
    ic_host_process::testing::process_state(pid).expect("inspect managed test process state")
}

#[cfg(unix)]
#[test]
fn reading_large_sparse_server_output_is_bounded() {
    let (files, _, _) = StartupFiles::create(None).expect("allocate startup files");
    let mut file = fs::File::create(&files.stdout).expect("create sparse log");
    file.write_all(b"server started\n").expect("write prefix");
    let size = 8_u64 * 1024 * 1024 * 1024;
    file.set_len(size).expect("extend sparse log");
    let output = super::read_bounded_lossy(&files.stdout);
    assert!(output.starts_with("server started\n"));
    assert!(output.ends_with(&format!(
        "<truncated {} bytes>",
        size - super::SERVER_OUTPUT_LIMIT as u64
    )));
    assert!(output.len() < super::SERVER_OUTPUT_LIMIT + 100);
}

#[cfg(unix)]
#[test]
fn startup_readers_reject_fifos_without_waiting_for_a_writer() {
    let (files, stdout, stderr) = StartupFiles::create(None).expect("allocate startup files");
    drop((stdout, stderr));
    fs::remove_file(&files.stdout).unwrap();
    fs::remove_file(&files.stderr).unwrap();
    for path in [&files.port, &files.stdout, &files.stderr] {
        ic_host_fs::testing::create_fifo(path, None).expect("create FIFO startup file");
    }
    for path in [&files.port, &files.stdout, &files.stderr] {
        // A delayed writer bounds a blocked read and records whether the
        // reader needed it. Completion cancels the writer; with no reader,
        // a nonblocking open fails and is retried if the reader starts late.
        let fifo = path.clone();
        let (stop_writer, stopped) = mpsc::channel();
        let writer = std::thread::spawn(move || {
            loop {
                match stopped.recv_timeout(Duration::from_millis(200)) {
                    Ok(()) | Err(mpsc::RecvTimeoutError::Disconnected) => return false,
                    Err(mpsc::RecvTimeoutError::Timeout) => {}
                }
                if fs::OpenOptions::new()
                    .write(true)
                    .custom_flags(libc::O_NONBLOCK)
                    .open(&fifo)
                    .is_ok()
                {
                    return true;
                }
            }
        });
        let result = if path == &files.port {
            Some(files.read_port())
        } else {
            assert_eq!(super::read_bounded_lossy(path), "");
            None
        };
        let _ = stop_writer.send(());
        assert!(
            !writer.join().expect("join delayed FIFO writer"),
            "startup reader waited for a writer: {}",
            path.display(),
        );
        if let Some(result) = result {
            assert!(matches!(
                result.as_ref().map_err(PocketIcStartupError::failure),
                Err(PocketIcStartupFailure::Io { source, .. })
                    if source.kind() == std::io::ErrorKind::InvalidData
            ));
        }
    }
}

#[test]
fn port_file_readiness_preserves_partial_writes_and_rejects_oversized_contents() {
    let (files, _, _) = StartupFiles::create(None).expect("allocate startup files");
    assert!(matches!(
        files.read_port().unwrap(),
        super::PortFileState::Pending
    ));
    for contents in ["", "34567"] {
        fs::write(&files.port, contents).unwrap();
        assert!(matches!(
            files.read_port().unwrap(),
            super::PortFileState::Pending
        ));
    }
    for (contents, expected) in [("1\n", 1), ("65535\n", 65535), (" 34567\r\n", 34567)] {
        fs::write(&files.port, contents).unwrap();
        assert!(matches!(
            files.read_port().unwrap(),
            super::PortFileState::Ready(port) if port == expected
        ));
    }
    for contents in ["0\n", "65536\n", "invalid\n", "1\n2\n"] {
        fs::write(&files.port, contents).unwrap();
        assert!(matches!(
            files.read_port().unwrap(),
            super::PortFileState::Invalid(_)
        ));
    }
    fs::write(&files.port, [0xff, b'\n']).unwrap();
    assert!(matches!(
        files.read_port().as_ref().map_err(PocketIcStartupError::failure),
        Err(PocketIcStartupFailure::Io { source, .. })
            if source.kind() == std::io::ErrorKind::InvalidData
    ));
    for contents in ["1\n".to_owned() + &" ".repeat(128), "0".repeat(128)] {
        fs::write(&files.port, contents).unwrap();
        assert!(
            matches!(files.read_port().unwrap(), super::PortFileState::Invalid(_)),
            "oversized port contents must fail even without a newline",
        );
    }
    // A large backing file must not enlarge the returned diagnostic.
    fs::File::options()
        .write(true)
        .open(&files.port)
        .unwrap()
        .set_len(1024 * 1024)
        .unwrap();
    assert!(matches!(
        files.read_port().unwrap(),
        super::PortFileState::Invalid(value) if value.len() < 256
    ));
}

#[test]
fn startup_config_requires_positive_bounds() {
    let config = PocketIcStartupConfig::Spawn {
        server_binary: "pocket-ic".into(),
        timeout: Duration::from_secs(1),
        server_hard_ttl: None,
        server_idle_ttl: Some(Duration::from_millis(1)),
        server_output_files: None,
    };
    assert!(matches!(
        config.validate().unwrap_err().failure(),
        PocketIcStartupFailure::InvalidConfiguration { .. }
    ));
    let error = PocketIcStartupConfig::Connect {
        server_url: ("http://127.0.0.1:1/").into(),
        timeout: Duration::ZERO,
    }
    .validate()
    .expect_err("zero startup timeout must fail");
    assert!(matches!(
        error.failure(),
        PocketIcStartupFailure::InvalidConfiguration { .. }
    ));

    let error = PocketIcStartupConfig::Spawn {
        server_binary: ("pocket-ic").into(),
        timeout: Duration::from_secs(1),
        server_hard_ttl: Some(Duration::from_millis(1)),
        server_idle_ttl: None,
        server_output_files: None,
    }
    .validate()
    .expect_err("subsecond server hard TTL must fail");
    assert!(matches!(
        error.failure(),
        PocketIcStartupFailure::InvalidConfiguration { .. }
    ));
}

#[test]
fn startup_files_leave_the_server_owned_port_path_absent() {
    let (files, stdout, stderr) = StartupFiles::create(None).expect("allocate startup files");
    let directory = files.directory.path().to_path_buf();

    assert!(directory.is_dir());
    assert!(!files.port.exists());
    assert!(files.stdout.is_file());
    assert!(files.stderr.is_file());
    #[cfg(unix)]
    {
        let mode = fs::metadata(&directory)
            .expect("inspect private startup directory")
            .permissions()
            .mode();
        assert_eq!(mode & 0o077, 0);
    }

    drop(stdout);
    drop(stderr);
    drop(files);
    assert!(!directory.exists());
}

#[cfg(unix)]
#[test]
fn caller_output_files_survive_startup_command_and_cancellation_cleanup() {
    for outcome in [
        "timeout",
        "startup-exit",
        "server-exit",
        "command-exit",
        "cancel",
        "success",
    ] {
        let (owner, _, _) = StartupFiles::create(None).unwrap();
        let stdout = owner.directory.path().join("retained-stdout");
        let stderr = owner.directory.path().join("retained-stderr");
        let ending = match outcome {
            "timeout" => "exec sleep 30",
            "startup-exit" => "exit 41",
            "server-exit" => {
                "printf '34567\\n' > \"$2\"; while [ ! -s \"$0.command\" ]; do sleep 0.02; done; exit 42"
            }
            _ => "printf '34567\\n' > \"$2\"; exec sleep 30",
        };
        let script = TestServerScript::new(
            outcome,
            &format!(
                "#!/bin/sh\nprintf '%s\\n%s\\n' \"$$\" \"$2\" > \"$0.pid\"\ndd if=/dev/zero bs=1024 count=20 2>/dev/null\nprintf raw-stdout-end\ndd if=/dev/zero bs=1024 count=20 >&2 2>/dev/null\nprintf raw-stderr-end >&2\n{ending}\n"
            ),
        );
        let pid_file = script.path().with_extension("pid");
        let command_file = script.path().with_extension("command");
        let config = PocketIcStartupConfig::Spawn {
            server_binary: script.path(),
            timeout: Duration::from_millis(if outcome == "timeout" { 1000 } else { 2000 }),
            server_hard_ttl: None,
            server_idle_ttl: None,
            server_output_files: Some(((&stdout).into(), (&stderr).into())),
        };
        if outcome == "timeout" || outcome == "startup-exit" {
            let error = config.start_managed_server().err().unwrap();
            match (outcome, error.failure()) {
                ("timeout", PocketIcStartupFailure::ReadinessTimeout { .. }) => {
                    assert!(error.output().stdout().contains("truncated"));
                    assert!(!error.output().stderr().contains("raw-stderr-end"));
                }
                ("startup-exit", PocketIcStartupFailure::ServerExited { status, .. }) => {
                    assert_eq!(status.code(), Some(41));
                }
                (_, error) => panic!("unexpected startup result: {error:?}"),
            }
        } else {
            let command_end = match outcome {
                "command-exit" => "exit 37",
                "success" => "exit 0",
                _ => "exec sleep 30",
            };
            let result = config.run_command(
                Command::new("/bin/sh")
                    .args([
                        "-c",
                        &format!("printf '%s' \"$$\" > \"$1\"; {command_end}"),
                        "fixture",
                    ])
                    .arg(&command_file),
                || outcome == "cancel" && fs::metadata(&command_file).is_ok_and(|m| m.len() > 0),
            );
            let result = result.as_ref().map_err(PocketIcStartupError::failure);
            match (outcome, result) {
                ("command-exit", Ok(status)) => assert_eq!(status.code(), Some(37)),
                ("success", Ok(status)) => assert!(status.success()),
                ("server-exit", Err(PocketIcStartupFailure::ServerExited { status, .. })) => {
                    assert_eq!(status.code(), Some(42));
                }
                ("cancel", Err(PocketIcStartupFailure::CommandRun { source, .. })) => {
                    assert_eq!(source.kind(), std::io::ErrorKind::Interrupted);
                }
                (_, result) => panic!("unexpected command result: {result:?}"),
            }
            let pid = fs::read_to_string(&command_file).unwrap().parse().unwrap();
            assert!(process_state(pid).is_none_or(|state| state == 'Z'));
        }
        for (path, suffix) in [(&stdout, b"raw-stdout-end"), (&stderr, b"raw-stderr-end")] {
            let bytes = fs::read(path).unwrap();
            assert_eq!(bytes.len(), 20 * 1024 + suffix.len());
            assert!(bytes.ends_with(suffix));
            assert_eq!(
                fs::metadata(path).unwrap().permissions().mode() & 0o777,
                0o600
            );
        }
        let report = fs::read_to_string(&pid_file).unwrap();
        let mut lines = report.lines();
        assert_eq!(process_state(lines.next().unwrap().parse().unwrap()), None);
        assert!(
            !PathBuf::from(lines.next().unwrap())
                .parent()
                .unwrap()
                .exists()
        );
        fs::remove_file(pid_file).unwrap();
    }
}

#[cfg(unix)]
#[test]
fn caller_output_files_refuse_existing_entries_and_keep_partial_preparation() {
    use std::os::unix::fs::symlink;

    let (owner, _, _) = StartupFiles::create(None).unwrap();
    let stdout = owner.directory.path().join("retained-stdout");
    let stderr = owner.directory.path().join("retained-stderr");
    let unrelated = owner.directory.path().join("unrelated");
    fs::write(&unrelated, b"original").unwrap();
    symlink(&unrelated, &stderr).unwrap();
    let error = StartupFiles::create(Some((stdout.clone(), stderr.clone())))
        .err()
        .unwrap();
    assert!(
        matches!(error.failure(), PocketIcStartupFailure::Io { source, .. } if source.kind() == std::io::ErrorKind::AlreadyExists)
    );
    assert!(stdout.is_file());
    assert_eq!(fs::read(&unrelated).unwrap(), b"original");
    fs::write(&stdout, b"retained attempt").unwrap();
    assert!(StartupFiles::create(Some((stdout.clone(), stderr.clone()))).is_err());
    assert_eq!(fs::read(&stdout).unwrap(), b"retained attempt");
    fs::remove_file(stderr.clone()).unwrap();
    ic_host_fs::testing::create_fifo(&stderr, None).unwrap();
    fs::remove_file(stdout.clone()).unwrap();
    assert!(StartupFiles::create(Some((stdout.clone(), stderr))).is_err());
    assert!(stdout.is_file());
}

#[cfg(unix)]
#[tokio::test]
async fn managed_startup_reports_an_exited_server_with_bounded_output() {
    let script = TestServerScript::new(
        "exit",
        "#!/bin/sh\nif [ \"$1\" != \"--port-file\" ] || [ -e \"$2\" ]; then exit 97; fi\nprintf 'synthetic server stdout'\nprintf 'synthetic bind failure' >&2\nexit 23\n",
    );

    let result = script
        .config(Duration::from_secs(2))
        .build_instance(
            PocketIcBuilder::new(std::time::Duration::from_secs(300)).with_application_subnet(),
        )
        .await;

    let error = result.err().unwrap();
    let PocketIcStartupFailure::ServerExited {
        server_binary,
        status,
        ..
    } = error.failure()
    else {
        panic!("expected a structured server exit, got {error:?}");
    };
    assert_eq!(server_binary, &script.path());
    assert_eq!(status.code(), Some(23));
    assert_eq!(error.output().stdout(), "synthetic server stdout");
    assert_eq!(error.output().stderr(), "synthetic bind failure");
}

#[cfg(unix)]
#[test]
fn managed_startup_rejects_oversized_port_files_and_cleans_up() {
    let script = TestServerScript::new(
        "oversized-port",
        "#!/bin/sh\nprintf '%s\\n%s\\n' \"$$\" \"$2\"\nprintf '34567\\n%064s' '' > \"$2.pending\"\nmv \"$2.pending\" \"$2\"\nexec sleep 30\n",
    );
    let result = script.config(Duration::from_secs(2)).start_managed_server();
    let error = result.err().unwrap();
    let PocketIcStartupFailure::InvalidServerPort { value, .. } = error.failure() else {
        panic!("oversized port publication must fail readiness");
    };
    assert!(value.contains("port file exceeds"));
    assert!(value.len() < 256);
    let mut lines = error.output().stdout().lines();
    let pid = lines.next().unwrap().parse::<u32>().unwrap();
    let port_path = PathBuf::from(lines.next().unwrap());
    assert!(!port_path.parent().unwrap().exists());
    assert_eq!(process_state(pid), None, "failed server must be reaped");
}

#[cfg(unix)]
#[tokio::test]
async fn managed_startup_terminates_a_server_that_never_becomes_ready() {
    let script = TestServerScript::new(
        "timeout",
        "#!/bin/sh\nif [ \"$1\" != \"--port-file\" ] || [ -e \"$2\" ]; then exit 97; fi\nexec sleep 30\n",
    );
    let timeout = Duration::from_millis(100);
    let started = Instant::now();

    let result = script
        .config(timeout)
        .build_instance(
            PocketIcBuilder::new(std::time::Duration::from_secs(300)).with_application_subnet(),
        )
        .await;

    assert!(
        started.elapsed() < Duration::from_secs(2),
        "bounded startup should not wait for the sleeping child"
    );
    let error = result.err().unwrap();
    assert!(
        matches!(error.failure(), PocketIcStartupFailure::ReadinessTimeout {
            server_binary, timeout: actual_timeout,
        } if server_binary == &script.path() && *actual_timeout == timeout)
    );
    assert!(error.server_cleanup().is_none());
}

#[cfg(unix)]
#[test]
fn managed_server_handle_exposes_process_id_url_output_and_raii_ownership() {
    let script = TestServerScript::new(
        "handle",
        "#!/bin/sh\nif [ \"$1\" != \"--port-file\" ] || [ -e \"$2\" ]; then echo 'unexpected managed server arguments' >&2; exit 97; fi\nprintf 'managed server ready: %s' \"$$\"\nprintf '34567\\n' > \"$2\"\nexec sleep 30\n",
    );

    let server = script
        .config(Duration::from_secs(2))
        .start_managed_server()
        .expect("start caller-owned managed server");

    assert_eq!(server.url(), "http://127.0.0.1:34567/");
    assert_eq!(
        server.output().stdout(),
        format!("managed server ready: {}", server.process_id())
    );
    assert_eq!(server.output().stderr(), "");
    let pid = server.process_id();
    assert!(process_state(pid).is_some_and(|state| state != 'Z'));
    drop(server);
    assert_eq!(process_state(pid), None, "owned server must be reaped");
}

#[cfg(unix)]
#[test]
fn managed_server_cleans_descendants_on_drop_timeout_and_exit() {
    for mode in ["drop", "timeout", "exit"] {
        let publish = if mode == "drop" {
            "printf '34567\\n' > \"$2\"\n"
        } else {
            ""
        };
        let finish = if mode == "exit" {
            "sleep 0.03\nexit 23\n"
        } else {
            "exec sleep 30\n"
        };
        let script = TestServerScript::new(
            mode,
            &format!("#!/bin/sh\nsleep 30 &\nprintf '%s' \"$!\"\n{publish}{finish}"),
        );
        let result = script
            .config(Duration::from_millis(300))
            .start_managed_server();
        let output = match result {
            Ok(server) => {
                let output = server.output().stdout().to_owned();
                drop(server);
                output
            }
            Err(error) => {
                match error.failure() {
                    PocketIcStartupFailure::ReadinessTimeout { .. } => {
                        assert_eq!(mode, "timeout");
                        assert!(error.server_cleanup().is_none());
                    }
                    PocketIcStartupFailure::ServerExited { status, .. } => {
                        assert_eq!(mode, "exit");
                        assert_eq!(status.code(), Some(23));
                    }
                    _ => panic!("unexpected {mode} startup result: {error}"),
                }
                error.output().stdout().to_owned()
            }
        };
        let pid = output
            .parse::<u32>()
            .expect("server published its descendant PID");
        let deadline = Instant::now() + Duration::from_secs(2);
        while process_state(pid).is_some_and(|state| state != 'Z') {
            assert!(
                Instant::now() < deadline,
                "{mode} left its descendant running"
            );
            std::thread::sleep(Duration::from_millis(10));
        }
    }
}

#[cfg(unix)]
#[test]
fn managed_server_passes_an_explicit_hard_ttl() {
    let script = TestServerScript::new(
        "hard-ttl",
        "#!/bin/sh\nif [ \"$1\" != \"--hard-ttl\" ] || [ \"$2\" != \"17\" ] || [ \"$3\" != \"--port-file\" ] || [ -e \"$4\" ]; then exit 97; fi\nprintf '34567\\n' > \"$4\"\nexec sleep 30\n",
    );

    let server = PocketIcStartupConfig::Spawn {
        server_binary: script.path(),
        timeout: Duration::from_secs(2),
        server_hard_ttl: Some(Duration::from_secs(17)),
        server_idle_ttl: None,
        server_output_files: None,
    }
    .start_managed_server()
    .expect("start managed server with an explicit hard TTL");

    assert_eq!(server.url(), "http://127.0.0.1:34567/");
}

#[cfg(unix)]
#[tokio::test]
#[ignore = "requires POCKET_IC_BIN=<caller-provided PocketIC server binary>"]
async fn caller_provided_server_publishes_port_constructs_instance_and_cleans_up() {
    let binary = std::env::var_os("POCKET_IC_BIN")
        .map(PathBuf::from)
        .expect("set POCKET_IC_BIN to the exact server binary");
    // Observe this exact child's port path without inspecting other startup
    // directories created concurrently.
    let wrapper = TestServerScript::new(
        "real-owned-instance",
        "#!/bin/sh\nprintf '%s\n%s\n' \"$$\" \"$4\" > \"$0.record\"\nexec \"$POCKET_IC_BIN\" \"$@\"\n",
    );
    let owned = PocketIcStartupConfig::Spawn {
        server_binary: wrapper.path(),
        timeout: Duration::from_secs(30),
        server_hard_ttl: Some(Duration::from_secs(60)),
        server_idle_ttl: None,
        server_output_files: None,
    }
    .build_instance(
        PocketIcBuilder::new(std::time::Duration::from_secs(300)).with_application_subnet(),
    )
    .await
    .expect("owned managed spawn must construct an instance");
    let record_file = wrapper.path().with_extension("record");
    let record = fs::read_to_string(&record_file).unwrap();
    let mut fields = record.lines();
    let pid = fields.next().unwrap().parse().unwrap();
    let port_path = PathBuf::from(fields.next().unwrap());
    let startup_directory = port_path.parent().unwrap();
    drop(owned);
    assert!(!startup_directory.exists());
    assert_eq!(process_state(pid), None);
    fs::remove_file(record_file).unwrap();

    let server = PocketIcStartupConfig::Spawn {
        server_binary: (&binary).into(),
        timeout: Duration::from_secs(30),
        server_hard_ttl: Some(Duration::from_secs(60)),
        server_idle_ttl: None,
        server_output_files: None,
    }
    .start_managed_server()
    .expect("caller-provided PocketIC server must publish its port");
    let files = &server.server.files;
    let startup_directory = files.directory.path().to_path_buf();

    assert!(files.port.is_file());
    let pocket_ic = PocketIcStartupConfig::Connect {
        server_url: (server.url()).into(),
        timeout: Duration::from_secs(30),
    }
    .build_instance(
        PocketIcBuilder::new(std::time::Duration::from_secs(300)).with_application_subnet(),
    )
    .await
    .expect("construct instance through caller-provided server");

    drop(pocket_ic);
    drop(server);
    assert!(!startup_directory.exists());
}

#[cfg(unix)]
struct TestServerScript {
    path: PathBuf,
    #[expect(
        dead_code,
        reason = "retains directory ownership until cleanup or drop"
    )]
    directory: OwnedDirectory,
}

#[cfg(unix)]
impl TestServerScript {
    fn new(label: &str, contents: &str) -> Self {
        let directory = OwnedDirectory::new(
            create_unique_directory(
                &std::env::temp_dir(),
                &format!("ic-testkit-pocket-ic-{label}"),
                0o700,
                None,
            )
            .expect("create test server directory"),
        );
        let path = directory.path().join("server");
        write_executable_script(&path, contents).expect("write executable test script");
        Self { path, directory }
    }

    fn path(&self) -> PathBuf {
        self.path.clone()
    }

    fn config(&self, timeout: Duration) -> PocketIcStartupConfig {
        PocketIcStartupConfig::Spawn {
            server_binary: self.path(),
            timeout,
            server_hard_ttl: None,
            server_idle_ttl: None,
            server_output_files: None,
        }
    }
}
