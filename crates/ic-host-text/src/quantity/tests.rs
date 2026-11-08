use super::*;

#[test]
fn caller_selected_decimal_and_binary_scales_preserve_rounding() {
    let decimal = [(1_000_000, "M"), (1_000, "k"), (1, "")];
    for (value, expected) in [
        (0, "0"),
        (999, "999"),
        (1_000, "1 k"),
        (1_004, "1 k"),
        (1_005, "1.01 k"),
        (10_100, "10.1 k"),
        (999_994, "999.99 k"),
        (999_995, "1 M"),
    ] {
        assert_eq!(scaled_quantity_text(value, &decimal).unwrap(), expected);
    }
    let binary = [(1_048_576, "MiB"), (1_024, "KiB"), (1, "B")];
    assert_eq!(scaled_quantity_text(0, &binary).unwrap(), "0 B");
    assert_eq!(scaled_quantity_text(1_372, &binary).unwrap(), "1.34 KiB");
    assert_eq!(scaled_quantity_text(1_048_575, &binary).unwrap(), "1 MiB");
}

#[test]
fn full_integer_range_and_maximum_divisor_do_not_overflow() {
    assert_eq!(
        scaled_quantity_text(u128::MAX, &[(1, "")]).unwrap(),
        u128::MAX.to_string()
    );
    let units = [(u64::MAX, "large"), (1, "")];
    assert_eq!(
        scaled_quantity_text(u128::MAX, &units).unwrap(),
        "18446744073709551617 large"
    );
    assert_eq!(
        scaled_quantity_text(u128::from(u64::MAX) * 2 - 1, &units).unwrap(),
        "2 large"
    );
}

#[test]
fn invalid_unit_tables_return_a_typed_error() {
    for units in [
        &[][..],
        &[(0, "zero")],
        &[(2, "no base unit")],
        &[(1, "a"), (1, "b")],
        &[(1, "base"), (1_000, "k")],
        &[(1_000, "k"), (0, "zero"), (1, "")],
        &[(1_000, "k"), (3, "third"), (1, "")],
    ] {
        assert_eq!(scaled_quantity_text(42, units), Err(InvalidQuantityScale));
    }
}

#[test]
fn unit_ratios_can_differ_and_suffix_controls_are_escaped() {
    let units = [(3_600, "h"), (60, "m"), (1, "s")];
    assert_eq!(scaled_quantity_text(3_599, &units).unwrap(), "59.98 m");
    assert_eq!(scaled_quantity_text(3_600, &units).unwrap(), "1 h");
    assert_eq!(scaled_quantity_text(1, &[(1, "\n")]).unwrap(), "1 \\n");
}

#[test]
fn decimal_quantities_round_trim_and_promote_with_caller_units() {
    let units = [
        (1_000_000_000_000, "T"),
        (1_000_000, "M"),
        (1_000, "k"),
        (1, ""),
    ];
    for (input, expected) in [
        ("0", "0"),
        ("000.000", "0"),
        ("0.125", "0.13"),
        ("999.994", "999.99"),
        ("999.995", "1 k"),
        ("999999.995", "1 M"),
        ("999999999999.9", "1 T"),
        ("00001000.000", "1 k"),
        ("12.30", "12.3"),
        ("1.", "1"),
        (
            "1000000000000000000000000000000000000000000",
            "1000000000000000000000000000000 T",
        ),
    ] {
        assert_eq!(
            scaled_decimal_quantity_text(input, &units).as_deref(),
            Some(expected)
        );
    }
    let units = [(1_000, "large"), (10, "small"), (1, "")];
    assert_eq!(
        scaled_decimal_quantity_text("9.995", &units).as_deref(),
        Some("1 small")
    );
    assert_eq!(
        scaled_decimal_quantity_text("999.95", &units).as_deref(),
        Some("1 large")
    );
    assert_eq!(
        scaled_decimal_quantity_text("999.949", &units).as_deref(),
        Some("99.99 small")
    );
    // The largest unit remains valid even when rounding carries past its boundary.
    assert_eq!(
        scaled_decimal_quantity_text("999.995", &[(1, "")]).as_deref(),
        Some("1000")
    );
}

#[test]
fn decimal_validation_checks_all_digits_and_does_not_bound_precision() {
    let units = [(1_000, "k"), (1, "")];
    let long = format!("0.125{}", "0".repeat(4_096));
    assert_eq!(
        scaled_decimal_quantity_text(&long, &units).as_deref(),
        Some("0.13")
    );
    let huge = format!("{}.995", "9".repeat(4_096));
    assert_eq!(
        scaled_decimal_quantity_text(&huge, &[(1, "")]),
        Some(format!("1{}", "0".repeat(4_096)))
    );
    for input in [
        "", ".", ".1", "-1", "+1", " 1", "1 ", "1_000", "1e3", "1.2.3", "١",
    ] {
        assert_eq!(scaled_decimal_quantity_text(input, &units), None);
    }
    for ending in ["x", ".", "\n", "é"] {
        assert_eq!(
            scaled_decimal_quantity_text(&format!("{long}{ending}"), &units),
            None
        );
    }
}

#[test]
fn decimal_scales_refuse_invalid_tables_and_escape_suffixes() {
    for units in [
        &[][..],
        &[(0, "zero")],
        &[(10, "no base")],
        &[(1, "a"), (1, "b")],
        &[(1, "base"), (10, "tens")],
        &[(10, "tens"), (0, "zero"), (1, "")],
        &[(1_024, "binary"), (1, "")],
        &[(1_000, "large"), (2, "small"), (1, "")],
    ] {
        assert_eq!(scaled_decimal_quantity_text("42", units), None);
    }
    assert_eq!(
        scaled_decimal_quantity_text("1", &[(1, "\n\t\u{1b}[31m\u{202e}")]).as_deref(),
        Some("1 \\n\\t\\u{1b}[31m\\u{202e}")
    );
    assert_eq!(
        scaled_decimal_quantity_text(
            "10000000000000000000",
            &[(10_000_000_000_000_000_000, "max"), (1, "")]
        )
        .as_deref(),
        Some("1 max")
    );
}

#[test]
fn decimal_and_integer_quantities_agree_for_integral_inputs() {
    let units = [(1_000_000, "M"), (1_000, "k"), (1, "")];
    for value in [0, 999, 1_005, 999_994, 999_995, 1_000_000, u128::MAX] {
        assert_eq!(
            scaled_decimal_quantity_text(&value.to_string(), &units),
            scaled_quantity_text(value, &units).ok()
        );
    }
}
