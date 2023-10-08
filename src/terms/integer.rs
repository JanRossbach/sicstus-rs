use crate::sys::*;

use super::Term;

#[derive(Debug)]
pub struct Integer {
    term_ref: SP_term_ref,
    integer: i64,
}

impl Integer {
    pub fn new(value: i64) -> Self {
        let term_ref = sp_new_term_ref();
        sp_put_integer(term_ref, value).unwrap();
        Integer {
            term_ref,
            integer: value,
        }
    }

    pub fn value(&self) -> i64 {
        self.integer
    }

    pub fn as_term_ref(&self) -> SP_term_ref {
        self.term_ref
    }

}

impl Term for Integer {
    fn from(term_ref: SP_term_ref) -> Self {
        let integer = sp_get_integer(term_ref).unwrap();
        Integer { term_ref, integer }
    }

    fn into(self) -> SP_term_ref {
        self.term_ref
    }
}
