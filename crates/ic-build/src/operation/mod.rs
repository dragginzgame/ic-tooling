//! Admission and ordering for the build-to-owned-artifact operation.

#[cfg(test)]
mod tests;

use std::{
    ffi::OsString,
    io::{self, Write},
    path::{Path, PathBuf},
    process::{Command, ExitStatus},
    sync::atomic::{AtomicBool, Ordering},
    time::Duration,
};

use thiserror::Error;

use crate::{bytes, custody::Target, process, protocol::Decoder};

///
/// Limits
/// Explicit communication and capture ceilings. No hidden unlimited mode exists.
///

#[derive(Debug, Clone)]
pub struct Limits {
    /// Maximum total Cargo stdout bytes before termination.
    pub stdout_bytes: usize,
    /// Maximum total stderr bytes, retained and sent to the diagnostic sink.
    pub stderr_bytes: usize,
    /// Maximum bytes in one JSON record, excluding newline.
    pub record_bytes: usize,
    /// Maximum matching Wasm candidates; ambiguity is always refused.
    pub candidates: usize,
    /// Maximum completed artifact bytes retained in memory.
    pub artifact_bytes: usize,
    /// Maximum communication time, including waiting for inherited pipe EOF.
    pub elapsed: Duration,
}

///
/// BuildRequest
/// One locked Wasm library build. All paths must be absolute.
///

#[derive(Debug)]
pub struct BuildRequest {
    /// Selected Cargo manifest; source stability is caller-owned.
    pub manifest: PathBuf,
    /// Cargo package selector passed with `--package`.
    pub package: String,
    /// Exact opaque package ID expected in Cargo's JSON artifact record.
    pub package_id: String,
    /// Exact library target name expected in Cargo's JSON artifact record.
    pub library: String,
    /// Prepared Cargo executable (not implicitly downloaded).
    pub cargo: PathBuf,
    /// Prepared Rust compiler executable selected through RUSTC.
    pub rustc: PathBuf,
    /// Stable, caller-owned scratch parent; each operation creates a private child.
    pub scratch_parent: PathBuf,
    /// Cargo profile name, e.g. `dev` or `release`.
    pub profile: String,
    /// Explicit selected features, passed as separate Cargo option values.
    pub features: Vec<String>,
    /// Whether Cargo should enable this package's default features.
    pub default_features: bool,
    /// Entire child environment, after clearing inherited values. RUSTC and
    /// CARGO_TARGET_DIR are operation-owned; wrapper overrides are refused.
    /// Supply CARGO_NET_OFFLINE here to select Cargo network mode explicitly.
    pub environment: Vec<(OsString, OsString)>,
    /// Communication, decoding and artifact bounds.
    pub limits: Limits,
}

///
/// Artifact
/// Completed artifact bytes independent of the mutable Cargo output directory.
///

#[derive(Debug)]
pub struct Artifact {
    bytes: Vec<u8>,
    digest: [u8; 32],
}

impl Artifact {
    /// Read captured bytes; no writable file handle or Cargo pathname escapes.
    #[must_use]
    pub fn bytes(&self) -> &[u8] {
        &self.bytes
    }

    /// SHA-256 of exactly `bytes()`, with no framing or transformations.
    #[must_use]
    pub fn sha256(&self) -> &[u8; 32] {
        &self.digest
    }
}

///
/// Failure
/// Primary refusal of an operation; original I/O and JSON causes remain typed.
///

#[derive(Debug, Error)]
pub enum Failure {
    /// Request rejected before effects.
    #[error("invalid build request: {0}")]
    InvalidRequest(&'static str),
    /// Original operating-system or input failure.
    #[error("build I/O failed: {0}")]
    Io(#[source] io::Error),
    /// Natural nonzero tool status (available on `BuildFailure`).
    #[error("Cargo exited unsuccessfully")]
    ToolFailed,
    /// Malformed original Cargo JSON bytes.
    #[error("invalid Cargo JSON: {0}")]
    Json(#[source] serde_json::Error),
    /// Missing, ambiguous or invalid protocol facts.
    #[error("Cargo protocol refused: {0}")]
    Protocol(&'static str),
    /// Total stdout bound was exceeded; process terminated.
    #[error("Cargo stdout limit exceeded")]
    OutputLimit,
    /// Total stderr bound was exceeded; process terminated.
    #[error("Cargo stderr limit exceeded")]
    DiagnosticLimit,
    /// One protocol record exceeded its bound.
    #[error("Cargo record limit exceeded")]
    RecordLimit,
    /// Matching artifact candidates exceeded their bound.
    #[error("Cargo candidate limit exceeded")]
    CandidateLimit,
    /// Owned byte capture exceeded its ceiling.
    #[error("artifact byte limit exceeded")]
    ArtifactLimit,
    /// Fallible buffer reservation failed.
    #[error("buffer allocation refused")]
    Allocation,
    /// Caller cancellation was observed.
    #[error("build cancelled")]
    Cancelled,
    /// Communication deadline expired.
    #[error("build communication deadline expired")]
    Deadline,
    /// Caller-provided diagnostics sink refused bytes or flush.
    #[error("diagnostics sink refused: {0}")]
    Sink(#[source] io::Error),
    /// Primary work completed but explicit cleanup failed.
    #[error("build cleanup failed")]
    Cleanup,
}

///
/// BuildFailure
/// A failed build retains primary, protocol, status and cleanup evidence together.
///

#[derive(Debug, Error)]
#[error("{primary}")]
pub struct BuildFailure {
    /// Main refusal; a natural unsuccessful exit takes precedence over bad JSON.
    #[source]
    pub primary: Failure,
    /// Additional protocol refusal when execution already failed.
    pub protocol: Option<Failure>,
    /// Reaped command status when available, including operation-triggered kills.
    pub status: Option<ExitStatus>,
    /// Bounded original stderr bytes. Escape these before terminal presentation.
    pub diagnostics: Vec<u8>,
    /// Shutdown, reap and scratch-removal failures, never hidden by primary success.
    pub cleanup: Vec<io::Error>,
    /// Scratch location needing reconciliation when explicit removal failed.
    pub scratch: Option<PathBuf>,
}

impl BuildFailure {
    fn new(primary: Failure) -> Self {
        Self {
            primary,
            protocol: None,
            status: None,
            diagnostics: Vec::new(),
            cleanup: Vec::new(),
            scratch: None,
        }
    }
}

/// Execute, validate, select and capture one Wasm library, then remove build scratch.
///
/// No artifact is returned if execution, protocol, capture or explicit cleanup
/// failed. Cancellation is cooperative; kernel operations and caller sink writes
/// have no hard time bound. No retries, downloads or shared target are performed.
/// The synchronous session owns its process group until shutdown and reap finish.
/// Caller source/tool stability and a private, stable scratch parent are required.
///
/// # Errors
/// Returns all available bounded failure evidence. External build-script effects
/// are not rolled back. A failed scratch cleanup names the path for reconciliation.
#[expect(
    clippy::result_large_err,
    reason = "bounded failure evidence retains primary, protocol, status and cleanup without an extra allocation"
)]
pub fn build(
    request: &BuildRequest,
    cancelled: &AtomicBool,
    diagnostics: &mut dyn Write,
) -> Result<Artifact, BuildFailure> {
    validate(request).map_err(BuildFailure::new)?;
    if cancelled.load(Ordering::Relaxed) {
        return Err(BuildFailure::new(Failure::Cancelled));
    }

    let target = Target::create(&request.scratch_parent).map_err(|error| {
        let mut failure = BuildFailure::new(Failure::Io(error.cause));
        if let Some((path, cause)) = error.cleanup {
            failure.scratch = Some(path);
            failure.cleanup.push(cause);
        }

        failure
    })?;

    let result = execute(request, &target, cancelled, diagnostics);

    complete(result, target.close())
}

#[expect(
    clippy::result_large_err,
    reason = "bounded failure evidence retains primary, protocol, status and cleanup without an extra allocation"
)]
fn complete(
    result: Result<Artifact, BuildFailure>,
    cleanup: Result<(), (PathBuf, io::Error)>,
) -> Result<Artifact, BuildFailure> {
    match cleanup {
        Ok(()) => result,
        Err((path, error)) => {
            let mut failure = match result {
                Ok(_) => BuildFailure::new(Failure::Cleanup),
                Err(failure) => failure,
            };
            failure.cleanup.push(error);
            failure.scratch = Some(path);

            Err(failure)
        }
    }
}

#[expect(
    clippy::result_large_err,
    reason = "bounded failure evidence retains primary, protocol, status and cleanup without an extra allocation"
)]
fn execute(
    request: &BuildRequest,
    target: &Target,
    cancelled: &AtomicBool,
    diagnostics: &mut dyn Write,
) -> Result<Artifact, BuildFailure> {
    let mut command = invocation(request, target.path());
    let mut decoder = Decoder::new(
        &request.package_id,
        &request.library,
        request.limits.record_bytes,
        request.limits.candidates,
    );
    let report = process::run(
        &mut command,
        &request.limits,
        cancelled,
        diagnostics,
        |bytes| decoder.feed(bytes),
    );
    let selection = decoder.finish();
    let primary = report.failure.or_else(|| {
        if report.status.is_some_and(|status| status.success()) {
            None
        } else {
            Some(Failure::ToolFailed)
        }
    });
    if let Some(primary) = primary {
        return Err(BuildFailure {
            primary,
            protocol: selection.err(),
            status: report.status,
            diagnostics: report.diagnostics,
            cleanup: report.cleanup,
            scratch: None,
        });
    }
    let path = match selection {
        Ok(path) => path,
        Err(primary) => {
            return Err(BuildFailure {
                primary,
                protocol: None,
                status: report.status,
                diagnostics: report.diagnostics,
                cleanup: report.cleanup,
                scratch: None,
            });
        }
    };
    if !report.cleanup.is_empty() {
        return Err(BuildFailure {
            primary: Failure::Cleanup,
            protocol: None,
            status: report.status,
            diagnostics: report.diagnostics,
            cleanup: report.cleanup,
            scratch: None,
        });
    }
    let captured = target
        .open_artifact(&path)
        .map_err(Failure::Io)
        .and_then(|file| bytes::capture(file, request.limits.artifact_bytes, cancelled));

    captured
        .map(|(bytes, digest)| Artifact { bytes, digest })
        .map_err(|primary| BuildFailure {
            primary,
            protocol: None,
            status: report.status,
            diagnostics: report.diagnostics,
            cleanup: Vec::new(),
            scratch: None,
        })
}

fn invocation(request: &BuildRequest, target: &Path) -> Command {
    let mut command = Command::new(&request.cargo);
    command
        .env_clear()
        .envs(request.environment.iter().cloned())
        .env("RUSTC", &request.rustc)
        .current_dir(request.manifest.parent().unwrap_or(Path::new("/")))
        .args([
            "build",
            "--locked",
            "--message-format=json,json-render-diagnostics",
            "--color=never",
            "--manifest-path",
        ])
        .arg(&request.manifest)
        .arg("--package")
        .arg(&request.package)
        .arg("--lib")
        .args(["--target", "wasm32-unknown-unknown", "--profile"])
        .arg(&request.profile)
        .arg("--target-dir")
        .arg(target)
        .args([
            "--config",
            "build.rustc-wrapper=\"\"",
            "--config",
            "build.rustc-workspace-wrapper=\"\"",
        ]);
    if !request.default_features {
        command.arg("--no-default-features");
    }
    for feature in &request.features {
        command.arg("--features").arg(feature);
    }

    command
}

fn validate(request: &BuildRequest) -> Result<(), Failure> {
    for path in [
        &request.manifest,
        &request.cargo,
        &request.rustc,
        &request.scratch_parent,
    ] {
        if !path.is_absolute()
            || path
                .as_os_str()
                .as_encoded_bytes()
                .iter()
                .any(|byte| matches!(byte, b'\r' | b'\n' | 0))
        {
            return Err(Failure::InvalidRequest(
                "paths must be absolute and contain no NUL/CR/LF",
            ));
        }
    }
    if !request.manifest.is_file()
        || !request.cargo.is_file()
        || !request.rustc.is_file()
        || !request.scratch_parent.is_dir()
    {
        return Err(Failure::InvalidRequest(
            "manifest, prepared tools and scratch parent must exist",
        ));
    }
    for value in [
        &request.package,
        &request.package_id,
        &request.library,
        &request.profile,
    ] {
        if value.is_empty() || value.starts_with('-') || value.chars().any(char::is_control) {
            return Err(Failure::InvalidRequest(
                "invalid target identity or profile",
            ));
        }
    }
    if request.features.iter().any(|value| {
        value.is_empty()
            || value.starts_with('-')
            || value.chars().any(char::is_whitespace)
            || value.chars().any(char::is_control)
    }) {
        return Err(Failure::InvalidRequest("invalid feature"));
    }
    for (index, (key, value)) in request.environment.iter().enumerate() {
        if key.is_empty()
            || key
                .as_encoded_bytes()
                .iter()
                .any(|byte| matches!(byte, b'=' | 0))
            || value.as_encoded_bytes().contains(&0)
            || [
                "RUSTC",
                "CARGO_TARGET_DIR",
                "RUSTC_WRAPPER",
                "RUSTC_WORKSPACE_WRAPPER",
            ]
            .iter()
            .any(|owned| key == owned)
            || request.environment[..index]
                .iter()
                .any(|(earlier, _)| earlier == key)
        {
            return Err(Failure::InvalidRequest(
                "invalid, duplicate or operation-owned environment entry",
            ));
        }
    }
    let limits = &request.limits;
    if limits.stdout_bytes == 0
        || limits.record_bytes == 0
        || limits.candidates == 0
        || limits.artifact_bytes == 0
        || limits.elapsed.is_zero()
    {
        return Err(Failure::InvalidRequest("required limits must be positive"));
    }

    Ok(())
}
