// Copyright DFINITY Foundation. Licensed under Apache-2.0.
//! The PocketIC server and the PocketIc library interface with HTTP/JSON.
//! This module retains the protocol DTOs used by the maintained client.
//! The server owns configuration validation and system-canister provisioning.

use crate::pocket_ic::RejectResponse;
use candid::Principal;
use hex;
use serde::{Deserialize, Serialize};
use std::collections::BTreeMap;
use std::path::PathBuf;

pub type InstanceId = usize;

#[derive(Debug, Clone, Eq, Hash, PartialEq, Serialize, Deserialize)]
pub struct AutoProgressConfig {
    pub artificial_delay_ms: Option<u64>,
}

#[derive(Debug, Clone, Eq, Hash, PartialEq, Serialize, Deserialize)]
pub enum HttpGatewayBackend {
    Replica(String),
    PocketIcInstance(InstanceId),
}

#[derive(Debug, Clone, Eq, Hash, PartialEq, Serialize, Deserialize)]
pub struct HttpsConfig {
    pub cert_path: String,
    pub key_path: String,
}

#[derive(Debug, Clone, Eq, Hash, PartialEq, Serialize, Deserialize)]
pub struct InstanceHttpGatewayConfig {
    pub ip_addr: Option<String>,
    pub port: Option<u16>,
    pub domains: Option<Vec<String>>,
    pub https_config: Option<HttpsConfig>,
    pub domain_custom_provider_local_file: Option<String>,
}

#[derive(Debug, Clone, Eq, Hash, PartialEq, Serialize, Deserialize)]
pub struct HttpGatewayConfig {
    pub ip_addr: Option<String>,
    pub port: Option<u16>,
    pub forward_to: HttpGatewayBackend,
    pub domains: Option<Vec<String>>,
    pub https_config: Option<HttpsConfig>,
    pub domain_custom_provider_local_file: Option<String>,
}

#[derive(Debug, Clone, Eq, Hash, PartialEq, Serialize, Deserialize)]
pub struct HttpGatewayInfo {
    pub instance_id: InstanceId,
    pub port: u16,
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub enum CreateHttpGatewayResponse {
    Created(HttpGatewayInfo),
    Error { message: String },
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub enum CreateInstanceResponse {
    Created {
        instance_id: InstanceId,
        topology: Topology,
        http_gateway_info: Option<HttpGatewayInfo>,
    },
    Error {
        message: String,
    },
}

#[derive(Debug, Clone, Copy, Eq, Hash, PartialEq, Serialize, Deserialize)]
pub struct RawTime {
    pub nanos_since_epoch: u64,
}

/// Relevant for calls to the management canister. If a subnet ID is
/// provided, the call will be sent to the management canister of that subnet.
/// If a canister ID is provided, the call will be sent to the management
/// canister of the subnet where the canister is on.
/// If None, the call will be sent to any management canister.
#[derive(Clone, Serialize, Deserialize, Debug, PartialEq, Eq, Hash)]
pub enum RawEffectivePrincipal {
    None,
    SubnetId(
        #[serde(deserialize_with = "base64::deserialize")]
        #[serde(serialize_with = "base64::serialize")]
        Vec<u8>,
    ),
    CanisterId(
        #[serde(deserialize_with = "base64::deserialize")]
        #[serde(serialize_with = "base64::serialize")]
        Vec<u8>,
    ),
}

impl std::fmt::Display for RawEffectivePrincipal {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            RawEffectivePrincipal::None => write!(f, "None"),
            RawEffectivePrincipal::SubnetId(subnet_id) => {
                let principal = Principal::from_slice(subnet_id);
                write!(f, "SubnetId({principal})")
            }
            RawEffectivePrincipal::CanisterId(canister_id) => {
                let principal = Principal::from_slice(canister_id);
                write!(f, "CanisterId({principal})")
            }
        }
    }
}

#[derive(Clone, Serialize, Deserialize, Debug)]
pub struct RawMessageId {
    pub effective_principal: RawEffectivePrincipal,
    #[serde(deserialize_with = "base64::deserialize")]
    #[serde(serialize_with = "base64::serialize")]
    pub message_id: Vec<u8>,
}

#[derive(Clone, Serialize, Deserialize, Debug)]
pub struct RawIngressStatusArgs {
    pub raw_message_id: RawMessageId,
    pub raw_caller: Option<RawPrincipalId>,
}

#[derive(Clone, Serialize, Deserialize, Debug)]
pub struct RawSenderInfo {
    #[serde(deserialize_with = "base64::deserialize")]
    #[serde(serialize_with = "base64::serialize")]
    pub info: Vec<u8>,
    #[serde(deserialize_with = "base64::deserialize")]
    #[serde(serialize_with = "base64::serialize")]
    pub signer: Vec<u8>,
}

#[derive(Clone, Serialize, Deserialize, Debug)]
pub struct RawCanisterCall {
    #[serde(deserialize_with = "base64::deserialize")]
    #[serde(serialize_with = "base64::serialize")]
    pub sender: Vec<u8>,
    #[serde(deserialize_with = "base64::deserialize")]
    #[serde(serialize_with = "base64::serialize")]
    pub canister_id: Vec<u8>,
    pub effective_principal: RawEffectivePrincipal,
    pub method: String,
    #[serde(deserialize_with = "base64::deserialize")]
    #[serde(serialize_with = "base64::serialize")]
    pub payload: Vec<u8>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sender_info: Option<RawSenderInfo>,
}

#[derive(Clone, Serialize, Deserialize, Debug)]
pub enum RawCanisterResult {
    Ok(
        #[serde(deserialize_with = "base64::deserialize")]
        #[serde(serialize_with = "base64::serialize")]
        Vec<u8>,
    ),
    Err(RejectResponse),
}

impl From<Result<Vec<u8>, RejectResponse>> for RawCanisterResult {
    fn from(result: Result<Vec<u8>, RejectResponse>) -> Self {
        match result {
            Ok(data) => RawCanisterResult::Ok(data),
            Err(reject_response) => RawCanisterResult::Err(reject_response),
        }
    }
}

impl From<RawCanisterResult> for Result<Vec<u8>, RejectResponse> {
    fn from(result: RawCanisterResult) -> Self {
        match result {
            RawCanisterResult::Ok(data) => Ok(data),
            RawCanisterResult::Err(reject_response) => Err(reject_response),
        }
    }
}

#[derive(Clone, Serialize, Deserialize, Debug)]
pub struct RawSetStableMemory {
    #[serde(deserialize_with = "base64::deserialize")]
    #[serde(serialize_with = "base64::serialize")]
    pub canister_id: Vec<u8>,
    pub blob_id: BlobId,
}

#[derive(Clone, Serialize, Deserialize, Debug)]
pub struct RawStableMemory {
    #[serde(deserialize_with = "base64::deserialize")]
    #[serde(serialize_with = "base64::serialize")]
    pub blob: Vec<u8>,
}

#[derive(Clone, Serialize, Deserialize, Debug)]
pub struct ApiError {
    pub(crate) message: String,
}

#[derive(Clone, Serialize, Deserialize, Debug)]
pub struct StartedOrBusyResponse {
    pub state_label: String,
    pub op_id: String,
}

#[derive(Clone, Serialize, Deserialize, Debug)]
pub struct RawAddCycles {
    #[serde(deserialize_with = "base64::deserialize")]
    #[serde(serialize_with = "base64::serialize")]
    pub canister_id: Vec<u8>,
    pub amount: u128,
}

#[derive(Clone, Serialize, Deserialize, Debug)]
pub struct RawCycles {
    pub cycles: u128,
}

#[derive(Clone, Serialize, Hash, Eq, PartialEq, Ord, PartialOrd, Deserialize, Debug)]
pub struct RawPrincipalId {
    // raw bytes of the principal
    #[serde(deserialize_with = "base64::deserialize")]
    #[serde(serialize_with = "base64::serialize")]
    pub principal_id: Vec<u8>,
}

impl From<Principal> for RawPrincipalId {
    fn from(principal: Principal) -> Self {
        Self {
            principal_id: principal.as_slice().to_vec(),
        }
    }
}

impl From<RawPrincipalId> for Principal {
    fn from(raw_principal_id: RawPrincipalId) -> Self {
        Principal::from_slice(&raw_principal_id.principal_id)
    }
}

#[derive(Clone, Serialize, Eq, PartialEq, Ord, PartialOrd, Deserialize, Debug)]
pub struct RawCanisterId {
    // raw bytes of the principal
    #[serde(deserialize_with = "base64::deserialize")]
    #[serde(serialize_with = "base64::serialize")]
    pub canister_id: Vec<u8>,
}

impl From<Principal> for RawCanisterId {
    fn from(principal: Principal) -> Self {
        Self {
            canister_id: principal.as_slice().to_vec(),
        }
    }
}

impl From<RawCanisterId> for Principal {
    fn from(raw_canister_id: RawCanisterId) -> Self {
        Principal::from_slice(&raw_canister_id.canister_id)
    }
}

#[derive(Clone, Serialize, Deserialize, Debug, PartialEq, Eq, Hash)]
pub struct RawSubnetId {
    #[serde(deserialize_with = "base64::deserialize")]
    #[serde(serialize_with = "base64::serialize")]
    pub subnet_id: Vec<u8>,
}

pub type SubnetId = Principal;

impl From<Principal> for RawSubnetId {
    fn from(principal: Principal) -> Self {
        Self {
            subnet_id: principal.as_slice().to_vec(),
        }
    }
}

impl From<RawSubnetId> for Principal {
    fn from(val: RawSubnetId) -> Self {
        Principal::from_slice(&val.subnet_id)
    }
}

#[derive(Clone, Debug, Eq, PartialEq, Hash, Serialize, Deserialize)]
pub struct BlobId(
    #[serde(deserialize_with = "base64::deserialize")]
    #[serde(serialize_with = "base64::serialize")]
    pub Vec<u8>,
);

impl std::fmt::Display for BlobId {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "BlobId{{{}}}", hex::encode(self.0.clone()))
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum BlobCompression {
    Gzip,
    NoCompression,
}

// By default, serde serializes Vec<u8> to a list of numbers, which is inefficient.
// This enables serializing Vec<u8> to a compact base64 representation.
pub mod base64 {
    use ::base64::prelude::*;
    use serde::{Deserialize, Serialize};
    use serde::{Deserializer, Serializer};

    pub fn serialize<S: Serializer>(v: &Vec<u8>, s: S) -> Result<S::Ok, S::Error> {
        let base64 = BASE64_STANDARD.encode(v);
        String::serialize(&base64, s)
    }

    pub fn deserialize<'de, D: Deserializer<'de>>(d: D) -> Result<Vec<u8>, D::Error> {
        let base64 = String::deserialize(d)?;
        BASE64_STANDARD
            .decode(base64.as_bytes())
            .map_err(serde::de::Error::custom)
    }
}

// ================================================================================================================= //

#[derive(Debug, Clone, Copy, Eq, Hash, PartialEq, Ord, PartialOrd, Serialize, Deserialize)]
pub enum SubnetKind {
    Application,
    Bitcoin,
    CloudEngine,
    Fiduciary,
    II,
    NNS,
    SNS,
    System,
    TestThresholdKeys,
    VerifiedApplication,
}

#[derive(Debug, Clone, Eq, Hash, PartialEq, Serialize, Deserialize, Default)]
pub enum IcpFeaturesConfig {
    /// Default configuration of an ICP feature resembling mainnet configuration as closely as possible.
    #[default]
    DefaultConfig,
}

/// Specifies ICP features enabled by deploying their corresponding system canisters
/// when creating a PocketIC instance and keeping them up to date
/// during the PocketIC instance lifetime.
/// The PocketIC server admits and provisions the required empty subnets.
/// An ICP feature is enabled if its `IcpFeaturesConfig` is provided, i.e.,
/// if the corresponding field is not `None`.
#[derive(Debug, Clone, Eq, Hash, PartialEq, Serialize, Deserialize, Default)]
pub struct IcpFeatures {
    /// Deploys the NNS registry canister and keeps its content in sync with registry used internally by PocketIC.
    /// Subnets: NNS.
    pub registry: Option<IcpFeaturesConfig>,
    /// Deploys the NNS cycles minting canister, sets ICP/XDR conversion rate, and keeps its subnet lists in sync with PocketIC topology.
    /// If the `cycles_minting` feature is enabled, then the default timestamp of a PocketIC instance is set to 10 May 2021 10:00:01 AM CEST (the smallest value that is strictly larger than the default timestamp hard-coded in the CMC state).
    /// Subnets: NNS.
    pub cycles_minting: Option<IcpFeaturesConfig>,
    /// Deploys the ICP ledger and index canisters and initializes the ICP account of the anonymous principal with 1,000,000,000 ICP.
    /// Subnets: NNS.
    pub icp_token: Option<IcpFeaturesConfig>,
    /// Deploys the cycles ledger and index canisters and initializes the cycles account of the anonymous principal with 2^127 cycles.
    /// Subnets: II.
    pub cycles_token: Option<IcpFeaturesConfig>,
    /// Deploys the Internet Identity canister.
    /// Subnets: II.
    pub ii: Option<IcpFeaturesConfig>,
}

#[derive(Debug, Clone, Eq, Hash, PartialEq, Serialize, Deserialize)]
pub enum InitialTime {
    /// Sets the initial timestamp of the new instance to the provided value which must be at least
    /// - 10 May 2021 10:00:01 AM CEST if the `cycles_minting` feature is enabled in `icp_features`;
    /// - 06 May 2021 21:17:10 CEST otherwise.
    Timestamp(RawTime),
    /// Configures the new instance to make progress automatically,
    /// i.e., periodically update the time of the IC instance
    /// to the real time and execute rounds on the subnets.
    /// Creating the instance only returns after the certified time
    /// of the IC instance has been updated for the first time.
    AutoProgress(AutoProgressConfig),
}

#[derive(Debug, Clone, Eq, Hash, PartialEq, Serialize, Deserialize, Default)]
pub(crate) struct InstanceConfig {
    pub subnet_config_set: ExtendedSubnetConfigSet,
    pub http_gateway_config: Option<InstanceHttpGatewayConfig>,
    pub state_dir: Option<PathBuf>,
    pub icp_features: Option<IcpFeatures>,
    pub initial_time: Option<InitialTime>,
}

#[derive(Debug, Clone, Eq, Hash, PartialEq, Serialize, Deserialize, Default)]
pub(crate) struct ExtendedSubnetConfigSet {
    pub nns: Option<SubnetSpec>,
    pub sns: Option<SubnetSpec>,
    pub ii: Option<SubnetSpec>,
    pub fiduciary: Option<SubnetSpec>,
    pub bitcoin: Option<SubnetSpec>,
    pub test_threshold_keys: Option<SubnetSpec>,
    pub system: Vec<SubnetSpec>,
    pub application: Vec<SubnetSpec>,
    #[serde(default)]
    pub cloud_engine: Vec<SubnetSpec>,
    pub verified_application: Vec<SubnetSpec>,
}

/// The client creates new subnets with the server's production defaults.
#[derive(Debug, Clone, Eq, Hash, PartialEq, Serialize, Deserialize)]
pub(crate) struct SubnetSpec {
    state_config: SubnetStateConfig,
    instruction_config: SubnetInstructionConfig,
    subnet_admins: Option<Vec<RawPrincipalId>>,
    #[serde(default)]
    cost_schedule: CanisterCyclesCostSchedule,
}

impl Default for SubnetSpec {
    fn default() -> Self {
        Self {
            state_config: SubnetStateConfig::New,
            instruction_config: SubnetInstructionConfig::Production,
            subnet_admins: None,
            cost_schedule: Default::default(),
        }
    }
}

/// Specifies instruction limits for canister execution on this subnet.
#[derive(Debug, Clone, Eq, Hash, PartialEq, Ord, PartialOrd, Serialize, Deserialize)]
pub enum SubnetInstructionConfig {
    /// Use default instruction limits as in production.
    Production,
    /// Use very high instruction limits useful for asymptotic canister benchmarking.
    Benchmarking,
}

/// Server representation for a fresh subnet.
#[derive(Debug, Clone, Eq, Hash, PartialEq, Serialize, Deserialize)]
enum SubnetStateConfig {
    /// Create new subnet with empty state.
    New,
}

/// Specifies how to charge canisters for their use of computational resources (such as
/// executing instructions, storing data, network, etc.).
#[derive(
    Debug, Default, Hash, Clone, Copy, Eq, PartialEq, Ord, PartialOrd, Serialize, Deserialize,
)]
pub enum CanisterCyclesCostSchedule {
    #[default]
    Normal,
    Free,
}

/// Configuration details for a subnet, returned by PocketIc server
#[derive(Debug, Clone, Eq, PartialEq, Ord, PartialOrd, Serialize, Deserialize)]
pub struct SubnetConfig {
    pub subnet_kind: SubnetKind,
    pub subnet_admins: Option<Vec<RawPrincipalId>>,
    pub cost_schedule: CanisterCyclesCostSchedule,
    pub subnet_seed: [u8; 32],
    /// Instruction limits for canister execution on this subnet.
    pub instruction_config: SubnetInstructionConfig,
    /// Some mainnet subnets have several disjunct canister ranges.
    pub canister_ranges: Vec<CanisterIdRange>,
}

#[derive(Debug, Clone, Eq, PartialEq, Ord, PartialOrd, Serialize, Deserialize)]
pub struct CanisterIdRange {
    pub start: RawCanisterId,
    pub end: RawCanisterId,
}

impl CanisterIdRange {
    fn contains(&self, canister_id: Principal) -> bool {
        Principal::from_slice(&self.start.canister_id) <= canister_id
            && canister_id <= Principal::from_slice(&self.end.canister_id)
    }
}

#[derive(Debug, Clone, Eq, PartialEq, Ord, PartialOrd, Serialize, Deserialize)]
pub struct Topology {
    pub subnet_configs: BTreeMap<SubnetId, SubnetConfig>,
    pub default_effective_canister_id: RawCanisterId,
}

impl Topology {
    pub fn get_subnet(&self, canister_id: Principal) -> Option<SubnetId> {
        self.subnet_configs
            .iter()
            .find(|(_, config)| {
                config
                    .canister_ranges
                    .iter()
                    .any(|r| r.contains(canister_id))
            })
            .map(|(subnet_id, _)| subnet_id)
            .copied()
    }

    pub fn get_app_subnets(&self) -> Vec<SubnetId> {
        self.subnet_configs
            .iter()
            .filter(|(_, config)| config.subnet_kind == SubnetKind::Application)
            .map(|(id, _)| *id)
            .collect()
    }

    pub fn get_ii(&self) -> Option<SubnetId> {
        self.find_subnet(SubnetKind::II)
    }

    pub fn get_nns(&self) -> Option<SubnetId> {
        self.find_subnet(SubnetKind::NNS)
    }

    fn find_subnet(&self, kind: SubnetKind) -> Option<SubnetId> {
        self.subnet_configs
            .iter()
            .find(|(_, config)| config.subnet_kind == kind)
            .map(|(id, _)| *id)
    }
}

#[derive(Clone, Serialize, Deserialize, Debug, Hash, Eq, PartialEq, Ord, PartialOrd)]
pub enum CanisterHttpMethod {
    GET,
    POST,
    HEAD,
    PUT,
    DELETE,
    PATCH,
}

#[derive(Clone, Serialize, Deserialize, Debug, Hash, Eq, PartialEq, Ord, PartialOrd)]
pub struct CanisterHttpHeader {
    pub name: String,
    pub value: String,
}

/// How a canister HTTP outcall is replicated across the nodes of its subnet.
#[derive(Clone, Serialize, Deserialize, Debug, Hash, Eq, PartialEq, Ord, PartialOrd)]
pub enum CanisterHttpReplication {
    /// Every node of the subnet performs the outcall and a response is delivered
    /// once `n - f` of them agree on it. Too many differing responses make the
    /// outcall fail with a "no consensus could be reached" rejection instead.
    /// Mock such an outcall with `PocketIc::mock_canister_http_response`, whose
    /// `additional_responses` are what lets a test produce either outcome.
    FullyReplicated,
    /// One node of the subnet performs the outcall (`is_replicated = false`) and
    /// its response is the one delivered. PocketIC does not report which node was
    /// picked, so mock such an outcall with
    /// `PocketIc::mock_canister_http_response` like a fully replicated one: give it
    /// a single `response`, which every node then reports, and it is delivered no
    /// matter which node was picked. Differing `additional_responses` would instead
    /// make the delivered response unpredictable.
    NonReplicated,
    /// A committee of `total_requests` nodes performs the outcall and between
    /// `min_responses` and `max_responses` of their (potentially differing)
    /// responses are delivered to the calling canister. Mock such an outcall
    /// by a caller with a flexible-outcall server adapter.
    Flexible {
        /// The number of nodes performing the outcall.
        total_requests: u32,
        /// The number of responses required to deliver a result.
        min_responses: u32,
        /// The largest number of responses that may be delivered.
        max_responses: u32,
    },
}

/// The pricing model applied to a canister HTTP outcall.
#[derive(Clone, Serialize, Deserialize, Debug, Hash, Eq, PartialEq, Ord, PartialOrd)]
pub enum CanisterHttpPricingVersion {
    /// The whole cost of the outcall is charged up front, based on the largest
    /// response it could receive.
    Legacy,
    /// The outcall is charged for the resources it actually consumes: a base fee is
    /// charged up front, a per-replica cycles allowance is withheld from the
    /// payment, and whatever the responding nodes do not spend is refunded.
    PayAsYouGo,
}

#[derive(Clone, Serialize, Deserialize, Debug)]
pub struct RawCanisterHttpRequest {
    pub subnet_id: RawSubnetId,
    pub request_id: u64,
    pub http_method: CanisterHttpMethod,
    pub url: String,
    pub headers: Vec<CanisterHttpHeader>,
    #[serde(deserialize_with = "base64::deserialize")]
    #[serde(serialize_with = "base64::serialize")]
    pub body: Vec<u8>,
    pub max_response_bytes: Option<u64>,
    pub replication: CanisterHttpReplication,
    pub pricing_version: CanisterHttpPricingVersion,
}

#[derive(Clone, Serialize, Deserialize, Debug, Hash, Eq, PartialEq, Ord, PartialOrd)]
pub struct CanisterHttpRequest {
    pub subnet_id: Principal,
    pub request_id: u64,
    pub http_method: CanisterHttpMethod,
    pub url: String,
    pub headers: Vec<CanisterHttpHeader>,
    #[serde(deserialize_with = "base64::deserialize")]
    #[serde(serialize_with = "base64::serialize")]
    pub body: Vec<u8>,
    pub max_response_bytes: Option<u64>,
    pub replication: CanisterHttpReplication,
    pub pricing_version: CanisterHttpPricingVersion,
}

impl From<RawCanisterHttpRequest> for CanisterHttpRequest {
    fn from(raw_canister_http_request: RawCanisterHttpRequest) -> Self {
        Self {
            subnet_id: candid::Principal::from_slice(
                &raw_canister_http_request.subnet_id.subnet_id,
            ),
            request_id: raw_canister_http_request.request_id,
            http_method: raw_canister_http_request.http_method,
            url: raw_canister_http_request.url,
            headers: raw_canister_http_request.headers,
            body: raw_canister_http_request.body,
            max_response_bytes: raw_canister_http_request.max_response_bytes,
            replication: raw_canister_http_request.replication,
            pricing_version: raw_canister_http_request.pricing_version,
        }
    }
}

impl From<CanisterHttpRequest> for RawCanisterHttpRequest {
    fn from(canister_http_request: CanisterHttpRequest) -> Self {
        Self {
            subnet_id: canister_http_request.subnet_id.into(),
            request_id: canister_http_request.request_id,
            http_method: canister_http_request.http_method,
            url: canister_http_request.url,
            headers: canister_http_request.headers,
            body: canister_http_request.body,
            max_response_bytes: canister_http_request.max_response_bytes,
            replication: canister_http_request.replication,
            pricing_version: canister_http_request.pricing_version,
        }
    }
}

#[derive(Clone, Serialize, Deserialize, Debug, Hash, Eq, PartialEq, Ord, PartialOrd)]
pub struct CanisterHttpReply {
    pub status: u16,
    pub headers: Vec<CanisterHttpHeader>,
    #[serde(deserialize_with = "base64::deserialize")]
    #[serde(serialize_with = "base64::serialize")]
    pub body: Vec<u8>,
}

#[derive(Clone, Serialize, Deserialize, Debug, Hash, Eq, PartialEq, Ord, PartialOrd)]
pub struct CanisterHttpReject {
    pub reject_code: u64,
    /// Bounded by the 1 KiB a node truncates its reject messages to.
    pub message: String,
}

#[derive(Clone, Serialize, Deserialize, Debug, Hash, Eq, PartialEq, Ord, PartialOrd)]
pub enum CanisterHttpResponse {
    CanisterHttpReply(CanisterHttpReply),
    CanisterHttpReject(CanisterHttpReject),
}

#[derive(Clone, Serialize, Deserialize, Debug)]
pub struct RawMockCanisterHttpResponse {
    pub subnet_id: RawSubnetId,
    pub request_id: u64,
    pub response: CanisterHttpResponse,
    pub additional_responses: Vec<CanisterHttpResponse>,
}

/// Mocked responses to a pending canister HTTP outcall made through the
/// `http_request` management canister endpoint — whether fully replicated or
/// non-replicated (see [`CanisterHttpReplication`]). A *flexible* outcall is
/// answered by a caller with a flexible-outcall server adapter instead.
///
/// This answers for every node of the subnet, which is what an `http_request`
/// outcall needs: a fully replicated one is performed by all of them, and a
/// non-replicated one by a single node that is not exposed. It is therefore also
/// the only shape that can make the nodes of a fully replicated outcall disagree
/// (see `additional_responses`).
///
/// An `http_request` outcall answered this way reports one of:
/// - the mocked response — as a reply, or as a rejection of the
///   calling canister's call carrying the mocked code and message;
/// - `SysTransient` and "No consensus could be reached" if its nodes disagree (see
///   `additional_responses`; a non-replicated outcall has no such outcome, it just
///   delivers its designated node's response);
/// - `SysTransient` and "Out of cycles" if what the nodes left unspent of their
///   per-replica cycles allowances no longer covers putting a response into a
///   block;
/// - `SysTransient` and "Canister http request timed out" once the outcall is
///   older than its 60 second timeout.
#[derive(Clone, Serialize, Deserialize, Debug, Hash, Eq, PartialEq, Ord, PartialOrd)]
pub struct MockCanisterHttpResponse {
    pub subnet_id: Principal,
    pub request_id: u64,
    /// The response every node of the subnet reports, unless
    /// `additional_responses` is non-empty — in which case this is the response of
    /// one node and `additional_responses` holds the rest.
    ///
    /// With this response alone, all nodes agree, so a fully replicated outcall
    /// delivers it (see [`CanisterHttpReplication::FullyReplicated`]).
    pub response: CanisterHttpResponse,
    /// The responses of the remaining nodes of the subnet. Either empty — then
    /// `response` is used for every node — or exactly one short of the subnet size,
    /// so that together with `response` there is one response per node.
    ///
    /// This is how a test makes the nodes of a *fully replicated* outcall disagree:
    /// unless `n - f` of the `n` responses are equal, it fails with a "no consensus
    /// could be reached" rejection.
    ///
    /// A *non-replicated* outcall has nothing to disagree about — only its
    /// designated node's response is delivered, and PocketIC does not report which
    /// node that is — so differing responses merely make the delivered one
    /// unpredictable. Leave this empty unless the disagreement is what is under
    /// test.
    pub additional_responses: Vec<CanisterHttpResponse>,
}

impl From<RawMockCanisterHttpResponse> for MockCanisterHttpResponse {
    fn from(raw_mock_canister_http_response: RawMockCanisterHttpResponse) -> Self {
        Self {
            subnet_id: candid::Principal::from_slice(
                &raw_mock_canister_http_response.subnet_id.subnet_id,
            ),
            request_id: raw_mock_canister_http_response.request_id,
            response: raw_mock_canister_http_response.response,
            additional_responses: raw_mock_canister_http_response.additional_responses,
        }
    }
}

impl From<MockCanisterHttpResponse> for RawMockCanisterHttpResponse {
    fn from(mock_canister_http_response: MockCanisterHttpResponse) -> Self {
        Self {
            subnet_id: RawSubnetId {
                subnet_id: mock_canister_http_response.subnet_id.as_slice().to_vec(),
            },
            request_id: mock_canister_http_response.request_id,
            response: mock_canister_http_response.response,
            additional_responses: mock_canister_http_response.additional_responses,
        }
    }
}
