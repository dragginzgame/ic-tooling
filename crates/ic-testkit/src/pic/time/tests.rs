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
async fn progression_failure_stops_before_polling_again() {
    let polls = Cell::new(0);
    let result = poll_with_rounds(
        3,
        async || Err(ClientError::Protocol("tick failed".into())),
        async || {
            polls.set(polls.get() + 1);
            Poll::<Result<(), ()>>::Pending
        },
    )
    .await;
    assert_eq!(polls.get(), 1);
    assert!(
        matches!(result, Err(TickUntilError::Progress(ClientError::Protocol(message))) if message == "tick failed")
    );
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
            Ok(())
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
                Ok(())
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
            Ok(())
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
                advance_and_tick(pic, advance, ticks).await.unwrap();
            })
            .await;
        }
    }
}

#[tokio::test]
async fn install_cooldown_uses_exactly_two_ticks() {
    use crate::pic::CanisterInstallExt as _;
    for advance in [Duration::ZERO, Duration::from_nanos(7)] {
        assert_time_requests(advance, 2, async |pic| {
            pic.wait_out_install_code_rate_limit(advance).await.unwrap();
        })
        .await;
    }
}

async fn assert_time_requests(advance: Duration, ticks: u32, run: impl AsyncFnOnce(&PocketIc)) {
    use std::{io::Write as _, net::TcpListener, thread, time::Instant};
    let mut expected = Vec::new();
    if !advance.is_zero() {
        expected.extend(["/instances/0/read/get_time", "/instances/0/update/set_time"]);
    }
    expected.extend(std::iter::repeat_n(
        "/instances/0/update/tick",
        ticks as usize,
    ));
    let request_count = expected.len();
    let listener = TcpListener::bind((std::net::Ipv4Addr::LOCALHOST, 0)).unwrap();
    listener.set_nonblocking(true).unwrap();
    let url = format!("http://{}/", listener.local_addr().unwrap())
        .parse()
        .unwrap();
    let worker = thread::spawn(move || {
        let deadline = Instant::now() + Duration::from_secs(5);
        let mut requests = Vec::new();
        while requests.len() < request_count {
            assert!(Instant::now() < deadline, "simulated-time peer timed out");
            let (mut stream, _) = match listener.accept() {
                Ok(connection) => connection,
                Err(error) if error.kind() == std::io::ErrorKind::WouldBlock => {
                    thread::sleep(Duration::from_millis(1));
                    continue;
                }
                Err(error) => panic!("accept simulated-time request: {error}"),
            };
            let request = ic_host_process::testing::http::capture_http_request(
                &mut stream,
                Duration::from_secs(1),
                16_384,
                16_384,
                ic_host_process::testing::http::HttpContentLength::Optional,
            )
            .into_result()
            .unwrap();
            let text = std::str::from_utf8(&request).unwrap();
            let path = text.split_whitespace().nth(1).unwrap().to_owned();
            let reply = if path.ends_with("/read/get_time") {
                "{\"nanos_since_epoch\":100}"
            } else {
                "null"
            };
            if path.ends_with("/update/set_time") {
                let body = text.split_once("\r\n\r\n").unwrap().1;
                assert_eq!(
                    serde_json::from_str::<serde_json::Value>(body).unwrap()["nanos_since_epoch"],
                    100 + u64::from(advance.subsec_nanos())
                );
            }
            stream
                .set_write_timeout(Some(Duration::from_secs(1)))
                .unwrap();
            write!(stream, "HTTP/1.1 200 OK\r\nContent-Type: application/json\r\nContent-Length: {}\r\nConnection: close\r\n\r\n{reply}", reply.len()).unwrap();
            requests.push(path);
        }
        requests
    });
    let pic = PocketIc::new_from_existing_instance(url, 0, Some(1_000));
    run(&pic).await;
    assert_eq!(worker.join().unwrap(), expected);
}
