//! Incremental selection without retaining Cargo diagnostics or unrelated artifacts.

use super::{BuildCompletion, CargoArtifactError, CargoRecipe, schema, wasm_artifact_paths};
use std::{
    collections::BTreeSet,
    io,
    path::{Path, PathBuf},
};

/// Select one exact cdylib Wasm from streamed Cargo messages.
///
/// Only distinct matching output paths and completion state are retained. Caller
/// limits bound candidate count and total encoded path bytes, not message decoding,
/// allocator overhead or consumer callbacks. The manifest is canonicalized at
/// construction; callers retain source and target custody during observation.
/// Diagnostic and build-script interpretation remains with the callback owner.
///
/// This supplies protocol selection, not process-success or artifact admission.
/// Check execution, decoder and build facts before calling [`Self::finish`], then
/// capture the selected artifact while holding the appropriate build lock.
pub struct CdylibWasmSelector {
    manifest: PathBuf,
    example: Option<String>,
    outputs: BTreeSet<PathBuf>,
    completion: BuildCompletion,
    max_outputs: usize,
    max_path_bytes: usize,
    path_bytes: usize,
    refused: bool,
}

impl CdylibWasmSelector {
    /// Admit the exact manifest and choose inclusive retained-candidate limits.
    ///
    /// Zero limits allow completion-only streams but refuse any matching output.
    ///
    /// # Errors
    /// Returns the original manifest canonicalization error.
    pub fn new(
        manifest: &Path,
        recipe: &CargoRecipe,
        max_outputs: usize,
        max_path_bytes: usize,
    ) -> io::Result<Self> {
        Ok(Self {
            manifest: manifest.canonicalize()?,
            example: match recipe {
                CargoRecipe::Example(name) => Some(name.clone()),
                CargoRecipe::Library | CargoRecipe::RustcLibrary(_) => None,
            },
            outputs: BTreeSet::new(),
            completion: BuildCompletion::default(),
            max_outputs,
            max_path_bytes,
            path_bytes: 0,
            refused: false,
        })
    }

    /// Observe one message without retaining it. Repeated output paths count once.
    ///
    /// # Errors
    /// Returns original manifest resolution errors or `InvalidData` for exceeded
    /// limits or a previous refusal. Any refusal permanently prevents selection,
    /// even if the caller ignores the error and later supplies successful completion.
    pub fn observe(&mut self, message: &schema::Message) -> io::Result<()> {
        if self.refused {
            return Err(io::Error::new(
                io::ErrorKind::InvalidData,
                "Cargo artifact selection previously refused",
            ));
        }
        self.refused = true;
        self.observe_inner(message)?;
        self.refused = false;
        Ok(())
    }

    fn observe_inner(&mut self, message: &schema::Message) -> io::Result<()> {
        if let schema::Message::BuildFinished(finished) = message {
            self.completion.record(finished.success);
        }
        let schema::Message::CompilerArtifact(artifact) = message else {
            return Ok(());
        };
        let example = artifact.target.kind.contains(&schema::TargetKind::Example);
        let matches_recipe = self
            .example
            .as_ref()
            .map_or(!example, |name| example && artifact.target.name == *name);
        if !artifact
            .target
            .crate_types
            .contains(&schema::CrateType::CDyLib)
            || !matches_recipe
            || artifact.manifest_path.canonicalize()? != self.manifest
        {
            return Ok(());
        }
        for path in wasm_artifact_paths(artifact) {
            if self.outputs.contains(path) {
                continue;
            }
            let bytes = path.as_os_str().as_encoded_bytes().len();
            if self.outputs.len() >= self.max_outputs
                || bytes > self.max_path_bytes - self.path_bytes
            {
                return Err(io::Error::new(
                    io::ErrorKind::InvalidData,
                    "Cargo artifact candidates exceed caller allowance",
                ));
            }
            self.outputs.insert(path.to_path_buf());
            self.path_bytes += bytes;
        }
        Ok(())
    }

    /// Require one successful completion and exactly one distinct matching output.
    ///
    /// # Errors
    /// Refuses prior observation failure, missing/failed/duplicate completion or
    /// missing/ambiguous outputs. The path does not establish artifact lifetime.
    pub fn finish(mut self) -> Result<PathBuf, CargoArtifactError> {
        if self.refused {
            return Err(io::Error::new(
                io::ErrorKind::InvalidData,
                "Cargo artifact selection previously refused",
            )
            .into());
        }
        self.completion.require_success()?;
        let count = self.outputs.len();
        match self.outputs.pop_first() {
            Some(path) if count == 1 => Ok(path),
            _ => Err(CargoArtifactError::Selection {
                manifest: self.manifest,
                count,
            }),
        }
    }
}
