use core::marker::PhantomData;

use crate::sys::*;

use super::Term;

#[derive(Debug)]
pub struct Integer;

impl From<i64> for Term<Integer> {
    fn from(i: i64) -> Self {
        let term_ref: SP_term_ref = sp_new_term_ref();
        sp_put_integer(term_ref, i).unwrap();
        Term {
            term_ref,
            kind: PhantomData::<Integer>,
        }
    }
}
