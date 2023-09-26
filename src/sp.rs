use crate::error::PrologError;
use crate::util::string_from_ref;
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

/// Save wrapper around the unsafe [SP_get_string] function from Prolog.
/// # Arguments
/// * `term_ref` - The term reference to convert.
/// # Returns a Result of
/// * `Ok(s)` - The String value of the term reference wrapped in an [Ok] variant.
/// * `Err(PrologError::TermConversionError)` - If the term reference could not be converted.
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

#[cfg(test)]
#[test]
fn test_sp_get_string() {
    let _mutex_guard = crate::test_utils::get_lock();
    use mockall::predicate::{always, eq};
    let t: SP_term_ref = SP_term_ref::default();
    let sp_get_string_ctx = SP_get_string_context();
    sp_get_string_ctx
        .expect()
        .with(eq(t), always())
        .returning(|_, pointer| {
            unsafe {
                *pointer = crate::test_utils::TEST_ATOM_NAME_STR.as_ptr() as *const c_char;
            }
            1
        });
    let res = sp_get_string(t).unwrap();
    assert_eq!(res, "test".to_string());
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
