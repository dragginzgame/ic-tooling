//! Admitted extractor execution with source identity and bounded output evidence.
//! Callers own grammar validation, publication and exclusion of concurrent writers.

use ic_host_artifacts::artifact::{ArtifactError, ArtifactIdentity};
use ic_host_fs::read::hash_file;
use ic_host_process::tool::{
    AdmittedTool, CompletedExecution, ExecutionContext, OutputLimits, ToolError,
};
use std::{fmt, path::Path};

#[cfg(test)]
mod test_support;
#[cfg(test)]
mod tests;

/// A Candid extraction failed, retaining process bytes where invocation occurred.
#[derive(Debug, thiserror::Error)]
pub enum ExtractionError {
    /// The source path was relative; no tool was invoked.
    #[error("Candid source path must be absolute")]
    SourcePath,
    /// Source inspection failed before invocation.
    #[error("Candid source inspection failed: {0}")]
    Input(#[source] ArtifactError),
    /// Tool verification or execution failed; execution failures retain evidence.
    #[error("{0}")]
    Tool(#[source] ToolError),
    /// Source could not be re-inspected after successful execution.
    #[error("Candid source re-inspection failed")]
    SourceInspection {
        /// Inspection failure.
        source: ArtifactError,
        /// Bounded successful process output.
        evidence: Box<CompletedExecution>,
    },
    /// Source bytes changed between preflight and post-extraction observation.
    #[error("Candid source changed during extraction")]
    SourceChanged {
        /// Identity observed before invocation.
        before: ArtifactIdentity,
        /// Identity observed after invocation.
        after: ArtifactIdentity,
        /// Bounded process output that must not be published as valid extraction.
        evidence: Box<CompletedExecution>,
    },
}

/// Source/tool identities and exact bounded process evidence, without text conversion.
pub struct ExtractedCandid {
    /// Matching source identity observed before and after invocation.
    pub source_identity: ArtifactIdentity,
    /// Tool bytes admitted by the consumer.
    pub tool_identity: ArtifactIdentity,
    /// Original stdout/stderr and successful direct-child exit status.
    pub evidence: CompletedExecution,
}

impl fmt::Debug for ExtractedCandid {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("ExtractedCandid")
            .field("source_identity", &self.source_identity)
            .field("tool_identity", &self.tool_identity)
            .field("evidence", &self.evidence)
            .finish()
    }
}

/// Invoke the admitted extractor once with the absolute source path as one arg.
///
/// Input regular-file hashing is bounded by `source_bytes`; process output uses
/// the selected retention limits and must be complete. No UTF-8 conversion,
/// normalization or grammar validation occurs. Callers can explicitly apply
/// their own normalization to retained stdout with a separate normalized-byte
/// allowance. The source is inspected again on successful completion; observed changes reject
/// publication. Callers must exclude concurrent writers to source/tool paths:
/// before/after digests cannot prove that transient changes did not occur.
///
/// # Errors
/// Returns typed path/read, tool-execution or source-change failures.
/// After invocation, process failure/validation evidence is retained.
pub fn extract(
    tool: &AdmittedTool,
    source: &Path,
    context: &ExecutionContext<'_>,
    source_bytes: u64,
    output: OutputLimits,
) -> Result<ExtractedCandid, ExtractionError> {
    if !source.is_absolute() {
        return Err(ExtractionError::SourcePath);
    }
    let before = hash_file(source, source_bytes).map_err(ExtractionError::Input)?;
    let evidence = tool
        .run(&[source.as_os_str().to_owned()], context, output)
        .and_then(CompletedExecution::require_success)
        .and_then(CompletedExecution::require_complete)
        .map_err(ExtractionError::Tool)?;
    let after = match hash_file(source, source_bytes) {
        Ok(identity) => identity,
        Err(source) => {
            return Err(ExtractionError::SourceInspection {
                source,
                evidence: Box::new(evidence),
            });
        }
    };
    if before != after {
        return Err(ExtractionError::SourceChanged {
            before,
            after,
            evidence: Box::new(evidence),
        });
    }
    Ok(ExtractedCandid {
        source_identity: before,
        tool_identity: tool.identity(),
        evidence,
    })
}
