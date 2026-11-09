//! Owned management identity and independently frozen management bytes.
#![cfg(not(target_family = "wasm"))]

use ic_platform::{
    Principal,
    candid_codec::{
        decoder::{Limits, decode_one},
        encode_one,
    },
    management_canister_types::{CanisterIdRecord, CanisterSettings, StatusVisibility},
};

const LIMITS: Limits = Limits {
    bytes: 65536,
    header: 4096,
    types: 128,
    work: 100_000,
    allocation: 100_000,
};

#[test]
fn client_management_types_have_the_workspace_identity() {
    let owned = CanisterSettings {
        status_visibility: Some(StatusVisibility::Public),
        minimum_incoming_canister_call_cycles: Some(123_u64.into()),
        ..Default::default()
    };
    let client: ic_testkit::pocket_ic::CanisterSettings = owned.clone();
    let bytes = encode_one(&client, LIMITS.bytes).unwrap();
    let decoded: CanisterSettings = decode_one(&bytes, LIMITS).unwrap();
    assert_eq!(decoded, owned);
}

#[test]
fn owned_principal_preserves_frozen_management_argument() {
    // Registry management 0.11.0 fixture, also preserved by Platform's
    // management-snapshot-target.hex. No registry DTO is needed to decode it.
    const WIRE: &[u8] = b"DIDL\x01\x6c\x01\xb3\xc4\xb1\xf2\x04\x68\x01\x00\x01\x01\x04";
    let record: CanisterIdRecord = decode_one(WIRE, LIMITS).unwrap();
    assert_eq!(record.canister_id, Principal::anonymous());
    assert_eq!(encode_one(&record, LIMITS.bytes).unwrap(), WIRE);
}
