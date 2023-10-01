use super::Term;
use core::cmp::Ordering;
use core::fmt::{Formatter, Display};
use core::marker::PhantomData;

use alloc::fmt;

use crate::error::SicstusRsError;
use crate::util::{is_valid_atom_name, is_valid_variable_name};
use crate::sys::*;

#[derive(Debug)]
pub struct Atom;

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
        let term_ref: SP_term_ref = sp_new_term_ref();
        // let atom = sp_get_atom();
        // sp_put_atom(term_ref, name).unwrap();
        Term {
            term_ref,
            kind: PhantomData::<Atom>,
        }
    }
}
