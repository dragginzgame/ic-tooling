//! Prepared source leases and shared input resolution for independent build batches.
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
    sync::{
        Arc, RwLock,
        atomic::{AtomicUsize, Ordering},
    },
    time::{Duration, Instant},
};

pub(in crate::artifacts) struct WasmBuildBatchInputResolver<'a, 'session> {
    entries: Vec<BatchResolutionEntry<'a>>,
    groups: Vec<BatchResolutionGroup>,
    reuse: Option<&'session WasmBuildInputSnapshotState>,
    metrics: WasmBuildBatchInputMetrics,
}

struct BatchResolutionEntry<'a> {
    spec: &'a WasmBuildSpec,
    group: usize,
    resolved: Option<Result<ResolvedCargoBuildInputs, (WasmBuildPhase, WasmBuildError)>>,
}

pub(in crate::artifacts) struct WasmBuildInputSnapshotState {
    snapshots: Vec<(WasmBuildSpec, ResolvedCargoBuildInputs)>,
    preparation_metrics: WasmBuildBatchInputMetrics,
    preparation_timings: WasmInputResolutionTimings,
    reader_reuses: AtomicUsize,
    invalidation: Arc<RwLock<bool>>,
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
    pub(in crate::artifacts) prepared_reuses: usize,
}

#[derive(Eq, Hash, PartialEq)]
struct BatchResolutionKey<'a> {
    workspace_root: &'a Path,
    cargo_program: &'a OsStr,
    rustc_program: &'a OsStr,
    metadata_arguments: Vec<OsString>,
    environment: &'a BTreeMap<OsString, OsString>,
}

impl WasmBuildInputSnapshotState {
    pub(in crate::artifacts) fn prepare(specs: &[WasmBuildSpec]) -> Result<Self, WasmBuildError> {
        for spec in specs {
            validate_spec(spec)?;
        }
        let mut resolver = WasmBuildBatchInputResolver::new(specs, None)
            .map_err(|_| WasmBuildError::PreparedInputSnapshotInvalidated)?;
        let mut snapshots = Vec::with_capacity(specs.len());
        let mut preparation_timings = WasmInputResolutionTimings::default();
        let mut progress = ProgressReporter::silent();
        for (index, spec) in specs.iter().enumerate() {
            let mut prepared_input = resolver.resolve(index, &mut progress)?;
            preparation_timings.include(prepared_input.timings);
            prepared_input.timings = WasmInputResolutionTimings::default();
            snapshots.push((spec.clone(), prepared_input));
        }
        Ok(Self {
            snapshots,
            preparation_metrics: resolver.metrics(),
            preparation_timings,
            reader_reuses: AtomicUsize::new(0),
            invalidation: Arc::new(RwLock::new(false)),
        })
    }

    fn admit(&self, specs: &[&WasmBuildSpec]) -> Result<Vec<ResolvedCargoBuildInputs>, usize> {
        let matched = specs
            .iter()
            .enumerate()
            .map(|(index, spec)| {
                self.snapshots
                    .iter()
                    .find(|(candidate, _)| candidate == *spec)
                    .map(|(_, snapshot)| snapshot.clone())
                    .ok_or(index)
            })
            .collect::<Result<Vec<_>, _>>()?;
        let _ = self
            .reader_reuses
            .try_update(Ordering::Relaxed, Ordering::Relaxed, |current| {
                Some(current.saturating_add(matched.len()))
            });
        Ok(matched)
    }

    pub(in crate::artifacts) const fn specification_count(&self) -> usize {
        self.snapshots.len()
    }

    pub(in crate::artifacts) const fn preparation_metrics(&self) -> WasmBuildBatchInputMetrics {
        self.preparation_metrics
    }

    pub(in crate::artifacts) const fn preparation_timings(&self) -> WasmInputResolutionTimings {
        self.preparation_timings
    }

    pub(in crate::artifacts) fn reader_reuses(&self) -> usize {
        self.reader_reuses.load(Ordering::Relaxed)
    }

    pub(in crate::artifacts) fn is_invalidated(&self) -> bool {
        *self
            .invalidation
            .read()
            .unwrap_or_else(std::sync::PoisonError::into_inner)
    }

    fn invalidate(&self) {
        *self
            .invalidation
            .write()
            .unwrap_or_else(std::sync::PoisonError::into_inner) = true;
    }

    fn invalidation(&self) -> Arc<RwLock<bool>> {
        Arc::clone(&self.invalidation)
    }
}

impl<'a, 'session> WasmBuildBatchInputResolver<'a, 'session> {
    pub(in crate::artifacts) fn new(
        specs: impl IntoIterator<Item = &'a WasmBuildSpec>,
        reuse: Option<&'session WasmBuildInputSnapshotState>,
    ) -> Result<Self, usize> {
        let specs = specs.into_iter().collect::<Vec<_>>();
        let prepared = match reuse.as_ref() {
            Some(snapshot) => Some(snapshot.admit(&specs)?),
            _ => None,
        };
        let mut groups_by_key = HashMap::new();
        let mut groups = Vec::<BatchResolutionGroup>::new();
        let mut entries = Vec::with_capacity(specs.len());
        let mut metrics = WasmBuildBatchInputMetrics::default();
        let mut prepared = prepared.map(|prepared| {
            metrics.prepared_reuses = prepared.len();
            prepared.into_iter()
        });
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
                resolved: prepared.as_mut().map(|snapshots| {
                    Ok(snapshots
                        .next()
                        .expect("admitted snapshot must cover every specification"))
                }),
            });
        }
        Ok(Self {
            entries,
            groups,
            reuse,
            metrics,
        })
    }

    pub(in crate::artifacts) const fn metrics(&self) -> WasmBuildBatchInputMetrics {
        self.metrics
    }

    pub(in crate::artifacts) fn invalidate_source_lease(&self) {
        if let Some(snapshot) = self.reuse {
            snapshot.invalidate();
        }
    }

    pub(in crate::artifacts) fn prepared_invalidation(&self) -> Option<Arc<RwLock<bool>>> {
        self.reuse.map(WasmBuildInputSnapshotState::invalidation)
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

#[cfg(all(test, unix))]
mod tests;
