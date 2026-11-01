//! Explicit Cargo execution, bounded observations and feature-safe package grouping.
//!
//! Responsibility: execute selected recipes, observe complete Cargo evidence and preserve isolated dependency trees.
//! Does not own: recipe selection, source custody, role contracts or runtime lifecycle.
//! Boundary: shared dependency feature unification must not change a role's isolated build.

#[cfg(test)]
mod tests;

pub mod diagnostics;
pub mod discovery;
pub mod environment;
pub mod execution;
pub mod flags;
pub mod lock_projection;
pub(crate) mod lockfile;
pub(crate) mod messages;
#[cfg(unix)]
pub mod native;
pub mod observation;
pub mod packages;
pub mod release_metadata;
pub mod rustc_recipe;
pub mod selection;

use ic_host_artifacts::batch::validate_labels;
use std::{collections::BTreeMap, path::Path};

/// Invalid Cargo dependency-tree evidence returned by a grouping observer.
#[derive(Debug, thiserror::Error, PartialEq, Eq)]
pub enum CargoTreeError {
    /// A root line did not name a package.
    #[error("empty Cargo tree root")]
    EmptyRoot,
    /// A selected root appeared more than once.
    #[error("duplicate Cargo tree root")]
    DuplicateRoot,
    /// Dependency lines appeared before any root.
    #[error("Cargo dependency has no selected root")]
    MissingRoot,
    /// The observer returned no dependency trees.
    #[error("empty Cargo dependency forest")]
    EmptyForest,
    /// Missing or additional roots prevent comparison with the requested selection.
    #[error("Cargo dependency roots do not match the selected packages")]
    SelectedRootsMismatch,
}

/// Partition packages without changing their resolved normal/build dependency trees.
///
/// The caller supplies the recipe's Cargo selection, target and tree observer.
/// The observer executes each specification with the selected command, complete
/// environment, source custody and output/deadline policy. It must admit success
/// and completeness before returning UTF-8 text. Every isolated
/// and combined observation uses that selection's features, lock and network
/// choices. Profiles do not affect Cargo tree and are not rendered here.
/// This does not publish artifacts or
/// establish a consumer's release validity.
/// Package names must be nonempty and unique; invalid inputs are rejected before
/// invoking the observer, with zero-based indices in the error. An empty
/// package list returns an empty plan without invoking the observer.
///
/// Packages are considered in input order and only appended to the most recent
/// group. A feature conflict starts a new group; earlier groups are not retried.
/// For `n > 0` packages this makes at most `2 * n - 1` observer calls, including
/// each isolated tree. Groups may be conservative rather than maximally packed.
/// Every candidate is observed through Cargo; no feature union is inferred from
/// isolated metadata and no observations are reused across calls.
///
/// # Errors
/// Returns invalid package input, the observer's original boxed error without
/// conversion, or [`CargoTreeError`] for invalid dependency-tree evidence.
/// Capture error identities belong to the observer's runner, independently of
/// Build's Host generation.
pub fn compatible_package_groups(
    workspace: &Path,
    packages: &[String],
    target: &str,
    selection: &selection::CargoSelection,
    mut observe_tree: impl FnMut(
        &observation::CargoTreeSpec<'_>,
    ) -> Result<String, Box<dyn std::error::Error>>,
) -> Result<Vec<Vec<String>>, Box<dyn std::error::Error>> {
    validate_labels(packages.iter().map(String::as_str))?;
    let mut current_trees = BTreeMap::new();
    let mut groups: Vec<Vec<String>> = Vec::new();
    for package in packages {
        let tree = resolved_trees(
            workspace,
            &[package.as_str()],
            target,
            selection,
            &mut observe_tree,
        )?;
        current_trees.extend(tree);
        if let Some(group) = groups.last_mut() {
            let candidate = group
                .iter()
                .map(String::as_str)
                .chain(std::iter::once(package.as_str()))
                .collect::<Vec<_>>();
            let combined =
                resolved_trees(workspace, &candidate, target, selection, &mut observe_tree)?;
            if preserves_individual_trees(&current_trees, &combined, &candidate) {
                group.push(package.clone());
                continue;
            }
        }
        // Earlier groups are sealed: only this package can enter the next one.
        current_trees.retain(|name, _| name == package);
        groups.push(vec![package.clone()]);
    }
    Ok(groups)
}

fn resolved_trees(
    workspace: &Path,
    packages: &[&str],
    target: &str,
    selection: &selection::CargoSelection,
    observe_tree: &mut impl FnMut(
        &observation::CargoTreeSpec<'_>,
    ) -> Result<String, Box<dyn std::error::Error>>,
) -> Result<BTreeMap<String, String>, Box<dyn std::error::Error>> {
    let manifest = workspace.join("Cargo.toml");
    let tree = observe_tree(&observation::CargoTreeSpec {
        manifest_path: &manifest,
        packages,
        target,
        edges: "normal,build",
        format: "{p}\t{f}",
        locked: selection.locked,
        offline: selection.offline,
        features: &selection.features,
    })?;
    let trees = parse_trees(&tree)?;
    if trees.len() != packages.len() || packages.iter().any(|package| !trees.contains_key(*package))
    {
        return Err(CargoTreeError::SelectedRootsMismatch.into());
    }
    Ok(trees)
}

fn parse_trees(text: &str) -> Result<BTreeMap<String, String>, CargoTreeError> {
    let mut trees = BTreeMap::new();
    let mut root = None;
    for line in text.lines().filter(|line| !line.is_empty()) {
        if let Some(package) = line.strip_prefix('0') {
            let name = package
                .split_whitespace()
                .next()
                .ok_or(CargoTreeError::EmptyRoot)?;
            if trees.insert(name.to_string(), String::new()).is_some() {
                return Err(CargoTreeError::DuplicateRoot);
            }
            root = Some(name);
        }
        let tree = root
            .and_then(|name| trees.get_mut(name))
            .ok_or(CargoTreeError::MissingRoot)?;
        tree.push_str(line);
        tree.push('\n');
    }
    if trees.is_empty() {
        return Err(CargoTreeError::EmptyForest);
    }
    Ok(trees)
}

fn preserves_individual_trees(
    isolated: &BTreeMap<String, String>,
    combined: &BTreeMap<String, String>,
    packages: &[impl AsRef<str>],
) -> bool {
    combined.len() == packages.len()
        && packages.iter().all(|name| {
            isolated
                .get(name.as_ref())
                .is_some_and(|expected| combined.get(name.as_ref()) == Some(expected))
        })
}
