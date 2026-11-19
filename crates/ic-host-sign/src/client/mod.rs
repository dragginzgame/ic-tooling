//! One cancellable, bounded signer invocation with retained cleanup custody.

#[cfg(test)]
mod tests;

use std::{
    future::poll_fn,
    path::{Path, PathBuf},
    process::Command,
    sync::{
        Arc, Mutex,
        atomic::{AtomicBool, Ordering},
    },
    task::{Poll, Waker},
    thread::JoinHandle,
};

use ic_host_process::{Input, Ownership, Policy, Stderr, Stream};
use ic_platform::{
    ingress::{Content, decode_envelope},
    native::{PreparedRequestStatus, PreparedUpdate},
};

use crate::{
    Error,
    protocol::{self, Configuration, Operation, Request, Response, Signed, WireRequest},
};

/// Observable local custody state; it never implies a signing retry is authorized.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum State {
    /// No worker is owned; an explicit invocation can be admitted.
    Ready,
    /// One worker owns admission, execution and cleanup.
    Running,
    /// Cancellation requested; cleanup/completion is still owned, not assumed done.
    Cancelling,
    /// Worker completed; its original result still needs explicit settlement.
    AwaitingSettlement,
    /// Cleanup evidence is uncertain. This signer refuses further invocations.
    Uncertain,
}

struct Completion {
    result: Option<Result<Signed, Error>>,
    waker: Option<Waker>,
}
struct Job {
    cancelled: AtomicBool,
    completion: Mutex<Completion>,
}
struct Active {
    shared: Arc<Job>,
    worker: JoinHandle<()>,
}

/// Selected offline keyring signer. Not cloneable or reconstructible over live work.
///
/// Its approved binary and public configuration are committed by SHA-256. Treat
/// the selected paths as cooperatively owned and stable; this is not a hostile
/// same-user executable sandbox. Keep one owner per approved signing context.
/// `sign` future drop requests cancellation; `settle` observes the actual result
/// and cleanup. Another invocation refuses until settlement. Dropping this owner
/// cancels and joins its worker, which may block in the existing native cleanup
/// contract; no detached signing worker or background reaper is introduced.
pub struct Signer {
    executable: PathBuf,
    tool: [u8; 32],
    config: Configuration,
    commitment: [u8; 32],
    active: Option<Active>,
    uncertain: bool,
}

impl Signer {
    /// Admit a source/receipt-qualified helper and the existing public identity.
    /// Caller supplies the expected executable hash from its trusted installation,
    /// not an arbitrary program whose version string merely looks appropriate.
    /// This reads public files but never accesses private keyring material.
    /// # Errors
    /// Refuses changed/insecure tools or unsupported identity/backend metadata.
    pub fn new(
        executable: &Path,
        expected_tool: [u8; 32],
        config: Configuration,
    ) -> Result<Self, Error> {
        config.admit()?;
        if protocol::tool_hash(executable)? != expected_tool {
            return Err(Error::Configuration);
        }
        let commitment = protocol::commitment(&config, expected_tool)?;

        Ok(Self {
            executable: executable.to_owned(),
            tool: expected_tool,
            config,
            commitment,
            active: None,
            uncertain: false,
        })
    }

    /// Stable original approval commitment to protocol, identity, backend and tool.
    #[must_use]
    pub const fn commitment(&self) -> [u8; 32] {
        self.commitment
    }

    /// Inspect actual local custody without waiting or assuming cleanup completed.
    #[must_use]
    pub fn state(&self) -> State {
        if self.uncertain {
            return State::Uncertain;
        }
        let Some(active) = &self.active else {
            return State::Ready;
        };
        let completion = active
            .shared
            .completion
            .lock()
            .unwrap_or_else(std::sync::PoisonError::into_inner);

        if completion.result.is_some() {
            State::AwaitingSettlement
        } else if active.shared.cancelled.load(Ordering::Relaxed) {
            State::Cancelling
        } else {
            State::Running
        }
    }

    /// Execute exactly one offline signing exchange, retaining cleanup custody.
    ///
    /// No work starts before the future is first polled. Dropping it requests
    /// cancellation; it does not prove the key was unused or cleanup completed.
    /// Call `settle` before any subsequent invocation to recover the original
    /// result, including signed bytes completed before cancellation was observed.
    /// No re-sign, submission, polling or operation journal is performed.
    /// # Errors
    /// Refuses changed approval/configuration, concurrent work, deadline, provider
    /// failure or output mismatch. Cleanup failures remain explicit and poison reuse.
    pub async fn sign(
        &mut self,
        request: Request,
        approved_commitment: [u8; 32],
    ) -> Result<Signed, Error> {
        if self.uncertain {
            return Err(Error::Uncertain);
        }
        if self.active.is_some() {
            return Err(Error::Busy);
        }
        if approved_commitment != self.commitment {
            return Err(Error::Configuration);
        }
        request.admit(&self.config)?;
        let shared = Arc::new(Job {
            cancelled: AtomicBool::new(false),
            completion: Mutex::new(Completion {
                result: None,
                waker: None,
            }),
        });
        let worker_job = Arc::clone(&shared);
        let executable = self.executable.clone();
        let wire = WireRequest {
            protocol: protocol::PROTOCOL.to_owned(),
            tool_sha256: self.tool,
            commitment: self.commitment,
            configuration: self.config.clone(),
            request,
        };
        let worker = std::thread::Builder::new()
            .name("ic-sign-custody".into())
            .spawn(move || {
                let result = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
                    execute(&executable, &wire, &worker_job.cancelled)
                }))
                .unwrap_or(Err(Error::Uncertain));
                let waker = {
                    let mut state = worker_job
                        .completion
                        .lock()
                        .unwrap_or_else(std::sync::PoisonError::into_inner);
                    state.result = Some(result);

                    state.waker.take()
                };

                if let Some(waker) = waker {
                    waker.wake();
                }
            })?;
        self.active = Some(Active {
            shared: Arc::clone(&shared),
            worker,
        });
        // This guard is held across the suspension point in this borrowed future.
        // The Signer retains the worker and prevents another invocation after drop.
        let mut cancel = CancelOnDrop {
            shared,
            armed: true,
        };
        let result = self.settle().await;
        cancel.armed = false;

        result
    }

    /// Wait for the original owned worker and return its original result.
    /// This never starts signing or retries anything. Dropping this future leaves
    /// custody with this signer and can be followed by another `settle` call.
    /// # Errors
    /// Reports no active work, original execution/output failure, or uncertain
    /// cleanup. A dropped signing observer can still have a successful original result.
    pub async fn settle(&mut self) -> Result<Signed, Error> {
        let shared = Arc::clone(&self.active.as_ref().ok_or(Error::Idle)?.shared);
        let result = poll_fn(|context| {
            let mut state = shared
                .completion
                .lock()
                .unwrap_or_else(std::sync::PoisonError::into_inner);

            if let Some(result) = state.result.take() {
                Poll::Ready(result)
            } else {
                state.waker = Some(context.waker().clone());

                Poll::Pending
            }
        })
        .await;
        let active = self.active.take().ok_or(Error::Uncertain)?;
        // Native child cleanup has completed before result publication. Joining
        // here retains thread custody through its final return, not process I/O.
        if active.worker.join().is_err() {
            self.uncertain = true;

            return Err(Error::Uncertain);
        }
        self.uncertain = matches!(&result, Err(Error::Uncertain))
            || matches!(&result, Err(Error::Execution { report }) if !report.cleanup.is_empty());

        result
    }
}

impl Drop for Signer {
    fn drop(&mut self) {
        if let Some(active) = self.active.take() {
            active.shared.cancelled.store(true, Ordering::Relaxed);
            let _ = active.worker.join();
        }
    }
}

struct CancelOnDrop {
    shared: Arc<Job>,
    armed: bool,
}
impl Drop for CancelOnDrop {
    fn drop(&mut self) {
        if self.armed {
            self.shared.cancelled.store(true, Ordering::Relaxed);
        }
    }
}

fn execute(executable: &Path, wire: &WireRequest, cancelled: &AtomicBool) -> Result<Signed, Error> {
    if cancelled.load(Ordering::Relaxed) {
        return Err(Error::Cancelled);
    }
    wire.configuration.admit()?;
    if protocol::tool_hash(executable)? != wire.tool_sha256 {
        return Err(Error::Configuration);
    }
    let input = protocol::encode_request(wire)?;
    let elapsed = wire.request.admit(&wire.configuration)?;
    if cancelled.load(Ordering::Relaxed) {
        return Err(Error::Cancelled);
    }
    let mut command = Command::new(executable);
    command.env_clear();
    if let Some(address) = &wire.configuration.local_bus {
        command.env("DBUS_SESSION_BUS_ADDRESS", address);
    }
    let mut output = Vec::new();
    let report = ic_host_process::run(
        &mut command,
        Policy {
            ownership: Ownership::Group,
            input: Input::Bytes(input),
            stdout_bytes: protocol::FRAME_BYTES,
            stderr: Stderr::Capture(65536),
            elapsed,
        },
        cancelled,
        |stream, bytes| {
            if stream == Stream::Stdout {
                output.extend_from_slice(bytes);
            }

            Ok(())
        },
    );
    if report.failure.is_some()
        || !report.cleanup.is_empty()
        || !report.status.is_some_and(|status| status.success())
    {
        return Err(Error::Execution { report });
    }
    // Completion includes output admission, not merely process exit. A provider
    // that finishes after the original deadline cannot return a success here.
    if protocol::now_ns()? >= wire.request.deadline_ns {
        return Err(Error::Deadline);
    }
    if protocol::tool_hash(executable)? != wire.tool_sha256 {
        return Err(Error::Configuration);
    }
    let response: Response = serde_json::from_slice(&output).map_err(|_| Error::Output)?;
    if response.protocol != protocol::PROTOCOL || response.commitment != wire.commitment {
        return Err(Error::Output);
    }
    let signed = admit(&wire.request, response.envelope)?;
    wire.configuration.admit()?;
    if protocol::now_ns()? >= wire.request.deadline_ns {
        return Err(Error::Deadline);
    }

    Ok(signed)
}

fn admit(request: &Request, envelope: Vec<u8>) -> Result<Signed, Error> {
    let effective = protocol::principal(&request.effective)?;
    let expected_sender = protocol::principal(&request.sender)?;

    match &request.operation {
        Operation::Update {
            recipient,
            method,
            arg,
            nonce,
        } => {
            let prepared = PreparedUpdate::from_bytes(&envelope, effective, request.envelope_bytes)
                .map_err(|_| Error::Output)?;
            let decoded =
                decode_envelope(&envelope, request.envelope_bytes).map_err(|_| Error::Output)?;
            let Content::Call {
                canister_id,
                method_name,
                arg: actual,
                nonce: actual_nonce,
                ingress_expiry,
                sender,
                sender_info,
            } = decoded.value.content
            else {
                return Err(Error::Output);
            };
            if canister_id != protocol::principal(recipient)?
                || method_name != *method
                || actual != *arg
                || actual_nonce != *nonce
                || ingress_expiry != request.expires_ns
                || sender != expected_sender
                || sender_info.is_some()
            {
                return Err(Error::Output);
            }

            Ok(Signed {
                envelope,
                request_id: *prepared.request_id(),
                sender: expected_sender,
                effective,
                expiry: request.expires_ns,
            })
        }
        Operation::Status { request_id } => {
            let prepared = PreparedRequestStatus::from_signed_bytes(
                &envelope,
                *request_id,
                expected_sender,
                effective,
                request.envelope_bytes,
            )
            .map_err(|_| Error::Output)?;
            if prepared.expiry() != request.expires_ns {
                return Err(Error::Output);
            }

            Ok(Signed {
                envelope,
                request_id: *request_id,
                sender: expected_sender,
                effective,
                expiry: request.expires_ns,
            })
        }
    }
}
