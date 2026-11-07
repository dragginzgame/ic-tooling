#![cfg(not(target_family = "wasm"))]
#![expect(
    missing_docs,
    clippy::too_many_arguments,
    clippy::pedantic,
    clippy::nursery,
    reason = "initial upstream source transfer preserves public signatures and protocol names; see upstream/NOTICE.md"
)]
// Copyright DFINITY Foundation. Licensed under Apache-2.0.
//! Maintained PocketIC 16.1 client implementation; see ../upstream/NOTICE.md.
//! The client is async-only and uses the caller's Tokio runtime.
//! Build with an explicit server URL and await operations. Explicitly await
//! `PocketIc::drop` before releasing a borrowed server; Rust Drop performs no I/O.
use self::common::rest::{
    AutoProgressConfig, IcpFeatures, InitialTime, InstanceConfig, InstanceHttpGatewayConfig,
    RawTime, SubnetId,
};
pub use ic_management_canister_types::{
    CanisterId, CanisterInstallMode, CanisterLogRecord, CanisterSettings, CanisterStatusResult,
    EnvironmentVariable, Snapshot,
};
pub use ic_platform::ic_protocol::SubnetMetrics;
use reqwest::Url;
use serde::{Deserialize, Serialize};
use std::{
    path::PathBuf,
    time::{Duration, SystemTime, UNIX_EPOCH},
};

mod error;
mod http;
pub use error::{CborErrorKind, ClientError, Operation, RequestContext};
pub mod common;
mod nonblocking;
pub use nonblocking::PocketIc;
pub mod snapshot;

/// Server version associated with the imported PocketIC 16.1 client.
pub const LATEST_SERVER_VERSION: &str = "16.1.0";

/// Select the supported instance configuration and an admitted server endpoint.
/// Persistent state paths remain caller-owned; this builder creates no directories.
pub struct PocketIcBuilder {
    config: InstanceConfig,
    server_url: Option<Url>,
    request_timeout: Duration,
}

impl PocketIcBuilder {
    /// Select a positive finite budget for each HTTP exchange, including polling.
    /// Build rejects zero or clock-overflowing budgets before sending requests.
    pub fn new(request_timeout: Duration) -> Self {
        Self {
            config: InstanceConfig {
                icp_features: Some(IcpFeatures::default()),
                ..InstanceConfig::default()
            },
            server_url: None,
            request_timeout,
        }
    }

    pub async fn build(self) -> Result<PocketIc, ClientError> {
        PocketIc::from_config(
            self.server_url.ok_or(ClientError::MissingServerUrl)?,
            self.request_timeout,
            self.config,
        )
        .await
    }

    /// Use an already running PocketIC server.
    pub fn with_server_url(mut self, server_url: Url) -> Self {
        self.server_url = Some(server_url);
        self
    }

    /// Select a caller-owned persistent instance directory accessible to the server.
    /// Instance deletion and client Drop never remove this directory.
    pub fn with_state_dir(mut self, state_dir: PathBuf) -> Self {
        self.config.state_dir = Some(state_dir);
        self
    }

    /// Add the unique NNS subnet.
    pub fn with_nns_subnet(mut self) -> Self {
        self.config.subnet_config_set.nns.get_or_insert_default();
        self
    }

    /// Add the unique Internet Identity subnet.
    pub fn with_ii_subnet(mut self) -> Self {
        self.config.subnet_config_set.ii.get_or_insert_default();
        self
    }

    pub fn with_system_subnet(mut self) -> Self {
        self.config
            .subnet_config_set
            .system
            .push(Default::default());
        self
    }

    pub fn with_application_subnet(mut self) -> Self {
        self.config
            .subnet_config_set
            .application
            .push(Default::default());
        self
    }

    /// Select system canisters provisioned and validated by the PocketIC server.
    pub fn with_icp_features(mut self, icp_features: IcpFeatures) -> Self {
        self.config.icp_features = Some(icp_features);
        self
    }

    /// Select the initial timestamp, subject to the server's system-canister minimum.
    pub fn with_initial_time(mut self, initial_time: Time) -> Self {
        self.config.initial_time = Some(InitialTime::Timestamp(RawTime {
            nanos_since_epoch: initial_time.as_nanos_since_unix_epoch(),
        }));
        self
    }

    /// Progress automatically from certified real time after instance creation.
    pub fn with_auto_progress(mut self) -> Self {
        self.config.initial_time = Some(InitialTime::AutoProgress(AutoProgressConfig {
            artificial_delay_ms: None,
        }));
        self
    }

    pub fn with_http_gateway(mut self, http_gateway_config: InstanceHttpGatewayConfig) -> Self {
        self.config.http_gateway_config = Some(http_gateway_config);
        self
    }
}

/// Representation of system time as duration since UNIX epoch
/// with cross-platform nanosecond precision.
#[derive(Copy, Clone, PartialEq, PartialOrd)]
pub struct Time(Duration);

impl Time {
    /// Number of nanoseconds since UNIX EPOCH.
    pub fn as_nanos_since_unix_epoch(&self) -> u64 {
        self.0.as_nanos().try_into().unwrap()
    }

    pub const fn from_nanos_since_unix_epoch(nanos: u64) -> Self {
        Time(Duration::from_nanos(nanos))
    }
}

impl std::fmt::Debug for Time {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let nanos_since_unix_epoch = self.as_nanos_since_unix_epoch();
        write!(f, "{nanos_since_unix_epoch}")
    }
}

impl std::ops::Add<Duration> for Time {
    type Output = Time;
    fn add(self, dur: Duration) -> Time {
        Time(self.0 + dur)
    }
}

impl From<SystemTime> for Time {
    fn from(time: SystemTime) -> Self {
        Self::from_nanos_since_unix_epoch(
            time.duration_since(UNIX_EPOCH)
                .unwrap()
                .as_nanos()
                .try_into()
                .unwrap(),
        )
    }
}

impl TryFrom<Time> for SystemTime {
    type Error = String;

    fn try_from(time: Time) -> Result<SystemTime, String> {
        let nanos = time.as_nanos_since_unix_epoch();
        let system_time = UNIX_EPOCH + Duration::from_nanos(nanos);
        let roundtrip: Time = system_time.into();
        if roundtrip.as_nanos_since_unix_epoch() == nanos {
            Ok(system_time)
        } else {
            Err(format!(
                "Converting UNIX timestamp {nanos} in nanoseconds to SystemTime failed due to losing precision"
            ))
        }
    }
}

/// Specifies where to place a newly created canister.
#[derive(Clone, Debug)]
pub enum CreateCanisterPlacement {
    /// Place the canister on the given subnet.
    SubnetId(SubnetId),
    /// Create the canister with the given specific canister ID.
    CanisterId(CanisterId),
}

/// Parameters for [`PocketIc::create_canister_with_params`].
#[derive(Clone, Debug, Default)]
pub struct CreateCanisterParams {
    /// Initial cycles balance; defaults to 100T if `None`.
    pub cycles: Option<u128>,
    /// Canister settings; defaults to default canister settings if `None`.
    pub settings: Option<CanisterSettings>,
    /// Canister placement (subnet or specific canister ID); a random application subnet is chosen if `None`.
    pub placement: Option<CreateCanisterPlacement>,
}

/// Error type for [`TryFrom<u64>`].
#[derive(Clone, Copy, Debug)]
pub enum TryFromError {
    ValueOutOfRange(u64),
}

/// User-facing error codes.
///
/// The error codes are currently assigned using an HTTP-like
/// convention: the most significant digit is the corresponding reject
/// code and the rest is just a sequentially assigned two-digit
/// number.
#[derive(PartialOrd, Ord, Clone, Copy, Debug, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[cfg_attr(test, derive(strum::EnumIter))]
pub enum ErrorCode {
    // 1xx -- `RejectCode::SysFatal`
    SubnetOversubscribed = 101,
    MaxNumberOfCanistersReached = 102,
    // 2xx -- `RejectCode::SysTransient`
    CanisterQueueFull = 201,
    IngressMessageTimeout = 202,
    CanisterQueueNotEmpty = 203,
    IngressHistoryFull = 204,
    CanisterIdAlreadyExists = 205,
    StopCanisterRequestTimeout = 206,
    CanisterOutOfCycles = 207,
    CertifiedStateUnavailable = 208,
    CanisterInstallCodeRateLimited = 209,
    CanisterHeapDeltaRateLimited = 210,
    SubnetCoolingDown = 211,
    // 3xx -- `RejectCode::DestinationInvalid`
    CanisterNotFound = 301,
    CanisterSnapshotNotFound = 305,
    // 4xx -- `RejectCode::CanisterReject`
    InsufficientMemoryAllocation = 402,
    InsufficientCyclesForCreateCanister = 403,
    SubnetNotFound = 404,
    CanisterNotHostedBySubnet = 405,
    CanisterRejectedMessage = 406,
    UnknownManagementMessage = 407,
    InvalidManagementPayload = 408,
    CanisterSnapshotImmutable = 409,
    InvalidSubnetAdmin = 410,
    // 5xx -- `RejectCode::CanisterError`
    CanisterTrapped = 502,
    CanisterCalledTrap = 503,
    CanisterContractViolation = 504,
    CanisterInvalidWasm = 505,
    CanisterDidNotReply = 506,
    CanisterOutOfMemory = 507,
    CanisterStopped = 508,
    CanisterStopping = 509,
    CanisterNotStopped = 510,
    CanisterStoppingCancelled = 511,
    CanisterInvalidController = 512,
    CanisterFunctionNotFound = 513,
    CanisterNonEmpty = 514,
    QueryCallGraphLoopDetected = 517,
    InsufficientCyclesInCall = 520,
    CanisterWasmEngineError = 521,
    CanisterInstructionLimitExceeded = 522,
    CanisterMemoryAccessLimitExceeded = 524,
    QueryCallGraphTooDeep = 525,
    QueryCallGraphTotalInstructionLimitExceeded = 526,
    CompositeQueryCalledInReplicatedMode = 527,
    QueryTimeLimitExceeded = 528,
    QueryCallGraphInternal = 529,
    InsufficientCyclesInComputeAllocation = 530,
    InsufficientCyclesInMemoryAllocation = 531,
    InsufficientCyclesInMemoryGrow = 532,
    ReservedCyclesLimitExceededInMemoryAllocation = 533,
    ReservedCyclesLimitExceededInMemoryGrow = 534,
    InsufficientCyclesInMessageMemoryGrow = 535,
    CanisterMethodNotFound = 536,
    CanisterWasmModuleNotFound = 537,
    CanisterAlreadyInstalled = 538,
    CanisterWasmMemoryLimitExceeded = 539,
    ReservedCyclesLimitIsTooLow = 540,
    CanisterInvalidControllerOrSubnetAdmin = 541,
    CanisterStatusAccessDenied = 542,
    // 6xx -- `RejectCode::SysUnknown`
    DeadlineExpired = 601,
    ResponseDropped = 602,
}

impl TryFrom<u64> for ErrorCode {
    type Error = TryFromError;
    fn try_from(err: u64) -> Result<ErrorCode, Self::Error> {
        match err {
            // 1xx -- `RejectCode::SysFatal`
            101 => Ok(ErrorCode::SubnetOversubscribed),
            102 => Ok(ErrorCode::MaxNumberOfCanistersReached),
            // 2xx -- `RejectCode::SysTransient`
            201 => Ok(ErrorCode::CanisterQueueFull),
            202 => Ok(ErrorCode::IngressMessageTimeout),
            203 => Ok(ErrorCode::CanisterQueueNotEmpty),
            204 => Ok(ErrorCode::IngressHistoryFull),
            205 => Ok(ErrorCode::CanisterIdAlreadyExists),
            206 => Ok(ErrorCode::StopCanisterRequestTimeout),
            207 => Ok(ErrorCode::CanisterOutOfCycles),
            208 => Ok(ErrorCode::CertifiedStateUnavailable),
            209 => Ok(ErrorCode::CanisterInstallCodeRateLimited),
            210 => Ok(ErrorCode::CanisterHeapDeltaRateLimited),
            211 => Ok(ErrorCode::SubnetCoolingDown),
            // 3xx -- `RejectCode::DestinationInvalid`
            301 => Ok(ErrorCode::CanisterNotFound),
            305 => Ok(ErrorCode::CanisterSnapshotNotFound),
            // 4xx -- `RejectCode::CanisterReject`
            402 => Ok(ErrorCode::InsufficientMemoryAllocation),
            403 => Ok(ErrorCode::InsufficientCyclesForCreateCanister),
            404 => Ok(ErrorCode::SubnetNotFound),
            405 => Ok(ErrorCode::CanisterNotHostedBySubnet),
            406 => Ok(ErrorCode::CanisterRejectedMessage),
            407 => Ok(ErrorCode::UnknownManagementMessage),
            408 => Ok(ErrorCode::InvalidManagementPayload),
            409 => Ok(ErrorCode::CanisterSnapshotImmutable),
            410 => Ok(ErrorCode::InvalidSubnetAdmin),
            // 5xx -- `RejectCode::CanisterError`
            502 => Ok(ErrorCode::CanisterTrapped),
            503 => Ok(ErrorCode::CanisterCalledTrap),
            504 => Ok(ErrorCode::CanisterContractViolation),
            505 => Ok(ErrorCode::CanisterInvalidWasm),
            506 => Ok(ErrorCode::CanisterDidNotReply),
            507 => Ok(ErrorCode::CanisterOutOfMemory),
            508 => Ok(ErrorCode::CanisterStopped),
            509 => Ok(ErrorCode::CanisterStopping),
            510 => Ok(ErrorCode::CanisterNotStopped),
            511 => Ok(ErrorCode::CanisterStoppingCancelled),
            512 => Ok(ErrorCode::CanisterInvalidController),
            513 => Ok(ErrorCode::CanisterFunctionNotFound),
            514 => Ok(ErrorCode::CanisterNonEmpty),
            517 => Ok(ErrorCode::QueryCallGraphLoopDetected),
            520 => Ok(ErrorCode::InsufficientCyclesInCall),
            521 => Ok(ErrorCode::CanisterWasmEngineError),
            522 => Ok(ErrorCode::CanisterInstructionLimitExceeded),
            524 => Ok(ErrorCode::CanisterMemoryAccessLimitExceeded),
            525 => Ok(ErrorCode::QueryCallGraphTooDeep),
            526 => Ok(ErrorCode::QueryCallGraphTotalInstructionLimitExceeded),
            527 => Ok(ErrorCode::CompositeQueryCalledInReplicatedMode),
            528 => Ok(ErrorCode::QueryTimeLimitExceeded),
            529 => Ok(ErrorCode::QueryCallGraphInternal),
            530 => Ok(ErrorCode::InsufficientCyclesInComputeAllocation),
            531 => Ok(ErrorCode::InsufficientCyclesInMemoryAllocation),
            532 => Ok(ErrorCode::InsufficientCyclesInMemoryGrow),
            533 => Ok(ErrorCode::ReservedCyclesLimitExceededInMemoryAllocation),
            534 => Ok(ErrorCode::ReservedCyclesLimitExceededInMemoryGrow),
            535 => Ok(ErrorCode::InsufficientCyclesInMessageMemoryGrow),
            536 => Ok(ErrorCode::CanisterMethodNotFound),
            537 => Ok(ErrorCode::CanisterWasmModuleNotFound),
            538 => Ok(ErrorCode::CanisterAlreadyInstalled),
            539 => Ok(ErrorCode::CanisterWasmMemoryLimitExceeded),
            540 => Ok(ErrorCode::ReservedCyclesLimitIsTooLow),
            541 => Ok(ErrorCode::CanisterInvalidControllerOrSubnetAdmin),
            542 => Ok(ErrorCode::CanisterStatusAccessDenied),
            // 6xx -- `RejectCode::SysUnknown`
            601 => Ok(ErrorCode::DeadlineExpired),
            602 => Ok(ErrorCode::ResponseDropped),
            _ => Err(TryFromError::ValueOutOfRange(err)),
        }
    }
}

impl std::fmt::Display for ErrorCode {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        // E.g. "IC0301"
        write!(f, "IC{:04}", *self as i32)
    }
}

/// User-facing reject codes.
///
/// They can be derived from the most significant digit of the
/// corresponding error code.
#[derive(PartialOrd, Ord, Clone, Copy, Debug, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[cfg_attr(test, derive(strum::EnumIter))]
pub enum RejectCode {
    SysFatal = 1,
    SysTransient = 2,
    DestinationInvalid = 3,
    CanisterReject = 4,
    CanisterError = 5,
    SysUnknown = 6,
}

impl TryFrom<u64> for RejectCode {
    type Error = TryFromError;
    fn try_from(err: u64) -> Result<RejectCode, Self::Error> {
        match err {
            1 => Ok(RejectCode::SysFatal),
            2 => Ok(RejectCode::SysTransient),
            3 => Ok(RejectCode::DestinationInvalid),
            4 => Ok(RejectCode::CanisterReject),
            5 => Ok(RejectCode::CanisterError),
            6 => Ok(RejectCode::SysUnknown),
            _ => Err(TryFromError::ValueOutOfRange(err)),
        }
    }
}

/// User-facing type describing an unsuccessful (also called reject) call response.
#[derive(Clone, Debug, Serialize, Deserialize, PartialEq, Eq)]
pub struct RejectResponse {
    pub reject_code: RejectCode,
    pub reject_message: String,
    pub error_code: ErrorCode,
    pub certified: bool,
}

impl std::fmt::Display for RejectResponse {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        // Follows [agent-rs](https://github.com/dfinity/agent-rs/blob/a651dbbe69e61d4e8508c144cd60cfa3118eeb3a/ic-agent/src/agent/agent_error.rs#L54)
        write!(
            f,
            "PocketIC returned a rejection error: reject code {:?}, reject message {}, error code {:?}",
            self.reject_code, self.reject_message, self.error_code
        )
    }
}

/// Converts an absolute Windows path with a drive prefix (e.g. `C:\Users\x\y`)
/// to the path under which WSL mounts it (e.g. `/mnt/c/Users/x/y`).
///
/// The drive letter is lower-cased and backslashes are replaced by forward slashes.
/// A verbatim prefix (`\\?\`), as produced by `std::fs::canonicalize` on Windows,
/// is stripped. Paths without a drive prefix (UNC paths, relative paths, ...) are
/// rejected.
#[cfg_attr(
    all(not(windows), not(test)),
    expect(
        dead_code,
        reason = "the retained WSL converter is used by Windows implementations"
    )
)]
fn windows_to_wsl(path: &str) -> Result<String, String> {
    let path = path.strip_prefix(r"\\?\").unwrap_or(path);
    let mut chars = path.chars();
    let drive = match (chars.next(), chars.next()) {
        (Some(drive), Some(':')) if drive.is_ascii_alphabetic() => drive.to_ascii_lowercase(),
        _ => {
            return Err(format!(
                "`{path}` does not start with a drive prefix such as `C:`"
            ));
        }
    };
    let rest = chars.as_str();
    if !rest.is_empty() && !rest.starts_with(['\\', '/']) {
        return Err(format!("`{path}` is a drive-relative path"));
    }
    Ok(format!("/mnt/{drive}{}", rest.replace('\\', "/")))
}

#[cfg(windows)]
fn wsl_path(path: &PathBuf, desc: &str) -> String {
    windows_to_wsl(
        path.as_os_str()
            .to_str()
            .unwrap_or_else(|| panic!("Could not convert {} path ({:?}) to String", desc, path)),
    )
    .unwrap_or_else(|e| {
        panic!(
            "Could not convert {} path ({:?}) to WSL path: {:?}",
            desc, path, e
        )
    })
}

#[cfg(test)]
mod tests;
