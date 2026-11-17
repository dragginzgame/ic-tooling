use std::fs;
use std::os::unix::fs::PermissionsExt;
use std::time::{Duration, Instant};

use ic_platform::native::{Secp256k1Identity, UpdateArgs};
use ic_platform_types::Principal;

use super::{Signer, State, admit};
use crate::Error;
use crate::protocol::{self, Configuration, Operation, Request};

fn scratch() -> tempfile::TempDir {
    let target = std::path::Path::new(env!("CARGO_MANIFEST_DIR")).join("../../target");
    fs::create_dir_all(&target).unwrap();
    tempfile::tempdir_in(target).unwrap()
}

fn disposable_identity() -> Secp256k1Identity {
    let generated = std::process::Command::new("openssl")
        .args([
            "genpkey",
            "-algorithm",
            "EC",
            "-pkeyopt",
            "ec_paramgen_curve:secp256k1",
        ])
        .output()
        .unwrap();
    assert!(generated.status.success());
    let pem = String::from_utf8(generated.stdout).unwrap();
    Secp256k1Identity::from_pem(&pem, 16384).unwrap()
}

fn configuration(directory: &std::path::Path, sender: Principal) -> Configuration {
    let manifest = directory.join("identity_list.json");
    fs::write(&manifest, serde_json::to_vec(&serde_json::json!({"identities": {
        "disposable": {"kind": "keyring", "algorithm": "secp256k1", "principal": sender.to_text()}
    }})).unwrap()).unwrap();
    fs::set_permissions(&manifest, fs::Permissions::from_mode(0o600)).unwrap();
    Configuration {
        identity: "disposable".into(),
        manifest: manifest.to_str().unwrap().into(),
        sender: sender.to_text(),
        local_bus: if cfg!(target_os = "linux") {
            Some("unix:path=/unopened-test-bus".into())
        } else {
            None
        },
    }
}

fn request(config: &Configuration) -> Request {
    let now = protocol::now_ns().unwrap();
    Request {
        sender: config.sender.clone(),
        effective: "rrkah-fqaaa-aaaaa-aaaaq-cai".into(),
        expires_ns: now + 90_123_456_789,
        deadline_ns: now + 5_000_000_000,
        envelope_bytes: 4096,
        network_requests: 0,
        operation: Operation::Update {
            recipient: "aaaaa-aa".into(),
            method: "fixture".into(),
            arg: vec![0, 255, 16, 0],
            nonce: Some(vec![]),
        },
    }
}

fn fixture(script: &str) -> (tempfile::TempDir, Signer, Request) {
    let directory = scratch();
    let executable = directory.path().join("fixture");
    fs::write(&executable, format!("#!/bin/sh\n{script}\n")).unwrap();
    fs::set_permissions(&executable, fs::Permissions::from_mode(0o700)).unwrap();
    let mut bytes = [1; 29];
    bytes[28] = 2;
    let config = configuration(directory.path(), Principal::from_slice(&bytes));
    let request = request(&config);
    let hash = protocol::tool_hash(&executable).unwrap();
    let signer = Signer::new(&executable, hash, config).unwrap();
    (directory, signer, request)
}

#[tokio::test]
async fn dropped_sign_future_retains_cleanup_and_refuses_reentry() {
    let (_directory, mut signer, request) = fixture("cat >/dev/null\nexec sleep 3");
    let approved = signer.commitment();
    let timed = tokio::time::timeout(
        Duration::from_millis(80),
        signer.sign(request.clone(), approved),
    )
    .await;
    assert!(timed.is_err());
    assert!(matches!(
        signer.state(),
        State::Cancelling | State::AwaitingSettlement
    ));
    assert!(matches!(
        signer.sign(request, approved).await,
        Err(Error::Busy)
    ));
    let settled = tokio::time::timeout(Duration::from_secs(2), signer.settle())
        .await
        .unwrap();
    assert!(matches!(
        settled,
        Err(Error::Execution { .. }) | Err(Error::Cancelled)
    ));
    assert_eq!(signer.state(), State::Ready);
    assert!(matches!(signer.settle().await, Err(Error::Idle)));
}

#[tokio::test]
async fn signer_drop_joins_original_worker() {
    let (_directory, mut signer, request) = fixture("cat >/dev/null\nexec sleep 3");
    let approved = signer.commitment();
    assert!(
        tokio::time::timeout(Duration::from_millis(80), signer.sign(request, approved))
            .await
            .is_err()
    );
    let original = signer.active.as_ref().unwrap().shared.clone();
    let start = Instant::now();
    drop(signer);
    assert!(start.elapsed() < Duration::from_secs(2));
    assert_eq!(
        std::sync::Arc::strong_count(&original),
        1,
        "worker still holds custody"
    );
    assert!(original.completion.lock().unwrap().result.is_some());
}

#[tokio::test]
async fn completed_signed_bytes_survive_observer_drop_without_resigning() {
    use std::future::Future;
    use std::sync::{
        Arc,
        atomic::{AtomicBool, Ordering},
    };
    use std::task::{Context, Wake, Waker};
    struct Notified(AtomicBool);
    impl Wake for Notified {
        fn wake(self: Arc<Self>) {
            self.0.store(true, Ordering::Release);
        }
    }
    let (directory, initial, _) =
        fixture("cat >/dev/null\nprintf x >> \"$0.count\"\ncat \"$0.response\"");
    let identity = disposable_identity();
    let config = configuration(directory.path(), identity.principal());
    let request = request(&config);
    let mut signer = Signer::new(&initial.executable, initial.tool, config).unwrap();
    let approved = signer.commitment();
    let envelope = identity
        .prepare_update(
            UpdateArgs {
                recipient: Principal::management_canister(),
                effective: protocol::principal(&request.effective).unwrap(),
                method: "fixture",
                arg: &[0, 255, 16, 0],
                nonce: Some(&[]),
                expiry: request.expires_ns,
            },
            4096,
        )
        .unwrap()
        .envelope_bytes()
        .to_vec();
    let response = protocol::Response {
        protocol: protocol::PROTOCOL.into(),
        commitment: approved,
        envelope: envelope.clone(),
    };
    fs::write(
        directory.path().join("fixture.response"),
        protocol::encode_response(&response).unwrap(),
    )
    .unwrap();
    let notified = Arc::new(Notified(AtomicBool::new(false)));
    let waker = Waker::from(Arc::clone(&notified));
    let mut future = Box::pin(signer.sign(request.clone(), approved));
    assert!(
        future
            .as_mut()
            .poll(&mut Context::from_waker(&waker))
            .is_pending()
    );
    // Worker publishes its result before waking this observer. Deliberately do
    // not poll Ready: drop the observer at the timeout/completion race boundary.
    let start = Instant::now();
    while !notified.0.load(Ordering::Acquire) {
        assert!(start.elapsed() < Duration::from_secs(3));
        std::thread::sleep(Duration::from_millis(1));
    }
    drop(future);
    assert_eq!(signer.state(), State::AwaitingSettlement);
    assert!(matches!(
        signer.sign(request, approved).await,
        Err(Error::Busy)
    ));
    assert_eq!(signer.settle().await.unwrap().envelope_bytes(), envelope);
    assert_eq!(signer.state(), State::Ready);
    assert_eq!(
        fs::read(directory.path().join("fixture.count")).unwrap(),
        b"x"
    );
}

#[tokio::test]
async fn admission_refuses_before_dispatch() {
    let (directory, mut signer, mut request) = fixture("touch dispatched\nexit 1");
    let approved = signer.commitment();
    assert!(matches!(
        signer.sign(request.clone(), [0; 32]).await,
        Err(Error::Configuration)
    ));
    request.network_requests = 1;
    assert!(matches!(
        signer.sign(request.clone(), approved).await,
        Err(Error::Request)
    ));
    request.network_requests = 0;
    request.deadline_ns = 1;
    assert!(matches!(
        signer.sign(request.clone(), approved).await,
        Err(Error::Deadline)
    ));
    assert_eq!(signer.state(), State::Ready);
    assert!(!directory.path().join("dispatched").exists());
}

#[tokio::test]
async fn provider_failure_keeps_original_status_without_payloads() {
    let (_directory, mut signer, request) =
        fixture("cat >/dev/null\nprintf private-sentinel >&2\nexit 23");
    let approved = signer.commitment();
    let Err(error) = signer.sign(request, approved).await else {
        panic!("accepted failed provider")
    };
    assert!(!format!("{error:?} {error}").contains("private-sentinel"));
    let Error::Execution { report } = error else {
        panic!("lost process evidence")
    };
    assert_eq!(report.status.unwrap().code(), Some(23));
    assert!(report.failure.is_none());
    assert!(report.cleanup.is_empty());
    assert_eq!(signer.state(), State::Ready);
}

#[tokio::test]
async fn expired_completion_stops_and_reaps_the_provider() {
    let (_directory, mut signer, mut request) = fixture("cat >/dev/null\nexec sleep 3");
    request.deadline_ns = protocol::now_ns().unwrap() + 1_000_000_000;
    let approved = signer.commitment();
    let Err(Error::Execution { report }) = signer.sign(request, approved).await else {
        panic!("deadline accepted")
    };
    assert!(matches!(
        report.failure,
        Some(ic_host_process::Failure::Deadline)
    ));
    assert!(report.status.is_some());
    assert!(report.cleanup.is_empty());
    assert_eq!(signer.state(), State::Ready);
}

#[tokio::test]
async fn valid_frames_with_changed_protocol_or_commitment_refuse() {
    for changed_protocol in [true, false] {
        let (directory, mut signer, request) = fixture("cat >/dev/null\ncat \"$0.response\"");
        let approved = signer.commitment();
        let response = protocol::Response {
            protocol: if changed_protocol {
                "changed".into()
            } else {
                protocol::PROTOCOL.into()
            },
            commitment: if changed_protocol { approved } else { [0; 32] },
            envelope: vec![],
        };
        fs::write(
            directory.path().join("fixture.response"),
            protocol::encode_response(&response).unwrap(),
        )
        .unwrap();
        assert!(matches!(
            signer.sign(request, approved).await,
            Err(Error::Output)
        ));
        assert_eq!(signer.state(), State::Ready);
    }
}

#[tokio::test]
async fn truncated_wrong_commitment_and_oversized_output_are_refused() {
    for script in [
        "cat >/dev/null\nprintf '{'",
        "cat >/dev/null\nprintf '%s' '{\"protocol\":\"wrong\",\"commitment\":[0],\"envelope\":[]}'",
        "cat >/dev/null\nhead -c 33554433 /dev/zero",
    ] {
        let (_directory, mut signer, request) = fixture(script);
        let approved = signer.commitment();
        assert!(matches!(
            signer.sign(request, approved).await,
            Err(Error::Output | Error::Execution { .. })
        ));
        assert_eq!(signer.state(), State::Ready);
    }
}

#[tokio::test]
async fn changed_tool_or_selected_identity_refuses() {
    let (directory, mut signer, request) = fixture("cat >/dev/null\nexit 0");
    let approved = signer.commitment();
    fs::write(&signer.executable, "#!/bin/sh\nexit 24\n").unwrap();
    assert!(matches!(
        signer.sign(request, approved).await,
        Err(Error::Configuration)
    ));
    let hash = protocol::tool_hash(&signer.executable).unwrap();
    fs::write(directory.path().join("identity_list.json"), "{}").unwrap();
    assert!(matches!(
        Signer::new(&signer.executable, hash, signer.config.clone()),
        Err(Error::Configuration)
    ));
}

#[test]
fn remote_or_fallback_bus_is_never_admitted() {
    let directory = scratch();
    let mut bytes = [1; 29];
    bytes[28] = 2;
    let mut config = configuration(directory.path(), Principal::from_slice(&bytes));
    for address in [
        "tcp:host=127.0.0.1,port=12345",
        "autolaunch:",
        "unix:path=/tmp/bus;tcp:host=x,port=1",
    ] {
        config.local_bus = Some(address.into());
        assert!(config.admit().is_err());
    }
}

#[test]
fn platform_authenticated_update_preserves_exact_binding_and_refuses_mutations() {
    // Generate disposable material directly in memory. No embedded key fixture,
    // keyring access, operator data or external network is involved in this test.
    let generated = std::process::Command::new("openssl")
        .args([
            "genpkey",
            "-algorithm",
            "EC",
            "-pkeyopt",
            "ec_paramgen_curve:secp256k1",
        ])
        .output()
        .unwrap();
    assert!(generated.status.success());
    let pem = String::from_utf8(generated.stdout).unwrap();
    let identity = Secp256k1Identity::from_pem(&pem, 16384).unwrap();
    let directory = scratch();
    let config = configuration(directory.path(), identity.principal());
    let original = request(&config);
    let effective = protocol::principal(&original.effective).unwrap();
    let envelope = identity
        .prepare_update(
            UpdateArgs {
                recipient: Principal::management_canister(),
                effective,
                method: "fixture",
                arg: &[0, 255, 16, 0],
                nonce: Some(&[]),
                expiry: original.expires_ns,
            },
            4096,
        )
        .unwrap()
        .envelope_bytes()
        .to_vec();
    let admitted = admit(&original, envelope.clone()).unwrap();
    assert_eq!(admitted.envelope_bytes(), envelope);
    assert_eq!(admitted.sender(), identity.principal());
    assert_eq!(admitted.effective_canister(), effective);
    assert_eq!(admitted.expiry(), original.expires_ns);
    let mut wrong = original.clone();
    wrong.expires_ns += 1;
    assert!(admit(&wrong, envelope.clone()).is_err());
    let mut wrong = original.clone();
    let mut other_sender = [1; 29];
    other_sender[28] = 2;
    wrong.sender = Principal::from_slice(&other_sender).to_text();
    assert!(admit(&wrong, envelope.clone()).is_err());
    let mut wrong = original.clone();
    wrong.envelope_bytes = envelope.len() - 1;
    assert!(admit(&wrong, envelope.clone()).is_err());
    for operation in [
        Operation::Update {
            recipient: original.effective.clone(),
            method: "fixture".into(),
            arg: vec![0, 255, 16, 0],
            nonce: Some(vec![]),
        },
        Operation::Update {
            recipient: "aaaaa-aa".into(),
            method: "changed".into(),
            arg: vec![0, 255, 16, 0],
            nonce: Some(vec![]),
        },
        Operation::Update {
            recipient: "aaaaa-aa".into(),
            method: "fixture".into(),
            arg: vec![0, 255, 16, 1],
            nonce: Some(vec![]),
        },
        Operation::Update {
            recipient: "aaaaa-aa".into(),
            method: "fixture".into(),
            arg: vec![0, 255, 16, 0],
            nonce: None,
        },
    ] {
        let mut wrong = original.clone();
        wrong.operation = operation;
        assert!(admit(&wrong, envelope.clone()).is_err());
    }
    let mut corrupt = envelope.clone();
    let last = corrupt.len() - 1;
    corrupt[last] ^= 1;
    assert!(admit(&original, corrupt).is_err());
    assert_eq!(
        admit(&original, envelope).unwrap().request_id(),
        admitted.request_id()
    );
}

#[test]
fn independent_status_authentication_preserves_original_id_and_refuses_mismatches() {
    let identity = disposable_identity();
    let directory = scratch();
    let config = configuration(directory.path(), identity.principal());
    let mut original = request(&config);
    let id = [71; 32];
    original.operation = Operation::Status { request_id: id };
    let effective = protocol::principal(&original.effective).unwrap();
    let envelope = identity
        .prepare_request_status(id, effective, original.expires_ns, 4096)
        .unwrap()
        .envelope_bytes()
        .to_vec();
    original.envelope_bytes = envelope.len();
    let admitted = admit(&original, envelope.clone()).unwrap();
    assert_eq!(admitted.request_id(), &id);
    assert_eq!(admitted.sender(), identity.principal());
    assert_eq!(admitted.expiry(), original.expires_ns);
    assert_eq!(admitted.effective_canister(), effective);
    assert_eq!(admitted.envelope_bytes(), envelope);
    let mut wrong = original.clone();
    wrong.expires_ns += 1;
    assert!(admit(&wrong, envelope.clone()).is_err());
    let mut wrong = original.clone();
    wrong.envelope_bytes -= 1;
    assert!(admit(&wrong, envelope.clone()).is_err());
    let mut wrong = original.clone();
    wrong.operation = Operation::Status {
        request_id: [72; 32],
    };
    assert!(admit(&wrong, envelope.clone()).is_err());
    let other = disposable_identity();
    let mut wrong = original.clone();
    wrong.sender = other.principal().to_text();
    assert!(admit(&wrong, envelope.clone()).is_err());
    let mut corrupt = envelope.clone();
    let last = corrupt.len() - 1;
    corrupt[last] ^= 1;
    assert!(admit(&original, corrupt).is_err());
    let mut trailing = envelope;
    trailing.push(0);
    assert!(admit(&original, trailing).is_err());
    let update = identity
        .prepare_update(
            UpdateArgs {
                recipient: Principal::management_canister(),
                effective,
                method: "fixture",
                arg: &[0, 255, 16, 0],
                nonce: Some(&[]),
                expiry: original.expires_ns,
            },
            4096,
        )
        .unwrap()
        .envelope_bytes()
        .to_vec();
    original.envelope_bytes = 4096;
    assert!(admit(&original, update).is_err());
}
