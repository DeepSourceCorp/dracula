#[derive(Debug)]
pub enum TreeSitterLanguage {
    C,
    Kotlin,
    Cpp,
    Rust,
    Java,
    Javascript,
    JSX,
    Typescript,
    TSX,
    CSharp,
    Scala,
    Ruby,
    Python,
    Go,
}

pub struct Parser {
    lang: TreeSitterLanguage,
    parser: tree_sitter::Parser,
}

#[derive(PartialEq, Eq)]
enum Order {
    Root,
    Pre,
    Post,
}

fn traverse_iterative<F>(mut c: tree_sitter::TreeCursor, order: Order, mut cb: F)
where
    F: FnMut(tree_sitter::Node) -> bool,
{
    loop {
        let mut step_into_node = true;
        // This is the first time we've encountered the node, so we'll call if preorder
        if order == Order::Pre {
            step_into_node = cb(c.node());
        }

        // Keep travelling down the tree as far as we can
        if step_into_node && c.goto_first_child() {
            continue;
        }

        let mut node = c.node();

        // If we can't travel any further down, try going to next sibling and repeating
        if c.goto_next_sibling() {
            // If we succeed in going to the previous nodes sibling,
            // we won't be encountering that node again, so we'll call if postorder
            if order == Order::Post {
                cb(node);
            }
            continue;
        }

        // Otherwise, we must travel back up; we'll loop until we reach the root or can
        // go to the next sibling of a node again.
        loop {
            // Since we're retracing back up the tree, this is the last time we'll encounter
            // this node, so we'll call if postorder
            if order == Order::Post {
                cb(node);
            }
            if !c.goto_parent() {
                // We have arrived back at the root, so we are done.
                return;
            }

            node = c.node();

            if c.goto_next_sibling() {
                // If we succeed in going to the previous node's sibling,
                // we will go back to travelling down that sibling's tree, and we also
                // won't be encountering the previous node again, so we'll call if postorder
                if order == Order::Post {
                    cb(node);
                }
                break;
            }
        }
    }
}

fn has_overlapping_range(
    start: usize,
    end: usize,
    ranges: &mut Vec<std::ops::Range<usize>>,
) -> Option<Vec<std::ops::Range<usize>>> {
    let mut ret_ranges = vec![];
    let mut remove_till = 0;
    for range in ranges.iter() {
        // println!("{}..{} :: {:?}", start, end, range);
        if range.end < start {
            // pop range
            remove_till += 1;
        } else if start <= range.end && range.start <= end {
            let s = start.max(range.start) - start;
            let e = end.min(range.end) - start;
            ret_ranges.push(s..e);
        } else if end < range.start {
            break;
        }
    }
    if ret_ranges.len() == 0 {
        return None;
    }
    ranges.drain(..remove_till);
    Some(ret_ranges)
}

pub fn get_list_of_meaningful_lines(
    src: &str,
    mut useless_ranges: Vec<std::ops::Range<usize>>,
) -> Vec<usize> {
    let lines = src.split_inclusive("\n");
    let mut line_index = 1usize;
    let mut start = 0usize;
    let mut meaningful_lines = vec![];
    lines.for_each(|line| {
        let end = start + line.len();
        if line.chars().all(char::is_whitespace) {
            // blank line
        } else if let Some(ranges) = has_overlapping_range(start, end, &mut useless_ranges) {
            let mut is_line_meaningless = true;
            let mut ranges = ranges.into_iter();
            if let Some(mut curr) = ranges.next() {
                is_line_meaningless &= line[0..curr.start].chars().all(char::is_whitespace);
                for next in ranges {
                    // got overlapping range
                    is_line_meaningless &= next.start > line.len()
                        || line[curr.end..next.start].chars().all(char::is_whitespace);
                    curr = next;
                }
                is_line_meaningless &=
                    curr.end > line.len() || line[curr.end..].chars().all(char::is_whitespace);
            }
            if !is_line_meaningless {
                meaningful_lines.push(line_index);
            }
        } else {
            meaningful_lines.push(line_index);
        }
        start = end;
        line_index += 1;
    });
    meaningful_lines
}

impl Parser {
    pub fn new(lang: TreeSitterLanguage) -> Option<Self> {
        let mut parser = tree_sitter::Parser::new();
        let tlang = match lang {
            TreeSitterLanguage::C => tree_sitter_c::language(),
            TreeSitterLanguage::Cpp => tree_sitter_cpp::language(),
            TreeSitterLanguage::Rust => tree_sitter_rust::language(),
            TreeSitterLanguage::Java => tree_sitter_java::language(),
            TreeSitterLanguage::Javascript => tree_sitter_javascript::language(),
            TreeSitterLanguage::CSharp => tree_sitter_c_sharp::language(),
            TreeSitterLanguage::Scala => tree_sitter_scala::language(),
            TreeSitterLanguage::Ruby => tree_sitter_ruby::language(),
            TreeSitterLanguage::Python => tree_sitter_python::language(),
            TreeSitterLanguage::Go => tree_sitter_go::language(),
            TreeSitterLanguage::JSX => tree_sitter_javascript::language(),
            TreeSitterLanguage::Typescript => tree_sitter_typescript::language_typescript(),
            TreeSitterLanguage::TSX => tree_sitter_typescript::language_tsx(),
            TreeSitterLanguage::Kotlin => tree_sitter_kotlin::language(),
        };
        parser.set_language(tlang).ok()?;
        Some(Parser { lang, parser })
    }

    // returns a sorted list of meaningless sources (ascending order)
    pub fn get_spans_of_meaningless_source(
        &mut self,
        src: &str,
    ) -> Option<Vec<std::ops::Range<usize>>> {
        let tree = self.parser.parse(src, None)?;
        let cursor = tree.walk();
        let mut spans = vec![];
        traverse_iterative(cursor, Order::Pre, |x| {
            // println!(">> {} {:?}", x.to_sexp(), x.byte_range());
            // println!("");
            // println!();
            // println!("");
            if matches!(self.lang, TreeSitterLanguage::Kotlin)
                && x.kind() == "function_value_parameters"
            {
                return if x.to_sexp().contains("expression") {
                    true
                } else {
                    spans.push(x.byte_range());
                    false
                };
            }
            if [
                "comment",
                "string",
                "string_literal",
                "raw_string_literal",
                "line_comment",
                "block_comment",
                "formal_parameters",
                "=",
                "(",
                ")",
            ]
            .contains(&x.kind())
            {
                spans.push(x.byte_range());
                false
            } else {
                true
            }
        });
        Some(spans)
    }
}

#[test]
fn meaningful_src_test() {
    let src = r#"
    #include "include"
    // thins
    int fn() {
        char* s = "";
        char* src = R"1(
            this si
        )1";
    }
    /* this */
    "#;
    let mut p = Parser::new(TreeSitterLanguage::Cpp).unwrap();
    p.get_spans_of_meaningless_source(src);
    let lines = get_list_of_meaningful_lines(
        src,
        p.get_spans_of_meaningless_source(src).unwrap_or_default(),
    );
    display_lines(src, &lines);
    let mut p = Parser::new(TreeSitterLanguage::Java).unwrap();
    let src = r#"
    class St {
        /**/
        public static void main(
            // this is interesting
            String args[]
        ) {
            // test
            var x = "";
        }
    }
    "#;
    let lines = get_list_of_meaningful_lines(
        src,
        p.get_spans_of_meaningless_source(src).unwrap_or_default(),
    );
    display_lines(src, &lines);
    let mut p = Parser::new(TreeSitterLanguage::Kotlin).unwrap();
    let src = r#"
        fun /*is this code valid*/ a(
            x: Int = 1,
            // comment here!
            y: Int = let {
                return Unit
            },
            z: Int = 3
        ) = 
        Unit
        fun /*is this code valid*/ a(
            x: Int = 1,
            // comment here!
            y: Int = 2,
            z: Int = 3
        )
        = 
        Unit
    "#;
    let lines = get_list_of_meaningful_lines(
        src,
        p.get_spans_of_meaningless_source(src).unwrap_or_default(),
    );
    display_lines(src, &lines);
}

#[cfg(test)]
fn display_lines(src: &str, lines: &[usize]) {
    let mut line_index = 0;
    src.lines().enumerate().try_for_each(|(i, s)| {
        if line_index >= lines.len() {
            return None;
        }
        if lines[line_index] == i + 1 {
            line_index += 1;
            println!("{s}");
        }
        return Some(());
    });
}

#[cfg(test)]
fn display_ranges(src: &str, ranges: &Option<Vec<std::ops::Range<usize>>>) {
    for range in ranges.clone().unwrap() {
        println!(
            ">>>>>>>>>>>>>>>>>>\n{}\n<<<<<<<<<<<<<<<<<<\n",
            &src[range.start..range.end]
        );
    }
}
