use super::*;
#[cfg(target_os = "linux")]
use std::fs;
use std::{
    cell::Cell,
    os::unix::process::ExitStatusExt,
    time::{Duration, Instant},
};

#[test]
fn setup_and_spawn_failures_keep_original_errors_without_child_effects() {
    let directory = tempfile::tempdir().unwrap();
    let marker = directory.path().join("effect");
    let mut command = Command::new("/bin/sh");
    command.args(["-c", "touch \"$1\"", "fixture"]).arg(&marker);
    let error = launch_with(command, |job| {
        drop(job);
        Err(io::Error::from_raw_os_error(12))
    })
    .err()
    .unwrap();
    assert_eq!(error.raw_os_error(), Some(12));
    assert!(!marker.exists());
    let error = launch(Command::new(directory.path().join("absent")))
        .err()
        .unwrap();
    assert_eq!(error.kind(), io::ErrorKind::NotFound);
}

#[test]
fn nonzero_completion_is_cached_and_wait_consumes_the_same_status() {
    let mut command = Command::new("/bin/sh");
    command.args(["-c", "exit 7"]);
    let mut completion = launch(command).unwrap();
    let deadline = Instant::now() + Duration::from_secs(5);
    while completion.try_result().is_none() {
        assert!(Instant::now() < deadline);
        std::thread::sleep(Duration::from_millis(2));
    }
    for _ in 0..2 {
        assert_eq!(
            completion.try_result().unwrap().as_ref().unwrap().code(),
            Some(7)
        );
    }
    assert_eq!(completion.wait().unwrap().code(), Some(7));
}

#[test]
fn interrupted_wait_retries_only_wait_and_preserves_terminal_error() {
    let spawns = Cell::new(0);
    let waits = Cell::new(0);
    let (startup, ready) = mpsc::channel();
    let (done, receiver) = mpsc::channel();
    work(
        || {
            spawns.set(spawns.get() + 1);
            Ok(())
        },
        |_| {
            waits.set(waits.get() + 1);
            if waits.get() == 1 {
                Err(io::ErrorKind::Interrupted.into())
            } else {
                Err(io::Error::from_raw_os_error(13))
            }
        },
        &startup,
        &done,
    );
    ready.recv().unwrap().unwrap();
    let mut completion = Completion {
        receiver,
        observed: None,
    };
    for _ in 0..2 {
        assert_eq!(
            completion
                .try_result()
                .unwrap()
                .as_ref()
                .unwrap_err()
                .raw_os_error(),
            Some(13)
        );
    }
    assert_eq!(completion.wait().unwrap_err().raw_os_error(), Some(13));
    assert_eq!((spawns.get(), waits.get()), (1, 2));
}

#[test]
fn lost_receivers_do_not_skip_reaping_or_repeat_spawn() {
    let (startup, ready) = mpsc::channel();
    let (done, receiver) = mpsc::channel();
    drop(ready);
    drop(receiver);
    let spawns = Cell::new(0);
    let waits = Cell::new(0);
    work(
        || {
            spawns.set(spawns.get() + 1);
            Ok(())
        },
        |_| {
            waits.set(waits.get() + 1);
            Ok(ExitStatus::from_raw(0))
        },
        &startup,
        &done,
    );
    assert_eq!((spawns.get(), waits.get()), (1, 1));
}

#[test]
fn worker_disconnect_is_uncertain_completion_not_success() {
    let (done, receiver) = mpsc::channel();
    drop(done);
    let completion = Completion {
        receiver,
        observed: None,
    };
    assert!(
        completion
            .wait()
            .unwrap_err()
            .to_string()
            .contains("unknown")
    );
    let error = launch_with(Command::new("/bin/false"), |job| {
        drop(job);
        Ok(())
    })
    .err()
    .unwrap();
    assert!(error.to_string().contains("unknown"));
}

#[cfg(target_os = "linux")]
#[test]
fn dropped_observer_leaves_child_running_and_worker_reaps_it() {
    fn identity(pid: u32) -> Option<String> {
        match fs::read_to_string(format!("/proc/{pid}/stat")) {
            Ok(stat) => Some(
                stat.rsplit_once(')')
                    .unwrap()
                    .1
                    .split_whitespace()
                    .nth(19)
                    .unwrap()
                    .to_owned(),
            ),
            Err(error) if error.kind() == io::ErrorKind::NotFound => None,
            Err(error) => panic!("cannot observe process identity: {error}"),
        }
    }
    let directory = tempfile::tempdir().unwrap();
    let url = "http://127.0.0.1/?literal=$(false);two words";
    let mut command = Command::new("/bin/sh");
    command
        .args([
            "-c",
            r#"
        printf '%s' "$$" > "$1/pid"
        count=0
        while test ! -f "$1/gate"; do
            count=$((count + 1))
            if test "$count" -ge 300; then exit 92; fi
            sleep 0.01
        done
        printf '%s' "$2" > "$1/witness"
    "#,
            "fixture",
        ])
        .arg(directory.path())
        .arg(url);
    let completion = launch(command).unwrap();
    let deadline = Instant::now() + Duration::from_secs(5);
    let pid = loop {
        if let Some(pid) = fs::read_to_string(directory.path().join("pid"))
            .ok()
            .and_then(|text| text.parse::<u32>().ok())
        {
            break pid;
        }
        assert!(Instant::now() < deadline);
        std::thread::sleep(Duration::from_millis(2));
    };
    let original = identity(pid).expect("child must still be waiting for gate");
    drop(completion);
    fs::write(directory.path().join("gate"), b"continue").unwrap();
    while identity(pid).as_ref() == Some(&original) {
        assert!(
            Instant::now() < deadline,
            "child remained present, including as a zombie"
        );
        std::thread::sleep(Duration::from_millis(2));
    }
    assert_eq!(
        fs::read(directory.path().join("witness")).unwrap(),
        url.as_bytes()
    );
}
