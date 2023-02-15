use crate::parse::*;

pub struct Java;
impl Language for Java {
    const PARSE_ITEMS: &'static [ParseItem] = &[
        ParseItem::Escaped(&ParseItem::Comment(
            ItemRange::fixed_start("//").pre_fixed_end("\n"),
            false,
        )),
        ParseItem::UnEscaped(&ParseItem::Comment(
            ItemRange::fixed_start("/*").fixed_end("*/"),
            false,
        )),
        ParseItem::Escaped(&ParseItem::String(
            ItemRange::fixed_start("\"\"\"").fixed_end("\"\"\""),
            false,
        )),
        ParseItem::UnEscaped(&ParseItem::String(
            ItemRange::fixed_start("\"").fixed_end("\""),
            false,
        )),
    ];
    fn is_meaningful_src(src: &str) -> bool {
        !src.chars().all(|ch| {
            char::is_whitespace(ch) || ch == '}' || ch == '{'
        })
    }
}

pub struct C;
impl Language for C {
    const PARSE_ITEMS: &'static [ParseItem] = &[
        ParseItem::Escaped(&ParseItem::Comment(
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
        ParseItem::UnEscaped(&ParseItem::String(
            ItemRange::start_matcher(
                Matcher::Exact("R\""),
                Matcher::AnyAlphaNumeric,
                Matcher::Exact("("),
            )
            .end_matcher(
                Matcher::Exact(r#")"#),
                Matcher::AnyAlphaNumeric,
                Matcher::Exact(r#"""#),
            ),
            true,
        )), // R"UNIQUE_KEY( RAW STRING )UNIQUE_KEY"
    ];
    fn is_meaningful_src(src: &str) -> bool {
        !src.chars().all(|ch| {
            char::is_whitespace(ch) || ch == '}' || ch == '{'
        })
    }    
}

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
            ItemRange::fixed_start(r#"""#).fixed_end(r#"""#),
            false,
        )),
        ParseItem::Escaped(&ParseItem::String(
            ItemRange::fixed_start(r#"b""#).fixed_end(r#"""#),
            false,
        )),
        ParseItem::UnEscaped(&ParseItem::String(
            ItemRange::start_matcher(
                Matcher::Exact(r#"r"#),
                Matcher::Repeat("#"),
                Matcher::Exact(r#"""#),
            )
            .end_matcher(Matcher::Exact(r#"""#), Matcher::Repeat("#"), Matcher::Any),
            true,
        )),
    ];
    fn is_meaningful_src(src: &str) -> bool {
        !src.chars().all(|ch| {
            char::is_whitespace(ch) || ch == '}' || ch == '{'
        })
    }
}

pub struct Python;
impl Language for Python {
    const PARSE_ITEMS: &'static [ParseItem] = &[
        ParseItem::UnEscaped(&ParseItem::Comment(
            ItemRange::fixed_start("\"\"\"").fixed_end("\"\"\""),
            false,
        )),
        ParseItem::UnEscaped(&ParseItem::Comment(
            ItemRange::fixed_start("#").pre_fixed_end("\n"),
            false,
        )),
        ParseItem::Escaped(&ParseItem::String(
            ItemRange::fixed_start("\"").fixed_end("\""),
            false,
        )),
    ];
}
