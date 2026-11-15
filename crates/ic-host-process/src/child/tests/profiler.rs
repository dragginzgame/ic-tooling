use super::*;
use std::ffi::OsStr;

#[test]
fn relinquishing_a_running_child_never_kills_it_on_drop() {
    let _fixture = Fixture::new();
    let mut command = command("exec sleep 30");
    let mut child = OwnedChild::spawn(&mut command, CleanupPolicy::KillAndWait).unwrap();
    let pid = child.id();
    assert!(child.relinquish().unwrap().is_none());
    assert!(child.poll_exit().is_err());
    drop(child);
    let pid = Pid::from_raw(i32::try_from(pid).unwrap()).unwrap();
    assert!(
        waitid(
            WaitId::Pid(pid),
            WaitIdOptions::EXITED | WaitIdOptions::NOHANG
        )
        .unwrap()
        .is_none()
    );
    kill_process_group(pid, Signal::KILL).unwrap();
    waitid(WaitId::Pid(pid), WaitIdOptions::EXITED).unwrap();
}

#[test]
fn relinquishing_an_exited_child_reaps_without_killing_its_group() {
    let fixture = Fixture::new();
    let marker = fixture.path().join("alive");
    let mut cmd = command("(sleep 0.1; printf alive > \"$MARKER\") & exit 7");
    cmd.env("MARKER", &marker);
    let mut child = OwnedChild::spawn(&mut cmd, CleanupPolicy::KillAndWait).unwrap();
    until(|| child.poll_exit().unwrap().is_some());
    assert_eq!(child.relinquish().unwrap().unwrap().code(), Some(7));
    assert_reaped(child.id());
    drop(child);
    until(|| std::fs::read(&marker).is_ok_and(|bytes| bytes == b"alive"));
}

#[test]
fn graceful_cleanup_preserves_the_final_unreaped_observation() {
    let _fixture = Fixture::new();
    let mut child = OwnedChild::spawn(&mut command("exit 7"), CleanupPolicy::KillAndWait).unwrap();
    until(|| child.poll_exit().unwrap().is_some());
    let pid = child.id();
    #[cfg(target_os = "linux")]
    assert!(crate::observation::basic_process(pid).is_some());
    let status = child
        .terminate_with_grace(Duration::from_millis(500))
        .unwrap();
    assert_eq!(status.code(), Some(7));
    assert_reaped(pid);
}

#[test]
fn graceful_cleanup_kills_a_group_that_ignores_term() {
    let _fixture = Fixture::new();
    let mut cmd = command("trap '' TERM; printf ready; while :; do sleep 1; done");
    cmd.stdout(Stdio::piped());
    let mut child = OwnedChild::spawn(&mut cmd, CleanupPolicy::KillAndWait).unwrap();
    let mut stdout = child.take_stdout().unwrap();
    let mut ready = [0; 5];
    stdout.read_exact(&mut ready).unwrap();
    let started = Instant::now();
    assert!(
        !child
            .terminate_with_grace(Duration::from_millis(25))
            .unwrap()
            .success()
    );
    assert!(started.elapsed() >= Duration::from_millis(25));
    assert_reaped(child.id());
}

#[test]
fn launch_gate_preserves_stdin_environment_and_exact_argument_bytes() {
    let _fixture = Fixture::new();
    let args = [
        "-c",
        "read value; printf '%s|%s|%s' \"$value\" \"$1\" \"$SELECTED\"",
        "fixture",
        "literal $() and spaces",
    ]
    .map(Into::into);
    let (mut cmd, mut gate) =
        gated_command(OsStr::new("/bin/sh"), &args, OsStr::new("profiler-fixture")).unwrap();
    cmd.env("SELECTED", "kept")
        .stdin(Stdio::piped())
        .stdout(Stdio::piped());
    let mut child = OwnedChild::spawn(&mut cmd, CleanupPolicy::KillAndWait).unwrap();
    let mut stdin = child.take_stdin().unwrap();
    stdin.write_all(b"input\n").unwrap();
    drop(stdin);
    std::thread::sleep(Duration::from_millis(20));
    assert!(child.poll_exit().unwrap().is_none());
    gate.write_all(b"start\n").unwrap();
    drop(gate);
    let mut result = Vec::new();
    child
        .take_stdout()
        .unwrap()
        .read_to_end(&mut result)
        .unwrap();
    assert!(child.wait().unwrap().success());
    assert_eq!(result, b"input|literal $() and spaces|kept");
}

#[test]
fn closing_the_gate_refuses_workload_execution() {
    let fixture = Fixture::new();
    let marker = fixture.path().join("must-not-exist");
    let args = ["-c".into(), "touch \"$MARKER\"".into()];
    let (mut cmd, gate) =
        gated_command(OsStr::new("/bin/sh"), &args, OsStr::new("gate-fixture")).unwrap();
    cmd.env("MARKER", &marker);
    let mut child = OwnedChild::spawn(&mut cmd, CleanupPolicy::KillAndWait).unwrap();
    drop(gate);
    assert_eq!(child.wait().unwrap().code(), Some(125));
    assert!(!marker.exists());
}
