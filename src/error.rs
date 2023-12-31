use alloc::string::String;

use crate::sys::PrologError;

#[derive(Debug)]
pub enum SicstusRsError {
    /// An error that occurred in the SICStus Prolog C API.
    InternalError(PrologError),
    InitializationError(String),
    AtomNotFound(PrologError),
    InvalidName(String),
}
