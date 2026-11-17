use super::*;
use crate::held::{Limits, Reply};

#[test]
fn cancellation_and_panic_do_not_restore_release_authority() {
    use std::task::{Context, Poll, Waker};

    let mut fixture = Fixture {
        binding: Arc::new(Binding {
            server: String::new(),
            instance: 0,
            prefix: String::new(),
        }),
        wasm: vec![],
        consumed: vec![false, false],
        clock_uncertain: false,
    };
    let mut context = Context::from_waker(Waker::noop());
    {
        let mut operation = Box::pin(fixture.dispatch(0, std::future::pending()));
        assert_eq!(operation.as_mut().poll(&mut context), Poll::Pending);
        // Drop the in-flight operation at its first suspension point.
    }
    assert!(fixture.consumed[0]);
    let failure = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
        let mut operation = Box::pin(fixture.dispatch(1, async { panic!("transport failed") }));
        let _ = operation.as_mut().poll(&mut context);
    }));
    assert!(failure.is_err());
    assert!(fixture.consumed[1]);
}

#[test]
#[ignore = "requires an explicitly owned IC_TESTKIT_POCKET_IC_URL server"]
fn installed_control_refuses_foreign_consumed_expired_and_automatic_releases() {
    tokio::runtime::Builder::new_current_thread()
        .enable_all()
        .build()
        .unwrap()
        .block_on(async {
            let endpoint = std::env::var("IC_TESTKIT_POCKET_IC_URL")
                .expect("controller test requires an externally owned server endpoint");
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
                        bytes: b"first",
                    },
                    Reply {
                        method: "hold",
                        bytes: b"second",
                    },
                ],
                Limits {
                    slots: 2,
                    name_bytes: 8,
                    reply_bytes: 11,
                },
            )
            .unwrap();
            let a = pic.create_canister().await;
            let b = pic.create_canister().await;
            let a_id = Principal::from_text(a.to_text()).unwrap();
            let b_id = Principal::from_text(b.to_text()).unwrap();
            assert_eq!(a_id.as_slice(), a.as_slice());
            assert_eq!(b_id.as_slice(), b.as_slice());
            let mut first = Fixture::new(&pic, a_id, &catalog, 50_000_000_000, 100_000)
                .await
                .unwrap();
            let other_catalog = Catalog::new(
                &[Reply {
                    method: "hold",
                    bytes: b"different fixture",
                }],
                Limits {
                    slots: 1,
                    name_bytes: 4,
                    reply_bytes: 17,
                },
            )
            .unwrap();
            let mut second = Fixture::new(&pic, b_id, &other_catalog, 50_000_000_000, 100_000)
                .await
                .unwrap();
            // A fresh constructor cannot observe a previous controller's live calls,
            // even when the caller supplies the same target. Never install these bytes.
            let fresh = Fixture::new(&pic, a_id, &catalog, 50_000_000_000, 100_000)
                .await
                .unwrap();
            assert_ne!(first.wasm(), fresh.wasm());
            for (target, fixture) in [(a, &first), (b, &second)] {
                pic.add_cycles(target, 100_000_000_000_000).await;
                pic.install_canister(target, fixture.wasm().to_vec(), vec![], None)
                    .await;
            }
            let sender = "2vxsx-fae".parse().unwrap();
            let call_a = pic.submit_call(a, sender, "hold", vec![]).await.unwrap();
            let call_b = pic.submit_call(b, sender, "hold", vec![]).await.unwrap();
            for _ in 0..20 {
                if pic.get_canister_http().await.len() == 2 {
                    break;
                }
                pic.tick().await;
            }
            assert_eq!(pic.get_canister_http().await.len(), 2);
            let tokens_a = first.pending(&pic).await.unwrap();
            let tokens_b = second.pending(&pic).await.unwrap();
            assert_eq!((tokens_a.len(), tokens_b.len()), (1, 1));
            assert_eq!((tokens_a[0].slot(), tokens_b[0].slot()), (0, 0));
            assert!(fresh.pending(&pic).await.unwrap().is_empty());
            let wrong_server = PocketIc::new_from_existing_instance(
                "http://127.0.0.1:1/".parse().unwrap(),
                pic.instance_id,
                Some(1),
            );
            assert!(matches!(
                first.pending(&wrong_server).await,
                Err(ControlError::WrongClient)
            ));
            assert!(matches!(
                first.release(&wrong_server, &tokens_a[0]).await,
                Err(ControlError::WrongClient)
            ));
            // Test-only altered observations prove that URL/body/slot equality
            // cannot substitute for the upstream request and subnet identities.
            for change_subnet in [false, true] {
                let mut request = tokens_a[0].request.clone();
                if change_subnet {
                    request.subnet_id = sender;
                } else {
                    request.request_id = u64::MAX;
                }
                let altered = Request {
                    binding: Arc::clone(&first.binding),
                    slot: 0,
                    request,
                };
                assert!(matches!(
                    first.release(&pic, &altered).await,
                    Err(ControlError::NotPending)
                ));
            }
            assert!(matches!(
                first.pending(&other).await,
                Err(ControlError::WrongClient)
            ));
            assert!(matches!(
                first.release(&other, &tokens_a[0]).await,
                Err(ControlError::WrongClient)
            ));
            assert!(matches!(
                first.release(&pic, &tokens_b[0]).await,
                Err(ControlError::ForeignToken)
            ));
            assert_eq!(pic.get_canister_http().await.len(), 2);
            second.release(&pic, &tokens_b[0]).await.unwrap();
            first.release(&pic, &tokens_a[0]).await.unwrap();
            assert!(matches!(
                first.release(&pic, &tokens_a[0]).await,
                Err(ControlError::Consumed)
            ));
            assert!(first.pending(&pic).await.unwrap().is_empty());
            assert_eq!(pic.await_call(call_b).await.unwrap(), b"different fixture");
            assert_eq!(pic.await_call(call_a).await.unwrap(), b"first");

            // Supported clock steps expire the original barrier without releasing
            // application bytes or consuming a local release attempt.
            let expiring = pic.submit_call(a, sender, "hold", vec![]).await.unwrap();
            for _ in 0..20 {
                if !first.pending(&pic).await.unwrap().is_empty() {
                    break;
                }
                pic.tick().await;
            }
            let stale = first.pending(&pic).await.unwrap();
            assert_eq!(stale.len(), 1);
            assert_eq!(stale[0].slot(), 1);
            first
                .advance_time(&pic, std::time::Duration::from_secs(60))
                .await
                .unwrap();
            first
                .advance_time(&pic, std::time::Duration::from_secs(1))
                .await
                .unwrap();
            for _ in 0..20 {
                pic.tick().await;
                if pic.get_canister_http().await.is_empty() {
                    break;
                }
            }
            assert!(pic.get_canister_http().await.is_empty());
            assert!(matches!(
                first.release(&pic, &stale[0]).await,
                Err(ControlError::NotPending)
            ));
            assert!(pic.await_call(expiring).await.is_err());

            // Enable live progression only after every held request has settled.
            pic.auto_progress().await;
            assert!(matches!(
                Fixture::new(&pic, a_id, &catalog, 0, 100_000).await,
                Err(ControlError::AutomaticProgress)
            ));
            assert!(matches!(
                first.pending(&pic).await,
                Err(ControlError::AutomaticProgress)
            ));
            assert!(matches!(
                first.release(&pic, &tokens_a[0]).await,
                Err(ControlError::AutomaticProgress)
            ));
            pic.stop_progress().await;
            other.drop().await;
            pic.drop().await;
        });
}
