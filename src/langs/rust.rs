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
            .all(|ch| char::is_whitespace(ch) || ch == '}' || ch == '{')
    }
}

// todo keep working on more of these features
#[cfg(feature = "v2")]
mod _v2 {

    enum OutputKind {
        STRING,
        COMMENT,
        SOURCE,
    }

    struct MatchTree {
        matcher: Matcher,
        save: Option<&'static str>,
        or: Option<&'static MatchTree>,
        then: Option<&'static MatchTree>,
        from_saved: Option<&'static str>,
    }
    impl MatchTree {
        const fn new(matcher: Matcher) -> Self {
            Self {
                matcher,
                save: None,
                or: None,
                then: None,
                from_saved: None,
            }
        }
        const fn save(mut self, key: &'static str) -> Self {
            _ = self.save.get_or_insert(key);
            self
        }
        const fn or(mut self, matchtree: &'static MatchTree) -> Self {
            _ = self.or.get_or_insert(matchtree);
            self
        }
        const fn then(mut self, matchtree: &'static MatchTree) -> Self {
            _ = self.then.get_or_insert(matchtree);
            self
        }
        const fn from_saved(key: &'static str) -> Self {
            Self {
                matcher: Matcher::Empty,
                from_saved: Some(key),
                save: None,
                or: None,
                then: None,
            }
        }
    }

    struct LanguageItem {
        output: OutputKind,
        escaped: bool,
        start: MatchTree,
        end: MatchTree,
    }

    struct LanguageItemBuilder {
        output: OutputKind,
        escaped: bool,
        start: Option<MatchTree>,
        end: Option<MatchTree>,
    }

    impl LanguageItemBuilder {
        const fn new(output: OutputKind) -> Self {
            Self {
                escaped: false,
                output,
                start: None,
                end: None,
            }
        }
        const fn escaped(mut self) -> Self {
            self.escaped = true;
            self
        }
        const fn start(mut self, matchtree: MatchTree) -> Self {
            _ = self.start.get_or_insert(matchtree);
            self
        }
        const fn end(mut self, matchtree: MatchTree) -> Self {
            _ = self.end.get_or_insert(matchtree);
            self
        }
        const fn build(self) -> LanguageItem {
            LanguageItem {
                output: self.output,
                escaped: self.escaped,
                start: self.start.unwrap(),
                end: self.end.unwrap_or(MatchTree::new(Matcher::Empty)),
            }
        }
    }

    trait LanguageV2 {
        const LANGUAGE_STATES: &'static [LanguageItem];
    }
    const RAW_OR_BINARY: MatchTree =
        MatchTree::new(Matcher::Exact("r")).or(&MatchTree::new(Matcher::Exact("b")));
    const RAW_OR_BINARY_STR_START: MatchTree = RAW_OR_BINARY.then(
        &MatchTree::new(Matcher::AnyAlphaNumeric)
            .save("raw_str_key")
            .then(&MatchTree::new(Matcher::Exact("\""))),
    );

    const RAW_OR_BINARY_STR_END: MatchTree =
        MatchTree::new(Matcher::Exact("\"")).then(&MatchTree::from_saved("raw_str_key"));

    impl LanguageV2 for Rust {
        const LANGUAGE_STATES: &'static [LanguageItem] = &[
            LanguageItemBuilder::new(OutputKind::COMMENT)
                .start(RAW_OR_BINARY_STR_START.then(&RAW_OR_BINARY_STR_START))
                .build(),
            LanguageItemBuilder::new(OutputKind::COMMENT)
                .start(MatchTree::new(Matcher::Empty))
                .build(),
            LanguageItemBuilder::new(OutputKind::COMMENT)
                .start(MatchTree::new(Matcher::Empty))
                .build(),
        ];
    }
}

#[cfg(feature = "v2")]
pub use _v2::*;