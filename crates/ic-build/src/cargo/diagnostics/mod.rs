//! Compile-diagnostic mechanics; callers retain sources, compiler recipes and messages.

use super::{
    messages::{BuildCompletion, MessageDecoder},
    observation::schema,
};
use ic_host_artifacts::artifact::visit_reader;
use ic_host_process::tool::{OutputLimit, OutputLimits, capture_command};
use std::{
    fs,
    io::{self, Read},
    path::{Path, PathBuf},
    process::Command,
};

#[cfg(test)]
mod tests;

/// Select one host library from a successful Cargo JSON build stream.
///
/// Target name and kind must identify exactly one artifact. Callers own the
/// build command, lockfile, host target and custody of the resulting files.
/// `maximum` bounds each JSON message before allocation.
/// Interrupted reads are retried; impossible reader counts refuse as invalid data.
///
/// # Errors
/// Rejects malformed, oversized, unsuccessful, missing or ambiguous evidence,
/// unsupported target kinds and missing library files.
pub fn library_artifact(
    mut input: impl Read,
    name: &str,
    kind: &schema::TargetKind,
    maximum: usize,
) -> io::Result<PathBuf> {
    if !matches!(
        kind,
        schema::TargetKind::Lib | schema::TargetKind::ProcMacro
    ) {
        return Err(io::Error::other("expected lib or proc-macro target"));
    }
    let mut selected = None;
    let mut finished = BuildCompletion::default();
    let mut receive = |message| {
        match message {
            schema::Message::CompilerArtifact(artifact)
                if artifact.target.name == name && artifact.target.kind.contains(kind) =>
            {
                for path in artifact.filenames {
                    let matches = match kind {
                        schema::TargetKind::Lib => path.extension() == Some("rlib"),
                        _ => matches!(path.extension(), Some("so" | "dylib" | "dll")),
                    };
                    if matches && selected.replace(path.into_std_path_buf()).is_some() {
                        return Err(io::Error::other("ambiguous library artifact"));
                    }
                }
            }
            schema::Message::BuildFinished(result) if !finished.record(result.success) => {
                return Err(io::Error::other("invalid Cargo build completion"));
            }
            _ => {}
        }
        Ok(())
    };
    let mut decoder = MessageDecoder::new(maximum, false);
    // Per-message admission stays with the decoder; no smaller total stream budget.
    visit_reader::<io::Error>(&mut input, &mut [0; 8192], u64::MAX, |bytes| {
        decoder.feed(bytes, &mut receive)
    })?;
    decoder.finish(&mut receive)?;
    let path = selected
        .filter(|path| finished.require_success().is_ok() && path.is_file())
        .ok_or_else(|| io::Error::other("missing successful host library artifact"))?;
    Ok(path)
}

/// Run a passing control before negative cases through a caller-owned compiler recipe.
///
/// The factory receives `None` for the control, then `Some(index)` for each
/// expected-message list. Each negative must exit with code 1 and include every
/// nonempty literal expected message in stderr. Crashes, truncation and empty
/// expectations never qualify. Logs are retained in the caller-created directory.
/// No elapsed-time deadline is imposed. `maximum` bounds each captured stream.
///
/// # Errors
/// Returns compiler/process, IO or diagnostic mismatch failures. Spawn/capture
/// failures retain Host's bounded evidence in their original typed error.
pub fn check_suite(
    expected: &[Vec<String>],
    log_directory: &Path,
    maximum: usize,
    mut command: impl FnMut(Option<usize>) -> Command,
) -> Result<(), Box<dyn std::error::Error>> {
    if expected.is_empty()
        || expected
            .iter()
            .any(|lines| lines.is_empty() || lines.iter().any(String::is_empty))
    {
        return Err("nonempty negative diagnostic expectations required".into());
    }
    for case in std::iter::once(None).chain((0..expected.len()).map(Some)) {
        let output = capture_command(
            &mut command(case),
            OutputLimits {
                stdout: OutputLimit::Terminate(maximum),
                stderr: OutputLimit::Terminate(maximum),
                timeout: None,
            },
        )?;
        let label = case.map_or_else(|| "control".to_string(), |index| index.to_string());
        fs::write(
            log_directory.join(format!("{label}.stdout")),
            &output.stdout,
        )?;
        fs::write(
            log_directory.join(format!("{label}.stderr")),
            &output.stderr,
        )?;
        let output = output.require_complete()?;
        if let Some(index) = case {
            let text = std::str::from_utf8(&output.stderr)?;
            if output.status.code() != Some(1)
                || !expected[index].iter().all(|message| text.contains(message))
            {
                return Err(format!(
                    "diagnostic case {index} failed; see {}",
                    log_directory.display()
                )
                .into());
            }
        } else {
            output.require_success()?;
        }
    }
    Ok(())
}
