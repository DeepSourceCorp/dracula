#[cfg(test)]
mod simple_c {
    use crate::parse::*;
    use crate::langs::*;

    #[test]
    fn try_parse() {
        Parser::new::<C>(
            r#"
            // The default entry point for C programs
            // --------------------------------------
            // This generally requires C runtime for
            // the platform to be present.
            int main(
                int x,
                int y,
                int z,
            ) {
                int c = 2;
                char* str = "hello from C!";
                char* str = R"(hello from
                     C!)";
                /* Multi-Line Comments
                 seems to work as well */
                return 0;
            }
        "#,
        )
        .enumerate()
        .for_each(|(i, x)| println!("{i}:: {x:?}"));
    }
}

#[cfg(test)]
mod simple_python {
    use crate::parse::*;
    use crate::langs::*;

    #[test]
    fn try_parse() {
        let parsed = Parser::new::<Python>(
            r#"# some top level comments
            def main():
                print("s");"""
                Multi-line Comments
                """
                print(x)
                """
                Multi-line Comments
                """
            "#,
        );
        let mut line_count: usize = 0;
        let mut stack = vec![];
        for p in parsed {
            if matches!(p, ParseOutput::EOL(_) | ParseOutput::EOF) {
                if stack.iter().any(|i| match i {
                    ParseOutput::Source(s) => Python::is_meaningful_src(s),
                    _ => false,
                }) {
                    line_count += 1;
                }
                stack.clear();
            } else {
                stack.push(p);
            }
        }
        assert_eq!(line_count, 3)
    }
}



#[cfg(test)]
mod simple_rust {
    use crate::parse::*;
    use crate::langs::*;

    #[test]
    fn try_parse() {
        Parser::new::<Rust>(
            r##"
            // The default entry point for C programs
            // --------------------------------------
            // This generally requires C runtime for
            // the platform to be present.
            fn main() {
                let c = 2;
                /* Multi-Line Comments
                 seems to work as well */
                 let src = "hello, World!";
                 let src2 = r#"hello, World!"#;
                 return 0;
            }
        "##,
        )
        .enumerate()
        .for_each(|(i, x)| println!("{i}:: {x:?}"));
    }
}
