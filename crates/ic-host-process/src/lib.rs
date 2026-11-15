//! Native process observations and consumer-admitted execution.
#[cfg(unix)]
pub mod child;
pub mod framing;
pub mod observation;
#[cfg(unix)]
pub mod signal;
pub mod status;
#[cfg(all(test, unix))]
mod tests;
#[cfg(unix)]
pub mod tool;

#[cfg(any(test, feature = "test-support"))]
pub mod testing;
