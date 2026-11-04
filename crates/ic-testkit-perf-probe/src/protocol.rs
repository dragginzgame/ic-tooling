//! Private method contracts shared by the probe and its native callers.
//! These bounds cover control values; seeded heap bytes never cross the wire.

use ic_wire::Limits;
use serde::{Deserialize, Serialize};

/// Maximum frame size admitted before the CDK copies an argument.
pub const FRAME_BYTES: usize = 64;
/// Shared byte, allocation, node and depth budgets for private control calls.
pub const LIMITS: Limits = Limits {
    bytes: FRAME_BYTES,
    allocation: 4096,
    nodes: 128,
    depth: 16,
};

/// Snapshot-contained heap state observed by the fixture-reuse workload.
#[derive(Debug, Deserialize, Eq, PartialEq, Serialize)]
pub struct State {
    /// Number of mutations since the state was seeded.
    pub mutations: u64,
    /// Size of the seeded heap allocation.
    pub bytes: u64,
    /// First seeded byte, absent for an empty allocation.
    pub first: Option<u8>,
}

/// Complete private endpoint identity and frame budgets.
#[derive(Clone, Copy)]
pub struct Method {
    /// Stable Wire method discriminator.
    pub id: u64,
    /// Exported canister method name.
    pub name: &'static str,
    /// Request frame and decoding budgets.
    pub request_limits: Limits,
    /// Response frame and decoding budgets.
    pub response_limits: Limits,
}

impl Method {
    const fn new(id: u64, name: &'static str) -> Self {
        Self {
            id,
            name,
            request_limits: LIMITS,
            response_limits: LIMITS,
        }
    }
}

/// Seed the fixture's deterministic heap allocation.
pub const SEED_STATE: Method = Method::new(0x544b_5052_4f42_0001, "fixture_seed_state");

/// Mutate the fixture's counter and first byte.
pub const MUTATE_STATE: Method = Method::new(0x544b_5052_4f42_0002, "fixture_mutate_state");

/// Inspect the fixture's heap without mutating it.
pub const INSPECT_STATE: Method = Method::new(0x544b_5052_4f42_0003, "fixture_state");

/// Confirm the same canister remains callable after an intentional trap.
pub const PING: Method = Method::new(0x544b_5052_4f42_0004, "ping");

/// Execute the measured arithmetic workload.
pub const BENCHMARK_ONCE: Method = Method::new(0x544b_5052_4f42_0005, "benchmark_once");

/// Emit a start marker and intentionally trap before its matching end.
pub const BENCHMARK_TRAP: Method = Method::new(0x544b_5052_4f42_0006, "benchmark_start_then_trap");

/// Observe the actual caller of a query through the typed native adapter.
pub const QUERY_CALLER: Method = Method::new(0x544b_5052_4f42_0007, "fixture_query_caller");

/// Observe the actual caller of an update through the typed native adapter.
pub const UPDATE_CALLER: Method = Method::new(0x544b_5052_4f42_0008, "fixture_update_caller");

#[cfg(test)]
mod tests;
