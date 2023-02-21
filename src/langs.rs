use crate::parse::*;

/// Java syntax doesn't have escaped ranges other
/// than simple string
pub struct Java;
impl Language for Java {
    const PARSE_ITEMS: &'static [ParseItem] = &[
        // single line comment
        ParseItem::UnEscaped(&ParseItem::Comment(
            ItemRange::fixed_start("//").pre_fixed_end("\n"),
            false,
        )),
        // multi-line comment
        ParseItem::UnEscaped(&ParseItem::Comment(
            ItemRange::fixed_start("/*").fixed_end("*/"),
            false,
        )),
        // multi-line string (defined above simple string as
        // parsing this has precedence over simple string(`".*"`))
        ParseItem::UnEscaped(&ParseItem::String(
            ItemRange::fixed_start("\"\"\"").fixed_end("\"\"\""),
            false,
        )),
        // simple string
        ParseItem::Escaped(&ParseItem::String(
            ItemRange::fixed_start("\"").fixed_end("\""),
            false,
        )),
    ];
    fn is_meaningful_src(src: &str) -> bool {
        !src.chars()
            .all(|ch| char::is_whitespace(ch) || ch == '}' || ch == '{')
    }
}

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
            .all(|ch| char::is_whitespace(ch) || ch == '}' || ch == '{')
    }
}

const PYTHON_STRING_START_MATCHER: Matcher = Matcher::Fn(
    &(|src| {
        if src.starts_with(['b', 'r', 'u']) {
            if src[1..].starts_with(['b', 'r', 'u']) {
                Some(&src[..2])
            } else {
                Some(&src[..1])
            }
        } else {
            Some("")
        }
    }),
);

const PYTHON_FORMAT_STRING_START_MATCHER: Matcher = Matcher::Fn(
    &(|src| {
        if src.starts_with("fr") || src.starts_with("rf") {
            Some(&src[..2])
        } else if src.starts_with("f") {
            Some(&src[..1])
        } else {
            None
        }
    }),
);

pub struct Python;
impl Language for Python {
    const PARSE_ITEMS: &'static [ParseItem] = &[
        ParseItem::UnEscaped(&ParseItem::String(
            ItemRange::start_matcher(
                PYTHON_STRING_START_MATCHER,
                Matcher::Empty,
                Matcher::Exact("\"\"\""),
            )
            .end_matcher(Matcher::Exact("\"\"\""), Matcher::Empty, Matcher::Empty),
            false,
        )),
        ParseItem::UnEscaped(&ParseItem::String(
            ItemRange::start_matcher(
                PYTHON_STRING_START_MATCHER,
                Matcher::Empty,
                Matcher::Exact("'''"),
            )
            .end_matcher(Matcher::Exact("'''"), Matcher::Empty, Matcher::Empty),
            false,
        )),
        ParseItem::UnEscaped(&ParseItem::InSource(
            ItemRange::start_matcher(
                PYTHON_FORMAT_STRING_START_MATCHER,
                Matcher::Empty,
                Matcher::Exact("\"\"\""),
            )
            .end_matcher(Matcher::Exact("\"\"\""), Matcher::Empty, Matcher::Empty),
            false,
        )),
        ParseItem::UnEscaped(&ParseItem::InSource(
            ItemRange::start_matcher(
                PYTHON_FORMAT_STRING_START_MATCHER,
                Matcher::Empty,
                Matcher::Exact("'''"),
            )
            .end_matcher(Matcher::Exact("'''"), Matcher::Empty, Matcher::Empty),
            false,
        )),
        ParseItem::UnEscaped(&ParseItem::Comment(
            ItemRange::fixed_start("#").pre_fixed_end("\n"),
            false,
        )),
        ParseItem::Escaped(&ParseItem::String(
            ItemRange::start_matcher(
                PYTHON_STRING_START_MATCHER,
                Matcher::Empty,
                Matcher::Exact("\""),
            )
            .end_matcher(Matcher::Exact("\""), Matcher::Empty, Matcher::Empty),
            false,
        )),
        ParseItem::Escaped(&ParseItem::String(
            ItemRange::start_matcher(
                PYTHON_STRING_START_MATCHER,
                Matcher::Empty,
                Matcher::Exact("'"),
            )
            .end_matcher(Matcher::Exact("'"), Matcher::Empty, Matcher::Empty),
            false,
        )),
        ParseItem::Escaped(&ParseItem::InSource(
            ItemRange::start_matcher(
                PYTHON_FORMAT_STRING_START_MATCHER,
                Matcher::Empty,
                Matcher::Exact("\""),
            )
            .end_matcher(Matcher::Exact("\""), Matcher::Empty, Matcher::Empty),
            false,
        )),
        ParseItem::Escaped(&ParseItem::InSource(
            ItemRange::start_matcher(
                PYTHON_FORMAT_STRING_START_MATCHER,
                Matcher::Empty,
                Matcher::Exact("'"),
            )
            .end_matcher(Matcher::Exact("'"), Matcher::Empty, Matcher::Empty),
            false,
        )),
    ];
}
