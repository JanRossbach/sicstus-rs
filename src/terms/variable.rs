use core::marker::PhantomData;

use crate::util::is_valid_variable_name;
use crate::sys::*;

use super::Term;

#[derive(Debug)]
pub struct Var;

impl From<&str> for Term<Var> {
    fn from(name: &str) -> Self {
        assert!(
            is_valid_variable_name(name),
            "{} is not a valid var name",
            name
        );
        let term_ref: SP_term_ref = sp_new_term_ref();
        Term {
            term_ref,
            kind: PhantomData::<Var>,
        }
    }
}
