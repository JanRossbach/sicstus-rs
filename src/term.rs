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

// Comparing the Terms with SP_compare.

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

#[cfg(test)]
#[test]
fn test_term_eq() {
    let _lock = crate::test_utils::get_lock();
    let ctx = SP_new_term_ref_context();
    ctx.expect().returning(SP_term_ref::default);

    let ctx1 = SP_compare_context();
    ctx1.expect().return_const(0);
    let t1 = Term::new(TermKind::Atom("a".to_string()));
    let t2 = Term::new(TermKind::Atom("a".to_string()));
    assert_eq!(t1, t2);
}

impl PartialOrd for Term {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        let s: sys::SP_term_ref = self.term_ref;
        let o: sys::SP_term_ref = other.term_ref;
        unsafe {
            let result: i32 = SP_compare(s, o);
            match result {
                0 => Some(std::cmp::Ordering::Equal),
                -1 => Some(std::cmp::Ordering::Less),
                1 => Some(std::cmp::Ordering::Greater),
                _ => None,
            }
        }
    }
}

impl Ord for Term {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        let r = self.partial_cmp(other);
        match r {
            Some(o) => o,
            None => panic!("Cannot compare these terms. SP_compare returned an invalid value, meaning not -1, 0, or 1."),
        }
    }
}