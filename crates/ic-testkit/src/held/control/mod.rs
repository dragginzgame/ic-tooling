//! Exclusive release authority and explicitly bounded manual clock progression.

use std::collections::TryReserveError;
use std::future::Future;
use std::sync::Arc;
use std::sync::atomic::{AtomicU64, Ordering};
use std::time::Duration;

use ic_platform_types::Principal;
use pocket_ic::Time;
use pocket_ic::common::rest::{
    CanisterHttpReply, CanisterHttpRequest, CanisterHttpResponse, MockCanisterHttpResponse,
};
use pocket_ic::nonblocking::PocketIc;

use super::{Catalog, CompileError};

static NEXT_FIXTURE: AtomicU64 = AtomicU64::new(0);

/// Local admission failure, distinct from an upstream panic or cancelled dispatch.
#[derive(Debug, thiserror::Error)]
pub enum ControlError {
    /// The supplied client selects a different server or instance.
    #[error("fixture belongs to a different server or instance")]
    WrongClient,
    /// The simulator is advancing automatically.
    #[error("held replies require manual progress")]
    AutomaticProgress,
    /// A clock step must be nonzero and at most [`Fixture::MAX_CLOCK_STEP`].
    #[error("clock step must be nonzero and at most 60 seconds")]
    ClockStep,
    /// The requested step and its processing round cannot fit PocketIC's clock.
    #[error("clock step exceeds simulator time representation")]
    ClockOverflow,
    /// A dispatched clock transition did not complete with an admitted observation.
    #[error("fixture clock transition is uncertain; further control is refused")]
    ClockUncertain,
    /// A token was produced by another controller.
    #[error("request token belongs to another fixture")]
    ForeignToken,
    /// Release was already attempted, possibly with an uncertain remote outcome.
    #[error("release authority already consumed")]
    Consumed,
    /// The exact request is no longer in the pending inventory.
    #[error("request is no longer pending")]
    NotPending,
    /// The process-local fixture namespace cannot admit another identity.
    #[error("fixture identity space exhausted")]
    IdentityExhausted,
    /// Module generation refused.
    #[error(transparent)]
    Compilation(#[from] CompileError),
    /// Local state reservation failed.
    #[error("fixture allocation failed: {0}")]
    Allocation(#[from] TryReserveError),
}

#[derive(Debug)]
struct Binding {
    server: String,
    instance: usize,
    prefix: String,
}

/// Opaque observation of one pending request. Only its catalog slot is public.
///
/// A token grants no independent release authority; its originating controller
/// must verify it again against the simulator before dispatch.
#[derive(Debug)]
pub struct Request {
    binding: Arc<Binding>,
    slot: usize,
    request: CanisterHttpRequest,
}

impl Request {
    /// Original catalog position, including duplicate method slots.
    #[must_use]
    pub fn slot(&self) -> usize {
        self.slot
    }
}

/// A generated module and its exclusive, non-cloneable release authority.
///
/// The caller installs these bytes once into the bound target and owns funding,
/// ticks and teardown. Do not upgrade, reinstall, restore or reconstruct control
/// over that target while calls are live. Serialize all instance operations across
/// clients/controllers; exclude automatic progress and raw clock changes. Use
/// [`Self::advance_time`] for clock transitions while replies are held.
/// Dropping this value performs no remote cleanup.
#[derive(Debug)]
pub struct Fixture {
    binding: Arc<Binding>,
    wasm: Vec<u8>,
    consumed: Vec<bool>,
    clock_uncertain: bool,
}

impl Fixture {
    /// Bind an admitted catalog to a manual-progress client and target.
    ///
    /// This checks progress and compiles bytes, but never installs, funds or ticks.
    /// Each construction has a distinct process-local wire namespace. Resuming
    /// controllers across process restarts is outside this contract.
    ///
    /// # Errors
    /// Returns progress, identity, allocation or module-generation refusal.
    /// Upstream transport failures retain PocketIC's panic behavior.
    pub async fn new(
        pic: &PocketIc,
        target: Principal,
        catalog: &Catalog,
        call_cycles: u128,
        module_bytes: usize,
    ) -> Result<Self, ControlError> {
        manual(pic).await?;
        let identity = NEXT_FIXTURE
            .try_update(Ordering::Relaxed, Ordering::Relaxed, |next| {
                next.checked_add(1)
            })
            .map_err(|_| ControlError::IdentityExhausted)?;
        let prefix = format!("https://ic-testkit.invalid/held/{target}/{identity}/");
        let wasm = catalog.compile_prefix(&prefix, call_cycles, module_bytes)?;
        let mut consumed = Vec::new();
        consumed.try_reserve_exact(catalog.len())?;
        consumed.resize(catalog.len(), false);
        Ok(Self {
            binding: Arc::new(Binding {
                server: pic.get_server_url().to_string(),
                instance: pic.instance_id,
                prefix,
            }),
            wasm,
            consumed,
            clock_uncertain: false,
        })
    }

    /// Exact generated bytes to install once into the bound target.
    #[must_use]
    pub fn wasm(&self) -> &[u8] {
        &self.wasm
    }

    /// Maximum explicitly requested advance in one operation (60 seconds).
    /// The single expiry-processing round adds at most one nanosecond.
    pub const MAX_CLOCK_STEP: Duration = Duration::from_secs(60);

    /// Advance the bound manual instance's clock, then execute one processing round.
    ///
    /// A nonzero step of at most [`Self::MAX_CLOCK_STEP`] is admitted with checked
    /// nanosecond arithmetic. The returned original PocketIC time includes the
    /// round's zero/one-nanosecond increment. The round can execute other instance
    /// work, including HTTP timeout rejections; it never mocks a successful reply.
    /// A still-pending token keeps its original identity and release authority.
    /// Expired/disappeared requests remain subject to [`Self::release`]'s inventory
    /// check. No deadline is extended, and this operation consumes no reply slot.
    ///
    /// Keep all operations on this instance serialized, including other clients,
    /// controllers and caller-owned ticks. After clock dispatch begins, cancellation
    /// or a transport panic leaves this controller unusable: subsequent observation,
    /// release and clock changes refuse. The caller still owns instance teardown.
    ///
    /// # Errors
    /// Refuses foreign clients, automatic progress, invalid/overflowing steps and
    /// uncertain clock state. Upstream transport failures retain panic behavior.
    pub async fn advance_time(
        &mut self,
        pic: &PocketIc,
        duration: Duration,
    ) -> Result<Time, ControlError> {
        self.admit(pic).await?;
        let step = clock_step(duration)?;
        let now = pic.get_time().await.as_nanos_since_unix_epoch();
        let target = clock_target(now, step)?;
        self.progress(async {
            pic.set_time(Time::from_nanos_since_unix_epoch(target))
                .await;
            pic.tick().await;
            let observed = pic.get_time().await;
            if !(target..=target + 1).contains(&observed.as_nanos_since_unix_epoch()) {
                return Err(ControlError::ClockUncertain);
            }
            Ok(observed)
        })
        .await
    }

    /// Observe this fixture's unconsumed slots without ticking or advancing time.
    ///
    /// Upstream inventory allocation occurs before filtering and is not bounded
    /// by catalog limits. Returned tokens are bounded by admitted slot count.
    ///
    /// # Errors
    /// Refuses a foreign client, automatic/uncertain progress or reservation failure.
    pub async fn pending(&self, pic: &PocketIc) -> Result<Vec<Request>, ControlError> {
        self.admit(pic).await?;
        let mut tokens = Vec::new();
        tokens.try_reserve_exact(self.consumed.len())?;
        for request in pic.get_canister_http().await {
            let Some(slot) = self.slot(&request) else {
                continue;
            };
            if !tokens.iter().any(|token: &Request| token.slot == slot) {
                tokens.push(Request {
                    binding: Arc::clone(&self.binding),
                    slot,
                    request,
                });
            }
        }
        Ok(tokens)
    }

    /// Open one barrier after checking client, ownership and exact pending identity.
    ///
    /// Local authority is consumed before awaiting simulator dispatch. Cancellation,
    /// transport panic or uncertain completion after that point cannot be retried.
    /// This method does not tick; the original callback returns its immutable bytes.
    ///
    /// # Errors
    /// Refuses foreign clients/tokens, automatic/uncertain progress, consumed
    /// authority or a request absent from the inventory. Upstream failures may panic.
    pub async fn release(&mut self, pic: &PocketIc, token: &Request) -> Result<(), ControlError> {
        self.admit(pic).await?;
        if !Arc::ptr_eq(&self.binding, &token.binding) {
            return Err(ControlError::ForeignToken);
        }
        if self.consumed[token.slot] {
            return Err(ControlError::Consumed);
        }
        if !pic
            .get_canister_http()
            .await
            .iter()
            .any(|request| request == &token.request)
        {
            return Err(ControlError::NotPending);
        }
        let response = MockCanisterHttpResponse {
            subnet_id: token.request.subnet_id,
            request_id: token.request.request_id,
            response: CanisterHttpResponse::CanisterHttpReply(CanisterHttpReply {
                status: 200,
                headers: vec![],
                body: vec![],
            }),
            additional_responses: vec![],
        };
        self.dispatch(token.slot, pic.mock_canister_http_response(response))
            .await;
        Ok(())
    }

    async fn admit(&self, pic: &PocketIc) -> Result<(), ControlError> {
        if self.binding.server != pic.get_server_url().as_str()
            || self.binding.instance != pic.instance_id
        {
            return Err(ControlError::WrongClient);
        }
        if self.clock_uncertain {
            return Err(ControlError::ClockUncertain);
        }
        manual(pic).await
    }

    fn slot(&self, request: &CanisterHttpRequest) -> Option<usize> {
        let suffix = request.url.strip_prefix(&self.binding.prefix)?;
        let slot = suffix.parse::<usize>().ok()?;
        if *self.consumed.get(slot)?
            || suffix != slot.to_string()
            || request.body != (slot as u64).to_le_bytes()
        {
            return None;
        }
        Some(slot)
    }

    async fn dispatch(&mut self, slot: usize, operation: impl Future<Output = ()>) {
        self.consumed[slot] = true;
        operation.await;
    }

    async fn progress(
        &mut self,
        operation: impl Future<Output = Result<Time, ControlError>>,
    ) -> Result<Time, ControlError> {
        // Consume clock authority before polling any remote mutation. A dropped
        // future, panic or divergent observation must not leave release enabled.
        self.clock_uncertain = true;
        let observed = operation.await?;
        self.clock_uncertain = false;
        Ok(observed)
    }
}

fn clock_step(duration: Duration) -> Result<u64, ControlError> {
    if duration.is_zero() || duration > Fixture::MAX_CLOCK_STEP {
        return Err(ControlError::ClockStep);
    }
    // The explicit 60-second bound fits u64 nanoseconds.
    Ok(duration.as_nanos() as u64)
}

fn clock_target(now: u64, step: u64) -> Result<u64, ControlError> {
    now.checked_add(step)
        .filter(|target| *target < u64::MAX)
        .ok_or(ControlError::ClockOverflow)
}

async fn manual(pic: &PocketIc) -> Result<(), ControlError> {
    if pic.auto_progress_enabled().await {
        Err(ControlError::AutomaticProgress)
    } else {
        Ok(())
    }
}

#[cfg(test)]
mod clock_tests;
#[cfg(test)]
mod tests;
