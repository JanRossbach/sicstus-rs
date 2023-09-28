//! This module contains the safe wrappers around the functions from the 'Foreign Interface' section.
//! See <https://sicstus.sics.se/sicstus/docs/latest4/pdf/sicstus.pdf#cpg-top-fin>

use super::errors::sp_err_message;
use super::sys::*;
use crate::error::PrologError;
use crate::util::string_from_ref;

use std::{
    ffi::{c_void, c_uchar},
    os::raw::{c_char, c_int},
};

/// Retrieve the Prolog ID corresponding to the given atom name.
///
/// # Arguments
/// * `atom_name` - A rust String containing the atom name.
///
/// # Returns
/// The Prolog ID corresponding to the given atom name if it exists, and an error otherwise.
/// See also: <https://sicstus.sics.se/sicstus/docs/latest4/pdf/sicstus.pdf#Creating%20and%20Manipulating%20SP_term_refs>
pub fn sp_atom_from_string(atom_name: String) -> Result<SP_atom, PrologError> {
    let atom_id: SP_atom = unsafe { SP_atom_from_string(atom_name.as_ptr() as *const c_char) };
    if atom_id == 0 {
        Err(PrologError::AtomNotFound(atom_name))
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
/// * term - The [SP_term_ref] to be assigned.
/// * name - The name of the functor.
/// * arity - The arity of the functor.
/// * arg - The argument array.
///
/// # Returns
///
/// Zero if the conversion fails (as far as failure can be detected), and a nonzero value otherwise.
/// See also: <https://sicstus.sics.se/sicstus/docs/latest4/pdf/sicstus.pdf#Creating%20Prolog%20Terms>
pub fn sp_cons_functor(
    term: SP_term_ref,
    name: SP_atom,
    args: &[SP_term_ref],
) -> Result<(), PrologError> {
    let arg_p = args.as_ptr() as *mut SP_term_ref;
    let arity = args.len() as c_int;
    // We call the array version of the C API because rust does not support variadic functions.
    let ret_value = unsafe { SP_cons_functor_array(term, name, arity, arg_p) };
    if ret_value == 0 {
        Err(PrologError::ConsFunctorError)
    } else {
        Ok(())
    }
}

pub fn sp_cons_functor_array() -> c_int {
    unimplemented!(
        "Use sp_cons_functor instead, since it already uses the array version of the C API."
    )
}

// TODO From here

/// Assigns to term a reference to a Prolog list whose head and tail are the values of head and tail.
///
/// # Arguments
/// * term - The [SP_term_ref] to be assigned.
/// * head - The head of the new list.
/// * tail - The tail of the new list.
///
/// # Returns
/// Zero if the conversion fails (as far as failure can be detected), and a nonzero value otherwise.
/// See also: <https://sicstus.sics.se/sicstus/docs/latest4/pdf/sicstus.pdf#Creating%20Prolog%20Terms>
pub fn sp_cons_list(term: SP_term_ref, head: SP_term_ref, tail: SP_term_ref) -> c_int {
    unsafe { SP_cons_list(term, head, tail) }
}

/// Commit to the current solution to the given query, and close it.
///
/// # Arguments
/// * query - The query, created by [SP_open_query].
///
/// # Return Value
/// [SP_SUCCESS] for success, [SP_FAILURE] for failure, [SP_ERROR] if an error condition occurred.
/// See also: <https://sicstus.sics.se/sicstus/docs/latest4/pdf/sicstus.pdf#Finding%20Multiple%20Solutions%20of%20a%20Call>
///
/// # Description
/// This will discard the choices created since the corresponding SP_open_query(), like the goal
/// !. The current solution is retained in the arguments until backtracking into any enclosing
/// query. The given argument does not have to be the innermost open query; any open queries
/// in its scope will also be cut.
pub fn sp_cut_query(query: SP_qid) -> c_int {
    unsafe { SP_cut_query(query) }
}

/// Defines a Prolog predicate such that when the Prolog predicate is called it will call a
/// C function with a term corresponding to the Prolog goal.
///
/// # Arguments
///
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
) -> c_int {
    unsafe { SP_define_c_predicate(name, arity, module, proc, stash) }
}

pub fn sp_get_address(term: SP_term_ref, p: *mut *mut c_void) -> c_int {
    unsafe { SP_get_address(term, p) }
}

pub fn sp_get_arg(index: c_int, term: SP_term_ref, arg: SP_term_ref) -> c_int {
    unsafe { SP_get_arg(index, term, arg) }
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

pub fn sp_next_solution(query: SP_qid) -> c_int {
    unsafe { SP_next_solution(query) }
}

// macro_rules! apply {
//     ( $(A) ) => {
//         "It matches"
//     }
// }

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
