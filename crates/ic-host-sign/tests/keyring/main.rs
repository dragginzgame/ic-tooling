//! Explicit real-provider qualification. Requires a prepared ICP CLI and native
//! credential session. Creates/deletes only a uniquely named disposable identity.

#[cfg(target_os = "macos")]
mod mac_keychain;

use std::{fs, os::unix::fs::PermissionsExt, path::PathBuf, process::Command};

use ic_host_sign::{
    Error, Signer,
    protocol::{self, Configuration, Operation, Request},
};

use zeroize::Zeroizing;

struct Disposable {
    #[cfg(target_os = "linux")]
    entry: Credential,
    #[cfg(target_os = "macos")]
    keychain: mac_keychain::PrivateKeychain,
    directory: tempfile::TempDir,
    config: Configuration,
    helper: PathBuf,
}

#[cfg(target_os = "linux")]
struct Credential(keyring::Entry, bool);
#[cfg(target_os = "linux")]
impl Drop for Credential {
    fn drop(&mut self) {
        if self.1 {
            let _ = self.0.delete_credential();
        }
    }
}

impl Disposable {
    fn create() -> Self {
        let directory = tempfile::tempdir_in(env!("CARGO_TARGET_TMPDIR")).unwrap();
        let cli = PathBuf::from(
            std::env::var_os("SIGNER_TEST_ICP").expect("select prepared ICP CLI 1.7.0"),
        );
        assert!(cli.is_absolute());
        let version = Command::new(&cli).arg("--version").output().unwrap();
        assert_eq!(version.stdout, b"icp 1.7.0\n");
        let name = format!(
            "tooling-disposable-{}-{}",
            std::process::id(),
            protocol::now_ns().unwrap()
        );
        let helper = directory.path().join("ic-sign");
        let selected = std::env::var_os("SIGNER_TEST_HELPER").map_or_else(
            || PathBuf::from(env!("CARGO_BIN_EXE_ic-sign")),
            PathBuf::from,
        );
        assert!(selected.is_absolute());
        fs::copy(&selected, &helper).unwrap();
        #[cfg(target_os = "linux")]
        assert!(
            Command::new("strip")
                .arg("--strip-debug")
                .arg(&helper)
                .status()
                .unwrap()
                .success()
        );
        fs::set_permissions(&helper, fs::Permissions::from_mode(0o700)).unwrap();
        #[cfg(target_os = "macos")]
        let keychain =
            mac_keychain::PrivateKeychain::create(directory.path(), &cli, &helper, &name);
        let created = Command::new(cli)
            .env("ICP_HOME", directory.path())
            .args(["identity", "new", &name, "--storage", "keyring", "--json"])
            .current_dir(directory.path())
            .output()
            .unwrap();
        let status = created.status;
        // `identity new` prints a disposable recovery seed. Never emit or retain
        // it as evidence, including on failure. No private operator data is read.
        drop(Zeroizing::new(created.stdout));
        drop(Zeroizing::new(created.stderr));
        assert!(
            status.success(),
            "disposable ICP keyring identity creation failed"
        );
        #[cfg(target_os = "linux")]
        let entry = Credential(
            keyring::Entry::new("icp-cli", &name).expect("native credential session unavailable"),
            true,
        );
        let manifest = directory.path().join("identity/identity_list.json");
        let metadata: serde_json::Value =
            serde_json::from_slice(&fs::read(&manifest).unwrap()).unwrap();
        let selected = &metadata["identities"][&name];
        assert_eq!(selected["kind"], "keyring");
        assert_eq!(selected["algorithm"], "secp256k1");
        let config = Configuration {
            identity: name,
            manifest: manifest.to_str().unwrap().into(),
            sender: selected["principal"].as_str().unwrap().into(),
            local_bus: if cfg!(target_os = "linux") {
                Some(std::env::var("DBUS_SESSION_BUS_ADDRESS").unwrap())
            } else {
                None
            },
        };
        Self {
            directory,
            #[cfg(target_os = "linux")]
            entry,
            config,
            helper,
            #[cfg(target_os = "macos")]
            keychain,
        }
    }

    fn request(&self) -> Request {
        let now = protocol::now_ns().unwrap();
        Request {
            sender: self.config.sender.clone(),
            effective: "rrkah-fqaaa-aaaaa-aaaaq-cai".into(),
            expires_ns: now + 90_123_456_789,
            deadline_ns: now + 60_000_000_000,
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

    fn signer(&self) -> Signer {
        let tool = protocol::tool_hash(&self.helper).unwrap();
        println!("Disposable helper SHA-256: {tool:02x?}");
        Signer::new(&self.helper, tool, self.config.clone()).unwrap()
    }

    fn delete_credential(&self) {
        #[cfg(target_os = "linux")]
        self.entry
            .0
            .delete_credential()
            .expect("disposable credential deletion failed");
        #[cfg(target_os = "macos")]
        self.keychain.delete_credential(&self.config.identity);
    }
}

#[tokio::test]
#[ignore = "requires prepared ICP 1.7.0 and a disposable native OS-keyring session"]
async fn native_icp_keyring_update() {
    let fixture = Disposable::create();
    let mut signer = fixture.signer();
    let request = fixture.request();
    let approved = signer.commitment();
    let signed = signer
        .sign(request.clone(), approved)
        .await
        .expect("actual keyring provider failed");
    assert_eq!(
        signed.sender().to_text(),
        fixture.config.sender,
        "principal differs from ICP CLI"
    );
    assert_eq!(signed.effective_canister().to_text(), request.effective);
    assert_eq!(signed.expiry(), request.expires_ns);
    let original = signed.envelope_bytes().to_vec();
    assert!(original.len() <= request.envelope_bytes);
    // Parent admitted the direct signature and exact method/raw bytes/nonce,
    // independently of helper response metadata. No IC endpoint is contacted.
    assert_eq!(signer.state(), ic_host_sign::State::Ready);

    // Only the original ID crosses the status boundary; neither the original
    // update envelope nor private key material is supplied to this operation.
    let mut lookup = fixture.request();
    lookup.expires_ns += 17_234_567_891;
    lookup.operation = Operation::Status {
        request_id: *signed.request_id(),
    };
    let status = signer
        .sign(lookup.clone(), approved)
        .await
        .expect("real keyring status signing failed");
    assert_eq!(status.request_id(), signed.request_id());
    assert_eq!(status.sender(), signed.sender());
    assert_eq!(status.effective_canister(), signed.effective_canister());
    assert_eq!(status.expiry(), lookup.expires_ns);
    assert_ne!(status.envelope_bytes(), original);
    assert_eq!(signed.envelope_bytes(), original);

    fixture.delete_credential();
    let next = fixture.request();
    assert!(matches!(
        signer.sign(next, approved).await,
        Err(Error::Execution { .. })
    ));
    assert_eq!(signed.envelope_bytes(), original);
    assert!(
        !fixture.directory.path().join("identity/keys").exists(),
        "unexpected plaintext key export"
    );
}

#[cfg(target_os = "linux")]
#[tokio::test]
#[ignore = "requires the qualification runner's owned disposable Secret Service"]
async fn native_keyring_cancellation_retains_custody() {
    use std::time::Duration;
    assert_eq!(
        std::env::var("SIGNER_TEST_PRIVATE_SESSION").as_deref(),
        Ok("1")
    );
    let daemon = std::env::var("SIGNER_TEST_KEYRING_PID").expect("owned test daemon PID");
    assert!(daemon.bytes().all(|c| c.is_ascii_digit()));
    let fixture = Disposable::create();
    let mut signer = fixture.signer();
    let mut request = fixture.request();
    request.deadline_ns = protocol::now_ns().unwrap() + 20_000_000_000;
    let approved = signer.commitment();
    // Freeze only the daemon launched by this disposable-session runner. The
    // actual helper blocks in local native credential IPC, without fake keys or
    // fake signer output. The runner owns this backend, not the signer library.
    assert!(
        Command::new("kill")
            .args(["-STOP", &daemon])
            .status()
            .unwrap()
            .success()
    );
    struct Resume(String);
    impl Drop for Resume {
        fn drop(&mut self) {
            let _ = Command::new("kill").args(["-CONT", &self.0]).status();
        }
    }
    let resume = Resume(daemon);
    let timed = tokio::time::timeout(
        Duration::from_secs(3),
        signer.sign(request.clone(), approved),
    )
    .await;
    assert!(
        timed.is_err(),
        "credential IPC unexpectedly completed while its daemon was stopped"
    );
    assert!(matches!(
        signer.sign(request, approved).await,
        Err(Error::Busy)
    ));
    let settled = tokio::time::timeout(Duration::from_secs(3), signer.settle())
        .await
        .unwrap();
    let Err(Error::Execution { report }) = settled else {
        panic!("lost native cancellation evidence")
    };
    assert!(matches!(
        report.failure,
        Some(ic_host_process::Failure::Cancelled)
    ));
    assert!(report.cleanup.is_empty());
    assert!(report.status.is_some(), "helper was not reaped");
    assert_eq!(signer.state(), ic_host_sign::State::Ready);
    drop(resume);
    fixture.delete_credential();
}

#[cfg(target_os = "linux")]
#[tokio::test]
#[ignore = "requires the qualification runner's private headless Secret Service"]
async fn native_locked_keyring_refuses_without_changing_identity() {
    use std::time::Duration;
    assert_eq!(
        std::env::var("SIGNER_TEST_PRIVATE_SESSION").as_deref(),
        Ok("1")
    );
    let mut fixture = Disposable::create();
    let mut signer = fixture.signer();
    let approved = signer.commitment();
    let signed = signer.sign(fixture.request(), approved).await.unwrap();
    let original = signed.envelope_bytes().to_vec();
    // After locking, only the runner-owned store teardown has cleanup custody.
    // A convenience credential destructor must not itself request an unlock UI.
    fixture.entry.1 = false;
    let locked = Command::new("gdbus")
        .args([
            "call",
            "--session",
            "--dest",
            "org.freedesktop.secrets",
            "--object-path",
            "/org/freedesktop/secrets",
            "--method",
            "org.freedesktop.Secret.Service.Lock",
            "['/org/freedesktop/secrets/collection/login']",
        ])
        .output()
        .unwrap();
    assert!(
        locked.status.success(),
        "could not lock the private collection"
    );
    let check = Command::new("gdbus")
        .args([
            "call",
            "--session",
            "--dest",
            "org.freedesktop.secrets",
            "--object-path",
            "/org/freedesktop/secrets/collection/login",
            "--method",
            "org.freedesktop.DBus.Properties.Get",
            "org.freedesktop.Secret.Collection",
            "Locked",
        ])
        .output()
        .unwrap();
    assert!(check.status.success() && String::from_utf8(check.stdout).unwrap().contains("true"));
    let attempt = tokio::time::timeout(
        Duration::from_secs(3),
        signer.sign(fixture.request(), approved),
    )
    .await;
    let result = match attempt {
        Ok(result) => result,
        Err(_) => {
            let _ = signer.settle().await;
            panic!("noninteractive provider did not promptly refuse the locked credential");
        }
    };
    let Err(Error::Execution { report }) = result else {
        panic!("locked keyring unexpectedly signed")
    };
    assert!(report.failure.is_none(), "provider did not refuse normally");
    assert_eq!(report.status.unwrap().code(), Some(2));
    assert!(report.cleanup.is_empty());
    assert_eq!(signer.state(), ic_host_sign::State::Ready);
    let matches = Command::new("gdbus")
        .args([
            "call",
            "--session",
            "--dest",
            "org.freedesktop.secrets",
            "--object-path",
            "/org/freedesktop/secrets",
            "--method",
            "org.freedesktop.Secret.Service.SearchItems",
            &format!(
                "{{'service': 'icp-cli', 'username': '{}'}}",
                fixture.config.identity
            ),
        ])
        .output()
        .unwrap();
    assert!(matches.status.success());
    assert!(
        String::from_utf8(matches.stdout)
            .unwrap()
            .contains("/org/freedesktop/secrets/collection/login/")
    );
    fixture.config.admit().unwrap();
    assert_eq!(signed.sender().to_text(), fixture.config.sender);
    assert_eq!(signed.envelope_bytes(), original);
}

#[cfg(target_os = "macos")]
#[tokio::test]
#[ignore = "requires the qualification runner's disposable keychain account"]
async fn native_locked_keychain_refuses_without_prompting() {
    use std::time::Duration;
    let fixture = Disposable::create();
    let mut signer = fixture.signer();
    let approved = signer.commitment();
    let signed = signer.sign(fixture.request(), approved).await.unwrap();
    let original = signed.envelope_bytes().to_vec();
    fixture.keychain.lock();
    let refused = tokio::time::timeout(
        Duration::from_secs(3),
        signer.sign(fixture.request(), approved),
    )
    .await;
    let result = match refused {
        Ok(result) => result,
        Err(_) => {
            let _ = signer.settle().await;
            panic!("locked keychain did not refuse without interaction");
        }
    };
    let Err(Error::Execution { report }) = result else {
        panic!("locked keychain unexpectedly signed")
    };
    assert!(report.failure.is_none() && report.cleanup.is_empty());
    assert_eq!(report.status.unwrap().code(), Some(2));
    assert_eq!(signer.state(), ic_host_sign::State::Ready);
    assert_eq!(signed.envelope_bytes(), original);
    assert_eq!(signed.sender().to_text(), fixture.config.sender);
}

#[cfg(target_os = "macos")]
#[test]
#[ignore = "requires the qualification runner's inherited network-denying sandbox"]
fn native_network_sandbox_denies_outbound() {
    assert_eq!(
        std::env::var("SIGNER_TEST_NETWORK_SANDBOX").as_deref(),
        Ok("1")
    );
    let failure = std::net::TcpStream::connect((std::net::Ipv4Addr::LOCALHOST, 9)).unwrap_err();
    assert_eq!(failure.kind(), std::io::ErrorKind::PermissionDenied);
}
