//! Public contracts shared by the independently released Tooling packages.

#[test]
fn build_keeps_its_public_foundation_error_identities() {
    use ic_build::artifacts::candid::ExtractionError;

    let _: fn(ic_host_artifacts::artifact::ArtifactError) -> ExtractionError =
        ExtractionError::Input;
    let _: fn(ic_host_process::tool::ToolError) -> ExtractionError = ExtractionError::Tool;
}

#[test]
fn ledger_blocks_keep_the_platform_value_identity() {
    let block = ic_host_tools::ledger_evidence::LedgerBlock {
        id: 0_u8.into(),
        block: ic_platform::icrc_ledger_types::Value::Nat(7_u8.into()),
    };
    assert_eq!(
        block.block,
        ic_platform::icrc_ledger_types::Value::Nat(7_u8.into())
    );
}

#[test]
fn tooling_callers_share_principal_and_wire_contracts() {
    let owner: ic_platform_types::Principal =
        ic_host_tools::registry::routing_shards::canister_range_start_from_key(
            "prefix/",
            "prefix/00000000000000000101",
        )
        .unwrap();
    let facade: ic_platform::Principal = owner;
    let upstream = pocket_ic::CanisterId::from_slice(owner.as_slice());
    assert_eq!(upstream.as_slice(), owner.as_slice());
    let _: ic_platform_types::Principal = facade;
    let limits = ic_wire::Limits {
        bytes: 1024,
        allocation: 16384,
        nodes: 1024,
        depth: 16,
    };
    let call = ic_testkit_wire::WireCallSpec {
        method: "probe",
        method_id: 1,
        request_limits: limits,
        response_limits: limits,
    };
    let _: ic_wire::Limits = call.request_limits;
    let _: ic_wire::Limits = call.response_limits;
    let request = ic_wire::native::encode_request(call.method_id, &facade, limits).unwrap();
    let decoded: ic_platform::Principal =
        ic_wire::native::decode_request(call.method_id, &request, limits).unwrap();
    assert_eq!(decoded, facade);
    let account = ic_platform::Account {
        owner: facade,
        subaccount: None,
    };
    let response = ic_wire::native::encode_response(call.method_id, &account, limits).unwrap();
    let decoded: ic_platform::Account =
        ic_wire::native::decode_response(call.method_id, &response, limits).unwrap();
    assert_eq!(decoded, account);
    assert!(decoded.subaccount.is_none());
}
