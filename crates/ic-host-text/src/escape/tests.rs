use super::*;

#[test]
fn controls_cannot_create_terminal_commands_or_report_lines() {
    assert_eq!(
        terminal_text("a\n\r\t\0\x1b[31m\u{009b}\u{2028}\u{202e}\u{2069}"),
        "a\\n\\r\\t\\u{0}\\u{1b}[31m\\u{9b}\\u{2028}\\u{202e}\\u{2069}"
    );
    assert_eq!(
        terminal_text("界 e\u{301} 👩‍💻\u{202e}x"),
        "界 e\u{301} 👩‍💻\\u{202e}x"
    );
}

#[test]
fn printable_unicode_spaces_and_literal_backslashes_are_preserved() {
    let text = "  界 e\u{301} 👩‍💻 '\" \\n  ";
    let escaped = terminal_text(text);
    assert_eq!(escaped, text);
    assert!(matches!(escaped, Cow::Borrowed(_)));
    assert!(matches!(terminal_text(""), Cow::Borrowed("")));
}
