use core::fmt::Debug;

#[cfg(backtrace)]
use std::backtrace::Backtrace;
use std::marker::PhantomData;

#[derive(Clone, Copy, Debug)]
pub struct Span {
    pub start: usize,
    pub end: usize,
}

impl Span {
    pub fn new(start: usize, end: usize) -> Self {
        Self { start, end }
    }
}

pub type Match = Span;
pub type Matches = [Match; 3];

pub enum Matcher {
    Exact(&'static str),
    PreExact(&'static str),
    Repeat(&'static str),
    Fn(&'static dyn Fn(&str) -> Option<&str>),
    AnyAlphaNumeric,
    Empty, // TODO: also add Regex(regex::Regex) for greater comfort as an optional feature
}

impl Matcher {
    /// All matches use this
    fn get_match<'a>(&self, src: &'a str) -> Option<&'a str> {
        match self {
            Matcher::Exact(s) => {
                if src.starts_with(s) {
                    Some(&src[0..s.len()])
                } else {
                    None
                }
            }
            Matcher::PreExact(s) => {
                if src.starts_with(s) {
                    Some(&src[..s.len() - 1])
                } else if s.eq(&"\n") && src.len() == 1 {
                    // only pre-exact is newline,
                    // and we know what the src has ended!
                    Some(&src[..s.len()])
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
                Some(&src[..i])
            }
            Matcher::AnyAlphaNumeric => src
                .char_indices()
                .find(|(_, c)| !c.is_alphanumeric())
                .map(|(i, _)| &src[..i])
                .or(Some(src)),
            Matcher::Empty => Some(""),
        }
    }
}

impl Debug for Matcher {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::PreExact(arg0) => f.debug_tuple("PreExact").field(arg0).finish(),
            Self::Exact(arg0) => f.debug_tuple("Str").field(arg0).finish(),
            Self::Repeat(arg0) => f.debug_tuple("Repeat").field(arg0).finish(),
            Self::Fn(_) => f.debug_tuple("Fn").finish(),
            Self::AnyAlphaNumeric => write!(f, "AnyAlphaNumeric"),
            Self::Empty => write!(f, "Empty"),
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
        let end_match = self.end.get_match(&src[s2..])?;
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
/// use dracula::parse::*;
/// const C_COMMENT: ParseItem = ParseItem::Comment(ItemRange::fixed_start("//").pre_fixed_end("\n"), false);
/// const C_COMMENT_MULTI_LINE: ParseItem = ParseItem::UnEscaped(&ParseItem::Comment(ItemRange::fixed_start("/*").fixed_end("*/"), false));
/// ```
#[derive(Debug)]
pub enum ParseItem {
    // Second argument is the if we should pick the key
    // for range matcher from the starting matcher
    Comment(ItemRange, bool),
    String(ItemRange, bool),
    // WithInner is for ParseItem that contain something else inside them,
    // for which we need a separate parse.
    WithInner(&'static ParseItem, &'static ParseItem),
    // SameAsSrcInterpolation is added just for Ruby for now
    SameAsSrcInterpolation(ItemRange),
    // Represents things like format strings, or the general case of
    // embedded DSLs which interpolate meaningful source within themselves.
    InSource(ItemRange, bool),
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
                key: Matcher::Empty,
                end: Matcher::Empty,
            },
        }
    }
    pub const fn pre_fixed_end(self, src: &'static str) -> ItemRange {
        assert!(src.len() > 0);
        ItemRange {
            begin: self.begin,
            end: EndPoint {
                start: Matcher::PreExact(src),
                key: Matcher::Empty,
                end: Matcher::Empty,
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
                key: Matcher::Empty,
                end: Matcher::Empty,
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
            Self::String(s, _)
            | Self::Comment(s, _)
            | Self::InSource(s, _)
            | Self::SameAsSrcInterpolation(s) => &s.begin,
            Self::Escaped(item) | Self::UnEscaped(item) | Self::WithInner(item, _) => item.begin(),
        }
    }
    pub fn end(&self) -> &EndPoint {
        match self {
            Self::String(s, _)
            | Self::Comment(s, _)
            | Self::InSource(s, _)
            | Self::SameAsSrcInterpolation(s) => &s.end,
            Self::Escaped(item) | Self::UnEscaped(item) | Self::WithInner(item, _) => item.end(),
        }
    }
    pub fn is_keyed(&self) -> bool {
        match self {
            Self::Escaped(k) | Self::UnEscaped(k) => k.is_keyed(),
            Self::String(_, true) | Self::Comment(_, true) => true,
            _ => false,
        }
    }
    pub fn to_parse_output<'a>(&self, src: &'a str) -> ParseOutput<'a> {
        match self {
            Self::Comment(..) => ParseOutput::Comment(src),
            Self::String(..) => ParseOutput::String(src),
            Self::Escaped(pi) | Self::UnEscaped(pi) | Self::WithInner(pi, _) => {
                pi.to_parse_output(src)
            }
            _ => ParseOutput::Source(src),
        }
    }
    pub fn is_escaped(&self) -> bool {
        matches!(self, ParseItem::Escaped(_))
    }
}

#[derive(Debug, Clone, Copy)]
pub enum ParseOutput<'a> {
    Comment(&'a str),
    String(&'a str),
    Source(&'a str),
    Invalid(usize, usize),
    EOL(&'a str),
    EOF,
}

impl ParseOutput<'_> {
    pub fn is_meaningful<L: Language>(&self) -> bool {
        match self {
            Self::Source(src) => L::is_meaningful_src(src),
            _ => false,
        }
    }
    pub fn len(&self) -> usize {
        match self {
            Self::Comment(s) | Self::String(s) | Self::Source(s) => s.len(),
            Self::EOL(_) => 1,
            Self::Invalid(..) | Self::EOF => 0,
        }
    }
}

pub trait Language: Sized {
    const PARSE_ITEMS: &'static [ParseItem];
    fn is_meaningful_src(src: &str) -> bool {
        src.chars().next().is_some() && !src.chars().all(char::is_whitespace)
    }
    fn get_parser(src: &str) -> Parser<Self> {
        Parser::<Self>::new(src)
    }
    fn is_meaningful(parse_output: &ParseOutput) -> bool {
        ParseOutput::is_meaningful::<Self>(parse_output)
    }
}

#[derive(Debug, Default)]
pub struct Parser<'a, L: Language> {
    src: &'a str,
    index: usize,
    language_items: &'static [ParseItem],
    _marker: PhantomData<L>,
}

// most this is only used in tests atm!
impl<L: Language> Parser<'_, L> {
    /// Creates a new [`Parser`].
    pub fn new<'a>(src: &'a str) -> Parser<'a, L> {
        Parser {
            src,
            language_items: L::PARSE_ITEMS,
            index: 0,
            _marker: PhantomData::default(),
        }
    }

    /// Try to parse as per the given grammar.
    /// This function will return an error if parsing as the given grammar fails
    fn parse_next<'a>(&self, src: &'a str) -> Result<ParseOutput<'a>, String> {
        let items = self.language_items;
        if src.starts_with('\n') {
            Ok(ParseOutput::EOL(&src[..1]))
        } else if let Some((i, b, end_matches)) = (0..items.len())
            .find_map(|i| Some((i, items[i].begin().matches(src)?)))
            .and_then(|(i, matches)| {
                let mut escape = false;
                (matches[2].end..src.len()).find_map(|b| {
                    if src.is_char_boundary(b) && !escape {
                        if items[i].is_escaped() && src[b..].starts_with('\\') {
                            escape = true;
                            return None;
                        }
                        Some((
                            i,
                            b,
                            if items[i].is_keyed() {
                                items[i].end().matches_with_key(
                                    &src[b..],
                                    &src[matches[1].start..matches[1].end],
                                )?
                            } else {
                                items[i].end().matches(&src[b..])?
                            },
                        ))
                    } else {
                        if escape {
                            escape = false;
                        }
                        None
                    }
                })
            })
        {
            Ok(items[i].to_parse_output(&src[..b + end_matches[2].end]))
        } else if let Some(end) = (1..=src.len()).find(|&idx| {
            src.is_char_boundary(idx)
                && (idx == src.len()
                    || src[idx..].starts_with('\n')
                    || items
                        .iter()
                        .find_map(|i| i.begin().matches(&src[idx..]))
                        .is_some())
        }) {
            // if it's not a range then it's a source line
            if end == 0 {
                Err("Failed to parse, for some random reason, pls lookie here")?;
            }
            Ok(ParseOutput::Source(&src[..end]))
        } else {
            Err("Failed to parse the rest.")?
        }
    }
}

/// Implementation of Iterator over Parser to allow pull-parsing of the source
impl<'a, L: Language> Iterator for Parser<'a, L> {
    type Item = ParseOutput<'a>;

    fn next(&mut self) -> Option<Self::Item> {
        if self.index > self.src.len() {
            None
        } else if self.index == self.src.len() {
            self.index = self.src.len() + 1;
            Some(ParseOutput::EOF)
        } else {
            let parse_output = self.parse_next(&self.src[self.index..]);
            self.index += parse_output.as_ref().map(|x| x.len()).unwrap_or_default();
            Some(parse_output.unwrap_or_else(|_| {
                let x = ParseOutput::Invalid(self.index, self.src.len());
                self.index = self.src.len();
                x
            }))
        }
    }
}

trait IntoString {
    fn into_string(self) -> String;
}

impl IntoString for &'_ str {
    fn into_string(self) -> String {
        self.to_string()
    }
}

impl IntoString for &'_ String {
    fn into_string(self) -> String {
        self.to_owned()
    }
}
