use super::*;
use std::cell::Cell;

#[tokio::test]
async fn initial_completion_and_failure_do_not_advance() {
    for budget in [0, 3] {
        for result in [Ok(42), Err("predicate failure")] {
            assert_eq!(
                poll_with_rounds(
                    budget,
                    async || panic!("unexpected round"),
                    async || Poll::Ready(result)
                )
                .await
                .map_err(|e| e.to_string()),
                result
                    .map_err(TickUntilError::Failed)
                    .map_err(|e| e.to_string())
            );
        }
    }
}

#[tokio::test]
async fn completion_on_last_round_preserves_poll_order() {
    let rounds = Cell::new(0);
    let polls = Cell::new(0);
    let result = poll_with_rounds(
        3,
        async || {
            assert_eq!(polls.get(), rounds.get() + 1);
            rounds.set(rounds.get() + 1);
        },
        async || {
            assert_eq!(polls.get(), rounds.get());
            polls.set(polls.get() + 1);
            if rounds.get() == 3 {
                Poll::Ready(Ok::<_, ()>(42))
            } else {
                Poll::Pending
            }
        },
    )
    .await;
    assert_eq!(result.unwrap(), 42);
    assert_eq!((rounds.get(), polls.get()), (3, 4));
}

#[tokio::test]
async fn exhaustion_performs_exactly_the_allowed_rounds() {
    for budget in [0, 1, 3] {
        let rounds = Cell::new(0);
        let polls = Cell::new(0);
        let result = poll_with_rounds(
            budget,
            async || {
                rounds.set(rounds.get() + 1);
            },
            async || {
                polls.set(polls.get() + 1);
                Poll::<Result<(), ()>>::Pending
            },
        )
        .await;
        assert!(
            matches!(result, Err(TickUntilError::ProgressLimit { rounds }) if rounds == budget)
        );
        assert_eq!((rounds.get(), polls.get()), (budget, budget + 1));
    }
}

#[tokio::test]
async fn failure_after_progress_preserves_the_original_cause() {
    let rounds = Cell::new(0);
    let result = poll_with_rounds(
        3,
        async || {
            rounds.set(rounds.get() + 1);
        },
        async || {
            if rounds.get() == 1 {
                Poll::Ready(Err::<(), _>(std::io::Error::from(
                    std::io::ErrorKind::PermissionDenied,
                )))
            } else {
                Poll::Pending
            }
        },
    )
    .await;
    let error = result.unwrap_err();
    let source = std::error::Error::source(&error).unwrap();
    assert_eq!(
        source.downcast_ref::<std::io::Error>().unwrap().kind(),
        std::io::ErrorKind::PermissionDenied
    );
    assert_eq!(rounds.get(), 1);
}
#[tokio::test]
async fn polling_transport_skips_zero_time_and_preserves_nonzero_advancement() {
    for advance in [Duration::ZERO, Duration::from_nanos(7)] {
        assert_time_requests(advance, 1, async |pic| {
            let mut polls = 0;
            let result = tick_until(pic, 1, advance, async |_| {
                polls += 1;
                if polls == 2 {
                    Poll::Ready(Ok::<_, ()>(42))
                } else {
                    Poll::Pending
                }
            })
            .await;
            assert_eq!(result.unwrap(), 42);
            assert_eq!(polls, 2);
        })
        .await;
    }
}

#[tokio::test]
async fn tick_batches_preserve_exact_transport_order_and_selected_counts() {
    for advance in [Duration::ZERO, Duration::from_nanos(7)] {
        for ticks in [0, 2, 4, 24] {
            assert_time_requests(advance, ticks, async |pic| {
                advance_and_tick(pic, advance, ticks).await;
            })
            .await;
        }
    }
}

#[tokio::test]
async fn install_cooldown_uses_exactly_two_ticks() {
    for advance in [Duration::ZERO, Duration::from_nanos(7)] {
        assert_time_requests(advance, 2, async |pic| {
            crate::pic::wait_out_install_code_rate_limit(pic, advance).await;
        })
        .await;
    }
}

async fn assert_time_requests(advance: Duration, ticks: u32, run: impl AsyncFnOnce(&PocketIc)) {
    use std::{
        io::Write as _,
        sync::{Arc, Mutex},
    };
    let mut expected = Vec::new();
    if !advance.is_zero() {
        expected.extend(["/instances/0/read/get_time", "/instances/0/update/set_time"]);
    }
    expected.extend(std::iter::repeat_n(
        "/instances/0/update/tick",
        ticks as usize,
    ));
    let request_count = expected.len();
    let requests = Arc::new(Mutex::new(Vec::new()));
    let recorded = Arc::clone(&requests);
    let peer = crate::test_support::http::Peer::spawn(
        Some(request_count),
        Duration::ZERO,
        ic_host_process::testing::http::HttpFixtureLimits {
            timeout: Duration::from_secs(5),
            max_header_bytes: 16_384,
            max_body_bytes: 16_384,
        },
        move |stream, request, _deadline, _stop| {
            let request = request.into_result().unwrap();
            let request = crate::test_support::http::Request::new(&request);
            let path = request.path.to_owned();
            let reply = if path.ends_with("/read/get_time") {
                "{\"nanos_since_epoch\":100}"
            } else {
                "null"
            };
            if path.ends_with("/update/set_time") {
                assert_eq!(
                    serde_json::from_slice::<serde_json::Value>(request.body).unwrap()["nanos_since_epoch"],
                    100 + u64::from(advance.subsec_nanos())
                );
            }
            write!(stream, "HTTP/1.1 200 OK\r\nContent-Type: application/json\r\nContent-Length: {}\r\nConnection: close\r\n\r\n{reply}", reply.len()).unwrap();
            recorded.lock().unwrap().push(path);
        },
    );

    let pic = PocketIc::new_from_existing_instance(peer.url().clone(), 0, Some(1000));
    run(&pic).await;
    peer.finish();
    assert_eq!(*requests.lock().unwrap(), expected);
}
