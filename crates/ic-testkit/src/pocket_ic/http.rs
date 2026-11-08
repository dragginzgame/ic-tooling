use crate::pocket_ic::{ClientError, Operation, RequestContext};
use ic_host_http::{ReadFailure, ReadOptions, read_body};
use serde::de::DeserializeOwned;
use std::{future::Future, time::Duration};
use tokio::time::Instant;

/// One budget covers dispatch, response body and all polling.
pub(crate) struct Exchange {
    pub(crate) context: RequestContext,
    deadline: Instant,
    pub(crate) response_limit: Option<usize>,
}

impl Exchange {
    pub(crate) fn new(url: &reqwest::Url, timeout: Duration) -> Result<Self, ClientError> {
        Ok(Self {
            context: RequestContext {
                url: url.to_string(),
                operation: None,
                ingress: None,
                may_have_executed: false,
                status: None,
            },
            deadline: request_deadline(timeout)?,
            response_limit: None,
        })
    }

    async fn bounded<T>(&self, future: impl Future<Output = T>) -> Result<T, ClientError> {
        tokio::time::timeout_at(self.deadline, future)
            .await
            .map_err(|_| ClientError::Timeout {
                context: Box::new(self.context.clone()),
            })
    }

    pub(crate) async fn pause(&self) -> Result<(), ClientError> {
        self.bounded(tokio::time::sleep(Duration::from_millis(10)))
            .await
    }

    pub(crate) async fn bytes(
        &mut self,
        client: &reqwest::Client,
        builder: reqwest::RequestBuilder,
    ) -> Result<(reqwest::StatusCode, Vec<u8>), ClientError> {
        let request = builder.build().map_err(|source| ClientError::Transport {
            context: Box::new(self.context.clone()),
            source,
        })?;
        if Instant::now() >= self.deadline {
            return Err(ClientError::Timeout {
                context: Box::new(self.context.clone()),
            });
        }
        // Conservative once dispatched: never infer non-execution from a lost reply.
        self.context.may_have_executed = true;
        self.context.status = None;
        let response = self
            .bounded(client.execute(request))
            .await?
            .map_err(|source| ClientError::Transport {
                context: Box::new(self.context.clone()),
                source,
            })?;
        let status = response.status();
        self.context.status = Some(status);
        let body = self
            .bounded(read_body(
                response,
                ReadOptions {
                    max_bytes: self.response_limit.unwrap_or(usize::MAX),
                    reject_declared_oversize: false,
                    retain_partial_on_error: false,
                },
            ))
            .await?
            .map_err(|error| {
                let context = Box::new(self.context.clone());
                match error.failure {
                    ReadFailure::Transport(source) => ClientError::Body { context, source },
                    ReadFailure::Allocation(source) => {
                        ClientError::BodyAllocation { context, source }
                    }
                    ReadFailure::DeclaredLength { limit, .. } | ReadFailure::Limit { limit } => {
                        ClientError::ResponseTooLarge { context, limit }
                    }
                }
            })?;
        Ok((status, body.bytes))
    }

    pub(crate) fn decode<T: DeserializeOwned>(&self, bytes: &[u8]) -> Result<T, ClientError> {
        serde_json::from_slice(bytes).map_err(|source| ClientError::Json {
            context: Box::new(self.context.clone()),
            source,
        })
    }

    pub(crate) fn server_error(&self, status: reqwest::StatusCode, bytes: &[u8]) -> ClientError {
        // Non-JSON error bodies still retain the status and server diagnostic.
        let message = serde_json::from_slice::<crate::pocket_ic::common::rest::ApiError>(bytes)
            .map(|error| error.message)
            .unwrap_or_else(|_| String::from_utf8_lossy(bytes).into_owned());
        ClientError::Server {
            context: Box::new(self.context.clone()),
            status,
            message,
        }
    }

    pub(crate) async fn json<T: DeserializeOwned>(
        &mut self,
        client: &reqwest::Client,
        builder: reqwest::RequestBuilder,
    ) -> Result<T, ClientError> {
        let (status, bytes) = self.bytes(client, builder).await?;
        if !status.is_success() {
            return Err(self.server_error(status, &bytes));
        }
        self.decode(&bytes)
    }

    pub(crate) fn started(&mut self, operation: Operation) {
        self.context.operation = Some(operation);
    }
}

pub(crate) fn request_deadline(timeout: Duration) -> Result<Instant, ClientError> {
    Instant::now()
        .checked_add(timeout)
        .filter(|_| !timeout.is_zero())
        .ok_or(ClientError::InvalidRequestBudget { timeout })
}
