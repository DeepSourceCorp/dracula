use std::ffi::{c_char, self, CString};

#[link(name = "cdracula")]
extern "C" {
    fn get_meaningful_line_count(src: *const c_char, lang: ffi::c_uint, kind: ffi::c_uint) -> ffi::c_ulonglong;
    fn get_cleaned_src(src: *const c_char, lang: ffi::c_uint, exclude: ffi::c_uint) -> *mut i8;
}

#[test]
fn test_get_meaningful_line_count() {
    // Will fail if `cargo b -p cdracula` hasn't been run before running this project's tests
    unsafe {
        let src = CString::from_vec_unchecked((String::from(r#"
        # skip this
        def python():
            """
                Multi line comments also should be zero?
            """
            pass # only two meaningful lines
        "#) + "\0").into());
        println!("{}", get_meaningful_line_count(src.as_ptr(), 1, 0));
    }
}

#[test]
fn test_get_cleaned_src() {
    unsafe {
        let src = CString::from_vec_unchecked((String::from(r#"
        # skip this
        def python():
            """
                Multi line comments also should be zero?
            """
            pass # only two meaningful lines
        def python(
            foo, bar
        ):
            pass
            "#) + "\0").into());
        let v = CString::from_raw(get_cleaned_src(src.as_ptr(), 1, 0));
        println!("{}", v.to_str().unwrap());
    }
}