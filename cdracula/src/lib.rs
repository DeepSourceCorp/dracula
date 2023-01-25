//! Functions provided here are a wrapper around `dracula` crate
#![deny(improper_ctypes_definitions)]

use dracula::{langs, parse::*};
use std::ffi::{self, c_char};

pub const PYTHON: ffi::c_uint = 1;
pub const C: ffi::c_uint = 2;
pub const RUST: ffi::c_uint = 3;

#[no_mangle]
pub unsafe fn get_meaningful_line_count(
    src: *const c_char,
    lang: ffi::c_uint,
    kind: ffi::c_uint,
) -> ffi::c_ulonglong {
    let cstr = ffi::CStr::from_ptr(src);
    cstr.to_str()
        .map(|x| {
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
                    if stack.iter().any(|i| match i {
                        ParseOutput::Source(s) => langs::Python::is_meaningful_src(s),
                        _ => false,
                    }) {
                        line_count += 1;
                    }
                    stack.clear();
                } else {
                    stack.push(p);
                }
            }
            line_count
        })
        .unwrap_or_default() as _
}

#[no_mangle]
pub unsafe fn meaningful_lines(
    src: *const c_char,
    lang: ffi::c_uint,
    r_lines_len: *mut ffi::c_ulonglong,
) -> *mut ffi::c_ulonglong {
    let mut vec = Vec::<ffi::c_ulonglong>::new();
    let cstr = ffi::CStr::from_ptr(src);
    _ = cstr.to_str().map(|x| {
        let parsed = match lang {
            1 => Parser::new::<langs::Python>(x),
            2 => Parser::new::<langs::C>(x),
            3 => Parser::new::<langs::Rust>(x),
            _ => return (),
        };
        let mut lines = x.lines().enumerate();
        for (usize, line) in lines {
            
        }
    });
    vec.shrink_to_fit();
    assert!(vec.len() == vec.capacity());
    let ptr = vec.as_mut_ptr();
    let len = vec.len();
    *r_lines_len = len as _;
    std::mem::forget(vec); // prevent deallocation in Rust
    ptr
}


#[no_mangle]
pub unsafe fn get_cleaned_src(
    src: *const c_char,
    lang: ffi::c_uint,
    exclude: ffi::c_uint,
) -> *mut i8 {
    let cstr = ffi::CStr::from_ptr(src);
    let src = cstr
        .to_str()
        .map(|x| {
            let parsed = match lang {
                1 => Parser::new::<langs::Python>(x),
                2 => Parser::new::<langs::C>(x),
                3 => Parser::new::<langs::Rust>(x),
                _ => return "".to_string(),
            };
            let mut meaningful_src = String::default();
            let mut stack = vec![];
            for p in parsed {
                if matches!(p, ParseOutput::EOL(_) | ParseOutput::EOF) {
                    let meaningful_src_len = meaningful_src.len();
                    for po in stack.iter() {
                        if let ParseOutput::Source(s) = po {
                            if langs::Python::is_meaningful_src(s) {
                                meaningful_src.push_str(s);
                            }
                        }
                    }
                    if meaningful_src_len != meaningful_src.len() {
                        meaningful_src.push('\n');
                    }
                    stack.clear();
                } else {
                    stack.push(p);
                }
            }
            meaningful_src
        })
        .unwrap_or_default();
    ffi::CString::from_vec_unchecked(src.into()).into_raw()
}
