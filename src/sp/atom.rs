use super::sys::*;
use crate::error::PrologError;

use std::ffi::c_char;

/// Retrieve the Prolog ID corresponding to the given atom name.
///
/// # Arguments
/// * `atom_name` - A rust String containing the atom name.
///
/// # Returns
/// The Prolog ID corresponding to the given atom name if it exists, and an error otherwise.
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
