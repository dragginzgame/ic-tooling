// Copyright DFINITY Foundation. Licensed under Apache-2.0.
use crate::pocket_ic::common::rest::{
    AutoProgressConfig, BlobCompression, BlobId, CanisterHttpRequest, CreateHttpGatewayResponse,
    CreateInstanceResponse, HttpGatewayBackend, HttpGatewayConfig, HttpGatewayInfo, HttpsConfig,
    InstanceConfig, InstanceId, MockCanisterHttpResponse, RawAddCycles, RawCanisterCall,
    RawCanisterHttpRequest, RawCanisterId, RawCanisterResult, RawCanisterSnapshotDownload,
    RawCanisterSnapshotId, RawCanisterSnapshotUpload, RawCycles, RawEffectivePrincipal,
    RawIngressStatusArgs, RawMessageId, RawMockCanisterHttpResponse, RawPrincipalId,
    RawSetStableMemory, RawStableMemory, RawSubnetId, RawTickConfigs, RawTime, SubnetId, Topology,
};
#[cfg(windows)]
use crate::pocket_ic::wsl_path;
use crate::pocket_ic::{ClientError, Operation, RequestContext, http::Exchange};
use crate::pocket_ic::{
    CreateCanisterParams, CreateCanisterPlacement, IngressStatusResult, PocketIcBuilder,
    RejectResponse, TickConfigs, Time,
};
use backon::{BackoffBuilder, ExponentialBuilder};
use candid::{
    Principal, decode_args, encode_args,
    utils::{ArgumentDecoder, ArgumentEncoder},
};
use ic_management_canister_types::{
    CanisterId, CanisterIdRecord, CanisterInstallMode, CanisterLogRecord, CanisterSettings,
    CanisterStatusResult, ChunkHash, DeleteCanisterSnapshotArgs, FetchCanisterLogsResult,
    InstallChunkedCodeArgs, InstallCodeArgs, LoadCanisterSnapshotArgs,
    ProvisionalCreateCanisterWithCyclesArgs, Snapshot, StoredChunksResult,
    TakeCanisterSnapshotArgs, UpdateSettingsArgs, UploadChunkArgs, UploadChunkResult,
};
use ic_platform::certification::cbor::decode_certificate;
use ic_platform::certification::{Certificate, Label, LookupResult};
use ic_platform::ic_protocol::Envelope;
use ic_platform::ic_protocol::EnvelopeContent::ReadState;
use ic_platform::ic_protocol::{
    ReadStateResponse, SubnetMetrics, decode_read_state_response, decode_subnet_metrics,
    encode_envelope,
};
use reqwest::{StatusCode, Url};
use serde::{Serialize, de::DeserializeOwned};
use sha2::{Digest, Sha256};
use std::future::Future;
use std::net::IpAddr;
use std::path::PathBuf;
use std::time::Duration;
use tracing::{debug, instrument};
use tracing_appender::non_blocking::WorkerGuard;
use tracing_subscriber::EnvFilter;

// wait time between polling requests

mod snapshot;

const LOG_DIR_PATH_ENV_NAME: &str = "POCKET_IC_LOG_DIR";
const LOG_DIR_LEVELS_ENV_NAME: &str = "POCKET_IC_LOG_DIR_LEVELS";

const LOCALHOST: &str = "localhost";

// The minimum joint size of a canister's WASM
// and its initial argument blob for which
// we install the canister WASM as a sequence of chunks.
// The size is chosen to be smaller than the maximum
// ingress message size of 2 MiB (2,097,152 B)
// on application subnet (which have the tighest
// ingress message size limit)
// to account for a few additional constant-size fields
// in the management canister payload to install code
// and the overhead of candid encoding.
const INSTALL_CHUNKED_CODE_THRESHOLD: usize = 2_000_000; // 2 MB

// The maximum size of one WASM chunk when installing a canister WASM
// as a sequence of chunks. This constant is specified
// in the IC protocol.
const INSTALL_CODE_CHUNK_SIZE: usize = 1 << 20; // 1 MiB

enum HttpMethod {
    Get,
    Post,
}

/// Main entry point for interacting with PocketIC.
pub struct PocketIc {
    /// The unique ID of this PocketIC instance.
    pub instance_id: InstanceId,
    // how long a get/post request may retry or poll
    max_request_time_ms: Option<u64>,
    http_gateway: Option<HttpGatewayInfo>,
    server_url: Url,
    reqwest_client: reqwest::Client,
    // the instance should only be deleted when dropping this handle if this handle owns the instance
    owns_instance: bool,
    _log_guard: Option<WorkerGuard>,
}

impl PocketIc {
    /// Creates a new PocketIC instance with a single application subnet on the server.
    /// The caller supplies an already admitted server endpoint.
    pub async fn new(server_url: Url) -> Result<Self, ClientError> {
        PocketIcBuilder::new()
            .with_server_url(server_url)
            .with_application_subnet()
            .build()
            .await
    }

    /// Creates a PocketIC handle to an existing instance on a running server.
    /// Note that this handle does not extend the lifetime of the existing instance,
    /// i.e., the existing instance is deleted and this handle stops working
    /// when the PocketIC handle that created the existing instance is dropped.
    pub fn new_from_existing_instance(
        server_url: Url,
        instance_id: InstanceId,
        max_request_time_ms: Option<u64>,
    ) -> Self {
        let test_driver_pid = std::process::id();
        let log_guard = setup_tracing(test_driver_pid);

        let reqwest_client = reqwest::Client::new();
        debug!("instance_id={} Reusing existing instance.", instance_id);

        Self {
            instance_id,
            max_request_time_ms,
            http_gateway: None,
            server_url,
            reqwest_client,
            owns_instance: false,
            _log_guard: log_guard,
        }
    }

    pub(crate) async fn from_config(
        server_url: Url,
        max_request_time_ms: Option<u64>,
        instance_config: InstanceConfig,
    ) -> Result<Self, ClientError> {
        #[cfg(windows)]
        let instance_config = InstanceConfig {
            state_dir: instance_config
                .state_dir
                .map(|path| wsl_path(&path, "state directory").into()),
            ..instance_config
        };

        let test_driver_pid = std::process::id();
        let log_guard = setup_tracing(test_driver_pid);

        let reqwest_client = reqwest::Client::new();
        let url = server_url.join("instances").unwrap();
        let mut exchange = Exchange::new(&url, max_request_time_ms);
        let response: CreateInstanceResponse = exchange
            .json(
                &reqwest_client,
                reqwest_client.post(url.clone()).json(&instance_config),
            )
            .await?;
        let (instance_id, http_gateway_info) = match response {
            CreateInstanceResponse::Created {
                instance_id,
                http_gateway_info,
                ..
            } => (instance_id, http_gateway_info),
            CreateInstanceResponse::Error { message } => {
                return Err(ClientError::Server {
                    context: Box::new(exchange.context.clone()),
                    status: exchange.context.status.unwrap_or(StatusCode::OK),
                    message,
                });
            }
        };
        debug!("instance_id={} New instance created.", instance_id);

        Ok(Self {
            instance_id,
            max_request_time_ms,
            http_gateway: http_gateway_info,
            server_url,
            reqwest_client,
            owns_instance: true,
            _log_guard: log_guard,
        })
    }

    /// Returns the URL of the PocketIC server on which this PocketIC instance is running.
    pub fn get_server_url(&self) -> Url {
        self.server_url.clone()
    }

    /// Returns the topology of the different subnets of this PocketIC instance.
    pub async fn topology(&self) -> Result<Topology, ClientError> {
        let endpoint = "read/topology";
        self.get(endpoint).await
    }

    /// Upload and store a binary blob to the PocketIC server.
    #[instrument(ret, skip(self, blob), fields(instance_id=self.instance_id, blob_len = %blob.len(), compression = ?compression))]
    pub async fn upload_blob(
        &self,
        blob: Vec<u8>,
        compression: BlobCompression,
    ) -> Result<BlobId, ClientError> {
        let url = self.server_url.join("blobstore/").unwrap();
        let mut request = self.reqwest_client.post(url.clone()).body(blob);
        if let BlobCompression::Gzip = compression {
            request = request.header(reqwest::header::CONTENT_ENCODING, "gzip");
        }
        let mut exchange = Exchange::new(&url, self.max_request_time_ms);
        let (status, bytes) = exchange.bytes(&self.reqwest_client, request).await?;
        if !status.is_success() {
            return Err(exchange.server_error(status, &bytes));
        }
        let hash = hex::decode(bytes).map_err(|error| ClientError::Protocol(error.to_string()))?;
        Ok(BlobId(hash))
    }

    /// Set stable memory of a canister. Optional GZIP compression can be used for reduced
    /// data traffic.
    #[instrument(skip(self, data), fields(instance_id=self.instance_id, canister_id = %canister_id.to_string(), data_len = %data.len(), compression = ?compression))]
    pub async fn set_stable_memory(
        &self,
        canister_id: CanisterId,
        data: Vec<u8>,
        compression: BlobCompression,
    ) -> Result<(), ClientError> {
        let blob_id = self.upload_blob(data, compression).await?;
        let endpoint = "update/set_stable_memory";
        self.post::<(), _>(
            endpoint,
            RawSetStableMemory {
                canister_id: canister_id.as_slice().to_vec(),
                blob_id,
            },
        )
        .await?;

        Ok(())
    }

    /// Get stable memory of a canister.
    #[instrument(skip(self), fields(instance_id=self.instance_id, canister_id = %canister_id.to_string()))]
    pub async fn get_stable_memory(&self, canister_id: CanisterId) -> Result<Vec<u8>, ClientError> {
        let endpoint = "read/get_stable_memory";
        let RawStableMemory { blob } = self
            .post(
                endpoint,
                RawCanisterId {
                    canister_id: canister_id.as_slice().to_vec(),
                },
            )
            .await?;
        Ok(blob)
    }

    /// List all instances and their status.
    #[instrument(ret)]
    pub async fn list_instances(server_url: Url) -> Result<Vec<String>, ClientError> {
        let url = server_url.join("instances").unwrap();
        let client = reqwest::Client::new();
        Exchange::new(&url, None)
            .json(&client, client.get(url.clone()))
            .await
    }

    /// Make the IC produce and progress by one block.
    /// Note that multiple ticks might be necessary to observe
    /// an expected effect, e.g., if the effect depends on
    /// inter-canister calls or heartbeats.
    #[instrument(skip(self), fields(instance_id=self.instance_id))]
    pub async fn tick(&self) -> Result<(), ClientError> {
        self.tick_with_configs(TickConfigs::default()).await?;

        Ok(())
    }

    /// Make the IC produce and progress by one block with custom
    /// configs for the round.
    #[instrument(skip(self), fields(instance_id=self.instance_id))]
    pub async fn tick_with_configs(&self, configs: TickConfigs) -> Result<(), ClientError> {
        let endpoint = "update/tick";
        self.post::<(), RawTickConfigs>(endpoint, configs.into())
            .await?;

        Ok(())
    }

    /// Configures the IC to make progress automatically,
    /// i.e., periodically update the time of the IC
    /// to the real time and execute rounds on the subnets.
    /// Only returns after the certified time of the IC
    /// has been updated for the first time.
    /// Returns the URL at which `/api` requests
    /// for this instance can be made.
    #[instrument(skip(self), fields(instance_id=self.instance_id))]
    pub async fn auto_progress(&self) -> Result<Url, ClientError> {
        let endpoint = "auto_progress";
        let auto_progress_config = AutoProgressConfig {
            artificial_delay_ms: None,
        };
        self.post::<(), _>(endpoint, auto_progress_config).await?;
        Ok(self.instance_url())
    }

    /// Returns whether automatic progress is enabled on the PocketIC instance.
    #[instrument(skip(self), fields(instance_id=self.instance_id))]
    pub async fn auto_progress_enabled(&self) -> Result<bool, ClientError> {
        self.get("auto_progress").await
    }

    pub(crate) fn instance_url(&self) -> Url {
        self.server_url
            .join("/instances/")
            .unwrap()
            .join(&format!("{}/", self.instance_id))
            .unwrap()
    }

    /// Stops automatic progress (see `auto_progress`) on the IC.
    #[instrument(skip(self), fields(instance_id=self.instance_id))]
    pub async fn stop_progress(&self) -> Result<(), ClientError> {
        let endpoint = "stop_progress";
        self.post::<(), _>(endpoint, "").await?;

        Ok(())
    }

    /// Returns the URL at which `/api` requests
    /// for this instance can be made if the HTTP
    /// gateway has been started.
    pub fn url(&self) -> Option<Url> {
        self.http_gateway
            .as_ref()
            .map(|res| Url::parse(&format!("http://{}:{}/", LOCALHOST, res.port)).unwrap())
    }

    /// Creates an HTTP gateway for this PocketIC instance binding to `127.0.0.1`
    /// and an optionally specified port (defaults to choosing an arbitrary unassigned port);
    /// listening on `localhost`;
    /// and configures the PocketIC instance to make progress automatically, i.e.,
    /// periodically update the time of the PocketIC instance to the real time
    /// and process messages on the PocketIC instance.
    /// Only returns after the certified time of the PocketIC instance
    /// has been updated for the first time.
    /// Returns the URL at which `/api` requests
    /// for this instance can be made.
    #[instrument(skip(self), fields(instance_id=self.instance_id))]
    pub async fn make_live(&mut self, listen_at: Option<u16>) -> Result<Url, ClientError> {
        self.make_live_with_params(None, listen_at, None, None)
            .await
    }

    /// Creates an HTTP gateway for this PocketIC instance binding
    /// to an optionally specified IP address (defaults to `127.0.0.1`)
    /// and port (defaults to choosing an arbitrary unassigned port);
    /// listening on optionally specified domains (default to `localhost`);
    /// and using an optionally specified TLS certificate (if provided, an HTTPS gateway is created)
    /// and configures the PocketIC instance to make progress automatically, i.e.,
    /// periodically update the time of the PocketIC instance to the real time
    /// and process messages on the PocketIC instance.
    /// Only returns after the certified time of the PocketIC instance
    /// has been updated for the first time.
    /// Returns the URL at which `/api` requests
    /// for this instance can be made.
    #[instrument(skip(self), fields(instance_id=self.instance_id))]
    pub async fn make_live_with_params(
        &mut self,
        ip_addr: Option<IpAddr>,
        listen_at: Option<u16>,
        domains: Option<Vec<String>>,
        https_config: Option<HttpsConfig>,
    ) -> Result<Url, ClientError> {
        if let Some(url) = self.url() {
            return Ok(url);
        }
        if !self.auto_progress_enabled().await? {
            self.auto_progress().await?;
        }
        self.start_http_gateway(
            ip_addr.map(|ip_addr| ip_addr.to_string()),
            listen_at,
            domains,
            https_config,
        )
        .await
    }

    async fn start_http_gateway(
        &mut self,
        ip_addr: Option<String>,
        port: Option<u16>,
        domains: Option<Vec<String>>,
        https_config: Option<HttpsConfig>,
    ) -> Result<Url, ClientError> {
        let endpoint = self.server_url.join("http_gateway").unwrap();
        let http_gateway_config = HttpGatewayConfig {
            ip_addr,
            port,
            forward_to: HttpGatewayBackend::PocketIcInstance(self.instance_id),
            domains: domains.clone(),
            https_config: https_config.clone(),
            domain_custom_provider_local_file: None,
        };
        let mut exchange = Exchange::new(&endpoint, self.max_request_time_ms);
        let res: CreateHttpGatewayResponse = exchange
            .json(
                &self.reqwest_client,
                self.reqwest_client
                    .post(endpoint.clone())
                    .json(&http_gateway_config),
            )
            .await?;
        Ok(match res {
            CreateHttpGatewayResponse::Created(info) => {
                let port = info.port;
                self.http_gateway = Some(info);
                let proto = if https_config.is_some() {
                    "https"
                } else {
                    "http"
                };
                Url::parse(&format!(
                    "{}://{}:{}/",
                    proto,
                    domains
                        .unwrap_or_default()
                        .into_iter()
                        .next()
                        .unwrap_or(LOCALHOST.to_string()),
                    port
                ))
                .unwrap()
            }
            CreateHttpGatewayResponse::Error { message } => {
                return Err(ClientError::Server {
                    context: Box::new(exchange.context.clone()),
                    status: exchange.context.status.unwrap_or(StatusCode::OK),
                    message,
                });
            }
        })
    }

    async fn stop_http_gateway(&mut self) -> Result<(), ClientError> {
        if let Some(info) = &self.http_gateway {
            let url = self
                .server_url
                .join(&format!("http_gateway/{}/stop", info.instance_id))
                .unwrap();
            Exchange::new(&url, self.max_request_time_ms)
                .json::<()>(&self.reqwest_client, self.reqwest_client.post(url.clone()))
                .await?;
            self.http_gateway = None;
        }
        Ok(())
    }

    /// Stops auto progress (automatic time updates and round executions)
    /// and the HTTP gateway for this IC instance.
    #[instrument(skip(self), fields(instance_id=self.instance_id))]
    pub async fn stop_live(&mut self) -> Result<(), ClientError> {
        self.stop_http_gateway().await?;
        self.stop_progress().await?;

        Ok(())
    }

    /// Get the root key of this IC instance. Returns `None` if the IC has no NNS subnet.
    #[instrument(skip(self), fields(instance_id=self.instance_id))]
    pub async fn root_key(&self) -> Result<Option<Vec<u8>>, ClientError> {
        let Some(subnet_id) = self.topology().await?.get_nns() else {
            return Ok(None);
        };
        let subnet_id: RawSubnetId = subnet_id.into();
        let endpoint = "read/pub_key";
        let res = self.post::<Vec<u8>, _>(endpoint, subnet_id).await?;
        Ok(Some(res))
    }

    /// Get the current time of the IC.
    #[instrument(ret, skip(self), fields(instance_id=self.instance_id))]
    pub async fn get_time(&self) -> Result<Time, ClientError> {
        let endpoint = "read/get_time";
        let result: RawTime = self.get(endpoint).await?;
        Ok(Time::from_nanos_since_unix_epoch(result.nanos_since_epoch))
    }

    /// Set the current time of the IC, on all subnets.
    #[instrument(skip(self), fields(instance_id=self.instance_id, time = ?time))]
    pub async fn set_time(&self, time: Time) -> Result<(), ClientError> {
        let endpoint = "update/set_time";
        self.post::<(), _>(
            endpoint,
            RawTime {
                nanos_since_epoch: time.as_nanos_since_unix_epoch(),
            },
        )
        .await?;

        Ok(())
    }

    /// Set the current certified time of the IC, on all subnets.
    #[instrument(skip(self), fields(instance_id=self.instance_id, time = ?time))]
    pub async fn set_certified_time(&self, time: Time) -> Result<(), ClientError> {
        let endpoint = "update/set_certified_time";
        self.post::<(), _>(
            endpoint,
            RawTime {
                nanos_since_epoch: time.as_nanos_since_unix_epoch(),
            },
        )
        .await?;

        Ok(())
    }

    /// Advance the time on the IC on all subnets by some nanoseconds.
    #[instrument(skip(self), fields(instance_id=self.instance_id, duration = ?duration))]
    pub async fn advance_time(&self, duration: Duration) -> Result<(), ClientError> {
        let now = self.get_time().await?;
        self.set_time(now + duration).await?;

        Ok(())
    }

    /// Get the controllers of a canister.
    /// Returns a server error if the canister does not exist.
    #[instrument(ret, skip(self), fields(instance_id=self.instance_id, canister_id = %canister_id.to_string()))]
    pub async fn get_controllers(
        &self,
        canister_id: CanisterId,
    ) -> Result<Vec<Principal>, ClientError> {
        let endpoint = "read/get_controllers";
        let result: Vec<RawPrincipalId> = self
            .post(
                endpoint,
                RawCanisterId {
                    canister_id: canister_id.as_slice().to_vec(),
                },
            )
            .await?;
        Ok(result.into_iter().map(|p| p.into()).collect())
    }

    /// Get the current cycles balance of a canister.
    #[instrument(ret, skip(self), fields(instance_id=self.instance_id, canister_id = %canister_id.to_string()))]
    pub async fn cycle_balance(&self, canister_id: CanisterId) -> Result<u128, ClientError> {
        let endpoint = "read/get_cycles";
        let result: RawCycles = self
            .post(
                endpoint,
                RawCanisterId {
                    canister_id: canister_id.as_slice().to_vec(),
                },
            )
            .await?;
        Ok(result.cycles)
    }

    /// Add cycles to a canister. Returns the new balance.
    #[instrument(ret, skip(self), fields(instance_id=self.instance_id, canister_id = %canister_id.to_string(), amount = %amount))]
    pub async fn add_cycles(
        &self,
        canister_id: CanisterId,
        amount: u128,
    ) -> Result<u128, ClientError> {
        let endpoint = "update/add_cycles";
        let result: RawCycles = self
            .post(
                endpoint,
                RawAddCycles {
                    canister_id: canister_id.as_slice().to_vec(),
                    amount,
                },
            )
            .await?;
        Ok(result.cycles)
    }

    /// Submit an update call (without executing it immediately).
    pub async fn submit_call(
        &self,
        canister_id: CanisterId,
        sender: Principal,
        method: &str,
        payload: Vec<u8>,
    ) -> Result<RawMessageId, ClientError> {
        self.submit_call_with_effective_principal(
            canister_id,
            RawEffectivePrincipal::CanisterId(canister_id.as_slice().to_vec()),
            sender,
            method,
            payload,
        )
        .await
    }

    /// Submit an update with explicit routing, retaining its accepted ingress receipt.
    pub async fn submit_call_with_effective_principal(
        &self,
        canister_id: CanisterId,
        effective_principal: RawEffectivePrincipal,
        sender: Principal,
        method: &str,
        payload: Vec<u8>,
    ) -> Result<RawMessageId, ClientError> {
        let endpoint = "update/submit_ingress_message";
        let raw_canister_call = RawCanisterCall {
            sender: sender.as_slice().to_vec(),
            canister_id: canister_id.as_slice().to_vec(),
            method: method.to_string(),
            payload,
            effective_principal,
            sender_info: None,
        };
        self.post::<Result<RawMessageId, RejectResponse>, _>(endpoint, raw_canister_call)
            .await?
            .map_err(ClientError::from)
    }

    /// Await an update call submitted previously by `submit_call` or `submit_call_with_effective_principal`.
    pub async fn await_call(&self, message_id: RawMessageId) -> Result<Vec<u8>, ClientError> {
        let endpoint = "update/await_ingress_message";
        let result: RawCanisterResult = self
            .post(endpoint, &message_id)
            .await
            .map_err(|error| error.with_ingress(message_id))?;
        Result::<Vec<u8>, RejectResponse>::from(result).map_err(ClientError::from)
    }

    /// Fetch the status of an update call submitted previously by `submit_call` or `submit_call_with_effective_principal`.
    /// Note that the status of the update call can only change if the PocketIC instance is in live mode
    /// or a round has been executed due to a separate PocketIC library call, e.g., `PocketIc::tick()`.
    pub async fn ingress_status(
        &self,
        raw_message_id: RawMessageId,
    ) -> Result<Option<Result<Vec<u8>, RejectResponse>>, ClientError> {
        let status = self.ingress_status_as_caller(raw_message_id, None).await?;
        Ok(match status {
            IngressStatusResult::NotAvailable => None,
            IngressStatusResult::Success(status) => Some(status),
            IngressStatusResult::Forbidden(err) => return Err(ClientError::Protocol(err)),
        })
    }

    /// Fetch the status of an update call submitted previously by `submit_call` or `submit_call_with_effective_principal`.
    /// Note that the status of the update call can only change if the PocketIC instance is in live mode
    /// or a round has been executed due to a separate PocketIC library call, e.g., `PocketIc::tick()`.
    /// If the status of the update call is known, but the update call was submitted by a different caller, then an error is returned.
    pub async fn ingress_status_as(
        &self,
        raw_message_id: RawMessageId,
        caller: Principal,
    ) -> Result<IngressStatusResult, ClientError> {
        self.ingress_status_as_caller(raw_message_id, Some(caller))
            .await
    }

    async fn ingress_status_as_caller(
        &self,
        raw_message_id: RawMessageId,
        caller: Option<Principal>,
    ) -> Result<IngressStatusResult, ClientError> {
        let args = RawIngressStatusArgs {
            raw_message_id: raw_message_id.clone(),
            raw_caller: caller.map(Into::into),
        };
        match self
            .post::<Option<RawCanisterResult>, _>("read/ingress_status", args)
            .await
        {
            Ok(None) => Ok(IngressStatusResult::NotAvailable),
            Ok(Some(result)) => Ok(IngressStatusResult::Success(result.into())),
            Err(ClientError::Server {
                status: StatusCode::FORBIDDEN,
                message,
                ..
            }) => Ok(IngressStatusResult::Forbidden(message)),
            Err(error) => Err(error.with_ingress(raw_message_id)),
        }
    }

    /// Await an update call submitted previously by `submit_call` or `submit_call_with_effective_principal`.
    /// Note that the status of the update call can only change if the PocketIC instance is in live mode
    /// or a round has been executed due to a separate PocketIC library call.
    pub async fn await_call_no_ticks(
        &self,
        message_id: RawMessageId,
    ) -> Result<Vec<u8>, ClientError> {
        let mut retry_policy = ExponentialBuilder::new()
            .with_min_delay(Duration::from_millis(10))
            .with_max_delay(Duration::from_secs(1))
            .with_factor(2.0)
            .with_jitter()
            .with_total_delay(Some(Duration::from_secs(15 * 60)))
            .without_max_times()
            .build();
        Ok(loop {
            if let Some(ingress_status) = self.ingress_status(message_id.clone()).await? {
                break ingress_status.map_err(ClientError::from)?;
            }
            let delay = retry_policy.next().ok_or_else(|| ClientError::Timeout {
                context: Box::new(RequestContext {
                    url: self.instance_url().to_string(),
                    operation: None,
                    ingress: Some(message_id.clone()),
                    may_have_executed: true,
                    status: None,
                }),
            })?;
            tokio::time::sleep(delay).await;
        })
    }

    /// Execute an update call on a canister.
    #[instrument(skip(self, payload), fields(instance_id=self.instance_id, canister_id = %canister_id.to_string(), sender = %sender.to_string(), method = %method, payload_len = %payload.len()))]
    pub async fn update_call(
        &self,
        canister_id: CanisterId,
        sender: Principal,
        method: &str,
        payload: Vec<u8>,
    ) -> Result<Vec<u8>, ClientError> {
        self.update_call_with_effective_principal(
            canister_id,
            RawEffectivePrincipal::CanisterId(canister_id.as_slice().to_vec()),
            sender,
            method,
            payload,
        )
        .await
    }

    /// Submit once with explicit routing, then await the accepted ingress.
    pub async fn update_call_with_effective_principal(
        &self,
        canister_id: CanisterId,
        effective_principal: RawEffectivePrincipal,
        sender: Principal,
        method: &str,
        payload: Vec<u8>,
    ) -> Result<Vec<u8>, ClientError> {
        let message_id = self
            .submit_call_with_effective_principal(
                canister_id,
                effective_principal,
                sender,
                method,
                payload,
            )
            .await?;
        self.await_call(message_id).await
    }

    /// Execute a query call on a canister.
    #[instrument(skip(self, payload), fields(instance_id=self.instance_id, canister_id = %canister_id.to_string(), sender = %sender.to_string(), method = %method, payload_len = %payload.len()))]
    pub async fn query_call(
        &self,
        canister_id: CanisterId,
        sender: Principal,
        method: &str,
        payload: Vec<u8>,
    ) -> Result<Vec<u8>, ClientError> {
        self.query_call_with_effective_principal(
            canister_id,
            RawEffectivePrincipal::CanisterId(canister_id.as_slice().to_vec()),
            sender,
            method,
            payload,
        )
        .await
    }

    /// Execute a query call on a canister explicitly specifying an effective principal to route the request:
    /// this API is useful for making generic query calls (including management canister query calls) without using dedicated functions from this library
    /// (e.g., making generic query calls in dfx to a PocketIC instance).
    #[instrument(skip(self, payload), fields(instance_id=self.instance_id, canister_id = %canister_id.to_string(), effective_principal = %effective_principal.to_string(), sender = %sender.to_string(), method = %method, payload_len = %payload.len()))]
    pub async fn query_call_with_effective_principal(
        &self,
        canister_id: CanisterId,
        effective_principal: RawEffectivePrincipal,
        sender: Principal,
        method: &str,
        payload: Vec<u8>,
    ) -> Result<Vec<u8>, ClientError> {
        let endpoint = "read/query";
        let raw_canister_call = RawCanisterCall {
            sender: sender.as_slice().to_vec(),
            canister_id: canister_id.as_slice().to_vec(),
            method: method.to_string(),
            payload,
            effective_principal,
            sender_info: None,
        };
        let result: RawCanisterResult = self.post(endpoint, raw_canister_call).await?;
        Result::<Vec<u8>, RejectResponse>::from(result).map_err(ClientError::from)
    }

    /// Fetch canister logs via a query call to the management canister.
    pub async fn fetch_canister_logs(
        &self,
        canister_id: CanisterId,
        sender: Principal,
    ) -> Result<Vec<CanisterLogRecord>, ClientError> {
        Ok(with_candid::<_, (FetchCanisterLogsResult,), _>(
            (CanisterIdRecord { canister_id },),
            |payload| async {
                self.query_call_with_effective_principal(
                    Principal::management_canister(),
                    RawEffectivePrincipal::CanisterId(canister_id.as_slice().to_vec()),
                    sender,
                    "fetch_canister_logs",
                    payload,
                )
                .await
            },
        )
        .await?
        .0
        .canister_log_records)
    }

    /// Request a canister's status.
    #[instrument(skip(self), fields(instance_id=self.instance_id, sender = %sender.unwrap_or(Principal::anonymous()).to_string()))]
    pub async fn canister_status(
        &self,
        canister_id: CanisterId,
        sender: Option<Principal>,
    ) -> Result<CanisterStatusResult, ClientError> {
        Ok(
            call_candid_as::<(CanisterIdRecord,), (CanisterStatusResult,)>(
                self,
                Principal::management_canister(),
                RawEffectivePrincipal::CanisterId(canister_id.as_slice().to_vec()),
                sender.unwrap_or(Principal::anonymous()),
                "canister_status",
                (CanisterIdRecord { canister_id },),
            )
            .await?
            .0,
        )
    }

    /// Create a canister with default settings as the anonymous principal.
    /// The canister is created with 100T cycles.
    #[instrument(ret, skip(self), fields(instance_id=self.instance_id))]
    pub async fn create_canister(&self) -> Result<CanisterId, ClientError> {
        self.create_canister_with_params(None, CreateCanisterParams::default())
            .await
    }

    /// Create a canister with optional custom settings and a sender.
    /// The canister is created with 100T cycles.
    #[instrument(ret, skip(self), fields(instance_id=self.instance_id, settings = ?settings, sender = %sender.unwrap_or(Principal::anonymous()).to_string()))]
    pub async fn create_canister_with_settings(
        &self,
        sender: Option<Principal>,
        settings: Option<CanisterSettings>,
    ) -> Result<CanisterId, ClientError> {
        self.create_canister_with_params(
            sender,
            CreateCanisterParams {
                settings,
                ..Default::default()
            },
        )
        .await
    }

    /// Creates a canister with a specific canister ID and optional custom settings.
    /// The canister is created with 100T cycles.
    /// Returns an error if the canister ID is already in use.
    /// Creates a new subnet if the canister ID is not contained in any of the subnets.
    ///
    /// The canister ID must be an IC mainnet canister ID that does not belong to the NNS or II subnet,
    /// otherwise the function might panic (for NNS and II canister IDs,
    /// the PocketIC instance should already be created with those subnets).
    #[instrument(ret, skip(self), fields(instance_id=self.instance_id, sender = %sender.unwrap_or(Principal::anonymous()).to_string(), settings = ?settings, canister_id = %canister_id.to_string()))]
    pub async fn create_canister_with_id(
        &self,
        sender: Option<Principal>,
        settings: Option<CanisterSettings>,
        canister_id: CanisterId,
    ) -> Result<CanisterId, ClientError> {
        self.create_canister_with_params(
            sender,
            CreateCanisterParams {
                settings,
                placement: Some(CreateCanisterPlacement::CanisterId(canister_id)),
                ..Default::default()
            },
        )
        .await
    }

    /// Create a canister on a specific subnet with optional custom settings.
    /// The canister is created with 100T cycles.
    #[instrument(ret, skip(self), fields(instance_id=self.instance_id, sender = %sender.unwrap_or(Principal::anonymous()).to_string(), settings = ?settings, subnet_id = %subnet_id.to_string()))]
    pub async fn create_canister_on_subnet(
        &self,
        sender: Option<Principal>,
        settings: Option<CanisterSettings>,
        subnet_id: SubnetId,
    ) -> Result<CanisterId, ClientError> {
        self.create_canister_with_params(
            sender,
            CreateCanisterParams {
                settings,
                placement: Some(CreateCanisterPlacement::SubnetId(subnet_id)),
                ..Default::default()
            },
        )
        .await
    }

    /// Create a canister with optional cycles, settings, and placement.
    /// The placement specifies either a target subnet or a specific canister ID.
    /// Defaults to 100T cycles if `params.cycles` is `None`.
    /// Returns an error if the specified canister ID is already in use.
    #[instrument(ret, skip(self), fields(instance_id=self.instance_id, sender = %sender.unwrap_or(Principal::anonymous()).to_string()))]
    pub async fn create_canister_with_params(
        &self,
        sender: Option<Principal>,
        params: CreateCanisterParams,
    ) -> Result<CanisterId, ClientError> {
        let cycles = params.cycles.unwrap_or(100_000_000_000_000);
        let (effective_principal, specified_id) = match params.placement {
            None => (RawEffectivePrincipal::None, None),
            Some(CreateCanisterPlacement::SubnetId(subnet_id)) => (
                RawEffectivePrincipal::SubnetId(subnet_id.as_slice().to_vec()),
                None,
            ),
            Some(CreateCanisterPlacement::CanisterId(canister_id)) => (
                RawEffectivePrincipal::CanisterId(canister_id.as_slice().to_vec()),
                Some(canister_id),
            ),
        };
        Ok(call_candid_as::<_, (CanisterIdRecord,)>(
            self,
            Principal::management_canister(),
            effective_principal,
            sender.unwrap_or(Principal::anonymous()),
            "provisional_create_canister_with_cycles",
            (ProvisionalCreateCanisterWithCyclesArgs {
                settings: params.settings,
                amount: Some(cycles.into()),
                specified_id,
                sender_canister_version: None,
            },),
        )
        .await?
        .0
        .canister_id)
    }

    /// Upload a WASM chunk to the WASM chunk store of a canister.
    /// Returns the WASM chunk hash.
    #[instrument(skip(self), fields(instance_id=self.instance_id, canister_id = %canister_id.to_string(), sender = %sender.unwrap_or(Principal::anonymous()).to_string()))]
    pub async fn upload_chunk(
        &self,
        canister_id: CanisterId,
        sender: Option<Principal>,
        chunk: Vec<u8>,
    ) -> Result<Vec<u8>, ClientError> {
        Ok(call_candid_as::<_, (UploadChunkResult,)>(
            self,
            Principal::management_canister(),
            RawEffectivePrincipal::CanisterId(canister_id.as_slice().to_vec()),
            sender.unwrap_or(Principal::anonymous()),
            "upload_chunk",
            (UploadChunkArgs { canister_id, chunk },),
        )
        .await?
        .0
        .hash)
    }

    /// List WASM chunk hashes in the WASM chunk store of a canister.
    #[instrument(skip(self), fields(instance_id=self.instance_id, canister_id = %canister_id.to_string(), sender = %sender.unwrap_or(Principal::anonymous()).to_string()))]
    pub async fn stored_chunks(
        &self,
        canister_id: CanisterId,
        sender: Option<Principal>,
    ) -> Result<Vec<Vec<u8>>, ClientError> {
        Ok(call_candid_as::<_, (StoredChunksResult,)>(
            self,
            Principal::management_canister(),
            RawEffectivePrincipal::CanisterId(canister_id.as_slice().to_vec()),
            sender.unwrap_or(Principal::anonymous()),
            "stored_chunks",
            (CanisterIdRecord { canister_id },),
        )
        .await?
        .0
        .into_iter()
        .map(|chunk| chunk.hash)
        .collect())
    }

    /// Clear the WASM chunk store of a canister.
    #[instrument(skip(self), fields(instance_id=self.instance_id, canister_id = %canister_id.to_string(), sender = %sender.unwrap_or(Principal::anonymous()).to_string()))]
    pub async fn clear_chunk_store(
        &self,
        canister_id: CanisterId,
        sender: Option<Principal>,
    ) -> Result<(), ClientError> {
        call_candid_as(
            self,
            Principal::management_canister(),
            RawEffectivePrincipal::CanisterId(canister_id.as_slice().to_vec()),
            sender.unwrap_or(Principal::anonymous()),
            "clear_chunk_store",
            (CanisterIdRecord { canister_id },),
        )
        .await
    }

    /// Install a WASM module assembled from chunks on an existing canister.
    #[instrument(skip(self, mode, chunk_hashes_list, wasm_module_hash, arg), fields(instance_id=self.instance_id, canister_id = %canister_id.to_string(), sender = %sender.unwrap_or(Principal::anonymous()).to_string(), store_canister_id = %store_canister_id.to_string(), arg_len = %arg.len()))]
    pub async fn install_chunked_canister(
        &self,
        canister_id: CanisterId,
        sender: Option<Principal>,
        mode: CanisterInstallMode,
        store_canister_id: CanisterId,
        chunk_hashes_list: Vec<Vec<u8>>,
        wasm_module_hash: Vec<u8>,
        arg: Vec<u8>,
    ) -> Result<(), ClientError> {
        call_candid_as(
            self,
            Principal::management_canister(),
            RawEffectivePrincipal::CanisterId(canister_id.as_slice().to_vec()),
            sender.unwrap_or(Principal::anonymous()),
            "install_chunked_code",
            (InstallChunkedCodeArgs {
                mode,
                target_canister: canister_id,
                store_canister: Some(store_canister_id),
                chunk_hashes_list: chunk_hashes_list
                    .into_iter()
                    .map(|hash| ChunkHash { hash })
                    .collect(),
                wasm_module_hash,
                arg,
                sender_canister_version: None,
            },),
        )
        .await
    }

    async fn install_canister_helper(
        &self,
        mode: CanisterInstallMode,
        canister_id: CanisterId,
        wasm_module: Vec<u8>,
        arg: Vec<u8>,
        sender: Option<Principal>,
    ) -> Result<(), ClientError> {
        if wasm_module.len() + arg.len() < INSTALL_CHUNKED_CODE_THRESHOLD {
            call_candid_as::<(InstallCodeArgs,), ()>(
                self,
                Principal::management_canister(),
                RawEffectivePrincipal::CanisterId(canister_id.as_slice().to_vec()),
                sender.unwrap_or(Principal::anonymous()),
                "install_code",
                (InstallCodeArgs {
                    mode,
                    canister_id,
                    wasm_module,
                    arg,
                    sender_canister_version: None,
                },),
            )
            .await?
        } else {
            self.clear_chunk_store(canister_id, sender).await?;
            let chunks: Vec<_> = wasm_module.chunks(INSTALL_CODE_CHUNK_SIZE).collect();
            let mut hashes = vec![];
            for chunk in chunks {
                let hash = self
                    .upload_chunk(canister_id, sender, chunk.to_vec())
                    .await?;
                hashes.push(hash);
            }
            let mut hasher = Sha256::new();
            hasher.update(wasm_module);
            let wasm_module_hash = hasher.finalize().to_vec();
            self.install_chunked_canister(
                canister_id,
                sender,
                mode,
                canister_id,
                hashes,
                wasm_module_hash,
                arg,
            )
            .await?
        };
        Ok(())
    }

    /// Install a WASM module on an existing canister.
    #[instrument(skip(self, wasm_module, arg), fields(instance_id=self.instance_id, canister_id = %canister_id.to_string(), wasm_module_len = %wasm_module.len(), arg_len = %arg.len(), sender = %sender.unwrap_or(Principal::anonymous()).to_string()))]
    pub async fn install_canister(
        &self,
        canister_id: CanisterId,
        wasm_module: Vec<u8>,
        arg: Vec<u8>,
        sender: Option<Principal>,
    ) -> Result<(), ClientError> {
        self.install_canister_helper(
            CanisterInstallMode::Install,
            canister_id,
            wasm_module,
            arg,
            sender,
        )
        .await
    }

    /// Upgrade a canister with a new WASM module.
    #[instrument(skip(self, wasm_module, arg), fields(instance_id=self.instance_id, canister_id = %canister_id.to_string(), wasm_module_len = %wasm_module.len(), arg_len = %arg.len(), sender = %sender.unwrap_or(Principal::anonymous()).to_string()))]
    pub async fn upgrade_canister(
        &self,
        canister_id: CanisterId,
        wasm_module: Vec<u8>,
        arg: Vec<u8>,
        sender: Option<Principal>,
    ) -> Result<(), ClientError> {
        self.install_canister_helper(
            CanisterInstallMode::Upgrade(None),
            canister_id,
            wasm_module,
            arg,
            sender,
        )
        .await
    }

    /// Reinstall a canister WASM module.
    #[instrument(skip(self, wasm_module, arg), fields(instance_id=self.instance_id, canister_id = %canister_id.to_string(), wasm_module_len = %wasm_module.len(), arg_len = %arg.len(), sender = %sender.unwrap_or(Principal::anonymous()).to_string()))]
    pub async fn reinstall_canister(
        &self,
        canister_id: CanisterId,
        wasm_module: Vec<u8>,
        arg: Vec<u8>,
        sender: Option<Principal>,
    ) -> Result<(), ClientError> {
        self.install_canister_helper(
            CanisterInstallMode::Reinstall,
            canister_id,
            wasm_module,
            arg,
            sender,
        )
        .await
    }

    /// Uninstall a canister.
    #[instrument(skip(self), fields(instance_id=self.instance_id, canister_id = %canister_id.to_string(), sender = %sender.unwrap_or(Principal::anonymous()).to_string()))]
    pub async fn uninstall_canister(
        &self,
        canister_id: CanisterId,
        sender: Option<Principal>,
    ) -> Result<(), ClientError> {
        call_candid_as::<(CanisterIdRecord,), ()>(
            self,
            Principal::management_canister(),
            RawEffectivePrincipal::CanisterId(canister_id.as_slice().to_vec()),
            sender.unwrap_or(Principal::anonymous()),
            "uninstall_code",
            (CanisterIdRecord { canister_id },),
        )
        .await
    }

    /// Take canister snapshot.
    #[instrument(skip(self), fields(instance_id=self.instance_id, canister_id = %canister_id.to_string(), sender = %sender.unwrap_or(Principal::anonymous()).to_string()))]
    pub async fn take_canister_snapshot(
        &self,
        canister_id: CanisterId,
        sender: Option<Principal>,
        replace_snapshot: Option<Vec<u8>>,
    ) -> Result<Snapshot, ClientError> {
        Ok(call_candid_as::<_, (Snapshot,)>(
            self,
            Principal::management_canister(),
            RawEffectivePrincipal::CanisterId(canister_id.as_slice().to_vec()),
            sender.unwrap_or(Principal::anonymous()),
            "take_canister_snapshot",
            (TakeCanisterSnapshotArgs {
                canister_id,
                replace_snapshot,
                uninstall_code: None,
                sender_canister_version: None,
            },),
        )
        .await?
        .0)
    }

    /// Load canister snapshot.
    #[instrument(skip(self), fields(instance_id=self.instance_id, canister_id = %canister_id.to_string(), sender = %sender.unwrap_or(Principal::anonymous()).to_string()))]
    pub async fn load_canister_snapshot(
        &self,
        canister_id: CanisterId,
        sender: Option<Principal>,
        snapshot_id: Vec<u8>,
    ) -> Result<(), ClientError> {
        call_candid_as(
            self,
            Principal::management_canister(),
            RawEffectivePrincipal::CanisterId(canister_id.as_slice().to_vec()),
            sender.unwrap_or(Principal::anonymous()),
            "load_canister_snapshot",
            (LoadCanisterSnapshotArgs {
                canister_id,
                snapshot_id,
                sender_canister_version: None,
            },),
        )
        .await
    }

    /// List canister snapshots.
    #[instrument(skip(self), fields(instance_id=self.instance_id, canister_id = %canister_id.to_string(), sender = %sender.unwrap_or(Principal::anonymous()).to_string()))]
    pub async fn list_canister_snapshots(
        &self,
        canister_id: CanisterId,
        sender: Option<Principal>,
    ) -> Result<Vec<Snapshot>, ClientError> {
        Ok(call_candid_as::<_, (Vec<Snapshot>,)>(
            self,
            Principal::management_canister(),
            RawEffectivePrincipal::CanisterId(canister_id.as_slice().to_vec()),
            sender.unwrap_or(Principal::anonymous()),
            "list_canister_snapshots",
            (CanisterIdRecord { canister_id },),
        )
        .await?
        .0)
    }

    /// Delete canister snapshot.
    #[instrument(skip(self), fields(instance_id=self.instance_id, canister_id = %canister_id.to_string(), sender = %sender.unwrap_or(Principal::anonymous()).to_string()))]
    pub async fn delete_canister_snapshot(
        &self,
        canister_id: CanisterId,
        sender: Option<Principal>,
        snapshot_id: Vec<u8>,
    ) -> Result<(), ClientError> {
        call_candid_as(
            self,
            Principal::management_canister(),
            RawEffectivePrincipal::CanisterId(canister_id.as_slice().to_vec()),
            sender.unwrap_or(Principal::anonymous()),
            "delete_canister_snapshot",
            (DeleteCanisterSnapshotArgs {
                canister_id,
                snapshot_id,
            },),
        )
        .await
    }

    /// Update canister settings.
    #[instrument(skip(self), fields(instance_id=self.instance_id, canister_id = %canister_id.to_string(), sender = %sender.unwrap_or(Principal::anonymous()).to_string()))]
    pub async fn update_canister_settings(
        &self,
        canister_id: CanisterId,
        sender: Option<Principal>,
        settings: CanisterSettings,
    ) -> Result<(), ClientError> {
        call_candid_as::<_, ()>(
            self,
            Principal::management_canister(),
            RawEffectivePrincipal::CanisterId(canister_id.as_slice().to_vec()),
            sender.unwrap_or(Principal::anonymous()),
            "update_settings",
            (UpdateSettingsArgs {
                canister_id,
                settings,
                sender_canister_version: None,
            },),
        )
        .await
    }

    /// Set canister's controllers.
    #[instrument(skip(self), fields(instance_id=self.instance_id, canister_id = %canister_id.to_string(), sender = %sender.unwrap_or(Principal::anonymous()).to_string()))]
    pub async fn set_controllers(
        &self,
        canister_id: CanisterId,
        sender: Option<Principal>,
        new_controllers: Vec<Principal>,
    ) -> Result<(), ClientError> {
        let settings = CanisterSettings {
            controllers: Some(new_controllers),
            ..CanisterSettings::default()
        };
        call_candid_as::<(UpdateSettingsArgs,), ()>(
            self,
            Principal::management_canister(),
            RawEffectivePrincipal::CanisterId(canister_id.as_slice().to_vec()),
            sender.unwrap_or(Principal::anonymous()),
            "update_settings",
            (UpdateSettingsArgs {
                canister_id,
                settings,
                sender_canister_version: None,
            },),
        )
        .await
    }

    /// Start a canister.
    #[instrument(skip(self), fields(instance_id=self.instance_id, canister_id = %canister_id.to_string(), sender = %sender.unwrap_or(Principal::anonymous()).to_string()))]
    pub async fn start_canister(
        &self,
        canister_id: CanisterId,
        sender: Option<Principal>,
    ) -> Result<(), ClientError> {
        call_candid_as::<(CanisterIdRecord,), ()>(
            self,
            Principal::management_canister(),
            RawEffectivePrincipal::CanisterId(canister_id.as_slice().to_vec()),
            sender.unwrap_or(Principal::anonymous()),
            "start_canister",
            (CanisterIdRecord { canister_id },),
        )
        .await
    }

    /// Stop a canister.
    #[instrument(skip(self), fields(instance_id=self.instance_id, canister_id = %canister_id.to_string(), sender = %sender.unwrap_or(Principal::anonymous()).to_string()))]
    pub async fn stop_canister(
        &self,
        canister_id: CanisterId,
        sender: Option<Principal>,
    ) -> Result<(), ClientError> {
        call_candid_as::<(CanisterIdRecord,), ()>(
            self,
            Principal::management_canister(),
            RawEffectivePrincipal::CanisterId(canister_id.as_slice().to_vec()),
            sender.unwrap_or(Principal::anonymous()),
            "stop_canister",
            (CanisterIdRecord { canister_id },),
        )
        .await
    }

    /// Delete a canister.
    #[instrument(skip(self), fields(instance_id=self.instance_id, canister_id = %canister_id.to_string(), sender = %sender.unwrap_or(Principal::anonymous()).to_string()))]
    pub async fn delete_canister(
        &self,
        canister_id: CanisterId,
        sender: Option<Principal>,
    ) -> Result<(), ClientError> {
        call_candid_as::<(CanisterIdRecord,), ()>(
            self,
            Principal::management_canister(),
            RawEffectivePrincipal::CanisterId(canister_id.as_slice().to_vec()),
            sender.unwrap_or(Principal::anonymous()),
            "delete_canister",
            (CanisterIdRecord { canister_id },),
        )
        .await
    }

    /// Checks whether the provided canister exists.
    #[instrument(ret, skip(self), fields(instance_id=self.instance_id, canister_id = %canister_id.to_string()))]
    pub async fn canister_exists(&self, canister_id: CanisterId) -> Result<bool, ClientError> {
        Ok(self.get_subnet(canister_id).await?.is_some())
    }

    /// Deletes a subnet. Returns a server error if the subnet does not exist or is a named subnet.
    #[instrument(ret, skip(self), fields(instance_id=self.instance_id, subnet_id = %subnet_id.to_string()))]
    pub async fn delete_subnet(&self, subnet_id: SubnetId) -> Result<(), ClientError> {
        let endpoint = "update/delete_subnet";
        self.post::<(), RawSubnetId>(
            endpoint,
            RawSubnetId {
                subnet_id: subnet_id.as_slice().to_vec(),
            },
        )
        .await?;

        Ok(())
    }

    /// Returns the subnet ID of the canister if the canister exists.
    #[instrument(ret, skip(self), fields(instance_id=self.instance_id, canister_id = %canister_id.to_string()))]
    pub async fn get_subnet(
        &self,
        canister_id: CanisterId,
    ) -> Result<Option<SubnetId>, ClientError> {
        let endpoint = "read/get_subnet";
        let result: Option<RawSubnetId> = self
            .post(
                endpoint,
                RawCanisterId {
                    canister_id: canister_id.as_slice().to_vec(),
                },
            )
            .await?;
        Ok(result.map(|RawSubnetId { subnet_id }| SubnetId::from_slice(&subnet_id)))
    }

    /// Returns subnet metrics for a given subnet.
    #[instrument(ret, skip(self), fields(instance_id=self.instance_id, subnet_id = %subnet_id.to_string()))]
    pub async fn get_subnet_metrics(
        &self,
        subnet_id: Principal,
    ) -> Result<Option<SubnetMetrics>, ClientError> {
        let path = vec![
            "subnet".into(),
            Label::from_bytes(subnet_id.as_slice()),
            "metrics".into(),
        ];
        let paths = vec![path.clone()];
        let content = ReadState {
            ingress_expiry: self.get_time().await?.as_nanos_since_unix_epoch() + 240_000_000_000,
            sender: Principal::anonymous(),
            paths,
        };
        let envelope = Envelope {
            content: std::borrow::Cow::Borrowed(&content),
            sender_pubkey: None,
            sender_sig: None,
            sender_delegation: None,
        };

        let serialized_bytes = encode_envelope(&envelope, usize::MAX)?;

        let endpoint = format!("api/v2/subnet/{}/read_state", subnet_id.to_text());
        let url = self.instance_url().join(&endpoint).unwrap();
        let mut exchange = Exchange::new(&url, self.max_request_time_ms);
        let (status, bytes) = exchange
            .bytes(
                &self.reqwest_client,
                self.reqwest_client
                    .post(url.clone())
                    .header(reqwest::header::CONTENT_TYPE, "application/cbor")
                    .body(serialized_bytes),
            )
            .await?;
        if !status.is_success() {
            return Err(exchange.server_error(status, &bytes));
        }
        let read_state_response: ReadStateResponse =
            decode_read_state_response(&bytes, bytes.len())?;
        let cert: Certificate = decode_certificate(
            &read_state_response.certificate,
            read_state_response.certificate.len(),
        )?;

        let metrics = match cert.tree.lookup_path(path) {
            LookupResult::Found(value) => value,
            _ => return Ok(None),
        };
        decode_subnet_metrics(metrics, metrics.len()).map_err(ClientError::from)
    }

    /// This (asynchronous) drop function must be called to drop the PocketIc instance.
    /// It must be called manually as Rust doesn't support asynchronous drop.
    pub async fn drop(mut self) -> Result<(), ClientError> {
        self.do_drop().await?;

        Ok(())
    }

    pub(crate) async fn do_drop(&mut self) -> Result<(), ClientError> {
        if self.owns_instance {
            let url = self.instance_url();
            let mut exchange = Exchange::new(&url, self.max_request_time_ms);
            let (status, bytes) = exchange
                .bytes(
                    &self.reqwest_client,
                    self.reqwest_client.delete(url.clone()),
                )
                .await?;
            if !status.is_success() {
                return Err(exchange.server_error(status, &bytes));
            }
            self.owns_instance = false;
            Ok(())
        } else {
            self.stop_http_gateway().await
        }
    }

    async fn get<T: DeserializeOwned>(&self, endpoint: &str) -> Result<T, ClientError> {
        self.request(HttpMethod::Get, endpoint, ()).await
    }

    async fn post<T: DeserializeOwned, B: Serialize>(
        &self,
        endpoint: &str,
        body: B,
    ) -> Result<T, ClientError> {
        self.request(HttpMethod::Post, endpoint, body).await
    }

    async fn request<T: DeserializeOwned, B: Serialize>(
        &self,
        method: HttpMethod,
        endpoint: &str,
        body: B,
    ) -> Result<T, ClientError> {
        self.request_limited(method, endpoint, body, None).await
    }

    async fn request_limited<T: DeserializeOwned, B: Serialize>(
        &self,
        method: HttpMethod,
        endpoint: &str,
        body: B,
        response_limit: Option<usize>,
    ) -> Result<T, ClientError> {
        let url = self.instance_url().join(endpoint).unwrap();
        let mut exchange = Exchange::new(&url, self.max_request_time_ms);
        exchange.response_limit = response_limit;
        loop {
            let builder = if let Some(operation) = &exchange.context.operation {
                self.reqwest_client.get(
                    self.server_url
                        .join(&format!(
                            "/read_graph/{}/{}",
                            operation.state_label, operation.op_id
                        ))
                        .unwrap(),
                )
            } else {
                match method {
                    HttpMethod::Get => self.reqwest_client.get(url.clone()),
                    HttpMethod::Post => self.reqwest_client.post(url.clone()).json(&body),
                }
            };
            let (status, bytes) = exchange.bytes(&self.reqwest_client, builder).await?;
            match status {
                StatusCode::OK => {
                    let value = exchange.decode(&bytes)?;
                    if let Some(operation) = &exchange.context.operation {
                        // The result is known; pruning is best-effort housekeeping, never a resubmission.
                        let request = self.reqwest_client.delete(
                            self.server_url
                                .join(&format!(
                                    "/prune_graph/{}/{}",
                                    operation.state_label, operation.op_id
                                ))
                                .unwrap(),
                        );
                        tokio::spawn(async move {
                            let _ = request.send().await;
                        });
                    }
                    return Ok(value);
                }
                StatusCode::ACCEPTED | StatusCode::CONFLICT => {
                    let receipt: crate::pocket_ic::common::rest::StartedOrBusyResponse =
                        exchange.decode(&bytes)?;
                    if exchange.context.operation.is_none() {
                        if status == StatusCode::ACCEPTED {
                            exchange.started(Operation {
                                state_label: receipt.state_label,
                                op_id: receipt.op_id,
                            });
                        } else {
                            // Busy explicitly says this submission was not accepted.
                            exchange.context.may_have_executed = false;
                        }
                    }
                    // Once accepted, always poll our original receipt, even if an
                    // unexpected response refers to a different computation.
                }
                StatusCode::NOT_FOUND if exchange.context.operation.is_some() => {}
                _ => return Err(exchange.server_error(status, &bytes)),
            }
            exchange.pause().await?;
        }
    }

    /// Get the pending canister HTTP outcalls.
    /// Note that an additional `PocketIc::tick` is necessary after a canister
    /// executes a message making a canister HTTP outcall for the HTTP outcall
    /// to be retrievable here.
    /// Note that, unless a PocketIC instance is in auto progress mode,
    /// a response to the pending canister HTTP outcalls
    /// must be produced by the test driver and passed on to the PocketIC instace
    /// using `PocketIc::mock_canister_http_response`. Flexible outcall mocking is
    /// not supported by this client.
    /// In auto progress mode, the PocketIC server produces a response for every
    /// pending canister HTTP outcall by actually making an HTTP request
    /// to the specified URL.
    #[instrument(ret, skip(self), fields(instance_id=self.instance_id))]
    pub async fn get_canister_http(&self) -> Result<Vec<CanisterHttpRequest>, ClientError> {
        let endpoint = "read/get_canister_http";
        let res: Vec<RawCanisterHttpRequest> = self.get(endpoint).await?;
        Ok(res.into_iter().map(|r| r.into()).collect())
    }

    /// Mock a response to a pending canister HTTP outcall: the same response for
    /// every node of the subnet, or one response per node if
    /// `MockCanisterHttpResponse::additional_responses` is non-empty.
    #[instrument(ret, skip(self), fields(instance_id=self.instance_id))]
    pub async fn mock_canister_http_response(
        &self,
        mock_canister_http_response: MockCanisterHttpResponse,
    ) -> Result<(), ClientError> {
        let endpoint = "update/mock_canister_http";
        let raw_mock_canister_http_response: RawMockCanisterHttpResponse =
            mock_canister_http_response.into();
        self.post(endpoint, raw_mock_canister_http_response).await
    }

    /// Download a canister snapshot to a given snapshot directory.
    /// The sender must be a controller of the canister.
    /// The snapshot directory must be empty if it exists.
    #[instrument(ret, skip(self), fields(instance_id=self.instance_id))]
    pub async fn canister_snapshot_download(
        &self,
        canister_id: CanisterId,
        sender: Principal,
        snapshot_id: Vec<u8>,
        snapshot_dir: PathBuf,
    ) -> Result<(), ClientError> {
        let endpoint = "update/canister_snapshot_download";
        #[cfg(not(windows))]
        let snapshot_dir = snapshot_dir;
        #[cfg(windows)]
        let snapshot_dir = wsl_path(&snapshot_dir, "snapshot directory").into();
        let raw_canister_snapshot_download = RawCanisterSnapshotDownload {
            sender: sender.into(),
            canister_id: canister_id.into(),
            snapshot_id,
            snapshot_dir,
        };
        self.post(endpoint, raw_canister_snapshot_download).await
    }

    /// Upload a canister snapshot from a given snapshot directory.
    /// The sender must be a controller of the canister.
    /// Returns the snapshot ID of the uploaded snapshot.
    #[instrument(ret, skip(self), fields(instance_id=self.instance_id))]
    pub async fn canister_snapshot_upload(
        &self,
        canister_id: CanisterId,
        sender: Principal,
        replace_snapshot: Option<Vec<u8>>,
        snapshot_dir: PathBuf,
    ) -> Result<Vec<u8>, ClientError> {
        let endpoint = "update/canister_snapshot_upload";
        let replace_snapshot =
            replace_snapshot.map(|snapshot_id| RawCanisterSnapshotId { snapshot_id });
        #[cfg(not(windows))]
        let snapshot_dir = snapshot_dir;
        #[cfg(windows)]
        let snapshot_dir = wsl_path(&snapshot_dir, "snapshot directory").into();
        let raw_canister_snapshot_upload = RawCanisterSnapshotUpload {
            sender: sender.into(),
            canister_id: canister_id.into(),
            replace_snapshot,
            snapshot_dir,
        };
        Ok(self
            .post::<RawCanisterSnapshotId, _>(endpoint, raw_canister_snapshot_upload)
            .await?
            .snapshot_id)
    }
}

/// Call a canister candid method, authenticated. The sender can be impersonated (i.e., the
/// signature is not verified).
/// PocketIC executes update calls synchronously, so there is no need to poll for the result.
pub async fn call_candid_as<Input, Output>(
    env: &PocketIc,
    canister_id: CanisterId,
    effective_principal: RawEffectivePrincipal,
    sender: Principal,
    method: &str,
    input: Input,
) -> Result<Output, ClientError>
where
    Input: ArgumentEncoder,
    Output: for<'a> ArgumentDecoder<'a>,
{
    with_candid(input, |payload| async {
        env.update_call_with_effective_principal(
            canister_id,
            effective_principal,
            sender,
            method,
            payload,
        )
        .await
    })
    .await
}

/// Call a canister candid method, anonymous.
/// PocketIC executes update calls synchronously, so there is no need to poll for the result.
pub async fn call_candid<Input, Output>(
    env: &PocketIc,
    canister_id: CanisterId,
    effective_principal: RawEffectivePrincipal,
    method: &str,
    input: Input,
) -> Result<Output, ClientError>
where
    Input: ArgumentEncoder,
    Output: for<'a> ArgumentDecoder<'a>,
{
    call_candid_as(
        env,
        canister_id,
        effective_principal,
        Principal::anonymous(),
        method,
        input,
    )
    .await
}

/// Call a canister candid query method, anonymous.
pub async fn query_candid<Input, Output>(
    env: &PocketIc,
    canister_id: CanisterId,
    method: &str,
    input: Input,
) -> Result<Output, ClientError>
where
    Input: ArgumentEncoder,
    Output: for<'a> ArgumentDecoder<'a>,
{
    query_candid_as(env, canister_id, Principal::anonymous(), method, input).await
}

/// Call a canister candid query method, authenticated. The sender can be impersonated (i.e., the
/// signature is not verified).
pub async fn query_candid_as<Input, Output>(
    env: &PocketIc,
    canister_id: CanisterId,
    sender: Principal,
    method: &str,
    input: Input,
) -> Result<Output, ClientError>
where
    Input: ArgumentEncoder,
    Output: for<'a> ArgumentDecoder<'a>,
{
    with_candid(input, |bytes| async {
        env.query_call(canister_id, sender, method, bytes).await
    })
    .await
}

/// Call a canister candid update method, anonymous.
pub async fn update_candid<Input, Output>(
    env: &PocketIc,
    canister_id: CanisterId,
    method: &str,
    input: Input,
) -> Result<Output, ClientError>
where
    Input: ArgumentEncoder,
    Output: for<'a> ArgumentDecoder<'a>,
{
    update_candid_as(env, canister_id, Principal::anonymous(), method, input).await
}

/// Call a canister candid update method, authenticated. The sender can be impersonated (i.e., the
/// signature is not verified).
pub async fn update_candid_as<Input, Output>(
    env: &PocketIc,
    canister_id: CanisterId,
    sender: Principal,
    method: &str,
    input: Input,
) -> Result<Output, ClientError>
where
    Input: ArgumentEncoder,
    Output: for<'a> ArgumentDecoder<'a>,
{
    with_candid(input, |bytes| async {
        env.update_call(canister_id, sender, method, bytes).await
    })
    .await
}

/// A helper function that we use to implement both [`call_candid`] and
/// [`query_candid`].
pub async fn with_candid<Input, Output, Fut>(
    input: Input,
    f: impl FnOnce(Vec<u8>) -> Fut,
) -> Result<Output, ClientError>
where
    Input: ArgumentEncoder,
    Output: for<'a> ArgumentDecoder<'a>,
    Fut: Future<Output = Result<Vec<u8>, ClientError>>,
{
    let input = encode_args(input)?;
    let output = f(input).await?;
    Ok(decode_args(&output)?)
}

fn setup_tracing(pid: u32) -> Option<WorkerGuard> {
    use tracing_subscriber::prelude::*;
    match std::env::var(LOG_DIR_PATH_ENV_NAME).map(std::path::PathBuf::from) {
        Ok(p) => {
            std::fs::create_dir_all(&p).expect("Could not create directory");

            let file_name = format!("pocket_ic_client_{pid}.log");
            let appender = tracing_appender::rolling::never(&p, file_name);
            let (non_blocking_appender, guard) = tracing_appender::non_blocking(appender);
            let log_dir_filter: EnvFilter =
                tracing_subscriber::EnvFilter::try_from_env(LOG_DIR_LEVELS_ENV_NAME)
                    .unwrap_or_else(|_| "trace".parse().unwrap());

            let layers = vec![
                tracing_subscriber::fmt::layer()
                    .with_writer(non_blocking_appender)
                    // disable color escape codes in files
                    .with_ansi(false)
                    .with_filter(log_dir_filter)
                    .boxed(),
            ];
            let _ = tracing_subscriber::registry().with(layers).try_init();
            Some(guard)
        }
        _ => None,
    }
}

/// Retrieves a default effective canister id for canister creation on a PocketIC instance
/// characterized by:
///  - a PocketIC instance URL of the form `http://<ip>:<port>/instances/<instance_id>`;
///  - a PocketIC HTTP gateway URL of the form `http://<ip>:port` for a PocketIC instance.
///
/// Returns an error if the PocketIC instance topology could not be fetched or parsed, e.g.,
/// because the given URL points to a replica (i.e., does not meet any of the above two properties).
pub async fn get_default_effective_canister_id(
    pocket_ic_url: String,
) -> Result<Principal, ClientError> {
    let url = Url::parse(&format!(
        "{}/_/topology",
        pocket_ic_url.trim_end_matches('/')
    ))
    .map_err(|e| ClientError::Protocol(e.to_string()))?;
    let client = reqwest::Client::new();
    let mut exchange = Exchange::new(&url, Some(crate::pocket_ic::DEFAULT_MAX_REQUEST_TIME_MS));
    loop {
        let (status, bytes) = exchange.bytes(&client, client.get(url.clone())).await?;
        if status == StatusCode::CONFLICT {
            exchange.pause().await?;
            continue;
        }
        if !status.is_success() {
            return Err(exchange.server_error(status, &bytes));
        }
        let topology: Topology = exchange.decode(&bytes)?;
        return Principal::try_from_slice(&topology.default_effective_canister_id.canister_id)
            .map_err(|e| ClientError::Protocol(e.to_string()));
    }
}
