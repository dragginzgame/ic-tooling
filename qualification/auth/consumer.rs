//! Exact packaged Auth interfaces and canonical principal identity.
pub use ic_auth;
pub use ic_auth_protocol_types;
pub use ic_auth_wire;

pub fn canonical_principal(
    principal: ic_platform_types::Principal,
) -> ic_auth_protocol_types::Principal {
    principal
}
