use core::{cmp::Ordering, ffi::c_void};

use alloc::string::String;
use sicstus_sys::{SP_atom, SP_term_ref};

use crate::{
    sys::{self, sp_cons_list, sp_get_list, sp_new_term_ref, PrologError},
    Atom,
};

#[derive(Debug)]
pub struct TermRef {
    term_ref: SP_term_ref,
}

impl TermRef {
    pub fn new() -> Self {
        TermRef {
            term_ref: sys::sp_new_term_ref(),
        }
    }

    pub fn new_integer(integer: i64) -> Self {
        let mut term_ref = TermRef::new();
        term_ref.put_integer(integer).unwrap();
        term_ref
    }

    pub fn new_float(float: f64) -> Self {
        let mut term_ref = TermRef::new();
        term_ref.put_float(float).unwrap();
        term_ref
    }

    pub fn new_list() -> Self {
        let mut term_ref = TermRef::new();
        term_ref.put_list().unwrap();
        term_ref
    }

    pub fn term_ref(&self) -> SP_term_ref {
        self.term_ref
    }

    pub fn unify(&self, other: &Self) -> Result<(), PrologError> {
        sys::sp_unify(self.term_ref, other.term_ref)
    }

    pub fn cmp(&self, other: &Self) -> Ordering {
        sys::sp_compare(self.term_ref, other.term_ref)
    }
}

// Assigning to the SP_term_ref
impl TermRef {
    pub fn put_address(&mut self, address: *mut core::ffi::c_void) -> Result<(), PrologError> {
        sys::sp_put_address(self.term_ref, address)
    }

    pub fn put_atom(&mut self, atom: SP_atom) -> Result<(), PrologError> {
        sys::sp_put_atom(self.term_ref, atom)
    }

    pub fn put_float(&mut self, float: f64) -> Result<(), PrologError> {
        sys::sp_put_float(self.term_ref, float)
    }

    pub fn put_functor(&mut self, atom: SP_atom, arity: usize) -> Result<(), PrologError> {
        sys::sp_put_functor(self.term_ref, atom, arity)
    }

    pub fn put_integer(&mut self, integer: i64) -> Result<(), PrologError> {
        sys::sp_put_integer(self.term_ref, integer)
    }

    pub fn put_integer_bytes(&mut self, bytes: &mut [u8], native: bool) -> Result<(), PrologError> {
        sys::sp_put_integer_bytes(
            self.term_ref,
            bytes.as_ptr() as *mut c_void,
            bytes.len(),
            native,
        )
    }

    pub fn put_list(&mut self) -> Result<(), PrologError> {
        sys::sp_put_list(self.term_ref)
    }

    pub fn put_list_codes(&mut self, tail: &TermRef, s: &str) -> Result<(), PrologError> {
        sys::sp_put_list_codes(self.term_ref, tail.term_ref(), s.as_ptr() as *const i8)
    }

    pub fn put_list_n_bytes(
        &mut self,
        tail: &TermRef,
        n: usize,
        bytes: &[u8],
    ) -> Result<(), PrologError> {
        sys::sp_put_list_n_bytes(
            self.term_ref,
            tail.term_ref(),
            n,
            bytes.as_ptr() as *const u8,
        )
    }

    pub fn put_list_n_codes(
        &mut self,
        tail: &TermRef,
        n: usize,
        s: &str,
    ) -> Result<(), PrologError> {
        sys::sp_put_list_n_codes(
            self.term_ref,
            tail.term_ref(),
            n,
            s as *const str as *const i8,
        )
    }

    pub fn put_number_codes(&mut self, s: &str) -> Result<(), PrologError> {
        sys::sp_put_number_codes(self.term_ref, s as *const str as *const i8)
    }

    pub fn put_string(&mut self, s: &str) -> Result<(), PrologError> {
        sys::sp_put_string(self.term_ref, s as *const str as *const i8)
    }

    pub fn put_term(&mut self, term: &TermRef) -> Result<(), PrologError> {
        sys::sp_put_term(self.term_ref, term.term_ref())
    }

    pub fn put_variable(&mut self) -> Result<(), PrologError> {
        sys::sp_put_variable(self.term_ref)
    }

    pub fn cons(&mut self, head: TermRef) -> Result<(), PrologError> {
        assert!(self.is_list());
        sys::sp_cons_list(self.term_ref, head.term_ref, self.term_ref)
    }
}

// Type Checking the SP_term_ref
impl TermRef {
    pub fn is_atom(&self) -> bool {
        sys::sp_is_atom(self.term_ref)
    }

    pub fn is_atomic(&self) -> bool {
        sys::sp_is_atomic(self.term_ref)
    }

    pub fn is_compound(&self) -> bool {
        sys::sp_is_compound(self.term_ref)
    }

    pub fn is_float(&self) -> bool {
        sys::sp_is_float(self.term_ref)
    }

    pub fn is_integer(&self) -> bool {
        sys::sp_is_integer(self.term_ref)
    }

    pub fn is_list(&self) -> bool {
        sys::sp_is_list(self.term_ref)
    }

    pub fn is_number(&self) -> bool {
        sys::sp_is_number(self.term_ref)
    }

    pub fn is_variable(&self) -> bool {
        sys::sp_is_variable(self.term_ref)
    }
}

// Extracting from the SP_term_ref
impl TermRef {
    pub fn get_address(&self) -> Result<*mut c_void, PrologError> {
        sys::sp_get_address(self.term_ref)
    }

    pub fn get_arg(&self, index: usize) -> Result<TermRef, PrologError> {
        let term_ref: SP_term_ref = sys::sp_get_arg(index, self.term_ref)?;
        Ok(term_ref.into())
    }

    pub fn get_atom(&self) -> Result<Atom, PrologError> {
        sys::sp_get_atom(self.term_ref).map(|atom_id| Atom::from(atom_id))
    }

    pub fn get_float(&self) -> Result<f64, PrologError> {
        sys::sp_get_float(self.term_ref)
    }

    pub fn get_functor(&self) -> Result<(Atom, usize), PrologError> {
        let (atom, arity) = sys::sp_get_functor(self.term_ref)?;
        Ok((Atom::from(atom), arity))
    }

    pub fn get_integer(&self) -> Result<i64, PrologError> {
        sys::sp_get_integer(self.term_ref)
    }

    pub fn get_list(&self) -> Option<(TermRef, TermRef)> {
        let (head, tail) = sys::sp_get_list(self.term_ref)?;
        Some((head.into(), tail.into()))
    }

    pub fn get_list_codes(&self) -> Result<String, PrologError> {
        sys::sp_get_list_codes(self.term_ref)
    }

    pub fn get_string(&self) -> Result<String, PrologError> {
        sys::sp_get_string(self.term_ref)
    }
}

impl From<SP_term_ref> for TermRef {
    fn from(term_ref: SP_term_ref) -> Self {
        TermRef { term_ref }
    }
}

impl Into<SP_term_ref> for TermRef {
    fn into(self) -> SP_term_ref {
        self.term_ref
    }
}

impl IntoIterator for TermRef {
    type Item = TermRef;
    type IntoIter = TermRefIterator;

    fn into_iter(self) -> Self::IntoIter {
        TermRefIterator::new(self)
    }
}

pub struct TermRefIterator {
    term_ref: SP_term_ref,
}

impl TermRefIterator {
    pub fn new(term_ref: TermRef) -> Self {
        assert!(term_ref.is_list());
        TermRefIterator {
            term_ref: term_ref.term_ref(),
        }
    }
}

impl Iterator for TermRefIterator {
    type Item = TermRef;

    fn next(&mut self) -> Option<Self::Item> {
        let (head, tail) = sp_get_list(self.term_ref)?;
        self.term_ref = tail;
        Some(head.into())
    }
}

impl Clone for TermRef {
    fn clone(&self) -> Self {
        let mut term_ref = TermRef::new();
        term_ref.put_term(self).unwrap();
        term_ref
    }
}

impl FromIterator<TermRef> for TermRef {
    fn from_iter<I: IntoIterator<Item = TermRef>>(iter: I) -> Self {
        let l = sp_new_term_ref();
        for item in iter {
            sp_cons_list(l, item.term_ref, l).unwrap();
        }
        l.into()
    }
}
