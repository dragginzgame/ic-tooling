//! Exact build-script producer evidence. Consumers select keys, recipe and persistence.
use super::observation::schema::{Message, TargetKind};
use serde::{Deserialize, Serialize};
use std::{
    collections::{BTreeMap, BTreeSet},
    io,
    path::{Path, PathBuf},
};
/// Bounded retained inputs tied to one exact Cargo producer and caller recipe.
#[derive(Clone, Debug, Eq, PartialEq, Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
pub struct ProducerExports {
    /// Exact caller-selected recipe identity.
    pub recipe: String,
    /// Exact Cargo package identifier.
    pub package_id: String,
    /// Canonical producer manifest path.
    pub manifest_path: PathBuf,
    /// Exported input paths. Relative paths are refused before retention.
    pub inputs: BTreeSet<PathBuf>,
}
/// Associate selected build-script environment exports with compiler-artifact producers.
/// Message and selected-input counts are bounded; ambiguous/unknown producers refuse.
/// # Errors
/// I/O failures or `InvalidData` for exceeded bounds, relative paths or ambiguous producers.
pub fn collect(
    messages: &[Message],
    keys: &BTreeSet<String>,
    recipe: &str,
    maximum_messages: usize,
    maximum_inputs: usize,
) -> io::Result<Vec<ProducerExports>> {
    if messages.len() > maximum_messages {
        return Err(invalid("message limit"));
    }
    let mut producers = BTreeMap::new();
    for m in messages {
        if let Message::CompilerArtifact(a) = m
            && a.target.kind.contains(&TargetKind::CustomBuild)
        {
            let id = a.package_id.repr.as_str();
            let path = a.manifest_path.canonicalize()?;
            if producers
                .insert(id, path.clone())
                .is_some_and(|old| old != path)
            {
                return Err(invalid("ambiguous Cargo producer"));
            }
        }
    }
    let mut records = BTreeMap::<PathBuf, ProducerExports>::new();
    let mut inputs = 0usize;
    for m in messages {
        if let Message::BuildScriptExecuted(script) = m {
            let id = script.package_id.repr.as_str();
            let manifest = producers
                .get(id)
                .ok_or_else(|| invalid("unknown Cargo producer"))?;
            let record = records
                .entry(manifest.clone())
                .or_insert_with(|| ProducerExports {
                    recipe: recipe.into(),
                    package_id: id.into(),
                    manifest_path: manifest.clone(),
                    inputs: BTreeSet::new(),
                });
            if record.package_id != id {
                return Err(invalid("ambiguous manifest producer"));
            }
            for (key, value) in &script.env {
                if keys.contains(key) {
                    inputs = inputs
                        .checked_add(1)
                        .filter(|n| *n <= maximum_inputs)
                        .ok_or_else(|| invalid("input limit"))?;
                    let path = PathBuf::from(value);
                    if !path.is_absolute() {
                        return Err(invalid("exported path is not absolute"));
                    }
                    record.inputs.insert(path);
                }
            }
        }
    }
    Ok(records.into_values().collect())
}
impl ProducerExports {
    /// Admit a retained record against current exact producer/recipe evidence before using inputs.
    /// # Errors
    /// I/O failures or `InvalidData` when current identity, paths or bounds differ.
    pub fn validate(
        &self,
        recipe: &str,
        package_id: &str,
        manifest: &Path,
        maximum_inputs: usize,
    ) -> io::Result<()> {
        if self.recipe != recipe
            || self.package_id != package_id
            || self.manifest_path != manifest.canonicalize()?
            || self.inputs.len() > maximum_inputs
            || self.inputs.iter().any(|p| !p.is_absolute())
        {
            return Err(invalid("retained producer identity or bounds differ"));
        }
        Ok(())
    }
}
fn invalid(message: &'static str) -> io::Error {
    io::Error::new(io::ErrorKind::InvalidData, message)
}
#[cfg(test)]
mod tests;
