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
    AutoProgressConfig, ExtendedSubnetConfigSet, IcpConfig, IcpFeatures, InitialTime,
    InstanceHttpGatewayConfig, RawSubnetBlockmakers, RawTickConfigs, RawTime, SubnetId, SubnetKind,
    SubnetSpec,
};
use candid::Principal;
pub use ic_management_canister_types::{
    CanisterId, CanisterInstallMode, CanisterLogRecord, CanisterSettings, CanisterStatusResult,
    EnvironmentVariable, Snapshot,
};
pub use ic_transport_types::SubnetMetrics;
use reqwest::Url;
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};
use slog::Level;
use std::{
    net::SocketAddr,
    path::PathBuf,
    time::{Duration, SystemTime, UNIX_EPOCH},
};
use strum_macros::EnumIter;
use tempfile::TempDir;

mod error;
mod http;
pub use error::{ClientError, Operation, RequestContext};
pub mod common;
mod nonblocking;
pub use nonblocking::{
    PocketIc, call_candid, call_candid_as, get_default_effective_canister_id, query_candid,
    query_candid_as, update_candid, update_candid_as, with_candid,
};
pub mod snapshot;

/// Server version associated with the imported PocketIC 16.1 client.
pub const LATEST_SERVER_VERSION: &str = "16.1.0";

// the default timeout of a PocketIC operation
const DEFAULT_MAX_REQUEST_TIME_MS: u64 = 300_000;

enum PocketIcStateKind {
    /// A persistent state dir managed by the user.
    StateDir(PathBuf),
    /// A fresh temporary directory used if the user does not provide
    /// a persistent state directory managed by the user.
    /// The temporary directory is deleted when `PocketIcState` is dropped
    /// unless `PocketIcState` is turned into a persistent state
    /// at the path given by `PocketIcState::into_path`.
    TempDir(TempDir),
}

pub struct PocketIcState {
    state: PocketIcStateKind,
}

impl PocketIcState {
    #[expect(
        clippy::new_without_default,
        reason = "preserve the imported PocketIC contract without an unrelated API change"
    )]
    pub fn new() -> Self {
        let temp_dir = TempDir::new().unwrap();
        Self {
            state: PocketIcStateKind::TempDir(temp_dir),
        }
    }

    pub fn new_from_path(state_dir: PathBuf) -> Self {
        Self {
            state: PocketIcStateKind::StateDir(state_dir),
        }
    }

    pub fn into_path(self) -> PathBuf {
        match self.state {
            PocketIcStateKind::StateDir(state_dir) => state_dir,
            PocketIcStateKind::TempDir(temp_dir) => temp_dir.keep(),
        }
    }

    pub(crate) fn state_dir(&self) -> PathBuf {
        match &self.state {
            PocketIcStateKind::StateDir(state_dir) => state_dir.clone(),
            PocketIcStateKind::TempDir(temp_dir) => temp_dir.path().to_path_buf(),
        }
    }
}

pub struct PocketIcBuilder {
    config: Option<ExtendedSubnetConfigSet>,
    http_gateway_config: Option<InstanceHttpGatewayConfig>,
    server_url: Option<Url>,
    max_request_time_ms: Option<u64>,
    read_only_state_dir: Option<PathBuf>,
    state_dir: Option<PocketIcState>,
    icp_config: IcpConfig,
    log_level: Option<Level>,
    bitcoind_addr: Option<Vec<SocketAddr>>,
    dogecoind_addr: Option<Vec<SocketAddr>>,
    icp_features: IcpFeatures,
    initial_time: Option<InitialTime>,
    mainnet_nns_subnet_id: Option<bool>,
    disable_ingress_validation: Option<bool>,
}

#[expect(
    clippy::new_without_default,
    reason = "preserve the imported PocketIC contract without an unrelated API change"
)]
impl PocketIcBuilder {
    pub fn new() -> Self {
        Self {
            config: None,
            http_gateway_config: None,
            server_url: None,
            max_request_time_ms: Some(DEFAULT_MAX_REQUEST_TIME_MS),
            read_only_state_dir: None,
            state_dir: None,
            icp_config: IcpConfig::default(),
            log_level: None,
            bitcoind_addr: None,
            dogecoind_addr: None,
            icp_features: IcpFeatures::default(),
            initial_time: None,
            mainnet_nns_subnet_id: None,
            disable_ingress_validation: None,
        }
    }

    pub fn new_with_config(config: impl Into<ExtendedSubnetConfigSet>) -> Self {
        let mut builder = Self::new();
        builder.config = Some(config.into());
        builder
    }

    pub async fn build(self) -> Result<PocketIc, ClientError> {
        PocketIc::from_components(
            self.config.unwrap_or_default(),
            self.server_url.ok_or(ClientError::MissingServerUrl)?,
            self.max_request_time_ms,
            self.read_only_state_dir,
            self.state_dir,
            self.icp_config,
            self.log_level,
            self.bitcoind_addr,
            self.dogecoind_addr,
            self.icp_features,
            self.initial_time,
            self.http_gateway_config,
            self.mainnet_nns_subnet_id,
            self.disable_ingress_validation,
        )
        .await
    }

    /// Use an already running PocketIC server.
    pub fn with_server_url(mut self, server_url: Url) -> Self {
        self.server_url = Some(server_url);
        self
    }

    pub fn with_max_request_time_ms(mut self, max_request_time_ms: Option<u64>) -> Self {
        self.max_request_time_ms = max_request_time_ms;
        self
    }

    pub fn with_state_dir(mut self, state_dir: PathBuf) -> Self {
        self.state_dir = Some(PocketIcState::new_from_path(state_dir));
        self
    }

    pub fn with_state(mut self, state_dir: PocketIcState) -> Self {
        self.state_dir = Some(state_dir);
        self
    }

    pub fn with_read_only_state(mut self, read_only_state_dir: &PocketIcState) -> Self {
        self.read_only_state_dir = Some(read_only_state_dir.state_dir());
        self
    }

    pub fn with_icp_config(mut self, icp_config: IcpConfig) -> Self {
        self.icp_config = icp_config;
        self
    }

    pub fn with_log_level(mut self, log_level: Level) -> Self {
        self.log_level = Some(log_level);
        self
    }

    pub fn with_bitcoind_addr(self, bitcoind_addr: SocketAddr) -> Self {
        self.with_bitcoind_addrs(vec![bitcoind_addr])
    }

    pub fn with_bitcoind_addrs(self, bitcoind_addrs: Vec<SocketAddr>) -> Self {
        Self {
            bitcoind_addr: Some(bitcoind_addrs),
            ..self
        }
    }

    pub fn with_dogecoind_addrs(self, dogecoind_addrs: Vec<SocketAddr>) -> Self {
        Self {
            dogecoind_addr: Some(dogecoind_addrs),
            ..self
        }
    }

    /// Add an empty NNS subnet unless an NNS subnet has already been added.
    pub fn with_nns_subnet(mut self) -> Self {
        let mut config = self.config.unwrap_or_default();
        config.nns = Some(config.nns.unwrap_or_default());
        self.config = Some(config);
        self
    }

    /// Add an NNS subnet with state loaded from the given state directory.
    /// Note that the provided path must be accessible for the PocketIC server process.
    ///
    /// `path_to_state` should lead to a directory which is expected to have
    /// the following structure:
    ///
    /// path_to_state/
    ///  |-- backups
    ///  |-- checkpoints
    ///  |-- diverged_checkpoints
    ///  |-- diverged_state_markers
    ///  |-- fs_tmp
    ///  |-- page_deltas
    ///  |-- states_metadata.pbuf
    ///  |-- tip
    ///  `-- tmp
    pub fn with_nns_state(self, path_to_state: PathBuf) -> Self {
        self.with_subnet_state(SubnetKind::NNS, path_to_state)
    }

    /// Add a subnet with state loaded from the given state directory.
    /// Note that the provided path must be accessible for the PocketIC server process.
    ///
    /// `path_to_state` should point to a directory which is expected to have
    /// the following structure:
    ///
    /// path_to_state/
    ///  |-- backups
    ///  |-- checkpoints
    ///  |-- diverged_checkpoints
    ///  |-- diverged_state_markers
    ///  |-- fs_tmp
    ///  |-- page_deltas
    ///  |-- states_metadata.pbuf
    ///  |-- tip
    ///  `-- tmp
    pub fn with_subnet_state(mut self, subnet_kind: SubnetKind, path_to_state: PathBuf) -> Self {
        let mut config = self.config.unwrap_or_default();
        #[cfg(not(windows))]
        let state_dir = path_to_state;
        #[cfg(windows)]
        let state_dir = wsl_path(&path_to_state, "subnet state").into();
        let subnet_spec = SubnetSpec::default().with_state_dir(state_dir);
        match subnet_kind {
            SubnetKind::NNS => config.nns = Some(subnet_spec),
            SubnetKind::SNS => config.sns = Some(subnet_spec),
            SubnetKind::II => config.ii = Some(subnet_spec),
            SubnetKind::Fiduciary => config.fiduciary = Some(subnet_spec),
            SubnetKind::Bitcoin => config.bitcoin = Some(subnet_spec),
            SubnetKind::TestThresholdKeys => config.test_threshold_keys = Some(subnet_spec),
            SubnetKind::Application => config.application.push(subnet_spec),
            SubnetKind::CloudEngine => config.cloud_engine.push(subnet_spec),
            SubnetKind::System => config.system.push(subnet_spec),
            SubnetKind::VerifiedApplication => config.verified_application.push(subnet_spec),
        };
        self.config = Some(config);
        self
    }

    /// Add an empty sns subnet unless an SNS subnet has already been added.
    pub fn with_sns_subnet(mut self) -> Self {
        let mut config = self.config.unwrap_or_default();
        config.sns = Some(config.sns.unwrap_or_default());
        self.config = Some(config);
        self
    }

    /// Add an empty II subnet unless an II subnet has already been added.
    pub fn with_ii_subnet(mut self) -> Self {
        let mut config = self.config.unwrap_or_default();
        config.ii = Some(config.ii.unwrap_or_default());
        self.config = Some(config);
        self
    }

    /// Add an empty fiduciary subnet unless a fiduciary subnet has already been added.
    pub fn with_fiduciary_subnet(mut self) -> Self {
        let mut config = self.config.unwrap_or_default();
        config.fiduciary = Some(config.fiduciary.unwrap_or_default());
        self.config = Some(config);
        self
    }

    /// Add an empty bitcoin subnet unless a bitcoin subnet has already been added.
    pub fn with_bitcoin_subnet(mut self) -> Self {
        let mut config = self.config.unwrap_or_default();
        config.bitcoin = Some(config.bitcoin.unwrap_or_default());
        self.config = Some(config);
        self
    }

    /// Add an empty test threshold keys subnet unless a test threshold keys subnet has already been added.
    pub fn with_test_threshold_keys_subnet(mut self) -> Self {
        let mut config = self.config.unwrap_or_default();
        config.test_threshold_keys = Some(config.test_threshold_keys.unwrap_or_default());
        self.config = Some(config);
        self
    }

    /// Add an empty generic system subnet.
    pub fn with_system_subnet(mut self) -> Self {
        let mut config = self.config.unwrap_or_default();
        config.system.push(SubnetSpec::default());
        self.config = Some(config);
        self
    }

    /// Add an empty generic application subnet.
    pub fn with_application_subnet(mut self) -> Self {
        let mut config = self.config.unwrap_or_default();
        config.application.push(SubnetSpec::default());
        self.config = Some(config);
        self
    }

    /// Add an empty generic verified application subnet.
    pub fn with_verified_application_subnet(mut self) -> Self {
        let mut config = self.config.unwrap_or_default();
        config.verified_application.push(SubnetSpec::default());
        self.config = Some(config);
        self
    }

    /// Add an empty generic application subnet with benchmarking instruction configuration.
    pub fn with_benchmarking_application_subnet(mut self) -> Self {
        let mut config = self.config.unwrap_or_default();
        config
            .application
            .push(SubnetSpec::default().with_benchmarking_instruction_config());
        self.config = Some(config);
        self
    }

    /// Add an empty generic system subnet with benchmarking instruction configuration.
    pub fn with_benchmarking_system_subnet(mut self) -> Self {
        let mut config = self.config.unwrap_or_default();
        config
            .system
            .push(SubnetSpec::default().with_benchmarking_instruction_config());
        self.config = Some(config);
        self
    }

    /// Enables selected ICP features supported by PocketIC and implemented by system canisters
    /// (deployed to the PocketIC instance automatically when creating a new PocketIC instance).
    /// Subnets to which the system canisters are deployed are automatically declared as empty subnets,
    /// e.g., `PocketIcBuilder::with_nns_subnet` is implicitly implied by specifying the `icp_token` feature.
    pub fn with_icp_features(mut self, icp_features: IcpFeatures) -> Self {
        self.icp_features = icp_features;
        self
    }

    /// Sets the initial timestamp of the new instance to the provided value which must be at least
    /// - 10 May 2021 10:00:01 AM CEST if the `cycles_minting` feature is enabled in `icp_features`;
    /// - 06 May 2021 21:17:10 CEST otherwise.
    #[deprecated(note = "Use `with_initial_time` instead")]
    pub fn with_initial_timestamp(mut self, initial_timestamp_nanos: u64) -> Self {
        self.initial_time = Some(InitialTime::Timestamp(RawTime {
            nanos_since_epoch: initial_timestamp_nanos,
        }));
        self
    }

    /// Sets the initial time of the new instance to the provided value which must be at least
    /// - 10 May 2021 10:00:01 AM CEST if the `cycles_minting` feature is enabled in `icp_features`;
    /// - 06 May 2021 21:17:10 CEST otherwise.
    pub fn with_initial_time(mut self, initial_time: Time) -> Self {
        self.initial_time = Some(InitialTime::Timestamp(RawTime {
            nanos_since_epoch: initial_time.as_nanos_since_unix_epoch(),
        }));
        self
    }

    /// Configures the new instance to make progress automatically,
    /// i.e., periodically update the time of the IC instance
    /// to the real time and execute rounds on the subnets.
    /// Building the instance only returns after the certified time
    /// of the IC instance has been updated for the first time.
    pub fn with_auto_progress(mut self) -> Self {
        let config = AutoProgressConfig {
            artificial_delay_ms: None,
        };
        self.initial_time = Some(InitialTime::AutoProgress(config));
        self
    }

    pub fn with_http_gateway(mut self, http_gateway_config: InstanceHttpGatewayConfig) -> Self {
        self.http_gateway_config = Some(http_gateway_config);
        self
    }

    pub fn with_mainnet_nns_subnet_id(mut self) -> Self {
        self.mainnet_nns_subnet_id = Some(true);
        self
    }

    pub fn disable_ingress_validation(mut self) -> Self {
        self.disable_ingress_validation = Some(true);
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
#[derive(
    PartialOrd,
    Ord,
    Clone,
    Copy,
    Debug,
    PartialEq,
    Eq,
    Hash,
    Serialize,
    Deserialize,
    JsonSchema,
    EnumIter,
)]
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
#[derive(
    PartialOrd,
    Ord,
    Clone,
    Copy,
    Debug,
    PartialEq,
    Eq,
    Hash,
    Serialize,
    Deserialize,
    JsonSchema,
    EnumIter,
)]
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
#[derive(Clone, Debug, Serialize, Deserialize, PartialEq, Eq, JsonSchema)]
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

/// This enum describes the result of retrieving ingress status.
/// The `IngressStatusResult::Forbidden` variant is produced
/// if an optional caller is provided and a corresponding read state request
/// for the status of the same update call signed by that specified caller
/// was rejected because the update call was submitted by a different caller.
#[derive(Debug, Serialize, Deserialize)]
pub enum IngressStatusResult {
    NotAvailable,
    Forbidden(String),
    Success(Result<Vec<u8>, RejectResponse>),
}

#[derive(Clone, Debug, Default)]
pub struct TickConfigs {
    pub blockmakers: Option<Vec<SubnetBlockmakers>>,
}

impl From<TickConfigs> for RawTickConfigs {
    fn from(tick_configs: TickConfigs) -> Self {
        Self {
            blockmakers: tick_configs.blockmakers.map(|blockmakers| {
                blockmakers
                    .into_iter()
                    .map(|blockmaker| blockmaker.into())
                    .collect()
            }),
        }
    }
}

#[derive(Clone, Debug)]
pub struct SubnetBlockmakers {
    pub subnet: Principal,
    pub blockmaker: Principal,
    pub failed_blockmakers: Vec<Principal>,
}

impl From<SubnetBlockmakers> for RawSubnetBlockmakers {
    fn from(blockmaker: SubnetBlockmakers) -> Self {
        Self {
            subnet: blockmaker.subnet.into(),
            blockmaker: blockmaker.blockmaker.into(),
            failed_blockmakers: blockmaker
                .failed_blockmakers
                .into_iter()
                .map(|p| p.into())
                .collect(),
        }
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

pub fn copy_dir(
    src: impl AsRef<std::path::Path>,
    dst: impl AsRef<std::path::Path>,
) -> std::io::Result<()> {
    std::fs::create_dir_all(&dst)?;
    for entry in std::fs::read_dir(src)? {
        let entry = entry?;
        let ty = entry.file_type()?;
        if ty.is_dir() {
            copy_dir(entry.path(), dst.as_ref().join(entry.file_name()))?;
        } else {
            std::fs::copy(entry.path(), dst.as_ref().join(entry.file_name()))?;
        }
    }
    Ok(())
}

#[cfg(test)]
mod tests;
