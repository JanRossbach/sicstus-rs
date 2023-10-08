use super::Term;
use crate::sys::*;

#[derive(Debug)]
pub struct Float {
    term_ref: SP_term_ref,
    float: f64,
}

impl Float {
    pub fn new(value: f64) -> Self {
        let term_ref = sp_new_term_ref();
        sp_put_float(term_ref, value).unwrap();
        Float {
            term_ref,
            float: value,
        }
    }

    pub fn value(&self) -> f64 {
        self.float
    }

    pub fn as_term_ref(&self) -> SP_term_ref {
        self.term_ref
    }
}

impl Term for Float {
    fn from(term_ref: SP_term_ref) -> Self {
        let float = sp_get_float(term_ref).unwrap();
        Float { term_ref, float }
    }

    fn into(self) -> SP_term_ref {
        self.term_ref
    }
}
