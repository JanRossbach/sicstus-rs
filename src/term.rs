use crate::sp::sys::*;
use crate::sp::*;

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
