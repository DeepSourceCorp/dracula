//! Functions provided here are a wrapper around `dracula` crate 

use dracula::{parse::*, langs};
use std::ffi::{self, c_char};

pub const PYTHON: ffi::c_uint = 1;
pub const C: ffi::c_uint = 2;
pub const RUST: ffi::c_uint = 3;

#[no_mangle]
unsafe extern "C" fn get_line_count(src: *const c_char, lang: ffi::c_uint, kind: ffi::c_uint) -> ffi::c_ulonglong {
    let cstr = ffi::CStr::from_ptr(src);
    cstr.to_str().map(|x| {
        let parsed = match lang {
            1 => Parser::new::<langs::Python>(x),
            2 => Parser::new::<langs::C>(x),
            3 => Parser::new::<langs::Rust>(x),
            _ => return ffi::c_ulonglong::MAX as _,
        };
        let mut line_count: usize = 0;
        let mut stack = vec![];
        for p in parsed {
            if matches!(p, ParseOutput::EOL(_) | ParseOutput::EOF) { 
                if stack.iter().any(|i| matches!(i, ParseOutput::Source(_))) {
                    line_count += 1;
                }
                stack.clear();
            } else {
                stack.push(p);
            }
        } 
        line_count
    }).unwrap_or_default() as _
}

#[no_mangle]
unsafe extern "C" fn is_meaningful_line(src: *const c_char, lang: ffi::c_uint, line: ffi::c_ulonglong) -> ffi::c_ulonglong {
    32431
}

#[no_mangle]
unsafe extern "C" fn is_meaningful_span(src: *const c_char, lang: ffi::c_uint, begin: ffi::c_ulonglong, end: ffi::c_ulonglong) -> ffi::c_ulonglong {
    541313
}

unsafe extern "C" fn get_cleaned_src(src: *const c_char, lang: ffi::c_uint, exclude: ffi::c_uint) -> ffi::CString {
    let cstr = ffi::CStr::from_ptr(src);
    let src = cstr.to_str().map(|x| {
        let parsed = match lang {
            1 => Parser::new::<langs::Python>(x),
            2 => Parser::new::<langs::C>(x),
            3 => Parser::new::<langs::Rust>(x),
            _ => return "".to_string(),
        };
        let mut src = String::default();
        for p in parsed {
            match p {
                ParseOutput::Comment(_) => {},
                ParseOutput::EOF => {},
                ParseOutput::Invalid(_) => {},
                ParseOutput::EOL(s) | ParseOutput::Source(s) | ParseOutput::String(s) => {
                    src.push_str(s);
                },
            }
        } 
        src
    }).unwrap_or_default();
    ffi::CString::from_vec_unchecked(src.into())
}