//! Shared complete-output admission for caller-configured tool probes.
use ic_host_process::tool::{CompletedExecution, OutputLimits, capture_command};
use std::{io, process::Command};
pub fn bytes(command: &mut Command, limits: OutputLimits) -> io::Result<Vec<u8>> {
    let output = capture_command(command, limits)
        .and_then(CompletedExecution::require_complete)
        .and_then(CompletedExecution::require_success)
        .map_err(io::Error::other)?;
    Ok(output.stdout)
}
pub fn text(command: &mut Command, limits: OutputLimits) -> io::Result<String> {
    let bytes = bytes(command, limits)?;
    let text =
        String::from_utf8(bytes).map_err(|e| io::Error::new(io::ErrorKind::InvalidData, e))?;
    let text = text.trim();
    if text.is_empty() {
        return Err(io::Error::from(io::ErrorKind::InvalidData));
    }
    Ok(text.to_owned())
}

#[cfg(all(test, unix))]
mod tests;
