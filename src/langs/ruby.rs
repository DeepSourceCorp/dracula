use crate::parse::{ItemRange, Language, ParseItem};

// I don't think parsing interpolation in strings out
// is a nice idea as they can be nested using interpolation
pub struct Ruby;
impl Language for Ruby {
    const PARSE_ITEMS: &'static [ParseItem] = &[
        ParseItem::UnEscaped(&ParseItem::Comment(
            ItemRange::fixed_start("#").pre_fixed_end("\n"),
            false,
        )),
        // we need to implemented nested parsing for this,
        // hence also having to clean up clean up the state machine code
        // ParseItem::Escaped(&ParseItem::String(
        //     ItemRange::fixed_start("\"").fixed_end("\""),
        //     false,
        // )),
        // ParseItem::Escaped(&ParseItem::String(
        //     ItemRange::fixed_start("\'").fixed_end("\'"),
        //     false,
        // )),
        // wip new feature to support strings in ruby properly
        // ParseItem::WithInner(
        //     &ParseItem::Escaped(&ParseItem::String(
        //         ItemRange::fixed_start("\"").fixed_end("\""),
        //         false,
        //     )),
        //     &ParseItem::SameAsSrcInterpolation(
        //         ItemRange::fixed_start("${").fixed_end("}"),
        //     )
        // ),
        ParseItem::UnEscaped(&ParseItem::Comment(
            ItemRange::fixed_start("\n=begin").fixed_end("\n=end"),
            false,
        )),
    ];
}
