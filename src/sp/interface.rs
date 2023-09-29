//! This module contains the safe wrappers around the functions from the 'Foreign Interface' section.
//! See <https://sicstus.sics.se/sicstus/docs/latest4/pdf/sicstus.pdf#cpg-top-fin>

use super::sys::*;
use super::{errors::sp_err_message, terms::sp_new_term_ref};
use crate::error::PrologError;
use crate::util::string_from_ref;

use std::{
    ffi::{c_uchar, c_void},
    os::raw::{c_char, c_int},
};

/// Retrieve the Prolog ID corresponding to the given atom name.
///
/// # Arguments
/// * `atom_name` - A rust &str containing the atom name.
///
/// # Returns
/// The Prolog ID corresponding to the given atom name if it exists, and an error otherwise.
/// See also: <https://sicstus.sics.se/sicstus/docs/latest4/pdf/sicstus.pdf#Creating%20and%20Manipulating%20SP_term_refs>
pub fn sp_atom_from_string(atom_name: &str) -> Result<SP_atom, PrologError> {
    let atom_id: SP_atom = unsafe { SP_atom_from_string(atom_name.as_ptr() as *const c_char) };
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
pub fn sp_cons_list(head: SP_term_ref, tail: SP_term_ref) -> Result<SP_term_ref, PrologError> {
    let term = sp_new_term_ref();
    let ret_val = unsafe { SP_cons_list(term, head, tail) };
    if ret_val == 0 {
        Err(PrologError::TermConversionError)
    } else {
        Ok(term)
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
pub fn sp_get_address(term: SP_term_ref) -> Result<*mut *mut c_void, PrologError> {
    let p = std::ptr::null_mut();
    let ret_val = unsafe { SP_get_address(term, p) };
    if ret_val == 0 {
        Err(PrologError::TermConversionError)
    } else {
        Ok(p)
    }
}

/// Returns a SP_term_ref to the i'th argument of a compound *term*.
pub fn sp_get_arg(i: usize, term: SP_term_ref) -> Result<SP_term_ref, PrologError> {
    let arg = sp_new_term_ref();
    let ret_val = unsafe { SP_get_arg(i as c_int, term, arg) };
    if ret_val == 0 {
        Err(PrologError::TermConversionError)
    } else {
        Ok(arg)
    }
}
pub fn sp_get_atom(term: SP_term_ref, atom: *mut SP_atom) -> c_int {
    unsafe { SP_get_atom(term, atom) }
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
            Err(PrologError::TermConversionError)
        } else {
            Ok(i)
        }
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

pub fn sp_get_list(list: SP_term_ref, head: SP_term_ref, tail: SP_term_ref) -> c_int {
    unsafe { SP_get_list(list, head, tail) }
}
pub fn sp_get_list_codes(term: SP_term_ref, s: *mut *const c_char) -> c_int {
    unsafe { SP_get_list_codes(term, s) }
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
        let mut s: *const c_char = std::ptr::null_mut();
        let ret_val: c_int = SP_get_string(term_ref, &mut s as *mut *const c_char);
        if ret_val == 0 || s.is_null() {
            Err(PrologError::TermConversionError)
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

/// Sets up a query for use by [SP_next_solution]. [SP_close_query] and [SP_cut_query]. Only supports arg arrays up to 15 args.
/// # Arguments
/// * `predicate` - The predicate to query.
/// * `arg1` - The first argument of the predicate.
/// # Returns
/// Result of SP_qid if the query was successful, and Err otherwise.
/// # Description
/// This function still contains a very ugly hack because Rust does not support this type of syntax.
/// If you want larger queries, call into the C API directly.
pub fn sp_open_query(
    predicate: SP_pred_ref,
    args: Vec<SP_term_ref>,
) -> Result<SP_qid, PrologError> {
    // TODO Rewrite with proc macro
    unsafe {
        let result = match args.len() {
            0 => panic!("Not enough arguments!"),
            1 => SP_open_query(predicate, args[0]),
            2 => SP_open_query(predicate, args[0], args[1]),
            3 => SP_open_query(predicate, args[0], args[1], args[2]),
            4 => SP_open_query(predicate, args[0], args[1], args[2], args[3]),
            5 => SP_open_query(predicate, args[0], args[1], args[2], args[3], args[4]),
            6 => SP_open_query(predicate, args[0], args[1], args[2], args[3], args[4], args[5]),
            7 => SP_open_query(predicate, args[0], args[1], args[2], args[3], args[4], args[5], args[6]),
            8 => SP_open_query(predicate, args[0], args[1], args[2], args[3], args[4], args[5], args[6], args[7]),
            9 => SP_open_query(predicate, args[0], args[1], args[2], args[3], args[4], args[5], args[6], args[7], args[8]),
            10 => SP_open_query(predicate, args[0], args[1], args[2], args[3], args[4], args[5], args[6], args[7], args[8], args[9]),
            11 => SP_open_query(predicate, args[0], args[1], args[2], args[3], args[4], args[5], args[6], args[7], args[8], args[9], args[10]),
            12 => SP_open_query(predicate, args[0], args[1], args[2], args[3], args[4], args[5], args[6], args[7], args[8], args[9], args[10], args[11]),
            13 => SP_open_query(predicate, args[0], args[1], args[2], args[3], args[4], args[5], args[6], args[7], args[8], args[9], args[10], args[11], args[12]),
            14 => SP_open_query(predicate, args[0], args[1], args[2], args[3], args[4], args[5], args[6], args[7], args[8], args[9], args[10], args[11], args[12], args[13]),
            15 => SP_open_query(predicate, args[0], args[1], args[2], args[3], args[4], args[5], args[6], args[7], args[8], args[9], args[10], args[11], args[12], args[13], args[14]),
            _ => panic!("Queries of more than 15 args are not supported by the wrapper API. For large Queries call the C API directly"),
        };
        if result == 0 {
            Err(PrologError::QueryOpenUnsuccessful)
        } else {
            Ok(result)
        }
    }
}

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
