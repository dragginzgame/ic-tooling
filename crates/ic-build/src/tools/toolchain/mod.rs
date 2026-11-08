//! Exact compiler sysroot inputs; consumers retain toolchain and target selection.
use super::probe;
use ic_host_process::tool::OutputLimits;
use std::{
    collections::BTreeSet,
    fs, io,
    path::{Path, PathBuf},
    process::Command,
};
/// Files and directories contributing to the selected compiler/target identity.
#[derive(Debug)]
pub struct ToolchainInputs {
    /// Canonical compiler sysroot.
    pub sysroot: PathBuf,
    /// Compiler-reported host tuple.
    pub host: String,
    /// Selected input paths. Callers own traversal, hashing and retention.
    pub paths: BTreeSet<PathBuf>,
}
/// Probe a caller-configured compiler and enumerate its bounded selected inputs.
/// No tool is installed, no environment is inherited on the caller's behalf, and no build runs.
/// # Errors
/// Process, filesystem or UTF-8 errors; invalid targets, missing inputs or exceeded limits refuse.
pub fn observe(
    mut compiler: impl FnMut() -> Command,
    target: &str,
    limits: OutputLimits,
    maximum_library_entries: usize,
) -> io::Result<ToolchainInputs> {
    valid_target(target)?;
    let sysroot = PathBuf::from(probe::text(
        compiler().args(["--print", "sysroot"]),
        limits,
    )?)
    .canonicalize()?;
    let host = probe::text(compiler().args(["--print", "host-tuple"]), limits)?;
    valid_target(&host)?;
    let paths = inventory(&sysroot, &host, target, maximum_library_entries)?;
    Ok(ToolchainInputs {
        sysroot,
        host,
        paths,
    })
}
fn valid_target(target: &str) -> io::Result<()> {
    if target.is_empty()
        || !target
            .bytes()
            .all(|b| b.is_ascii_alphanumeric() || matches!(b, b'-' | b'_' | b'.'))
        || target == "."
        || target == ".."
    {
        Err(io::Error::from(io::ErrorKind::InvalidInput))
    } else {
        Ok(())
    }
}
fn inventory(
    sysroot: &Path,
    host: &str,
    target: &str,
    maximum: usize,
) -> io::Result<BTreeSet<PathBuf>> {
    let mut paths = BTreeSet::new();
    for name in ["rustc", "cargo"] {
        let p = sysroot.join("bin").join(name);
        if !fs::symlink_metadata(&p)?.is_file() {
            return Err(io::Error::from(io::ErrorKind::InvalidData));
        }
        paths.insert(p);
    }
    for (i, e) in fs::read_dir(sysroot.join("lib"))?.enumerate() {
        if i >= maximum {
            return Err(io::Error::from(io::ErrorKind::InvalidData));
        }
        let p = e?.path();
        if fs::symlink_metadata(&p)?.is_file() {
            paths.insert(p);
        }
    }
    for t in [host, target] {
        let p = sysroot.join("lib/rustlib").join(t);
        if !fs::symlink_metadata(&p)?.is_dir() {
            return Err(io::Error::from(io::ErrorKind::InvalidData));
        }
        paths.insert(p);
    }
    Ok(paths)
}
#[cfg(test)]
mod tests;
