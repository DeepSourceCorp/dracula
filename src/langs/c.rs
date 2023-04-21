use crate::parse::{ItemRange, Language, ParseItem, Matcher};

/// C supports escapes in single line comments as well
pub struct C;
impl Language for C {
    const PARSE_ITEMS: &'static [ParseItem] = &[
        // single line comment
        ParseItem::Escaped(&ParseItem::Comment(
            ItemRange::fixed_start("//").pre_fixed_end("\n"),
            false,
        )),
        // multiline line comment
        ParseItem::UnEscaped(&ParseItem::Comment(
            ItemRange::fixed_start("/*").fixed_end("*/"),
            false,
        )),
        // simple string
        ParseItem::Escaped(&ParseItem::String(
            ItemRange::fixed_start("\"").fixed_end("\""),
            false,
        )),
        // raw string comment
        ParseItem::UnEscaped(&ParseItem::String(
            ItemRange::start_matcher(
                Matcher::Exact("R\""),
                Matcher::AnyAlphaNumeric,
                Matcher::Exact("("),
            )
            .end_matcher(
                Matcher::Exact(")"),
                Matcher::AnyAlphaNumeric,
                Matcher::Exact("\""),
            ),
            true,
        )), // R"UNIQUE_KEY( RAW STRING )UNIQUE_KEY"
    ];
    fn is_meaningful_src(src: &str) -> bool {
        !src.chars()
            .all(|ch| char::is_whitespace(ch) || ch == '}' || ch == '{')
    }
}