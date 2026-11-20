#[cfg(test)]
mod tests;

use crate::{Directory, Error, FileId, directory::name};
use std::{
    ffi::{OsStr, OsString},
    io,
};

///
/// DirectoryPublication
/// Namespace publication evidence, not a statement about tree contents.
///

#[derive(Debug)]
pub struct DirectoryPublication {
    /// Held parent identity.
    pub parent: FileId,
    /// Published directory identity; retain its descriptor for later comparison.
    pub directory: FileId,
    /// Destination component under the held parent.
    pub destination: OsString,
}

///
/// DirectoryPublishError
/// Publication failure distinguished by whether this attempt renamed the tree.
///

#[derive(Debug, thiserror::Error)]
pub enum DirectoryPublishError {
    /// This attempt did not publish; no staging or foreign entry was removed.
    #[error("directory not published: {0}")]
    NotPublished(#[source] Error),
    /// Rename succeeded but parent synchronization failed. Reconcile before retry.
    #[error("directory visible but parent synchronization unconfirmed: {cause}")]
    PublishedUnconfirmed {
        /// Exact namespace transition observed before the synchronization failure.
        receipt: DirectoryPublication,
        /// Original parent synchronization cause.
        #[source]
        cause: io::Error,
    },
}

impl Directory {
    /// Publish an expected, caller-prepared child directory without replacement.
    /// The caller validates and synchronizes all tree members before calling,
    /// and excludes namespace/content writers. This syncs only the selected root
    /// before rename and the held parent afterward. No cleanup or recovery occurs.
    /// Concurrent publishers may compete for the destination; only one wins.
    /// # Errors
    /// Refuses invalid names, changed staging identity, occupied destinations and
    /// native errors. Parent-sync failure after rename carries visibility evidence.
    /// Unsupported no-replace platforms refuse without a replacement fallback.
    pub fn publish_directory(
        &self,
        staging: &OsStr,
        expected: &Self,
        destination: &OsStr,
    ) -> Result<DirectoryPublication, DirectoryPublishError> {
        self.publish_directory_with(staging, expected, destination, &mut Self::sync)
    }

    fn publish_directory_with(
        &self,
        staging: &OsStr,
        expected: &Self,
        destination: &OsStr,
        sync: &mut impl FnMut(&Self) -> io::Result<()>,
    ) -> Result<DirectoryPublication, DirectoryPublishError> {
        let prepare = || -> Result<DirectoryPublication, Error> {
            name(staging)?;
            name(destination)?;
            if staging == destination {
                return Err(Error::Name);
            }
            let receipt = DirectoryPublication {
                parent: self.identity()?,
                directory: expected.identity()?,
                destination: destination.to_owned(),
            };
            self.check_directory(staging, receipt.directory)?;

            Ok(receipt)
        };
        let receipt = prepare().map_err(DirectoryPublishError::NotPublished)?;
        sync(expected).map_err(|cause| DirectoryPublishError::NotPublished(cause.into()))?;
        self.check_directory(staging, receipt.directory)
            .and_then(|()| rename_new(self, staging, destination))
            .map_err(DirectoryPublishError::NotPublished)?;

        match sync(self) {
            Ok(()) => Ok(receipt),
            Err(cause) => Err(DirectoryPublishError::PublishedUnconfirmed { receipt, cause }),
        }
    }

    fn check_directory(&self, name: &OsStr, expected: FileId) -> Result<(), Error> {
        let named = self.open_directory(name, 0)?.ok_or(Error::Changed)?;
        if named.identity()? != expected {
            return Err(Error::Changed);
        }

        Ok(())
    }
}

fn rename_new(parent: &Directory, staging: &OsStr, destination: &OsStr) -> Result<(), Error> {
    #[cfg(any(target_os = "linux", target_os = "android", target_vendor = "apple"))]
    {
        rustix::fs::renameat_with(
            &*parent.file,
            staging,
            &*parent.file,
            destination,
            rustix::fs::RenameFlags::NOREPLACE,
        )?;

        Ok(())
    }

    #[cfg(not(any(target_os = "linux", target_os = "android", target_vendor = "apple")))]
    {
        let _ = (parent, staging, destination);

        Err(io::Error::from(io::ErrorKind::Unsupported).into())
    }
}
