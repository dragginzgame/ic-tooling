use super::*;
use crate::held::{Limits, Reply};
use std::task::{Context, Poll, Waker};

#[test]
fn clock_steps_and_round_headroom_are_checked_before_mutation() {
    assert!(matches!(
        clock_step(Duration::ZERO),
        Err(ControlError::ClockStep)
    ));
    assert!(matches!(
        clock_step(Fixture::MAX_CLOCK_STEP + Duration::from_nanos(1)),
        Err(ControlError::ClockStep)
    ));
    assert_eq!(clock_step(Duration::from_nanos(1)).unwrap(), 1);
    let maximum = clock_step(Fixture::MAX_CLOCK_STEP).unwrap();
    assert_eq!(maximum, 60_000_000_000);
    assert_eq!(
        clock_target(u64::MAX - maximum - 1, maximum).unwrap(),
        u64::MAX - 1
    );
    assert!(matches!(
        clock_target(u64::MAX - maximum, maximum),
        Err(ControlError::ClockOverflow)
    ));
    assert!(matches!(
        clock_target(u64::MAX, maximum),
        Err(ControlError::ClockOverflow)
    ));
}

#[test]
fn interrupted_clock_changes_refuse_all_further_control() {
    let mut fixture = Fixture {
        binding: Arc::new(Binding {
            server: "http://127.0.0.1:1/".to_owned(),
            instance: 0,
            prefix: String::new(),
        }),
        wasm: vec![],
        consumed: vec![false],
        clock_uncertain: false,
    };
    let mut context = Context::from_waker(Waker::noop());
    {
        let mut operation = Box::pin(fixture.progress(std::future::pending()));
        assert!(operation.as_mut().poll(&mut context).is_pending());
    }
    assert!(fixture.clock_uncertain);
    assert!(!fixture.consumed[0]);
    // A clock failure never consumes reply authority, but all subsequent APIs
    // refuse before touching this deliberately unreachable server.
    tokio::runtime::Builder::new_current_thread()
        .enable_all()
        .build()
        .unwrap()
        .block_on(async {
            let pic = PocketIc::new_from_existing_instance(
                "http://127.0.0.1:1/".parse().unwrap(),
                0,
                Some(1),
            );
            let token = Request {
                binding: Arc::clone(&fixture.binding),
                slot: 0,
                request: CanisterHttpRequest {
                    subnet_id: "2vxsx-fae".parse().unwrap(),
                    request_id: 0,
                    http_method: pocket_ic::common::rest::CanisterHttpMethod::GET,
                    url: String::new(),
                    headers: vec![],
                    body: vec![],
                    max_response_bytes: Some(256),
                    replication: pocket_ic::common::rest::CanisterHttpReplication::FullyReplicated,
                    pricing_version: pocket_ic::common::rest::CanisterHttpPricingVersion::Legacy,
                },
            };
            assert!(matches!(
                fixture.pending(&pic).await,
                Err(ControlError::ClockUncertain)
            ));
            assert!(matches!(
                fixture.release(&pic, &token).await,
                Err(ControlError::ClockUncertain)
            ));
            assert!(matches!(
                fixture.advance_time(&pic, Duration::from_secs(1)).await,
                Err(ControlError::ClockUncertain)
            ));
        });
    // The same boundary survives a transport panic and an explicit divergent
    // observation. Successful completion alone restores clock authority.
    fixture.clock_uncertain = false;
    let panic = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
        let mut operation = Box::pin(fixture.progress(async { panic!("transport failed") }));
        let _ = operation.as_mut().poll(&mut context);
    }));
    assert!(panic.is_err());
    assert!(fixture.clock_uncertain);
    fixture.clock_uncertain = false;
    let mut divergent = Box::pin(fixture.progress(async { Err(ControlError::ClockUncertain) }));
    assert!(matches!(
        divergent.as_mut().poll(&mut context),
        Poll::Ready(Err(ControlError::ClockUncertain))
    ));
    drop(divergent);
    assert!(fixture.clock_uncertain);
    let mut completed =
        Box::pin(fixture.progress(async { Ok(Time::from_nanos_since_unix_epoch(123)) }));
    assert!(matches!(
        completed.as_mut().poll(&mut context),
        Poll::Ready(Ok(_))
    ));
    drop(completed);
    assert!(!fixture.clock_uncertain);
}

#[test]
#[ignore = "requires an explicitly owned IC_TESTKIT_POCKET_IC_URL server"]
fn installed_clock_progression_preserves_live_replies_and_original_expiry() {
    tokio::runtime::Builder::new_current_thread()
        .enable_all()
        .build()
        .unwrap()
        .block_on(async {
            let endpoint =
                std::env::var("IC_TESTKIT_POCKET_IC_URL").expect("explicit server endpoint");
            let pic = pocket_ic::PocketIcBuilder::new()
                .with_application_subnet()
                .with_server_url(endpoint.parse().unwrap())
                .build_async()
                .await;
            let other = pocket_ic::PocketIcBuilder::new()
                .with_application_subnet()
                .with_server_url(endpoint.parse().unwrap())
                .build_async()
                .await;
            let catalog = Catalog::new(
                &[
                    Reply {
                        method: "hold",
                        bytes: b"original before expiry",
                    },
                    Reply {
                        method: "hold",
                        bytes: b"original at expiry",
                    },
                    Reply {
                        method: "hold",
                        bytes: b"must never be released",
                    },
                ],
                Limits {
                    slots: 3,
                    name_bytes: 12,
                    reply_bytes: 100,
                },
            )
            .unwrap();
            let target = pic.create_canister().await;
            let mut fixture = Fixture::new(
                &pic,
                Principal::from_text(target.to_text()).unwrap(),
                &catalog,
                50_000_000_000,
                100_000,
            )
            .await
            .unwrap();
            let mut foreign = Fixture::new(
                &pic,
                Principal::from_text(target.to_text()).unwrap(),
                &catalog,
                50_000_000_000,
                100_000,
            )
            .await
            .unwrap();
            pic.add_cycles(target, 100_000_000_000_000).await;
            pic.install_canister(target, fixture.wasm().to_vec(), vec![], None)
                .await;
            let sender = "2vxsx-fae".parse().unwrap();
            for slot in 0..3 {
                let call = pic
                    .submit_call(target, sender, "hold", vec![])
                    .await
                    .unwrap();
                pic.tick().await;
                assert!(fixture.pending(&pic).await.unwrap().is_empty());
                // On the qualified server the next round dispatches the HTTP
                // management call. Capture that round, not ingress submission.
                pic.tick().await;
                let dispatched = pic.get_time().await.as_nanos_since_unix_epoch();
                let tokens = fixture.pending(&pic).await.unwrap();
                assert_eq!(tokens.len(), 1);
                let token = &tokens[0];
                assert_eq!(token.slot(), slot);
                let inventory = pic.get_canister_http().await;
                let before = pic.get_time().await;
                for duration in [
                    Duration::ZERO,
                    Fixture::MAX_CLOCK_STEP + Duration::from_nanos(1),
                ] {
                    assert!(matches!(
                        fixture.advance_time(&pic, duration).await,
                        Err(ControlError::ClockStep)
                    ));
                }
                assert!(matches!(
                    fixture.advance_time(&other, Duration::from_secs(1)).await,
                    Err(ControlError::WrongClient)
                ));
                let wrong_server = PocketIc::new_from_existing_instance(
                    "http://127.0.0.1:1/".parse().unwrap(),
                    pic.instance_id,
                    Some(1),
                );
                assert!(matches!(
                    fixture
                        .advance_time(&wrong_server, Duration::from_secs(1))
                        .await,
                    Err(ControlError::WrongClient)
                ));
                assert_eq!(pic.get_time().await, before);
                assert_eq!(pic.get_canister_http().await, inventory);

                // Cross Canic's genuine 30-second receipt window without exhausting
                // the independent HTTP barrier. Original tokens and inventory survive.
                let observed = fixture
                    .advance_time(&pic, Duration::from_secs(31))
                    .await
                    .unwrap();
                assert!(
                    observed.as_nanos_since_unix_epoch()
                        >= before.as_nanos_since_unix_epoch() + 31_000_000_000
                );
                assert!(
                    observed.as_nanos_since_unix_epoch()
                        <= before.as_nanos_since_unix_epoch() + 31_000_000_001
                );
                assert_eq!(pic.get_canister_http().await, inventory);
                assert!(matches!(
                    foreign.release(&pic, token).await,
                    Err(ControlError::ForeignToken)
                ));
                assert!(!fixture.consumed[slot]);
                if slot != 0 {
                    // Reach exactly the original HTTP dispatch time + 60 seconds.
                    // A forward set beyond the last round needs no extra clock
                    // increment on the pinned server; assert its exact observation.
                    let deadline = dispatched + 60_000_000_000;
                    let now = pic.get_time().await.as_nanos_since_unix_epoch();
                    let at = fixture
                        .advance_time(&pic, Duration::from_nanos(deadline - now))
                        .await
                        .unwrap();
                    assert_eq!(at.as_nanos_since_unix_epoch(), deadline);
                    assert_eq!(pic.get_canister_http().await, inventory);
                    if slot == 2 {
                        fixture
                            .advance_time(&pic, Duration::from_nanos(1))
                            .await
                            .unwrap();
                        assert!(matches!(
                            fixture.release(&pic, token).await,
                            Err(ControlError::NotPending)
                        ));
                        assert!(!fixture.consumed[slot]);
                        assert!(fixture.pending(&pic).await.unwrap().is_empty());
                        assert!(pic.await_call(call).await.is_err());
                        continue;
                    }
                }
                fixture.release(&pic, token).await.unwrap();
                assert!(matches!(
                    fixture.release(&pic, token).await,
                    Err(ControlError::Consumed)
                ));
                assert_eq!(
                    pic.await_call(call).await.unwrap(),
                    catalog.get(slot).unwrap().bytes
                );
            }
            // No held calls remain when live progress is enabled. It must still be
            // refused by the new operation without changing local authority.
            pic.auto_progress().await;
            assert!(matches!(
                fixture.advance_time(&pic, Duration::from_secs(1)).await,
                Err(ControlError::AutomaticProgress)
            ));
            assert!(!fixture.clock_uncertain);
            pic.stop_progress().await;
            other.drop().await;
            pic.drop().await;
        });
}
