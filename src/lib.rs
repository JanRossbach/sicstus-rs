//! High Level API for SICStus Prolog.
#![no_std]
#![feature(error_in_core)]

extern crate alloc;

mod error;

pub mod sys;

mod atom;
mod term_ref;
mod util;

pub use sys::{SP_term_ref, SP_atom, SP_integer};
pub use term_ref::TermRef;

pub use error::SicstusRsError;

pub use atom::Atom;

#[cfg(feature = "alloc")]
use sicstus_sys::SICStusAllocator;

#[cfg(feature = "alloc")]
#[global_allocator]
static ALLOCATOR: SICStusAllocator = SICStusAllocator;

custom_print::define_macros!({ print, println }, fmt, |value: &str| {
    let c_str = alloc::ffi::CString::new(value).unwrap();
    unsafe {
        panic!("Print not implemented");
        sicstus_sys::SP_printf(c_str.as_ptr());
    }
});
