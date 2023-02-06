use crate::parse::*;

pub struct Java;
impl Language for Java {
    const PARSE_ITEMS: &'static [ParseItem] = &[
        ParseItem::Comment(ItemRange::fixed_start("//").pre_fixed_end("\n"), false),
        ParseItem::Comment(ItemRange::fixed_start("/*").fixed_end("*/"), false),
        ParseItem::String(ItemRange::fixed_start("\"\"\"").fixed_end("\"\"\""), false),
        ParseItem::String(ItemRange::fixed_start("\"").fixed_end("\""), false),
    ];
}

pub struct C;
impl Language for C {
    const PARSE_ITEMS: &'static [ParseItem] = &[
        ParseItem::Comment(ItemRange::fixed_start("//").pre_fixed_end("\n"), false),
        ParseItem::Comment(ItemRange::fixed_start("/*").fixed_end("*/"), false),
        ParseItem::String(ItemRange::fixed_start("\"").fixed_end("\""), false),
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
}

pub struct Rust;
impl Language for Rust {
    const PARSE_ITEMS: &'static [ParseItem] = &[
        ParseItem::Comment(ItemRange::fixed_start("//").pre_fixed_end("\n"), false),
        ParseItem::Comment(ItemRange::fixed_start("/*").fixed_end("*/"), false),
        ParseItem::String(ItemRange::fixed_start(r#"""#).fixed_end(r#"""#), false),
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
}

pub struct Python;
impl Language for Python {
    const PARSE_ITEMS: &'static [ParseItem] = &[
        ParseItem::Comment(ItemRange::fixed_start("\"\"\"").fixed_end("\"\"\""), false),
        ParseItem::Comment(ItemRange::fixed_start("#").pre_fixed_end("\n"), false),
        ParseItem::String(ItemRange::fixed_start("\"").fixed_end("\""), false),
    ];
}
