//! Explicit locked shared-target retention.

use super::{
    CacheRetentionLimits, SharedIncrementalTargetMaintenance, WasmBuildError,
    record_cache_entry_use,
};
use crate::artifacts::cache_fs::cache_entry_last_used;
use ic_host_fs::{owned_directory::remove_path_if_present, read::directory_logical_size};
use std::{
    fs,
    path::Path,
    time::{Duration, Instant, SystemTime},
};

pub(super) fn maintain_shared_incremental_target_locked(
    canonical: &Path,
    policy: CacheRetentionLimits,
    lock_wait: Duration,
) -> Result<SharedIncrementalTargetMaintenance, WasmBuildError> {
    let started = Instant::now();
    let logical_size_bytes_before = directory_logical_size(canonical).map_err(|source| {
        WasmBuildError::io(
            "measure shared incremental Cargo target before maintenance",
            canonical.to_owned(),
            source,
        )
    })?;
    let last_used_before = cache_entry_last_used(canonical).map_err(|source| {
        WasmBuildError::io(
            "read shared incremental Cargo target use time before maintenance",
            canonical.to_owned(),
            source,
        )
    })?;
    let expired = policy.max_age().is_some_and(|max_age| {
        SystemTime::now()
            .duration_since(last_used_before)
            .is_ok_and(|age| age > max_age)
    });
    let oversized = policy
        .max_size_bytes()
        .is_some_and(|max_size_bytes| logical_size_bytes_before > max_size_bytes);
    let cleared = expired || oversized;
    if cleared {
        clear_shared_incremental_target_contents(canonical)?;
        record_cache_entry_use(canonical)?;
    }
    let logical_size_bytes_after = if cleared {
        directory_logical_size(canonical).map_err(|source| {
            WasmBuildError::io(
                "measure shared incremental Cargo target after maintenance",
                canonical.to_owned(),
                source,
            )
        })?
    } else {
        logical_size_bytes_before
    };
    Ok(SharedIncrementalTargetMaintenance {
        target_dir: canonical.to_owned(),
        logical_size_bytes_before,
        logical_size_bytes_after,
        last_used_before,
        cleared,
        lock_wait,
        maintenance: started.elapsed(),
    })
}

fn clear_shared_incremental_target_contents(target_dir: &Path) -> Result<(), WasmBuildError> {
    let entries = fs::read_dir(target_dir).map_err(|source| {
        WasmBuildError::io(
            "read shared incremental Cargo target for maintenance",
            target_dir.to_owned(),
            source,
        )
    })?;
    for entry in entries {
        let path = entry
            .map_err(|source| {
                WasmBuildError::io(
                    "read shared incremental Cargo target entry for maintenance",
                    target_dir.to_owned(),
                    source,
                )
            })?
            .path();
        let preserved = path
            .file_name()
            .is_some_and(|name| name == ".ic-testkit" || name == "CACHEDIR.TAG");
        if !preserved {
            remove_path_if_present(&path).map_err(|source| {
                WasmBuildError::io("clear shared incremental Cargo target entry", path, source)
            })?;
        }
    }
    Ok(())
}
