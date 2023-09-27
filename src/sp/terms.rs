//! The functions relating to sections 12.2.9 and 12.2.10 of the Prolog manual.
//! See <https://sicstus.sics.se/sicstus/docs/latest4/pdf/sicstus.pdf#cpg-top-tic>

use crate::error::PrologError;

use super::sys::*;

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
pub fn sp_compare(x: SP_term_ref, y: SP_term_ref) -> std::cmp::Ordering {
    let res = unsafe { SP_compare(x, y) };
    match res {
        -1 => std::cmp::Ordering::Less,
        0 => std::cmp::Ordering::Equal,
        1 => std::cmp::Ordering::Greater,
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
