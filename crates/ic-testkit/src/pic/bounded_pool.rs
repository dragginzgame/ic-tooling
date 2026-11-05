use std::{
    collections::VecDeque,
    num::NonZeroUsize,
    sync::{Mutex, OnceLock},
    time::{Duration, Instant},
};
use tokio::sync::{Semaphore, SemaphorePermit};

type AvailableSlots<T, R> = Mutex<VecDeque<(usize, Slot<T, R>)>>;

pub(super) struct BoundedSlotPool<T, R> {
    capacity: NonZeroUsize,
    state: OnceLock<AvailableSlots<T, R>>,
    permits: Semaphore,
}

pub(super) enum Slot<T, R> {
    Empty,
    Reusable(T),
    Invalidated { value: T, reason: R },
    Unwound(Option<T>),
    Interrupted(Option<T>),
}

impl<T, R> Slot<T, R> {
    fn into_value(self) -> Option<T> {
        match self {
            Self::Empty => None,
            Self::Reusable(value) | Self::Invalidated { value, .. } => Some(value),
            Self::Unwound(value) | Self::Interrupted(value) => value,
        }
    }
}

pub(super) struct BoundedSlotLease<'a, T, R> {
    pool: &'a BoundedSlotPool<T, R>,
    slot_index: usize,
    slot: Slot<T, R>,
    _permit: SemaphorePermit<'a>,
    wait: Duration,
    preparing: bool,
}

impl<T, R> BoundedSlotPool<T, R> {
    pub(super) const fn new(capacity: NonZeroUsize) -> Self {
        Self {
            capacity,
            state: OnceLock::new(),
            permits: Semaphore::const_new(capacity.get()),
        }
    }

    pub(super) async fn acquire(&self) -> BoundedSlotLease<'_, T, R> {
        let state = self.state();
        let started = Instant::now();
        // Tokio owns FIFO admission and cancellation of queued acquisitions.
        let permit = self.permits.acquire().await.expect("pool is never closed");
        let (slot_index, slot) = state
            .lock()
            .unwrap_or_else(std::sync::PoisonError::into_inner)
            .pop_front()
            .expect("each admitted permit owns one available slot");
        BoundedSlotLease {
            pool: self,
            slot_index,
            slot,
            _permit: permit,
            wait: started.elapsed(),
            preparing: false,
        }
    }

    pub(super) const fn capacity(&self) -> NonZeroUsize {
        self.capacity
    }

    fn state(&self) -> &Mutex<VecDeque<(usize, Slot<T, R>)>> {
        self.state.get_or_init(|| {
            Mutex::new((0..self.capacity.get()).map(|i| (i, Slot::Empty)).collect())
        })
    }

    pub(super) fn into_values(self) -> impl Iterator<Item = T> {
        self.state
            .into_inner()
            .into_iter()
            .flat_map(|state| {
                state
                    .into_inner()
                    .unwrap_or_else(std::sync::PoisonError::into_inner)
            })
            .filter_map(|(_, slot)| slot.into_value())
    }
}

impl<T, R> BoundedSlotLease<'_, T, R> {
    pub(super) const fn slot_index(&self) -> usize {
        self.slot_index
    }
    pub(super) const fn wait(&self) -> Duration {
        self.wait
    }
    pub(super) const fn get(&self) -> Option<&T> {
        match &self.slot {
            Slot::Empty => None,
            Slot::Reusable(value) | Slot::Invalidated { value, .. } => Some(value),
            Slot::Unwound(value) | Slot::Interrupted(value) => value.as_ref(),
        }
    }
    pub(super) fn replace(&mut self, value: T) -> Option<T> {
        std::mem::replace(&mut self.slot, Slot::Reusable(value)).into_value()
    }
    pub(super) fn take(&mut self) -> Option<T> {
        std::mem::replace(&mut self.slot, Slot::Empty).into_value()
    }
    pub(super) fn invalidate(&mut self, reason: R) {
        self.slot = match std::mem::replace(&mut self.slot, Slot::Empty) {
            Slot::Reusable(value) | Slot::Invalidated { value, .. } => {
                Slot::Invalidated { value, reason }
            }
            other => other,
        };
    }
    pub(super) const fn state(&self) -> &Slot<T, R> {
        &self.slot
    }
    pub(super) const fn begin_preparation(&mut self) {
        self.preparing = true;
    }
    pub(super) const fn finish_preparation(&mut self) {
        self.preparing = false;
    }
}

impl<T, R> Drop for BoundedSlotLease<'_, T, R> {
    fn drop(&mut self) {
        if self.preparing || std::thread::panicking() {
            let value = self.take();
            self.slot = if std::thread::panicking() {
                Slot::Unwound(value)
            } else {
                Slot::Interrupted(value)
            };
        }
        let slot = std::mem::replace(&mut self.slot, Slot::Empty);
        self.pool
            .state()
            .lock()
            .unwrap_or_else(std::sync::PoisonError::into_inner)
            .push_back((self.slot_index, slot));
        // Return the slot before releasing its semaphore permit.
    }
}

#[cfg(test)]
mod tests;
