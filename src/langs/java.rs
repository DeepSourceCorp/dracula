use crate::parse::{ItemRange, Language, ParseItem};

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
