//! Version-one sign-only exchange. Request data is never a credential channel.

use std::fs::File;
use std::io::{self, Read, Write};
use std::os::unix::fs::MetadataExt;
use std::path::Path;
use std::time::{Duration, SystemTime, UNIX_EPOCH};

use ic_host_artifacts::Length;
use ic_platform_types::Principal;
use serde::{Deserialize, Serialize};

use crate::Error;

/// Exact protocol identifier included in the approved configuration commitment.
pub const PROTOCOL: &str = "ic-keyring-sign/secp256k1/noninteractive/1";
/// Inclusive complete JSON request/response ceiling (32 MiB).
pub const FRAME_BYTES: usize = 32 * 1024 * 1024;
/// Inclusive complete signed-envelope ceiling (4 MiB).
pub const ENVELOPE_BYTES: usize = 4 * 1024 * 1024;
/// Inclusive public manifest ceiling (1 MiB).
pub const MANIFEST_BYTES: usize = 1024 * 1024;
/// Maximum signing-completion interval, including unlock and admission.
pub const SIGNING_INTERVAL: Duration = Duration::from_secs(300);

/// Public, explicitly selected existing ICP identity and local credential backend.
#[derive(Clone, Debug, Serialize, Deserialize, PartialEq, Eq)]
#[serde(deny_unknown_fields)]
pub struct Configuration {
    /// Existing ICP identity name; used in the `icp-cli` keyring service.
    pub identity: String,
    /// Existing public ICP `identity_list.json`, selected by the caller.
    pub manifest: String,
    /// Original approved canonical self-authenticating principal.
    pub sender: String,
    /// Linux Secret Service session bus; one local Unix address only. On macOS
    /// this must be absent and the native keychain is used.
    pub local_bus: Option<String>,
}

impl Configuration {
    /// Admit the explicit profile and its current public manifest record.
    /// No private keyring entry is accessed.
    /// # Errors
    /// Refuses invalid/changed identity selection or a nonlocal backend.
    pub fn admit(&self) -> Result<(), Error> {
        if self.identity.is_empty()
            || self.identity.len() > 64
            || !self
                .identity
                .bytes()
                .all(|b| b.is_ascii_alphanumeric() || b"_-".contains(&b))
            || self.manifest.len() > 1024
            || !Path::new(&self.manifest).is_absolute()
        {
            return Err(Error::Configuration);
        }
        let sender = principal(&self.sender)?;
        if sender.as_slice().len() != 29 || sender.as_slice().last() != Some(&2) {
            return Err(Error::Configuration);
        }
        #[cfg(target_os = "linux")]
        if !self.local_bus.as_deref().is_some_and(local_unix_bus) {
            return Err(Error::Configuration);
        }
        #[cfg(target_os = "macos")]
        if self.local_bus.is_some() {
            return Err(Error::Configuration);
        }
        let bytes = read_regular(Path::new(&self.manifest), MANIFEST_BYTES)?;
        let manifest: serde_json::Value =
            serde_json::from_slice(&bytes).map_err(|_| Error::Configuration)?;
        let selected = &manifest["identities"][&self.identity];
        if selected["kind"] != "keyring"
            || selected["algorithm"] != "secp256k1"
            || selected["principal"] != self.sender
        {
            return Err(Error::Configuration);
        }
        Ok(())
    }
}

/// The two supported exact operations. There is no submit or poll operation.
#[derive(Clone, Serialize, Deserialize)]
#[serde(tag = "kind", rename_all = "snake_case", deny_unknown_fields)]
pub enum Operation {
    /// Sign exact update content; preserve recipient versus effective routing.
    Update {
        /// Signed recipient, including the management canister.
        recipient: String,
        /// Exact update method.
        method: String,
        /// Original raw argument bytes.
        arg: Vec<u8>,
        /// Original nonce; absent and present-empty remain different.
        nonce: Option<Vec<u8>>,
    },
    /// Independently sign exactly one original `request_status` path.
    Status {
        /// Original update's representation-independent identity.
        request_id: [u8; 32],
    },
}

/// One already-approved signing request. No private key/password fields exist.
#[derive(Clone, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct Request {
    /// Original approved sender.
    pub sender: String,
    /// Effective routing/authority target, independent of update recipient.
    pub effective: String,
    /// Exact absolute expiry in Unix nanoseconds, never rounded or renewed.
    pub expires_ns: u64,
    /// Absolute signing-completion deadline in Unix nanoseconds.
    pub deadline_ns: u64,
    /// Inclusive complete signed-envelope ceiling.
    pub envelope_bytes: usize,
    /// Original network ceiling; the selected provider always performs zero
    /// external network requests and accepts only the explicit offline value 0.
    pub network_requests: u64,
    /// Exact update fields or one original request identity.
    pub operation: Operation,
}

impl Request {
    /// Check local limits/bindings and return the remaining completion interval.
    /// # Errors
    /// Refuses a changed sender, unsupported limits or expired/long deadline.
    pub fn admit(&self, config: &Configuration) -> Result<Duration, Error> {
        if self.sender != config.sender
            || self.network_requests != 0
            || !(1..=ENVELOPE_BYTES).contains(&self.envelope_bytes)
        {
            return Err(Error::Request);
        }
        principal(&self.sender)?;
        principal(&self.effective)?;
        if let Operation::Update {
            recipient,
            method,
            arg,
            nonce,
        } = &self.operation
        {
            principal(recipient)?;
            if method.is_empty()
                || method.len() > 1024
                || method.chars().any(char::is_control)
                || method
                    .len()
                    .checked_add(arg.len())
                    .and_then(|n| n.checked_add(nonce.as_ref().map_or(0, Vec::len)))
                    .is_none_or(|n| n > self.envelope_bytes)
            {
                return Err(Error::Request);
            }
        }
        let now = now_ns()?;
        let remaining = self
            .deadline_ns
            .checked_sub(now)
            .filter(|n| *n > 0)
            .ok_or(Error::Deadline)?;
        if self.expires_ns <= now || Duration::from_nanos(remaining) > SIGNING_INTERVAL {
            return Err(Error::Request);
        }
        Ok(Duration::from_nanos(remaining))
    }
}

/// Complete public request frame. Commitment is checked before credentials access.
#[derive(Clone, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct WireRequest {
    /// Exact supported protocol.
    pub protocol: String,
    /// Approved helper executable SHA-256.
    pub tool_sha256: [u8; 32],
    /// Original approved protocol/configuration/tool commitment.
    pub commitment: [u8; 32],
    /// Explicit existing identity/provider selection.
    pub configuration: Configuration,
    /// Exact approved operation.
    pub request: Request,
}

/// Complete provider success frame. No diagnostics or private material exist.
#[derive(Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct Response {
    /// Exact supported protocol.
    pub protocol: String,
    /// Original approved commitment.
    pub commitment: [u8; 32],
    /// Exact signed CBOR envelope, constructed and admitted by Platform.
    pub envelope: Vec<u8>,
}

/// Exact retained signed bytes and public request binding. This has no dispatcher.
pub struct Signed {
    pub(crate) envelope: Vec<u8>,
    pub(crate) request_id: [u8; 32],
    pub(crate) sender: Principal,
    pub(crate) effective: Principal,
    pub(crate) expiry: u64,
}

impl Signed {
    /// Original signed bytes, unchanged after provider completion/admission.
    #[must_use]
    pub fn envelope_bytes(&self) -> &[u8] {
        &self.envelope
    }
    /// Update identity, or the original update identity selected by a status read.
    #[must_use]
    pub const fn request_id(&self) -> &[u8; 32] {
        &self.request_id
    }
    /// Original sender.
    #[must_use]
    pub const fn sender(&self) -> Principal {
        self.sender
    }
    /// Original effective routing target.
    #[must_use]
    pub const fn effective_canister(&self) -> Principal {
        self.effective
    }
    /// Original exact signed expiry.
    #[must_use]
    pub const fn expiry(&self) -> u64 {
        self.expiry
    }
}

/// Commit to the exact protocol, credential configuration and selected executable.
/// # Errors
/// Refuses an unencodable/oversized public configuration.
pub fn commitment(config: &Configuration, tool: [u8; 32]) -> Result<[u8; 32], Error> {
    let bytes = encode(&(PROTOCOL, config, tool))?;
    Ok(ic_host_artifacts::copy(
        bytes.as_slice(),
        io::sink(),
        Length::AtMost(FRAME_BYTES as u64),
    )
    .map_err(|_| Error::Configuration)?
    .sha256)
}

/// Read one complete bounded frame, including an extra-byte refusal.
/// # Errors
/// Refuses oversized input and preserves reader errors. JSON syntax is admitted
/// separately after complete EOF; no partial frame is returned.
pub fn read_frame(reader: impl Read) -> Result<Vec<u8>, Error> {
    ic_host_artifacts::read(reader, Length::AtMost(FRAME_BYTES as u64))
        .map(|(bytes, _)| bytes)
        .map_err(|error| match error {
            ic_host_artifacts::Error::Io(cause) => Error::Io(cause),
            _ => Error::Request,
        })
}

/// Encode a complete request without producing oversized scratch.
/// # Errors
/// Refuses allocation, serialization or the complete-frame ceiling.
pub fn encode_request(request: &WireRequest) -> Result<Vec<u8>, Error> {
    encode(request)
}

/// Encode a complete response without producing oversized scratch.
/// # Errors
/// Refuses allocation, serialization or the complete-frame ceiling.
pub fn encode_response(response: &Response) -> Result<Vec<u8>, Error> {
    encode(response)
}

pub(crate) fn encode(value: &impl Serialize) -> Result<Vec<u8>, Error> {
    struct Counter(usize);
    impl Write for Counter {
        fn write(&mut self, bytes: &[u8]) -> io::Result<usize> {
            self.0 = self
                .0
                .checked_add(bytes.len())
                .filter(|n| *n <= FRAME_BYTES)
                .ok_or_else(|| io::Error::other("frame limit"))?;
            Ok(bytes.len())
        }
        fn flush(&mut self) -> io::Result<()> {
            Ok(())
        }
    }
    let mut count = Counter(0);
    serde_json::to_writer(&mut count, value).map_err(|_| Error::Request)?;
    let mut bytes = Vec::new();
    bytes
        .try_reserve_exact(count.0)
        .map_err(|_| Error::Request)?;
    serde_json::to_writer(&mut bytes, value).map_err(|_| Error::Request)?;
    Ok(bytes)
}

/// Hash an explicitly selected regular executable, refusing changed/insecure files.
/// This assumes cooperative path owners; it is not a hostile same-user sandbox.
/// # Errors
/// Refuses nonregular/symlink, writable-by-others, changed or oversized files.
pub fn tool_hash(path: &Path) -> Result<[u8; 32], Error> {
    let before = std::fs::symlink_metadata(path)?;
    if !path.is_absolute()
        || !before.is_file()
        || before.mode() & 0o022 != 0
        || before.mode() & 0o111 == 0
    {
        return Err(Error::Configuration);
    }
    let mut file = File::open(path)?;
    let opened = file.metadata()?;
    if before.dev() != opened.dev() || before.ino() != opened.ino() {
        return Err(Error::Configuration);
    }
    let identity =
        ic_host_artifacts::copy(&mut file, io::sink(), Length::AtMost(256 * 1024 * 1024))
            .map_err(|_| Error::Configuration)?;
    let after = std::fs::symlink_metadata(path)?;
    if before.dev() != after.dev()
        || before.ino() != after.ino()
        || before.len() != after.len()
        || before.modified()? != after.modified()?
        || before.ctime() != after.ctime()
        || before.ctime_nsec() != after.ctime_nsec()
    {
        return Err(Error::Configuration);
    }
    Ok(identity.sha256)
}

pub(crate) fn read_regular(path: &Path, maximum: usize) -> Result<Vec<u8>, Error> {
    let before = std::fs::symlink_metadata(path)?;
    if !before.is_file() {
        return Err(Error::Configuration);
    }
    let file = File::open(path)?;
    let opened = file.metadata()?;
    if before.dev() != opened.dev() || before.ino() != opened.ino() {
        return Err(Error::Configuration);
    }
    let bytes = ic_host_artifacts::read(file, Length::AtMost(maximum as u64))
        .map_err(|_| Error::Configuration)?
        .0;
    let after = std::fs::symlink_metadata(path)?;
    if before.dev() != after.dev()
        || before.ino() != after.ino()
        || before.len() != after.len()
        || before.modified()? != after.modified()?
        || before.ctime() != after.ctime()
        || before.ctime_nsec() != after.ctime_nsec()
    {
        return Err(Error::Configuration);
    }
    Ok(bytes)
}

pub(crate) fn principal(text: &str) -> Result<Principal, Error> {
    let value = Principal::from_text(text).map_err(|_| Error::Request)?;
    if value.to_text() != text {
        return Err(Error::Request);
    }
    Ok(value)
}

/// Current Unix nanoseconds, checked against protocol representation.
/// # Errors
/// Refuses a clock outside the supported absolute range.
pub fn now_ns() -> Result<u64, Error> {
    SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .map_err(|_| Error::Deadline)?
        .as_nanos()
        .try_into()
        .map_err(|_| Error::Deadline)
}

#[cfg(target_os = "linux")]
fn local_unix_bus(address: &str) -> bool {
    address.len() <= 1024
        && !address.contains(';')
        && !address.chars().any(char::is_control)
        && (address.starts_with("unix:path=/")
            || address
                .strip_prefix("unix:abstract=")
                .is_some_and(|s| !s.is_empty()))
}
