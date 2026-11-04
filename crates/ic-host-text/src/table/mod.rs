//! Unicode display-width alignment over escaped caller-selected cells.
use crate::{escape::terminal_text, width::display_width};
use std::borrow::Cow;

struct Cell<'a> {
    text: Cow<'a, str>,
    columns: usize,
}

impl<'a> Cell<'a> {
    fn new(text: &'a str) -> Self {
        let text = terminal_text(text);
        let columns = display_width(&text);
        Self { text, columns }
    }
}

/// Caller-selected alignment of one complete column.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum Alignment {
    /// Pad after the cell.
    Left,
    /// Pad before the cell.
    Right,
}

/// Explicit formatting choices; no ambient terminal policy or defaults.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct Options {
    /// Spaces between columns.
    pub column_gap: usize,
    /// Whether a dashed line follows the headers.
    pub header_separator: bool,
}

/// Render headers and rows after escaping every cell, including headers.
///
/// Widths sum Unicode scalar columns, like [`crate::width::fit_line`] and
/// [`crate::progress::ProgressLine`]. Joined emoji count their scalar widths
/// separately; sequence-specific ligatures do not alter column accounting.
/// Rows have the same column count by type. Header/data lines omit trailing
/// whitespace; the result has no final newline. Zero columns return empty text.
/// Caller owns input/output size, row construction, headers, column choices,
/// truncation and all output, terminal detection, colour and progress behavior.
/// Storage allocation follows ordinary String/Vec allocation semantics.
#[must_use]
pub fn render<const N: usize>(
    headers: &[impl AsRef<str>; N],
    rows: &[[impl AsRef<str>; N]],
    alignments: &[Alignment; N],
    options: Options,
) -> String {
    if N == 0 {
        return String::new();
    }
    let headers = headers.each_ref().map(|cell| Cell::new(cell.as_ref()));
    let rows = rows
        .iter()
        .map(|row| row.each_ref().map(|cell| Cell::new(cell.as_ref())))
        .collect::<Vec<_>>();
    let mut widths = [0; N];
    measure_rows(
        &mut widths,
        std::iter::once(headers.as_slice()).chain(rows.iter().map(<[_; N]>::as_slice)),
    );
    let gap = " ".repeat(options.column_gap);
    let mut output = String::new();
    render_row(&headers, &widths, alignments, &gap, &mut output);
    if options.header_separator {
        output.push('\n');
        for (index, width) in widths.iter().enumerate() {
            if index != 0 {
                output.push_str(&gap);
            }
            output.extend(std::iter::repeat_n('-', *width));
        }
    }
    for row in &rows {
        output.push('\n');
        render_row(row, &widths, alignments, &gap, &mut output);
    }
    output
}

/// Render a dynamic-column table with ASCII borders and left-aligned cells.
///
/// All cells are terminal-escaped before measuring the same Unicode scalar
/// columns as [`render`]. Missing cells are empty; extra cells create unnamed
/// columns. No columns returns empty text. Header-only output retains its
/// separator. The result has no final newline and preserves cell whitespace.
/// Callers own value semantics, row construction, input/output budgets and all
/// terminal/output policy. Allocation follows ordinary String/Vec semantics.
#[must_use]
pub fn render_bordered(headers: &[impl AsRef<str>], rows: &[Vec<impl AsRef<str>>]) -> String {
    let headers = headers
        .iter()
        .map(|cell| Cell::new(cell.as_ref()))
        .collect::<Vec<_>>();
    let rows = rows
        .iter()
        .map(|row| {
            row.iter()
                .map(|cell| Cell::new(cell.as_ref()))
                .collect::<Vec<_>>()
        })
        .collect::<Vec<_>>();
    let columns = rows.iter().map(Vec::len).fold(headers.len(), usize::max);
    let mut widths = vec![0; columns];
    measure_rows(
        &mut widths,
        std::iter::once(headers.as_slice()).chain(rows.iter().map(Vec::as_slice)),
    );
    if widths.is_empty() {
        return String::new();
    }
    let mut separator = String::from("+");
    for width in &widths {
        separator.extend(std::iter::repeat_n('-', width.saturating_add(2)));
        separator.push('+');
    }
    let mut output = separator.clone();
    // Header and data rows share one border/padding path. The separator belongs
    // after the header even when there are no data rows.
    for (index, row) in std::iter::once(&headers).chain(&rows).enumerate() {
        output.push_str("\n| ");
        append_cells(
            row,
            &widths,
            std::iter::repeat(Alignment::Left),
            " | ",
            &mut output,
        );
        output.push_str(" |");
        if index == 0 {
            output.push('\n');
            output.push_str(&separator);
        }
    }
    if !rows.is_empty() {
        output.push('\n');
        output.push_str(&separator);
    }
    output
}

// Both renderers measure the same escaped cells. Fixed-column tables retain
// their arrays; only the dynamic renderer allocates a width vector.
fn measure_rows<'text: 'row, 'row>(
    widths: &mut [usize],
    rows: impl Iterator<Item = &'row [Cell<'text>]>,
) {
    for row in rows {
        for (width, value) in widths.iter_mut().zip(row) {
            *width = (*width).max(value.columns);
        }
    }
}

fn render_row(
    row: &[Cell<'_>],
    widths: &[usize],
    alignments: &[Alignment],
    gap: &str,
    output: &mut String,
) {
    let start = output.len();
    append_cells(row, widths, alignments.iter().copied(), gap, output);
    // Trim only this row so empty cells cannot erase earlier line separators.
    output.truncate(start + output[start..].trim_end().len());
}

fn append_cells(
    row: &[Cell<'_>],
    widths: &[usize],
    alignments: impl Iterator<Item = Alignment>,
    gap: &str,
    output: &mut String,
) {
    for (index, (width, alignment)) in widths.iter().zip(alignments).enumerate() {
        if index != 0 {
            output.push_str(gap);
        }
        let cell = row.get(index);
        let value = cell.map_or("", |cell| cell.text.as_ref());
        let padding = width.saturating_sub(cell.map_or(0, |cell| cell.columns));
        if alignment == Alignment::Right {
            output.extend(std::iter::repeat_n(' ', padding));
        }
        output.push_str(value);
        if alignment == Alignment::Left {
            output.extend(std::iter::repeat_n(' ', padding));
        }
    }
}

#[cfg(test)]
mod tests;
