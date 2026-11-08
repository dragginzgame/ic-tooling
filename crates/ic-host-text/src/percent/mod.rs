//! Exact ratio-to-percentage text; callers own totals and unavailable-value policy.

/// Render a `u64` ratio as a percentage rounded half up to one decimal place.
///
/// Returns `None` for a zero denominator. Ratios above one are allowed, and the
/// full input range is preserved using `u128` intermediate arithmetic.
///
/// ```
/// use ic_host_text::percent::ratio_percent_text;
/// assert_eq!(ratio_percent_text(1, 16).as_deref(), Some("6.3%"));
/// assert_eq!(ratio_percent_text(3, 2).as_deref(), Some("150.0%"));
/// assert_eq!(ratio_percent_text(1, 0), None);
/// ```
#[must_use]
pub fn ratio_percent_text(numerator: u64, denominator: u64) -> Option<String> {
    if denominator == 0 {
        return None;
    }
    // Both scaling and half-up rounding fit even for the full u64 input range.
    let denominator = u128::from(denominator);
    let tenths = (u128::from(numerator) * 1_000 + denominator / 2) / denominator;
    Some(format!("{}.{:01}%", tenths / 10, tenths % 10))
}

#[cfg(test)]
mod tests;
