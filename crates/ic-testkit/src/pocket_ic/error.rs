//! Errors retain the request evidence needed for consumer-owned recovery.

use reqwest::StatusCode;

/// Recovery-relevant CBOR refusal, independent of Platform's codec error type.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum CborErrorKind {
    /// Encoded input exceeded its allowance before decoding.
    InputTooLarge,
    /// CBOR or its required protocol shape was invalid, including trailing bytes.
    Decode,
    /// Encoded output would exceed its allowance; no partial output escaped.
    OutputTooLarge,
    /// Serialization failed for a reason other than the output allowance.
    Encode,
}

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
    #[error(
        "PocketIC requires an explicit server URL; provision/start it through ic_testkit::pic or supply a running server"
    )]
    MissingServerUrl,
    #[error("PocketIC request budget must be positive and fit the host clock: {timeout:?}")]
    InvalidRequestBudget { timeout: std::time::Duration },
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
    #[error("PocketIC response body allocation failed: {source}")]
    BodyAllocation {
        context: Box<RequestContext>,
        #[source]
        source: std::collections::TryReserveError,
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
    #[error("CBOR failed: {source}")]
    Cbor {
        /// Stable Testkit classification; no supplier downcast is required.
        kind: CborErrorKind,
        /// Original typed codec failure and its causal chain. Downcasting to a
        /// supplier type is optional and couples that caller to its version.
        #[source]
        source: Box<dyn std::error::Error + Send + Sync>,
    },
    #[error("I/O failed: {0}")]
    Io(#[from] std::io::Error),
    #[error("invalid PocketIC response: {0}")]
    Protocol(String),
}

pub(super) fn cbor_error(source: ic_platform::certification::cbor::CborError) -> ClientError {
    use ic_platform::certification::cbor::CborError;
    let kind = match &source {
        CborError::InputTooLarge => CborErrorKind::InputTooLarge,
        CborError::Decode(_) => CborErrorKind::Decode,
        CborError::OutputTooLarge => CborErrorKind::OutputTooLarge,
        CborError::Encode(_) => CborErrorKind::Encode,
    };
    ClientError::Cbor {
        kind,
        source: Box::new(source),
    }
}

impl ClientError {
    pub(crate) fn with_ingress(
        mut self,
        ingress: crate::pocket_ic::common::rest::RawMessageId,
    ) -> Self {
        match &mut self {
            Self::Transport { context, .. }
            | Self::Body { context, .. }
            | Self::BodyAllocation { context, .. }
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
            | Self::BodyAllocation { context, .. }
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
