use super::*;
use std::fs;
use std::os::unix::fs::{PermissionsExt, symlink};

fn private_directory() -> tempfile::TempDir {
    tempfile::Builder::new()
        .permissions(fs::Permissions::from_mode(0o700))
        .tempdir()
        .unwrap()
}

fn limits() -> Limits {
    Limits {
        startup: Duration::from_millis(100),
        lifetime: Duration::from_secs(2),
        stdout_bytes: 256,
        stderr_bytes: 256,
    }
}
fn executable(source: &str) -> (tempfile::TempDir, PreparedServer) {
    let directory = private_directory();
    let path = directory.path().join("pocket-ic");
    fs::write(&path, format!("#!/bin/sh\n{source}\n")).unwrap();
    fs::set_permissions(&path, fs::Permissions::from_mode(0o755)).unwrap();
    let prepared = PreparedServer::fixture(directory.path());
    (directory, prepared)
}
fn startup_failure(prepared: PreparedServer) -> (String, Stopped) {
    match Server::start(prepared, limits()) {
        Err(Error::Startup { reason, evidence }) => (reason, *evidence),
        Err(other) => panic!("unexpected {other:?}"),
        Ok(_) => panic!("invalid server admitted"),
    }
}

#[test]
fn offline_admission_refuses_missing_corrupt_nonexecutable_and_linked_assets() {
    let directory = private_directory();
    assert!(matches!(
        PreparedServer::check(&directory.path().join("absent")),
        Err(Error::Asset("missing directory"))
    ));
    assert!(matches!(
        PreparedServer::check(directory.path()),
        Err(Error::Asset("missing executable"))
    ));
    let path = directory.path().join("pocket-ic");
    fs::write(&path, b"wrong executable").unwrap();
    fs::set_permissions(&path, fs::Permissions::from_mode(0o600)).unwrap();
    assert!(matches!(
        PreparedServer::check(directory.path()),
        Err(Error::Asset("not executable"))
    ));
    fs::set_permissions(&path, fs::Permissions::from_mode(0o755)).unwrap();
    assert!(matches!(
        PreparedServer::check(directory.path()),
        Err(Error::Asset("executable checksum"))
    ));
    fs::hard_link(&path, directory.path().join("alias")).unwrap();
    assert!(matches!(
        PreparedServer::check(directory.path()),
        Err(Error::Filesystem(ic_host_fs::Error::Aliased))
    ));
    fs::remove_file(&path).unwrap();
    symlink("alias", &path).unwrap();
    assert!(matches!(
        PreparedServer::check(directory.path()),
        Err(Error::Filesystem(_))
    ));
    let archive = directory.path().join("bad.gz");
    fs::write(&archive, b"not a release archive").unwrap();
    fs::set_permissions(&archive, fs::Permissions::from_mode(0o600)).unwrap();
    let destination = private_directory();
    assert!(matches!(
        prepare(
            &archive,
            destination.path(),
            std::path::Path::new("/unavailable-gzip")
        ),
        Err(Error::Asset("archive checksum"))
    ));
    assert_eq!(fs::read_dir(destination.path()).unwrap().count(), 0);
}

#[test]
fn prepared_identity_is_rechecked_before_spawn() {
    let (directory, prepared) = executable("exit 0");
    let path = directory.path().join("pocket-ic");
    fs::rename(&path, directory.path().join("original")).unwrap();
    fs::copy(directory.path().join("original"), &path).unwrap();
    assert!(matches!(
        Server::start(prepared, limits()),
        Err(Error::Asset("prepared identity changed"))
    ));
}

#[test]
fn startup_exit_deadline_and_output_limits_keep_original_cleanup_evidence() {
    let (_directory, prepared) = executable("printf hello; printf problem >&2; exit 7");
    let (_, result) = startup_failure(prepared);
    assert_eq!(result.process.status.unwrap().code(), Some(7));
    assert!(result.process.cleanup.is_empty());
    assert_eq!(result.output.stdout, b"hello");
    assert_eq!(result.output.stderr, b"problem");
    let (_directory, prepared) = executable("sleep 60");
    let began = Instant::now();
    let (reason, result) = startup_failure(prepared);
    assert!(reason.contains("startup deadline"));
    assert!(began.elapsed() < Duration::from_secs(5));
    assert!(matches!(
        result.process.failure,
        Some(ic_host_process::Failure::Cancelled)
    ));
    assert!(result.process.status.is_some());
    assert!(result.process.cleanup.is_empty());
    for (command, stream) in [
        ("head -c 4096 /dev/zero", Stream::Stdout),
        ("head -c 4096 /dev/zero >&2", Stream::Stderr),
    ] {
        let (_directory, prepared) = executable(command);
        let (_, result) = startup_failure(prepared);
        assert!(
            matches!(result.process.failure, Some(ic_host_process::Failure::Limit(actual)) if actual == stream)
        );
        assert!(result.output.stdout.len() <= 256 && result.output.stderr.len() <= 256);
        assert!(result.process.status.is_some());
        assert!(result.process.cleanup.is_empty());
    }
}

#[test]
fn malformed_readiness_is_refused_and_joined() {
    let (_directory, prepared) = executable(
        "while [ $# -gt 0 ]; do if [ \"$1\" = --port-file ]; then shift; port=$1; fi; shift; done\nprintf invalid > \"$port\"\nsleep 60",
    );
    let (reason, result) = startup_failure(prepared);
    assert!(reason.contains("exceeds") || reason.contains("malformed"));
    assert!(result.process.status.is_some());
    assert!(result.process.cleanup.is_empty());
}

#[test]
#[ignore = "requires TESTKIT_SERVER_ARCHIVE; local socket access"]
fn installed_decompression_failure_abandonment_and_expiry() {
    let archive = std::path::PathBuf::from(
        std::env::var_os("TESTKIT_SERVER_ARCHIVE").expect("explicit archive"),
    );
    let gzip = std::path::PathBuf::from(std::env::var_os("TESTKIT_GZIP").expect("explicit gzip"));
    let destination = private_directory();
    let (_tool, failed_gzip) = executable("printf decompression-refused >&2; exit 19");
    match prepare(&archive, destination.path(), failed_gzip.path()) {
        Err(Error::Decompression(evidence)) => {
            assert_eq!(evidence.process.status.unwrap().code(), Some(19));
            assert_eq!(evidence.output.stderr, b"decompression-refused");
            assert!(evidence.process.cleanup.is_empty());
        }
        other => panic!("unexpected {other:?}"),
    }
    assert_eq!(fs::read_dir(destination.path()).unwrap().count(), 0);
    let (_tool, wrong_gzip) = executable("printf wrong-executable");
    assert!(matches!(
        prepare(&archive, destination.path(), wrong_gzip.path()),
        Err(Error::Asset("decompressed checksum"))
    ));
    assert_eq!(fs::read_dir(destination.path()).unwrap().count(), 0);
    let prepared = prepare(&archive, destination.path(), &gzip).unwrap();
    let mut server = Server::start(
        prepared,
        Limits {
            startup: Duration::from_secs(10),
            lifetime: Duration::from_secs(60),
            stdout_bytes: 1_048_576,
            stderr_bytes: 1_048_576,
        },
    )
    .unwrap();
    let endpoint = server.endpoint().to_string();
    tokio::runtime::Builder::new_current_thread()
        .enable_all()
        .build()
        .unwrap()
        .block_on(async {
            let timeout = Duration::from_secs(20);
            // Cancelling creation cannot be reported as proof of no remote effect.
            let creation =
                server.instance(PocketIcBuilder::new().with_application_subnet(), timeout);
            assert!(
                tokio::time::timeout(Duration::ZERO, creation)
                    .await
                    .is_err()
            );
            let instance = server
                .instance(PocketIcBuilder::new().with_application_subnet(), timeout)
                .await
                .unwrap();
            assert_eq!(instance.close(Duration::ZERO).await, Deletion::TimedOut);
            let instance = server
                .instance(PocketIcBuilder::new().with_application_subnet(), timeout)
                .await
                .unwrap();
            let panic = std::panic::catch_unwind(std::panic::AssertUnwindSafe(move || {
                let _instance = instance;
                panic!("consumer fixture failed");
            }));
            assert!(panic.is_err());
        });
    let stopped = server.stop().unwrap();
    assert_eq!(stopped.abandoned, 3);
    assert!(stopped.process.status.is_some());
    assert!(stopped.process.cleanup.is_empty());
    let address: SocketAddr = endpoint
        .trim_start_matches("http://")
        .trim_end_matches('/')
        .parse()
        .unwrap();
    assert!(TcpStream::connect_timeout(&address, Duration::from_millis(100)).is_err());

    let mut server = Server::start(
        PreparedServer::check(destination.path()).unwrap(),
        Limits {
            startup: Duration::from_secs(1),
            lifetime: Duration::from_secs(1),
            stdout_bytes: 1_048_576,
            stderr_bytes: 1_048_576,
        },
    )
    .unwrap();
    std::thread::sleep(Duration::from_millis(1300));
    let stopped = server.stop().unwrap();
    assert!(matches!(
        stopped.process.failure,
        Some(ic_host_process::Failure::Deadline)
    ));
    assert!(stopped.process.status.is_some());
    assert!(stopped.process.cleanup.is_empty());
}

#[test]
#[ignore = "requires TESTKIT_SERVER_ARCHIVE and TESTKIT_GZIP; local socket access"]
#[cfg(feature = "held-reply-fixtures")]
fn installed_assets_instances_gateway_held_replies_and_cleanup() {
    let archive = std::path::PathBuf::from(
        std::env::var_os("TESTKIT_SERVER_ARCHIVE").expect("explicit archive"),
    );
    let gzip = std::path::PathBuf::from(std::env::var_os("TESTKIT_GZIP").expect("explicit gzip"));
    let directory = private_directory();
    let prepared = prepare(&archive, directory.path(), &gzip).unwrap();
    let identity = fs::metadata(prepared.path()).unwrap().len();
    assert!(identity > 1_000_000);
    assert_eq!(
        prepare(&archive, directory.path(), &gzip).unwrap().path(),
        prepared.path()
    );
    let mut server = Server::start(
        prepared,
        Limits {
            startup: Duration::from_secs(10),
            lifetime: Duration::from_secs(90),
            stdout_bytes: 1_048_576,
            stderr_bytes: 1_048_576,
        },
    )
    .unwrap();
    let endpoint = server.endpoint().to_string();
    let runtime = tokio::runtime::Builder::new_current_thread()
        .enable_all()
        .build()
        .unwrap();
    runtime.block_on(async {
        let timeout = Duration::from_secs(20);
        let mut first = server
            .instance(PocketIcBuilder::new().with_application_subnet(), timeout)
            .await
            .unwrap();
        let second = server
            .instance(PocketIcBuilder::new().with_application_subnet(), timeout)
            .await
            .unwrap();
        assert_ne!(first.client().instance_id, second.client().instance_id);
        assert!(matches!(server.stop(), Err(Error::LiveInstances)));
        let gateway = first.client_mut().make_live(None).await;
        assert!(matches!(
            gateway.host_str(),
            Some("127.0.0.1" | "localhost")
        ));
        let response = reqwest::Client::new()
            .get(gateway.join("api/v2/status").unwrap())
            .timeout(timeout)
            .send()
            .await
            .unwrap();
        assert!(response.status().is_success());
        first.client_mut().stop_live().await;

        use crate::held::{Catalog, Fixture, Limits as ReplyLimits, Reply};
        let pic = second.client();
        let canister = pic.create_canister().await;
        pic.add_cycles(canister, 100_000_000_000_000).await;
        let catalog = Catalog::new(
            &[Reply {
                method: "hold",
                bytes: b"owned-server-reply",
            }],
            ReplyLimits {
                slots: 1,
                name_bytes: 4,
                reply_bytes: 18,
            },
        )
        .unwrap();
        let mut held = Fixture::new(
            pic,
            canister.to_text().parse().unwrap(),
            &catalog,
            50_000_000_000,
            100_000,
        )
        .await
        .unwrap();
        pic.install_canister(canister, held.wasm().to_vec(), vec![], None)
            .await;
        let message = pic
            .submit_call(canister, "2vxsx-fae".parse().unwrap(), "hold", vec![])
            .await
            .unwrap();
        let mut requests = vec![];
        for _ in 0..30 {
            requests = held.pending(pic).await.unwrap();
            if !requests.is_empty() {
                break;
            }
            pic.tick().await;
        }
        assert_eq!(requests.len(), 1);
        held.release(pic, &requests[0]).await.unwrap();
        assert_eq!(
            pic.await_call(message).await.unwrap(),
            b"owned-server-reply"
        );
        assert_eq!(second.close(timeout).await, Deletion::Attempted);
        assert_eq!(first.close(timeout).await, Deletion::Attempted);
        // Independent observation; neither Instance::close nor upstream drop
        // makes this stronger claim on behalf of the caller.
        let states: Vec<String> = reqwest::Client::new()
            .get(format!("{endpoint}instances"))
            .timeout(timeout)
            .send()
            .await
            .unwrap()
            .json()
            .await
            .unwrap();
        assert_eq!(states, ["Deleted", "Deleted"]);
    });
    let address: SocketAddr = endpoint
        .trim_start_matches("http://")
        .trim_end_matches('/')
        .parse()
        .unwrap();
    let stopped = server.stop().unwrap();
    assert_eq!(stopped.abandoned, 0);
    assert!(matches!(
        stopped.process.failure,
        Some(ic_host_process::Failure::Cancelled)
    ));
    assert!(stopped.process.status.is_some());
    assert!(stopped.process.cleanup.is_empty());
    assert!(TcpStream::connect_timeout(&address, Duration::from_millis(100)).is_err());
    assert!(matches!(server.stop(), Err(Error::Stopped)));

    // Dropping the server handle cannot tear down a live retained instance.
    let server = Server::start(
        PreparedServer::check(directory.path()).unwrap(),
        Limits {
            startup: Duration::from_secs(10),
            lifetime: Duration::from_secs(30),
            stdout_bytes: 1_048_576,
            stderr_bytes: 1_048_576,
        },
    )
    .unwrap();
    let endpoint = server.endpoint().to_string();
    runtime.block_on(async {
        let instance = server
            .instance(
                PocketIcBuilder::new().with_application_subnet(),
                Duration::from_secs(20),
            )
            .await
            .unwrap();
        drop(server);
        assert!(
            !instance
                .client()
                .topology()
                .await
                .get_app_subnets()
                .is_empty()
        );
        assert_eq!(
            instance.close(Duration::from_secs(20)).await,
            Deletion::Attempted
        );
    });
    let address: SocketAddr = endpoint
        .trim_start_matches("http://")
        .trim_end_matches('/')
        .parse()
        .unwrap();
    assert!(TcpStream::connect_timeout(&address, Duration::from_millis(100)).is_err());
}
