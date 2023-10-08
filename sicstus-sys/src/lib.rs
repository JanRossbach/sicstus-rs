#![no_std]
#![allow(non_upper_case_globals)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
///! This crate provides documented, low level bindings to the SICStus Prolog C API.
///! This provides a way to call SICStus Prolog from Rust and the other way around.
///! The SICStus C API is documented here: <https://sicstus.sics.se/sicstus/docs/latest4/html/sicstus.html/Mixing-C-and-Prolog.html#Mixing-C-and-Prolog>
///! By trouble building check if you have installed the SICStus Prolog and the Environment variable SP_PATH is set to the right directory (see README).
///! The API functions are curretnly only partyly documented.
///! The full SICStus C API documentation can be found in the manual <https://sicstus.sics.se/sicstus/docs/latest4/pdf/sicstus.pdf>, starting at page 1275.

/// This module contains the bindings generated bindgen <https://github.com/rust-lang/rust-bindgen> on the SICStus installation header files starting from ${SP_Home}/include/sicstus/sicstus.h
#[allow(dead_code)]
mod bindings {
    include!(concat!(env!("OUT_DIR"), "/bindings.rs"));
}

use core::ffi::c_char;
use core::ffi::c_int;
use core::ffi::c_uchar;
use core::ffi::c_void;

use bindings::SP_MainFun;
use bindings::SP_get_dispatch_40800;
use bindings::DISPATCH_TABLE_STRUCT_SICSTUS_H;
use bindings::SP_GLUE_INITIALIZE_OPTION_RESTORE;
pub use bindings::{
    spio_t_bits, spio_t_error_code, spio_t_offset, spio_t_simple_device_close,
    spio_t_simple_device_flush_output, spio_t_simple_device_interrupt, spio_t_simple_device_ioctl,
    spio_t_simple_device_read, spio_t_simple_device_seek, spio_t_simple_device_write, spio_t_uint8,
    spio_t_wchar, SP_CPredFun, SP_EventFun, SP_SigFun, SP_UserStreamHook, SP_UserStreamPostHook,
    SP_atom, SP_get_dispatch_type, SP_integer, SP_mutex, SP_options, SP_pred_ref, SP_qid,
    SP_stream, SP_term_ref, SICSTUS_API_STRUCT, SP_ERROR, SP_FAILURE, SP_SUCCESS, SP_TYPE_ATOM,
    SP_TYPE_COMPOUND, SP_TYPE_ERROR, SP_TYPE_FLOAT, SP_TYPE_INTEGER, SP_TYPE_VARIABLE,
};

#[macro_use]
extern crate lazy_static;

lazy_static! {
    static ref SICSTUS: Sicstus = Sicstus::new();
}

// We only ever read the pointers in the dispatch table, so it is safe to share it between threads.
unsafe impl Send for Sicstus {}
unsafe impl Sync for Sicstus {}

#[derive(Debug)]
pub struct Sicstus {
    _sicstus: *mut SICSTUS_API_STRUCT,
    pub dt: DISPATCH_TABLE_STRUCT_SICSTUS_H,
}

impl Sicstus {
    unsafe fn get_stash(&self) -> *mut c_void {
        (*(self._sicstus)).stash
    }
}

impl Sicstus {
    fn new() -> Self {
        unsafe {
            let sicstus: *mut SICSTUS_API_STRUCT = SP_get_dispatch_40800(core::ptr::null_mut());
            let dt = (*sicstus).dispatch_API_SICSTUS_H;
            let dt = *dt;
            let initialized = dt.psp_prolog_initialized.unwrap()();
            assert!(initialized != 0, "SICStus Prolog runtime not initialized!");
            Sicstus {
                _sicstus: sicstus,
                dt,
            }
        }
    }
}

/// We use a macro to generate functions that call the dispatch table. This makes for a slightly nicer API similar to that of C.
/// The macro makes it easier to maintain the implementations in case of changes.
macro_rules! define_dispatch_fns {
    (
        $(
        #[field_name=($field_name:ident)] //  This is the name of DISPATCH_TABLE_STRUCT_SICSTUS_H struct from the bindings
            $name:ident ( $( $arg_name:ident : $arg_ty:ty ),* $(,)? )
            $( -> $ret_ty:ty )?
        ),+ $(,)? // This allows a trailing comma

    ) => {
        $(
            pub unsafe fn $name($($arg_name: $arg_ty),*) $(->$ret_ty)? {
                SICSTUS.dt.$field_name.expect("Crashed getting function from dipatch table")($($arg_name),*)
            }
        )+
    }
}

define_dispatch_fns! {
    #[field_name=(pSP_atom_from_string)]
    SP_atom_from_string(str: *const c_char) -> SP_atom,
    #[field_name=(pSP_atom_length)]
    SP_atom_length(atom: SP_atom) -> usize,
    #[field_name=(pSP_calloc)]
    SP_calloc(nmemb: usize, size: usize) -> *mut c_void,
    #[field_name=(pSP_flush_output)]
    SP_flush_output(stream: *mut SP_stream,flush_options: spio_t_bits) -> spio_t_error_code,
    #[field_name=(pSP_fopen)]
    SP_fopen(
        pathname: *const c_char,
        reserved: *mut c_void,
        options: spio_t_bits,
        pstream: *mut *mut SP_stream
    ) -> spio_t_error_code,
    #[field_name=(pSP_free)]
    SP_free(ptr: *mut c_void),
    #[field_name=(pSP_get_address)]
    SP_get_address(term: SP_term_ref, p: *mut *mut c_void) -> c_int,
    #[field_name=(pSP_get_arg)]
    SP_get_arg(index: c_int, term: SP_term_ref, arg: SP_term_ref) -> c_int,
    #[field_name=(pSP_get_atom)]
    SP_get_atom(term: SP_term_ref, atom: *mut SP_atom) -> c_int,
    #[field_name=(psp_get_byte_helper)]
    SP_get_byte(stream: *mut SP_stream, options: spio_t_bits) -> spio_t_error_code,
    #[field_name=(psp_get_code_helper)]
    SP_get_code(stream: *mut SP_stream, options: spio_t_bits) -> spio_t_error_code,
    #[field_name=(pSP_get_current_dir)]
    SP_get_current_dir() -> *mut c_char,
    #[field_name=(pSP_get_float)]
    SP_get_float(term: SP_term_ref, f: *mut f64) -> c_int,
    #[field_name=(pSP_get_errno)]
    SP_get_errno() -> c_int,
    #[field_name=(pSP_get_functor)]
    SP_get_functor(term: SP_term_ref, name: *mut SP_atom, arity: *mut c_int) -> c_int,
    #[field_name=(pSP_get_integer)]
    SP_get_integer(term: SP_term_ref, integer: *mut SP_integer) -> c_int,
    #[field_name=(pSP_get_integer_bytes)]
    SP_get_integer_bytes(
        term: SP_term_ref,
        buf: *mut c_void,
        pbuf_size: *mut usize,
        native: c_int,
    ) -> c_int,
    #[field_name=(pSP_get_list)]
    SP_get_list(list: SP_term_ref, head: SP_term_ref, tail: SP_term_ref) -> c_int,
    #[field_name=(pSP_get_list_codes)]
    SP_get_list_codes(term: SP_term_ref, s: *mut *const c_char) -> c_int,
    #[field_name=(pSP_get_list_n_bytes)]
    SP_get_list_n_bytes(
        term: SP_term_ref,
        tail: SP_term_ref,
        n: usize,
        w: *mut usize,
        arg5: *mut c_uchar,
    ) -> c_int,
    #[field_name=(pSP_get_list_n_codes)]
    SP_get_list_n_codes(
        term: SP_term_ref,
        tail: SP_term_ref,
        n: usize,
        w: *mut usize,
        s: *mut c_char,
    ) -> c_int,
    #[field_name=(pSP_get_number_codes)]
    SP_get_number_codes(term: SP_term_ref, s: *mut *const c_char) -> c_int,
    #[field_name=(pSP_get_stream_counts)]
    SP_get_stream_counts(
        stream: *mut SP_stream,
        ptiem_count: *mut spio_t_offset,
        pnewline_count: *mut spio_t_offset,
        pline_length: *mut spio_t_offset,
        options: spio_t_bits,
    ) -> spio_t_error_code,
    #[field_name=(pSP_get_stream_user_data)]
    SP_get_stream_user_data(
        stream: *mut SP_stream,
        user_class: *const c_void,
        puser_data: *mut *mut c_void,
    ) -> spio_t_error_code,
    #[field_name=(pSP_get_string)]
    SP_get_string(term: SP_term_ref, string: *mut *const c_char) -> c_int,
    #[field_name=(pSP_getenv)]
    SP_getenv(name: *const c_char) -> *mut c_char,
    #[field_name=(psp_prolog_initialized)]
    sp_prolog_initialized() -> c_int,
    #[field_name=(pSP_is_atom)]
    SP_is_atom(term: SP_term_ref) -> c_int,
    #[field_name=(pSP_is_atomic)]
    SP_is_atomic(term: SP_term_ref) -> c_int,
    #[field_name=(pSP_is_compound)]
    SP_is_compound(term: SP_term_ref) -> c_int,
    #[field_name=(pSP_is_float)]
    SP_is_float(term: SP_term_ref) -> c_int,
    #[field_name=(pSP_is_integer)]
    SP_is_integer(term: SP_term_ref) -> c_int,
    #[field_name=(pSP_is_list)]
    SP_is_list(term: SP_term_ref) -> c_int,
    #[field_name=(pSP_is_number)]
    SP_is_number(term: SP_term_ref) -> c_int,
    #[field_name=(pSP_is_variable)]
    SP_is_variable(term: SP_term_ref) -> c_int,
    #[field_name=(pSP_load)]
    SP_load(filename: *const c_char) -> c_int,
    #[field_name=(pSP_load_sicstus_run_time)]
    SP_load_sicstus_run_time(
        ppfuncp: *mut SP_get_dispatch_type,
        phandle: *mut *mut c_void,
    ) -> c_int,
    #[field_name=(pSP_malloc)]
    SP_malloc(size: usize) -> *mut c_void,
    #[field_name=(pSP_mutex_lock)]
    SP_mutex_lock(pmx: *mut SP_mutex) -> c_int,
    #[field_name=(pSP_mutex_unlock)]
    SP_mutex_unlock(pmx: *mut SP_mutex) -> c_int,
    #[field_name=(pSP_new_term_ref)]
    SP_new_term_ref() -> SP_term_ref,
    #[field_name=(pSP_next_solution)]
    SP_next_solution(query: SP_qid) -> c_int,
    #[field_name=(pSP_next_stream)]
    SP_next_stream(
        stream: *mut SP_stream,
        pnext: *mut *mut SP_stream,
    ) -> spio_t_error_code,
    #[field_name=(pSP_pred)]
    SP_pred(name_atom: SP_atom, arity: SP_integer, module_atom: SP_atom) -> SP_pred_ref,
    #[field_name=(pSP_predicate)]
    SP_predicate(
        arg1: *const c_char,
        arg2: SP_integer,
        arg3: *const c_char,
    ) -> SP_pred_ref,
    #[field_name=(pSP_put_address)]
    SP_put_address(term: SP_term_ref, pointer: *mut c_void) -> c_int,
    #[field_name=(pSP_put_atom)]
    SP_put_atom(term: SP_term_ref, atom: SP_atom) -> c_int,
    #[field_name=(pSP_put_bytes)]
    SP_put_bytes(
        strea: *mut SP_stream,
        codes: *const spio_t_uint8,
        byte_count: usize,
        options: spio_t_bits,
    ) -> spio_t_error_code,
    #[field_name=(pSP_put_codes)]
    SP_put_codes(
        strea: *mut SP_stream,
        codes: *const spio_t_wchar,
        code_count: usize,
        options: spio_t_bits,
    ) -> spio_t_error_code,

    #[field_name=(pSP_put_encoded_string)]
    SP_put_encoded_string(
        stream: *mut SP_stream,
        encoded_string: *const c_char,
        options: spio_t_bits,
    ) -> spio_t_error_code,

    #[field_name=(pSP_put_float)]
    SP_put_float(term: SP_term_ref, f: f64) -> c_int,
    #[field_name=(pSP_put_functor)]
    SP_put_functor(term: SP_term_ref, name: SP_atom, arity: c_int) -> c_int,
    #[field_name=(pSP_put_integer)]
    SP_put_integer(term: SP_term_ref, integer: SP_integer) -> c_int,
    #[field_name=(pSP_put_integer_bytes)]
    SP_put_integer_bytes(
        term: SP_term_ref,
        buf: *mut c_void,
        buf_size: usize,
        native: c_int,
    ) -> c_int,

    #[field_name=(pSP_put_list)]
    SP_put_list(term: SP_term_ref) -> c_int,
    #[field_name=(pSP_put_list_codes)]
    SP_put_list_codes(term: SP_term_ref, tail: SP_term_ref, s: *const c_char) -> c_int,
    #[field_name=(pSP_put_list_n_bytes)]
    SP_put_list_n_bytes(
        term: SP_term_ref,
        tail: SP_term_ref,
        n: usize,
        s: *const c_uchar,
    ) -> c_int,

    #[field_name=(pSP_put_list_n_codes)]
    SP_put_list_n_codes(
        term: SP_term_ref,
        tail: SP_term_ref,
        n: usize,
        s: *const c_char,
    ) -> c_int,

    #[field_name=(pSP_put_number_codes)]
    SP_put_number_codes(term: SP_term_ref, s: *const c_char) -> c_int,
    #[field_name=(pSP_put_string)]
    SP_put_string(term: SP_term_ref, s: *const c_char) -> c_int,
    #[field_name=(pSP_put_term)]
    SP_put_term(to: SP_term_ref, from: SP_term_ref) -> c_int,
    #[field_name=(pSP_put_variable)]
    SP_put_variable(term: SP_term_ref) -> c_int,
    #[field_name=(pSP_raise_exception)]
    SP_raise_exception(term: SP_term_ref),
    #[field_name=(pSP_read_from_string)]
    SP_read_from_string(
        t: SP_term_ref,
        string: *const c_char,
        values: *mut SP_term_ref,
    ) -> c_int,
    #[field_name=(pSP_realloc)]
    SP_realloc(ptr: *mut c_void, size: usize) -> *mut c_void,
    #[field_name=(pSP_register_atom)]
    SP_register_atom(atom: SP_atom) -> c_int,
    #[field_name=(pSP_restore)]
    SP_restore(filenmae: *const c_char) -> c_int,
    #[field_name=(pSP_set_argv)]
    SP_set_argv(argc: c_int, argv: *mut *mut c_char, options: spio_t_bits) -> c_int,
    #[field_name=(pSP_set_current_dir)]
    SP_set_current_dir(dir: *const c_char) -> c_int,
    #[field_name=(pSP_set_user_stream_hook)]
    SP_set_user_stream_hook(
        hook: SP_UserStreamHook,
        user_data: *mut c_void,
    ) -> SP_UserStreamHook,
    #[field_name=(pSP_set_user_stream_post_hook)]
    SP_set_user_stream_post_hook(
        hook: SP_UserStreamPostHook,
        user_data: *mut c_void,
    ) -> SP_UserStreamPostHook,
    #[field_name=(pSP_signal)]
    SP_signal(sig: c_int, fun: SP_SigFun, user_data: *mut c_void) -> SP_SigFun,
    #[field_name=(pSP_strdup)]
    SP_strdup(str: *const c_char) -> *mut c_char,
    #[field_name=(pSP_string_from_atom)]
    SP_string_from_atom(atom: SP_atom) -> *const c_char,
    #[field_name=(pSP_term_type)]
    SP_term_type(term: SP_term_ref) -> c_int,
    #[field_name=(pSP_unget_byte)]
    SP_unget_byte(SP_stream: *mut SP_stream, item: c_int) -> spio_t_error_code,
    #[field_name=(pSP_unget_code)]
    SP_unget_code(SP_stream: *mut SP_stream, item: c_int) -> spio_t_error_code,
    #[field_name=(pSP_unify)]
    SP_unify(term1: SP_term_ref, term2: SP_term_ref) -> c_int,
    #[field_name=(pSP_unregister_atom)]
    SP_unregister_atom(atom: SP_atom) -> c_int,
    #[field_name=(psp_glue_initialize)]
    sp_glue_initialize(
        argc: c_int,
        argv: *mut *mut c_char,
        options: *const SP_options,
        sp_pre_linkage: *mut SP_MainFun,
        sp_pre_map: *mut *mut c_char,
        spld_dsp: c_int,
        sp_glue_initialize_option_restore: c_int,
    ) -> c_int,
    #[field_name=(pSP_close_query)]
     SP_close_query(query: SP_qid) -> c_int,
    #[field_name=(pSP_compare)]
     SP_compare(x: SP_term_ref, y: SP_term_ref) -> c_int,
    #[field_name=(pSP_cons_functor_array)]
     SP_cons_functor_array(
        term: SP_term_ref,
        name: SP_atom,
        arity: c_int,
        arg: *mut SP_term_ref,
    ) -> c_int,
    #[field_name=(pSP_cons_list)]
    SP_cons_list(term: SP_term_ref, head: SP_term_ref, tail: SP_term_ref) -> c_int,
    #[field_name=(pSP_create_stream)]
     SP_create_stream(
        user_data: *mut c_void,
        user_class: *const c_void,
        user_read: spio_t_simple_device_read,
        user_write: spio_t_simple_device_write,
        user_flush_output: spio_t_simple_device_flush_output,
        user_seek: spio_t_simple_device_seek,
        user_close:  spio_t_simple_device_close,
        user_interrupt:  spio_t_simple_device_interrupt,
        user_ioctl: spio_t_simple_device_ioctl,
        args: *mut c_void,
        create_stream_options: spio_t_bits,
        pstream: *mut *mut SP_stream,
    ) -> spio_t_error_code,
    #[field_name=(pSP_cut_query)]
     SP_cut_query(query: SP_qid) -> c_int,
    #[field_name=(pSP_define_c_predicate)]
     SP_define_c_predicate(
        name: *const c_char,
        arity: c_int,
        module: *const c_char,
        proc: SP_CPredFun,
        stash: *mut c_void,
    ) -> c_int,
    #[field_name=(pSP_deinitialize)]
     SP_deinitialize(),
    #[field_name=(pSP_error_message)]
     SP_error_message(errnum: c_int) -> *const c_char,
    #[field_name=(pSP_event)]
     SP_event(func: SP_EventFun, arg: *mut c_void) -> c_int,
    #[field_name=(pSP_exception_term)]
     SP_exception_term(term: SP_term_ref) -> c_int,
    #[field_name=(pSP_expand_file_name)]
     SP_expand_file_name(
        relpath: *const c_char,
        cwd: *mut c_char,
        options: spio_t_bits,
        pabspath: *mut *mut c_char,
    ) -> c_int,
    #[field_name=(pSP_fail)]
     SP_fail(),
    #[field_name=(pSP_fclose)]
    SP_fclose(stream: *mut SP_stream, close_options: spio_t_bits) -> spio_t_error_code,
}

pub fn sicstus() -> &'static Sicstus {
    &SICSTUS
}

/// Get a raw void pointer to the SICStus foreign stash.
pub unsafe fn SP_foreign_stash() -> *mut c_void {
    sicstus().get_stash()
}

// The C variadic functions become macros in Rust.

#[macro_export]
macro_rules! SP_cons_functor {
    ($term:expr,$atom:expr,$arity:expr,$($arg:expr),*) => {
        unsafe {
            $crate::sicstus().dt.pSP_cons_functor($term,$atom,$arity,$($arg),*)
        }
    }
}

#[macro_export]
macro_rules! SP_fprintf {
    ($stream:expr,$fmt:expr,$($arg:expr),*) => {
        unsafe {
            $crate::sicstus().dt.pSP_fprintf($stream,$fmt,$($arg),*)
        }
    }
}

#[macro_export]
macro_rules! SP_open_query {
    ($predicate:expr,$($arg:expr),*) => {
        unsafe {
            $crate::sicstus().dt.pSP_open_query($predicate,$($arg),*)
        }
    }
}

#[macro_export]
macro_rules! SP_printf {
    ($fmt:expr,$($arg:expr),*) => {
        unsafe {
            $crate::sicstus().dt.pSP_printf($fmt,$($arg),*)
        }
    }
}

#[macro_export]
macro_rules! SP_query {
    ($predicate:expr,$($arg:expr),*) => {
        unsafe {
            $crate::sicstus().dt.pSP_query($predicate,$($arg),*)
        }
    }
}

#[macro_export]
macro_rules! SP_query_cut_fail {
    ($predicate:expr,$($arg:expr),*) => {
        unsafe {
            $crate::sicstus().dt.pSP_query_cut_fail($predicate,$($arg),*)
        }
    }
}

// TODO Maybe make this work?
/// In C This function starts the SICStus Prolog runtime. In rust it does not export properly. Do not use.
pub fn SP_initialize(argc: c_int, argv: *mut *mut c_char, options: *const SP_options) -> c_int {
    unsafe {
        sp_glue_initialize(
            argc,
            argv,
            options,
            core::ptr::null_mut(),
            core::ptr::null_mut(),
            0,
            SP_GLUE_INITIALIZE_OPTION_RESTORE as c_int,
        )
    }
}

// It is recommended to use the sicstus memory management functions instead of the Rust ones in order to
// avoid memory fragmentation. In order to use the Rust allocator you can disable the alloc feature.

#[cfg(feature = "alloc")]
use core::alloc::{GlobalAlloc, Layout};

#[cfg(feature = "alloc")]
pub struct SICStusAllocator;

#[cfg(feature = "alloc")]
unsafe impl GlobalAlloc for SICStusAllocator {
    unsafe fn alloc(&self, layout: Layout) -> *mut u8 {
        SP_malloc(layout.size()) as *mut u8
    }

    unsafe fn dealloc(&self, ptr: *mut u8, _layout: Layout) {
        SP_free(ptr as *mut c_void)
    }

    unsafe fn realloc(&self, ptr: *mut u8, _layout: Layout, new_size: usize) -> *mut u8 {
        SP_realloc(ptr as *mut c_void, new_size) as *mut u8
    }
}
