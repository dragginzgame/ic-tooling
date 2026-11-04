use super::{BoundedSlotPool, Slot, WaitTicket};
use std::{
    num::NonZeroUsize,
    panic::{AssertUnwindSafe, catch_unwind},
    sync::{Arc, Barrier, mpsc},
    thread,
    time::{Duration, Instant},
};

const TIMEOUT: Duration = Duration::from_secs(5);

#[test]
#[expect(
    clippy::significant_drop_tightening,
    reason = "lease and coordinator scope deliberately cover the complete operation"
)]
fn concurrent_first_acquisitions_share_one_lazy_capacity() {
    let capacity = NonZeroUsize::new(4).unwrap();
    let pool = Arc::new(BoundedSlotPool::<usize, ()>::new(capacity));
    assert_eq!(pool.capacity(), capacity);
    assert!(pool.state.get().is_none());

    let barrier = Arc::new(Barrier::new(capacity.get()));
    let (acquired_tx, acquired_rx) = mpsc::channel();
    let workers = (0..capacity.get())
        .map(|_| {
            let pool = Arc::clone(&pool);
            let barrier = Arc::clone(&barrier);
            let acquired_tx = acquired_tx.clone();
            thread::spawn(move || {
                barrier.wait();
                let lease = pool.acquire();
                acquired_tx.send(lease.slot_index()).unwrap();
                barrier.wait();
            })
        })
        .collect::<Vec<_>>();
    drop(acquired_tx);
    let mut slots = (0..capacity.get())
        .map(|_| acquired_rx.recv_timeout(TIMEOUT).unwrap())
        .collect::<Vec<_>>();
    slots.sort_unstable();
    assert_eq!(slots, [0, 1, 2, 3]);
    for worker in workers {
        worker.join().expect("first acquisition should not panic");
    }
}

#[test]
fn capacity_allows_independent_leases() {
    let pool = BoundedSlotPool::<usize, ()>::new(NonZeroUsize::new(2).unwrap());
    let first = pool.acquire();
    let second = pool.acquire();

    assert_ne!(first.slot_index(), second.slot_index());
}

#[test]
fn exhausted_capacity_waits_until_release() {
    let pool = Arc::new(BoundedSlotPool::<usize, ()>::new(
        NonZeroUsize::new(1).unwrap(),
    ));
    let first = pool.acquire();
    let worker_pool = Arc::clone(&pool);
    let (acquired_tx, acquired_rx) = mpsc::channel();
    let worker = thread::spawn(move || {
        let second = worker_pool.acquire();
        acquired_tx
            .send(second.slot_index())
            .expect("capacity test receiver should remain live");
    });

    wait_for_waiters(&pool, 1);
    assert!(acquired_rx.try_recv().is_err());
    drop(first);

    assert_eq!(
        acquired_rx
            .recv_timeout(TIMEOUT)
            .expect("waiting lease should acquire after release"),
        0,
    );
    worker.join().expect("capacity worker should not panic");
}

#[test]
fn waiter_tickets_are_served_in_fifo_order() {
    let pool = Arc::new(BoundedSlotPool::<usize, ()>::new(
        NonZeroUsize::new(1).unwrap(),
    ));
    let held = pool.acquire();
    let (order_tx, order_rx) = mpsc::channel();

    let first_pool = Arc::clone(&pool);
    let first_tx = order_tx.clone();
    let first = thread::spawn(move || {
        let _lease = first_pool.acquire();
        first_tx.send(1).expect("order receiver should remain live");
        thread::sleep(Duration::from_millis(20));
    });
    wait_for_waiters(&pool, 1);

    let second_pool = Arc::clone(&pool);
    let second = thread::spawn(move || {
        let _lease = second_pool.acquire();
        order_tx.send(2).expect("order receiver should remain live");
    });
    wait_for_waiters(&pool, 2);
    drop(held);

    assert_eq!(order_rx.recv_timeout(TIMEOUT).unwrap(), 1);
    assert_eq!(order_rx.recv_timeout(TIMEOUT).unwrap(), 2);
    first.join().expect("first waiter should not panic");
    second.join().expect("second waiter should not panic");
}

#[test]
#[expect(
    clippy::significant_drop_tightening,
    reason = "lease and coordinator scope deliberately cover the complete operation"
)]
fn cancelling_the_head_waiter_wakes_the_next_waiter() {
    let pool = Arc::new(BoundedSlotPool::<usize, ()>::new(
        NonZeroUsize::new(1).unwrap(),
    ));
    // Represent an interrupted acquisition at the head of the queue. A
    // slot is available, so cancellation must itself wake the next caller.
    let cancelled = {
        let mut coordinator = pool.state().coordinator.lock().unwrap();
        let ticket = coordinator.next_ticket;
        coordinator.next_ticket += 1;
        coordinator.waiters.push_back(ticket);
        WaitTicket {
            pool: pool.state(),
            ticket: Some(ticket),
        }
    };
    let worker_pool = Arc::clone(&pool);
    let (acquired_tx, acquired_rx) = mpsc::channel();
    let worker = thread::spawn(move || {
        let lease = worker_pool.acquire();
        acquired_tx.send(lease.slot_index()).unwrap();
    });
    wait_for_waiters(&pool, 2);
    assert!(acquired_rx.try_recv().is_err());
    drop(cancelled);
    assert_eq!(acquired_rx.recv_timeout(TIMEOUT).unwrap(), 0);
    worker.join().expect("remaining waiter should acquire");
}

#[test]
fn explicit_invalidation_retains_latest_reason_and_value_until_replacement() {
    let pool = BoundedSlotPool::new(NonZeroUsize::new(1).unwrap());
    let mut lease = pool.acquire();
    lease.replace(42);
    lease.invalidate("first");
    lease.invalidate("latest");
    drop(lease);

    let mut lease = pool.acquire();
    assert!(matches!(
        lease.state(),
        Slot::Invalidated {
            reason: "latest",
            ..
        }
    ));
    assert_eq!(lease.get(), Some(&42));
    assert_eq!(lease.replace(7), Some(42));
    assert!(matches!(lease.state(), Slot::Reusable(7)));
    drop(lease);
}

#[test]
#[expect(
    clippy::significant_drop_tightening,
    reason = "lease and coordinator scope deliberately cover the complete operation"
)]
fn unwind_invalidates_but_preserves_slot_value_for_safe_teardown() {
    for (value, invalidate) in [(None, false), (Some(42), false), (Some(42), true)] {
        let pool = BoundedSlotPool::<_, ()>::new(NonZeroUsize::new(1).unwrap());

        let panic = catch_unwind(AssertUnwindSafe(|| {
            let mut lease = pool.acquire();
            if let Some(value) = value {
                lease.replace(value);
                if invalidate {
                    lease.invalidate(());
                }
            }
            panic!("synthetic leased-test panic");
        }));
        assert!(panic.is_err());

        let mut lease = pool.acquire();
        assert!(matches!(lease.state(), Slot::Unwound(actual) if *actual == value));
        assert_eq!(lease.get(), value.as_ref());
        assert_eq!(lease.replace(7), value);
        drop(lease);

        let lease = pool.acquire();
        assert!(matches!(lease.state(), Slot::Reusable(7)));
        assert_eq!(lease.get(), Some(&7));
    }
}

fn wait_for_waiters<T, R>(pool: &BoundedSlotPool<T, R>, expected: usize) {
    let started = Instant::now();
    while pool.waiting_count() < expected {
        assert!(started.elapsed() < TIMEOUT, "waiter did not enter queue");
        thread::yield_now();
    }
}

impl<T, R> BoundedSlotPool<T, R> {
    fn waiting_count(&self) -> usize {
        self.state()
            .coordinator
            .lock()
            .unwrap_or_else(std::sync::PoisonError::into_inner)
            .waiters
            .len()
    }
}
