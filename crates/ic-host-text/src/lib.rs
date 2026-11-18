//! Pure terminal-safe text and table layouts; callers own budgets and output.
#[cfg(test)]
mod tests;

use std::borrow::Cow;
use unicode_width::UnicodeWidthChar;

/// Escape controls, Unicode line separators and bidirectional controls.
///
/// Printable Unicode and literal backslashes remain unchanged. Unsafe scalars
/// use Rust's `escape_default` spelling, including visible `\n`, `\r` and `\t`.
/// This is presentation, not reversible encoding or identity normalization.
/// Unchanged input is borrowed; changed input uses ordinary String allocation.
#[must_use]
pub fn escape(value: &str) -> Cow<'_, str> {
    let unsafe_scalar = |c: char| {
        c.is_control()
            || matches!(c, '\u{061c}' | '\u{200e}' | '\u{200f}'
                | '\u{2028}'..='\u{202e}' | '\u{2066}'..='\u{2069}')
    };
    if !value.chars().any(unsafe_scalar) {
        return Cow::Borrowed(value);
    }
    let mut output = String::new();
    for c in value.chars() {
        if unsafe_scalar(c) {
            output.extend(c.escape_default());
        } else {
            output.push(c);
        }
    }
    Cow::Owned(output)
}

/// Render left-aligned escaped cells with ASCII borders and no final newline.
///
/// Columns cover the longest row or header. Missing cells are empty; extra cells
/// have empty headers. Zero columns produces empty output. Cell whitespace is
/// retained. Width is the sum of Unicode scalar widths, not grapheme/emoji-sequence
/// width; joined emoji count separately. Escaping precedes width calculation.
/// Callers own row/column choice, input/output bounds and ordinary allocation costs.
#[must_use]
pub fn bordered_table(headers: &[impl AsRef<str>], rows: &[Vec<impl AsRef<str>>]) -> String {
    let cells: Vec<Vec<_>> = std::iter::once(headers.iter().map(|v| escape(v.as_ref())).collect())
        .chain(
            rows.iter()
                .map(|row| row.iter().map(|v| escape(v.as_ref())).collect()),
        )
        .collect();
    let columns = cells.iter().map(Vec::len).max().unwrap_or(0);
    if columns == 0 {
        return String::new();
    }
    let mut widths = vec![0; columns];
    for row in &cells {
        for (index, cell) in row.iter().enumerate() {
            widths[index] = widths[index].max(width(cell));
        }
    }
    let mut border = String::from("+");
    for columns in &widths {
        border.push('-');
        border.extend(std::iter::repeat_n('-', *columns));
        border.push_str("-+");
    }
    let mut output = border.clone();
    for (index, row) in cells.iter().enumerate() {
        output.push_str("\n|");
        for (column, columns) in widths.iter().enumerate() {
            let cell = row.get(column).map_or("", |v| v.as_ref());
            output.push(' ');
            output.push_str(cell);
            output.extend(std::iter::repeat_n(' ', columns - width(cell)));
            output.push_str(" |");
        }
        if index == 0 || index + 1 == cells.len() {
            output.push('\n');
            output.push_str(&border);
        }
    }
    output
}

fn width(value: &str) -> usize {
    value.chars().map(|c| c.width().unwrap_or(0)).sum()
}

/// Padding direction for a plain table column.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum Alignment {
    /// Pad after the cell.
    Left,
    /// Pad before the cell.
    Right,
}

/// Render an escaped plain table with two-space gaps and a dashed header rule.
///
/// Column counts agree by type. Unicode scalar widths follow [`bordered_table`].
/// Every line omits trailing whitespace; no extra newline is appended.
/// A final empty data row still contributes its line separator.
/// Zero columns returns empty text. Callers own limits, output and ordinary
/// allocation costs; no terminal detection or truncation is performed.
#[must_use]
pub fn plain_table<const N: usize>(
    headers: &[impl AsRef<str>; N],
    rows: &[[impl AsRef<str>; N]],
    alignments: &[Alignment; N],
) -> String {
    if N == 0 {
        return String::new();
    }
    let cells: Vec<_> = std::iter::once(headers.each_ref().map(|v| escape(v.as_ref())))
        .chain(
            rows.iter()
                .map(|r| r.each_ref().map(|v| escape(v.as_ref()))),
        )
        .collect();
    let widths: [usize; N] =
        std::array::from_fn(|column| cells.iter().map(|r| width(&r[column])).max().unwrap_or(0));
    let mut lines = Vec::with_capacity(cells.len());
    for row in &cells {
        let mut line = String::new();
        for column in 0..N {
            if column != 0 {
                line.push_str("  ");
            }
            let padding = widths[column] - width(&row[column]);
            if alignments[column] == Alignment::Right {
                line.extend(std::iter::repeat_n(' ', padding));
            }
            line.push_str(&row[column]);
            if alignments[column] == Alignment::Left {
                line.extend(std::iter::repeat_n(' ', padding));
            }
        }
        line.truncate(line.trim_end().len());
        lines.push(line);
    }
    let rule = widths.map(|columns| "-".repeat(columns)).join("  ");
    lines.insert(1, rule.trim_end().to_owned());
    lines.join("\n")
}
