//! Noninteractive access to the selected existing native credential.
//! No unlock, prompt, item creation or fallback identity store is introduced.

use zeroize::Zeroizing;

#[cfg(target_os = "linux")]
pub(super) fn read(name: &str) -> Result<Zeroizing<Vec<u8>>, ()> {
    use secret_service::{EncryptionType, blocking::SecretService};
    use std::collections::HashMap;
    let service = SecretService::connect(EncryptionType::Dh).map_err(|_| ())?;
    // Existing ICP/keyring attributes; reject ambiguous or locked selections.
    // GetSecret refuses a concurrent lock without creating an unlock prompt.
    let found = service
        .search_items(HashMap::from([("service", "icp-cli"), ("username", name)]))
        .map_err(|_| ())?;
    if !found.locked.is_empty() || found.unlocked.len() != 1 {
        return Err(());
    }
    found.unlocked[0]
        .get_secret()
        .map(Zeroizing::new)
        .map_err(|_| ())
}

#[cfg(target_os = "macos")]
pub(super) fn read(name: &str) -> Result<Zeroizing<Vec<u8>>, ()> {
    use security_framework::os::macos::keychain::SecKeychain;
    // This safe upstream guard is process-local. Locked/ACL-denied credentials
    // fail; the helper never launches an OS authentication dialog.
    let _no_ui = SecKeychain::disable_user_interaction().map_err(|_| ())?;
    let entry = keyring::Entry::new("icp-cli", name).map_err(|_| ())?;
    entry.get_secret().map(Zeroizing::new).map_err(|_| ())
}
