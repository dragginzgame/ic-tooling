//! Native process-leader RSS observations, independent of workload policy.
//! Linux retains page-based /proc measurements; Darwin reports native ps RSS.

use std::collections::{BTreeMap, BTreeSet};

use crate::tool::{OutputLimits, ToolError};

#[cfg(any(target_os = "macos", test))]
use std::process::Command;
#[cfg(target_os = "linux")]
use std::{
    fs::{self, File},
    path::Path,
};

use std::{fmt, io};

/// Failure to observe a native process table or sum its resident bytes.
#[derive(Debug)]
pub enum SamplingError {
    /// Native I/O, stat-record storage/byte admission or stat UTF-8 failed.
    /// Non-I/O stream refusals retain an artifact error as the I/O error's cause.
    Io(io::Error),
    /// A native record was malformed or its byte arithmetic overflowed.
    InvalidData(&'static str),
    /// Native table capture, exit or completeness admission failed, with bounded
    /// output and independent cleanup evidence retained by the capture engine.
    Command(Box<ToolError>),
    /// This host has no supported process-table observation backend.
    UnsupportedHost,
}

impl fmt::Display for SamplingError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Io(error) => error.fmt(formatter),
            Self::InvalidData(message) => formatter.write_str(message),
            Self::Command(source) => write!(formatter, "process table capture failed: {source}"),
            Self::UnsupportedHost => formatter.write_str("native sampling requires Linux or macOS"),
        }
    }
}

impl std::error::Error for SamplingError {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        match self {
            Self::Io(error) => Some(error),
            Self::Command(error) => Some(error.as_ref()),
            _ => None,
        }
    }
}

impl From<io::Error> for SamplingError {
    fn from(error: io::Error) -> Self {
        Self::Io(error)
    }
}

/// One process leader observed in a native table.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct Process {
    /// Observed parent process identifier.
    pub parent: u32,
    /// Native resident memory in bytes; shared pages may occur in multiple processes.
    pub resident_bytes: u64,
}

/// Native observations indexed by process-leader identifier.
pub type Snapshot = BTreeMap<u32, Process>;

/// Measurement-source identity; consumers retain it with their comparisons.
#[must_use]
pub const fn source() -> &'static str {
    if cfg!(target_os = "linux") {
        "linux-proc-stat"
    } else if cfg!(target_os = "macos") {
        "macos-ps-rss"
    } else {
        "unsupported"
    }
}

#[cfg(any(target_os = "linux", test))]
fn parse_stat(stat: &str, page_bytes: u64) -> Result<Process, SamplingError> {
    // comm can contain spaces and closing parentheses. Remaining fields start
    // at state (3), then PPID (4), and RSS in pages (24).
    let (_, fields) = stat.rsplit_once(')').ok_or(SamplingError::InvalidData(
        "missing process command terminator",
    ))?;
    let mut fields = fields.split_whitespace();
    let parent = fields
        .nth(1)
        .and_then(|field| field.parse().ok())
        .ok_or(SamplingError::InvalidData("invalid process parent"))?;
    let pages: u64 = fields
        .nth(19)
        .and_then(|field| field.parse().ok())
        .ok_or(SamplingError::InvalidData("invalid process RSS pages"))?;
    let resident_bytes = pages
        .checked_mul(page_bytes)
        .ok_or(SamplingError::InvalidData("RSS byte count overflow"))?;
    Ok(Process {
        parent,
        resident_bytes,
    })
}

#[cfg(target_os = "linux")]
fn process_snapshot(
    proc: &Path,
    page_bytes: u64,
    record_bytes: u64,
) -> Result<Snapshot, SamplingError> {
    let record_bytes = usize::try_from(record_bytes)
        .map_err(|_| SamplingError::InvalidData("stat allowance exceeds the host address space"))?;
    let mut processes = Snapshot::new();
    for entry in fs::read_dir(proc)? {
        let entry = entry?;
        let Some(pid) = entry
            .file_name()
            .to_str()
            .and_then(|name| name.parse::<u32>().ok())
        else {
            continue;
        };
        let stat = File::open(entry.path().join("stat")).and_then(|file| {
            ic_host_artifacts::artifact::read_reader(file, record_bytes).map_err(io::Error::from)
        });
        match stat {
            Ok(stat) => {
                let stat = std::str::from_utf8(&stat)
                    .map_err(|source| io::Error::new(io::ErrorKind::InvalidData, source))?;
                processes.insert(pid, parse_stat(stat, page_bytes)?);
            }
            Err(error)
                if matches!(
                    error.kind(),
                    io::ErrorKind::NotFound | io::ErrorKind::PermissionDenied
                ) => {}
            Err(error) if error.raw_os_error() == Some(rustix::io::Errno::SRCH.raw_os_error()) => {}
            Err(error) => return Err(error.into()),
        }
    }
    Ok(processes)
}

#[cfg(any(target_os = "macos", test))]
fn parse_ps(output: &str) -> Result<Snapshot, SamplingError> {
    let mut processes = Snapshot::new();
    for line in output.lines().filter(|line| !line.trim().is_empty()) {
        let mut fields = line.split_whitespace();
        let pid = fields
            .next()
            .and_then(|field| field.parse::<u32>().ok())
            .ok_or(SamplingError::InvalidData("invalid ps PID"))?;
        let parent = fields
            .next()
            .and_then(|field| field.parse().ok())
            .ok_or(SamplingError::InvalidData("invalid ps parent"))?;
        let rss: u64 = fields
            .next()
            .and_then(|field| field.parse().ok())
            .ok_or(SamplingError::InvalidData("invalid ps RSS"))?;
        if fields.next().is_some() {
            return Err(SamplingError::InvalidData("unexpected ps fields"));
        }
        // Darwin ps documents RSS in 1024-byte units, not host memory pages.
        let resident_bytes = rss
            .checked_mul(1024)
            .ok_or(SamplingError::InvalidData("ps RSS byte count overflow"))?;
        if processes
            .insert(
                pid,
                Process {
                    parent,
                    resident_bytes,
                },
            )
            .is_some()
        {
            return Err(SamplingError::InvalidData(
                "duplicate process leader in ps output",
            ));
        }
    }
    Ok(processes)
}

#[cfg(any(target_os = "macos", test))]
fn command_snapshot(
    command: &mut Command,
    limits: OutputLimits,
) -> Result<Snapshot, SamplingError> {
    let output = crate::tool::capture_command(command, limits)
        .and_then(crate::tool::CompletedExecution::require_success)
        .and_then(crate::tool::CompletedExecution::require_complete)
        .map_err(|source| SamplingError::Command(Box::new(source)))?;
    let output = std::str::from_utf8(&output.stdout)
        .map_err(|_| SamplingError::InvalidData("ps output is not UTF-8"))?;
    parse_ps(output)
}

/// Observe process leaders using Linux /proc or the macOS native ps table.
///
/// `record_bytes` bounds each complete Linux stat record; oversized records refuse
/// before parsing, even if a retained prefix contains plausible numeric fields.
/// Consumers own sampling cadence,
/// workload selection and total snapshot memory. `command_limits` selects macOS
/// stdout/stderr retention and an optional capture deadline; `timeout: None`
/// imposes no elapsed-time deadline. Linux does not use `command_limits`, and
/// macOS does not use `record_bytes`.
/// Processes that disappear or deny stat access during Linux enumeration are skipped.
/// macOS ps RSS uses 1024-byte units, independently of native memory page size.
/// The native command's stderr is captured rather than printed. Failed captures
/// retain bounded output and cleanup evidence in [`SamplingError::Command`].
/// Successful but truncated output refuses before parsing, even if the retained
/// prefix contains complete rows. This is not a bound on total parsed table memory.
///
/// # Errors
/// Returns native I/O, capture/exit/truncation, malformed-record or checked-arithmetic
/// failures. macOS uses the direct-child capture engine; deadlines and cleanup
/// have that engine's documented synchronous syscall and reaping limitations.
pub fn snapshot(
    record_bytes: u64,
    command_limits: OutputLimits,
) -> Result<Snapshot, SamplingError> {
    #[cfg(not(target_os = "linux"))]
    let _ = record_bytes;
    #[cfg(not(target_os = "macos"))]
    let _ = command_limits;
    #[cfg(target_os = "linux")]
    {
        let page_bytes = rustix::param::page_size();
        let page_bytes = u64::try_from(page_bytes)
            .ok()
            .filter(|value| *value > 0)
            .ok_or(SamplingError::InvalidData("native page size unavailable"))?;
        process_snapshot(Path::new("/proc"), page_bytes, record_bytes)
    }
    #[cfg(target_os = "macos")]
    {
        command_snapshot(
            Command::new("/bin/ps")
                .args(["-A", "-o", "pid=,ppid=,rss="])
                .env("LC_ALL", "C"),
            command_limits,
        )
    }
    #[cfg(not(any(target_os = "linux", target_os = "macos")))]
    Err(SamplingError::UnsupportedHost)
}

/// Sum observed resident bytes for a root and its descendants, counting each leader once.
///
/// Missing roots yield zero; cycles cannot repeat an observation. This is a sampled
/// sum of native RSS, not unique physical memory or proof that a process is still live.
///
/// # Errors
/// Returns an arithmetic overflow rather than wrapping the resident byte total.
pub fn tree_rss(root: u32, processes: &Snapshot) -> Result<(u64, usize), SamplingError> {
    if !processes.contains_key(&root) {
        return Ok((0, 0));
    }
    let mut children: BTreeMap<u32, Vec<u32>> = BTreeMap::new();
    for (pid, process) in processes {
        children.entry(process.parent).or_default().push(*pid);
    }
    let mut pending = vec![root];
    let mut seen = BTreeSet::new();
    let mut resident = 0_u64;
    while let Some(pid) = pending.pop() {
        if !seen.insert(pid) {
            continue;
        }
        resident = resident
            .checked_add(processes[&pid].resident_bytes)
            .ok_or(SamplingError::InvalidData("process-tree RSS overflow"))?;
        if let Some(children) = children.get(&pid) {
            pending.extend(children);
        }
    }
    Ok((resident, seen.len()))
}

#[cfg(test)]
mod tests;
