use super::Term;
use core::cmp::Ordering;
use core::fmt::{Formatter, Display};
use core::marker::PhantomData;

use alloc::fmt;

use crate::error::SicstusRsError;
use crate::util::{is_valid_atom_name, is_valid_variable_name};
use crate::sys::*;

#[derive(Debug)]
pub struct Atom {
    term_ref: SP_term_ref,
}

impl Term<Atom> {
    pub fn len(&self) -> usize {
        let atom = sp_get_atom(self.term_ref).unwrap();
        sp_atom_length(atom)
    }

    pub fn as_str(&self) -> &str {
        let atom = sp_get_atom(self.term_ref).unwrap();
        sp_get_string(atom).unwrap()
    }
}

impl Display for Term<Atom> {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        let s = sp_get_string(self.term_ref).unwrap();
        write!(f, "{}", s)
    }
}

impl From<&str> for Term<Atom> {
    fn from(name: &str) -> Self {
        assert!(
            is_valid_atom_name(name),
            "{} is not a valid atom name",
            name
        );
        // let term_ref: SP_term_ref = sp_new_term_ref();
        // let atom =
        Term {
            term_ref,
            kind: PhantomData::<Atom>,
        }
    }
}

impl From<SP_term_ref> for Term<Atom> {
    fn from(term_ref: SP_term_ref) -> Self {
        assert!(sp_is_atom(term_ref));
        Term {
            term_ref,
            kind: PhantomData::<Atom>,
        }
    }
}
