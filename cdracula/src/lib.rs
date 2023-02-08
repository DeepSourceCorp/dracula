//! Functions provided here are a wrapper around `dracula` crate
#![deny(improper_ctypes_definitions)]
#![allow(non_upper_case_globals)]

mod util_macros;
use std::ffi::{self, c_char};

languages_supported! {
    const Python = 1;
    const C = 2;
    const Rust = 3;
    const Java = 4;
}

#[no_mangle]
pub static PYTHON_LANG: ffi::c_uint = Python;
#[no_mangle]
pub static C_LANG: ffi::c_uint = C;
#[no_mangle]
pub static RUST_LANG: ffi::c_uint = Rust;
#[no_mangle]
pub static JAVA_LANG: ffi::c_uint = Java;

#[no_mangle]
/// This function is used to get the count of meaningful lines in the source.
///
/// It currently doesn't support setting the multiple ways(`kind`) of meaningful line
/// search eg. `ignore whitespace`, `specific character`, etc.
/// aka the definition of a meaningful line.
/// But provided as field to avoid ABI incompatibility later.
pub unsafe fn get_meaningful_line_count(
    src: *const c_char,
    lang: ffi::c_uint,
    _kind: ffi::c_uint,
) -> ffi::c_ulonglong {
    let cstr = ffi::CStr::from_ptr(src);
    cstr.to_str()
        .ok()
        .map(|src| get_count_of_meaningful_lines_as_u64(lang, src))
        .flatten()
        .unwrap_or_default() as _
}

#[no_mangle]
/// This function is used to get the list of meaningful lines in the source.
///
/// It currently doesn't support setting the multiple ways(`kind`) of meaningful line
/// search eg. `ignore whitespace`, `specific character`, etc.
/// aka the definition of a meaningful line.
/// But provided as field to avoid ABI incompatibility later.
///
/// NOTE:
/// The caller is responsible for free'ing the obtained array
pub unsafe fn meaningful_lines(
    src: *const c_char,
    lang: ffi::c_uint,
    _kind: ffi::c_uint,
    r_lines_len: *mut ffi::c_ulonglong,
) -> *mut ffi::c_ulonglong {
    if cfg!(dbg) {
        assert!(!src.is_null());
    }
    let cstr = ffi::CStr::from_ptr(src);
    let mut meaningful_lines = cstr
        .to_str()
        .ok()
        .and_then(|src| get_meaningful_line_indices_as_u64(lang, src))
        .unwrap_or_else(|| vec![]);
    meaningful_lines.shrink_to_fit();
    if cfg!(dbg) {
        assert!(meaningful_lines.len() == meaningful_lines.capacity());
    }
    let ptr = meaningful_lines.as_mut_ptr();
    let len = meaningful_lines.len();
    *r_lines_len = len as _;
    std::mem::forget(meaningful_lines); // prevent deallocation in Rust
    ptr
}

#[no_mangle]
/// This function is used to get the source of just the meaningful parts in the source,
/// including the whitespaces.
///
/// It currently doesn't support setting the multiple ways(`kind`) of meaningful line
/// search eg. `ignore whitespace`, `specific character`, etc.
/// aka the definition of a meaningful line.
/// But provided as field to avoid ABI incompatibility later.
///
/// NOTE:
/// The caller is responsible for free'ing the obtained array
pub unsafe fn get_cleaned_src(
    src: *const c_char,
    lang: ffi::c_uint,
    _kind: ffi::c_uint,
    _exclude: ffi::c_uint,
) -> *mut i8 {
    let cstr = ffi::CStr::from_ptr(src);
    let src = cstr
        .to_str()
        .ok()
        .map(|src| get_cleaned_source_code(lang, src))
        .flatten()
        .unwrap_or_default();
    ffi::CString::from_vec_unchecked(src.into()).into_raw()
}
