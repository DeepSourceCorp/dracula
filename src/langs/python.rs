use crate::parse::{
    Matcher, ParseItem, Language,ItemRange,
};

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
