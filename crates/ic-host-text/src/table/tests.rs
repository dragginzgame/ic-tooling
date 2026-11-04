use super::*;
const OPTIONS: Options = Options {
    column_gap: 2,
    header_separator: true,
};

#[test]
fn joined_sequences_share_clipping_and_progress_columns() {
    let emoji = "👩‍💻";
    let clipped = crate::width::fit_line(emoji, 4);
    assert_eq!(clipped, emoji);
    let mut progress = crate::progress::ProgressLine::new();
    assert_eq!(progress.update(&clipped), "\r👩‍💻");
    assert_eq!(progress.finish("x"), "\rx   \n");
    assert_eq!(
        render(
            &["X", "N"],
            &[[clipped.as_str(), "1"], ["abcd", "2"]],
            &[Alignment::Left, Alignment::Right],
            OPTIONS,
        ),
        "X     N\n----  -\n👩‍💻  1\nabcd  2"
    );
}

#[test]
fn wide_and_combining_cells_align_in_display_columns() {
    assert_eq!(
        render(
            &["NAME", "N"],
            &[["界", "7"], ["e\u{301}", "123"]],
            &[Alignment::Left, Alignment::Right],
            OPTIONS
        ),
        "NAME    N\n----  ---\n界      7\ne\u{301}     123"
    );
}

#[test]
fn escaped_headers_and_cells_are_measured_and_stay_on_their_rows() {
    let text = render(
        &["H\n"],
        &[["x\x1b[2J\t\u{2029}"]],
        &[Alignment::Left],
        OPTIONS,
    );
    assert_eq!(text.lines().count(), 3);
    assert!(text.starts_with("H\\n\n"));
    assert!(text.ends_with("x\\u{1b}[2J\\t\\u{2029}"));
    assert!(!text.contains('\x1b'));
}

#[test]
fn whitespace_cells_preserve_empty_rows_and_separator_spacing() {
    assert_eq!(
        render(
            &["  ", "B "],
            &[["", ""], ["x ", "\u{2003}"], ["界", "7"]],
            &[Alignment::Left, Alignment::Right],
            OPTIONS,
        ),
        "    B\n--  --\n\nx\n界   7"
    );
    assert_eq!(
        render(&["H"], &[[""], [""]], &[Alignment::Left], OPTIONS),
        "H\n-\n\n"
    );
    assert_eq!(
        render(&["", ""], &[["", ""]], &[Alignment::Left; 2], OPTIONS),
        "\n  \n"
    );
}

#[test]
fn explicit_options_and_empty_tables_have_no_hidden_defaults() {
    assert_eq!(
        render(
            &["A", "B"],
            &[["1", "2"]],
            &[Alignment::Left; 2],
            Options {
                column_gap: 0,
                header_separator: false
            }
        ),
        "AB\n12"
    );
    assert_eq!(
        render(&["A"], &[] as &[[&str; 1]], &[Alignment::Left], OPTIONS),
        "A\n-"
    );
    assert_eq!(
        render(&[] as &[&str; 0], &[] as &[[&str; 0]], &[], OPTIONS),
        ""
    );
}

#[test]
fn bordered_dynamic_columns_preserve_layout_and_ragged_cells() {
    // Match CLI-owned String headers and dynamic String rows.
    assert_eq!(
        render_bordered(&[String::from("A")], &[vec![String::from("x")]]),
        "+---+\n| A |\n+---+\n| x |\n+---+"
    );
    assert_eq!(
        render_bordered(&["A", "B"], &[vec!["x", "123"], vec!["long"]]),
        "+------+-----+\n| A    | B   |\n+------+-----+\n| x    | 123 |\n| long |     |\n+------+-----+"
    );
    assert_eq!(
        render_bordered(&["A"], &[vec!["x", "y"]]),
        "+---+---+\n| A |   |\n+---+---+\n| x | y |\n+---+---+"
    );
    assert_eq!(
        render_bordered(&["A"], &[] as &[Vec<&str>]),
        "+---+\n| A |\n+---+"
    );
    assert_eq!(render_bordered(&[] as &[&str], &[] as &[Vec<&str>]), "");
    assert_eq!(render_bordered(&[] as &[&str], &[vec![] as Vec<&str>]), "");
    assert_eq!(
        render_bordered(&[] as &[&str], &[vec!["x"]]),
        "+---+\n|   |\n+---+\n| x |\n+---+"
    );
    assert_eq!(
        render_bordered(&["A", "B"], &[vec![] as Vec<&str>, vec!["  ", "x "]]),
        "+----+----+\n| A  | B  |\n+----+----+\n|    |    |\n|    | x  |\n+----+----+"
    );
}

#[test]
fn bordered_tables_escape_before_shared_unicode_alignment() {
    let result = render_bordered(
        &["界", "h\n"],
        &[
            vec!["e\u{301}", "\x1b]52;c;x\x07"],
            vec!["x\ny", "\u{202e}\u{0085}\u{2028}"],
        ],
    );
    let lines = result.lines().collect::<Vec<_>>();
    assert_eq!(lines.len(), 6);
    assert!(!result.contains('\x1b'));
    assert!(!result.contains('\u{202e}'));
    assert!(result.contains("e\u{301}"));
    assert!(result.contains("x\\ny"));
    assert!(result.contains("h\\n"));
    let width = display_width(lines[0]);
    for line in lines {
        assert_eq!(display_width(line), width);
    }
    assert_eq!(
        render_bordered(&["X"], &[vec!["👩‍💻"]]),
        "+------+\n| X    |\n+------+\n| 👩‍💻 |\n+------+"
    );
}
