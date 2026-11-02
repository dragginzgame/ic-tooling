//! Canister-side benchmark marker emission using IC execution counters.

use crate::benchmark::{BenchmarkCounters, BenchmarkMarkerError, DEFAULT_PREFIX, format_marker};
const WASM_PAGE_BYTES: u128 = 65_536;

/// Admit a start/end label, then print one complete benchmark marker.
///
/// # Errors
/// Returns the marker grammar refusal before printing any bytes.
pub fn measure(label: &str) -> Result<(), BenchmarkMarkerError> {
    let marker = format_marker(DEFAULT_PREFIX, label, counters())?;
    ic_platform_runtime::ic0::debug_print(marker.as_bytes());
    Ok(())
}

/// Read the instruction, Wasm heap, and stable-memory counters.
#[must_use]
pub fn counters() -> BenchmarkCounters {
    BenchmarkCounters {
        instructions: u128::from(ic_platform_runtime::ic0::performance_counter(1)),
        heap_bytes: wasm_memory_size_bytes(),
        memory_bytes: u128::from(ic_platform_runtime::ic0::stable64_size()) * WASM_PAGE_BYTES,
    }
}

fn wasm_memory_size_bytes() -> u128 {
    u128::try_from(std::arch::wasm32::memory_size(0)).expect("usize fits into u128")
        * WASM_PAGE_BYTES
}
