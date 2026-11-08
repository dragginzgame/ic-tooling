//! Exact Git worktree observations with inherited repository overrides removed.
use crate::tools::probe;
use ic_host_process::tool::OutputLimits;
use std::{
    io,
    path::{Path, PathBuf},
    process::Command,
};
/// Observed Git facts. These are not a release authorization or immutable snapshot.
#[derive(Debug)]
pub struct GitSource {
    /// Observed HEAD revision.
    pub revision: String,
    /// Observed branch, including Git's detached-HEAD representation.
    pub branch: String,
    /// Raw NUL-delimited porcelain status. May contain private path names; callers own redaction.
    pub status: Vec<u8>,
}
/// Observe only an exact worktree root using caller-selected Git and process budgets.
/// Failed or incomplete observations return errors; they never become a clean result.
/// # Errors
/// Process, output, UTF-8 or filesystem errors; `InvalidInput` for a non-root selection.
pub fn observe(
    root: &Path,
    mut git: impl FnMut() -> Command,
    limits: OutputLimits,
) -> io::Result<GitSource> {
    let mut command = || {
        let mut c = git();
        clear_git_environment(&mut c);
        c.current_dir(root).args(["-c", "core.fsmonitor=false"]);
        c
    };
    let top = PathBuf::from(probe::text(
        command().args(["rev-parse", "--show-toplevel"]),
        limits,
    )?)
    .canonicalize()?;
    if top != root.canonicalize()? {
        return Err(io::Error::new(
            io::ErrorKind::InvalidInput,
            "selected path is not the Git worktree root",
        ));
    }
    let revision = probe::text(command().args(["rev-parse", "HEAD"]), limits)?;
    let branch = probe::text(
        command().args(["rev-parse", "--abbrev-ref", "HEAD"]),
        limits,
    )?;
    let status = probe::bytes(
        command().args([
            "status",
            "--porcelain=v1",
            "-z",
            "--untracked-files=all",
            "--ignore-submodules=none",
        ]),
        limits,
    )?;
    Ok(GitSource {
        revision,
        branch,
        status,
    })
}
fn clear_git_environment(command: &mut Command) {
    for key in [
        "GIT_ALTERNATE_OBJECT_DIRECTORIES",
        "GIT_CEILING_DIRECTORIES",
        "GIT_COMMON_DIR",
        "GIT_CONFIG",
        "GIT_CONFIG_COUNT",
        "GIT_CONFIG_GLOBAL",
        "GIT_CONFIG_NOSYSTEM",
        "GIT_CONFIG_PARAMETERS",
        "GIT_CONFIG_SYSTEM",
        "GIT_DIR",
        "GIT_DISCOVERY_ACROSS_FILESYSTEM",
        "GIT_INDEX_FILE",
        "GIT_NAMESPACE",
        "GIT_OBJECT_DIRECTORY",
        "GIT_PREFIX",
        "GIT_WORK_TREE",
    ] {
        command.env_remove(key);
    }
}

#[cfg(test)]
mod tests;
