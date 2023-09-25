use crate::error::PrologError;
use std::ffi::{c_char, c_int};

/// Re export of the sicstus_sys crate low level bindings for SICStus Prolog.
/// This is purely for convenience and in order to opt out of the high level API if
/// you need fine grained control over the C API.
pub mod sys {
    #[cfg(test)]
    pub use sicstus_sys::mock_ffi::*;

    #[cfg(not(test))]
    pub use sicstus_sys::ffi::*;

    pub use sicstus_sys::variadic::*;
    pub use sicstus_sys::*;
}

use ffi::*;
pub use sys as ffi;

/// Create a new [String] from a *const pointer to a C string.
/// This does not take ownership of the pointer. The caller is responsible for freeing the memory.
/// # Safety
/// The pointer must be valid and point to a null terminated C string.
pub unsafe fn string_from_ref(sp: *const c_char) -> String {
    let mut result = String::new();
    let mut cp: *const c_char = sp;
    loop {
        let c: c_char = *cp;
        let c = c as u8 as char;
        if c == '\0' {
            break;
        }
        result.push(c as u8 as char);
        cp = cp.add(1);
    }
    result
}

/// Save wrapper around the unsafe [SP_get_string] function from Prolog.
pub fn sp_get_string(term_ref: SP_term_ref) -> Result<String, PrologError> {
    unsafe {
        let mut s: *const c_char = std::ptr::null_mut();
        let ret_val: c_int = SP_get_string(term_ref, &mut s as *mut *const c_char);
        if ret_val == 0 || s.is_null() {
            Err(PrologError::TermConversionError)
        } else {
            Ok(string_from_ref(s))
        }
    }
}

/// Save wrapper around the unsafe [SP_get_integer] function from Prolog.
/// # Arguments
/// * `term_ref` - The term reference to convert.
/// # Returns a Result
/// * `Ok(i)` - The integer value of the term reference wrapped in an [Ok] variant.
/// * `Err(PrologError::TermConversionError)` - If the term reference could not be converted.
pub fn sp_get_integer(term_ref: SP_term_ref) -> Result<i64, PrologError> {
    unsafe {
        let mut i: SP_integer = 0;
        let p: *mut SP_integer = &mut i;
        let ret_val: c_int = SP_get_integer(term_ref, p);
        if ret_val == 0 {
            Err(PrologError::TermConversionError)
        } else {
            Ok(i)
        }
    }
}
