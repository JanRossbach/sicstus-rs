//! This module contains a light wrapper around the low level FFI bindings for SICStus Prolog.
//! It is intended to be used by a more high level API, but can be used directly if you need.
//! The functions in this module are responsible for ensuring the safety requirements and doing the necessary type conversions.
//! The error Handling has been converted to something more Rust idiomatic, e.g. Result types instead of returning Error codes.
//! The direct and unsafe C bindings are in the [sys] submodule.
//! The other modules contain the wrapper functions.
//! They are organized according to the Topical List of C Functions in the Prolog Manual <https://sicstus.sics.se/sicstus/docs/latest4/pdf/sicstus.pdf#cpg-top>

pub mod atom;
pub mod errors;
pub mod interface;
pub mod io;
pub mod memory;
pub mod streams;
pub mod terms;

/// Re export of the sicstus_sys crate low level bindings for SICStus Prolog.
/// For when you need fine grained control over the C API.
pub mod sys {
    #[cfg(test)]
    pub use sicstus_sys::mock_ffi::*;

    #[cfg(not(test))]
    pub use sicstus_sys::ffi::*;

    pub use sicstus_sys::variadic::*;
    pub use sicstus_sys::*;
}
