use super::{Directory, Error, name, valid_mode};
use rustix::fs::{self, Mode};
use std::ffi::OsStr;
use std::io;

/// Directory preparation stopped; any created prefix remains for reconciliation.
#[derive(Debug, thiserror::Error)]
#[error("directory preparation refused at component {component:?}: {source}")]
pub struct PrepareFailure {
    /// Zero-based component, or `None` for creation-policy refusal before traversal.
    pub component: Option<usize>,
    /// Original admission or native cause. Sync refusal may follow visible creation.
    #[source]
    pub source: Error,
}

impl Directory {
    /// Prepare explicit children and return the held leaf without reopening a path.
    /// All names and mode policy are validated before effects. Existing children
    /// retain their permissions; new modes are reduced by the process umask.
    /// Newly observed missing children and their parent links are synchronized.
    /// An empty slice retains this root, whose admission policy is unchanged.
    /// Callers exclude concurrent namespace mutation except cooperative creation.
    /// # Errors
    /// Refuses links, non-directories, forbidden permissions and original I/O.
    /// Created prefixes remain, including a possibly created failed component.
    /// No automatic cleanup or rollback occurs after admission/sync failure.
    pub fn prepare_directories(
        &self,
        names: &[&OsStr],
        mode: u32,
        forbidden: u32,
    ) -> Result<Self, PrepareFailure> {
        self.prepare_with(names, mode, forbidden, &mut Self::sync)
    }

    fn prepare_with(
        &self,
        names: &[&OsStr],
        mode: u32,
        forbidden: u32,
        sync: &mut impl FnMut(&Self) -> io::Result<()>,
    ) -> Result<Self, PrepareFailure> {
        let policy = || {
            valid_mode(mode)?;
            if mode & forbidden != 0 {
                return Err(Error::Permissions { mode, forbidden });
            }
            Ok(())
        };
        policy().map_err(|source| PrepareFailure {
            component: None,
            source,
        })?;
        for (index, child) in names.iter().enumerate() {
            name(child).map_err(|source| PrepareFailure {
                component: Some(index),
                source,
            })?;
        }
        let mut parent = self.clone();
        for (index, child) in names.iter().enumerate() {
            let mut step = || -> Result<Self, Error> {
                if let Some(existing) = parent.open_directory(child, forbidden)? {
                    return Ok(existing);
                }
                match fs::mkdirat(&*parent.file, *child, Mode::from_bits_truncate(mode)) {
                    Ok(()) | Err(rustix::io::Errno::EXIST) => {}
                    Err(error) => return Err(error.into()),
                }
                let created = parent
                    .open_directory(child, forbidden)?
                    .ok_or(Error::Changed)?;
                // A cooperative creator may have stopped after mkdir; sync both
                // objects even when our own mkdir observed its existing entry.
                sync(&created)?;
                sync(&parent)?;
                Ok(created)
            };
            parent = step().map_err(|source| PrepareFailure {
                component: Some(index),
                source,
            })?;
        }
        Ok(parent)
    }
}

#[cfg(test)]
mod tests;
