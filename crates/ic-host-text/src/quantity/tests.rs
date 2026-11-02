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
