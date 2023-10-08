use crate::sys::{sp_new_term_ref, sp_put_variable, SP_term_ref};

use super::Term;

#[derive(Debug)]
pub struct Var {
    term_ref: SP_term_ref,
}

impl Var {
    pub fn new() -> Self {
        let term_ref = sp_new_term_ref();
        sp_put_variable(term_ref).unwrap();
        Var { term_ref }
    }
}

impl Term for Var {
    fn from(term_ref: SP_term_ref) -> Self {
        Var { term_ref }
    }

    fn into(self) -> SP_term_ref {
        self.term_ref
    }
}
