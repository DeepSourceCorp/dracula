#[cfg(test)]
mod simple_c {
    use crate::count;
    use crate::langs::*;
    use crate::parse::v2::get_lines_without_ranges;
    use crate::parse::v2::Parser;
    use crate::parse::v2::TreeSitterLanguage;

    #[test]
    fn try_parse() {
        let src = r#"
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
        "#;
        // let lines = count::get_meaningful_line_indices::<C>(src)
        //     .flatten()
        //     .collect::<Vec<usize>>();
        // crate::parse::v2::display_lines(src, &lines);
        let cnt = count::get_count_of_meaningful_lines::<C>(src);
        let cnt_executable = {
            let mut parser = Parser::new(TreeSitterLanguage::C).unwrap();
            let ranges = parser.non_executable_src_spans(src).unwrap();
            let lines = get_lines_without_ranges(src, ranges);
            // crate::parse::v2::display_lines(src, &lines);
            lines.len()
        };
        assert_eq!(cnt, 10);
        assert_eq!(cnt_executable, 9); // we can now ignore parens and curlies
    }
}

#[cfg(test)]
mod simple_python {
    use crate::count::{self, get_cleaned_source_code};
    use crate::langs::*;
    use crate::parse::v2::get_lines_without_ranges;
    use crate::parse::v2::Parser;
    use crate::parse::v2::TreeSitterLanguage;
    use pretty_assertions::assert_eq;

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
                print("s"); # we can't provide illegal grammar any more
                """
                Multi-line Comments
                """
                print(x)
                """
                Multi-line Comments
                """
            "#;
        // let lines = count::get_meaningful_line_indices::<Python>(src)
        //     .flatten()
        //     .collect::<Vec<usize>>();
        // crate::parse::v2::display_lines(src, &lines);
        let cnt = count::get_count_of_meaningful_lines::<Python>(src);
        let cnt_executable = {
            let mut parser = Parser::new(TreeSitterLanguage::Python).unwrap();
            let ranges = parser.non_executable_src_spans(src).unwrap();
            let lines = get_lines_without_ranges(src, ranges);
            // crate::parse::v2::display_lines(src, &lines);
            lines.len()
        };
        assert_eq!(cnt, 5);
        assert_eq!(cnt_executable, 5);
    }

    #[test]
    fn cleaned_src_test_python() {
        let src = r###"# https://github.com/PyCQA/pylint/blob/main/tests/functional/a/anomalous_backslash_escape.py
        # https://github.com/PyCQA/pylint/blob/main/tests/functional/a/anomalous_unicode_escape.py
        # https://github.com/PyCQA/pylint/blob/main/tests/functional/u/unnecessary/unnecessary_dunder_call.py
        def foo( # this should go
          bar,
          car = "bar",
        ):
            """Docstring"""
            pass
        
        def test():
            x = "asd"
            x = 'asd'
            x = '''a"""
            sd"""
            '''
            x = """asd"""
            x = u"asd"
            x = u'asd'
            x = u'''asd'''
            x = u"""asd"""
            x = r"asd"
            x = r'asd'
            x = r'''asd'''
            x = r"""asd"""
            x = f"asd"
            x = f'asd'
            x = f'''asd"""
            kk"""
            '''
            x = f"""asd""" # new feature
            x = b"asd"
            x = b'asd'
            x = b'''asd'''
            x = b"""asd"""
            x = rb"asd"
            x = rb'asd'
            x = rb'''asd'''
            x = rb"""asd"""
            x = br"asd"
            x = br'asd'
            x = br'''asd'''
            x = br"""asd"""
            x = rf"asd"
            x = rf'asd'
            x = rf'''asd'''
            x = rf"""asd"""
            x = fr"asd"
            x = fr'asd'
            x = fr'''asd'''
            x = fr"""asd"""
            "###;
        assert_eq!(
            get_cleaned_source_code::<Python>(src).unwrap(),
            r###"        def foo( 
          bar,
          car = ,
        ):
            pass
        def test():
            x = 
            x = 
            x = 
            x = 
            x = 
            x = 
            x = 
            x = 
            x = 
            x = 
            x = 
            x = 
            x = f"asd"
            x = f'asd'
            x = f'''asd"""
            kk"""
            '''
            x = f"""asd"""
            x = 
            x = 
            x = 
            x = 
            x = 
            x = 
            x = 
            x = 
            x = 
            x = 
            x = 
            x = 
            x = rf"asd"
            x = rf'asd'
            x = rf'''asd'''
            x = rf"""asd"""
            x = fr"asd"
            x = fr'asd'
            x = fr'''asd'''
            x = fr"""asd"""
"###
        );
    }

    #[test]
    fn golden_file_test() {
        // ./src/fixtures/python_tests.py
        let src = std::fs::read_to_string("./src/fixtures/python_tests.py").unwrap();
        let golden_src = std::fs::read_to_string("./src/fixtures/python_tests_golden.py").unwrap();
        assert_eq!(get_cleaned_source_code::<Python>(&src).unwrap(), golden_src);
        // ./src/fixtures/more_python_tests.py
        let src = std::fs::read_to_string("./src/fixtures/more_python_tests.py").unwrap();
        let golden_src =
            std::fs::read_to_string("./src/fixtures/more_python_tests_golden.py").unwrap();
        // println!("{}", get_cleaned_source_code::<Python>(&src).unwrap());
        assert_eq!(get_cleaned_source_code::<Python>(&src).unwrap(), golden_src);
    }
}

#[cfg(test)]
mod simple_rust {
    use pretty_assertions::assert_eq;
    use ra_ap_syntax::SyntaxKind;

    use crate::count;
    use crate::langs::*;
    use crate::parse::v2::{get_lines_without_ranges, Parser, TreeSitterLanguage};

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
        let cnt_executable = {
            let mut parser = Parser::new(TreeSitterLanguage::Rust).unwrap();
            let ranges = parser.non_executable_src_spans(src).unwrap();
            let lines = get_lines_without_ranges(src, ranges);
            // crate::parse::v2::display_lines(src, &lines);
            lines.len()
        };
        assert_eq!(cnt, 6);
        assert_eq!(cnt, meaningful_lines_in_src_using_ast(src).len());
        assert_eq!(cnt_executable, 6);
    }

    #[test]
    fn possible_misparse_occurrences() {
        let src = r####"
        pub fn main() {
            let s = "\u{2603}";
            assert_eq!(s, "☃");
        
            let s = "\u{2a10}\u{2A01}\u{2Aa0}";
            assert_eq!(s, "⨐⨁⪠");
        
            let s = "\\{20}";
            let mut correct_s = String::from("\\");
            correct_s.push_str("{20}");
            assert_eq!(s, correct_s);
        }
        
        // run-pass
        pub fn main() {
            assert_eq!(r"abc", "abc");
        
            assert_eq!(r#"abc"#, "abc");
        
            assert_eq!(r"###", "###");
        
            assert_eq!(r"\", "\\");
        
            assert_eq!(r#"\""#, "\\\"");
        
            assert_eq!(r#"#"\n""#, "#\"\\n\"");
        
            assert_eq!(r##"a"#"b"##, "a\"#\"b");
        
            // from rust.vim
            assert_eq!(r#""%\(\d\+\$\)\=[-+' #0*]*\(\d*\|\*\|\*\d\+\$\)\(\.\(\d*\|\*\|\*\d\+\$\)\)\=\([hlLjzt]\|ll\|hh\)\=\([aAbdiuoxXDOUfFeEgGcCsSpn?]\|\[\^\=.[^]]*\]\)""#,
                       "\"%\\(\\d\\+\\$\\)\\=[-+' #0*]*\\(\\d*\\|\\*\\|\\*\\d\\+\\$\\)\\(\\.\\(\\d*\\|\\*\\|\\*\\d\\+\\$\\)\\)\\=\\([hlLjzt]\\|ll\\|hh\\)\\=\\([aAbdiuoxXDOUfFeEgGcCsSpn?]\\|\\[\\^\\=.[^]]*\\]\\)\"");
        
            assert_eq!(r"newline:'
        ', tab:'	', unicode:'●', null:''",
                "newline:'\n', tab:'\t', unicode:'\u{25cf}', null:'\0'");
        }
        "####;
        let cnt = count::get_count_of_meaningful_lines::<Rust>(src);
        assert_eq!(cnt, 22);
        assert_eq!(cnt, meaningful_lines_in_src_using_ast(src).len());
        let cnt_executable = {
            let mut parser = Parser::new(TreeSitterLanguage::Rust).unwrap();
            let ranges = parser.non_executable_src_spans(src).unwrap();
            let lines = get_lines_without_ranges(src, ranges);
            // crate::parse::v2::display_lines(src, &lines);
            lines.len()
        };
        assert_eq!(cnt_executable, 22);
        let cleaned_src = count::get_cleaned_source_code::<Rust>(src).unwrap();
        assert_eq!(
            cleaned_src,
            "        pub fn main() {
            let s = ;
            assert_eq!(s, );
            let s = ;
            assert_eq!(s, );
            let s = ;
            let mut correct_s = String::from();
            correct_s.push_str();
            assert_eq!(s, correct_s);
        pub fn main() {
            assert_eq!(, );
            assert_eq!(, );
            assert_eq!(, );
            assert_eq!(, );
            assert_eq!(, );
            assert_eq!(, );
            assert_eq!(, );
            assert_eq!(,
);
            assert_eq!(,
);
"
        );
    }

    /// utility function to verify the parse was accurate for rust
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
                    if let ra_ap_syntax::NodeOrToken::Token(token) = se {
                        if [
                            SyntaxKind::WHITESPACE,
                            SyntaxKind::COMMENT,
                            SyntaxKind::L_CURLY,
                            SyntaxKind::R_CURLY,
                            SyntaxKind::L_PAREN,
                            SyntaxKind::R_PAREN,
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
                Some(())
            });
            if ue.saturating_sub(ub) > 0 {
                v.push(idx + 1);
            }
        }
        v
    }
    #[test]
    fn run_on_self() {
        // run `dracula_v2` vs `meaningful_lines_in_src_using_ast`
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
            "./pydracula/src/lib.rs",
            "./src/fixtures/rust_tests.rs",
        ];
        src_files
            .into_iter()
            .map(|x| std::fs::read_to_string(dbg!(x)).ok())
            .flatten()
            .for_each(|src| {
                meaningful_lines_in_src_using_ast(&src)
                    .into_iter()
                    // .zip(meaningful_lines_in_src_using_ast(&src).into_iter())
                    .zip({
                        let mut parser = Parser::new(TreeSitterLanguage::Rust).unwrap();
                        let ranges = parser.non_executable_src_spans(&src).unwrap();
                        let lines = get_lines_without_ranges(&src, ranges);
                        // crate::parse::v2::display_lines(&src, &lines);
                        lines
                    })
                    .for_each(|(x, y)| assert_eq!(x, y));
            });
    }
}

#[cfg(test)]
mod simple_js {
    use crate::parse::v2::get_lines_without_ranges;
    use crate::parse::v2::Parser;
    use crate::parse::v2::TreeSitterLanguage;

    #[test]
    fn try_parse() {
        let src = r#"
        function test() { // :01
            // empty
            let x = 10; // :02
            let y = ` // :03
                /* this is part of the string? */ // :04 as this is template string
            ${ // :05
            // this is empty line
            }
            `; // :06
        }
        "#;
        let cnt_executable = {
            let mut parser = Parser::new(TreeSitterLanguage::Javascript).unwrap();
            let ranges = parser.non_executable_src_spans(src).unwrap();
            let lines = get_lines_without_ranges(src, ranges);
            // crate::parse::v2::display_lines(src, &lines);
            lines.len()
        };
        assert_eq!(cnt_executable, 6); // we can now ignore parens and curlies
    }
}

#[cfg(test)]
mod simple_jsx {
    use crate::parse::v2::get_lines_without_ranges;
    use crate::parse::v2::Parser;
    use crate::parse::v2::TreeSitterLanguage;

    #[test]
    fn try_parse() {
        let src = r#"
        function test() {
        return <Test>
        {      
            /*
            <!--
                this type of comment doesn't work in jsx
            -->
            */
        }
            {
                /* ** Ignore comment
                */
                x 
                + yy
            }
        </Test>;
        }
        "#;
        let cnt_executable = {
            let mut parser = Parser::new(TreeSitterLanguage::Javascript).unwrap();
            let ranges = parser.non_executable_src_spans(src).unwrap();
            let lines = get_lines_without_ranges(src, ranges);
            // crate::parse::v2::display_lines(src, &lines);
            lines.len()
        };
        assert_eq!(cnt_executable, 5); // we can now ignore parens and curlies
    }
}

#[cfg(test)]
mod simple_ts {
    use crate::parse::v2::get_lines_without_ranges;
    use crate::parse::v2::Parser;
    use crate::parse::v2::TreeSitterLanguage;

    #[test]
    fn try_parse() {
        let src = r#"
        function test(_: any) { // :01
            // empty
            let x: // :02
                Ty = 10; // :03
            let y = ` // :04
                /* this is part of the string? */ // :04 as this is template string
            ${ // :06
            // this is empty line
            }
            `; // :07
        }
        "#;
        let cnt_executable = {
            let mut parser = Parser::new(TreeSitterLanguage::Typescript).unwrap();
            let ranges = parser.non_executable_src_spans(src).unwrap();
            let lines = get_lines_without_ranges(src, ranges);
            // crate::parse::v2::display_lines(src, &lines);
            lines.len()
        };
        assert_eq!(cnt_executable, 7); // we can now ignore parens and curlies
    }
}
