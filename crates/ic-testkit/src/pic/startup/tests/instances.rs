use std::{
    fs,
    path::PathBuf,
    time::{Duration, Instant},
};

use pocket_ic::PocketIcBuilder;

#[cfg(unix)]
use super::super::PocketIcInstance;
use super::super::{PocketIcManagedServerOutput, PocketIcStartupConfig, PocketIcStartupFailure};

#[cfg(unix)]
use super::{TestServerScript, process_state};
#[cfg(unix)]
use ic_host_process::testing::write_executable_script;
#[cfg(unix)]
use std::io::Write as _;

#[cfg(unix)]
#[test]
fn client_owner_retains_server_through_borrow_and_transfer() {
    let script = TestServerScript::new(
        "owner-drop-order",
        "#!/bin/sh\nprintf '34567\\n' > \"$2\"\nexec sleep 30\n",
    );
    let server = script
        .config(Duration::from_secs(2))
        .start_managed_server()
        .unwrap();
    let pid = server.process_id();
    let mut owned = PocketIcInstance {
        instance: pocket_ic::nonblocking::PocketIc::new_from_existing_instance(
            server.url().parse().unwrap(),
            7,
            Some(1000),
        ),
        server: Some(server),
    };
    owned.instance_mut().instance_id = 8;
    assert_eq!(owned.instance().instance_id, 8);
    assert_eq!(owned.server().unwrap().process_id(), pid);
    let (client, server) = owned.into_parts();
    assert!(process_state(pid).is_some());
    drop(client);
    assert!(process_state(pid).is_some());
    drop(server);
    assert_eq!(process_state(pid), None);
}

#[tokio::test]
async fn async_configuration_failures_are_typed_without_server_effects() {
    for (url, timeout, invalid_configuration) in [
        ("not a URL", Duration::from_secs(1), false),
        ("http://127.0.0.1:1/", Duration::ZERO, true),
    ] {
        let error = PocketIcStartupConfig::Connect {
            server_url: url.into(),
            timeout,
        }
        .build_instance(PocketIcBuilder::new().with_max_request_time_ms(Some(300_000)))
        .await
        .err()
        .unwrap();
        assert!(matches!(
            (invalid_configuration, error.failure()),
            (true, PocketIcStartupFailure::InvalidConfiguration { .. })
                | (false, PocketIcStartupFailure::InvalidServerUrl { .. })
        ));
        assert_eq!(error.output(), &PocketIcManagedServerOutput::default());
        assert!(error.server_cleanup().is_none());
    }
}

#[cfg(unix)]
#[tokio::test]
async fn async_readiness_timeout_and_cancellation_reap_the_exact_server() {
    use std::future::Future as _;
    for cancel in [false, true] {
        let script = TestServerScript::new(
            "async-readiness",
            "#!/bin/sh\nprintf '%s' \"$$\" > \"$0.pid\"\nprintf 'readiness diagnostic' >&2\nexec sleep 30\n",
        );
        let stdout = script.path().with_extension("stdout");
        let stderr = script.path().with_extension("stderr");
        let config = PocketIcStartupConfig::Spawn {
            server_binary: script.path(),
            timeout: Duration::from_secs(2),
            server_hard_ttl: None,
            server_idle_ttl: None,
            server_output_files: Some(((&stdout).into(), (&stderr).into())),
        };
        (async {
            let mut construction = Box::pin(
                config
                    .build_instance(PocketIcBuilder::new().with_max_request_time_ms(Some(300_000))),
            );
            if cancel {
                // Poll once to spawn, then cancel while readiness is pending.
                std::future::poll_fn(|cx| {
                    assert!(construction.as_mut().poll(cx).is_pending());
                    std::task::Poll::Ready(())
                })
                .await;
                let deadline = Instant::now() + Duration::from_secs(2);
                while fs::read(&stderr).unwrap().is_empty() {
                    assert!(
                        Instant::now() < deadline,
                        "server did not publish diagnostics"
                    );
                    tokio::time::sleep(Duration::from_millis(10)).await;
                }
                drop(construction);
            } else {
                let error = construction.await.err().unwrap();
                assert!(matches!(
                    error.failure(),
                    PocketIcStartupFailure::ReadinessTimeout { .. }
                ));
                assert_eq!(error.output().stderr(), "readiness diagnostic");
                assert!(error.server_cleanup().is_none());
            }
        })
        .await;
        let pid_file = script.path().with_extension("pid");
        let pid = fs::read_to_string(&pid_file).unwrap().parse().unwrap();
        assert_eq!(process_state(pid), None);
        assert_eq!(fs::read_to_string(&stderr).unwrap(), "readiness diagnostic");
        for file in [stdout, stderr, pid_file] {
            fs::remove_file(file).unwrap();
        }
    }
}

#[cfg(unix)]
#[tokio::test]
async fn async_instance_wait_timeout_exit_and_cancellation_keep_ownership() {
    use std::future::Future as _;
    for mode in ["timeout", "exit", "cancel"] {
        let script = TestServerScript::new("async-instance", "#!/bin/sh\n");
        let exit_file = script.path().with_extension("exit");
        let http =
            PendingHttpServer::new((mode == "exit").then(|| exit_file.clone()), mode == "panic");
        let body = if mode == "exit" {
            "while [ ! -f \"$0.exit\" ]; do sleep 0.02; done\nexit 23"
        } else {
            "exec sleep 30"
        };
        write_executable_script(
            &script.path(),
            format!(
                "#!/bin/sh\nprintf '%s' \"$$\" > \"$0.pid\"\nprintf 'instance diagnostic' >&2\nprintf '{}\\n' > \"$2\"\n{body}\n",
                http.port
            ),
        ).expect("write executable test script");
        let stderr = script.path().with_extension("stderr");
        let stdout = script.path().with_extension("stdout");
        (async {
            let mut construction = Box::pin(
                PocketIcStartupConfig::Spawn { server_binary: script.path(), timeout: Duration::from_secs(2), server_hard_ttl: None, server_idle_ttl: None, server_output_files: Some(((&stdout).into(), (&stderr).into())) }
                    .build_instance(PocketIcBuilder::new().with_max_request_time_ms(Some(300_000)).with_application_subnet()));
            if mode == "cancel" {
                let deadline = Instant::now() + Duration::from_secs(2);
                while !http.received.load(std::sync::atomic::Ordering::Acquire) {
                    std::future::poll_fn(|cx| {
                        assert!(construction.as_mut().poll(cx).is_pending());
                        std::task::Poll::Ready(())
                    }).await;
                    assert!(Instant::now() < deadline, "instance request never arrived");
                    tokio::time::sleep(Duration::from_millis(10)).await;
                }
                drop(construction);
            } else {
                let error = construction.await.err().unwrap();
                if mode == "timeout" {
                    assert!(matches!(error.failure(), PocketIcStartupFailure::InstanceCreationTimeout { .. }));
                } else if mode == "exit" {
                    assert!(matches!(error.failure(), PocketIcStartupFailure::ServerExited { status, .. } if status.code() == Some(23)));
                } else {
                    unreachable!("only timeout, exit and cancellation are selected");
                }
                assert_eq!(error.output().stderr(), "instance diagnostic");
                assert!(error.server_cleanup().is_none());
            }
        }).await;
        assert!(http.received.load(std::sync::atomic::Ordering::Acquire));
        let pid_file = script.path().with_extension("pid");
        let pid = fs::read_to_string(&pid_file).unwrap().parse().unwrap();
        assert_eq!(process_state(pid), None);
        assert_eq!(fs::read_to_string(&stderr).unwrap(), "instance diagnostic");
        for file in [stdout, stderr, pid_file] {
            fs::remove_file(file).unwrap();
        }
        if exit_file.exists() {
            fs::remove_file(exit_file).unwrap();
        }
    }
}

#[cfg(unix)]
#[tokio::test]
async fn upstream_builder_panic_leaves_borrowed_server_authority_with_the_caller() {
    let http = PendingHttpServer::new(None, true);
    let config = PocketIcStartupConfig::Connect {
        server_url: format!("http://127.0.0.1:{}/", http.port),
        timeout: Duration::from_secs(2),
    };
    let panic = tokio::spawn(async move {
        config
            .build_instance(PocketIcBuilder::new().with_application_subnet())
            .await
    })
    .await
    .err()
    .expect("upstream refusal must panic");
    assert!(panic.is_panic());
    assert!(std::net::TcpStream::connect((std::net::Ipv4Addr::LOCALHOST, http.port)).is_ok());
}

#[cfg(unix)]
struct PendingHttpServer {
    port: u16,
    received: std::sync::Arc<std::sync::atomic::AtomicBool>,
    _peer: crate::test_support::http::Peer,
}

#[cfg(unix)]
impl PendingHttpServer {
    fn new(exit_file: Option<PathBuf>, respond: bool) -> Self {
        use std::sync::{
            Arc,
            atomic::{AtomicBool, Ordering},
        };
        let received = Arc::new(AtomicBool::new(false));
        let request_received = Arc::clone(&received);
        let peer = crate::test_support::http::Peer::spawn(
            None,
            Duration::ZERO,
            ic_host_process::testing::http::HttpFixtureLimits {
                timeout: Duration::from_secs(5),
                max_header_bytes: 65_536,
                max_body_bytes: 65_536,
            },
            move |stream, request, stop| {
                if request.failure.is_some() {
                    return;
                }
                request_received.store(true, Ordering::Release);
                if let Some(path) = &exit_file {
                    fs::write(path, b"exit").unwrap();
                }
                if respond {
                    stream
                        .write_all(
                            b"HTTP/1.1 500 Error\r\nContent-Length: 0\r\nConnection: close\r\n\r\n",
                        )
                        .unwrap();
                } else {
                    while !stop.load(Ordering::Acquire) {
                        std::thread::sleep(Duration::from_millis(10));
                    }
                }
            },
        );
        Self {
            port: peer.url().port().unwrap(),
            received,
            _peer: peer,
        }
    }
}

#[cfg(unix)]
#[tokio::test]
#[ignore = "requires POCKET_IC_BIN=<caller-provided PocketIC server binary>"]
async fn owned_instance_drop_preserves_topology_and_stops_server() {
    let binary = std::env::var_os("POCKET_IC_BIN").unwrap();
    let config = PocketIcStartupConfig::Spawn {
        server_binary: PathBuf::from(binary),
        timeout: Duration::from_secs(30),
        server_hard_ttl: None,
        server_idle_ttl: None,
        server_output_files: None,
    };
    let owned = config
        .build_instance(
            PocketIcBuilder::new()
                .with_max_request_time_ms(Some(300_000))
                .with_application_subnet()
                .with_nns_subnet(),
        )
        .await
        .unwrap();
    let server = owned.server().unwrap();
    let pid = server.process_id();
    let directory = server.server.files.directory.path().to_path_buf();
    assert!(owned.instance().topology().await.get_nns().is_some());
    assert!(process_state(pid).is_some());
    let canister = owned.instance().create_canister().await;
    assert!(owned.instance().canister_exists(canister).await);
    let borrowed = PocketIcStartupConfig::Connect {
        server_url: (server.url()).into(),
        timeout: Duration::from_secs(30),
    }
    .build_instance(
        PocketIcBuilder::new()
            .with_max_request_time_ms(Some(300_000))
            .with_application_subnet(),
    )
    .await
    .unwrap();
    assert!(borrowed.server().is_none());
    drop(borrowed);
    assert!(owned.instance().canister_exists(canister).await);
    drop(owned);
    assert_eq!(process_state(pid), None);
    assert!(!directory.exists());
}

#[cfg(unix)]
#[tokio::test]
#[ignore = "requires POCKET_IC_BIN=<caller-provided PocketIC server binary>"]
async fn explicit_transfer_preserves_the_external_server_and_client_identity() {
    let owned = PocketIcStartupConfig::Spawn {
        server_binary: PathBuf::from(std::env::var_os("POCKET_IC_BIN").unwrap()),
        timeout: Duration::from_secs(30),
        server_hard_ttl: None,
        server_idle_ttl: None,
        server_output_files: None,
    }
    .build_instance(
        PocketIcBuilder::new()
            .with_max_request_time_ms(Some(300_000))
            .with_application_subnet(),
    )
    .await
    .unwrap();
    let id = owned.instance().instance_id;
    let pid = owned.server().unwrap().process_id();
    let (instance, server) = owned.into_parts();
    assert!(process_state(pid).is_some());
    let borrowed = PocketIcInstance::from_borrowed_server(instance);
    assert!(borrowed.server().is_none());
    assert_eq!(borrowed.instance().instance_id, id);
    let canister = borrowed.instance().create_canister().await;
    assert!(borrowed.instance().canister_exists(canister).await);
    borrowed.close().await;
    assert!(
        process_state(pid).is_some(),
        "borrowed teardown stopped the external server"
    );

    // The transferred server remains usable after deletion of the original client.
    let other = PocketIcStartupConfig::Connect {
        server_url: server.as_ref().unwrap().url().to_owned(),
        timeout: Duration::from_secs(30),
    }
    .build_instance(
        PocketIcBuilder::new()
            .with_max_request_time_ms(Some(300_000))
            .with_application_subnet(),
    )
    .await
    .unwrap();
    let canister = other.instance().create_canister().await;
    assert!(other.instance().canister_exists(canister).await);
    other.close().await;
    drop(server);
    assert_eq!(process_state(pid), None);
}

#[cfg(unix)]
#[tokio::test]
#[ignore = "requires POCKET_IC_BIN=<caller-provided PocketIC server binary>"]
async fn explicit_close_and_borrowed_connections_preserve_lifetimes() {
    let binary = PathBuf::from(std::env::var_os("POCKET_IC_BIN").unwrap());
    let owned = PocketIcStartupConfig::Spawn {
        server_binary: (&binary).into(),
        timeout: Duration::from_secs(30),
        server_hard_ttl: None,
        server_idle_ttl: None,
        server_output_files: None,
    }
    .build_instance(
        PocketIcBuilder::new()
            .with_max_request_time_ms(Some(300_000))
            .with_application_subnet()
            .with_nns_subnet(),
    )
    .await
    .unwrap();
    let server = owned.server().unwrap();
    let pid = server.process_id();
    let directory = server.server.files.directory.path().to_path_buf();
    assert!(owned.instance().topology().await.get_nns().is_some());
    let canister = owned.instance().create_canister().await;
    assert!(owned.instance().canister_exists(canister).await);
    let borrowed = PocketIcStartupConfig::Connect {
        server_url: (server.url()).into(),
        timeout: Duration::from_secs(30),
    }
    .build_instance(
        PocketIcBuilder::new()
            .with_max_request_time_ms(Some(300_000))
            .with_application_subnet(),
    )
    .await
    .unwrap();
    assert!(borrowed.server().is_none());
    let (borrowed, external_server) = borrowed.into_parts();
    borrowed.drop().await;
    drop(external_server);
    assert!(owned.instance().canister_exists(canister).await);
    let (instance, server) = owned.into_parts();
    instance.drop().await;
    assert!(process_state(pid).is_some());
    drop(server);
    assert_eq!(process_state(pid), None);
    assert!(!directory.exists());
}

#[cfg(unix)]
#[tokio::test]
#[ignore = "requires POCKET_IC_BIN=<caller-provided PocketIC server binary>"]
async fn fixtures_and_baselines_retain_the_owned_server() {
    use crate::pic::{CachedPocketIcBaseline, InstallSpec, StandaloneCanisterFixture};
    let binary = PathBuf::from(std::env::var_os("POCKET_IC_BIN").unwrap());
    let owned = PocketIcStartupConfig::Spawn {
        server_binary: (&binary).into(),
        timeout: Duration::from_secs(30),
        server_hard_ttl: None,
        server_idle_ttl: None,
        server_output_files: None,
    }
    .build_instance(
        PocketIcBuilder::new()
            .with_max_request_time_ms(Some(300_000))
            .with_application_subnet(),
    )
    .await
    .unwrap();
    let pid = owned.server().unwrap().process_id();
    let fixture = StandaloneCanisterFixture::install(
        owned,
        InstallSpec {
            wasm: b"\0asm\x01\0\0\0".to_vec(),
            init_bytes: vec![],
            cycles: 5_000_000_000_000,
            install_sender: None,
        },
    )
    .await;
    let (owned, canister) = fixture.into_parts();
    owned
        .instance()
        .stop_canister(canister, None)
        .await
        .unwrap();
    assert_eq!(owned.server().unwrap().process_id(), pid);
    let baseline = CachedPocketIcBaseline::capture(
        owned,
        [crate::pic::CanisterSnapshotTarget {
            canister_id: canister,
            sender: None,
        }],
        (),
    )
    .await
    .unwrap();
    baseline
        .pocket_ic()
        .uninstall_canister(canister, None)
        .await
        .unwrap();
    baseline
        .restore(crate::pic::SnapshotRestoreFunding::Preserve)
        .await
        .unwrap();
    assert!(
        baseline
            .pocket_ic()
            .canister_status(canister, None)
            .await
            .unwrap()
            .module_hash
            .is_some()
    );
    assert!(process_state(pid).is_some());
    drop(baseline);
    assert_eq!(
        process_state(pid),
        None,
        "baseline drop must reap its server"
    );
}

#[cfg(unix)]
#[tokio::test]
#[ignore = "requires POCKET_IC_BIN=<caller-provided PocketIC server binary>"]
async fn failed_baseline_capture_reaps_the_owned_server() {
    use crate::pic::CachedPocketIcBaseline;

    let binary = PathBuf::from(std::env::var_os("POCKET_IC_BIN").unwrap());
    let owned = PocketIcStartupConfig::Spawn {
        server_binary: binary,
        timeout: Duration::from_secs(30),
        server_hard_ttl: None,
        server_idle_ttl: None,
        server_output_files: None,
    }
    .build_instance(
        PocketIcBuilder::new()
            .with_max_request_time_ms(Some(300_000))
            .with_application_subnet(),
    )
    .await
    .unwrap();
    let pid = owned.server().unwrap().process_id();
    // A real, routed canister without installed Wasm exercises the returned
    // capture failure. An unroutable Principal instead panics in upstream HTTP
    // routing before management admission, which is a different contract.
    let empty_canister = owned.instance().create_canister().await;
    let capture = CachedPocketIcBaseline::capture(
        owned,
        [crate::pic::CanisterSnapshotTarget {
            canister_id: empty_canister,
            sender: None,
        }],
        (),
    )
    .await;
    assert!(
        capture.is_err(),
        "empty canister must refuse snapshot capture"
    );
    assert_eq!(
        process_state(pid),
        None,
        "failed capture must release its owner"
    );
}
