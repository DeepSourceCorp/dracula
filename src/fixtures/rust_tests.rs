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
', tab:'	', unicode:'●', null:' '",
        "newline:'\n', tab:'\t', unicode:'\u{25cf}', null:'\0'");
}