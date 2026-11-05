use crate::icp::{CanisterListError, decode_canister_list};

#[test]
fn inventory_preserves_order_duplicates_empty_names_and_ignored_metadata() {
    let input = br#"{"canisters":["b","a","b",""],"extra":{"nested":[1,null]}}"#;
    assert_eq!(
        decode_canister_list(input, input.len()).unwrap(),
        ["b", "a", "b", ""]
    );
    assert_eq!(
        decode_canister_list(br#"{"canisters":[]}"#, 64).unwrap(),
        Vec::<String>::new()
    );
    assert!(matches!(
        decode_canister_list(input, input.len() - 1),
        Err(CanisterListError::InputLimit { .. })
    ));
    // Byte admission precedes even malformed-input parsing.
    assert!(matches!(
        decode_canister_list(b"[", 0),
        Err(CanisterListError::InputLimit { limit: 0 })
    ));
}

#[test]
fn inventory_refuses_missing_null_repeated_wrong_type_and_incomplete_json() {
    for input in [
        b"{}".as_slice(),
        b"null",
        b"[]",
        b"[[]]",
        b"{",
        b"",
        br#"{"canisters":null}"#,
        br#"{"canisters":42}"#,
        br#"{"canisters":["a",42]}"#,
        br#"{"canisters":[],"canisters":[]}"#,
        br#"{"canisters":[]} trailing"#,
        br#"{"canisters":[]} {}"#,
    ] {
        assert!(
            matches!(
                decode_canister_list(input, input.len()),
                Err(CanisterListError::Json(_))
            ),
            "{input:?}"
        );
    }
}

#[test]
fn complete_input_must_be_utf8_even_in_ignored_metadata() {
    for input in [
        b"{\"canisters\":[\"\xff\"]}".as_slice(),
        b"{\"canisters\":[],\"extra\":\"\xff\"}",
    ] {
        assert!(matches!(
            decode_canister_list(input, input.len()),
            Err(CanisterListError::Utf8(_))
        ));
    }
}
