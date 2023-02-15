#[cfg(test)]
mod simple_c {
    use crate::count;
    use crate::langs::*;

    #[test]
    fn try_parse() {
        let cnt = count::get_count_of_meaningful_lines::<C>(
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
        );
        assert_eq!(cnt, 10);
    }
}

#[cfg(test)]
mod simple_python {
    use crate::count::get_count_of_meaningful_lines;
    use crate::langs::*;

    #[test]
    fn try_parse() {
        let src = r#"# entp için anayzer
            if index == 10:
                " \
                "
                "\""
                pass
            # some top level comments
            def main():
                print("s");"""\""""""
                Multi-line Comments
                """
                print(x)
                """
                Multi-line Comments
                """
            "#;
        assert_eq!(get_count_of_meaningful_lines::<Python>(src), 5)
    }
}

#[cfg(test)]
mod simple_rust {
    use ra_ap_syntax::SyntaxKind;

    use crate::count;
    use crate::langs::*;

    #[test]
    fn try_parse() {
        let src = r##"
        // The default entry point for C programs
        // --------------------------------------
        // This generally requires C runtime for
        // the platform to be present.
        fn main() {
            let c = 2;
            /* Multi-Line Comments यह काम करना चाहिए
             seems to work as well */
             let यह = "hello, World!";
             let src = "Gello, World!";
             let src2 = r#"यह, काम!"#;
             return 0;
        }
    "##;
        let cnt = count::get_count_of_meaningful_lines::<Rust>(src);
        assert_eq!(cnt, meaningful_lines_in_src_using_ast(src).len());
    }

    fn meaningful_lines_in_src_using_ast(src: &str) -> Vec<usize> {
        let rs_src = ra_ap_syntax::SourceFile::parse(src);
        assert!(rs_src.errors().len() == 0);
        let mut begin;
        let mut end = 0;
        let mut v = vec![];
        for (idx, line) in src.split_inclusive('\n').enumerate() {
            begin = end;
            end += line.len();
            // useless span
            let (mut ub, ue) = (begin, end);
            let mut src = rs_src.syntax_node().clone().preorder_with_tokens();
            src.try_for_each(|nd| {
                if let ra_ap_syntax::WalkEvent::Enter(se) = nd {
                    match se {
                        ra_ap_syntax::NodeOrToken::Node(node) => {
                            if !node
                                .parent()
                                .map(|x| x.kind() != SyntaxKind::SOURCE_FILE)
                                .unwrap_or(true)
                            {}
                        }
                        ra_ap_syntax::NodeOrToken::Token(token) => {
                            if [
                                SyntaxKind::WHITESPACE,
                                SyntaxKind::COMMENT,
                                SyntaxKind::L_CURLY,
                                SyntaxKind::R_CURLY,
                                SyntaxKind::STRING,
                                SyntaxKind::BYTE_STRING,
                            ]
                            .contains(&token.kind())
                            {
                                let b: usize = token.text_range().start().into();
                                let e: usize = token.text_range().end().into();
                                // println!("<< {:?}, {:?}", token, (ub, ue));
                                if ub >= b && ue <= e {
                                    ub = ue;
                                    // println!(">> {:?}, {:?}", token, (ub, ue));
                                    return None; // exit this loop
                                } else if ub >= b && ub < e && ue > e {
                                    ub = e;
                                }
                                // println!(">> {:?}, {:?}", token, (ub, ue));
                            }
                        }
                    }
                }
                Some(())
            });
            if ue.saturating_sub(ub) > 0 {
                v.push(idx);
            }
        }
        v
    }

    #[test]
    fn run_on_self() {
        // run `dracula` vs `meaningful_lines_in_src_using_ast`
        // for as many rust files as possible
        // for starters all of this project should be a good start
        let src_files = [
            "./src/count.rs",
            "./src/langs.rs",
            "./src/lib.rs",
            "./src/parse.rs",
            "./src/tests.rs",
            "./src/remove_non_meaningful.rs",
            "./cdracula/src/lib.rs",
            "./cdracula/src/util_macros.rs",
            "./cdracula/tests/test_capi.rs",
            "./pydracula/src/lib.rs"
        ];
        src_files
            .into_iter()
            .map(|x| std::fs::read_to_string(dbg!(x)).ok())
            .flatten()
            .for_each(|src| {
                assert_eq!(
                    count::get_count_of_meaningful_lines::<Rust>(&src),
                    meaningful_lines_in_src_using_ast(&src).len()
                );
            });
    }
}
