use core::cmp::Ordering;
use core::marker::PhantomData;

use crate::error::SicstusRsError;
use crate::sys::*;
use crate::util::is_valid_variable_name;

/// A Term that represents a Free Term Ref that has not been assigned yet.
#[derive(Debug)]
pub struct Free;

mod atom;
mod compound;
mod float;
mod integer;
mod variable;

pub use atom::Atom;
pub use compound::Compound;
pub use float::Float;
pub use integer::Integer;
pub use variable::Var;

#[derive(Debug)]
pub struct Term<Kind = Free> {
    pub term_ref: SP_term_ref,
    pub kind: PhantomData<Kind>, // 0 size type marker to differentiate term kinds like Atom, Integer or Compound
}

impl Term<Free> {
    pub fn new() -> Self {
        Term {
            term_ref: sp_new_term_ref(),
            kind: PhantomData::<Free>,
        }
    }

    pub fn to_variable(self) -> Result<Term<Var>, SicstusRsError> {
        sp_put_variable(self.term_ref).map_err(|e| SicstusRsError::InternalError(e))?;
        Ok(Term {
            term_ref: self.term_ref,
            kind: PhantomData::<Var>,
        })
    }
}

impl<Kind> Clone for Term<Kind> {
    fn clone(&self) -> Self {
        let term_ref = sp_new_term_ref();
        sp_put_term(term_ref, self.term_ref).unwrap();
        Term {
            term_ref,
            kind: PhantomData::<Kind>,
        }
    }
}

impl<Kind> Term<Kind> {
    pub fn to_term_ref(&self) -> SP_term_ref {
        self.term_ref
    }
}

// Comparing the Terms with SP_compare.

impl<Kind> PartialEq for Term<Kind> {
    fn eq(&self, other: &Self) -> bool {
        let s: SP_term_ref = self.term_ref;
        let o: SP_term_ref = other.term_ref;
        sp_compare(s, o) == Ordering::Equal
    }
}
impl<Kind> Eq for Term<Kind> {}

impl<Kind> PartialOrd for Term<Kind> {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        let s: SP_term_ref = self.term_ref;
        let o: SP_term_ref = other.term_ref;
        Some(sp_compare(s, o))
    }
}

impl<Kind> Ord for Term<Kind> {
    fn cmp(&self, other: &Self) -> Ordering {
        let r = self.partial_cmp(other);
        match r {
            Some(o) => o,
            None => panic!("Unexpected return value from sp_compare: None"),
        }
    }
}

// impl TryFrom<SP_term_ref> for Term {
//     type Error = SrsError;
//     fn try_from(term_ref: SP_term_ref) -> Result<Term, SrsError> {
//         let term_type: i32 = sp_term_type(term_ref)?;
//         match term_type
//             .try_into() // converting i32 to u32 might Fail but usually shouldn't
//             .map_err(|_| SrsError::TermConversionError)?
//         {
//             SP_TYPE_VARIABLE => {
//                 let name = sp_get_string(term_ref)?;
//                 Ok(Term {
//                     term_ref,
//                     kind: PhantomData::<Var>
//                 })
//             }
//             SP_TYPE_ATOM => {
//                 let name = sp_get_string(term_ref)?;
//                 Ok(Term {
//                     term_ref,
//                     kind: TermKind::Atom(name),
//                 })
//             }
//             SP_TYPE_INTEGER => {
//                 let i = sp_get_integer(term_ref)?;
//                 Ok(Term {
//                     term_ref,
//                     kind: TermKind::Integer(i),
//                 })
//             }
//             SP_TYPE_FLOAT => {
//                 unimplemented!()
//                 // let f = sp_get_float(term_ref)?;
//                 // return Ok(Term {
//                 //     term_ref,
//                 //     kind: TermKind::Float(f),
//                 // });
//             }
//             SP_TYPE_COMPOUND => {
//                 unimplemented!()
//             }
//             _ => Err(SrsError::TermConversionError),
//         }
//     }
// }
