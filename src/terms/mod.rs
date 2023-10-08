mod atom;
mod compound;
mod float;
mod integer;
mod variable;

pub use atom::Atom;
pub use compound::List;
pub use float::Float;
pub use integer::Integer;
pub use variable::Var;

use crate::sys::SP_term_ref;

pub trait Term {
    fn from(term_ref: SP_term_ref) -> Self;
    fn into(self) -> SP_term_ref;
}

pub trait Queryable {
    fn query(self, query: &str) -> bool;
}
