use crate::error::SicstusRsError;
use crate::sys;
use crate::SP_atom;
use alloc::string::String;
use sicstus_sys::SP_term_ref;

use core::cmp::Ordering;

use crate::TermRef;

#[derive(Debug, Clone)]
pub struct Atom {
    term_ref: TermRef,
    atom_id: SP_atom,
    name: String,
}

impl Atom {
    pub fn new(name: String) -> Self {
        let atom_id = sys::sp_atom_from_string(&name).unwrap();
        sys::sp_register_atom(atom_id).unwrap();
        let mut term_ref = TermRef::new();
        term_ref.put_atom(atom_id).unwrap();
        Atom {
            term_ref,
            atom_id,
            name,
        }
    }

    pub fn len(&self) -> usize {
        sys::sp_atom_length(self.atom_id)
    }

    pub fn atom_id(&self) -> SP_atom {
        self.atom_id
    }

    pub fn name(&self) -> &str {
        &self.name
    }

    pub fn as_str(&self) -> &str {
        &self.name
    }

    pub fn as_string(&self) -> String {
        self.name.clone()
    }

    pub fn as_atom(&self) -> SP_atom {
        self.atom_id
    }

    pub fn as_term_ref(&self) -> &TermRef {
        &self.term_ref
    }

    pub fn as_mut_term_ref(&mut self) -> &mut TermRef {
        &mut self.term_ref
    }

    pub fn unify(&mut self, other: &TermRef) -> Result<(), SicstusRsError> {
        self.term_ref.unify(other).map_err(|e| e.into())
    }

    pub fn from_string(name: String) -> Self {
        Atom::new(name)
    }
}

impl Drop for Atom {
    fn drop(&mut self) {
        sys::sp_unregister_atom(self.atom_id).unwrap();
    }
}

impl PartialEq for Atom {
    fn eq(&self, other: &Self) -> bool {
        self.cmp(other) == Ordering::Equal
    }
}

impl Eq for Atom {}

impl PartialOrd for Atom {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for Atom {
    fn cmp(&self, other: &Self) -> Ordering {
        self.term_ref.cmp(&other.term_ref)
    }
}

impl From<SP_atom> for Atom {
    fn from(atom_id: SP_atom) -> Self {
        let name = sys::sp_string_from_atom(atom_id);
        let mut term_ref = TermRef::new();
        term_ref.put_atom(atom_id).unwrap();
        Atom {
            term_ref,
            atom_id,
            name,
        }
    }
}

impl From<String> for Atom {
    fn from(name: String) -> Self {
        Atom::new(name)
    }
}

impl From<&str> for Atom {
    fn from(name: &str) -> Self {
        Atom::new(String::from(name))
    }
}

impl From<&SP_atom> for Atom {
    fn from(atom_id: &SP_atom) -> Self {
        Atom::from(*atom_id)
    }
}

impl From<&Atom> for SP_atom {
    fn from(atom: &Atom) -> Self {
        atom.atom_id
    }
}

impl TryFrom<SP_term_ref> for Atom {
    type Error = SicstusRsError;
    fn try_from(term_ref: SP_term_ref) -> Result<Self, SicstusRsError> {
        if !sys::sp_is_atom(term_ref) {
            return Err(SicstusRsError::TypeError(String::from(
                "Given Term is not an atom",
            )));
        }
        let atom_id =
            crate::sys::sp_get_atom(term_ref).map_err(|e| SicstusRsError::InternalError(e))?;
        Ok(atom_id.into())
    }
}
