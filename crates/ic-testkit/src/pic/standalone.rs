use pocket_ic::CanisterId as Principal;
use pocket_ic::nonblocking::PocketIc;

use super::{InstallSpec, PocketIcInstance};

/// One caller-built `PocketIC` instance/server owner and one installed canister.
///
/// This fixture retains the instance and its optional managed server together.
/// It does not impose process-wide synchronization or hide `PocketIC` methods.
pub struct StandaloneCanisterFixture {
    instance: PocketIcInstance,
    canister_id: Principal,
}

impl StandaloneCanisterFixture {
    /// Install one canister into a caller-configured `PocketIC` instance.
    ///
    /// Upstream installation panics propagate. Unwinding drops this record's
    /// managed server; an externally owned server requires caller reconciliation.
    pub async fn install(instance: PocketIcInstance, spec: InstallSpec) -> Self {
        let canister_id = super::create_and_install(instance.instance(), spec).await;

        Self {
            instance,
            canister_id,
        }
    }

    /// Borrow the `PocketIC` instance that owns this standalone fixture.
    #[must_use]
    pub const fn pocket_ic(&self) -> &PocketIc {
        self.instance.instance()
    }

    /// Read the installed canister id for this standalone fixture.
    #[must_use]
    pub const fn canister_id(&self) -> Principal {
        self.canister_id
    }

    /// Consume the fixture and return the instance/server ownership record and canister id.
    pub fn into_parts(self) -> (PocketIcInstance, Principal) {
        (self.instance, self.canister_id)
    }
}
