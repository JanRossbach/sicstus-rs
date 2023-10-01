use crate::sys::PrologError;

#[derive(Debug)]
pub enum SrsError {
    /// An error that occurred in the SICStus Prolog C API.
    PlError(PrologError),
}
