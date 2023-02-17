//! unstable stablized rust api for count dracula

use crate::parse::{Language, ParseOutput, Parser};

#[derive(Default)]
struct Span {
    start: usize,
    end: usize,
}

#[derive(Default)]
pub struct ParseLineMeaningfulIndexIter<'a, L: Language> {
    parser: Parser<'a, L>,
    src: &'a str,
    line_span: Span,
    parse_span: Span,
    line_index: usize,
    max_lines: usize,
    last_parsed_output: Option<ParseOutput<'a>>,
    failed: bool,
}

impl<'a, L: Language> Iterator for ParseLineMeaningfulIndexIter<'a, L> {
    type Item = Option<usize>;

    fn next(&mut self) -> Option<Self::Item> {
        // Self::Item is Option<usize> to signify that a line may also not be meaningful.
        if self.failed {
            let li = self.line_index;
            if li > self.max_lines {
                return None;
            } else {
                return Some(Some(li));
            }
        }
        if self.line_span.end >= self.src.len() {
            return None;
        }
        self.src[self.line_span.end..]
            .split_inclusive('\n')
            .next()
            .map(|line| (line, self.line_index))
            .or(Some((self.src, self.line_index)))
            .map(|(line, idx)| {
                let last_parsed_output = self.last_parsed_output.clone();
                // setup line start and end
                self.line_span.start = self.line_span.end;
                self.line_span.end += line.len();
                // traverse parsed output until the span end is reached
                let mut po_stack = if let Some(po) = last_parsed_output {
                    vec![po]
                } else {
                    vec![]
                };
                while self.parse_span.end < self.line_span.end {
                    if let Some(parsed_output) = self.parser.next() {
                        // setup parsed start and end
                        self.parse_span.start = self.parse_span.end;
                        self.parse_span.end += parsed_output.len();
                        po_stack.push(parsed_output);
                    }
                }
                let is_meaningful_line = po_stack.iter().any(L::is_meaningful);
                if cfg!(dbg) {
                    eprintln!("{} == {}", idx, po_stack.iter().any(L::is_meaningful));
                    eprintln!("{:?}", po_stack);
                    eprintln!("-------------------------------------------------");
                }
                if self.parse_span.end != self.line_span.end {
                    self.last_parsed_output = po_stack.pop();
                } else {
                    self.last_parsed_output = None;
                }
                self.line_index += 1;
                if is_meaningful_line {
                    Some(idx)
                } else {
                    None
                }
            })
    }
}

/// Builds the iterator [`ParseLineMeaningfulIndexIter`] to run over lines of src while
/// figuring out meaningful lines from it
pub fn get_meaningful_line_indices<L: Language + 'static>(
    src: &str,
) -> ParseLineMeaningfulIndexIter<L> {
    ParseLineMeaningfulIndexIter {
        parser: Parser::<L>::new(src),
        src,
        line_span: Span::default(),
        parse_span: Span::default(),
        line_index: 0,
        last_parsed_output: None,
        failed: false,
        max_lines: src.lines().count(),
    }
}

/// Uses the [`Parser`] to try and figure out the parts of the source
/// that are meaningful
pub fn get_cleaned_source_code<L: Language>(src: &str) -> String {
    let parsed = L::get_parser(src);
    let mut meaningful_src = String::default();
    let mut stack = vec![];
    for p in parsed {
        if matches!(p, ParseOutput::Invalid(..)) {
            return src.to_string();
        }
        if matches!(p, ParseOutput::EOL(_) | ParseOutput::EOF) {
            let meaningful_src_len = meaningful_src.len();
            for po in stack.iter() {
                if let ParseOutput::Source(s) = po {
                    if L::is_meaningful_src(s) {
                        meaningful_src.push_str(s);
                    }
                }
            }
            if matches!(p, ParseOutput::EOL(_)) && meaningful_src_len != meaningful_src.len() {
                meaningful_src.push('\n');
            }
            stack.clear();
        } else {
            stack.push(p);
        }
    }
    meaningful_src
}

/// Uses the [`get_meaningful_line_indices`] function to build an iterator
/// and count all meaningful lines
pub fn get_count_of_meaningful_lines<L: Language + 'static>(src: &str) -> usize {
    get_meaningful_line_indices::<L>(src).flatten().count()
}

/// No halting tests for [`get_count_of_meaningful_lines`] as it uses
/// Iterator provided by [`get_meaningful_line_indices`]
#[cfg(test)]
mod halting_tests_count_api {
    use super::*;

    #[test]
    fn test_halting_get_cleaned_source_code() {
        get_cleaned_source_code::<crate::langs::C>("\nint main() {}\n");
        get_cleaned_source_code::<crate::langs::Rust>("\nfn main() {}\n");
        get_cleaned_source_code::<crate::langs::Python>("\ndef main():\n\tpass");
        get_cleaned_source_code::<crate::langs::Java>("\nvoid main() {}\n");
    }

    #[test]
    fn test_halting_get_meaningful_line_indices() {
        get_meaningful_line_indices::<crate::langs::C>("\nint main() {}\n")
            .flatten()
            .for_each(|_| ());
        get_meaningful_line_indices::<crate::langs::Rust>("")
            .flatten()
            .for_each(|_| ());
        get_meaningful_line_indices::<crate::langs::Python>("")
            .flatten()
            .for_each(|_| ());
        get_meaningful_line_indices::<crate::langs::Java>("")
            .flatten()
            .for_each(|_| ());
    }
}