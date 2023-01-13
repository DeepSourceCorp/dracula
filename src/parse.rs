use core::fmt::Debug;

#[cfg(backtrace)]
use std::backtrace::Backtrace;

#[derive(Clone, Copy, Debug)]
pub struct Span {
    pub start: usize,
    pub end: usize,
}

impl Span {
    // TODO: don't think this is useful, but let's see
    // pub fn start(&self) -> usize {
    //     self.start
    // }
    // pub fn end(&self) -> usize {
    //     self.end
    // }
    pub fn new(start: usize, end: usize) -> Self {
        Self { start, end }
    }
}

pub type Match = Span;
pub type Matches = [Match; 3];

pub enum Matcher {
    Exact(&'static str),
    Repeat(&'static str),
    Fn(&'static dyn Fn(&str) -> Option<&str>),
    AnyAlphaNumeric,
    Any, // TODO: also add Regex(regex::Regex) for greater comfort as an optional feature
}

impl Matcher {
    /// All matches use this
    pub fn get_match<'a>(&self, src: &'a str) -> Option<&'a str> {
        match self {
            Matcher::Exact(s) => {
                if src.starts_with(s) {
                    Some(&src[0..s.len()])
                } else {
                    None
                }
            }
            Matcher::Fn(f) => f(src),
            Matcher::Repeat(s) => {
                let mut i = 0;
                while src[i..].starts_with(s) {
                    i += s.len();
                }
                // if atleast 1 occurence
                if i > 0 {
                    Some(&src[..i])
                } else {
                    // else no match
                    None
                }
            }
            Matcher::AnyAlphaNumeric => {
                let mut i = 0;
                while let Some(c) = src[i..].chars().next() {
                    if !c.is_alphanumeric() {
                        break;
                    } else {
                        i += 1;
                    }
                }
                Some(&src[..i]) // empty is also valid
            }
            Matcher::Any => Some(""),
        }
    }

    // TODO: again doesn't seem useful
    // till match is generally only used for end match and trying to get key from it.
    // pub fn till_match(&self, src: &str) -> Option<usize> {
    //     (0..src.len().min(256)).find_map(|x| {
    //         if self.get_match(&src[x..]).is_some() {
    //             Some(x)
    //         } else {
    //             None
    //         }
    //     })
    // }
}

impl Debug for Matcher {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Exact(arg0) => f.debug_tuple("Str").field(arg0).finish(),
            Self::Repeat(arg0) => f.debug_tuple("Repeat").field(arg0).finish(),
            Self::Fn(_) => f.debug_tuple("Fn").finish(),
            Self::AnyAlphaNumeric => write!(f, "AnyAlphaNumeric"),
            Self::Any => write!(f, "Any"),
        }
    }
}

#[derive(Debug)]
pub struct EndPoint {
    pub start: Matcher,
    pub key: Matcher,
    pub end: Matcher,
}

impl EndPoint {
    pub fn matches(&self, src: &str) -> Option<Matches> {
        let start_match = self.start.get_match(src)?;
        let s1 = start_match.len();
        let key_match = self.key.get_match(&src[s1..])?;
        let s2 = s1 + key_match.len();
        let end_match = self.key.get_match(&src[s2..])?;
        let s3 = s2 + end_match.len();
        Some([Span::new(0, s1), Span::new(s1, s2), Span::new(s2, s3)])
    }
    pub fn matches_with_key(&self, src: &str, key: &str) -> Option<Matches> {
        self.matches(src).and_then(|span| {
            let Span { start, end } = span[1];
            src[start..end].starts_with(key).then_some(span)
        })
    }
}
/// ParseItem stores information to have a "Range-Based" scanner for an item
/// Here, `&'static` is useful as it pushes the API user to build this structure as a const structure
///
/// usage:
/// ```rust
/// const C_COMMENT: ParseItem = ParseItem::Comment(ItemRange::begin("//").end("\n"));
/// const C_COMMENT_MULTI_LINE: ParseItem = ParseItem::UnEscaped(ParseItem::Comment(ItemRange::begin("/*").end("*/")));
/// ```
#[derive(Debug)]
pub enum ParseItem {
    /// second argument is the keyedness
    Comment(ItemRange, bool),
    String(ItemRange, bool),
    Escaped(&'static ParseItem),
    UnEscaped(&'static ParseItem),
}

#[derive(Debug)]
pub struct ItemRange {
    pub begin: EndPoint,
    pub end: EndPoint,
}

pub struct BuilderItemRange {
    begin: EndPoint,
}

impl BuilderItemRange {
    pub const fn fixed_end(self, src: &'static str) -> ItemRange {
        ItemRange {
            begin: self.begin,
            end: EndPoint {
                start: Matcher::Exact(src),
                key: Matcher::Any,
                end: Matcher::Any,
            },
        }
    }
    pub const fn end_matcher(self, start: Matcher, key: Matcher, end: Matcher) -> ItemRange {
        ItemRange {
            begin: self.begin,
            end: EndPoint { start, key, end },
        }
    }
}

impl ItemRange {
    pub const fn fixed_start(src: &'static str) -> BuilderItemRange {
        BuilderItemRange {
            begin: EndPoint {
                start: Matcher::Exact(src),
                key: Matcher::Any,
                end: Matcher::Any,
            },
        }
    }

    pub const fn start_matcher(start: Matcher, key: Matcher, end: Matcher) -> BuilderItemRange {
        BuilderItemRange {
            begin: EndPoint { start, key, end },
        }
    }
}

impl ParseItem {
    pub fn begin(&self) -> &EndPoint {
        match self {
            Self::String(s, _) | Self::Comment(s, _) => &s.begin,
            Self::Escaped(item) | Self::UnEscaped(item) => item.begin(),
        }
    }
    pub fn end(&self) -> &EndPoint {
        match self {
            Self::String(s, _) | Self::Comment(s, _) => &s.end,
            Self::Escaped(item) | Self::UnEscaped(item) => item.end(),
        }
    }
    pub fn is_key_matched(&self) -> bool {
        match self {
            Self::String(_, true) | Self::Comment(_, true) => true,
            _ => false,
        }
    }
    pub fn to_parse_output<'a>(&self, src: &'a str) -> ParseOutput<'a> {
        match self {
            Self::Comment(_, _) => ParseOutput::Comment(src),
            Self::String(_, _) => ParseOutput::String(src),
            Self::Escaped(pi) | Self::UnEscaped(pi) => pi.to_parse_output(src),
        }
    }
}

// Most of this is manually implemented elsewhere
// impl ParseItem {
//     // pub fn begin_match(&self, src: &str) -> Option<Matches> {
//     //     self.begin().matches(src)
//     // }
//     // pub fn till_end_match(&self, src: &str) -> Option<usize> {
//     //     (0..src.len().min(25600)).find_map(|x| {
//     //         self.end()
//     //             .matches(&src[x..])
//     //             .and_then(|x| x.last().copied())
//     //             .map(|x| x.end)
//     //     })
//     // }
// }

#[derive(Debug)]
pub enum ParseOutput<'a> {
    Comment(&'a str),
    String(&'a str),
    Source(&'a str),
    Invalid(&'a str),
    EOL(&'a str),
    EOF,
}

impl ParseOutput<'_> {
    // pub fn invalid(err: impl AsRef<str>) -> Self {
    //     Self::Invalid(err.as_ref().to_string())
    // }
    pub fn len(&self) -> usize {
        match self {
            Self::Comment(s) | Self::String(s) | Self::Source(s) => s.len(),
            Self::EOL(_) => 1,
            Self::Invalid(_) | Self::EOF => 0,
        }
    }
}

pub trait Language {
    const PARSE_ITEMS: &'static [ParseItem];
}

pub struct C;
impl Language for C {
    const PARSE_ITEMS: &'static [ParseItem] = &[
        ParseItem::Comment(ItemRange::fixed_start("//").fixed_end("\n"), false),
        ParseItem::Comment(ItemRange::fixed_start("/*").fixed_end("*/"), false),
        ParseItem::String(ItemRange::fixed_start(r#"""#).fixed_end(r#"""#), false),
        ParseItem::UnEscaped(&ParseItem::String(
            ItemRange::start_matcher(
                Matcher::Exact(r#"R""#),
                Matcher::AnyAlphaNumeric,
                Matcher::Exact("("),
            )
            .end_matcher(
                Matcher::Exact(r#")"#),
                Matcher::AnyAlphaNumeric,
                Matcher::Exact(r#"""#),
            ),
            true,
        )),
    ];
}

pub struct Rust;
impl Language for Rust {
    const PARSE_ITEMS: &'static [ParseItem] = &[
        ParseItem::Comment(ItemRange::fixed_start("//").fixed_end("\n"), false),
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
        ParseItem::Comment(ItemRange::fixed_start(r#"""""#).fixed_end(r#"""""#), false),
        ParseItem::Comment(ItemRange::fixed_start("#").fixed_end("\n"), false),
        ParseItem::String(ItemRange::fixed_start("\"").fixed_end("\""), false),
    ];
}

#[derive(Debug)]
pub struct Parser<'a> {
    src: &'a str,
    index: usize,
    language_items: &'static [ParseItem],
}

// most this is only used in tests atm!
impl Parser<'_> {
    /// Creates a new [`Parser`].
    pub fn new<'a, L: Language>(src: &'a str) -> Parser<'a> {
        Parser {
            src,
            language_items: L::PARSE_ITEMS,
            index: 0,
        }
    }

    pub fn parse<'a>(&self, src: &'a str) -> Result<ParseOutput<'a>, String> {
        let items = self.language_items;
        if src.starts_with('\n') {
            Ok(ParseOutput::EOL(&src[..1]))
        } else if let Some((i, b, end_matches)) = (0..items.len())
            .find_map(|i| Some((i, items[i].begin().matches(src)?)))
            .and_then(|(i, matches)| {
                (matches[2].end..src.len()).find_map(|b| {
                    Some((
                        i,
                        b,
                        if items[i].is_key_matched() {
                            items[i].end().matches_with_key(
                                &src[b..],
                                &src[matches[1].start..matches[1].end],
                            )?
                        } else {
                            items[i].end().matches(&src[b..])?
                        },
                    ))
                })
            })
        {
            Ok(items[i].to_parse_output(&src[0..b + end_matches[2].end]))
        } else if let Some(end) = (1..=src.len()).find(|&idx| {
            idx == src.len()
                || src[idx..].starts_with('\n')
                || items
                    .iter()
                    .find_map(|i| i.begin().matches(&src[idx..]))
                    .is_some()
        }) {
            // if it's not a range then it's a source line
            if end == 0 {
                Err("Failed to parse, for some random reason, pls lookie here")?;
            }
            Ok(ParseOutput::Source(&src[0..end]))
        } else {
            Err("Failed to parse the rest.")?
        }
    }
}

/// Implementation of Iterator over Parser to allow pull-parsing of the source
impl<'a> Iterator for Parser<'a> {
    type Item = ParseOutput<'a>;

    fn next(&mut self) -> Option<Self::Item> {
        if self.index > self.src.len() {
            None
        } else if self.index == self.src.len() {
            self.index = self.src.len() + 1;
            Some(ParseOutput::EOF)
        } else {
            let parse_output = self.parse(&self.src[self.index..]);
            self.index += parse_output.as_ref().map(|x| x.len()).unwrap_or_default();
            Some(parse_output.unwrap_or_else(|_| {
                let x = ParseOutput::Invalid(&self.src[self.index..]);
                self.index = self.src.len();
                x
            }))
        }
    }
}
