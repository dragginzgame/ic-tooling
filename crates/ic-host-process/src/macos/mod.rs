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
mod tests;
