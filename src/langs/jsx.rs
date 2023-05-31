use crate::parse::{ItemRange, Language, ParseItem};

// we don't parse raw strings atm here,
// as generally they are meaningful
pub struct JSX;
impl Language for JSX {
    const PARSE_ITEMS: &'static [ParseItem] = &[
        ParseItem::UnEscaped(&ParseItem::Comment(
            ItemRange::fixed_start("<!--").pre_fixed_end("-->"),
            false,
        )),
        ParseItem::UnEscaped(&ParseItem::Comment(
            ItemRange::fixed_start("//").pre_fixed_end("\n"),
            false,
        )),
        ParseItem::Escaped(&ParseItem::String(
            ItemRange::fixed_start("\"").fixed_end("\""),
            false,
        )),
        ParseItem::Escaped(&ParseItem::String(
            ItemRange::fixed_start("\'").fixed_end("\'"),
            false,
        )),
        ParseItem::UnEscaped(&ParseItem::Comment(
            ItemRange::fixed_start("/*").fixed_end("*/"),
            false,
        )),
    ];
}
