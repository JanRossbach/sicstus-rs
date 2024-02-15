use crate::{sys::PrologError, Atom};
use alloc::string::String;
use sicstus_sys::SP_pred;
use crate::SP_pred_ref;


pub enum QueryError {
    InternalError(PrologError),
}

impl From<PrologError> for QueryError {
    fn from(error: PrologError) -> Self {
        QueryError::InternalError(error)
    }
}

pub struct Predicate {
    _pred_ref: SP_pred_ref,
}

impl Predicate {
    pub fn new(module: String, name: String, arity: usize) -> Result<Self, QueryError> {
        let module = Atom::from_string(module);
        let name = Atom::from_string(name);
        let pred_ref = unsafe { SP_pred(name.as_atom(), arity as i64, module.as_atom()) };
        if pred_ref.is_null() {
            return Err(QueryError::InternalError(PrologError::PredicateNotFound));
        }
        Ok(Predicate { _pred_ref: pred_ref })
    }
}
