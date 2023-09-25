use crate::sp::sys::*;
use crate::sp::*;
use crate::error::PrologError;

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
                let i = sp_get_integer(term_ref)?;
                return Ok(Term {
                    term_ref,
                    kind: TermKind::Integer(i),
                });
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
