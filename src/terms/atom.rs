use core::cmp::Ordering;

use alloc::string::String;

use crate::{sys::*, util::is_valid_atom_name};

use super::Term;

#[derive(Debug, Clone)]
pub struct Atom {
    term_ref: SP_term_ref,
    atom_id: SP_atom,
    name: String,
}

impl Atom {
    pub fn new(name: String) -> Self {
        assert!(is_valid_atom_name(&name));
        let term_ref = sp_new_term_ref();
        let atom_id = sp_atom_from_string(&name).unwrap();
        sp_register_atom(atom_id).unwrap();
        Atom {
            term_ref,
            atom_id,
            name,
        }
    }

    pub fn len(&self) -> usize {
        sp_atom_length(self.atom_id)
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

    pub fn as_term_ref(&self) -> SP_term_ref {
        self.term_ref
    }
}

impl Drop for Atom {
    fn drop(&mut self) {
        sp_unregister_atom(self.atom_id).unwrap();
    }
}

impl PartialEq for Atom {
    fn eq(&self, other: &Self) -> bool {
        sp_compare(self.term_ref, other.term_ref) == Ordering::Equal
    }
}

impl Eq for Atom {}

impl PartialOrd for Atom {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(sp_compare(self.term_ref, other.term_ref))
    }
}

impl Ord for Atom {
    fn cmp(&self, other: &Self) -> Ordering {
        sp_compare(self.term_ref, other.term_ref)
    }
}

impl From<SP_atom> for Atom {
    fn from(atom_id: SP_atom) -> Self {
        let name = sp_string_from_atom(atom_id);
        let term_ref = sp_new_term_ref();
        sp_put_atom(term_ref, atom_id).unwrap();
        Atom {
            term_ref,
            atom_id,
            name,
        }
    }
}

impl Term for Atom {
    fn from(term_ref: SP_term_ref) -> Self {
        let atom_id = sp_get_atom(term_ref).unwrap();
        let name = sp_string_from_atom(atom_id);
        Atom {
            term_ref,
            atom_id,
            name,
        }
    }

    fn into(self) -> SP_term_ref {
        self.term_ref
    }
}

impl From<&str> for Atom {
    fn from(name: &str) -> Self {
        Atom::new(String::from(name))
    }
}
