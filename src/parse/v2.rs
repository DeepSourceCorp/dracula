#[derive(Debug)]
enum TreeSitterLanguage {
    C,
    Cpp,
    Rust,
    Java,
    Javascript,
    CSharp,
    Scala,
    Ruby,
    Python,
    Go,
}

pub struct Parser {
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

impl Parser {
    fn new(lang: TreeSitterLanguage) -> Option<Self> {
        let mut parser = tree_sitter::Parser::new();
        let lang = match lang {
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
        };
        parser.set_language(lang).ok()?;
        Some(Parser { parser })
    }
    //     *
    //     * *
    //   * * * * *
    // * * * * * * * *
    //     * * * * *

    fn get_meaningful_src(&mut self, src: &str) -> Option<String> {
        let tree = self.parser.parse(src, None)?;
        let cursor = tree.walk();
        traverse_iterative(cursor, Order::Pre, |x| {
            println!(">> {} {:?}", x.kind(), x.byte_range());
            println!();
            println!();
            println!();
            if ["comment", "string_literal", "raw_string_literal"].contains(&x.kind()) {
                false
            } else {
                true
            }
        });
        None
    }
}

#[test]
fn meaningful_src_test() {
    // let mut p = Parser::new(TreeSitterLanguage::Cpp).unwrap();
    // p.get_meaningful_src(
    //     r#"
    // #include "include"
    // // thins
    // int fn() {
    //     char* s = "";
    //     char* src = R"1(
    //         this si 
    //     )1";
    // }
    // /* this */
    // "#,
    // );
    let mut p = Parser::new(TreeSitterLanguage::Javascript).unwrap();
    p.get_meaningful_src(
        r#"
    function ft() {
        let x = `this is a ${
            10 // this is interesting
        } string literal`;
    }
    "#,
    );
}
