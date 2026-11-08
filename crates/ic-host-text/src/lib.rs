//! Pure text decoding, normalization, terminal-safe rendering and tables.
//!
//! Callers own schemas, input-byte allowances and output.
pub mod decimal;
pub mod duration;
pub mod escape;
pub mod normalize;
pub mod percent;
pub mod progress;
pub mod quantity;
pub mod table;
pub mod timestamp;
pub mod width;
#[cfg(feature = "yaml")]
pub mod yaml;
