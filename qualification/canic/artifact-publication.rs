//! Export-only mapping of Canic output destination policy to held custody.
use crate::{Error, Result, supplier};
use ic_fs_next::{Commit, Destination, Directory, Policy, Retention};
use std::ffi::OsString;
use std::os::unix::ffi::OsStrExt;
use std::path::{Component, Path};
use std::sync::atomic::{AtomicU64, Ordering};

static STAGE: AtomicU64 = AtomicU64::new(0);

#[derive(Debug, thiserror::Error)]
enum Failure {
    #[error("output not published: {cause}; staging cleanup: {cleanup:?}")]
    NotPublished {
        #[source]
        cause: ic_fs_next::Error,
        cleanup: Option<ic_fs_next::Error>,
    },
    #[error(
        "output visible but unconfirmed at {phase:?}: {cause}; receipt: {receipt:?}; sync: {sync_error:?}"
    )]
    Unconfirmed {
        #[source]
        cause: ic_fs_next::Error,
        phase: ic_fs_next::Phase,
        receipt: ic_fs_next::Receipt,
        sync_error: Option<std::io::Error>,
    },
}

pub(crate) fn publish(
    destination: &Path,
    bytes: &[u8],
    mode: Destination,
    parent_mode: u32,
) -> Result<()> {
    let raw = destination.as_os_str().as_bytes();
    if raw.contains(&0)
        || matches!(
            raw.rsplit(|byte| *byte == b'/').next(),
            None | Some(b"" | b"." | b"..")
        )
    {
        return Err(Error::Invalid("output needs a normal filename".into()));
    }
    // Keep path authority in Canic. Parent traversal refuses before creation;
    // root/current-directory selection happens once, then all work is relative.
    let mut names = Vec::new();
    for component in destination.components() {
        match component {
            Component::Normal(name) => names.push(name),
            Component::RootDir | Component::CurDir => {}
            _ => {
                return Err(Error::Invalid(
                    "output cannot contain parent traversal".into(),
                ));
            }
        }
    }
    let name = names
        .pop()
        .ok_or_else(|| Error::Invalid("output needs a filename".into()))?;
    let anchor = if destination.is_absolute() {
        Path::new("/")
    } else {
        Path::new(".")
    };
    let root = Directory::open(anchor, 0)
        .map_err(supplier)?
        .ok_or_else(|| Error::Invalid("output root disappeared".into()))?;
    // The calling writer selects parent policy; existing modes remain unchanged.
    let parent = root
        .prepare_directories(&names, parent_mode, 0)
        .map_err(supplier)?;
    let staging = OsString::from(format!(
        ".canic-artifact-{}-{}",
        std::process::id(),
        STAGE.fetch_add(1, Ordering::Relaxed)
    ));
    let sealed = parent
        .stage(
            &staging,
            name,
            Policy {
                destination: mode,
                permissions: 0o600,
                retention: Retention::Discard,
            },
        )
        .map_err(supplier)?
        .write_from(
            &mut &*bytes,
            ic_bytes_next::Length::Exactly(bytes.len() as u64),
        )
        .map_err(supplier)?
        .seal()
        .map_err(supplier)?;
    match sealed.commit() {
        Commit::Committed(_) => Ok(()),
        Commit::NotPublished { cause, stage } => Err(supplier(Failure::NotPublished {
            cause,
            cleanup: stage.abort().err(),
        })),
        Commit::PublishedUnconfirmed {
            receipt,
            phase,
            cause,
            sync_error,
        } => Err(supplier(Failure::Unconfirmed {
            receipt,
            phase,
            cause,
            sync_error,
        })),
    }
}
