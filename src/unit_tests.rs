use crate::sp::*;
use crate::term::{Term, TermKind};
use crate::sp::sys::*;

use mockall::predicate::*;

use std::ffi::CString;

use std::ffi::c_char;

static TEST_ATOM_NAME_STR: &str = "test\0";

#[test]
fn test_term_try_from() {
    // Sometimes fails after rebuild because of some mockall issue...
    // Arrange
    let term_ref_ctx = SP_new_term_ref_context();
    term_ref_ctx.expect().returning(SP_term_ref::default);

    let is_atom_ctx = SP_is_atom_context();
    is_atom_ctx.expect().with(eq(0)).return_const(1);

    let get_string_ctx = SP_get_string_context();
    get_string_ctx
        .expect()
        .with(always(), always())
        .returning(|_, pointer| {
            unsafe {
                *pointer = TEST_ATOM_NAME_STR.as_ptr() as *const c_char;
            }
            1
        });

    // Act
    let term: Term = Term::from("test");
    let term_ref = term.to_term_ref();
    let t = Term::try_from(term_ref).unwrap();

    // Assert
    assert_eq!(t.kind, TermKind::Atom("test".to_string()));
}

#[test]
fn test_sp_get_string() {
    let t: SP_term_ref = SP_term_ref::default();
    let sp_get_string_ctx = SP_get_string_context();
    sp_get_string_ctx
        .expect()
        .with(eq(t), always())
        .returning(|_, pointer| {
            unsafe {
                *pointer = TEST_ATOM_NAME_STR.as_ptr() as *const c_char;
            }
            1
        });
    let res = sp_get_string(t).unwrap();
    assert_eq!(res, "test".to_string());
}

#[test]
fn test_string_copy() {
    let s: CString = CString::new("Hello, World!").expect("CString::new failed");
    let s: *const c_char = s.as_ptr();
    unsafe {
        let pp: *const c_char = s;
        let pp = pp.add(1);
        assert_eq!('l', *(pp.add(1)) as u8 as char);
    }
    let copied_string: String = unsafe { string_from_ref(s) };
    assert_eq!(copied_string, "Hello, World!".to_string());
}
