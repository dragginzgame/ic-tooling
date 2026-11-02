//! Exact scaled integer quantities; callers select units and input admission.

use crate::escape::terminal_text;
use std::fmt::{self, Write};

///
/// `InvalidQuantityScale`
///
/// Unit divisors are not descending integer multiples ending in one.
///

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct InvalidQuantityScale;

impl fmt::Display for InvalidQuantityScale {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter.write_str(
            "expected descending unit divisors that are integer multiples, ending in one",
        )
    }
}

impl std::error::Error for InvalidQuantityScale {}

/// Render an integer with the largest applicable caller-selected unit.
///
/// Each `(divisor, suffix)` defines a unit, from largest to smallest. Divisors
/// must be positive, strictly descending integer multiples ending in one.
/// Divisors are bounded by `u64`; values retain the full `u128` range.
/// Uses exact round-half-up arithmetic to two places, trims trailing fractional
/// zeroes and promotes to the next unit when rounding reaches its boundary.
/// Nonempty suffixes are terminal-escaped and separated by one space.
///
/// # Errors
/// Returns [`InvalidQuantityScale`] for an empty or invalid unit list.
///
/// ```
/// use ic_host_text::quantity::scaled_quantity_text;
/// let units = [(1_048_576, "MiB"), (1_024, "KiB"), (1, "B")];
/// assert_eq!(scaled_quantity_text(1_372, &units).unwrap(), "1.34 KiB");
/// assert_eq!(scaled_quantity_text(1_048_575, &units).unwrap(), "1 MiB");
/// ```
pub fn scaled_quantity_text(
    value: u128,
    units: &[(u64, &str)],
) -> Result<String, InvalidQuantityScale> {
    if units.last().map(|unit| unit.0) != Some(1)
        || units.windows(2).any(|pair| {
            let (larger, smaller) = (pair[0].0, pair[1].0);
            smaller == 0 || larger <= smaller || !larger.is_multiple_of(smaller)
        })
    {
        return Err(InvalidQuantityScale);
    }
    let mut index = units
        .iter()
        .position(|(divisor, _)| value >= u128::from(*divisor))
        .unwrap_or(units.len() - 1);
    let (mut whole, mut hundredths) = rounded_parts(value, units[index].0);
    if index > 0 && whole >= u128::from(units[index - 1].0 / units[index].0) {
        index -= 1;
        (whole, hundredths) = rounded_parts(value, units[index].0);
    }
    let mut number = whole.to_string();
    if hundredths != 0 {
        write!(number, ".{hundredths:02}").expect("writing to a String cannot fail");
        if hundredths.is_multiple_of(10) {
            number.pop();
        }
    }
    if !units[index].1.is_empty() {
        number.push(' ');
        number.push_str(&terminal_text(units[index].1));
    }
    Ok(number)
}

const fn rounded_parts(value: u128, divisor: u64) -> (u128, u128) {
    let divisor = divisor as u128;
    let mut whole = value / divisor;
    let remainder = value % divisor;
    // The u64 divisor bounds the remainder, so scaling it cannot overflow u128.
    let mut hundredths = (remainder * 100 + divisor / 2) / divisor;
    if hundredths == 100 {
        whole += 1;
        hundredths = 0;
    }
    (whole, hundredths)
}

#[cfg(test)]
mod tests;
