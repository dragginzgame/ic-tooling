#[cfg(test)]
mod tests;

use std::{
    fs::File,
    io::{self, Seek},
    os::unix::fs::PermissionsExt,
    path::{Path, PathBuf},
    process::Command,
    sync::atomic::AtomicBool,
    time::Duration,
};

use ic_host_artifacts::{Identity, Length};
use ic_host_fs::{Access, Commit, Destination, Directory, FileId, FilePolicy, Policy, Retention};

use super::{Error, capture, policy};

const ARCHIVE_LIMIT: u64 = 64 * 1024 * 1024;
const BINARY_LIMIT: usize = 256 * 1024 * 1024;
const BINARY: &str = "pocket-ic";

/// Authenticated selection for one supported native host, always PocketIC 16.1.0.
#[derive(Clone, Copy, Debug)]
pub struct Asset {
    /// Official explicit-download location; library use never fetches it.
    pub url: &'static str,
    /// SHA-256 reported by the official release for the compressed archive.
    pub archive_sha256: &'static str,
    /// SHA-256 of its independently decompressed executable.
    pub binary_sha256: &'static str,
}

/// Select the pinned release for this host; no environment override or discovery.
/// # Errors
/// Refuses unsupported native OS/architecture combinations.
pub fn host_asset() -> Result<Asset, Error> {
    match (std::env::consts::OS, std::env::consts::ARCH) {
        ("linux", "x86_64") => Ok(Asset {
            url: "https://github.com/dfinity/pocketic/releases/download/16.1.0/pocket-ic-x86_64-linux.gz",
            archive_sha256: "131219d90dcf9bf6f3ed8ee02d8f55504ea24a6db382f5680675b7bd6b7ed3bf",
            binary_sha256: "b44e1eccd66e02328146b3e209e3403e3a3428a25d25555aca2f67500e5ae9db",
        }),
        ("macos", "x86_64") => Ok(Asset {
            url: "https://github.com/dfinity/pocketic/releases/download/16.1.0/pocket-ic-x86_64-darwin.gz",
            archive_sha256: "af9ad2d781530a43556ef2d1c8f93db99a2425c6cabdc78520f61922399ed530",
            binary_sha256: "a2ad872a5d84778b25a254c4eb4a8df99917b5792edaa7702d730de2d7de664d",
        }),
        ("macos", "aarch64") => Ok(Asset {
            url: "https://github.com/dfinity/pocketic/releases/download/16.1.0/pocket-ic-arm64-darwin.gz",
            archive_sha256: "9ae843fbb7ae6c6eb30137671c8629a80ef53b3a3652eb85b344847ac39f9fcd",
            binary_sha256: "2ffd9d5ae103cbb85289424056e68920459e702bf27317e38003b019a0a959d1",
        }),
        _ => Err(Error::UnsupportedHost),
    }
}

/// Admitted offline executable and held filesystem identities. Keep its directory
/// and contents unchanged during use; this is not a fence against hostile writers.
#[derive(Debug)]
pub struct PreparedServer {
    path: PathBuf,
    directory: Directory,
    file: File,
    asset: Asset,
}
impl PreparedServer {
    /// Verify an existing prepared directory without downloading or executing.
    /// # Errors
    /// Refuses missing/unsafe entries, permission violations, excess bytes and hashes.
    pub fn check(directory: &Path) -> Result<Self, Error> {
        Self::check_for(directory, host_asset()?)
    }

    fn check_for(path: &Path, asset: Asset) -> Result<Self, Error> {
        let directory = Directory::open(path, 0o022)?.ok_or(Error::Asset("missing directory"))?;
        let canonical = path.canonicalize()?;
        let observed =
            Directory::open(&canonical, 0o022)?.ok_or(Error::Asset("missing directory"))?;
        if observed.identity()? != directory.identity()? {
            return Err(Error::Asset("directory changed"));
        }
        let mut file = binary(&directory)?;
        let identity =
            ic_host_artifacts::copy(&mut file, io::sink(), Length::AtMost(BINARY_LIMIT as u64))?;
        if hex(identity) != asset.binary_sha256 {
            return Err(Error::Asset("executable checksum"));
        }
        file.rewind()?;
        Ok(Self {
            path: canonical.join(BINARY),
            directory,
            file,
            asset,
        })
    }

    /// Exact admitted path. Reading this path grants no discovery or setup authority.
    #[must_use]
    pub fn path(&self) -> &Path {
        &self.path
    }

    pub(super) fn verify(&self) -> Result<(), Error> {
        let parent = self.path.parent().ok_or(Error::Asset("missing parent"))?;
        let checked = Self::check_for(parent, self.asset)?;
        if checked.directory.identity()? != self.directory.identity()?
            || FileId::from_file(&checked.file)? != FileId::from_file(&self.file)?
        {
            return Err(Error::Asset("prepared identity changed"));
        }
        Ok(())
    }
}

fn binary(directory: &Directory) -> Result<File, Error> {
    let file = directory
        .open_file(
            BINARY.as_ref(),
            Access::Read,
            FilePolicy {
                forbidden: 0o022,
                single_link: true,
            },
        )?
        .ok_or(Error::Asset("missing executable"))?;
    if file.metadata()?.permissions().mode() & 0o111 == 0 {
        return Err(Error::Asset("not executable"));
    }
    Ok(file)
}

/// Prepare an explicitly downloaded gzip archive into an existing directory.
/// `gzip` must name an explicitly prepared absolute executable. No network, tool
/// search or overwrite occurs. Reusing an existing valid preparation is allowed.
/// Decompression is limited to 60 seconds, 256 MiB stdout and 8 KiB stderr. The
/// complete admitted binary buffer is retained until create-only FS publication.
/// # Errors
/// Refuses unsupported hosts, archive/executable checksums, decompression status,
/// limits, unsafe files and publication/cleanup failures with original evidence.
pub fn prepare(archive: &Path, directory: &Path, gzip: &Path) -> Result<PreparedServer, Error> {
    let asset = host_asset()?;
    if !gzip.is_absolute() {
        return Err(Error::Configuration);
    }
    let destination =
        Directory::open(directory, 0o022)?.ok_or(Error::Asset("missing directory"))?;
    if destination
        .open_file(
            BINARY.as_ref(),
            Access::Read,
            FilePolicy {
                forbidden: 0o022,
                single_link: true,
            },
        )?
        .is_some()
    {
        return PreparedServer::check(directory);
    }
    let archive = archive.canonicalize()?;
    let parent = Directory::open(archive.parent().ok_or(Error::Configuration)?, 0)?
        .ok_or(Error::Asset("missing archive directory"))?;
    let input = parent
        .open_file(
            archive.file_name().ok_or(Error::Configuration)?,
            Access::Read,
            FilePolicy {
                forbidden: 0o022,
                single_link: true,
            },
        )?
        .ok_or(Error::Asset("missing archive"))?;
    let identity = ic_host_artifacts::copy(input, io::sink(), Length::AtMost(ARCHIVE_LIMIT))?;
    if hex(identity) != asset.archive_sha256 {
        return Err(Error::Asset("archive checksum"));
    }
    let mut command = Command::new(gzip);
    command.arg("-dc").arg("--").arg(&archive);
    let result = capture(
        &mut command,
        policy(Duration::from_secs(60), BINARY_LIMIT, 8192),
        &AtomicBool::new(false),
    );
    if result.process.failure.is_some()
        || !result.process.cleanup.is_empty()
        || !result.process.status.is_some_and(|status| status.success())
    {
        return Err(Error::Decompression(Box::new(result)));
    }
    let identity = ic_host_artifacts::copy(
        result.output.stdout.as_slice(),
        io::sink(),
        Length::AtMost(BINARY_LIMIT as u64),
    )?;
    if hex(identity) != asset.binary_sha256 {
        return Err(Error::Asset("decompressed checksum"));
    }
    // Stage names are allocated privately; destination remains create-only even
    // when another explicit preparation races this attempt.
    let name = format!(".pocket-ic-{}-{}", std::process::id(), next_stage());
    let sealed = destination
        .stage(
            name.as_ref(),
            BINARY.as_ref(),
            Policy {
                destination: Destination::CreateNew,
                permissions: 0o755,
                retention: Retention::Discard,
            },
        )?
        .write_from(
            result.output.stdout.as_slice(),
            Length::Exactly(identity.bytes),
        )?
        .seal()?;
    match sealed.commit() {
        Commit::Committed(_) => PreparedServer::check(directory),
        other => Err(Error::Publication(Box::new(other))),
    }
}

fn next_stage() -> usize {
    static NEXT: std::sync::atomic::AtomicUsize = std::sync::atomic::AtomicUsize::new(0);
    NEXT.fetch_add(1, std::sync::atomic::Ordering::Relaxed)
}

fn hex(identity: Identity) -> String {
    use std::fmt::Write;
    let mut text = String::with_capacity(64);
    for byte in identity.sha256 {
        let _ = write!(text, "{byte:02x}");
    }
    text
}
