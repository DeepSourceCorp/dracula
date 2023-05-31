use crate::parse::{ItemRange, Language, Matcher, ParseItem};

/// Rust needs to define keyedness for Raw Strings
/// as they are delimited by specific `#` count
pub struct Rust;
impl Language for Rust {
    const PARSE_ITEMS: &'static [ParseItem] = &[
        ParseItem::UnEscaped(&ParseItem::Comment(
            ItemRange::fixed_start("//").pre_fixed_end("\n"),
            false,
        )),
        ParseItem::UnEscaped(&ParseItem::Comment(
            ItemRange::fixed_start("/*").fixed_end("*/"),
            false,
        )),
        ParseItem::Escaped(&ParseItem::String(
            ItemRange::fixed_start("\"").fixed_end("\""),
            false,
        )),
        ParseItem::Escaped(&ParseItem::String(
            ItemRange::fixed_start("b\"").fixed_end("\""),
            false,
        )),
        ParseItem::UnEscaped(&ParseItem::String(
            ItemRange::start_matcher(
                Matcher::Exact("r"),
                Matcher::Repeat("#"),
                Matcher::Exact("\""),
            )
            .end_matcher(Matcher::Exact("\""), Matcher::Repeat("#"), Matcher::Empty),
            true,
        )),
    ];

    fn is_meaningful_src(src: &str) -> bool {
        !src.chars()
            .all(|ch| char::is_whitespace(ch) || ch == '}' || ch == '{' || ch == '(' || ch == ')')
    }
}
