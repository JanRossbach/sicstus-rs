///! High Level API for SICStus Prolog.
use std::ffi::{c_char, c_int};

// #[cfg(test)]
// use mockall::*;

/// Re export of the sicstus_sys crate low level bindings for SICStus Prolog.
/// This is purely for convenience and in order to opt out of the high level API if
/// you need fine grained control over the C API.
pub mod sys {
    #[cfg(test)]
    pub use sicstus_sys::mock_ffi::*;

    #[cfg(not(test))]
    pub use sicstus_sys::ffi::*;

    pub use sicstus_sys::variadic::*;
    pub use sicstus_sys::*;
}

#[cfg(test)]
mod unit_tests;

use sys::*;

#[derive(Debug)]
pub enum PrologError {
    TermConversionError,
    NoTermVariantMatch,
}

// region:    --- Error Boilerplate
impl core::fmt::Display for PrologError {
    fn fmt(&self, fmt: &mut core::fmt::Formatter) -> core::result::Result<(), core::fmt::Error> {
        write!(fmt, "{self:?}")
    }
}
impl std::error::Error for PrologError {}

#[derive(Debug, PartialEq)]
pub enum TermKind {
    Atom(String),
    Integer(i64),
    Float(f64),
    Compound(String, Vec<Term>),
    List(Vec<Term>),
    Var(String),
    EmptyList,
}

#[allow(dead_code)]
#[derive(Debug)]
pub struct Term {
    pub term_ref: SP_term_ref,
    pub kind: TermKind,
}

impl Term {
    pub fn new(kind: TermKind) -> Self {
        let term_ref: SP_term_ref = unsafe { SP_new_term_ref() };
        Term { term_ref, kind }
    }

    pub fn to_term_ref(&self) -> SP_term_ref {
        self.term_ref
    }
}

impl From<&str> for Term {
    fn from(name: &str) -> Self {
        let term_ref: SP_term_ref = unsafe { SP_new_term_ref() };
        Term {
            term_ref,
            kind: TermKind::Atom(name.to_string()),
        }
    }
}

pub fn string_from_ref(sp: *const c_char) -> String {
    let mut result = String::new();
    let mut cp: *const c_char = sp;
    unsafe {
        loop {
            let c: c_char = *cp;
            let c = c as u8 as char;
            if c == '\0' {
                break;
            }
            result.push(c as u8 as char);
            cp = cp.add(1);
        }
    }
    result
}

/// Save wrapper around the unsafe [SP_get_string] function from Prolog.
pub fn sp_get_string(term_ref: SP_term_ref) -> Result<String, PrologError> {
    unsafe {
        let mut s: *const c_char = std::ptr::null_mut();
        let ret_val: c_int = SP_get_string(term_ref, &mut s as *mut *const c_char);
        if ret_val == 0 || s == 0 as *const c_char {
            return Err(PrologError::TermConversionError);
        } else {
            Ok(string_from_ref(s))
        }
    }
}

impl TryFrom<SP_term_ref> for Term {
    type Error = PrologError;
    fn try_from(term_ref: SP_term_ref) -> Result<Term, PrologError> {
        unsafe {
            if SP_is_atom(term_ref) == 1 {
                let name = sp_get_string(term_ref)?;
                return Ok(Term {
                    term_ref,
                    kind: TermKind::Atom(name),
                });
            }
            if SP_is_integer(term_ref) == 1 {
                let mut i: SP_integer = 0;
                let p: *mut SP_integer = &mut i;
                let ret_val: c_int = SP_get_integer(term_ref, p);
                if ret_val == 0 {
                    return Err(PrologError::TermConversionError);
                } else {
                    return Ok(Term {
                        term_ref,
                        kind: TermKind::Integer(i),
                    });
                }
            }
            // TODO Other Variants ...
        }
        Err(PrologError::NoTermVariantMatch)
    }
}

impl Term {}

impl PartialEq for Term {
    fn eq(&self, other: &Self) -> bool {
        let s: sys::SP_term_ref = self.term_ref;
        let o: sys::SP_term_ref = other.term_ref;
        unsafe {
            let result = SP_compare(s, o);
            result == 0
        }
    }
}
impl Eq for Term {}

pub struct Prolog {}
impl Prolog {
    pub fn new() -> Self {
        Prolog {}
    }
}
