//! Complete bounded HTTP bodies. Consumers own clients and request/effect policy.
//!
//! Limits count bytes yielded by Reqwest; its network and decoding buffers are
//! outside this budget. No request, retry, runtime or detached task is created.
#![cfg(not(target_family = "wasm"))]

#[cfg(test)]
mod tests;

use std::collections::TryReserveError;

use reqwest::Response;
use thiserror::Error;

///
/// Error
/// Refused body admission; no partial bytes or resumable response are returned.
/// Display is redacted. Original transport sources may contain sensitive URLs.
///

#[derive(Debug, Error)]
pub enum Error {
    /// An available declared length or observed chunk exceeds the caller's ceiling.
    #[error("HTTP body exceeds {limit} bytes")]
    TooLarge {
        /// Selected maximum number of accumulated bytes.
        limit: usize,
        /// Oversized declared length, or None when streamed bytes exceeded the limit.
        declared: Option<u64>,
    },
    /// Original body framing, timeout, connection or decoding failure.
    #[error("HTTP body transport failed")]
    Transport(#[source] reqwest::Error),
    /// Original fallible allocation failure.
    #[error("HTTP body allocation failed")]
    Allocation(#[source] TryReserveError),
}

/// Consume a response into complete owned bytes within `limit`.
///
/// An oversized available Content-Length refuses before polling the body. Each
/// chunk is admitted before allocation/copy; storage is never reserved from a
/// remote length hint. Requested capacity stays within `limit`, though the
/// allocator may round up. Empty bodies are accepted at a zero limit.
///
/// Caller-selected request timeouts still apply. Dropping this future drops its
/// buffer/response, without creating a retry or proving no server-side effect.
/// Status, headers, TLS, redirects and decompression remain consumer decisions.
///
/// # Errors
/// Returns size, transport or allocation refusal without a partial body.
pub async fn body(mut response: Response, limit: usize) -> Result<Vec<u8>, Error> {
    if let Some(declared) = response.content_length()
        && u128::from(declared) > limit as u128
    {
        return Err(Error::TooLarge {
            limit,
            declared: Some(declared),
        });
    }
    let mut bytes = Vec::new();
    while let Some(chunk) = response.chunk().await.map_err(Error::Transport)? {
        if chunk.len() > limit - bytes.len() {
            return Err(Error::TooLarge {
                limit,
                declared: None,
            });
        }
        let needed = bytes.len() + chunk.len();
        if needed > bytes.capacity() {
            let capacity = needed.max(bytes.capacity().saturating_mul(2)).min(limit);
            bytes
                .try_reserve_exact(capacity - bytes.len())
                .map_err(Error::Allocation)?;
        }
        bytes.extend_from_slice(&chunk);
    }

    Ok(bytes)
}
