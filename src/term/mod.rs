use std::cmp::Ordering;
use std::marker::PhantomData;

use crate::error::PrologError;
use crate::sp::interface::sp_get_string;
use crate::sp::terms::{sp_compare, sp_new_term_ref, sp_put_float, sp_put_variable, sp_term_type};
use crate::sp::*;
use crate::util::{is_valid_atom_name, is_valid_variable_name};

/// A Term that represents a Free Term Ref that has not been assigned yet.
pub struct Free;
pub struct Atom;
pub struct Var;
pub struct Integer;
pub struct Float;
pub struct Compound;

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

    pub fn to_variable(self) -> Result<Term<Var>, PrologError> {
        sp_put_variable(self.term_ref)?;
        Ok(Term {
            term_ref: self.term_ref,
            kind: PhantomData::<Var>,
        })
    }
}

#[test]
fn term_test() {
    let t = Term::new();
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

// #[cfg(test)]
// #[test]
// fn test_term_eq() {
//     use sicstus_sys::mock_ffi::{SP_compare_context, SP_new_term_ref_context};

//     let _lock = crate::test_utils::get_lock();
//     let ctx = SP_new_term_ref_context();
//     ctx.expect().returning(SP_term_ref::default);

//     let ctx1 = SP_compare_context();
//     ctx1.expect().return_const(0);
//     let t1 = Term::new(TermKind::Atom("a".to_string()));
//     let t2 = Term::new(TermKind::Atom("a".to_string()));
//     assert_eq!(t1, t2);
// }

impl<Kind> PartialOrd for Term<Kind> {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        let s: SP_term_ref = self.term_ref;
        let o: SP_term_ref = other.term_ref;
        Some(sp_compare(s, o))
    }
}

impl<Kind> Ord for Term<Kind> {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        let r = self.partial_cmp(other);
        match r {
            Some(o) => o,
            None => panic!("Unexpected return value from sp_compare: None"),
        }
    }
}

impl From<&str> for Term<Atom> {
    fn from(name: &str) -> Self {
        assert!(
            is_valid_atom_name(name),
            "{} is not a valid atom name",
            name
        );
        let term_ref: SP_term_ref = sp_new_term_ref();
        Term {
            term_ref,
            kind: PhantomData::<Atom>,
        }
    }
}

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

impl From<i64> for Term<Integer> {
    fn from(i: i64) -> Self {
        let term_ref: SP_term_ref = sp_new_term_ref();
        Term {
            term_ref,
            kind: PhantomData::<Integer>,
        }
    }
}

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

// #[cfg(test)]
// #[test]
// fn test_term_from() {
//     // Sometimes fails after rebuild because of some mockall issue...
//     let _mutex_guard = crate::test_utils::get_lock();
//     let term_ref_ctx = SP_new_term_ref_context();
//     term_ref_ctx.expect().returning(SP_term_ref::default);

//     let atom_term: Term = "test".into();
//     let int_term: Term = 42.into();
//     let float_term: Term = 3.14.into();
//     let var_term: Term = "X".into();
//     let var_term2: Term = "_HELLO".into();

//     assert_eq!(atom_term.kind, TermKind::Atom("test".to_string()));
//     assert_eq!(int_term.kind, TermKind::Integer(42));
//     assert_eq!(float_term.kind, TermKind::Float(3.14));
//     assert_eq!(var_term.kind, TermKind::Var("X".to_string()));
//     assert_eq!(var_term2.kind, TermKind::Var("_HELLO".to_string()));
// }

// impl TryFrom<SP_term_ref> for Term {
//     type Error = PrologError;
//     fn try_from(term_ref: SP_term_ref) -> Result<Term, PrologError> {
//         let term_type: i32 = sp_term_type(term_ref)?;
//         match term_type
//             .try_into() // converting i32 to u32 might Fail but usually shouldn't
//             .map_err(|_| PrologError::TermConversionError)?
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
//             _ => Err(PrologError::TermConversionError),
//         }
//     }
// }

// #[cfg(test)]
// #[test]
// fn test_term_try_from() {
//     let _mutex_guard = crate::test_utils::get_lock();
//     use std::ffi::c_char;

//     let term_ref_ctx = SP_new_term_ref_context();
//     term_ref_ctx.expect().returning(SP_term_ref::default);

//     let is_atom_ctx = SP_is_atom_context();
//     is_atom_ctx.expect().return_const(1);

//     let get_string_ctx = SP_get_string_context();
//     get_string_ctx.expect().returning(|_, pointer| {
//         unsafe {
//             *pointer = crate::test_utils::TEST_ATOM_NAME_STR.as_ptr() as *const c_char;
//         }
//         1
//     });

//     // Act
//     let term: Term = Term::try_from("test").unwrap();
//     let term_ref = term.to_term_ref();
//     let t = Term::try_from(term_ref).unwrap();

//     // Assert
//     assert_eq!(t.kind, TermKind::Atom("test".to_string()));
// }
