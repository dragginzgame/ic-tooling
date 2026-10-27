//! Wasm target admission and the captured output projection of Cargo metadata.

use super::{WasmBuildSpec, expected_artifacts};
use crate::cargo::{observation::schema, selection::is_normal_component};
use std::{
    collections::BTreeSet,
    path::{Path, PathBuf},
    sync::Arc,
};

#[derive(Clone, Debug, Eq, PartialEq)]
pub(super) struct WasmArtifactTarget {
    pub(super) package_id: String,
    pub(super) name: String,
    pub(super) kind: schema::TargetKind,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub(super) struct ResolvedWasmOutputs {
    pub(super) targets: Arc<[WasmArtifactTarget]>,
    pub(super) artifact_paths: Arc<[PathBuf]>,
}

impl ResolvedWasmOutputs {
    pub(super) fn new(spec: &WasmBuildSpec, targets: Vec<WasmArtifactTarget>) -> Self {
        Self {
            artifact_paths: expected_artifacts(spec, &spec.target_dir, &targets).into(),
            targets: targets.into(),
        }
    }
}

// Capture the projection with the input snapshot, but defer its refusal until
// artifact_paths or Wasm acquisition is requested. Generic Cargo evidence does
// not require a cdylib; reading the captured projection never probes Cargo again.
pub(super) fn select_targets<'a>(
    packages: impl IntoIterator<Item = &'a schema::Package>,
    example: Option<&str>,
) -> Result<Vec<WasmArtifactTarget>, String> {
    let mut artifact_names = BTreeSet::new();
    packages
        .into_iter()
        .map(|package| {
            let mut matching = package.targets.iter().filter(|target| {
                target.crate_types.contains(&schema::CrateType::CDyLib)
                    && example.map_or_else(
                        || target.is_cdylib(),
                        |example| target.name == example && target.is_example(),
                    )
            });
            let Some(target) = matching.next() else {
                return Err(format!(
                    "Cargo package `{}` must declare the selected cdylib target",
                    package.name
                ));
            };
            if matching.next().is_some() || !is_normal_component(Path::new(&target.name)) {
                return Err(format!(
                    "Cargo package `{}` has an ambiguous or unsafe cdylib target",
                    package.name
                ));
            }
            let cache_name = target.name.replace('-', "_");
            if !artifact_names.insert(cache_name.clone()) {
                return Err(format!(
                    "selected Cargo packages share the Wasm artifact name `{cache_name}`"
                ));
            }
            Ok(WasmArtifactTarget {
                package_id: package.id.repr.clone(),
                name: target.name.clone(),
                kind: if example.is_some() {
                    schema::TargetKind::Example
                } else {
                    schema::TargetKind::CDyLib
                },
            })
        })
        .collect()
}
