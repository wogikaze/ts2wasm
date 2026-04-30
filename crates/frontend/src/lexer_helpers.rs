pub(super) fn is_ascii_identifier_start(ch: char) -> bool {
    matches!(ch, 'a'..='z' | 'A'..='Z' | '_' | '$')
}

pub(super) fn is_ascii_identifier_part(ch: char) -> bool {
    is_ascii_identifier_start(ch) || ch.is_ascii_digit()
}

pub(super) fn is_identifier_start_escape_char(ch: char) -> bool {
    is_ascii_identifier_start(ch) || (ch != '\u{200c}' && ch != '\u{200d}' && ch.is_alphabetic())
}

pub(super) fn is_identifier_part_escape_char(ch: char) -> bool {
    is_identifier_start_escape_char(ch)
        || ch.is_ascii_digit()
        || ch.is_numeric()
        || ch == '\u{200c}'
        || ch == '\u{200d}'
        || is_unicode_5_2_identifier_part_mark(ch)
}

pub(super) fn is_unicode_5_2_identifier_part_mark(ch: char) -> bool {
    matches!(
        ch as u32,
        0x0300..=0x036f
            | 0x0483..=0x0487
            | 0x0591..=0x05bd
            | 0x05bf
            | 0x05c1..=0x05c2
            | 0x05c4..=0x05c5
            | 0x05c7
            | 0x0610..=0x061a
            | 0x064b..=0x065f
            | 0x0670
            | 0x06d6..=0x06dc
            | 0x06df..=0x06e4
            | 0x06e7..=0x06e8
            | 0x06ea..=0x06ed
            | 0x0711
            | 0x0730..=0x074a
            | 0x07a6..=0x07b0
            | 0x0816..=0x082d
            | 0x0900
            | 0x094e
            | 0x0955
            | 0x109a..=0x109d
            | 0x135d..=0x135f
            | 0x1712..=0x1714
            | 0x1732..=0x1734
            | 0x1752..=0x1753
            | 0x1772..=0x1773
            | 0x17b4..=0x17d3
            | 0x17dd
            | 0x180b..=0x180d
            | 0x19da
            | 0x1a55..=0x1a7f
            | 0x1a80..=0x1a99
            | 0x1ab0..=0x1aff
            | 0x1cd0..=0x1cf2
            | 0x1dc0..=0x1dff
            | 0x20d0..=0x20ff
            | 0x2cef..=0x2cf1
            | 0xa6f0..=0xa6f1
            | 0xa8e0..=0xa8f1
            | 0xa980..=0xa9c0
            | 0xa9d0..=0xa9d9
            | 0xaa7b
            | 0xaab0..=0xaac1
            | 0xabe3..=0xabed
            | 0xabf0..=0xabf9
            | 0xfe00..=0xfe0f
            | 0xfe20..=0xfe2f
            | 0x11080..=0x110ba
    )
}

pub(super) fn is_line_terminator(ch: char) -> bool {
    matches!(ch, '\n' | '\r' | '\u{2028}' | '\u{2029}')
}

pub(super) fn is_digit_for_radix(ch: char, radix_name: &str) -> bool {
    match radix_name {
        "binary" => matches!(ch, '0' | '1'),
        "octal" => matches!(ch, '0'..='7'),
        "hexadecimal" => ch.is_ascii_hexdigit(),
        _ => false,
    }
}

pub(super) fn source_has_use_strict_directive(source: &str) -> bool {
    let mut cursor = 0usize;
    loop {
        cursor = skip_directive_trivia(source, cursor);
        let Some(quote) = source[cursor..].chars().next() else {
            return false;
        };
        if quote != '"' && quote != '\'' {
            return false;
        }
        let Some((value, end)) = read_simple_directive_literal(source, cursor, quote) else {
            return false;
        };
        cursor = skip_inline_whitespace(source, end);
        match source[cursor..].chars().next() {
            Some(';') => cursor += 1,
            Some(ch) if is_line_terminator(ch) => {}
            None => {}
            _ => return false,
        }
        if value == "use strict" {
            return true;
        }
    }
}

pub(super) fn skip_directive_trivia(source: &str, mut cursor: usize) -> usize {
    loop {
        let rest = &source[cursor..];
        if let Some(ch) = rest.chars().next()
            && ch.is_whitespace()
        {
            cursor += ch.len_utf8();
            continue;
        }
        if rest.starts_with("//") {
            cursor += 2;
            while let Some(ch) = source[cursor..].chars().next() {
                if is_line_terminator(ch) {
                    break;
                }
                cursor += ch.len_utf8();
            }
            continue;
        }
        if rest.starts_with("/*") {
            if let Some(end) = rest.find("*/") {
                cursor += end + 2;
                continue;
            }
            return source.len();
        }
        return cursor;
    }
}

pub(super) fn skip_inline_whitespace(source: &str, mut cursor: usize) -> usize {
    while let Some(ch) = source[cursor..].chars().next() {
        if is_line_terminator(ch) || !ch.is_whitespace() {
            break;
        }
        cursor += ch.len_utf8();
    }
    cursor
}

pub(super) fn read_simple_directive_literal(
    source: &str,
    start: usize,
    quote: char,
) -> Option<(String, usize)> {
    let mut cursor = start + quote.len_utf8();
    let mut value = String::new();
    while let Some(ch) = source[cursor..].chars().next() {
        cursor += ch.len_utf8();
        if ch == '\\' || is_line_terminator(ch) {
            return None;
        }
        if ch == quote {
            return Some((value, cursor));
        }
        value.push(ch);
    }
    None
}
