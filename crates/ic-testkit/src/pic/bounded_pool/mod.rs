use std::{
    collections::VecDeque,
    num::NonZeroUsize,
    sync::{Mutex, OnceLock},
    time::{Duration, Instant},
};
use tokio::sync::{RwLock, RwLockReadGuard, Semaphore, SemaphorePermit};

type AvailableSlots<T, R> = Mutex<VecDeque<(usize, Slot<T, R>)>>;

pub(super) struct BoundedSlotPool<T, R> {
    capacity: NonZeroUsize,
    state: OnceLock<AvailableSlots<T, R>>,
    permits: Semaphore,
    lifecycle: RwLock<()>,
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
    _lifecycle: RwLockReadGuard<'a, ()>,
}

impl<T, R> BoundedSlotPool<T, R> {
    pub(super) const fn new(capacity: NonZeroUsize) -> Self {
        Self {
            capacity,
            state: OnceLock::new(),
            permits: Semaphore::const_new(capacity.get()),
            lifecycle: RwLock::const_new(()),
        }
    }

    pub(super) async fn acquire(&self) -> Option<BoundedSlotLease<'_, T, R>> {
        if self.permits.is_closed() {
            return None;
        }
        let lifecycle = self.lifecycle.read().await;
        let started = Instant::now();
        // Tokio owns FIFO admission, cancellation and waking closed waiters.
        let permit = self.permits.acquire().await.ok()?;
        if self.permits.is_closed() {
            return None;
        }
        let (slot_index, slot) = self
            .state()
            .lock()
            .unwrap_or_else(std::sync::PoisonError::into_inner)
            .pop_front()
            .expect("each admitted permit owns one available slot");
        Some(BoundedSlotLease {
            pool: self,
            slot_index,
            slot,
            _permit: permit,
            wait: started.elapsed(),
            preparing: false,
            _lifecycle: lifecycle,
        })
    }

    pub(super) const fn capacity(&self) -> NonZeroUsize {
        self.capacity
    }

    fn state(&self) -> &Mutex<VecDeque<(usize, Slot<T, R>)>> {
        self.state.get_or_init(|| {
            Mutex::new((0..self.capacity.get()).map(|i| (i, Slot::Empty)).collect())
        })
    }

    pub(super) async fn shutdown(&self, mut delete: impl AsyncFnMut(T)) {
        self.permits.close();
        let _lifecycle = self.lifecycle.write().await;
        while let Some(value) = self.take_value() {
            delete(value).await;
        }
    }

    // Exclusive shutdown owns admission. Taking one value at a time retains
    // unattempted values if deletion is cancelled; no cleanup occurs in Drop.
    fn take_value(&self) -> Option<T> {
        let mut state = self
            .state
            .get()?
            .lock()
            .unwrap_or_else(std::sync::PoisonError::into_inner);
        while let Some((_, slot)) = state.pop_front() {
            if let Some(value) = slot.into_value() {
                return Some(value);
            }
        }
        None
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
