//! Resolve Darwin's zombie-only group EPERM without ignoring live members.
use std::process::Child;

use libproc::processes::{ProcFilter, pids_by_type};
use rustix::process::{Pid, Signal, WaitId, WaitIdOptions, kill_process, waitid};

pub(super) fn exited_group(child: &Child) -> bool {
    // Keep the leader unreaped throughout: its PID still pins group identity.
    // Darwin can exclude an exiting leader before WNOHANG observes its status.
    // Perform the same owned-child fallback as ordinary cleanup, then observe
    // terminal status without reaping before deciding whether the group is empty.
    match kill_process(Pid::from_child(child), Signal::KILL) {
        Ok(()) | Err(rustix::io::Errno::SRCH) => {}
        Err(_) => return false,
    }
    if !matches!(
        waitid(
            WaitId::Pid(Pid::from_child(child)),
            WaitIdOptions::EXITED | WaitIdOptions::NOWAIT,
        ),
        Ok(Some(_))
    ) {
        return false;
    }
    // libproc's zero-result convention consults errno. Do not let the original
    // kill EPERM masquerade as an enumeration failure after a successful query.
    errno::set_errno(errno::Errno(0));
    pids_by_type(ProcFilter::ByProgramGroup { pgrpid: child.id() })
        .is_ok_and(|members| members.iter().all(|pid| *pid == child.id()))
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{Ownership, Session};
    use std::os::unix::process::CommandExt;
    use std::process::Command;
    use std::time::{Duration, Instant};

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
}
