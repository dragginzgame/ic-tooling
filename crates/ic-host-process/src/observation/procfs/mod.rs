//! Linux procfs observations. Callers select a mount and own read budgets.
use super::{ChildProcesses, ProcessObservation};
use std::{collections::BTreeSet, fs, path::Path, sync::OnceLock};

fn ticks_per_second() -> u64 {
    static VALUE: OnceLock<u64> = OnceLock::new();
    *VALUE.get_or_init(|| {
        #[cfg(unix)]
        return positive_sysconf(libc::_SC_CLK_TCK, 100);
        #[cfg(not(unix))]
        100
    })
}

fn page_size() -> u64 {
    static VALUE: OnceLock<u64> = OnceLock::new();
    *VALUE.get_or_init(|| {
        #[cfg(unix)]
        return positive_sysconf(libc::_SC_PAGESIZE, 4096);
        #[cfg(not(unix))]
        4096
    })
}

#[cfg(unix)]
#[expect(
    unsafe_code,
    reason = "read signed sysconf results for the existing fallback contract"
)]
fn positive_sysconf(name: libc::c_int, fallback: u64) -> u64 {
    // Keep the signed result: Rustix's unsigned parameter helpers cannot retain
    // sysconf's negative failure sentinel for this existing fallback contract.
    // SAFETY: both callers use defined sysconf names; no pointers are involved.
    let value = unsafe { libc::sysconf(name) };
    if value > 0 {
        u64::try_from(value).ok().unwrap_or_default()
    } else {
        fallback
    }
}

/// Read the Linux creation token from a caller-selected procfs mount.
#[must_use]
pub fn process_identity(proc_root: &Path, pid: u32) -> Option<u64> {
    let stat = fs::read_to_string(proc_root.join(pid.to_string()).join("stat")).ok()?;
    let (_, fields) = stat_parts(&stat)?;
    let mut fields = fields.split_whitespace();
    fields.next()?; // state
    fields.next()?.parse::<u64>().ok()?; // preserve full-stat PPID validation
    fields.nth(17)?.parse().ok() // starttime is field 19 after the command
}

/// Read base Linux process facts from a caller-selected procfs mount.
#[must_use]
pub fn basic_process(proc_root: &Path, pid: u32) -> Option<ProcessObservation> {
    let stat = fs::read_to_string(proc_root.join(pid.to_string()).join("stat")).ok()?;
    let (name, fields) = parse_stat(&stat)?;
    let number = |index: usize| fields.get(index)?.parse::<u64>().ok();
    let ns = |index| number(index).unwrap_or(0).saturating_mul(1_000_000_000) / ticks_per_second();
    Some(ProcessObservation {
        pid,
        ppid: u32::try_from(number(1)?).ok()?,
        start_ticks: number(19)?,
        name: name.to_owned(),
        user_ns: ns(11),
        system_ns: ns(12),
        waited_user_ns: ns(13),
        waited_system_ns: ns(14),
        rss_bytes: number(21).unwrap_or(0).saturating_mul(page_size()),
        pss_bytes: None,
        read_bytes: None,
        write_bytes: None,
        minor_faults: number(7)
            .unwrap_or(0)
            .saturating_add(number(8).unwrap_or(0)),
        major_faults: number(9)
            .unwrap_or(0)
            .saturating_add(number(10).unwrap_or(0)),
        threads: number(17).unwrap_or(0),
    })
}

fn stat_parts(stat: &str) -> Option<(&str, &str)> {
    let begin = stat.find('(')?;
    let end = stat.rfind(')')?;
    Some((stat.get(begin + 1..end)?, stat.get(end + 1..)?))
}

fn parse_stat(stat: &str) -> Option<(&str, [&str; 22])> {
    let (name, fields) = stat_parts(stat)?;
    let mut fields = fields.split_whitespace();
    // Only the first 22 fields are used. Keep them borrowed and on the stack;
    // samples and identity checks need neither the rest nor a temporary Vec.
    Some((name, std::array::from_fn(|_| fields.next().unwrap_or(""))))
}

fn field_value(text: &str, name: &str) -> Option<u64> {
    text.lines()
        .filter_map(|line| {
            let mut fields = line.split_whitespace();
            if fields.next()?.trim_end_matches(':') != name {
                return None;
            }
            fields.next()?.parse().ok()
        })
        .next_back()
}

/// Read Linux I/O and optionally PSS for an already selected observation.
/// This does not revalidate identity; consumers retain before/after checks.
pub fn read_details(proc_root: &Path, result: &mut ProcessObservation, memory_detail: bool) {
    let io = fs::read_to_string(proc_root.join(result.pid.to_string()).join("io")).ok();
    result.read_bytes = io
        .as_deref()
        .and_then(|text| field_value(text, "read_bytes"));
    result.write_bytes = io
        .as_deref()
        .and_then(|text| field_value(text, "write_bytes"));
    if memory_detail {
        result.pss_bytes =
            fs::read_to_string(proc_root.join(result.pid.to_string()).join("smaps_rollup"))
                .ok()
                .and_then(|text| field_value(&text, "Pss"))
                .map(|kb| kb.saturating_mul(1024));
    }
}
/// Read all thread child lists from an explicitly selected procfs mount.
#[must_use]
pub fn children(proc_root: &Path, pid: u32) -> ChildProcesses {
    let mut result = ChildProcesses::default();
    // A Linux task's children file only lists children forked by that task.
    // Inspect every thread, including nonleaders that spawn sandboxes/adapters.
    // Never enumerate the top-level /proc directory to discover unrelated PIDs.
    let tasks = proc_root.join(pid.to_string()).join("task");
    let Ok(entries) = fs::read_dir(&tasks) else {
        result.unavailable = true;
        return result;
    };
    let mut children = BTreeSet::new();
    let mut saw_task = false;
    for entry in entries {
        let Ok(entry) = entry else {
            result.incomplete = true;
            continue;
        };
        if entry
            .file_name()
            .to_str()
            .and_then(|s| s.parse::<u32>().ok())
            .is_none()
        {
            continue;
        }
        saw_task = true;
        match fs::read_to_string(entry.path().join("children")) {
            Ok(value) => {
                for child in value.split_whitespace() {
                    if let Ok(child) = child.parse::<u32>() {
                        children.insert(child);
                    } else {
                        result.incomplete = true;
                    }
                }
            }
            Err(_) => {
                // A thread can disappear during the scan; even that race can
                // hide children reparented to a previously inspected thread.
                result.incomplete = true;
            }
        }
    }
    if !saw_task {
        result.unavailable = true;
    }
    result.pids = children.into_iter().collect();
    result
}

#[cfg(all(test, unix))]
mod tests;
