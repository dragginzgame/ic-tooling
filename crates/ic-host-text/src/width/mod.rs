//! Pure clipping to a caller-selected terminal column allowance.
use unicode_width::UnicodeWidthChar;

use crate::escape::escaped_chars;

// One scalar-column policy for clipping, tables and progress erasure.
pub(crate) fn scalar_width(character: char) -> usize {
    character.width().unwrap_or(0)
}

pub(crate) fn display_width(text: &str) -> usize {
    text.chars().map(scalar_width).sum()
}

/// Escape controls and fit one line within `columns`, adding `…` when clipped.
///
/// Zero columns return an empty string. Width is the sum of Unicode scalar
/// widths, conservatively counting joined emoji separately. Clipping preserves
/// UTF-8 boundaries, but does not promise grapheme-cluster boundaries. This is
/// display text, not an identity-preserving encoding or an ANSI parser.
/// Escaping stops once clipping is established; no complete escaped copy is built.
/// Callers own input byte limits, terminal dimensions, output and redraw policy.
#[must_use]
pub fn fit_line(line: &str, columns: usize) -> String {
    if columns == 0 {
        return String::new();
    }
    let mut width = 0;
    let mut text = String::new();
    let mut clip_at = None;
    for ch in escaped_chars(line) {
        let next = scalar_width(ch);
        let remaining = columns - width;
        if next > remaining {
            text.truncate(clip_at.unwrap_or(text.len()));
            text.push('…');
            return text;
        }
        if next == remaining && clip_at.is_none() {
            // Reserve the same prefix as clipping with one column for the ellipsis.
            // Keep it only if a later character proves that clipping is necessary.
            clip_at = Some(text.len());
        }
        text.push(ch);
        width += next;
    }
    text
}

#[cfg(test)]
mod tests;
