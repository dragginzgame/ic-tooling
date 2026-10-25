//! Two direct test workers coordinated through one caller-owned fixture root.

use ic_host_process::testing::crash_barrier::{wait_for_child_path, wait_for_path};
use std::{fs, path::Path, process::Command, time::Duration};

const WORKERS: [&str; 2] = ["first", "second"];
const TIMEOUT: Duration = Duration::from_secs(10);

pub fn run_workers(root: &Path, test: &str, root_env: &str, worker_env: &str) {
    let executable = std::env::current_exe().expect("resolve process-test executable");
    let mut children = WORKERS.map(|worker| {
        Command::new(&executable)
            .args(["--ignored", "--exact", test])
            .env(root_env, root)
            .env(worker_env, worker)
            .spawn()
            .expect("spawn process-test worker")
    });
    for (child, worker) in children.iter_mut().zip(WORKERS) {
        wait_for_child_path(
            child,
            &root.join(format!("ready-{worker}")),
            worker,
            TIMEOUT,
        )
        .expect("wait for worker readiness");
    }
    fs::write(root.join("go"), b"go").expect("release process-test workers");
    for child in &mut children {
        assert!(
            child
                .wait()
                .expect("wait for process-test worker")
                .success()
        );
    }
}

pub fn await_parent(root: &Path, worker: &str) {
    fs::write(root.join(format!("ready-{worker}")), b"ready").expect("mark worker ready");
    wait_for_path(&root.join("go"), "parent start marker", TIMEOUT).expect("wait for parent start");
}
