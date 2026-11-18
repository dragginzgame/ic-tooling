use super::*;
use crate::{Ownership, Session};
use std::{
    os::unix::process::CommandExt,
    process::Command,
    time::{Duration, Instant},
};

#[test]
fn exited_leader_is_distinguished_from_live_group_members() {
    for script in ["exit 0", "sleep 30 & exit 0"] {
        let child = Command::new("/bin/sh")
            .args(["-c", script])
            .process_group(0)
            .spawn()
            .unwrap();
        let mut owner = Session {
            child: Some(child),
            ownership: Ownership::Group,
        };
        let child = owner.child.as_ref().unwrap();
        let started = Instant::now();
        while waitid(
            WaitId::Pid(Pid::from_child(child)),
            WaitIdOptions::EXITED | WaitIdOptions::NOHANG | WaitIdOptions::NOWAIT,
        )
        .unwrap()
        .is_none()
        {
            assert!(started.elapsed() < Duration::from_secs(5));
            std::thread::sleep(Duration::from_millis(1));
        }
        assert_eq!(exited_group(child), script == "exit 0");
        let (status, cleanup) = owner.close();
        assert!(status.unwrap().success());
        assert!(cleanup.is_empty(), "{cleanup:?}");
    }
}
