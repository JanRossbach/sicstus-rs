use super::sys::*;
use crate::error::PrologError;
use crate::util::string_from_ref;
use super::errors::sp_err_message;

use std::os::raw::{c_char, c_int};

/// Discard the current solution to the given query, and close it.
///
/// # Arguments
/// * query - The query, created by [SP_open_query].
///
/// # Returns
///
/// Result
///
/// # Description
/// This will discard the choices created since the corresponding [SP_open_query], and then
/// backtrack into the query, throwing away any current solution, like the goal !, fail. The
/// given argument does not have to be the innermost open query; any open queries in its scope will also be closed.
/// See also: <https://sicstus.sics.se/sicstus/docs/latest4/pdf/sicstus.pdf#Finding%20Multiple%20Solutions%20of%20a%20Call>
pub fn sp_close_query(query: SP_qid) -> Result<(), PrologError> {
    let res = unsafe { SP_close_query(query) };
    if res == SP_ERROR {
        // We got a SP_ERROR, so the safety condition is met and we can call sp_error_message.
        unsafe { Err(PrologError::CloseQueryError(sp_err_message())) }
    } else if res == SP_SUCCESS as c_int {
        Ok(())
    } else {
        Err(PrologError::UnexpectedReturnCode)
    }
}

pub fn sp_cons_functor(_term: SP_term_ref, _name: SP_atom, _args: Vec<SP_term_ref>) {
    unimplemented!();
}

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
