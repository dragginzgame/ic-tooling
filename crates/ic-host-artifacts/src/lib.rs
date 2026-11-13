//! Bounded host artifact streams, raw identities, gzip and Wasm tooling.
#![forbid(unsafe_code)]
pub mod artifact;
pub mod batch;
#[cfg(feature = "wasm")]
pub mod wasm;
#[cfg(feature = "wasm-text")]
pub mod wasm_text;
