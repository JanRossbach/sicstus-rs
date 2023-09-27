//! This module contains a light wrapper around the low level FFI bindings for SICStus Prolog.
//! It is intended to be used by a more high level API, but can be used directly if you need.
//! The functions in this module are responsible for ensuring the safety requirements and doing the necessary type conversions.
//! The error Handling has been converted to something more Rust idiomatic, e.g. Result types instead of returning Error codes.
//! The direct and unsafe C bindings are in the [sys] submodule.
//! The other modules contain the wrapper functions.
//! They are organized according to the Topical List of C Functions in the Prolog Manual <https://sicstus.sics.se/sicstus/docs/latest4/pdf/sicstus.pdf#cpg-top>

pub mod errors;
pub mod interface;
pub mod io;
pub mod memory;
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

pub use sys::{
    spio_t_bits, spio_t_error_code, spio_t_offset, spio_t_simple_device_close,
    spio_t_simple_device_flush_output, spio_t_simple_device_interrupt, spio_t_simple_device_ioctl,
    spio_t_simple_device_read, spio_t_simple_device_seek, spio_t_simple_device_write, spio_t_uint8,
    spio_t_wchar, SP_CPredFun, SP_SigFun, SP_UserStreamHook, SP_UserStreamPostHook, SP_atom,
    SP_get_dispatch_type, SP_integer, SP_mutex, SP_options, SP_pred_ref, SP_qid, SP_stream,
    SP_term_ref, SICSTUS_API_STRUCT, SP_ERROR, SP_SUCCESS,
};
