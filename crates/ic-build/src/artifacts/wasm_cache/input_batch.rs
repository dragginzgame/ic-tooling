//! Shared input resolution within one independent build batch.
//! Cache acquisition, publication and target maintenance stay with the parent.
use super::{
    DiscoveredBuildInputs, LabeledPathDigestCache, ParsedCargoMetadata, ProgressReporter,
    ResolvedCargoBuildInputs, WasmBuildError, WasmBuildPhase, WasmBuildSpec,
    WasmInputResolutionTimings, invalid_metadata, observe_cargo_resolution, schema, source_inputs,
    validate_spec,
};
use std::{
    collections::{BTreeMap, HashMap},
    ffi::{OsStr, OsString},
    path::Path,
    time::{Duration, Instant},
};

pub(in crate::artifacts) struct WasmBuildBatchInputResolver<'a> {
    entries: Vec<BatchResolutionEntry<'a>>,
    groups: Vec<BatchResolutionGroup>,
    metrics: WasmBuildBatchInputMetrics,
}

struct BatchResolutionEntry<'a> {
    spec: &'a WasmBuildSpec,
    group: usize,
    resolved: Option<Result<ResolvedCargoBuildInputs, (WasmBuildPhase, WasmBuildError)>>,
}

struct BatchResolutionGroup {
    indexes: Vec<usize>,
}

struct DiscoveredBatchInputs {
    index: usize,
    discovered: DiscoveredBuildInputs,
}

#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub(in crate::artifacts) struct WasmBuildBatchInputMetrics {
    pub(in crate::artifacts) runs: usize,
    pub(in crate::artifacts) reuses: usize,
}

#[derive(Eq, Hash, PartialEq)]
struct BatchResolutionKey<'a> {
    workspace_root: &'a Path,
    cargo_program: &'a OsStr,
    rustc_program: &'a OsStr,
    metadata_arguments: Vec<OsString>,
    environment: &'a BTreeMap<OsString, OsString>,
}

impl<'a> WasmBuildBatchInputResolver<'a> {
    pub(in crate::artifacts) fn new(specs: impl IntoIterator<Item = &'a WasmBuildSpec>) -> Self {
        let mut groups_by_key = HashMap::new();
        let mut groups = Vec::<BatchResolutionGroup>::new();
        let mut entries = Vec::new();
        for (index, spec) in specs.into_iter().enumerate() {
            let group = *groups_by_key
                .entry(BatchResolutionKey::for_spec(spec))
                .or_insert_with(|| {
                    groups.push(BatchResolutionGroup {
                        indexes: Vec::new(),
                    });
                    groups.len() - 1
                });
            groups[group].indexes.push(index);
            entries.push(BatchResolutionEntry {
                spec,
                group,
                resolved: None,
            });
        }
        Self {
            entries,
            groups,
            metrics: WasmBuildBatchInputMetrics::default(),
        }
    }

    pub(in crate::artifacts) const fn metrics(&self) -> WasmBuildBatchInputMetrics {
        self.metrics
    }

    pub(super) fn resolve(
        &mut self,
        index: usize,
        progress: &mut ProgressReporter<'_>,
    ) -> Result<ResolvedCargoBuildInputs, WasmBuildError> {
        if self.entries[index].resolved.is_none() {
            self.resolve_group(index, progress)?;
        }
        self.entries[index]
            .resolved
            .take()
            .expect("resolved batch input must be populated")
            .map_err(|(phase, error)| {
                progress.begin_phase(phase);
                error
            })
    }

    fn resolve_group(
        &mut self,
        active_index: usize,
        progress: &mut ProgressReporter<'_>,
    ) -> Result<(), WasmBuildError> {
        let total_started = Instant::now();
        let group = self.entries[active_index].group;
        let active = self.entries[active_index].spec;

        let (facts, metadata, mut timings) = observe_cargo_resolution(active, progress, &mut None)?;

        let (discovered, input_discovery) = self.discover_group_inputs(group, &metadata, progress);

        let hashing_started = Instant::now();
        let mut batch_digest_cache = LabeledPathDigestCache::default();
        let mut declarations = source_inputs::WorkspaceDeclarations::new();
        let digest_cache = &mut batch_digest_cache;
        let workspace_root = &active.workspace_root;
        let resolved_inputs = progress.run_phase(WasmBuildPhase::ContentHashing, || {
            discovered
                .into_iter()
                .map(|discovered| {
                    let result = discovered.discovered.hash(
                        digest_cache,
                        &mut declarations,
                        workspace_root,
                        (
                            "hash batched Wasm build inputs",
                            "hash batched semantic Wasm build inputs",
                        ),
                    );
                    (discovered.index, result)
                })
                .collect::<Vec<_>>()
        });
        let content_hashing = hashing_started.elapsed();
        timings.input_discovery = input_discovery;
        timings.content_hashing = content_hashing;
        timings.total = total_started.elapsed();
        let resolved_count = resolved_inputs
            .iter()
            .filter(|(_, result)| result.is_ok())
            .count();
        self.metrics.runs += usize::from(resolved_count > 0);
        self.metrics.reuses += resolved_count.saturating_sub(1);
        let timing_index = resolved_inputs
            .iter()
            .find(|(index, result)| *index == active_index && result.is_ok())
            .or_else(|| resolved_inputs.iter().find(|(_, result)| result.is_ok()))
            .map(|(index, _)| *index);
        for (index, result) in resolved_inputs {
            let hashed = match result {
                Ok(resolved) => resolved,
                Err(error) => {
                    self.entries[index].resolved =
                        Some(Err((WasmBuildPhase::ContentHashing, error)));
                    continue;
                }
            };
            let spec = self.entries[index].spec;
            let resolved = ResolvedCargoBuildInputs::assemble(
                spec,
                hashed,
                spec.cargo_invocation()
                    .facts(facts.cargo_identity.clone(), facts.rustc_identity.clone()),
                || {
                    if Some(index) == timing_index {
                        timings
                    } else {
                        WasmInputResolutionTimings::default()
                    }
                },
            );
            self.entries[index].resolved = Some(Ok(resolved));
        }
        Ok(())
    }

    fn discover_group_inputs(
        &mut self,
        group: usize,
        metadata: &schema::Metadata,
        progress: &mut ProgressReporter<'_>,
    ) -> (Vec<DiscoveredBatchInputs>, Duration) {
        let started = Instant::now();
        let indexes = &self.groups[group].indexes;
        let entries = &mut self.entries;
        let discovered = progress.run_phase(WasmBuildPhase::InputDiscovery, || {
            let parsed = ParsedCargoMetadata::parse(metadata);
            let mut discovered = Vec::new();
            for &index in indexes {
                let entry = &mut entries[index];
                if entry.resolved.is_some() || validate_spec(entry.spec).is_err() {
                    continue;
                }
                let result = parsed
                    .as_ref()
                    .map_err(|message| invalid_metadata(message))
                    .and_then(|parsed| DiscoveredBuildInputs::resolve(entry.spec, parsed));
                match result {
                    Ok(inputs) => discovered.push(DiscoveredBatchInputs {
                        index,
                        discovered: inputs,
                    }),
                    Err(error) => {
                        entry.resolved = Some(Err((WasmBuildPhase::InputDiscovery, error)));
                    }
                }
            }
            discovered
        });
        (discovered, started.elapsed())
    }
}

impl<'a> BatchResolutionKey<'a> {
    fn for_spec(spec: &'a WasmBuildSpec) -> Self {
        Self {
            workspace_root: &spec.workspace_root,
            cargo_program: &spec.cargo_program,
            rustc_program: spec
                .environment
                .get(OsStr::new("RUSTC"))
                .unwrap_or(&spec.rustc_program),
            metadata_arguments: spec.cargo_selection.metadata_arguments(),
            environment: &spec.environment,
        }
    }
}
