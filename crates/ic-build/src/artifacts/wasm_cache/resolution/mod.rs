//! Cargo metadata closure selection, configuration discovery and input hashing.

use super::{
    ResolvedLocalInputs, WasmBuildError, WasmBuildSpec, add_if_present, invalid_metadata,
    shared_incremental_target, source_inputs, wasm_cache_fs_error,
};
use crate::{
    artifacts::{
        cache_fs::canonicalize_allow_missing,
        digest::{
            InputDigest, InputHasher, LabeledPathDigestCache, digest_bytes,
            digest_labeled_paths_composable, os_bytes,
        },
    },
    cargo::observation::schema,
};
use std::{
    cell::OnceCell,
    collections::{BTreeSet, HashMap, HashSet, VecDeque},
    ffi::{OsStr, OsString},
    fs,
    path::{Path, PathBuf},
};
use toml::Value as TomlValue;

#[cfg(test)]
mod tests;

// Parsed once per Cargo resolution context. Each specification still selects
// its own closure and independently validates filesystem inputs.
pub(super) struct ParsedCargoMetadata<'a> {
    packages: HashMap<&'a str, &'a schema::Package>,
    workspace_packages: HashMap<&'a str, Option<&'a str>>,
    nodes: HashMap<&'a str, &'a schema::Node>,
    pub(super) workspace_root: &'a Path,
    workspace: OnceCell<TomlValue>,
}

impl<'a> ParsedCargoMetadata<'a> {
    pub(super) fn parse(metadata: &'a schema::Metadata) -> Result<Self, String> {
        let packages: HashMap<_, _> = metadata
            .packages
            .iter()
            .map(|package| (package.id.repr.as_str(), package))
            .collect();
        let workspace_members: HashSet<_> = metadata
            .workspace_members
            .iter()
            .map(|id| id.repr.as_str())
            .collect();
        let mut workspace_packages = HashMap::new();
        for package in packages
            .values()
            .filter(|package| workspace_members.contains(package.id.repr.as_str()))
        {
            workspace_packages
                .entry(package.name.as_str())
                .and_modify(|id| *id = None)
                .or_insert(Some(package.id.repr.as_str()));
        }
        let resolve = metadata
            .resolve
            .as_ref()
            .ok_or_else(|| "Cargo metadata has no resolved dependency nodes".to_owned())?;
        let nodes = resolve
            .nodes
            .iter()
            .map(|node| (node.id.repr.as_str(), node))
            .collect();
        Ok(Self {
            packages,
            workspace_packages,
            nodes,
            workspace_root: metadata.workspace_root.as_std_path(),
            workspace: OnceCell::new(),
        })
    }

    // This metadata context lives for one discovery group, never across a
    // fresh acquisition or mutation guard. Failed reads remain independently
    // observable by later entries instead of caching a reconstructed error.
    fn workspace(&self) -> Result<&TomlValue, WasmBuildError> {
        if let Some(workspace) = self.workspace.get() {
            return Ok(workspace);
        }
        let workspace = read_workspace_manifest(self.workspace_root)?;
        Ok(self.workspace.get_or_init(|| workspace))
    }
}

pub(super) fn resolve_local_inputs(
    spec: &WasmBuildSpec,
    metadata: &ParsedCargoMetadata<'_>,
) -> Result<(ResolvedLocalInputs, Vec<PathBuf>), WasmBuildError> {
    let mut selected_ids = selected_package_ids(spec, metadata)?;
    let wasm_targets = super::outputs::select_targets(
        selected_ids.iter().map(|id| metadata.packages[id]),
        spec.example.as_deref(),
    );
    let mut closure = BTreeSet::new();
    while let Some(id) = selected_ids.pop_front() {
        if !closure.insert(id) {
            continue;
        }
        if let Some(node) = metadata.nodes.get(id) {
            selected_ids.extend(
                node.deps
                    .iter()
                    .map(|dependency| dependency.pkg.repr.as_str()),
            );
        }
    }

    let workspace_root = metadata.workspace_root;
    let root = metadata.workspace()?;
    let (mut validation_inputs, configuration_identity) =
        workspace_configuration_inputs(spec, workspace_root, root)?;
    let declared_package_files = append_package_inputs(
        &mut validation_inputs,
        &metadata.packages,
        closure,
        workspace_root,
    )?;
    append_additional_inputs(&mut validation_inputs, spec, workspace_root);
    validate_shared_incremental_target_boundary(spec, &validation_inputs)?;
    let base_exclusions = source_exclusions(spec, &validation_inputs);
    let exclusions = base_exclusions.clone();
    Ok((
        ResolvedLocalInputs {
            validation_inputs,
            base_exclusions,
            declared_package_files,
            configuration_identity,
            wasm_targets,
        },
        exclusions,
    ))
}

pub(super) fn selected_package_ids<'a>(
    spec: &WasmBuildSpec,
    metadata: &ParsedCargoMetadata<'a>,
) -> Result<VecDeque<&'a str>, WasmBuildError> {
    let mut selected_ids = VecDeque::new();
    for requested in &spec.packages {
        match metadata.workspace_packages.get(requested.as_str()) {
            Some(Some(id)) => selected_ids.push_back(*id),
            None => {
                return Err(WasmBuildError::InvalidSpec {
                    message: format!("Cargo workspace contains no package named `{requested}`"),
                });
            }
            Some(None) => {
                return Err(WasmBuildError::InvalidSpec {
                    message: format!("Cargo workspace package name `{requested}` is ambiguous"),
                });
            }
        }
    }
    Ok(selected_ids)
}

fn hash_toml_setting(hasher: &mut InputHasher, label: &str, value: Option<&TomlValue>) {
    hasher.field("workspace-setting-name", label.as_bytes());
    match value {
        Some(value) => hasher.field("workspace-setting-value", value.to_string().as_bytes()),
        None => hasher.field("workspace-setting-missing", b""),
    }
}

pub(super) fn digest_resolved_local_inputs(
    inputs: &ResolvedLocalInputs,
    exclusions: &mut Vec<PathBuf>,
    cache: &mut LabeledPathDigestCache,
    declarations: &mut source_inputs::WorkspaceDeclarations,
    error_path: &Path,
    operation: &'static str,
) -> Result<InputDigest, WasmBuildError> {
    // Tree admission is part of hashing, including missing/unreadable inputs.
    // Keep each batch entry's original failure phase and independent outcome.
    source_inputs::exclude_nested_workspace_targets(
        inputs
            .validation_inputs
            .iter()
            .map(|(_, path)| path.as_path()),
        &inputs.declared_package_files,
        exclusions,
        declarations,
    )
    .map_err(|source| WasmBuildError::Io {
        operation,
        path: error_path.to_owned(),
        source,
    })?;
    digest_labeled_paths_composable(
        "wasm-source-inputs-v1",
        inputs
            .validation_inputs
            .iter()
            .map(|(label, path)| (label.as_path(), path.as_path())),
        exclusions,
        cache,
    )
    .map_err(|source| WasmBuildError::Io {
        operation,
        path: error_path.to_owned(),
        source,
    })
}

fn workspace_configuration_inputs(
    spec: &WasmBuildSpec,
    workspace_root: &Path,
    root: &TomlValue,
) -> Result<(Vec<(PathBuf, PathBuf)>, InputDigest), WasmBuildError> {
    let mut inputs = Vec::new();
    add_if_present(
        &mut inputs,
        "workspace/Cargo.toml",
        workspace_root.join("Cargo.toml"),
    );
    add_if_present(
        &mut inputs,
        "workspace/Cargo.lock",
        workspace_root.join("Cargo.lock"),
    );
    add_if_present(
        &mut inputs,
        "workspace/rust-toolchain.toml",
        workspace_root.join("rust-toolchain.toml"),
    );
    add_if_present(
        &mut inputs,
        "workspace/rust-toolchain",
        workspace_root.join("rust-toolchain"),
    );
    let configuration = append_cargo_configuration_inputs(&mut inputs, spec, workspace_root)?;
    let mut hasher = InputHasher::new("cargo-configuration-recipe-v1");
    hasher.field("configuration", configuration.as_bytes());
    hash_toml_setting(&mut hasher, "profile", root.get("profile"));
    hash_toml_setting(&mut hasher, "cargo-features", root.get("cargo-features"));
    hash_toml_setting(
        &mut hasher,
        "resolver",
        root.get("workspace").and_then(|v| v.get("resolver")),
    );
    Ok((inputs, hasher.finish()))
}

pub(super) fn append_cargo_configuration_inputs(
    inputs: &mut Vec<(PathBuf, PathBuf)>,
    spec: &WasmBuildSpec,
    workspace_root: &Path,
) -> Result<InputDigest, WasmBuildError> {
    let invocation_root =
        spec.workspace_root
            .canonicalize()
            .map_err(|source| WasmBuildError::Io {
                operation: "resolve Cargo invocation directory",
                path: spec.workspace_root.clone(),
                source,
            })?;
    let canonical_workspace =
        workspace_root
            .canonicalize()
            .map_err(|source| WasmBuildError::Io {
                operation: "resolve Cargo workspace directory",
                path: workspace_root.to_owned(),
                source,
            })?;

    let cargo_home = command_environment_value(spec, "CARGO_HOME");
    let default_home = default_home_directory(spec);
    let directories = crate::cargo::environment::configuration_directories(
        &invocation_root,
        cargo_home.as_deref(),
        default_home.as_deref().map(Path::as_os_str),
    )
    .map_err(|source| WasmBuildError::Io {
        operation: "resolve Cargo configuration directories",
        path: invocation_root.clone(),
        source,
    })?;
    let roots: Vec<_> = directories
        .iter()
        .filter_map(|directory| crate::cargo::environment::preferred_configuration(directory))
        .collect();
    let observed =
        crate::cargo::environment::observe_configurations(&roots, spec.configuration_limits)
            .map_err(|error| match error {
                crate::cargo::environment::ConfigurationError::Io {
                    operation,
                    path,
                    source,
                } => WasmBuildError::Io {
                    operation,
                    path,
                    source,
                },
                crate::cargo::environment::ConfigurationError::Read { path, source } => {
                    WasmBuildError::InvalidCargoConfiguration {
                        path,
                        message: source.to_string(),
                    }
                }
                crate::cargo::environment::ConfigurationError::Invalid { path, message } => {
                    WasmBuildError::InvalidCargoConfiguration { path, message }
                }
            })?;
    let mut hasher = InputHasher::new("cargo-observed-configuration-recipe-v1");
    for config in observed {
        if let Some(identity) = config.recipe_sha256 {
            hasher.field(
                "configuration-location",
                &os_bytes(config.location.as_os_str()),
            );
            hasher.field("configuration-settings", identity.as_bytes());
        }
        if !inputs.iter().any(|(_, path)| path == &config.path) {
            inputs.push((
                cargo_configuration_label(&config.location, &canonical_workspace),
                config.path,
            ));
        }
    }
    Ok(hasher.finish())
}

#[cfg(windows)]
fn default_home_directory(spec: &WasmBuildSpec) -> Option<PathBuf> {
    command_environment_value(spec, "USERPROFILE")
        .or_else(|| command_environment_value(spec, "HOME"))
        .map(PathBuf::from)
}

#[cfg(not(windows))]
fn default_home_directory(spec: &WasmBuildSpec) -> Option<PathBuf> {
    command_environment_value(spec, "HOME").map(PathBuf::from)
}

fn command_environment_value(spec: &WasmBuildSpec, name: &str) -> Option<OsString> {
    spec.environment.get(OsStr::new(name)).cloned()
}

fn cargo_configuration_label(config: &Path, workspace_root: &Path) -> PathBuf {
    if let Ok(relative) = config.strip_prefix(workspace_root) {
        return PathBuf::from("cargo-config/workspace").join(relative);
    }
    let location = digest_bytes("cargo-config-location-v1", &os_bytes(config.as_os_str()));
    PathBuf::from("cargo-config/external").join(location.to_hex())
}

fn append_package_inputs(
    inputs: &mut Vec<(PathBuf, PathBuf)>,
    packages: &HashMap<&str, &schema::Package>,
    closure: BTreeSet<&str>,
    workspace_root: &Path,
) -> Result<Vec<PathBuf>, WasmBuildError> {
    let mut declared_package_files = Vec::new();
    for id in closure {
        let Some(package) = packages.get(id) else {
            return Err(invalid_metadata(&format!(
                "resolved package `{id}` is missing"
            )));
        };
        if package.source.is_some() {
            continue;
        }
        let root = package.manifest_path.parent().ok_or_else(|| {
            invalid_metadata(&format!(
                "package `{}` manifest has no parent",
                package.name
            ))
        })?;
        let relative_manifest = package
            .manifest_path
            .as_std_path()
            .strip_prefix(workspace_root)
            .unwrap_or(package.manifest_path.as_std_path());
        let label = PathBuf::from(format!("package/{}@{}", package.name, package.version))
            .join(relative_manifest.parent().unwrap_or_else(|| Path::new(".")));
        inputs.push((label, root.as_std_path().to_owned()));
        declared_package_files.push(package.manifest_path.clone().into_std_path_buf());
        declared_package_files.extend(
            package
                .targets
                .iter()
                .map(|target| target.src_path.clone().into_std_path_buf()),
        );
    }
    Ok(declared_package_files)
}

fn append_additional_inputs(
    inputs: &mut Vec<(PathBuf, PathBuf)>,
    spec: &WasmBuildSpec,
    workspace_root: &Path,
) {
    for additional in &spec.additional_inputs {
        let path = if additional.is_absolute() {
            additional.clone()
        } else {
            workspace_root.join(additional)
        };
        inputs.push((PathBuf::from("additional").join(additional), path));
    }
}

fn source_exclusions(spec: &WasmBuildSpec, inputs: &[(PathBuf, PathBuf)]) -> Vec<PathBuf> {
    let mut exclusions = vec![
        spec.target_dir.clone(),
        spec.workspace_root.join("target"),
        spec.workspace_root.join(".git"),
    ];
    exclusions.push(shared_incremental_target(spec));
    for (_, path) in inputs {
        if path.is_dir() {
            exclusions.push(path.join("target"));
            exclusions.push(path.join(".git"));
        }
    }
    exclusions
}

pub(super) fn validate_shared_incremental_target_boundary(
    spec: &WasmBuildSpec,
    inputs: &[(PathBuf, PathBuf)],
) -> Result<(), WasmBuildError> {
    let shared_target = shared_incremental_target(spec);
    let shared_target =
        canonicalize_allow_missing(&shared_target).map_err(|source| WasmBuildError::Io {
            operation: "resolve shared incremental Cargo target boundary",
            path: shared_target.clone(),
            source,
        })?;
    let exact_entries = spec.target_dir.join(".ic-testkit/wasm-targets");
    let exact_entries =
        canonicalize_allow_missing(&exact_entries).map_err(|source| WasmBuildError::Io {
            operation: "resolve exact Wasm cache boundary",
            path: exact_entries,
            source,
        })?;
    // Maintenance preserves only the shared target's direct metadata child.
    // An exact cache elsewhere beneath that target would lose retained entries.
    if shared_target.starts_with(&exact_entries)
        || (exact_entries.starts_with(&shared_target)
            && !exact_entries.starts_with(shared_target.join(".ic-testkit")))
    {
        return Err(WasmBuildError::InvalidSpec {
            message: "shared incremental target must not overlap removable exact Wasm cache state"
                .to_owned(),
        });
    }
    let mut boundaries = crate::artifacts::inputs::boundaries::CargoInputBoundaries::default();
    let resolved_inputs = inputs
        .iter()
        .map(|(_, input)| {
            boundaries
                .input(
                    input,
                    "resolve Cargo input boundary",
                    "inspect Cargo input boundary",
                )
                .cloned()
                .map_err(wasm_cache_fs_error)
        })
        .collect::<Result<Vec<_>, WasmBuildError>>()?;
    let has_safe_generated_root = std::iter::once(spec.target_dir.clone())
        .chain(std::iter::once(spec.workspace_root.join("target")))
        .chain(
            inputs
                .iter()
                .zip(&resolved_inputs)
                .filter(|(_, (_, is_directory))| *is_directory)
                .map(|((_, path), _)| path.join("target")),
        )
        .filter_map(|path| canonicalize_allow_missing(&path).ok())
        .filter(|root| {
            !resolved_inputs
                .iter()
                .any(|(input, _is_directory)| input.starts_with(root))
        })
        // Keep observing every candidate after finding a safe generated root.
        .fold(false, |safe, root| safe | shared_target.starts_with(root));
    if has_safe_generated_root {
        return Ok(());
    }

    for (input, is_directory) in resolved_inputs {
        if shared_target == input
            || (is_directory && shared_target.starts_with(&input))
            || input.starts_with(&shared_target)
        {
            return Err(WasmBuildError::InvalidSpec {
                message: format!(
                    "shared incremental target {} must not overlap exact Cargo inputs unless it is inside a generated target directory",
                    shared_target.display()
                ),
            });
        }
    }
    Ok(())
}

/// Comparison identity deliberately excludes feature selection and source inputs.
pub(super) fn recipe_identity(
    spec: &WasmBuildSpec,
    facts: &super::CargoBuildFacts,
    configuration: InputDigest,
) -> InputDigest {
    let mut hasher = InputHasher::new("cargo-build-comparison-recipe-v1");
    hasher.field("configuration", configuration.as_bytes());
    for package in &spec.packages {
        hasher.field("package", package.as_bytes());
    }
    hasher.field("target", facts.target.as_bytes());
    hasher.field(
        "example",
        spec.example.as_deref().unwrap_or_default().as_bytes(),
    );
    hasher.field(
        "profile",
        facts
            .selection
            .profile
            .as_deref()
            .unwrap_or("dev")
            .as_bytes(),
    );
    hasher.field("cargo", &facts.cargo_identity);
    hasher.field("rustc", &facts.rustc_identity);
    for (name, value) in &facts.environment {
        if name
            .to_str()
            .is_some_and(crate::cargo::environment::is_recipe_variable)
        {
            hasher.field("environment-name", &os_bytes(name));
            hasher.field("environment-value", &os_bytes(value));
        }
    }
    hasher.finish()
}

fn read_workspace_manifest(workspace_root: &Path) -> Result<TomlValue, WasmBuildError> {
    let manifest = workspace_root.join("Cargo.toml");
    let bytes = fs::read_to_string(&manifest).map_err(|source| WasmBuildError::Io {
        operation: "read workspace manifest for input resolution",
        path: manifest.clone(),
        source,
    })?;
    toml::from_str::<TomlValue>(&bytes).map_err(|_| WasmBuildError::InvalidCargoConfiguration {
        path: manifest,
        message: "invalid workspace TOML".to_owned(),
    })
}
