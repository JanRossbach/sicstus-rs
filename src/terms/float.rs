use core::marker::PhantomData;

use crate::sys::*;
use super::Term;


#[derive(Debug)]
pub struct Float;

impl From<f64> for Term<Float> {
    fn from(f: f64) -> Self {
        let term_ref: SP_term_ref = sp_new_term_ref();
        sp_put_float(term_ref, f).unwrap();
        Term {
            term_ref,
            kind: PhantomData::<Float>,
        }
    }
}
