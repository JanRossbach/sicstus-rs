use crate::{error::PrologError, util::string_from_ref};

use super::{terms::sp_new_term_ref, sys::*};
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

/// Retracts the current pending exception term, if it exits.
///
/// # Returns
/// Result of either the successfully extracted exception term, or an error if no exception term exists.
/// See also: <https://sicstus.sics.se/sicstus/docs/latest4/pdf/sicstus.pdf#Exception%20Handling%20in%20C>
pub fn sp_exception_term() -> Result<SP_term_ref, PrologError> {
    let term = sp_new_term_ref();
    let return_value = unsafe { SP_exception_term(term) };
    if return_value == 1 {
        Ok(term)
    } else if return_value == 0 {
        Err(PrologError::NoExceptionTerm(term))
    } else {
        Err(PrologError::UnexpectedReturnCode(return_value))
    }
}

/// Fails in the scope of Prolog calling C.
///
/// # Arguments
/// * term - The [SP_term_ref] whose value will be the exception term.
///
/// # Description
/// This function is normally used in the context of a call from Prolog to C,
/// and will cause Prolog to backtrack on return from the call.
/// *Please note*: this should only be called right before returning to Prolog.
/// See also: <https://sicstus.sics.se/sicstus/docs/latest4/pdf/sicstus.pdf#Exception%20Handling%20in%20C>
pub fn sp_fail() {
    unsafe { SP_fail() }
}

/// Raise an exception that will detected when the Program returns to Prolog.
/// This should be called right before returning to Prolog.
/// See also: <https://sicstus.sics.se/sicstus/docs/latest4/pdf/sicstus.pdf#Exception%20Handling%20in%20C>
/// To propagate failure to Prolog, call [sp_fail] instead.
pub fn sp_raise_exception(term: SP_term_ref) {
    unsafe { SP_raise_exception(term) }
}
