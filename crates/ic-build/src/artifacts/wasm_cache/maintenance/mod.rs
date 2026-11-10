//! Locked shared-target retention, scheduling and outcome projection.

use super::{
    CacheRetentionLimits, SharedIncrementalTargetMaintenance,
    SharedIncrementalTargetMaintenanceOutcome, WasmBuildError, record_cache_entry_use,
    wasm_cache_fs_error,
};
use crate::artifacts::cache_fs::{
    cache_entry_last_used, cache_maintenance_due, record_cache_maintenance,
};
use ic_host_fs::{owned_directory::remove_path_if_present, read::directory_logical_size};
use std::{
    fs,
    path::{Path, PathBuf},
    time::{Duration, Instant, SystemTime},
};

pub(super) enum SharedIncrementalTargetMaintenanceSchedule {
    Skipped(SharedIncrementalTargetMaintenanceOutcome),
    Due(DueSharedIncrementalTargetMaintenance),
}

pub(super) struct DueSharedIncrementalTargetMaintenance {
    schedule_root: PathBuf,
    maintenance_identity: String,
    schedule_check: Duration,
}

pub(super) fn schedule_shared_incremental_target_maintenance(
    canonical: &Path,
    policy: CacheRetentionLimits,
    minimum_interval: Duration,
    lock_wait: Duration,
) -> Result<SharedIncrementalTargetMaintenanceSchedule, WasmBuildError> {
    let schedule_root = canonical.join(".ic-testkit");
    let maintenance_identity = policy.maintenance_identity();
    let schedule_started = Instant::now();
    let due = cache_maintenance_due(&schedule_root, minimum_interval, &maintenance_identity)
        .map_err(wasm_cache_fs_error)?;
    let schedule_check = schedule_started.elapsed();
    if !due {
        return Ok(SharedIncrementalTargetMaintenanceSchedule::Skipped(
            SharedIncrementalTargetMaintenanceOutcome::Skipped {
                target_dir: canonical.to_owned(),
                lock_wait,
                schedule_check,
            },
        ));
    }
    Ok(SharedIncrementalTargetMaintenanceSchedule::Due(
        DueSharedIncrementalTargetMaintenance {
            schedule_root,
            maintenance_identity,
            schedule_check,
        },
    ))
}

pub(super) fn perform_due_shared_incremental_target_maintenance(
    canonical: &Path,
    policy: CacheRetentionLimits,
    lock_wait: Duration,
    due: DueSharedIncrementalTargetMaintenance,
) -> Result<SharedIncrementalTargetMaintenanceOutcome, WasmBuildError> {
    let DueSharedIncrementalTargetMaintenance {
        schedule_root,
        maintenance_identity,
        schedule_check,
    } = due;
    let maintenance = maintain_shared_incremental_target_locked(canonical, policy, lock_wait)?;
    record_cache_maintenance(&schedule_root, &maintenance_identity).map_err(wasm_cache_fs_error)?;
    Ok(SharedIncrementalTargetMaintenanceOutcome::Performed {
        maintenance,
        schedule_check,
    })
}

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
