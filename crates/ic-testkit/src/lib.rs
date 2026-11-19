//! Shared upstream simulator types and explicitly selected test mechanisms.
//!
//! Callers own prepared servers, runtimes, instances and simulated progression.
//! The held-reply slice admits catalogs and generates bounded Wasm modules.
//! Exclusive controllers observe/release held requests and admit bounded clock steps.

#[cfg(feature = "managed-server")]
pub mod server;

#[cfg(feature = "held-reply-fixtures")]
pub mod held;

/// Original upstream client, builder, identifier and error types.
pub use pocket_ic;
