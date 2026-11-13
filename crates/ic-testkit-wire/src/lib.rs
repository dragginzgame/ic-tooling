//! Version-specific native Wire calls over Testkit's selected upstream client.
//!
//! Wire owns framing and validation. Testkit owns simulator admission and lifecycle;
//! this independently versioned adapter only composes their public interfaces.
//! Consumers select the same Wire generation as this adapter for limits and errors.
#![forbid(unsafe_code)]
#![cfg(not(target_family = "wasm"))]

use ic_testkit::pocket_ic::{CanisterId as Principal, RejectResponse, nonblocking::PocketIc};

use ic_wire::{Limits, native};
use serde::{Serialize, de::DeserializeOwned};

/// Caller-selected endpoint identity and native Serde traversal allowances.
///
/// Limits are explicit per direction and include the frame header. Testkit owns
/// no endpoint registry or application schema; use the same contract as the server.
#[derive(Clone, Copy, Debug)]
pub struct WireCallSpec {
    /// Exact exported query or update name.
    pub method: &'static str,
    /// Stable application-owned Wire method identity.
    pub method_id: u64,
    /// Request frame and Serde traversal allowances.
    pub request_limits: Limits,
    /// Response frame and Serde traversal allowances.
    pub response_limits: Limits,
}

/// Target, caller and method identity attached to a Wire call failure.
#[non_exhaustive]
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct WireCallContext {
    /// Underlying `PocketIC` operation: `query_call` or `update_call`.
    pub operation: &'static str,
    /// Target canister selected by the caller.
    pub canister_id: Principal,
    /// Explicit effective message caller.
    pub caller: Principal,
    /// Exact exported name selected by the caller.
    pub method: &'static str,
    /// Wire identity selected by the caller.
    pub method_id: u64,
}

/// Codec or transport refusal with typed causes and bounded diagnostics.
///
/// Raw reply bytes are not retained or rendered. Use `PocketIC`'s raw methods when
/// original bytes or deliberately malformed requests are required as evidence.
#[non_exhaustive]
#[derive(Debug)]
pub enum WireCallError {
    /// Request encoding failed before transport was invoked.
    Encode {
        /// Target, caller, operation and method identity.
        context: Box<WireCallContext>,
        /// Original Wire refusal.
        source: ic_wire::Error,
    },
    /// The underlying operation returned a canister rejection.
    Call {
        /// Target, caller, operation and method identity.
        context: Box<WireCallContext>,
        /// Complete rejection; upstream does not implement `Error` for this value.
        source: Box<RejectResponse>,
    },
    /// The returned bytes failed the selected method's response admission.
    Decode {
        /// Target, caller, operation and method identity.
        context: Box<WireCallContext>,
        /// Number of already allocated `PocketIC` reply bytes.
        bytes: usize,
        /// Original Wire refusal, including oversized reply admission.
        source: ic_wire::Error,
    },
}

impl std::fmt::Display for WireCallError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let context = match self {
            Self::Encode { context, source } => {
                write!(f, "Wire request encoding failed: {source}")?;
                context
            }
            Self::Call { context, source } => {
                write!(
                    f,
                    "PocketIC call rejected ({:?}, {:?})",
                    source.reject_code, source.error_code
                )?;
                context
            }
            Self::Decode {
                context,
                bytes,
                source,
            } => {
                write!(f, "Wire response decoding failed (bytes={bytes}): {source}")?;
                context
            }
        };
        write!(
            f,
            " (operation={}, canister={}, caller={}, method={}, method_id={})",
            context.operation,
            context.canister_id,
            context.caller,
            context.method,
            context.method_id
        )
    }
}

impl std::error::Error for WireCallError {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        match self {
            Self::Encode { source, .. } | Self::Decode { source, .. } => Some(source),
            Self::Call { .. } => None,
        }
    }
}

/// Encode a method-bound request, execute one query as `caller`, and decode its reply.
///
/// The caller supplies the exact name, identity and request/response limits.
/// Explicit anonymous callers are permitted. Application-level results are returned
/// unchanged. Returned upstream rejections are retained; transport panics propagate.
/// Requests may borrow their input; responses must own their decoded data because
/// the helper releases `PocketIC`'s reply buffer before returning.
/// IC Wire owns native frame admission, primitive validation and traversal limits.
/// Decoding may normalize collections or custom visitors. Use raw `PocketIC` calls
/// to retain original frames when exact bytes are needed for hashes or retries.
/// Primitive acceptance follows upstream Postcard, including redundant varint
/// groups within width bounds; accepted bytes need not re-encode identically.
///
/// # Errors
/// Returns request encoding, query rejection or response decoding errors.
/// Upstream transport panics propagate unchanged.
pub async fn query_wire<Request, Response>(
    pic: &PocketIc,
    canister_id: Principal,
    caller: Principal,
    spec: WireCallSpec,
    request: &Request,
) -> Result<Response, WireCallError>
where
    Request: Serialize + ?Sized,
    Response: DeserializeOwned,
{
    call_wire(
        context("query_call", spec, canister_id, caller),
        spec,
        request,
        async |bytes| {
            pic.query_call(canister_id, caller, spec.method, bytes)
                .await
        },
    )
    .await
}

/// Encode a method-bound request and execute one update as `caller`, without retry.
///
/// The response uses the same admission as `query_wire`. No installation, authority,
/// effect reconciliation or replay policy is selected by this helper.
/// A refused reply does not establish that the update had no effect.
///
/// # Errors
/// Returns request encoding, update rejection or response decoding errors.
/// Upstream transport panics propagate unchanged.
pub async fn update_wire<Request, Response>(
    pic: &PocketIc,
    canister_id: Principal,
    caller: Principal,
    spec: WireCallSpec,
    request: &Request,
) -> Result<Response, WireCallError>
where
    Request: Serialize + ?Sized,
    Response: DeserializeOwned,
{
    call_wire(
        context("update_call", spec, canister_id, caller),
        spec,
        request,
        async |bytes| {
            pic.update_call(canister_id, caller, spec.method, bytes)
                .await
        },
    )
    .await
}

const fn context(
    operation: &'static str,
    spec: WireCallSpec,
    canister_id: Principal,
    caller: Principal,
) -> WireCallContext {
    WireCallContext {
        operation,
        canister_id,
        caller,
        method: spec.method,
        method_id: spec.method_id,
    }
}

async fn call_wire<Request, Response>(
    context: WireCallContext,
    spec: WireCallSpec,
    request: &Request,
    call: impl AsyncFnOnce(Vec<u8>) -> Result<Vec<u8>, RejectResponse>,
) -> Result<Response, WireCallError>
where
    Request: Serialize + ?Sized,
    Response: DeserializeOwned,
{
    let request =
        native::encode_request(spec.method_id, request, spec.request_limits).map_err(|source| {
            WireCallError::Encode {
                context: Box::new(context.clone()),
                source,
            }
        })?;
    let reply = call(request).await.map_err(|source| WireCallError::Call {
        context: Box::new(context.clone()),
        source: Box::new(source),
    })?;
    native::decode_response(spec.method_id, &reply, spec.response_limits).map_err(|source| {
        WireCallError::Decode {
            context: Box::new(context),
            bytes: reply.len(),
            source,
        }
    })
}

#[cfg(test)]
mod tests;
