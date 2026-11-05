use crate::icp::{CycleBalanceError, decode_canister_cycles};

#[test]
fn cycles_preserve_exact_range_and_distinguish_public_absence_from_zero() {
    for cycles in [
        "0",
        "0001",
        "001_000",
        "1_418_380_664_222",
        "18_446_744_073_709_551_616",
        "340_282_366_920_938_463_463_374_607_431_768_211_455",
        "18446744073709551616",
        "340282366920938463463374607431768211455",
    ] {
        let input =
            format!(r#"{{"cycles":"{cycles}","module_hash":null,"extra":[1,{{"x":true}}]}}"#);
        assert_eq!(
            decode_canister_cycles(input.as_bytes(), input.len()).unwrap(),
            Some(cycles.replace('_', "").parse().unwrap())
        );
        assert!(matches!(
            decode_canister_cycles(input.as_bytes(), input.len() - 1),
            Err(CycleBalanceError::InputLimit { .. })
        ));
    }
    for input in [
        b"{}".as_slice(),
        br#"{"module_hash":"0x123","controllers":[]}"#,
    ] {
        assert_eq!(decode_canister_cycles(input, input.len()).unwrap(), None);
    }
    assert!(matches!(
        decode_canister_cycles(b"[", 0),
        Err(CycleBalanceError::InputLimit { limit: 0 })
    ));
}

#[test]
fn supplied_cycle_strings_must_be_exact_unsigned_decimal() {
    for cycles in [
        "",
        "-1",
        "+1",
        "1.0",
        "_1",
        "1_",
        "1__000",
        "1_00",
        "1234_567",
        "1_0000",
        "1,000",
        "340_282_366_920_938_463_463_374_607_431_768_211_456",
        " 1",
        "1 ",
        "1e3",
        "١",
        "340282366920938463463374607431768211456",
    ] {
        let input = serde_json::to_vec(&serde_json::json!({"cycles": cycles})).unwrap();
        assert!(
            matches!(
                decode_canister_cycles(&input, input.len()),
                Err(CycleBalanceError::InvalidCycles)
            ),
            "{cycles:?}"
        );
    }
}

#[test]
fn cycle_status_refuses_null_repeated_wrong_type_and_incomplete_json() {
    for input in [
        b"null".as_slice(),
        b"[]",
        b"[null]",
        b"[\"1\"]",
        b"{",
        b"",
        br#"{"cycles":null}"#,
        br#"{"cycles":1}"#,
        br#"{"cycles":false}"#,
        br#"{"cycles":[]}"#,
        br#"{"cycles":{}}"#,
        br#"{"cycles":"1","cycles":"2"}"#,
        br#"{"cycles":"1"} {}"#,
        br#"{"cycles":"1"} trailing"#,
    ] {
        assert!(
            matches!(
                decode_canister_cycles(input, input.len()),
                Err(CycleBalanceError::Json(_))
            ),
            "{input:?}"
        );
    }
}

#[test]
fn complete_input_must_be_utf8_even_in_ignored_metadata() {
    for input in [b"{\"cycles\":\"\xff\"}".as_slice(), b"{\"extra\":\"\xff\"}"] {
        assert!(matches!(
            decode_canister_cycles(input, input.len()),
            Err(CycleBalanceError::Utf8(_))
        ));
    }
}
