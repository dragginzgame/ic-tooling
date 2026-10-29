//! Shared workload report fields; execution, admission and summaries remain caller-owned.

use serde::{Deserialize, Serialize};

use crate::fixture_reuse_protocol::Mode;

#[derive(Default, Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
#[expect(
    clippy::struct_field_names,
    reason = "serialized field names explicitly carry their millisecond units"
)]
pub struct Phases {
    pub(super) wait_ms: f64,
    #[serde(deserialize_with = "Option::deserialize")]
    pub(super) build_ms: Option<f64>,
    #[serde(deserialize_with = "Option::deserialize")]
    pub(super) restore_ms: Option<f64>,
    #[serde(deserialize_with = "Option::deserialize")]
    pub(super) reset_ms: Option<f64>,
    #[serde(deserialize_with = "Option::deserialize")]
    pub(super) readiness_ms: Option<f64>,
    #[serde(deserialize_with = "Option::deserialize")]
    pub(super) validation_ms: Option<f64>,
    pub(super) acquisition_ms: f64,
}

impl Phases {
    pub(super) const fn values(&self) -> [Option<f64>; 7] {
        [
            Some(self.wait_ms),
            self.build_ms,
            self.restore_ms,
            self.reset_ms,
            self.readiness_ms,
            self.validation_ms,
            Some(self.acquisition_ms),
        ]
    }
}

#[derive(Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
pub struct Sample {
    pub(super) iteration: usize,
    pub(super) phases: Phases,
    pub(super) body_ms: f64,
    pub(super) release_ms: f64,
    pub(super) total_ms: f64,
}

#[derive(Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
pub struct WorkerReport {
    pub(super) mode: Mode,
    pub(super) capacity: usize,
    pub(super) iterations: usize,
    pub(super) workers: usize,
    pub(super) state_bytes_per_canister: u32,
    pub(super) wall_ms: f64,
    pub(super) fixture_wall_ms: f64,
    pub(super) samples: Vec<Sample>,
    pub(super) server_startup_ms: f64,
    pub(super) cold_preparation: Vec<Phases>,
    pub(super) preparation_wall_ms: f64,
    pub(super) pool_teardown_ms: f64,
    pub(super) server_teardown_ms: f64,
}

#[cfg(test)]
pub fn worker_report_fixture(iterations: usize) -> WorkerReport {
    WorkerReport {
        mode: Mode::Fresh,
        capacity: 1,
        iterations,
        workers: 1,
        state_bytes_per_canister: 0,
        wall_ms: 4.0,
        fixture_wall_ms: 5.0,
        samples: (0..iterations)
            .map(|iteration| Sample {
                iteration,
                phases: Phases {
                    build_ms: Some(1.0),
                    validation_ms: Some(1.0),
                    acquisition_ms: 2.0,
                    ..Phases::default()
                },
                body_ms: 1.0,
                release_ms: 1.0,
                total_ms: 4.0,
            })
            .collect(),
        server_startup_ms: 6.0,
        cold_preparation: Vec::new(),
        preparation_wall_ms: 1.0,
        pool_teardown_ms: 0.5,
        server_teardown_ms: 0.5,
    }
}
