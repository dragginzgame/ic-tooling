//! Bounded process/lock observations extracted from Canic bd75bdecc.
//! Callers own limits, workload selection, metadata, redaction and lock custody.

#[cfg(test)]
mod tests;
use ic_host_artifacts::artifact::read_reader;
#[cfg(target_os = "linux")]
use std::os::unix::fs::MetadataExt;
use std::{fs, path::Path};

/// Same-host process identity. An observation grants no ownership.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ProcessIdentity {
    /// Linux boot identity.
    pub boot_id: String,
    /// PID namespace identity, matching the inspector.
    pub pid_namespace: String,
    /// Process birth ticks.
    pub start_ticks: u64,
}
/// Raw process sample. Names and states require consumer interpretation.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ProcessSample {
    /// Process identifier.
    pub pid: u32,
    /// Process birth ticks.
    pub start_ticks: u64,
    /// Raw command name, without arguments or environment.
    pub name: String,
    /// Raw scheduler state.
    pub state: char,
    /// User plus system CPU ticks.
    pub cpu_ticks: u64,
}
/// Global process/thread and per-thread child traversal budgets.
#[derive(Clone, Copy, Debug)]
pub struct TreeLimits {
    /// Maximum distinct processes visited.
    pub processes: usize,
    /// Maximum task records visited across the entire tree.
    pub threads: usize,
    /// Maximum child identifiers admitted per thread.
    pub children_per_thread: usize,
}
/// Kernel lock observation, never authority to remove a lock.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum KernelLock {
    /// A matching write flock was observed.
    Held {
        /// Observed owner PID.
        pid: u32,
    },
    /// No matching flock appeared in the bounded snapshot.
    NotObserved,
}
fn bounded_text(path: &Path, limit: u64) -> Option<String> {
    let bytes = read_reader(fs::File::open(path).ok()?, usize::try_from(limit).ok()?).ok()?;
    String::from_utf8(bytes).ok()
}

/// Read boot, PID namespace and process birth identity; foreign namespaces refuse.
#[must_use]
pub fn process_identity(proc_root: &Path, pid: u32) -> Option<ProcessIdentity> {
    let process = read_process(proc_root, pid)?;
    let boot_id = bounded_text(&proc_root.join("sys/kernel/random/boot_id"), 64)?
        .trim()
        .to_string();
    if boot_id.len() != 36
        || !boot_id
            .bytes()
            .all(|ch| ch.is_ascii_hexdigit() || ch == b'-')
    {
        return None;
    }
    let pid_namespace = fs::read_link(proc_root.join(format!("{pid}/ns/pid")))
        .ok()?
        .into_os_string()
        .into_string()
        .ok()?;
    let inspector_namespace = fs::read_link(proc_root.join("self/ns/pid")).ok()?;
    if Path::new(&pid_namespace) != inspector_namespace || pid_namespace.len() > 64 {
        return None;
    }
    Some(ProcessIdentity {
        boot_id,
        pid_namespace,
        start_ticks: process.start_ticks,
    })
}

fn read_process(proc_root: &Path, pid: u32) -> Option<ProcessSample> {
    parse_process(
        pid,
        &bounded_text(&proc_root.join(format!("{pid}/stat")), 4096)?,
    )
}

fn parse_process(pid: u32, stat: &str) -> Option<ProcessSample> {
    let (identity, _) = stat.rsplit_once(") ")?;
    let (number, _) = identity.split_once(" (")?;
    let (name, fields) = super::procfs::parse_stat(stat)?;
    if number.parse::<u32>().ok()? != pid {
        return None;
    }
    Some(ProcessSample {
        pid,
        start_ticks: fields.get(19)?.parse().ok()?,
        name: name.to_owned(),
        state: fields.first()?.chars().next()?,
        cpu_ticks: fields
            .get(11)?
            .parse::<u64>()
            .ok()?
            .checked_add(fields.get(12)?.parse().ok()?)?,
    })
}

// Global budgets bound traversal even for large compiler process trees.
/// Observe a bounded descendant snapshot; false means incomplete, never unhealthy.
#[must_use]
pub fn process_tree(
    proc_root: &Path,
    owner: u32,
    limits: TreeLimits,
) -> (Vec<ProcessSample>, bool) {
    let mut pending = vec![owner];
    let mut seen = std::collections::BTreeSet::new();
    let mut result = Vec::new();
    let mut complete = true;
    let mut threads_left = limits.threads;
    while let Some(pid) = pending.pop() {
        if !seen.insert(pid) {
            continue;
        }
        if seen.len() > limits.processes {
            complete = false;
            break;
        }
        let Some(before) = read_process(proc_root, pid) else {
            complete = false;
            continue;
        };
        let Ok(tasks) = fs::read_dir(proc_root.join(format!("{pid}/task"))) else {
            complete = false;
            result.push(before);
            continue;
        };
        let mut children = Vec::new();
        for task in tasks {
            if threads_left == 0 {
                complete = false;
                break;
            }
            threads_left -= 1;
            let Some(text) = task
                .ok()
                .and_then(|task| bounded_text(&task.path().join("children"), 4096))
            else {
                complete = false;
                continue;
            };
            for (index, child) in text.split_ascii_whitespace().enumerate() {
                if index >= limits.children_per_thread {
                    complete = false;
                    break;
                }
                if let Ok(child) = child.parse::<u32>() {
                    children.push(child);
                } else {
                    complete = false;
                }
            }
        }
        if read_process(proc_root, pid).is_some_and(|after| after.start_ticks == before.start_ticks)
        {
            result.push(before);
            pending.extend(children);
        } else {
            complete = false;
        }
    }
    result.sort_by_key(|process| process.pid);
    (result, complete)
}

#[cfg(target_os = "linux")]
/// Observe an opened inode's Linux flock holder; absence never proves unlock.
#[must_use]
pub fn kernel_holder(
    file: &fs::File,
    proc_root: &Path,
    maximum_bytes: usize,
) -> Option<KernelLock> {
    let metadata = file.metadata().ok()?;
    let locks = bounded_text(&proc_root.join("locks"), u64::try_from(maximum_bytes).ok()?)?;
    Some(parse_kernel_holder(&locks, metadata.dev(), metadata.ino()))
}

#[cfg(not(target_os = "linux"))]
/// Observe an opened inode's Linux flock holder; absence never proves unlock.
#[must_use]
pub fn kernel_holder(
    _file: &fs::File,
    _proc_root: &Path,
    _maximum_bytes: usize,
) -> Option<KernelLock> {
    None
}

#[cfg(target_os = "linux")]
fn parse_kernel_holder(locks: &str, device: u64, inode: u64) -> KernelLock {
    for line in locks.lines() {
        let fields = line.split_ascii_whitespace().collect::<Vec<_>>();
        if fields.get(1) != Some(&"FLOCK") || fields.get(3) != Some(&"WRITE") {
            continue;
        }
        let Some(key) = fields.get(5) else {
            continue;
        };
        let parts = key.split(':').collect::<Vec<_>>();
        if parts.len() != 3 {
            continue;
        }
        let device_matches = u32::from_str_radix(parts[0], 16).ok()
            == Some(rustix::fs::major(device))
            && u32::from_str_radix(parts[1], 16).ok() == Some(rustix::fs::minor(device));
        if device_matches
            && parts[2].parse::<u64>().ok() == Some(inode)
            && let Some(pid) = fields
                .get(4)
                .and_then(|pid| pid.parse::<u32>().ok())
                .filter(|pid| *pid > 0)
        {
            return KernelLock::Held { pid };
        }
    }
    KernelLock::NotObserved
}
