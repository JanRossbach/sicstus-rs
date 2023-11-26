//! A close wrapper module around the sicstus_sys crate bindings to the SICStus Prolog C API.
//! The functions here wrap the unsafe C API in a safe and more idiomatic rust API.
//! The functions are named like the C API functions, but with the SP_ prefix removed.
//! In general the changes have mostly stuck to the following rules:
//! * Using Results instead of return codes.
//! * Using return values instead of out parameters.
//! * Using rust types instead of C types.
//! The functions still use the concepts of SP_term_ref and SP_pred_ref to represent Prolog terms and predicates.
//! For more rust idiomatic wrapper types look in the rest of the API.
//! If you need more fine grained control over the C API, you can still call it directly from the sicstus_sys crate.
//! This module contains plenty of examples of how to do that.

use alloc::ffi::CString;
pub use sicstus_sys::{
    spio_t_bits, spio_t_error_code, spio_t_offset, spio_t_simple_device_close,
    spio_t_simple_device_flush_output, spio_t_simple_device_interrupt, spio_t_simple_device_ioctl,
    spio_t_simple_device_read, spio_t_simple_device_seek, spio_t_simple_device_write, spio_t_uint8,
    spio_t_wchar, SP_CPredFun, SP_EventFun, SP_SigFun, SP_UserStreamHook, SP_UserStreamPostHook,
    SP_atom, SP_get_dispatch_type, SP_integer, SP_mutex, SP_options, SP_pred_ref, SP_qid,
    SP_stream, SP_term_ref, SICSTUS_API_STRUCT, SP_ERROR, SP_FAILURE, SP_SUCCESS, SP_TYPE_ATOM,
    SP_TYPE_COMPOUND, SP_TYPE_ERROR, SP_TYPE_FLOAT, SP_TYPE_INTEGER, SP_TYPE_VARIABLE,
};

pub use sicstus_sys::SP_printf;

mod error {
    use core::error::Error;

    use alloc::string::String;

    use crate::sys::SP_term_ref;

    #[derive(Debug)]
    pub enum PrologError {
        TermConversionError(String),
        NoTermVariantMatch,
        AtomNotFound(String),
        CloseQueryError(String),
        UnexpectedReturnCode(i32),
        NoExceptionTerm(SP_term_ref),
        UncussefulUnificationError(i32, i32),
        ConsFunctorError,
        QueryOpenUnsuccessful,
        PredicateNotFound,
        NextSolutionError(String),
        NoMoreSolutions,
        CutQueryError(String),
        CutQueryFailure,
        DefineCPredicateError,
        TypeCheckError,
        AtomRegistrationError(u64),
        AtomUnregistrationError(u64),
    }

    // region:    --- Error Boilerplate
    impl core::fmt::Display for PrologError {
        fn fmt(
            &self,
            fmt: &mut core::fmt::Formatter,
        ) -> core::result::Result<(), core::fmt::Error> {
            write!(fmt, "{self:?}")
        }
    }
    impl Error for PrologError {}
}

use alloc::format;
pub use error::PrologError;
use sicstus_sys::*;

use alloc::string::{String, ToString};
use alloc::vec::Vec;

use crate::error::SicstusRsError;
use crate::util::string_from_ref;

use core::cmp::Ordering;
use core::ffi::{c_char, c_int, CStr};
use core::ffi::{c_uchar, c_void};

use super::*;

/// TODO ALERT! Not working yet at all!
pub fn sp_initialze(argc: usize, argv: Vec<String>) -> Result<(), SicstusRsError> {
    unsafe {
        let ret_val = SP_initialize(
            argc as c_int,
            argv.as_ptr() as *mut *mut c_char,
            core::ptr::null_mut(),
        );
        if ret_val == SP_FAILURE as i32 {
            Err(SicstusRsError::InitializationError(sp_err_message()))
        } else {
            Ok(())
        }
    }
}

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

/// Retrieve the Prolog Atom ID corresponding to the given atom name.
///
/// # Arguments
/// * `atom_name` - A rust &str containing the atom name.
///
/// # Returns
/// The Prolog ID corresponding to the given atom name if it exists, and an error otherwise.
/// # Description
/// Prolog has a unique integer ID for each atom. This represenation is needed for some of the C API functions.
/// See also: <https://sicstus.sics.se/sicstus/docs/latest4/pdf/sicstus.pdf#Creating%20and%20Manipulating%20SP_term_refs>
pub fn sp_atom_from_string(atom_name: &str) -> Result<SP_atom, PrologError> {
    let atom_cstring: CString = CString::new(atom_name).unwrap();
    let atom_id: SP_atom = unsafe { SP_atom_from_string(atom_cstring.as_ptr() as *const c_char) };
    if atom_id == 0 {
        Err(PrologError::AtomNotFound(atom_name.to_string()))
    } else {
        Ok(atom_id)
    }
}

/// Obtains the length of the encoded string representing a Prolog atom.
///
/// # Arguments
///
/// * atom - The atom to inspect.
///
/// # Returns
///
/// The length if the atom is valid, and 0 otherwise.
///
/// # Description
///
/// Same as strlen(SP_string_from_atom(a)), but runs in O(1) time.
///
/// See also: <https://sicstus.sics.se/sicstus/docs/latest4/pdf/sicstus.pdf#Creating%20and%20Manipulating%20SP_term_refs>
pub fn sp_atom_length(atom: SP_atom) -> usize {
    unsafe { SP_atom_length(atom) }
}

/// Discard the current solution to the given query, and close it.
///
/// # Arguments
/// * query - The query, created by [SP_open_query].
///
/// # Returns
///
/// Result of union if the closing was successful, and Err otherwise.
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
        Err(PrologError::UnexpectedReturnCode(res))
    }
}

/// Creates a compound term with the arguments filled in.
///
/// # Description
/// Assigns to term a reference to a compound term whose arguments are the values of arg.
/// If arity is 0, assigns the Prolog atom whose canonical representation is name. This is
/// similar to calling =../2 with the first argument unbound and the second argument bound.
///
/// # Arguments
/// * name - The name of the functor.
/// * arity - The arity of the functor.
/// * arg - The argument array.
///
/// # Returns
/// Ok(SP_term_ref) of the assigned new term if the conversion was successful, and Err otherwise.
///
/// See also: <https://sicstus.sics.se/sicstus/docs/latest4/pdf/sicstus.pdf#Creating%20Prolog%20Terms>
pub fn sp_cons_functor(name: SP_atom, args: &[SP_term_ref]) -> Result<SP_term_ref, PrologError> {
    let term = sp_new_term_ref();
    let arg_p = args.as_ptr() as *mut SP_term_ref;
    let arity = args.len() as c_int;
    // We call the array version of the C API because rust does not support variadic functions.
    let ret_value = unsafe { SP_cons_functor_array(term, name, arity, arg_p) };
    if ret_value == 0 {
        Err(PrologError::ConsFunctorError)
    } else {
        Ok(term)
    }
}

/// Not implemented, use [sp_cons_functor] instead.
pub fn sp_cons_functor_array() -> c_int {
    unimplemented!(
        "Use sp_cons_functor instead, since it already uses the array version of the C API."
    )
}

/// Assigns to term a reference to a Prolog list whose head and tail are the values of head and tail.
///
/// # Arguments
/// * head - The head of the new list.
/// * tail - The tail of the new list.
///
/// # Returns
/// Ok(term_ref) of the assigned new term if the conversion was successful, and Err otherwise.
/// See also: <https://sicstus.sics.se/sicstus/docs/latest4/pdf/sicstus.pdf#Creating%20Prolog%20Terms>
pub fn sp_cons_list(term: SP_term_ref, head: SP_term_ref, tail: SP_term_ref) -> Result<(), PrologError> {
    let ret_val = unsafe { SP_cons_list(term, head, tail) };
    if ret_val == 0 {
        Err(PrologError::TermConversionError(format!(
            "Could not convert head {:?} and tail {:?} to a list.",
            head, tail
        )))
    } else {
        Ok(())
    }
}

/// Commit to the current solution to the given query, and close it.
///
/// # Arguments
/// * query - The query, created by [SP_open_query].
///
/// # Return Value
/// Ok(()) for success, and appropriate error otherwise.
/// See also: <https://sicstus.sics.se/sicstus/docs/latest4/pdf/sicstus.pdf#Finding%20Multiple%20Solutions%20of%20a%20Call>
///
/// # Description
/// This will discard the choices created since the corresponding SP_open_query(), like the goal
/// !. The current solution is retained in the arguments until backtracking into any enclosing
/// query. The given argument does not have to be the innermost open query; any open queries
/// in its scope will also be cut.
pub fn sp_cut_query(query: SP_qid) -> Result<(), PrologError> {
    let re_val = unsafe { SP_cut_query(query) };
    if re_val == SP_ERROR {
        // We got a SP_ERROR, so the safety condition is met and we can call sp_error_message.
        unsafe { Err(PrologError::CutQueryError(sp_err_message())) }
    } else if re_val == SP_FAILURE as c_int {
        Err(PrologError::CutQueryFailure)
    } else if re_val == SP_SUCCESS as c_int {
        Ok(())
    } else {
        Err(PrologError::UnexpectedReturnCode(re_val))
    }
}

/// Defines a Prolog predicate that calls a C function.
///
/// # Arguments
/// * name - The predicate name.
/// * arity - The predicate arity.
/// * module - The predicate module name.
/// * proc - The C function to call.
/// * stash - See below.
///
/// # Return Value
/// Nonzero on success, and 0 otherwise.
///
/// # Description
///  The Prolog predicate module:name/arity will be defined (the module module must already exist).
///  The stash argument can be anything and is simply passed as the second argument to the C function proc.
///  The C function should return SP_SUCCESS for success and SP_FAILURE for failure. The C
///  function may also call SP_fail() or SP_raise_exception() in which case the return value will be ignored.
pub fn sp_define_c_predicate(
    name: *const c_char,
    arity: c_int,
    module: *const c_char,
    proc: SP_CPredFun,
    stash: *mut c_void,
) -> Result<(), PrologError> {
    let ret_val = unsafe { SP_define_c_predicate(name, arity, module, proc, stash) };
    if ret_val == 0 {
        Err(PrologError::DefineCPredicateError)
    } else {
        Ok(())
    }
}

/// Returns a pointer to the
pub fn sp_get_address(term: SP_term_ref) -> Result<*mut c_void, PrologError> {
    let p = core::ptr::null_mut();
    let ret_val = unsafe { SP_get_address(term, p) };
    if ret_val == 0 {
        Err(PrologError::TermConversionError(format!(
            "Could not convert term {:?} to a pointer.",
            term
        )))
    } else {
        unsafe {
            return Ok(*p);
        }
    }
}

/// Returns a SP_term_ref to the i'th argument of a compound *term*.
pub fn sp_get_arg(i: usize, term: SP_term_ref) -> Result<SP_term_ref, PrologError> {
    let arg = sp_new_term_ref();
    let ret_val = unsafe { SP_get_arg(i as c_int, term, arg) };
    if ret_val == 0 {
        Err(PrologError::TermConversionError(format!(
            "Failed getting the {}th arg in term {:?}.",
            i, term
        )))
    } else {
        Ok(arg)
    }
}

/// Get the canoncial represenation of an atom.
///
/// # Arguments
/// * term - The term representing the atom.
///
/// # Returns
/// Ok(SP_atom) of the canonical representation of the atom if the conversion was successful, and Err otherwise.
pub fn sp_get_atom(term: SP_term_ref) -> Result<SP_atom, PrologError> {
    let mut atom = SP_atom::default();
    let atom_ptr = &mut atom as *mut SP_atom;
    let ret_val = unsafe { SP_get_atom(term, atom_ptr) as u32 };
    if ret_val == 0 {
        Err(PrologError::TermConversionError(format!(
            "Could not convert term {:?} to an atom.",
            term
        )))
    } else {
        Ok(atom)
    }
}

// pub fn SP_get_float(term: SP_term_ref, f: *mut f64) -> c_int;
// pub fn SP_get_functor(term: SP_term_ref, name: *mut SP_atom, arity: *mut c_int) -> c_int;

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
            Err(PrologError::TermConversionError(format!(
                "Could not retrieve term {:?} as an integer.",
                term_ref
            )))
        } else {
            Ok(i)
        }
    }
}

/// Save wrapper around the unsafe [SP_get_float] function from Prolog.
/// If the term is an Integer that does not fit in a double, then the call will fail.
pub fn sp_get_float(term: SP_term_ref) -> Result<f64, PrologError> {
    unsafe {
        let mut f: f64 = 0.0;
        let p: *mut f64 = &mut f;
        let ret_val: c_int = SP_get_float(term, p);
        if ret_val == 0 {
            Err(PrologError::TermConversionError(format!(
                "Could not retrieve term {:?} as a float.",
                term
            )))
        } else {
            Ok(f)
        }
    }
}

/// Save wrapper around the unsafe [SP_get_functor] function from Prolog.
/// # Arguments
/// * `term` - The term reference to convert.
/// # Returns a Result of
/// * `Ok((atom, arity))` - The atom and arity of the term reference wrapped in an [Ok] variant.
/// * `Err(PrologError::TermConversionError)` - If the term reference could not be converted.
pub fn sp_get_functor(term: SP_term_ref) -> Result<(SP_atom, usize), PrologError> {
    let mut atom = SP_atom::default();
    let atom_ptr = &mut atom as *mut SP_atom;
    let mut arity: c_int = 0;
    let arity_ptr = &mut arity as *mut c_int;
    let ret_val = unsafe { SP_get_functor(term, atom_ptr, arity_ptr) };
    if ret_val == 0 {
        Err(PrologError::TermConversionError(format!(
            "Could not retrieve term {:?} as a functor.",
            term
        )))
    } else {
        Ok((atom, arity as usize))
    }
}

pub fn sp_get_integer_bytes(
    term: SP_term_ref,
    buf: *mut c_void,
    pbuf_size: *mut usize,
    native: c_int,
) -> c_int {
    unsafe { SP_get_integer_bytes(term, buf, pbuf_size, native) }
}

pub fn sp_get_list(list: SP_term_ref) -> Option<(SP_term_ref, SP_term_ref)> {
    let head = sp_new_term_ref();
    let tail = sp_new_term_ref();
    let result: c_int = unsafe { SP_get_list(list, head, tail) };
    if result == 0 {
        None
    } else {
        Some((head, tail))
    }
}

pub fn sp_get_list_codes(term: SP_term_ref) -> Result<String, PrologError> {
    let s = core::ptr::null_mut();
    let ret_val = unsafe { SP_get_list_codes(term, s) };
    if ret_val == 0 {
        Err(PrologError::TermConversionError(format!(
            "Could not convert term {:?} to a list of codes.",
            term
        )))
    } else {
        unsafe {
            Ok(CStr::from_ptr(s as *const c_char)
                .to_str()
                .unwrap()
                .to_string())
        }
    }
}

pub fn sp_get_list_n_bytes(
    term: SP_term_ref,
    tail: SP_term_ref,
    n: usize,
    w: *mut usize,
    s: *mut c_uchar,
) -> c_int {
    unsafe { SP_get_list_n_bytes(term, tail, n, w, s) }
}
pub fn sp_get_list_n_codes(
    term: SP_term_ref,
    tail: SP_term_ref,
    n: usize,
    w: *mut usize,
    s: *mut c_char,
) -> c_int {
    unsafe { SP_get_list_n_codes(term, tail, n, w, s) }
}

pub fn sp_get_number_codes(term: SP_term_ref, s: *mut *const c_char) -> c_int {
    unsafe { SP_get_number_codes(term, s) }
}

/// Save wrapper around the unsafe [SP_get_string] function from Prolog.
/// # Arguments
/// * `term_ref` - The term reference to convert.
/// # Returns a Result of
/// * `Ok(s)` - The String value of the term reference wrapped in an [Ok] variant.
/// * `Err(PrologError::TermConversionError)` - If the term reference could not be converted.
pub fn sp_get_string(term_ref: SP_term_ref) -> Result<String, PrologError> {
    unsafe {
        let mut s: *const c_char = core::ptr::null_mut();
        let ret_val: c_int = SP_get_string(term_ref, &mut s as *mut *const c_char);
        if ret_val == 0 || s.is_null() {
            Err(PrologError::TermConversionError(format!(
                "Could not retrieve term {:?} as a string.",
                term_ref
            )))
        } else {
            Ok(string_from_ref(s))
        }
    }
}

// #[cfg(test)]
// #[test]
// fn test_sp_get_string() {
//     let _mutex_guard = crate::test_utils::get_lock();
//     use mockall::predicate::{always, eq};
//     let t: SP_term_ref = SP_term_ref::default();
//     let sp_get_string_ctx = SP_get_string_context();
//     sp_get_string_ctx
//         .expect()
//         .with(eq(t), always())
//         .returning(|_, pointer| {
//             unsafe {
//                 *pointer = crate::test_utils::TEST_ATOM_NAME_STR.as_ptr() as *const c_char;
//             }
//             1
//         });
//     let res = sp_get_string(t).unwrap();
//     assert_eq!(res, "test".to_string());
// }

/// Look for the next solution in the given query.
///
/// # Arguments
/// * query - The query, created by [SP_open_query].
/// # Return Value
/// Ok(()) for success, and appropriate error otherwise.
/// # Description
/// This will cause the Prolog engine to backtrack over any current solution of an open query
/// and look for a new one. The given argument must be the innermost query that is still open,
/// i.e. it must not have been terminated explicitly by SP_close_query() or SP_cut_query().
/// Only when the return value is SP_SUCCESS are the values in the query arguments valid, and
/// will remain so until backtracking into this query or an enclosing one.
pub fn sp_next_solution(query: SP_qid) -> Result<(), PrologError> {
    let ret_val: c_int = unsafe { SP_next_solution(query) };
    if ret_val == SP_ERROR {
        // We got a SP_ERROR, so the safety condition is met and we can call sp_error_message.
        unsafe { Err(PrologError::NextSolutionError(sp_err_message())) }
    } else if ret_val == SP_FAILURE as c_int {
        Err(PrologError::NoMoreSolutions)
    } else if ret_val == SP_SUCCESS as c_int {
        Ok(())
    } else {
        Err(PrologError::UnexpectedReturnCode(ret_val))
    }
}

#[macro_export]
macro_rules! open_query {
    ($pred:expr,$($arg:expr),*) => {
        unsafe {
            let rewopar_valkjdasöfl = SP_open_query($pred, $($arg),*);
            if rewopar_valkjdasöfl == 0 {
                Err(PrologError::QueryOpenUnsuccessful)
            } else {
                Ok(rewopar_valkjdasöfl)
            }
        }
    };
}

/// Sets up a query for use by [SP_next_solution]. [SP_close_query] and [SP_cut_query]. Only supports arg arrays up to 15 args.
/// # Arguments
/// * `predicate` - The predicate to query.
/// * `arg1` - The first argument of the predicate.
/// # Returns
/// Result of SP_qid if the query was successful, and Err otherwise.
/// # Description
/// This function still contains a very ugly hack because Rust does not support this type of syntax.
/// If you want larger queries, call into the C API directly.
// pub fn sp_open_query(
//     predicate: SP_pred_ref,
//     args: Vec<SP_term_ref>,
// ) -> Result<SP_qid, PrologError> {
//     // TODO Rewrite with proc macro
//     unsafe {
//         let result = match args.len() {
//             0 => panic!("Not enough arguments!"),
//             1 => SP_open_query!(predicate, args[0]),
//             2 => SP_open_query!(predicate, args[0], args[1]),
//             3 => SP_open_query!(predicate, args[0], args[1], args[2]),
//             4 => SP_open_query!(predicate, args[0], args[1], args[2], args[3]),
//             5 => SP_open_query!(predicate, args[0], args[1], args[2], args[3], args[4]),
//             6 => SP_open_query!(predicate, args[0], args[1], args[2], args[3], args[4], args[5]),
//             7 => SP_open_query!(predicate, args[0], args[1], args[2], args[3], args[4], args[5], args[6]),
//             8 => SP_open_query!(predicate, args[0], args[1], args[2], args[3], args[4], args[5], args[6], args[7]),
//             9 => SP_open_query!(predicate, args[0], args[1], args[2], args[3], args[4], args[5], args[6], args[7], args[8]),
//             10 => SP_open_query!(predicate, args[0], args[1], args[2], args[3], args[4], args[5], args[6], args[7], args[8], args[9]),
//             11 => SP_open_query!(predicate, args[0], args[1], args[2], args[3], args[4], args[5], args[6], args[7], args[8], args[9], args[10]),
//             12 => SP_open_query!(predicate, args[0], args[1], args[2], args[3], args[4], args[5], args[6], args[7], args[8], args[9], args[10], args[11]),
//             13 => SP_open_query!(predicate, args[0], args[1], args[2], args[3], args[4], args[5], args[6], args[7], args[8], args[9], args[10], args[11], args[12]),
//             14 => SP_open_query!(predicate, args[0], args[1], args[2], args[3], args[4], args[5], args[6], args[7], args[8], args[9], args[10], args[11], args[12], args[13]),
//             15 => SP_open_query!(predicate, args[0], args[1], args[2], args[3], args[4], args[5], args[6], args[7], args[8], args[9], args[10], args[11], args[12], args[13], args[14]),
//             _ => panic!("Queries of more than 15 args are not supported by the wrapper API. For large Queries call the C API directly"),
//         };
//         if result == 0 {
//             Err(PrologError::QueryOpenUnsuccessful)
//         } else {
//             Ok(result)
//         }
//     }
// }

/// Returns a pointer to the predicate definition.
///
/// # Return Value
/// The reference if the predicate is found, NULL otherwise with error code PRED_NOT_FOUND.
pub fn sp_pred(name_atom: &str, arity: u32, module_atom: &str) -> Result<SP_pred_ref, PrologError> {
    let name_atom = sp_atom_from_string(name_atom)?;
    let module_atom = sp_atom_from_string(module_atom)?;
    let ret_val = unsafe { SP_pred(name_atom, arity as SP_integer, module_atom) };
    if ret_val.is_null() {
        Err(PrologError::PredicateNotFound)
    } else {
        Ok(ret_val)
    }
}

pub fn sp_expand_file_name(
    relpath: *const c_char,
    cwd: *mut c_char,
    options: spio_t_bits,
    pabspath: *mut *mut c_char,
) -> c_int {
    unsafe { SP_expand_file_name(relpath, cwd, options, pabspath) }
}

// pub fn sp_get_byte(stream: *mut SP_stream) -> spio_t_error_code {
//     unsafe { SP_get_byte(stream) }
// }
// pub fn sp_get_code(stream: *mut SP_stream) -> spio_t_error_code {
//     unsafe { SP_get_code(stream) }
// }
pub fn sp_unget_byte(stream: *mut SP_stream, item: c_int) -> spio_t_error_code {
    unsafe { SP_unget_byte(stream, item) }
}
pub fn sp_unget_code(stream: *mut SP_stream, item: c_int) -> spio_t_error_code {
    unsafe { SP_unget_code(stream, item) }
}
// pub fn sp_fprintf(stream: *mut SP_stream, fmt: *const c_char, args: Vec<_>) -> spio_t_error_code {
//     unsafe { SP_fprintf(stream, fmt) }
// }

// pub fn sp_printf(fmt: *const c_char, ...) -> spio_t_error_code;
// pub fn sp_put_byte(stream: *mut SP_stream, item: c_int) -> spio_t_error_code {
//     unsafe { SP_put_byte(stream, item) }
// }
pub fn sp_put_bytes(
    strea: *mut SP_stream,
    codes: *const spio_t_uint8,
    byte_count: usize,
    options: spio_t_bits,
) -> spio_t_error_code {
    unsafe { SP_put_bytes(strea, codes, byte_count, options) }
}
// pub fn sp_put_code(stream: *mut SP_stream, item: c_int) -> spio_t_error_code {
//     unsafe { SP_put_code(stream, item) }
// }

pub fn sp_put_codes(
    strea: *mut SP_stream,
    codes: *const spio_t_wchar,
    code_count: usize,
    options: spio_t_bits,
) -> spio_t_error_code {
    unsafe { SP_put_codes(strea, codes, code_count, options) }
}
pub fn sp_put_encoded_string(
    stream: *mut SP_stream,
    encoded_string: *const c_char,
    options: spio_t_bits,
) -> spio_t_error_code {
    unsafe { SP_put_encoded_string(stream, encoded_string, options) }
}

pub fn sp_fclose(stream: *mut SP_stream, close_options: spio_t_bits) -> spio_t_error_code {
    unsafe { SP_fclose(stream, close_options) }
}

pub fn sp_fopen(
    pathname: *const c_char,
    reserved: *mut c_void,
    options: spio_t_bits,
    pstream: *mut *mut SP_stream,
) -> spio_t_error_code {
    unsafe { SP_fopen(pathname, reserved, options, pstream) }
}

// pub fn user_flush_output(user_data: *mut c_void, flush_options: spio_t_bits) -> spio_t_error_code {
//     unsafe { super::sys::user_flush_output(user_data, flush_options) }
// }

pub fn sp_load(filename: *const c_char) -> c_int {
    unsafe { SP_load(filename) }
}
pub fn sp_restore(filenmae: *const c_char) -> c_int {
    unsafe { SP_restore(filenmae) }
}
/// Create a Prolog stream that will call user defined functions to perform stream operations.
///
/// # Arguments
/// * user_data - This is a pointer to arbitrary user specified data. It is passed to all user defined stream methods. It must not be NULL.
/// * user_class - Arbitrary pointer. This is used with [SP_get_stream_user_data], which see.
/// * user_read - If non-NULL then this is an input stream.
/// * user_write - If non-NULL then this is an output stream.
/// * user_flush_output - Will be called to flush output on the stream. Ignored if user_write is NULL.
///   Can be NULL if the stream need not be flushed, e.g. if user_write always ensures that any output reaches its destination immediately.
/// * user_seek - Reserved, should be NULL.
/// * user_close - Closes the stream. See Section 12.3.104 [cpg-ref-user close], page 1394, for details.
/// * user_interrupt - Reserved, should be NULL.
/// * user_ioctl - Reserved, should be NULL.
/// * create_stream_options - Reserved, should be 0.
///
/// ## The following bits can be set:
/// * SP_CREATE_STREAM_OPTION_BINARY
///    This is a binary stream. The user_read and user_write methods
///    transfer bytes.
/// * SP_CREATE_STREAM_OPTION_TEXT
///    This is a TEXT stream. The user_read and user_write methods
///    transfer wide characters.
/// * SP_CREATE_STREAM_OPTION_AUTOFLUSH
///    After writing to this stream prolog predicates will do a flush_
///    output/1. In essence this ensures that the stream behaves as if it
///    were unbuffered.
/// * SP_CREATE_STREAM_OPTION_INTERACTIVE
///    Treat this stream as an interactive stream. Implies SP_CREATE_
/// * STREAM_OPTION_AUTOFLUSH.
/// * SP_CREATE_STREAM_OPTION_EOF_ON_EOF
/// * SP_CREATE_STREAM_OPTION_RESET_ON_EOF
///    These correspond to the open/4 options eof_action(eof) and
///    eof_action(reset) respectively. The default is to give an error if
///    reading after reaching end of file.
///    Exactly one of SP_CREATE_STREAM_OPTION_BINARY and SP_CREATE_STREAM_
///    OPTION_TEXT must be set.
/// * pstream - The new stream is returned here.
///
/// # Returns
///
/// On success, *pstream is assigned, and [SPIO_S_NOERR] or some other success code is returned.
/// You should use the [SPIO_FAILED()] macro to determine if the return value signifies failure
/// or success.
/// See also: <https://sicstus.sics.se/sicstus/docs/latest4/pdf/sicstus.pdf#Defining%20a%20New%20Stream>
pub fn sp_create_stream(
    user_data: *mut c_void,
    user_class: *const c_void,
    user_read: spio_t_simple_device_read,
    user_write: spio_t_simple_device_write,
    user_flush_output: spio_t_simple_device_flush_output,
    user_seek: spio_t_simple_device_seek,
    user_close: spio_t_simple_device_close,
    user_interrupt: spio_t_simple_device_interrupt,
    user_ioctl: spio_t_simple_device_ioctl,
    args: *mut c_void,
    create_stream_options: spio_t_bits,
    pstream: *mut *mut SP_stream,
) -> spio_t_error_code {
    unsafe {
        SP_create_stream(
            user_data,
            user_class,
            user_read,
            user_write,
            user_flush_output,
            user_seek,
            user_close,
            user_interrupt,
            user_ioctl,
            args,
            create_stream_options,
            pstream,
        )
    }
}
pub fn sp_set_user_stream_hook(
    hook: SP_UserStreamHook,
    user_data: *mut c_void,
) -> SP_UserStreamHook {
    unsafe { SP_set_user_stream_hook(hook, user_data) }
}

pub fn sp_set_user_stream_post_hook(
    hook: SP_UserStreamPostHook,
    user_data: *mut c_void,
) -> SP_UserStreamPostHook {
    unsafe { SP_set_user_stream_post_hook(hook, user_data) }
}

pub fn sp_get_current_dir() -> *mut c_char {
    unsafe { SP_get_current_dir() }
}

pub fn sp_get_stream_counts(
    stream: *mut SP_stream,
    ptiem_count: *mut spio_t_offset,
    pnewline_count: *mut spio_t_offset,
    pline_length: *mut spio_t_offset,
    options: spio_t_bits,
) -> spio_t_error_code {
    unsafe { SP_get_stream_counts(stream, ptiem_count, pnewline_count, pline_length, options) }
}

pub fn sp_get_stream_user_data(
    stream: *mut SP_stream,
    user_class: *const c_void,
    puser_data: *mut *mut c_void,
) -> spio_t_error_code {
    unsafe { SP_get_stream_user_data(stream, user_class, puser_data) }
}

pub fn sp_getenv(name: *const c_char) -> *mut c_char {
    unsafe { SP_getenv(name) }
}

/// Schedules a function for execution in the main thread contexts where queries cannot be issued.
///
/// # Arguments
/// * func - The function to call.
/// * arg - The argument to pass to the function.
///
/// # Returns
/// Nonzero on success, and 0 otherwise.
///
/// # Description
/// If you wish to call Prolog back from a signal handler that has been installed with [SP_signal]
/// or a thread other than the thread that called [SP_initialize], that is, the main thread,
/// you cannot use [SP_query] etc. directly. The call to Prolog has to be delayed until such
/// time that the Prolog execution can accept an interrupt and the call has to be performed
/// from the main thread (the Prolog execution thread). This function serves this purpose, and
/// installs func to be called from Prolog (in the main thread) when the execution can accept
/// a callback.
/// A queue of functions, with corresponding arguments, is maintained; that is, if several calls
/// to [SP_event] occur before Prolog can accept an interrupt, the functions are queued and
/// executed in turn at the next possible opportunity. A func installed with SP_event() will
/// not be called until SICStus is actually running. One way of ensuring that all pending
/// functions installed with [SP_event] are run is to call, from the main thread, some dummy
/// goal, such as,
/// ```c
/// SP_query_cut_fail(SP_predicate("true",0,"user"));
/// ```
/// While SP_event() is safe to call from any thread, it is not safe to call from arbitrary signal
/// handlers. If you want to call SP_event() when a signal is delivered, you need to install
/// your signal handler with SP_signal().
/// Note that SP_event() is one of the very few functions in the SICStus API that can safely
/// be called from another thread than the main thread.
/// Depending on the value returned from func, the interrupted Prolog execution will just
/// continue (SP_SUCCESS) or backtrack (SP_FAILURE or SP_ERROR). An exception raised by
/// func, using SP_raise_exception(), will be processed in the interrupted Prolog execution.
/// If func calls SP_fail() or SP_raise_exception() the return value from func is ignored
/// and handled as if func returned SP_FAILURE or SP_ERROR, respectively. In case of failure
/// or exception, the event queue is flushed.
/// It is generally not robust to let func raise an exception or (even worse) fail. The reason is
/// that not all Prolog code is written such that it gracefully handles being interrupted. If you
/// want to interrupt some long-running Prolog code, it is better to let the event handler set a
/// flag (in C) and let your Prolog code test the flag (using a foreign predicate) in some part
/// of your code that is executed repeatedly.
/// See also: <https://sicstus.sics.se/sicstus/docs/latest4/pdf/sicstus.pdf#Calling%20Prolog%20Asynchronously>
pub fn sp_event(func: SP_EventFun, arg: *mut c_void) -> c_int {
    unsafe { SP_event(func, arg) }
}

/// Compares two terms.
///
/// # Arguments
/// * x - The first term to compare.
/// * y - The second term to compare.
///
/// # Returns
/// * An appropriate rust [std::cmp::Ordering] value.
/// # Safety
/// If Prolog returns a value that does not match the ones expected in the documentation this will panic.
/// See also: <https://sicstus.sics.se/sicstus/docs/latest4/pdf/sicstus.pdf#ref-lte-cte>
pub fn sp_compare(x: SP_term_ref, y: SP_term_ref) -> Ordering {
    let res = unsafe { SP_compare(x, y) };
    match res {
        -1 => Ordering::Less,
        0 => Ordering::Equal,
        1 => Ordering::Greater,
        _ => panic!("Unexpected return value from SP_compare: {}", res),
    }
}

/// Create a new empty term reference, initialized to the empty list [].
/// See also: <https://sicstus.sics.se/sicstus/docs/latest4/pdf/sicstus.pdf#Creating%20and%20Manipulating%20SP_term_refs>
pub fn sp_new_term_ref() -> SP_term_ref {
    unsafe { SP_new_term_ref() }
}

/// Unifies two terms.
///
/// # Arguments
/// * x - The first term to unify.
/// * y - The second term to unify.
///
/// # Returns
/// * Result of union if the unification is successful, and Err otherwise.
///
/// Bear in mind that the unification may unblock some goals. such goals are not run in the
/// scope of SP_unify; they remain pending until the next Prolog goal is run.
/// See also: <https://sicstus.sics.se/sicstus/docs/latest4/pdf/sicstus.pdf#Unifying%20and%20Comparing%20Terms>
pub fn sp_unify(x: SP_term_ref, y: SP_term_ref) -> Result<(), PrologError> {
    let res = unsafe { SP_unify(x, y) };
    if res == 1 {
        Ok(())
    } else {
        Err(PrologError::UncussefulUnificationError(x, y))
    }
}

// Type Tests

/// Determines whether the value of *term* is a Prolog atom.
pub fn sp_is_atom(term: SP_term_ref) -> bool {
    unsafe { SP_is_atom(term) == 1 }
}

/// Determines whether the value of *term* is a Prolog atomic term.
/// Atomic terms are atoms, integers or floats.
pub fn sp_is_atomic(term: SP_term_ref) -> bool {
    unsafe { SP_is_atomic(term) == 1 }
}

/// Determines whether the value of *term* is a Prolog compound term.
/// See: <https://sicstus.sics.se/sicstus/docs/latest4/pdf/sicstus.pdf#ref-syn-cpt>
pub fn sp_is_compound(term: SP_term_ref) -> bool {
    unsafe { SP_is_compound(term) == 1 }
}

/// Determines whether the value of *term* is a Prolog float.
pub fn sp_is_float(term: SP_term_ref) -> bool {
    unsafe { SP_is_float(term) == 1 }
}

/// Determines whether the value of *term* is a Prolog integer.
pub fn sp_is_integer(term: SP_term_ref) -> bool {
    unsafe { SP_is_integer(term) == 1 }
}

/// Determines whether the value of *term* is a Prolog list.
pub fn sp_is_list(term: SP_term_ref) -> bool {
    unsafe { SP_is_list(term) == 1 }
}

/// Determines whether the value of *term* is a Prolog number, meaning float or integer.
pub fn sp_is_number(term: SP_term_ref) -> bool {
    unsafe { SP_is_number(term) == 1 }
}

/// Determines whether the value of *term* is a Prolog variable.
pub fn sp_is_variable(term: SP_term_ref) -> bool {
    unsafe { SP_is_variable(term) == 1 }
}

/// Determines the type of the value of term.
pub fn sp_term_type(term: SP_term_ref) -> Result<c_int, PrologError> {
    let res = unsafe { SP_term_type(term) };
    if res == SP_TYPE_ERROR as c_int {
        Err(PrologError::TypeCheckError)
    } else {
        Ok(res)
    }
}

pub fn sp_put_address(term: SP_term_ref, address: *mut c_void) -> Result<(), PrologError> {
    let ret_val = unsafe { SP_put_address(term, address) };
    if ret_val == 0 {
        Err(PrologError::TermConversionError(format!(
            "Failed putting address {:?} into term {:?}",
            address, term
        )))
    } else {
        Ok(())
    }
}

pub fn sp_put_list(term: SP_term_ref) -> Result<(), PrologError> {
    let ret_val = unsafe { SP_put_list(term) };
    if ret_val == 0 {
        Err(PrologError::TermConversionError(format!(
            "Failed putting list into term {:?}",
            term
        )))
    } else {
        Ok(())
    }
}

pub fn sp_put_list_codes(
    term: SP_term_ref,
    tail: SP_term_ref,
    s: *const c_char,
) -> Result<(), PrologError> {
    let ret_val = unsafe { SP_put_list_codes(term, tail, s) };
    if ret_val == 0 {
        Err(PrologError::TermConversionError(format!(
            "Failed putting list codes for string {:?} with tail {:?} into term {:?}",
            s, tail, term
        )))
    } else {
        Ok(())
    }
}

pub fn sp_put_list_n_bytes(
    term: SP_term_ref,
    tail: SP_term_ref,
    n: usize,
    s: *const u8,
) -> Result<(), PrologError> {
    let ret_val = unsafe { SP_put_list_n_bytes(term, tail, n, s) };
    if ret_val == 0 {
        Err(PrologError::TermConversionError(format!(
            "Failed putting list codes for string {:?} with tail {:?} into term {:?}",
            s, tail, term
        )))
    } else {
        Ok(())
    }
}

pub fn sp_put_list_n_codes(
    term: SP_term_ref,
    tail: SP_term_ref,
    n: usize,
    s: *const c_char,
) -> Result<(), PrologError> {
    let ret_val = unsafe { SP_put_list_n_codes(term, tail, n, s) };
    if ret_val == 0 {
        Err(PrologError::TermConversionError(format!(
            "Failed putting list codes for string {:?} with tail {:?} into term {:?}",
            s, tail, term
        )))
    } else {
        Ok(())
    }
}

pub fn sp_put_number_codes(term: SP_term_ref, s: *const c_char) -> Result<(), PrologError> {
    let ret_val = unsafe { SP_put_number_codes(term, s) };
    if ret_val == 0 {
        Err(PrologError::TermConversionError(format!(
            "Failed putting number codes for string {:?} into term {:?}",
            s, term
        )))
    } else {
        Ok(())
    }
}

pub fn sp_put_string(term: SP_term_ref, s: *const c_char) -> Result<(), PrologError> {
    let ret_val = unsafe { SP_put_string(term, s) };
    if ret_val == 0 {
        Err(PrologError::TermConversionError(format!(
            "Failed putting string {:?} into term {:?}",
            s, term
        )))
    } else {
        Ok(())
    }
}

pub fn sp_put_atom(term: SP_term_ref, atom: SP_atom) -> Result<(), PrologError> {
    let ret_val = unsafe { SP_put_atom(term, atom) };
    if ret_val == 0 {
        Err(PrologError::TermConversionError(format!(
            "Failed putting atom {} into term {:?}",
            atom, term
        )))
    } else {
        Ok(())
    }
}

/// Make the term a variable.
pub fn sp_put_variable(term: SP_term_ref) -> Result<(), PrologError> {
    let ret_val = unsafe { SP_put_variable(term) };
    if ret_val == 0 {
        Err(PrologError::TermConversionError(format!(
            "Failed putting variable into term {:?}",
            term
        )))
    } else {
        Ok(())
    }
}

/// Make the term a float.
pub fn sp_put_float(term: SP_term_ref, f: f64) -> Result<(), PrologError> {
    let ret_val = unsafe { SP_put_float(term, f) };
    if ret_val == 0 {
        Err(PrologError::TermConversionError(format!(
            "Failed putting float {} into term {:?}",
            f, term
        )))
    } else {
        Ok(())
    }
}

/// Assigns to *term* a reference to a compund term with all the arguments unbound variables.
/// If arity is 0, assigns the Prolog atom whose canonical representation is *name*. This is similar
/// calling functor/3 with the first argument unbound and the second and third arguments bound to an atom and an iteger, respectively.
pub fn sp_put_functor(term: SP_term_ref, name: SP_atom, arity: usize) -> Result<(), PrologError> {
    let ret_val = unsafe { SP_put_functor(term, name, arity as c_int) };
    if ret_val == 0 {
        Err(PrologError::TermConversionError(format!(
            "Failed putting functor {} with arity {} into term {:?}",
            name, arity, term
        )))
    } else {
        Ok(())
    }
}

/// Make the term a float.
pub fn sp_put_integer(term: SP_term_ref, i: i64) -> Result<(), PrologError> {
    let ret_val = unsafe { SP_put_integer(term, i) };
    if ret_val == 0 {
        Err(PrologError::TermConversionError(format!(
            "Failed putting integer {} into term {:?}",
            i, term
        )))
    } else {
        Ok(())
    }
}

pub fn sp_put_integer_bytes(
    term: SP_term_ref,
    buf: *mut c_void,
    buf_size: usize,
    native: bool,
) -> Result<(), PrologError> {
    let native = if native { 1 } else { 0 };
    let ret_val = unsafe { SP_put_integer_bytes(term, buf, buf_size, native) };
    if ret_val == 0 {
        Err(PrologError::TermConversionError(format!(
            "Failed putting integer bytes from buffer {:?} into term {:?}",
            buf, term
        )))
    } else {
        Ok(())
    }
}

/// Copy a Prolog term into another term.
pub fn sp_put_term(to: SP_term_ref, from: SP_term_ref) -> Result<(), PrologError> {
    let ret_val = unsafe { SP_put_term(to, from) };
    if ret_val == 0 {
        Err(PrologError::TermConversionError(format!(
            "Failed putting term {:?} into term {:?}",
            from, to
        )))
    } else {
        Ok(())
    }
}

/// Obtain the encoded string holding the characters of a Prolog atom.
pub fn sp_string_from_atom(atom: SP_atom) -> String {
    let s: *const c_char = unsafe { SP_string_from_atom(atom) };
    unsafe { string_from_ref(s) }
}

/// Registers the atom *atom* with the Prolog memory manager by incrementing its reference count.
pub fn sp_register_atom(atom: SP_atom) -> Result<(), PrologError> {
    let ret_val = unsafe { SP_register_atom(atom) };
    if ret_val == 0 {
        Err(PrologError::AtomRegistrationError(atom))
    } else {
        Ok(())
    }
}

/// Unregisters the atom *atom* with the Prolog memory manager by decrementing its reference count.
pub fn sp_unregister_atom(atom: SP_atom) -> Result<(), PrologError> {
    let ret_val = unsafe { SP_unregister_atom(atom) };
    if ret_val == 0 {
        Err(PrologError::AtomUnregistrationError(atom))
    } else {
        Ok(())
    }
}
