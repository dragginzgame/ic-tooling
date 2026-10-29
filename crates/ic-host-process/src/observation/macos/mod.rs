//! Darwin host observations through libc's libproc and Mach interfaces.
//! Linux-only counters remain explicitly unavailable.
#![allow(
    unsafe_code,
    reason = "reviewed libproc and Mach observations imported from Profiler"
)]

use super::{ChildProcesses, ProcessObservation};
use std::mem::{MaybeUninit, size_of};
use std::sync::OnceLock;

fn bsd_info(pid: u32) -> Option<libc::proc_bsdinfo> {
    let pid = i32::try_from(pid).ok()?;
    let mut info = MaybeUninit::<libc::proc_bsdinfo>::uninit();
    // SAFETY: the flavor and buffer size match proc_bsdinfo; read only on a
    // full successful write. libproc does not retain the output pointer.
    let written = unsafe {
        libc::proc_pidinfo(
            pid,
            libc::PROC_PIDTBSDINFO,
            0,
            info.as_mut_ptr().cast(),
            i32::try_from(size_of::<libc::proc_bsdinfo>()).ok()?,
        )
    };
    (usize::try_from(written).ok()? == size_of::<libc::proc_bsdinfo>())
        .then(|| unsafe { info.assume_init() })
}

fn creation(info: &libc::proc_bsdinfo) -> Option<u64> {
    info.pbi_start_tvsec
        .checked_mul(1_000_000)?
        .checked_add(info.pbi_start_tvusec)
}

pub(super) fn identity(pid: u32) -> Option<u64> {
    creation(&bsd_info(pid)?)
}

// libc still supplies this ABI. Keep the existing integration dependency graph
// rather than adding a second Mach binding solely for its timebase struct.
#[allow(deprecated)]
fn mach_ns(ticks: u64) -> Option<u64> {
    static SCALE: OnceLock<Option<(u32, u32)>> = OnceLock::new();
    let (numer, denom) = (*SCALE.get_or_init(|| {
        let mut scale = MaybeUninit::<libc::mach_timebase_info_data_t>::uninit();
        // SAFETY: writable output of the ABI's declared type and size.
        if unsafe { libc::mach_timebase_info(scale.as_mut_ptr()) } != libc::KERN_SUCCESS {
            return None;
        }
        let scale = unsafe { scale.assume_init() };
        (scale.denom != 0).then_some((scale.numer, scale.denom))
    }))?;
    Some(
        u64::try_from(u128::from(ticks) * u128::from(numer) / u128::from(denom))
            .unwrap_or(u64::MAX),
    )
}

pub(super) fn process(pid: u32) -> Option<ProcessObservation> {
    let before = identity(pid)?;
    let mut info = MaybeUninit::<libc::proc_taskallinfo>::uninit();
    // SAFETY: flavor/size match proc_taskallinfo; no read until the entire
    // structure was written. The output pointer is not retained.
    let written = unsafe {
        libc::proc_pidinfo(
            i32::try_from(pid).ok()?,
            libc::PROC_PIDTASKALLINFO,
            0,
            info.as_mut_ptr().cast(),
            i32::try_from(size_of::<libc::proc_taskallinfo>()).ok()?,
        )
    };
    if usize::try_from(written).ok()? != size_of::<libc::proc_taskallinfo>() {
        return None;
    }
    let info = unsafe { info.assume_init() };
    if info.pbsd.pbi_pid != pid || creation(&info.pbsd)? != before || identity(pid)? != before {
        return None;
    }
    let name = if info.pbsd.pbi_name[0] != 0 {
        &info.pbsd.pbi_name[..]
    } else {
        &info.pbsd.pbi_comm[..]
    };
    let name: Vec<_> = name
        .iter()
        .take_while(|ch| **ch != 0)
        .map(|ch| ch.to_ne_bytes()[0])
        .collect();
    Some(ProcessObservation {
        pid,
        ppid: info.pbsd.pbi_ppid,
        start_ticks: before,
        name: String::from_utf8_lossy(&name).into_owned(),
        // proc_taskinfo exposes Mach absolute-time units, including on arm64.
        user_ns: mach_ns(info.ptinfo.pti_total_user)?,
        system_ns: mach_ns(info.ptinfo.pti_total_system)?,
        rss_bytes: info.ptinfo.pti_resident_size,
        threads: u64::try_from(info.ptinfo.pti_threadnum.max(0)).ok()?,
        ..Default::default()
    })
}

pub(super) fn children(pid: u32, capacity: usize) -> ChildProcesses {
    let mut result = ChildProcesses::default();
    let Ok(pid) = i32::try_from(pid) else {
        return result;
    };
    if capacity == 0 {
        result.truncated = true;
        return result;
    }
    let capacity = capacity.min(usize::try_from(i32::MAX).unwrap_or(usize::MAX) / size_of::<i32>());
    let mut buffer = vec![0_i32; 256.min(capacity)];
    loop {
        // SAFETY: buffer contains capacity for the supplied byte count;
        // proc_listchildpids returns a PID count, not a byte count.
        let count = unsafe {
            libc::proc_listchildpids(
                pid,
                buffer.as_mut_ptr().cast(),
                i32::try_from(buffer.len() * size_of::<i32>())
                    .expect("capacity is bounded by the native byte count"),
            )
        };
        if count < 0 {
            result.unavailable = true;
            return result;
        }
        let count = usize::try_from(count).expect("negative native counts were refused");
        if count >= buffer.len() && buffer.len() < capacity {
            buffer.resize((buffer.len() * 2).min(capacity), 0);
            continue;
        }
        if count >= buffer.len() {
            result.truncated = true;
        }
        result.pids = buffer
            .into_iter()
            .take(count)
            .filter_map(|pid| u32::try_from(pid).ok())
            .filter(|pid| *pid != 0)
            .collect();
        return result;
    }
}
