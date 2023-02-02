//! Functions provided here are a wrapper around `dracula` crate
#![deny(improper_ctypes_definitions)]

use dracula::{langs, parse::*};
use std::ffi::{self, c_char};

const PYTHON: ffi::c_uint = 1;
const C: ffi::c_uint = 2;
const RUST: ffi::c_uint = 3;

#[no_mangle]
pub static PYTHON_LANG: ffi::c_uint = PYTHON;
#[no_mangle]
pub static C_LANG: ffi::c_uint = C;
#[no_mangle]
pub static RUST_LANG: ffi::c_uint = RUST;

#[no_mangle]
/// get the count of meaningful lines in the source currently doesn't support
/// setting the kind for the definition of a meaningful line, but used as field
/// to avoid ABI incompatibility later
pub unsafe fn get_meaningful_line_count(
    src: *const c_char,
    lang: ffi::c_uint,
    kind: ffi::c_uint,
) -> ffi::c_ulonglong {
    let parser = match lang {
        PYTHON => langs::Python::get_parser(),
        C => langs::C::get_parser(),
        RUST => langs::Rust::get_parser(),
        _ => return -1 as _,
    };
    let is_meaningful = match lang {
        PYTHON => langs::Python::is_meaningful(),
        C => langs::C::is_meaningful(),
        RUST => langs::Rust::is_meaningful(),
        _ => return -1 as _,
    };
    let cstr = ffi::CStr::from_ptr(src);
    cstr.to_str()
        .map(|src| {
            let parsed = parser(src);
            let mut line_count: usize = 0;
            let mut stack = vec![];
            for p in parsed {
                if matches!(p, ParseOutput::EOL(_) | ParseOutput::EOF) {
                    if stack.iter().any(is_meaningful) {
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
/// get the list of meaningful lines in the source currently doesn't support
/// setting the `kind` for the definition of a meaningful line, but used as field
/// to avoid ABI incompatibility later
pub unsafe fn meaningful_lines(
    src: *const c_char,
    lang: ffi::c_uint,
    r_lines_len: *mut ffi::c_ulonglong,
) -> *mut ffi::c_ulonglong {
    let parser = match lang {
        PYTHON => langs::Python::get_parser(),
        C => langs::C::get_parser(),
        RUST => langs::Rust::get_parser(),
        _ => return -1 as _,
    };
    let is_meaningful = match lang {
        PYTHON => langs::Python::is_meaningful(),
        C => langs::C::is_meaningful(),
        RUST => langs::Rust::is_meaningful(),
        _ => return -1 as _,
    };

    let mut meaningful_lines = Vec::<ffi::c_ulonglong>::new();
    let cstr = ffi::CStr::from_ptr(src);
    _ = cstr.to_str().map(|src| {
        let mut parsed = parser(src);
        let lines = src.split_inclusive('\n').enumerate();
        struct Span {
            start: usize,
            end: usize,
        }
        let mut line_span = Span { start: 0, end: 0 };
        let mut parse_span = Span { start: 0, end: 0 };
        let mut last_parsed_output = None;
        for (idx, line) in lines {
            // setup line start and end
            line_span.start = line_span.end;
            line_span.end += line.len();
            // traverse parsed output until the span end is reached
            let mut po_stack = if let Some(po) = last_parsed_output {
                vec![po]
            } else {
                vec![]
            };
            while parse_span.end < line_span.end {
                if let Some(parsed_output) = parsed.next() {
                    // setup parsed start and end
                    parse_span.start = parse_span.end;
                    parse_span.end += parsed_output.len();
                    po_stack.push(parsed_output);
                }
            }
            if po_stack.iter().any(is_meaningful) {
                meaningful_lines.push(idx as u64);
            }
            if cfg!(dbg) {
                eprintln!("{} == {}", idx, po_stack.iter().any(is_meaningful));
                eprintln!("{:?}", po_stack);
                eprintln!("-------------------------------------------------");
            }
            if parse_span.end != line_span.end {
                last_parsed_output = po_stack.pop();
            } else {
                last_parsed_output = None;
            }
        }
    });
    meaningful_lines.shrink_to_fit();
    assert!(meaningful_lines.len() == meaningful_lines.capacity());
    let ptr = meaningful_lines.as_mut_ptr();
    let len = meaningful_lines.len();
    *r_lines_len = len as _;
    std::mem::forget(meaningful_lines); // prevent deallocation in Rust
    ptr
}

#[no_mangle]
/// get the source with just the meaningful lines in the source currently doesn't support
/// setting the `exclude` for the definition of a meaningful line, but used as field
/// to avoid ABI incompatibility later
pub unsafe fn get_cleaned_src(
    src: *const c_char,
    lang: ffi::c_uint,
    exclude: ffi::c_uint,
) -> *mut i8 {
    let parser = match lang {
        PYTHON => langs::Python::get_parser(),
        C => langs::C::get_parser(),
        RUST => langs::Rust::get_parser(),
        _ => return -1 as _,
    };
    let is_meaningful_src = match lang {
        PYTHON => langs::Python::is_meaningful_src,
        C => langs::C::is_meaningful_src,
        RUST => langs::Rust::is_meaningful_src,
        _ => return -1 as _,
    };
    let cstr = ffi::CStr::from_ptr(src);
    let src = cstr
        .to_str()
        .map(|src| {
            let parsed = parser(src);
            let mut meaningful_src = String::default();
            let mut stack = vec![];
            for p in parsed {
                if matches!(p, ParseOutput::EOL(_) | ParseOutput::EOF) {
                    let meaningful_src_len = meaningful_src.len();
                    for po in stack.iter() {
                        if let ParseOutput::Source(s) = po {
                            if is_meaningful_src(s) {
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
