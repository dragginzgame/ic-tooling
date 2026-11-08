use super::ratio_percent_text;

#[test]
fn exact_halves_round_up_and_percentages_can_exceed_one_hundred() {
    for (numerator, denominator, expected) in [
        (0, 1, "0.0%"),
        (1, 16, "6.3%"),
        (1, 2_001, "0.0%"),
        (1, 2_000, "0.1%"),
        (1, 1_999, "0.1%"),
        (1, 3, "33.3%"),
        (2, 3, "66.7%"),
        (3, 2, "150.0%"),
        (u64::MAX, u64::MAX, "100.0%"),
        (u64::MAX, 1, "1844674407370955161500.0%"),
        (1, u64::MAX, "0.0%"),
        (u64::MAX / 2, u64::MAX, "50.0%"),
    ] {
        assert_eq!(
            ratio_percent_text(numerator, denominator).as_deref(),
            Some(expected)
        );
    }
}

#[test]
fn zero_denominator_refuses_without_selecting_a_fallback() {
    for numerator in [0, 1, u64::MAX] {
        assert_eq!(ratio_percent_text(numerator, 0), None);
    }
}
