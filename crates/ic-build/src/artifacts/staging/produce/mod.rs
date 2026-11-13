use std::{fs, io, path::Path, process::ExitStatus};

use super::CapturedArtifact;

/// Produce and retain one artifact, removing its producer only after exact capture.
///
/// The caller owns physically distinct paths, their parents and custody against
/// other writers, including any parent aliases. Identical paths refuse early.
/// Both producer and log are created exclusively, in that order, before `run`.
/// The callback receives the producer path and owned log file; it owns the tool,
/// arguments, process execution, output routing and cancellation policy. This
/// function starts no process itself and creates no cache or default deadline.
///
/// An unsuccessful exit is returned unchanged, without capture or cleanup. I/O
/// errors and panics also leave existing producer/log files for caller recovery.
/// On success, capture admits `maximum_bytes`, retains the exact bytes at
/// `destination`, then removes only the producer. The log always remains.
///
/// # Errors
/// Returns creation, callback, capture, retention or producer-removal errors.
/// Capture/retention errors retain their existing typed causes, including
/// [`super::CaptureRetentionError`] when capture custody has transferred.
pub fn produce_and_retain_artifact(
    producer: &Path,
    log: &Path,
    destination: &Path,
    maximum_bytes: usize,
    run: impl FnOnce(&Path, fs::File) -> io::Result<ExitStatus>,
) -> io::Result<ExitStatus> {
    if producer == log || producer == destination || log == destination {
        return Err(io::Error::new(
            io::ErrorKind::InvalidInput,
            "producer, log and destination must be distinct",
        ));
    }
    let parent = destination
        .parent()
        .ok_or_else(|| io::Error::other("missing evidence parent"))?;
    fs::File::create_new(producer)?;
    let log = fs::File::create_new(log)?;
    let status = run(producer, log)?;
    if status.success() {
        CapturedArtifact::capture(producer, parent, maximum_bytes)?
            .retain_to(destination)
            .map_err(io::Error::other)?;
        fs::remove_file(producer)?;
    }
    Ok(status)
}

#[cfg(all(test, unix))]
mod tests;
