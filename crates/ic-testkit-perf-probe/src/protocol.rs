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
mod tests {
    use super::*;

    #[test]
    fn selected_limits_admit_the_largest_control_values() {
        let request =
            ic_wire::native::encode_request(SEED_STATE.id, &u32::MAX, SEED_STATE.request_limits)
                .unwrap();
        assert_eq!(
            ic_wire::native::decode_request::<u32>(
                SEED_STATE.id,
                &request,
                SEED_STATE.request_limits
            ),
            Ok(u32::MAX)
        );
        for first in [None, Some(u8::MAX)] {
            let state = State {
                mutations: u64::MAX,
                bytes: u64::MAX,
                first,
            };
            let reply = ic_wire::native::encode_response(
                INSPECT_STATE.id,
                &state,
                INSPECT_STATE.response_limits,
            )
            .unwrap();
            assert_eq!(
                ic_wire::native::decode_response::<State>(
                    INSPECT_STATE.id,
                    &reply,
                    INSPECT_STATE.response_limits
                ),
                Ok(state)
            );
        }
        let reply = ic_wire::native::encode_response(
            BENCHMARK_ONCE.id,
            &u64::MAX,
            BENCHMARK_ONCE.response_limits,
        )
        .unwrap();
        assert_eq!(
            ic_wire::native::decode_response::<u64>(
                BENCHMARK_ONCE.id,
                &reply,
                BENCHMARK_ONCE.response_limits
            ),
            Ok(u64::MAX)
        );
    }
}
