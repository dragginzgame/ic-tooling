use std::{
    io,
    path::{Path, PathBuf},
    process::Command,
};

/// Render Cargo's workspace-location command without executing it.
///
/// Cargo owns membership, explicit `package.workspace`, exclusions and nested
/// independent workspaces. This selects an offline `locate-project`, without
/// resolving dependencies or installing a toolchain. It never guesses from the
/// directory layout or falls back to the supplied directory.
/// The caller supplies the Cargo command and environment. Existing launcher
/// arguments and working directory are preserved. Execute through the caller's
/// runner and admit successful, complete output before [`decode_workspace_root`].
/// No elapsed-time deadline is added;
/// toolchain auto-installation is disabled for this observation.
///
/// # Errors
/// Returns the filesystem error if the manifest directory cannot be canonicalized.
pub fn workspace_root_command(
    crate_manifest_dir: impl AsRef<Path>,
    mut command: Command,
) -> io::Result<Command> {
    let manifest = crate_manifest_dir
        .as_ref()
        .canonicalize()?
        .join("Cargo.toml");
    command
        .env("RUSTUP_AUTO_INSTALL", "0")
        .args([
            "--offline",
            "locate-project",
            "--workspace",
            "--message-format=json",
        ])
        .arg("--manifest-path")
        .arg(manifest);
    Ok(command)
}

/// Decode successful, complete `cargo locate-project` JSON into its workspace root.
///
/// The response must name an absolute Cargo manifest. This validates Cargo's
/// response shape without guessing membership, checking the returned directory
/// or falling back to the supplied crate directory. Capture policy and typed
/// invocation failures belong to the caller's runner.
///
/// # Errors
/// Returns `InvalidData` for malformed JSON or a nonabsolute/nonmanifest response.
pub fn decode_workspace_root(bytes: &[u8]) -> io::Result<PathBuf> {
    let response: serde_json::Value = serde_json::from_slice(bytes)
        .map_err(|error| io::Error::new(io::ErrorKind::InvalidData, error))?;
    let manifest = response
        .get("root")
        .and_then(serde_json::Value::as_str)
        .map(Path::new)
        .filter(|path| {
            path.is_absolute() && path.file_name() == Some(std::ffi::OsStr::new("Cargo.toml"))
        })
        .ok_or_else(|| {
            io::Error::new(
                io::ErrorKind::InvalidData,
                "Cargo returned no absolute workspace manifest",
            )
        })?;
    manifest.parent().map(Path::to_owned).ok_or_else(|| {
        io::Error::new(
            io::ErrorKind::InvalidData,
            "Cargo workspace manifest has no parent",
        )
    })
}

#[cfg(all(test, unix))]
mod tests;
