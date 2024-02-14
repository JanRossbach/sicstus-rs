use alloc::{format, string::String};

use crate::{
    sys::{sp_raise_exception, PrologError},
    TermRef,
};

#[derive(Debug)]
pub enum SicstusRsError {
    /// An error that occurred in the SICStus Prolog C API.
    InternalError(PrologError),
    InitializationError(String),
    AtomNotFound(PrologError),
    InvalidName(String),
    TypeError(String)
}

pub fn throw_exception(message: String) {
    let mut exception_term = TermRef::new();
    exception_term.put_string(message.as_str()).unwrap();
    sp_raise_exception(exception_term.term_ref());
}

/// A Function that takes a PrologError and propagates it to Prolog without panicking.
pub fn handle_prolog_error(error: PrologError) {
    throw_exception(format!("Prolog error: {:?}", error));
}

/// A Function that takes a SicstusRsError and propagates it to Prolog without panicking.
pub fn handle_sicstus_rs_error(error: SicstusRsError) {
    match error {
        SicstusRsError::InternalError(e) => {
            handle_prolog_error(e);
        }
        SicstusRsError::InitializationError(e) => {
            throw_exception(format!("Initialization error: {:?}", e));
        }
        SicstusRsError::AtomNotFound(e) => {
            handle_prolog_error(e);
        }
        SicstusRsError::TypeError(e) => {
            throw_exception(format!("Type error: {:?}", e));
        }
        SicstusRsError::InvalidName(e) => {
            throw_exception(format!("Invalid name error: {:?}", e));
        }
    };
}

impl From<PrologError> for SicstusRsError {
    fn from(error: PrologError) -> Self {
        SicstusRsError::InternalError(error)
    }
}
