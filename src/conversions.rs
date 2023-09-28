use crate::error::PrologError;
use crate::sp::interface::sp_get_integer;
use crate::sp::interface::sp_get_string;
use crate::sp::sys::*;
use crate::sp::terms::sp_new_term_ref;
use crate::term::Term;
use crate::term::TermKind;
use crate::util::*;

impl From<&str> for Term {
    fn from(name: &str) -> Self {
        let term_ref: SP_term_ref = sp_new_term_ref();
        let first_char: char = name
            .chars()
            .next()
            .expect("Empty string can not be converted to a valid Term.");
        if first_char.is_uppercase() || first_char == '_' {
            assert!(
                is_valid_variable_name(name),
                "Expected a variable because of first char but got invalid variable name: {}",
                name
            );
            Term {
                term_ref,
                kind: TermKind::Var(name.to_string()),
            }
        } else {
            assert!(
                is_valid_atom_name(name),
                "Expected an atom because of first char but got invalid atom name: {}",
                name
            );
            Term {
                term_ref,
                kind: TermKind::Atom(name.to_string()),
            }
        }
    }
}

impl From<i64> for Term {
    fn from(i: i64) -> Self {
        let term_ref: SP_term_ref = sp_new_term_ref();
        Term {
            term_ref,
            kind: TermKind::Integer(i),
        }
    }
}

impl From<f64> for Term {
    fn from(f: f64) -> Self {
        let term_ref: SP_term_ref = sp_new_term_ref();
        Term {
            term_ref,
            kind: TermKind::Float(f),
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

impl TryFrom<SP_term_ref> for Term {
    type Error = PrologError;
    fn try_from(term_ref: SP_term_ref) -> Result<Term, PrologError> {
        unsafe {
            if SP_is_atom(term_ref) == 1 {
                let name = sp_get_string(term_ref)?;
                return Ok(Term {
                    term_ref,
                    kind: TermKind::Atom(name),
                });
            }
            if SP_is_integer(term_ref) == 1 {
                let i = sp_get_integer(term_ref)?;
                return Ok(Term {
                    term_ref,
                    kind: TermKind::Integer(i),
                });
            }
            // TODO Other Variants ...
        }
        Err(PrologError::NoTermVariantMatch)
    }
}

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
