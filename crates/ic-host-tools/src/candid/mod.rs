//! Candid service interpretation and bounded reply decoding.

pub mod decode;
#[cfg(feature = "candid-service")]
pub mod service;
