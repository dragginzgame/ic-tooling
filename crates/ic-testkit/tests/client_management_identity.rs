//! Public management DTO identity and Candid round-trip qualification.
#![cfg(not(target_family = "wasm"))]

#[test]
fn client_management_types_have_the_workspace_identity() {
    let external = ic_management_canister_types::CanisterSettings {
        status_visibility: Some(ic_management_canister_types::StatusVisibility::Public),
        minimum_incoming_canister_call_cycles: Some(123_u64.into()),
        ..Default::default()
    };
    let client: ic_testkit::pocket_ic::CanisterSettings = external.clone();
    assert_eq!(client, external);
    let facade: ic_platform::management_canister_types::CanisterSettings = client.clone();
    assert_eq!(facade, external);
    let bytes = candid::encode_one(&client).unwrap();
    let decoded: ic_management_canister_types::CanisterSettings =
        candid::decode_one(&bytes).unwrap();
    assert_eq!(decoded, external);
}
