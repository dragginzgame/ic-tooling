use super::{BoundedSlotPool, Slot};
use std::future::Future as _;
use std::{num::NonZeroUsize, sync::Arc, time::Duration};

fn pool(capacity: usize) -> BoundedSlotPool<usize, &'static str> {
    BoundedSlotPool::new(NonZeroUsize::new(capacity).unwrap())
}

#[tokio::test]
async fn independent_leases_preserve_values_and_capacity() {
    let pool = pool(2);
    let mut first = pool.acquire().await.unwrap();
    let second = pool.acquire().await.unwrap();
    assert_ne!(first.slot_index(), second.slot_index());
    first.replace(42);
    first.invalidate("first");
    first.invalidate("latest");
    let index = first.slot_index();
    drop(first);
    let mut again = pool.acquire().await.unwrap();
    assert_eq!(again.slot_index(), index);
    assert!(matches!(
        again.state(),
        Slot::Invalidated {
            reason: "latest",
            ..
        }
    ));
    assert_eq!(again.get(), Some(&42));
    assert_eq!(again.replace(7), Some(42));
    assert!(matches!(again.state(), Slot::Reusable(7)));
    drop((again, second));
}

#[tokio::test]
async fn cancelling_a_queued_acquisition_returns_admission_to_the_next_waiter() {
    let pool = pool(1);
    let held_lease = pool.acquire().await.unwrap();
    let mut head = Box::pin(pool.acquire());
    let mut next = Box::pin(pool.acquire());
    let _ = std::future::poll_fn(|cx| std::task::Poll::Ready(head.as_mut().poll(cx))).await;
    assert!(
        std::future::poll_fn(|cx| std::task::Poll::Ready(next.as_mut().poll(cx)))
            .await
            .is_pending()
    );
    drop(head);
    drop(held_lease);
    let lease = tokio::time::timeout(Duration::from_secs(1), next)
        .await
        .unwrap()
        .unwrap();
    assert_eq!(lease.slot_index(), 0);
}

#[tokio::test]
async fn fifo_waiters_cannot_bypass_an_earlier_acquisition() {
    let pool = pool(1);
    let held_lease = pool.acquire().await.unwrap();
    let mut first = Box::pin(pool.acquire());
    let mut second = Box::pin(pool.acquire());
    assert!(
        std::future::poll_fn(|cx| std::task::Poll::Ready(first.as_mut().poll(cx)))
            .await
            .is_pending()
    );
    assert!(
        std::future::poll_fn(|cx| std::task::Poll::Ready(second.as_mut().poll(cx)))
            .await
            .is_pending()
    );
    drop(held_lease);
    assert!(
        std::future::poll_fn(|cx| std::task::Poll::Ready(second.as_mut().poll(cx)))
            .await
            .is_pending()
    );
    let first = first.await.unwrap();
    drop(first);
    let _second = second.await.unwrap();
}

#[tokio::test]
async fn cancelled_preparation_keeps_the_value_and_releases_capacity() {
    let pool = Arc::new(pool(1));
    let worker_pool = Arc::clone(&pool);
    let (started, ready) = tokio::sync::oneshot::channel();
    let worker = tokio::spawn(async move {
        let mut lease = worker_pool.acquire().await.unwrap();
        lease.replace(42);
        lease.begin_preparation();
        started.send(()).unwrap();
        std::future::pending::<()>().await;
        lease.finish_preparation();
    });
    ready.await.unwrap();
    worker.abort();
    assert!(worker.await.unwrap_err().is_cancelled());
    let lease = pool.acquire().await.unwrap();
    assert!(matches!(lease.state(), Slot::Interrupted(Some(42))));
    drop(lease);
}

#[tokio::test]
#[expect(
    clippy::significant_drop_tightening,
    reason = "the lease must remain held during the intentional panic"
)]
async fn panic_while_leased_preserves_the_value_and_releases_capacity() {
    let pool = Arc::new(pool(1));
    let worker_pool = Arc::clone(&pool);
    let worker = tokio::spawn(async move {
        let mut lease = worker_pool.acquire().await.unwrap();
        lease.replace(42);
        std::hint::black_box(&lease);
        panic!("synthetic leased-test panic");
    });
    assert!(worker.await.unwrap_err().is_panic());
    let lease = pool.acquire().await.unwrap();
    assert!(matches!(lease.state(), Slot::Unwound(Some(42))));
    drop(lease);
}

#[tokio::test]
async fn shutdown_waits_for_leases_rejects_waiters_and_keeps_all_failures() {
    let pool = pool(3);
    let mut first = pool.acquire().await.unwrap();
    let mut second = pool.acquire().await.unwrap();
    let third = pool.acquire().await.unwrap();
    first.replace(1);
    second.replace(2);
    let mut queued = Box::pin(pool.acquire());
    assert!(
        std::future::poll_fn(|cx| std::task::Poll::Ready(queued.as_mut().poll(cx)))
            .await
            .is_pending()
    );
    let attempts = Arc::new(std::sync::Mutex::new(Vec::new()));
    let observed = Arc::clone(&attempts);
    let mut shutdown = Box::pin(pool.shutdown(move |value| {
        observed.lock().unwrap().push(value);
        async move { Err(value) }
    }));
    assert!(
        std::future::poll_fn(|cx| std::task::Poll::Ready(shutdown.as_mut().poll(cx)))
            .await
            .is_pending()
    );
    assert!(pool.acquire().await.is_none());
    drop(first);
    assert!(queued.await.is_none());
    assert!(attempts.lock().unwrap().is_empty());
    drop((second, third));
    let failures = shutdown.await;
    assert_eq!(failures, vec![1, 2]);
    assert_eq!(*attempts.lock().unwrap(), vec![1, 2]);
    assert!(
        pool.shutdown(|value| async move { Err(value) })
            .await
            .is_empty()
    );
}

#[tokio::test]
async fn cancelled_shutdown_retains_unattempted_values_without_replaying_deletion() {
    let pool = pool(2);
    let mut first = pool.acquire().await.unwrap();
    let mut second = pool.acquire().await.unwrap();
    first.replace(1);
    second.replace(2);
    // Cancelling while quiescing closes admission without taking either value.
    let mut waiting = Box::pin(pool.shutdown(|value| async move { Err(value) }));
    assert!(
        std::future::poll_fn(|cx| std::task::Poll::Ready(waiting.as_mut().poll(cx)))
            .await
            .is_pending()
    );
    drop(waiting);
    assert!(pool.acquire().await.is_none());
    drop((first, second));
    let attempts = Arc::new(std::sync::Mutex::new(Vec::new()));
    let observed = Arc::clone(&attempts);
    let mut deleting = Box::pin(pool.shutdown(move |value| {
        observed.lock().unwrap().push(value);
        std::future::pending::<Result<(), usize>>()
    }));
    assert!(
        std::future::poll_fn(|cx| std::task::Poll::Ready(deleting.as_mut().poll(cx)))
            .await
            .is_pending()
    );
    drop(deleting);
    assert_eq!(*attempts.lock().unwrap(), vec![1]);
    let failures = pool.shutdown(|value| async move { Err(value) }).await;
    assert_eq!(failures, vec![2]); // the unknown first deletion is not replayed.
}

#[tokio::test]
async fn concurrent_shutdowns_serialize_and_an_unused_pool_never_allocates_slots() {
    let pool = pool(1);
    let (first, second) = tokio::join!(
        pool.shutdown(|value| async move { Err(value) }),
        pool.shutdown(|value| async move { Err(value) }),
    );
    assert!(first.is_empty() && second.is_empty());
    assert!(pool.state.get().is_none());
    assert!(pool.acquire().await.is_none());
}
