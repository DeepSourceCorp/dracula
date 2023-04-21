
pub struct Span {
    pub start: usize,
    pub end: usize,
}

impl Span {
    pub fn new(start: usize, end: usize) -> Span {
        Span { start, end }
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

pub struct Range {
    start: Matcher,
    end: Matcher,
    inner: Option<&'static Range>
}


