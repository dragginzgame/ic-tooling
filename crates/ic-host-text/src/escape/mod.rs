//! Escape untrusted scalar text without selecting terminal or colour policy.
use std::borrow::Cow;

/// Escape terminal controls, Unicode line separators and bidirectional controls.
///
/// LF, CR and tab become visible `\n`, `\r` and `\t`; other unsafe characters
/// become `\u{hex}`. Printable Unicode and literal backslashes remain unchanged.
/// This is a presentation, not an identity-preserving or reversible encoding.
/// It performs no terminal detection, output, truncation or size admission.
/// Text needing no escapes is borrowed; call `.into_owned()` when independent
/// ownership is required. Changed text uses ordinary String allocation semantics.
#[must_use]
pub fn terminal_text(value: &str) -> Cow<'_, str> {
    let Some(start) = value.find(needs_escape) else {
        return Cow::Borrowed(value);
    };
    let mut escaped = String::with_capacity(value.len());
    escaped.push_str(&value[..start]);
    escaped.extend(escaped_chars(&value[start..]));
    Cow::Owned(escaped)
}

// Yield the presentation encoding without allocating a complete escaped copy.
pub(crate) fn escaped_chars(value: &str) -> impl Iterator<Item = char> + '_ {
    let mut characters = value.chars();
    let mut pending: Option<std::char::EscapeDefault> = None;
    std::iter::from_fn(move || {
        if let Some(escaped) = &mut pending
            && let Some(character) = escaped.next()
        {
            return Some(character);
        }
        pending = None;
        let character = characters.next()?;
        if needs_escape(character) {
            let mut escaped = character.escape_default();
            let first = escaped.next();
            pending = Some(escaped);
            first
        } else {
            Some(character)
        }
    })
}

const fn needs_escape(character: char) -> bool {
    character.is_control()
        || matches!(character, '\u{061c}' | '\u{200e}' | '\u{200f}'
        | '\u{2028}'..='\u{202e}' | '\u{2066}'..='\u{2069}')
}

#[cfg(test)]
mod tests;
