//! Explicitly disposable account only: restore defaults/search list on exit.
//! Preauthorize an empty test item, then let ICP populate its actual key in place.
//! No operator keychain, private-key export or password argv is used.

use std::path::{Path, PathBuf};
use std::process::Command;

use security_framework::os::macos::keychain::CreateOptions;

pub(super) struct PrivateKeychain {
    path: PathBuf,
    previous: PathBuf,
    search: Vec<PathBuf>,
}

fn paths(arguments: &[&str]) -> Vec<PathBuf> {
    let output = Command::new("/usr/bin/security")
        .args(arguments)
        .output()
        .unwrap();
    assert!(
        output.status.success(),
        "cannot snapshot public keychain preferences"
    );
    String::from_utf8(output.stdout)
        .unwrap()
        .lines()
        .map(|line| {
            let value = line
                .trim()
                .strip_prefix('"')
                .and_then(|s| s.strip_suffix('"'))
                .unwrap();
            let path = PathBuf::from(value);
            assert!(path.is_absolute());
            path
        })
        .collect()
}

impl PrivateKeychain {
    pub(super) fn create(directory: &Path, icp: &Path, helper: &Path, name: &str) -> Self {
        assert_eq!(
            std::env::var("SIGNER_TEST_DISPOSABLE_KEYCHAIN").as_deref(),
            Ok("1"),
            "use a disposable account for native keychain qualification"
        );
        let previous = paths(&["default-keychain", "-d", "user"]);
        assert_eq!(previous.len(), 1);
        let search = paths(&["list-keychains", "-d", "user"]);
        let path = directory.join("fixture.keychain-db");
        let _created = CreateOptions::new()
            .password("disposable-native-keychain-only")
            .prompt_user(false)
            .create(&path)
            .unwrap();
        let owner = Self {
            path,
            previous: previous[0].clone(),
            search,
        };
        assert!(
            Command::new("/usr/bin/security")
                .args(["default-keychain", "-d", "user", "-s"])
                .arg(&owner.path)
                .status()
                .unwrap()
                .success()
        );
        // No -w/-p: Apple's CLI creates empty data without prompting. The
        // existing ICP writer then updates that data without replacing its ACL.
        // Trust only the selected producer/helper and this fixture cleanup owner.
        let prepared = Command::new("/usr/bin/security")
            .args(["add-generic-password", "-a", name, "-s", "icp-cli", "-T"])
            .arg(helper)
            .arg("-T")
            .arg(icp)
            .arg("-T")
            .arg(std::env::current_exe().unwrap())
            .arg(&owner.path)
            .output()
            .unwrap();
        assert!(
            prepared.status.success(),
            "could not prepare the disposable item's narrow application ACL"
        );
        owner
    }

    pub(super) fn lock(&self) {
        assert!(
            Command::new("/usr/bin/security")
                .arg("lock-keychain")
                .arg(&self.path)
                .status()
                .unwrap()
                .success()
        );
    }
}

impl Drop for PrivateKeychain {
    fn drop(&mut self) {
        let restored = Command::new("/usr/bin/security")
            .args(["default-keychain", "-d", "user", "-s"])
            .arg(&self.previous)
            .output();
        let searched = Command::new("/usr/bin/security")
            .args(["list-keychains", "-d", "user", "-s"])
            .args(&self.search)
            .output();
        let deleted = Command::new("/usr/bin/security")
            .arg("delete-keychain")
            .arg(&self.path)
            .output();
        if !std::thread::panicking() {
            assert!(
                restored.is_ok_and(|s| s.status.success())
                    && searched.is_ok_and(|s| s.status.success())
                    && deleted.is_ok_and(|s| s.status.success()),
                "disposable keychain cleanup/ref restoration failed"
            );
        }
    }
}
