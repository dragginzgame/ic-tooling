//! Publication states have no publicly writable handles and never retry publication.
use std::ffi::{OsStr, OsString};
use std::fs::File;
use std::io::{self, Read};

use ic_host_artifacts::{Identity, Length};
use rustix::fs::{AtFlags, linkat, renameat};
use thiserror::Error;

use crate::directory::name;
use crate::{Access, Directory, Error, FileId, FilePolicy};

/// Destination effect selected before staging.
#[derive(Clone, Copy, Debug)]
pub enum Destination {
    /// Atomically refuse an occupied destination.
    CreateNew,
    /// Replace its final entry without following a symlink.
    Replace,
}
/// What happens to an owned staging entry when work is abandoned or fails.
#[derive(Clone, Copy, Debug)]
pub enum Retention {
    /// Remove only the still-original stage; Drop is fallback.
    Discard,
    /// Leave partial work for application-owned recovery.
    Retain,
}
/// Explicit publication authority and failure retention.
#[derive(Clone, Copy, Debug)]
pub struct Policy {
    /// Create-only or replace destination effect.
    pub destination: Destination,
    /// Permissions requested at stage creation.
    pub permissions: u32,
    /// Failure/abandonment retention policy.
    pub retention: Retention,
}
/// Successful visibility/content evidence; later mutations are not fenced.
#[derive(Debug)]
pub struct Receipt {
    /// Held parent identity at publication.
    pub directory: FileId,
    /// Selected final component.
    pub destination: OsString,
    /// Exact admitted content identity.
    pub content: Identity,
}
/// Phase whose completion is unconfirmed after visibility.
#[derive(Debug, Eq, PartialEq)]
pub enum Phase {
    /// Stage link removal refused after create-only publication.
    StageRemoval,
    /// Parent synchronization refused after publication.
    DirectorySync,
}
/// Write/seal failure preserves its original cause and any explicit cleanup refusal.
#[derive(Debug, Error)]
#[error("{cause}")]
pub struct AttemptFailure {
    /// Original admission, write or synchronization cause.
    #[source]
    pub cause: Error,
    /// Explicit cleanup cause, if any.
    pub cleanup: Option<Error>,
    /// Name requiring reconciliation, not proof it still names our file.
    pub recovery: Option<OsString>,
}
/// Publication outcome. NotPublished retains the sealed attempt for explicit abort
/// or a caller-directed retry; PublishedUnconfirmed never offers retry authority.
#[derive(Debug)]
pub enum Commit {
    /// Publication and required parent sync completed.
    Committed(Receipt),
    /// No publication by this attempt; original sealed ownership remains.
    NotPublished {
        /// Original refusal.
        cause: Error,
        /// Sealed attempt, still subject to its retention policy.
        stage: Sealed,
    },
    /// Destination became visible; do not infer safe retry or completed durability.
    PublishedUnconfirmed {
        /// Exact destination/content evidence.
        receipt: Receipt,
        /// First refused post-publication phase.
        phase: Phase,
        /// Original post-publication cause.
        cause: Error,
        /// Additional parent-sync refusal following a cleanup refusal.
        sync_error: Option<io::Error>,
    },
}

#[derive(Debug)]
struct Attempt {
    directory: Directory,
    file: File,
    staging: OsString,
    destination: OsString,
    policy: Policy,
    active: bool,
}
/// Newly created writable attempt. Writing consumes it, including on failure.
#[derive(Debug)]
#[must_use = "finish or explicitly abort the publication attempt"]
pub struct Stage(Attempt);
/// Complete admitted bytes, not yet synced or publishable.
#[derive(Debug)]
#[must_use = "finish or explicitly abort the publication attempt"]
pub struct Written {
    attempt: Attempt,
    content: Identity,
}
/// Synced complete bytes, with only commit or abort authority.
#[derive(Debug)]
#[must_use = "finish or explicitly abort the publication attempt"]
pub struct Sealed {
    attempt: Attempt,
    content: Identity,
}

impl Directory {
    /// Admit an original retained file for create-only publication without rewriting it.
    ///
    /// `original` must remain held from the caller's recovery inspection. `expected`
    /// is the complete content identity authorized by that recovery protocol, not
    /// merely a digest of whatever bytes happen to exist. The named entry must be
    /// the same regular file, have one hard link and no `forbidden` permission bits.
    /// Reading uses a fresh descriptor at offset zero and leaves `original`'s offset
    /// unchanged. Callers exclude namespace/content writers and writable aliases
    /// from admission through commit; this API cannot revoke external write access.
    ///
    /// The returned [`Written`] requires synchronization through [`Written::seal`].
    /// It always retains staging on failure or Drop and never replaces an occupied
    /// destination. Only an explicit abort or successful publication removes the
    /// original staging name. Admission refusal never removes or modifies evidence.
    /// # Errors
    /// Refuses invalid/equal names, unsafe entries, changed file/content identity,
    /// short/excess bytes and original I/O failures. The caller retains `original`.
    pub fn admit_retained(
        &self,
        staging: &OsStr,
        destination: &OsStr,
        original: &File,
        expected: Identity,
        forbidden: u32,
    ) -> Result<Written, Error> {
        name(staging)?;
        name(destination)?;
        if staging == destination {
            return Err(Error::Name);
        }
        let mut file = self
            .open_file(
                staging,
                Access::Read,
                FilePolicy {
                    forbidden,
                    single_link: true,
                },
            )?
            .ok_or(Error::Changed)?;
        if FileId::from_file(&file)? != FileId::from_file(original)? {
            return Err(Error::Changed);
        }
        let content =
            ic_host_artifacts::copy(&mut file, io::sink(), Length::Exactly(expected.bytes))?;
        if content != expected {
            return Err(Error::Changed);
        }
        self.check_file(staging, &file)?;
        Ok(Written {
            attempt: Attempt {
                directory: self.clone(),
                file,
                staging: staging.into(),
                destination: destination.into(),
                policy: Policy {
                    destination: Destination::CreateNew,
                    // No creation or permission changes occur for a retained file.
                    permissions: 0,
                    retention: Retention::Retain,
                },
                active: true,
            },
            content,
        })
    }

    /// Create a new named stage after admitting both names, then sync its parent.
    /// No parents are implicitly created. Stage and destination must differ.
    /// # Errors
    /// Preserves original creation/sync refusal, cleanup and recovery information.
    pub fn stage(
        &self,
        staging: &OsStr,
        destination: &OsStr,
        policy: Policy,
    ) -> Result<Stage, AttemptFailure> {
        let before = |cause| AttemptFailure {
            cause,
            cleanup: None,
            recovery: None,
        };
        name(staging).map_err(before)?;
        name(destination).map_err(before)?;
        if staging == destination {
            return Err(before(Error::Name));
        }
        let file = self
            .create_file(staging, policy.permissions)
            .map_err(before)?;
        let attempt = Attempt {
            directory: self.clone(),
            file,
            staging: staging.into(),
            destination: destination.into(),
            policy,
            active: true,
        };
        if let Err(error) = self.sync() {
            return Err(attempt.failed(error.into()));
        }
        Ok(Stage(attempt))
    }
}
impl Stage {
    /// Admit exactly one complete stream. Failure consumes the attempt and handles
    /// its selected retention; a partial write cannot later be sealed.
    /// # Errors
    /// Returns original byte/I/O failure plus any explicit cleanup refusal.
    pub fn write_from(
        mut self,
        reader: impl Read,
        length: Length,
    ) -> Result<Written, AttemptFailure> {
        match ic_host_artifacts::copy(reader, &mut self.0.file, length) {
            Ok(content) => Ok(Written {
                attempt: self.0,
                content,
            }),
            Err(error) => Err(self.0.failed(error.into())),
        }
    }
    /// Explicitly discard our original stage, even when retention was selected.
    /// # Errors
    /// Refuses observed replacement and preserves original cleanup I/O failures.
    pub fn abort(self) -> Result<(), Error> {
        self.0.abort()
    }
}
impl Written {
    /// Sync admitted bytes before granting publication authority.
    /// # Errors
    /// Preserves sync/identity refusal and explicit cleanup failure.
    pub fn seal(self) -> Result<Sealed, AttemptFailure> {
        self.seal_with(File::sync_all)
    }
    fn seal_with(
        self,
        sync: impl FnOnce(&File) -> io::Result<()>,
    ) -> Result<Sealed, AttemptFailure> {
        let result = self
            .attempt
            .check()
            .and_then(|()| sync(&self.attempt.file).map_err(Error::Io));
        match result {
            Ok(()) => Ok(Sealed {
                attempt: self.attempt,
                content: self.content,
            }),
            Err(error) => Err(self.attempt.failed(error)),
        }
    }
    /// Explicitly discard a complete but unpublished stage.
    /// # Errors
    /// Reports original identity or cleanup refusal.
    pub fn abort(self) -> Result<(), Error> {
        self.attempt.abort()
    }
}
impl Sealed {
    /// Publish once with the originally selected authority, then sync the parent.
    /// All failures are represented in the returned phase-specific outcome.
    #[must_use]
    pub fn commit(self) -> Commit {
        self.commit_with(Directory::sync, Attempt::remove)
    }
    fn commit_with(
        mut self,
        sync: impl FnOnce(&Directory) -> io::Result<()>,
        remove: impl FnOnce(&Attempt) -> Result<(), Error>,
    ) -> Commit {
        if let Err(cause) = self.attempt.check() {
            return Commit::NotPublished { cause, stage: self };
        }
        let directory = match self.attempt.directory.identity() {
            Ok(identity) => identity,
            Err(error) => {
                return Commit::NotPublished {
                    cause: error.into(),
                    stage: self,
                };
            }
        };
        let parent = &*self.attempt.directory.file;
        let result = match self.attempt.policy.destination {
            Destination::CreateNew => linkat(
                parent,
                &self.attempt.staging,
                parent,
                &self.attempt.destination,
                AtFlags::empty(),
            ),
            Destination::Replace => renameat(
                parent,
                &self.attempt.staging,
                parent,
                &self.attempt.destination,
            ),
        };
        if let Err(error) = result {
            return Commit::NotPublished {
                cause: error.into(),
                stage: self,
            };
        }
        // Visibility has occurred. Never retry publication or hide post-effect failures.
        self.attempt.active = false;
        let receipt = Receipt {
            directory,
            destination: self.attempt.destination.clone(),
            content: self.content,
        };
        let cleanup = if matches!(self.attempt.policy.destination, Destination::CreateNew) {
            remove(&self.attempt)
        } else {
            Ok(())
        };
        let synchronized = sync(&self.attempt.directory);
        match (cleanup, synchronized) {
            (Ok(()), Ok(())) => Commit::Committed(receipt),
            (Err(cause), result) => Commit::PublishedUnconfirmed {
                receipt,
                phase: Phase::StageRemoval,
                cause,
                sync_error: result.err(),
            },
            (Ok(()), Err(error)) => Commit::PublishedUnconfirmed {
                receipt,
                phase: Phase::DirectorySync,
                cause: Error::Io(error),
                sync_error: None,
            },
        }
    }
    /// Explicitly discard the original sealed stage without touching the destination.
    /// # Errors
    /// Reports identity or cleanup refusal. No publication is attempted.
    pub fn abort(self) -> Result<(), Error> {
        self.attempt.abort()
    }
}
impl Attempt {
    fn check(&self) -> Result<(), Error> {
        self.directory.check_file(&self.staging, &self.file)
    }
    fn remove(&self) -> Result<(), Error> {
        self.directory.remove_file(&self.staging, &self.file)
    }
    fn abort(mut self) -> Result<(), Error> {
        self.active = false;
        self.remove()?;
        self.directory.sync()?;
        Ok(())
    }
    fn failed(mut self, cause: Error) -> AttemptFailure {
        self.active = false;
        let cleanup = if matches!(self.policy.retention, Retention::Discard) {
            self.remove()
                .and_then(|()| self.directory.sync().map_err(Error::Io))
                .err()
        } else {
            None
        };
        let recovery = if cleanup.is_some() || matches!(self.policy.retention, Retention::Retain) {
            Some(self.staging.clone())
        } else {
            None
        };
        AttemptFailure {
            cause,
            cleanup,
            recovery,
        }
    }
}
impl Drop for Attempt {
    fn drop(&mut self) {
        if self.active && matches!(self.policy.retention, Retention::Discard) {
            let _ = self.remove();
        }
    }
}

#[cfg(test)]
mod tests;
