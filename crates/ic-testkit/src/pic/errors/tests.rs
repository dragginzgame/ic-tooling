use crate::pocket_ic::ClientError;
use ic_platform_types::principal::ExternalPrincipal as Principal;

use super::CanisterInstallError;

#[test]
fn labeled_install_error_display_includes_label() {
    let err = CanisterInstallError::InstallCode {
        canister_id: Principal::anonymous(),
        label: Some("authority".into()),
        source: ClientError::Protocol("trap".into()),
    };
    assert!(
        err.to_string()
            .contains("(authority): invalid PocketIC response: trap")
    );
}
