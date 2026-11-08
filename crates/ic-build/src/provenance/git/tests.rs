use super::*;
use ic_host_process::tool::OutputLimit;
use std::{fs, time::Duration};
fn git(root: &Path, args: &[&str]) {
    assert!(
        Command::new("git")
            .current_dir(root)
            .args(args)
            .status()
            .unwrap()
            .success()
    );
}
#[test]
fn exact_root_clean_dirty_and_inherited_override_refusal() {
    let root = crate::testing::fs::unique_temp_directory("git-observe");
    git(&root, &["init", "--quiet"]);
    git(
        &root,
        &[
            "-c",
            "user.name=Fixture",
            "-c",
            "user.email=fixture@example.invalid",
            "commit",
            "--allow-empty",
            "--quiet",
            "-m",
            "fixture",
        ],
    );
    let limits = OutputLimits {
        stdout: OutputLimit::Terminate(65536),
        stderr: OutputLimit::Terminate(65536),
        timeout: Some(Duration::from_secs(10)),
    };
    let factory = || {
        let mut c = Command::new("git");
        c.env("GIT_DIR", "/missing")
            .env("GIT_WORK_TREE", "/missing");
        c
    };
    let clean = observe(&root, factory, limits).unwrap();
    assert_eq!(clean.status, Vec::<u8>::new());
    fs::write(root.join("untracked"), b"value").unwrap();
    assert_ne!(
        observe(&root, factory, limits).unwrap().status,
        Vec::<u8>::new()
    );
    fs::create_dir(root.join("child")).unwrap();
    assert!(observe(&root.join("child"), factory, limits).is_err());
    let tiny = OutputLimits {
        stdout: OutputLimit::Terminate(1),
        ..limits
    };
    assert!(observe(&root, factory, tiny).is_err());
    fs::remove_dir_all(root).unwrap();
}
