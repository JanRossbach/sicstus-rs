use crate::util::string_from_ref;

use super::sys::*;
use std::os::raw::{c_char, c_int};

/// Get the error message from Prolog when a SP_ERROR is returned by a function.
///
/// # Returns
/// The error message as a String.
/// # Safety
/// This should only be called when a SP_ERROR is actually returned. Otherwise it will confuse Prolog and cause UB.
pub unsafe fn sp_err_message() -> String {
    let errno: c_int = SP_get_errno();
    let message: *const c_char = unsafe { SP_error_message(errno) };
    string_from_ref(message)
}

/// Retracts the current pending exception term, if it exits, and assigns it to term.
///
/// # Arguments
/// * term - The [SP_term_ref] to assign.
///
/// # Returns
/// 1 if an exception term was retracted and assigned, and 0 otherwise.
/// See also: <https://sicstus.sics.se/sicstus/docs/latest4/pdf/sicstus.pdf#Exception%20Handling%20in%20C>
pub fn sp_exception_term(term: SP_term_ref) -> c_int {
    unsafe { SP_exception_term(term) }
}

/// Fails in the scope of Prolog calling C.
///
/// # Arguments
/// * term - The [SP_term_ref] to assign.
///
/// # Description
/// This function is normally used in the context of a call from Prolog to C,
/// and will cause Prolog to backtrack on return from the call.
/// *Please note*: this should only be called right before returning to Prolog.
/// See also: <https://sicstus.sics.se/sicstus/docs/latest4/pdf/sicstus.pdf#Exception%20Handling%20in%20C>
pub fn sp_fail(term: SP_term_ref) {
    unsafe { SP_fail(term) }
}

pub fn sp_raise_exception(term: SP_term_ref) {
    unsafe { SP_raise_exception(term) }
}
