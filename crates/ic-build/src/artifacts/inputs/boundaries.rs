//! Filesystem facts scoped to one pass of pre-effect input boundary validation.

use crate::artifacts::cache_fs::{CacheFsError, canonicalize_allow_missing};
use std::{
    collections::{BTreeMap, btree_map::Entry},
    fs,
    path::{Path, PathBuf},
};

#[derive(Default)]
pub(in crate::artifacts) struct CargoInputBoundaries<'a> {
    exclusions: BTreeMap<&'a Path, PathBuf>,
    inputs: BTreeMap<&'a Path, (PathBuf, bool)>,
}

impl<'a> CargoInputBoundaries<'a> {
    pub(in crate::artifacts) fn exclusion(
        &mut self,
        path: &'a Path,
    ) -> Result<&Path, CacheFsError> {
        Ok(match self.exclusions.entry(path) {
            Entry::Occupied(entry) => entry.into_mut(),
            Entry::Vacant(entry) => {
                let canonical =
                    canonicalize_allow_missing(path).map_err(|source| CacheFsError {
                        operation: "resolve Cargo build input exclusion",
                        path: path.to_owned(),
                        source,
                    })?;
                entry.insert(canonical)
            }
        })
    }

    pub(in crate::artifacts) fn input(
        &mut self,
        path: &'a Path,
        resolve_operation: &'static str,
        inspect_operation: &'static str,
    ) -> Result<&(PathBuf, bool), CacheFsError> {
        match self.inputs.entry(path) {
            Entry::Occupied(entry) => Ok(entry.into_mut()),
            Entry::Vacant(entry) => {
                let canonical = path.canonicalize().map_err(|source| CacheFsError {
                    operation: resolve_operation,
                    path: path.to_owned(),
                    source,
                })?;
                let is_directory = fs::metadata(&canonical)
                    .map_err(|source| CacheFsError {
                        operation: inspect_operation,
                        path: canonical.clone(),
                        source,
                    })?
                    .is_dir();
                Ok(entry.insert((canonical, is_directory)))
            }
        }
    }
}
