#[cfg(test)]
mod python {
    use std::ffi::{self, c_char, CString};

    #[link(name = "cdracula")]
    extern "C" {
        static PYTHON_LANG: ffi::c_uint;
        fn get_meaningful_line_count(
            src: *const c_char,
            lang: ffi::c_uint,
            kind: ffi::c_uint,
        ) -> ffi::c_ulonglong;
        fn get_cleaned_src(src: *const c_char, lang: ffi::c_uint, exclude: ffi::c_uint) -> *mut i8;
        fn meaningful_lines(
            src: *const c_char,
            lang: ffi::c_uint,
            r_lines_len: *mut ffi::c_ulonglong,
        ) -> *mut ffi::c_ulonglong;
    }

    #[test]
    fn test_get_meaningful_line_count() {
        // Will fail if `cargo b -p cdracula` hasn't been run before running this project's tests
        unsafe {
            let src = CString::from_vec_unchecked(
                (String::from(
                    r#"
            # skip this
            def python():
                """
                    Multi line comments also should be zero?
                """
                pass # only two meaningful lines
            "#,
                ) + "\0")
                    .into(),
            );
            assert_eq!(get_meaningful_line_count(src.as_ptr(), PYTHON_LANG, 0), 2);
        }
    }

    #[test]
    fn test_get_meaningful_lines() {
        // Will fail if `cargo b -p cdracula` hasn't been run before running this project's tests
        unsafe {
            let src = CString::from_vec_unchecked(
                (String::from(
                    r#"
            # skip this
            def python():
                """
                    Multi line comments also should be zero?
                """
                pass # only two meaningful lines
            "#,
                ) + "\0")
                    .into(),
            );
            let mut len = 0u64;
            let ptr = meaningful_lines(src.as_ptr(), PYTHON_LANG, &mut len as *mut u64);
            let v = Vec::from_raw_parts(ptr, len as _, len as _);

            assert_eq!(&v, &[2, 6]);
        }
    }

    #[test]
    fn test_get_cleaned_src() {
        unsafe {
            let src = CString::from_vec_unchecked(
                (String::from(
                    r#"
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
                "#,
                ) + "\0")
                    .into(),
            );
            let v = CString::from_raw(get_cleaned_src(src.as_ptr(), PYTHON_LANG, 0));
            assert_eq!(
                v.to_str(),
                Ok("            def python():\n                pass \n            def python(\n                foo, bar\n            ):\n                pass\n")
            );
        }
    }
}

#[cfg(test)]
mod c_and_cpp {
    use std::ffi::{self, c_char, CString};

    #[link(name = "cdracula")]
    extern "C" {
        static C_LANG: ffi::c_uint;
        fn get_meaningful_line_count(
            src: *const c_char,
            lang: ffi::c_uint,
            kind: ffi::c_uint,
        ) -> ffi::c_ulonglong;
        fn get_cleaned_src(src: *const c_char, lang: ffi::c_uint, exclude: ffi::c_uint) -> *mut i8;
        fn meaningful_lines(
            src: *const c_char,
            lang: ffi::c_uint,
            r_lines_len: *mut ffi::c_ulonglong,
        ) -> *mut ffi::c_ulonglong;
    }

    #[test]
    fn test_get_meaningful_line_count() {
        unsafe {
            let src = CString::from_vec_unchecked(
                (String::from(
                    r#"
                    // interesting line
                    int main() {
                        // maybe not useful
                        return 0;
                        /*
                            this is useful
                        */ int x = 10;
                    }
                    "#,
                ) + "\0")
                    .into(),
            );
            assert_eq!(get_meaningful_line_count(src.as_ptr(), C_LANG, 0), 4);
        }
    }

    #[test]
    fn test_get_meaningful_lines() {
        unsafe {
            let src = CString::from_vec_unchecked(
                (String::from(
                    r#"
                    // interesting line
                    int main() {
                        // maybe not useful
                        return 0;
                        /*
                            this is useful
                        */ int x = 10;
                    }
                    "#,
                ) + "\0")
                    .into(),
            );
            let mut len = 0u64;
            let ptr = meaningful_lines(src.as_ptr(), C_LANG, &mut len as *mut u64);
            let v = Vec::from_raw_parts(ptr, len as _, len as _);
            assert_eq!(&v, &[2, 4, 7, 8]);
        }
    }

    #[test]
    fn test_get_cleaned_src() {
        unsafe {
            let src = CString::from_vec_unchecked(
                (String::from(
                    r#"
                    // interesting line
                    int main() {
                        // maybe not useful
                        return 0;
                        /*
                            this is useful
                        */ int x = 10;
                    }
                    "#,
                ) + "\0")
                    .into(),
            );
            let v = CString::from_raw(get_cleaned_src(src.as_ptr(), C_LANG, 0));
            assert_eq!(
                v.to_str(),
                Ok("                    int main() {\n                        return 0;\n int x = 10;\n                    }\n")
            );
        }
    }
}
