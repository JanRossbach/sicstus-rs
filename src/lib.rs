//! High Level API for SICStus Prolog.
#![no_std]
#![feature(error_in_core)]

extern crate alloc;

pub mod spmacro;
mod error;

pub mod sys;

pub mod terms;
mod util;
pub mod query;
pub mod stash;

#[cfg(feature="alloc")]
use sicstus_sys::SICStusAllocator;

#[cfg(feature="alloc")]
#[global_allocator]
static ALLOCATOR: SICStusAllocator = SICStusAllocator;

// #[cfg(feature="print")]
// custom_print::define_macros!({ print, println }, fmt, |value: &str| {
//     let c_str = alloc::ffi::CString::new(value).unwrap();
//     unsafe {
//         panic!("Print not implemented");
//         sicstus_sys::SP_printf(c_str.as_ptr());
//     }
// });
