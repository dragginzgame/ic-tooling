use std::{
    collections::VecDeque,
    num::NonZeroUsize,
    sync::{Condvar, Mutex, MutexGuard, OnceLock},
    time::{Duration, Instant},
};

pub(super) struct BoundedSlotPool<T, R> {
    capacity: NonZeroUsize,
    state: OnceLock<PoolState<T, R>>,
}

struct PoolState<T, R> {
    slots: Box<[Mutex<Slot<T, R>>]>,
    coordinator: Mutex<Coordinator>,
    slot_available: Condvar,
}

struct Coordinator {
    available: VecDeque<usize>,
    waiters: VecDeque<u64>,
    next_ticket: u64,
}

pub(super) enum Slot<T, R> {
    Empty,
    Reusable(T),
    Invalidated { value: T, reason: R },
    // A build can unwind before populating the slot. Retain that cause even
    // without a value so the next acquisition still reports an unwind rebuild.
    Unwound(Option<T>),
}

impl<T, R> Slot<T, R> {
    fn into_value(self) -> Option<T> {
        match self {
            Self::Empty => None,
            Self::Reusable(value) | Self::Invalidated { value, .. } => Some(value),
            Self::Unwound(value) => value,
        }
    }
}

pub(super) struct BoundedSlotLease<'a, T, R> {
    pool: &'a PoolState<T, R>,
    slot_index: usize,
    slot: Option<MutexGuard<'a, Slot<T, R>>>,
    wait: Duration,
}

struct WaitTicket<'a, T, R> {
    pool: &'a PoolState<T, R>,
    ticket: Option<u64>,
}

impl<T, R> BoundedSlotPool<T, R> {
    pub(super) const fn new(capacity: NonZeroUsize) -> Self {
        Self {
            capacity,
            state: OnceLock::new(),
        }
    }

    pub(super) fn acquire(&self) -> BoundedSlotLease<'_, T, R> {
        // First-use slot allocation is setup, not capacity-queue wait.
        let pool = self.state();
        let started = Instant::now();
        // Declare the ticket guard before taking the coordinator lock so an
        // unwind releases the lock before cancellation tries to acquire it.
        let mut ticket_guard = WaitTicket { pool, ticket: None };

        let slot_index = {
            let mut coordinator = pool
                .coordinator
                .lock()
                .unwrap_or_else(std::sync::PoisonError::into_inner);
            let ticket = coordinator.next_ticket;
            coordinator.next_ticket = coordinator.next_ticket.wrapping_add(1);
            coordinator.waiters.push_back(ticket);
            ticket_guard.ticket = Some(ticket);
            loop {
                if coordinator.waiters.front() == Some(&ticket)
                    && let Some(slot_index) = coordinator.available.pop_front()
                {
                    let removed = coordinator.waiters.pop_front();
                    debug_assert_eq!(removed, Some(ticket));
                    ticket_guard.ticket = None;
                    pool.slot_available.notify_all();
                    break slot_index;
                }

                coordinator = pool
                    .slot_available
                    .wait(coordinator)
                    .unwrap_or_else(std::sync::PoisonError::into_inner);
            }
        };

        let slot = pool.slots[slot_index]
            .lock()
            .unwrap_or_else(std::sync::PoisonError::into_inner);
        BoundedSlotLease {
            pool,
            slot_index,
            slot: Some(slot),
            wait: started.elapsed(),
        }
    }

    pub(super) const fn capacity(&self) -> NonZeroUsize {
        self.capacity
    }

    fn state(&self) -> &PoolState<T, R> {
        self.state.get_or_init(|| PoolState {
            slots: (0..self.capacity.get())
                .map(|_| Mutex::new(Slot::Empty))
                .collect(),
            coordinator: Mutex::new(Coordinator {
                available: (0..self.capacity.get()).collect(),
                waiters: VecDeque::new(),
                next_ticket: 0,
            }),
            slot_available: Condvar::new(),
        })
    }
}

impl<T, R> BoundedSlotLease<'_, T, R> {
    pub(super) const fn slot_index(&self) -> usize {
        self.slot_index
    }

    pub(super) const fn wait(&self) -> Duration {
        self.wait
    }

    pub(super) fn get(&self) -> Option<&T> {
        match self.state() {
            Slot::Empty => None,
            Slot::Reusable(value) | Slot::Invalidated { value, .. } => Some(value),
            Slot::Unwound(value) => value.as_ref(),
        }
    }

    pub(super) fn replace(&mut self, value: T) -> Option<T> {
        std::mem::replace(self.slot_mut(), Slot::Reusable(value)).into_value()
    }

    pub(super) fn take(&mut self) -> Option<T> {
        std::mem::replace(self.slot_mut(), Slot::Empty).into_value()
    }

    pub(super) fn invalidate(&mut self, reason: R) {
        let slot = self.slot_mut();
        let state = std::mem::replace(slot, Slot::Empty);
        *slot = match state {
            Slot::Reusable(value) | Slot::Invalidated { value, .. } => {
                Slot::Invalidated { value, reason }
            }
            other => other,
        };
    }

    pub(super) fn state(&self) -> &Slot<T, R> {
        self.slot
            .as_deref()
            .expect("bounded slot lease must retain its slot")
    }

    fn slot_mut(&mut self) -> &mut Slot<T, R> {
        self.slot
            .as_deref_mut()
            .expect("bounded slot lease must retain its slot")
    }
}

impl<T, R> Drop for BoundedSlotLease<'_, T, R> {
    fn drop(&mut self) {
        if std::thread::panicking() {
            let value = self.take();
            *self.slot_mut() = Slot::Unwound(value);
        }
        drop(self.slot.take());

        let mut coordinator = self
            .pool
            .coordinator
            .lock()
            .unwrap_or_else(std::sync::PoisonError::into_inner);
        coordinator.available.push_back(self.slot_index);
        drop(coordinator);
        self.pool.slot_available.notify_all();
    }
}

impl<T, R> Drop for WaitTicket<'_, T, R> {
    fn drop(&mut self) {
        let Some(ticket) = self.ticket else {
            return;
        };
        let mut coordinator = self
            .pool
            .coordinator
            .lock()
            .unwrap_or_else(std::sync::PoisonError::into_inner);
        if let Some(position) = coordinator
            .waiters
            .iter()
            .position(|queued_ticket| *queued_ticket == ticket)
        {
            coordinator.waiters.remove(position);
            drop(coordinator);
            self.pool.slot_available.notify_all();
        }
    }
}

#[cfg(test)]
mod tests;
