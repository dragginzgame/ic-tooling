//! Explicit real-provider qualification. Requires a prepared ICP CLI and native
//! credential session. Creates/deletes only a uniquely named disposable identity.

#[cfg(target_os = "linux")]
mod linux;
#[cfg(target_os = "macos")]
mod mac_keychain;

#[cfg(target_os = "linux")]
use linux::Credential;
#[cfg(target_os = "macos")]
use mac_keychain::PrivateKeychain as Credential;

use std::{
    fs,
    os::unix::fs::PermissionsExt,
    path::{Path, PathBuf},
    process::Command,
};

use ic_host_sign::{
    Error, Signer,
    protocol::{self, Configuration, Operation, Request},
};

use zeroize::Zeroizing;

struct Disposable {
    credential: Credential,
    directory: tempfile::TempDir,
    config: Configuration,
    helper: PathBuf,
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
        let credential = Credential::create(directory.path(), &cli, &helper, &name);
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
            credential,
            config,
            helper,
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
        self.credential.delete_credential(&self.config.identity);
    }
}

fn create_identity(directory: &Path, cli: &Path, name: &str) {
    let created = Command::new(cli)
        .env("ICP_HOME", directory)
        .args(["identity", "new", name, "--storage", "keyring", "--json"])
        .current_dir(directory)
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
