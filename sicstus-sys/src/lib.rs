// Very light wrapper around the SICStus C API bindings generated with bindgen.
// The main purpose of this crate is to provide a way to call SICStus Prolog from Rust.
// The SICStus C API is documented here: https://sicstus.sics.se/sicstus/docs/latest4/html/sicstus.html/Mixing-C-and-Prolog.html#Mixing-C-and-Prolog
// The SICStus C API is also documented in the SICStus Prolog manual, chapter 6.
// By trouble building check if you have installed the SICStus Prolog and the Environment variable SP_PATH is set to the right directory (see README).

mod bindings;

pub use bindings::*;
use std::os::raw::c_int;

// Because of the heavy use of macros in the SICStus C API, we need to manually export several api functions.
extern "C" {
    pub fn SP_new_term_ref() -> SP_term_ref;
    pub fn SP_get_integer(term: SP_term_ref, integer: *mut SP_integer) -> c_int;
    pub fn SP_get_arg(index: c_int, term: SP_term_ref, arg: SP_term_ref) -> c_int;
    pub fn SP_is_compound(term: SP_term_ref) -> c_int;
}
