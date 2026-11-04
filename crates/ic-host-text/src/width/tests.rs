use super::*;

#[test]
fn exact_width_and_small_allowances() {
    for (text, columns, expected) in [
        ("", 0, ""),
        ("", usize::MAX, ""),
        ("abc", 0, ""),
        ("abc", 1, "…"),
        ("abc", 2, "a…"),
        ("abc", 3, "abc"),
        ("abc", usize::MAX, "abc"),
        ("界界", 3, "界…"),
        ("e\u{301}x", 2, "e\u{301}x"),
        ("a\u{301}", 1, "a\u{301}"),
        ("a\u{301}x", 1, "…"),
        ("\u{301}界", 1, "\u{301}…"),
        ("界\u{301}", 2, "界\u{301}"),
        ("界\u{301}x", 2, "…"),
        ("a\u{200d}b", 2, "a\u{200d}b"),
        ("a\u{200d}bc", 2, "a\u{200d}…"),
        ("\u{202e}", 8, "\\u{202e}"),
        ("\u{202e}", 7, "\\u{202…"),
    ] {
        assert_eq!(fit_line(text, columns), expected);
    }
}

#[test]
fn controls_are_visible_before_width_admission() {
    assert_eq!(fit_line("a\nb", 4), "a\\nb");
    assert_eq!(fit_line("a\nb", 3), "a\\…");
    assert_eq!(fit_line("\x1b[31m", 40), "\\u{1b}[31m");
}

#[test]
fn unicode_output_obeys_each_selected_allowance() {
    for input in ["界界界", "e\u{301}e\u{301}xx", "👩‍💻 ready", "\n\t\u{202e}"] {
        for columns in 0..32 {
            let fitted = fit_line(input, columns);
            assert!(
                fitted
                    .chars()
                    .map(|ch| ch.width().unwrap_or(0))
                    .sum::<usize>()
                    <= columns
            );
            assert!(!fitted.chars().any(char::is_control));
        }
    }
}
