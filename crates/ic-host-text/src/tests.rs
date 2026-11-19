use super::{bordered_table, escape};
use std::borrow::Cow;

#[test]
fn terminal_controls_and_bidi_are_visible_without_changing_printable_evidence() {
    assert!(matches!(escape("界e\u{301}\\n"), Cow::Borrowed(_)));
    assert_eq!(
        escape("\x1b[31m\n\r\t\0\u{85}"),
        "\\u{1b}[31m\\n\\r\\t\\u{0}\\u{85}"
    );

    for c in [
        '\u{61c}', '\u{200e}', '\u{200f}', '\u{2028}', '\u{2029}', '\u{202a}', '\u{202b}',
        '\u{202c}', '\u{202d}', '\u{202e}', '\u{2066}', '\u{2067}', '\u{2068}', '\u{2069}',
    ] {
        assert_eq!(escape(&c.to_string()), c.escape_default().to_string());
    }
}

#[test]
fn tables_preserve_ragged_shape_empty_headers_and_scalar_widths() {
    let rows = vec![vec!["界", "e\u{301}"], vec!["a\n", "👩\u{200d}💻"]];
    assert_eq!(
        bordered_table(&["H", "V"], &rows),
        "+-----+------+\n| H   | V    |\n+-----+------+\n| 界  | é    |\n| a\\n | 👩‍💻 |\n+-----+------+"
    );
    assert_eq!(
        bordered_table(&["x"], &[vec!["", "z"], vec![]]),
        "+---+---+\n| x |   |\n+---+---+\n|   | z |\n|   |   |\n+---+---+"
    );
    assert_eq!(
        bordered_table(&["\t"], &Vec::<Vec<String>>::new()),
        "+----+\n| \\t |\n+----+"
    );
    assert_eq!(
        bordered_table(&Vec::<String>::new(), &[Vec::<String>::new()]),
        ""
    );
}

#[test]
fn plain_tables_align_numbers_after_escaping_and_trim_only_current_lines() {
    use super::{Alignment, plain_table};
    let rows = [["界", "2"], ["a\n", "10"]];
    assert_eq!(
        plain_table(&["NAME", "N"], &rows, &[Alignment::Left, Alignment::Right]),
        "NAME   N\n----  --\n界     2\na\\n   10"
    );
    assert_eq!(
        plain_table(&["H"], &[[""], ["  "]], &[Alignment::Left]),
        "H\n--\n\n"
    );
    assert_eq!(
        plain_table(&["H"], &Vec::<[String; 1]>::new(), &[Alignment::Left]),
        "H\n-"
    );
    assert_eq!(
        plain_table(&[] as &[&str; 0], &Vec::<[String; 0]>::new(), &[]),
        ""
    );
}
