//! Errors retain the request evidence needed for consumer-owned recovery.

use reqwest::StatusCode;

/// A computation accepted by the PocketIC server.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Operation {
    pub state_label: String,
    pub op_id: String,
}

/// Evidence about a failed HTTP exchange, not permission to retry it.
#[derive(Clone, Debug)]
pub struct RequestContext {
    pub url: String,
    pub operation: Option<Operation>,
    /// An ingress message already accepted before this failure.
    pub ingress: Option<crate::pocket_ic::common::rest::RawMessageId>,
    /// The request may have reached the server. A transport failure is not rejection.
    pub may_have_executed: bool,
    pub status: Option<StatusCode>,
}

/// Failures from the maintained client. Sources are preserved without string matching.
#[derive(Debug, thiserror::Error)]
pub enum ClientError {
    #[error("PocketIC transport failed: {source}")]
    Transport {
        context: Box<RequestContext>,
        #[source]
        source: reqwest::Error,
    },
    #[error("PocketIC response body failed: {source}")]
    Body {
        context: Box<RequestContext>,
        #[source]
        source: reqwest::Error,
    },
    #[error("PocketIC response JSON failed: {source}")]
    Json {
        context: Box<RequestContext>,
        #[source]
        source: serde_json::Error,
    },
    #[error("PocketIC server returned {status}: {message}")]
    Server {
        context: Box<RequestContext>,
        status: StatusCode,
        message: String,
    },
    #[error("PocketIC request deadline expired")]
    Timeout { context: Box<RequestContext> },
    #[error("PocketIC response exceeds {limit} bytes")]
    ResponseTooLarge {
        context: Box<RequestContext>,
        limit: usize,
    },
    #[error("canister rejected the call: {0:?}")]
    Canister(crate::pocket_ic::RejectResponse),
    #[error("Candid failed: {0}")]
    Candid(#[from] candid::Error),
    #[error("CBOR failed: {0}")]
    Cbor(#[from] serde_cbor::Error),
    #[error("I/O failed: {0}")]
    Io(#[from] std::io::Error),
    #[error("invalid PocketIC response: {0}")]
    Protocol(String),
}

impl ClientError {
    pub(crate) fn with_ingress(
        mut self,
        ingress: crate::pocket_ic::common::rest::RawMessageId,
    ) -> Self {
        match &mut self {
            Self::Transport { context, .. }
            | Self::Body { context, .. }
            | Self::Json { context, .. }
            | Self::Server { context, .. }
            | Self::ResponseTooLarge { context, .. }
            | Self::Timeout { context } => context.ingress = Some(ingress),
            _ => {}
        }
        self
    }

    pub fn request_context(&self) -> Option<&RequestContext> {
        match self {
            Self::Transport { context, .. }
            | Self::Body { context, .. }
            | Self::Json { context, .. }
            | Self::Server { context, .. }
            | Self::ResponseTooLarge { context, .. }
            | Self::Timeout { context } => Some(context),
            _ => None,
        }
    }
}

impl From<crate::pocket_ic::RejectResponse> for ClientError {
    fn from(value: crate::pocket_ic::RejectResponse) -> Self {
        Self::Canister(value)
    }
}
