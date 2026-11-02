//! Progress observation and failure attribution for Wasm acquisition.

use super::{
    WasmBuildError, WasmBuildFailureDetails, WasmBuildFailureTimings, WasmBuildPhase,
    WasmBuildProgressConfig, WasmBuildProgressEvent,
};
use ic_host_process::tool::OutputStream;
use std::{
    sync::mpsc::{self, RecvTimeoutError},
    thread,
    time::{Duration, Instant},
};

struct ProgressObservation<'a> {
    config: WasmBuildProgressConfig,
    observer: &'a mut dyn FnMut(WasmBuildProgressEvent),
    last_event: Instant,
}

pub(super) struct ProgressReporter<'a> {
    observation: Option<ProgressObservation<'a>>,
    failure_phase: WasmBuildPhase,
    failure_timings: WasmBuildFailureTimings,
}

impl<'a> ProgressReporter<'a> {
    pub(super) fn heartbeat_interval(&self) -> Option<Duration> {
        self.observation
            .as_ref()
            .and_then(|observation| observation.config.heartbeat_interval())
    }

    pub(super) fn silent() -> Self {
        Self {
            observation: None,
            failure_phase: WasmBuildPhase::Specification,
            failure_timings: WasmBuildFailureTimings::default(),
        }
    }

    pub(super) fn new(
        observation: Option<(
            WasmBuildProgressConfig,
            &'a mut dyn FnMut(WasmBuildProgressEvent),
        )>,
    ) -> Result<Self, WasmBuildError> {
        let Some((config, observer)) = observation else {
            return Ok(Self::silent());
        };
        if config.heartbeat_interval == Some(Duration::ZERO) {
            return Err(WasmBuildError::InvalidSpec {
                message: "Wasm build progress heartbeat interval must be greater than zero"
                    .to_owned(),
            });
        }
        Ok(Self {
            observation: Some(ProgressObservation {
                config,
                observer,
                last_event: Instant::now(),
            }),
            ..Self::silent()
        })
    }

    pub(super) fn emit_cargo_output(&mut self, stream: OutputStream, bytes: &[u8]) {
        if self
            .observation
            .as_ref()
            .is_some_and(|observation| observation.config.emit_cargo_output)
        {
            self.emit(WasmBuildProgressEvent::CargoOutput {
                stream,
                bytes: bytes.to_vec(),
            });
        }
    }

    pub(super) fn emit(&mut self, event: WasmBuildProgressEvent) {
        if let Some(observation) = &mut self.observation {
            (observation.observer)(event);
            observation.last_event = Instant::now();
        }
    }

    pub(super) fn heartbeat_due_in(&self) -> Option<Duration> {
        self.observation.as_ref().and_then(|observation| {
            observation
                .config
                .heartbeat_interval
                .map(|interval| interval.saturating_sub(observation.last_event.elapsed()))
        })
    }

    pub(super) fn emit_heartbeat(&mut self, phase: WasmBuildPhase, elapsed: Duration) {
        self.emit(WasmBuildProgressEvent::Heartbeat { phase, elapsed });
    }

    pub(super) fn emit_heartbeat_if_due(&mut self, phase: WasmBuildPhase, elapsed: Duration) {
        if self.heartbeat_due_in() == Some(Duration::ZERO) {
            self.emit_heartbeat(phase, elapsed);
        }
    }

    pub(super) fn run_phase<T, F>(&mut self, phase: WasmBuildPhase, operation: F) -> T
    where
        T: Send,
        F: FnOnce() -> T + Send,
    {
        let started = Instant::now();
        self.begin_phase(phase);
        let result = if self.heartbeat_due_in().is_none() {
            operation()
        } else {
            thread::scope(|scope| {
                let (finished, completion) = mpsc::sync_channel(0);
                let worker = scope.spawn(move || {
                    let result = operation();
                    let _ = finished.send(());
                    result
                });
                loop {
                    let wait = self
                        .heartbeat_due_in()
                        .expect("observed phase must have a heartbeat interval");
                    match completion.recv_timeout(wait) {
                        Ok(()) | Err(RecvTimeoutError::Disconnected) => {
                            return worker
                                .join()
                                .unwrap_or_else(|panic| std::panic::resume_unwind(panic));
                        }
                        Err(RecvTimeoutError::Timeout) => {
                            self.emit_heartbeat(phase, started.elapsed());
                        }
                    }
                }
            })
        };
        self.record_phase(phase, started.elapsed());
        result
    }

    pub(super) const fn begin_phase(&mut self, phase: WasmBuildPhase) {
        self.failure_phase = phase;
    }

    pub(super) fn record_phase(&mut self, phase: WasmBuildPhase, elapsed: Duration) {
        self.failure_phase = phase;
        let timings = &mut self.failure_timings;
        // Select the authoritative timing slot once. Optional slots distinguish
        // an unrun phase from one that completed with zero elapsed time.
        let timing = match phase {
            WasmBuildPhase::Specification => return,
            WasmBuildPhase::ExactCacheCoordination => &mut timings.exact_cache_coordination,
            WasmBuildPhase::CargoIdentity | WasmBuildPhase::RustcIdentity => {
                &mut timings.input_resolution.tool_identity
            }
            WasmBuildPhase::CargoMetadata => &mut timings.input_resolution.cargo_metadata,
            WasmBuildPhase::InputDiscovery => &mut timings.input_resolution.input_discovery,
            WasmBuildPhase::ContentHashing => &mut timings.input_resolution.content_hashing,
            WasmBuildPhase::SharedTargetCoordination => timings
                .shared_target_coordination
                .get_or_insert(Duration::ZERO),
            WasmBuildPhase::SharedTargetMaintenance => timings
                .shared_target_maintenance
                .get_or_insert(Duration::ZERO),
            WasmBuildPhase::CargoBuild => timings.cargo_build.get_or_insert(Duration::ZERO),
            WasmBuildPhase::ArtifactPublication => {
                timings.artifact_publication.get_or_insert(Duration::ZERO)
            }
            WasmBuildPhase::ExactCacheMaintenance => timings
                .exact_cache_maintenance
                .get_or_insert(Duration::ZERO),
            WasmBuildPhase::Cleanup => timings.cleanup.get_or_insert(Duration::ZERO),
        };
        *timing = timing.saturating_add(elapsed);
        if matches!(
            phase,
            WasmBuildPhase::CargoIdentity
                | WasmBuildPhase::RustcIdentity
                | WasmBuildPhase::CargoMetadata
                | WasmBuildPhase::InputDiscovery
                | WasmBuildPhase::ContentHashing
        ) {
            timings.input_resolution.total = timings.input_resolution.total.saturating_add(elapsed);
        }
    }

    pub(super) const fn failure_details(&self, total: Duration) -> WasmBuildFailureDetails {
        let phase = self.failure_phase;
        let mut timings = self.failure_timings;
        timings.total = total;
        WasmBuildFailureDetails { phase, timings }
    }
}
