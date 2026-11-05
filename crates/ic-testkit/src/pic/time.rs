use crate::pocket_ic::ClientError;
use crate::pocket_ic::PocketIc;
use std::{task::Poll, time::Duration};

/// A caller-owned readiness check failed or exhausted its simulated round budget.
#[derive(Debug, thiserror::Error)]
pub enum TickUntilError<E> {
    /// A simulated progression operation failed; no remaining ticks are dispatched.
    #[error("PocketIC progression failed: {0}")]
    Progress(#[source] ClientError),
    /// The predicate was still pending after this many advance/tick rounds.
    #[error("readiness remained pending after {rounds} rounds")]
    ProgressLimit {
        /// Completed simulated advance/tick rounds.
        rounds: u32,
    },
    /// The predicate returned its own failure.
    #[error("readiness check failed: {0}")]
    Failed(#[source] E),
}

/// Advance simulated time once, then deliver exactly the caller-selected ticks.
///
/// Zero duration performs no time read or write; zero ticks permits advancement
/// without delivering a round. This function does not poll, retry, start a server
/// or decide application readiness. Callers own completion and cadence budgets.
///
/// # Errors
/// Returns the first client failure without dispatching remaining ticks.
pub async fn advance_and_tick(
    pic: &PocketIc,
    advance: Duration,
    ticks: u32,
) -> Result<(), ClientError> {
    if !advance.is_zero() {
        pic.advance_time(advance).await?;
    }
    for _ in 0..ticks {
        pic.tick().await?;
    }
    Ok(())
}

/// Check a caller-owned predicate, advancing simulated time and ticking while pending.
///
/// The predicate runs once before any mutation. Each pending result permits one
/// round of `advance_time(advance)`, then `tick()`, then another predicate check,
/// up to `max_rounds`. A zero budget still performs the initial check. A zero
/// duration permits ticks without reading or writing simulated time. Completion
/// on the last allowed round succeeds; predicate failures return immediately without another round.
///
/// This bounds simulated progression, not wall-clock time spent inside `PocketIC`
/// calls or the predicate. Client failures return immediately as `Progress`.
/// Applications own readiness meaning and predicate error types.
///
/// # Errors
/// Returns the predicate error or `ProgressLimit` when all permitted rounds remain pending.
pub async fn tick_until<T, E>(
    pic: &PocketIc,
    max_rounds: u32,
    advance: Duration,
    mut poll: impl AsyncFnMut(&PocketIc) -> Poll<Result<T, E>>,
) -> Result<T, TickUntilError<E>> {
    poll_with_rounds(
        max_rounds,
        async || advance_and_tick(pic, advance, 1).await,
        async || poll(pic).await,
    )
    .await
}

async fn poll_with_rounds<T, E>(
    max_rounds: u32,
    mut round: impl AsyncFnMut() -> Result<(), ClientError>,
    mut poll: impl AsyncFnMut() -> Poll<Result<T, E>>,
) -> Result<T, TickUntilError<E>> {
    let mut rounds = 0;
    loop {
        match poll().await {
            Poll::Ready(result) => return result.map_err(TickUntilError::Failed),
            Poll::Pending if rounds == max_rounds => {
                return Err(TickUntilError::ProgressLimit { rounds });
            }
            Poll::Pending => {
                round().await.map_err(TickUntilError::Progress)?;
                rounds += 1;
            }
        }
    }
}

#[cfg(test)]
mod tests;
