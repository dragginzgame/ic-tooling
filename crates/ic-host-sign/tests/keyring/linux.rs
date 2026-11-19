use std::{path::Path, process::Command};

use ic_host_sign::{Error, protocol};

use super::{Disposable, create_identity};

pub(super) struct Credential {
    entry: keyring::Entry,
    delete_on_drop: bool,
}

impl Credential {
    pub(super) fn create(directory: &Path, cli: &Path, _helper: &Path, name: &str) -> Self {
        create_identity(directory, cli, name);

        Self {
            entry: keyring::Entry::new("icp-cli", name)
                .expect("native credential session unavailable"),
            delete_on_drop: true,
        }
    }

    pub(super) fn delete_credential(&self, _name: &str) {
        self.entry
            .delete_credential()
            .expect("disposable credential deletion failed");
    }
}

impl Drop for Credential {
    fn drop(&mut self) {
        if self.delete_on_drop {
            let _ = self.entry.delete_credential();
        }
    }
}

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
    fixture.credential.delete_on_drop = false;
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
