#![allow(non_upper_case_globals)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
///! This crate provides fully documented, low level bindings to the SICStus Prolog C API.
///! This provides a way to call SICStus Prolog from Rust and the other way around.
///! The SICStus C API is documented here: https://sicstus.sics.se/sicstus/docs/latest4/html/sicstus.html/Mixing-C-and-Prolog.html#Mixing-C-and-Prolog
///! By trouble building check if you have installed the SICStus Prolog and the Environment variable SP_PATH is set to the right directory (see README).

/// This module contains The bindings from bindgen https://github.com/rust-lang/rust-bindgen
/// If the SICSTus header files change, or the API is somehow different on your system, you might find appropriate bindings in this module.
/// We don't use these bindings here in general because most of them are irrellevant and do not contain documentation.
pub mod bindings {
    include!(concat!(env!("OUT_DIR"), "/bindings.rs"));
}

use core::ffi::c_char;
use core::ffi::c_int;
use core::ffi::c_long;
use core::ffi::c_ulong;
use core::ffi::c_void;

/// Return value signifying success.
pub const SP_SUCCESS: u32 = 1;
/// Return value signifying failure.
pub const SP_FAILURE: u32 = 0;
/// Return value signifying an error.
pub const SP_ERROR: i32 = -1;

/// A prolog term.
pub type SP_term = c_ulong;
/// A Prolog atom.
pub type SP_atom = c_ulong;
/// An unsigned integer type that can hold a pointer.
pub type SP_uinteger = c_ulong;
/// A signed integer type that can hold a pointer.
pub type SP_integer = c_long;
/// The ID of a Prolog query. Can be optained by [SP_open_query].
pub type SP_qid = c_long;

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct SP_pred_ref_ {
    _unused: [u8; 0],
}
/// A pointer to a Prolog predicate.
pub type SP_pred_ref = *mut SP_pred_ref_;
/// A pointer to a Prolog term.
pub type SP_term_ref = c_int;

/// The Error Code of a SICStus Prolog IO Function.
pub type spio_t_error_code = c_int;

// The SICStus C API functions with proper documentation according to the manual https://sicstus.sics.se/sicstus/docs/latest4/pdf/sicstus.pdf, starting at page 1275.
extern "C" {
    /// Finds the Prolog atom whose characters are encoded by str.
    ///
    /// # Arguments
    ///
    /// * `str` - The characters comprising the atom.
    ///
    /// # Returns
    /// The SP_atom, if str is a valid internal character encoding, and 0 otherwise.
    ///
    /// See also: https://sicstus.sics.se/sicstus/docs/latest4/pdf/sicstus.pdf#Creating%20and%20Manipulating%20SP_term_refs
    pub fn SP_atom_from_string(str: *const c_char) -> SP_atom;

    /// Obtains the length of the encoded string representing a Prolog atom.
    ///
    /// # Arguments
    ///
    /// * atom - The atom to inspect.
    ///
    /// # Returns
    ///
    /// The length if the atom is valid, and 0 otherwise.
    ///
    /// # Description
    ///
    /// Same as strlen(SP_string_from_atom(a)), but runs in O(1) time.
    ///
    /// See also: https://sicstus.sics.se/sicstus/docs/latest4/pdf/sicstus.pdf#Creating%20and%20Manipulating%20SP_term_refs
    pub fn SP_atom_length(atom: SP_atom) -> usize;

    /// Allocates a block of at least size * nmemb. The first size * nmemb bytes are set to zero.
    ///
    /// # Arguments
    ///
    /// * nmemb - The number of elements to allocate.
    /// * size - The size of each element.
    ///
    /// # Returns
    ///
    /// The pointer, if allocation was successful, and NULL otherwise.
    /// See also: https://sicstus.sics.se/sicstus/docs/latest4/pdf/sicstus.pdf#OS%20Memory%20Management
    pub fn SP_calloc(nmemb: usize, size: usize) -> *mut c_void;

    /// Discard the current solution to the given query, and close it.
    ///
    /// # Arguments
    /// * query - The query, created by [SP_open_query].
    ///
    /// # Returns
    ///
    /// SP_SUCCESS if successful, and SP_ERROR if an error condition occurred.
    ///
    /// # Description
    /// This will discard the choices created since the corresponding [SP_open_query], and then
    /// backtrack into the query, throwing away any current solution, like the goal !, fail. The
    /// given argument does not have to be the innermost open query; any open queries in its scope will also be closed.
    /// See also: https://sicstus.sics.se/sicstus/docs/latest4/pdf/sicstus.pdf#Finding%20Multiple%20Solutions%20of%20a%20Call
    pub fn SP_close_query(query: SP_qid) -> c_int;

    /// Compares two terms.
    ///
    /// # Arguments
    /// * x - The first term to compare.
    /// * y - The second term to compare.
    ///
    /// # Returns
    ///
    /// -1 if x < y, zero if x = y, and 1 if x > y.
    /// See also: https://sicstus.sics.se/sicstus/docs/latest4/pdf/sicstus.pdf#ref-lte-cte
    pub fn SP_compare(x: SP_term_ref, y: SP_term_ref) -> c_int;

    /// Assigns to term a reference to a compound term whose arguments are the values of arg. ...
    /// If arity is 0, assigns the Prolog atom whose canonical representation is name. This is
    /// similar to calling =../2 with the first argument unbound and the second argument bound.
    ///
    /// # Arguments
    /// * term - The [SP_term_ref] to be assigned.
    /// * name - The name of the functor.
    /// * arity - The arity of the functor.
    /// * arg ... - The arguments of the compound term.
    ///
    /// # Returns
    ///
    /// Zero if the conversion fails (as far as failure can be detected), and a nonzero value otherwise.
    /// See also: https://sicstus.sics.se/sicstus/docs/latest4/pdf/sicstus.pdf#Creating%20Prolog%20Terms
    pub fn SP_cons_functor(
        term: SP_term_ref,
        name: SP_atom,
        arity: c_int,
        arg: SP_term_ref,
        ...
    ) -> c_int;

    /// Assigns to term a reference to a compound term whose arguments are the values of arg.
    /// If arity is 0, assigns the Prolog atom whose canonical representation is name. This is
    /// similar to calling =../2 with the first argument unbound and the second argument bound.
    ///
    /// # Arguments
    /// * term - The [SP_term_ref] to be assigned.
    /// * name - The name of the functor.
    /// * arity - The arity of the functor.
    /// * arg - The argument array.
    ///
    /// # Returns
    ///
    /// Zero if the conversion fails (as far as failure can be detected), and a nonzero value otherwise.
    /// See also: https://sicstus.sics.se/sicstus/docs/latest4/pdf/sicstus.pdf#Creating%20Prolog%20Terms
    pub fn SP_cons_functor_array(
        term: SP_term_ref,
        name: SP_atom,
        arity: c_int,
        arg: *mut SP_term_ref,
    ) -> c_int;

    /// Assigns to term a reference to a Prolog list whose head and tail are the values of head and tail.
    ///
    /// # Arguments
    /// * term - The [SP_term_ref] to be assigned.
    /// * head - The head of the new list.
    /// * tail - The tail of the new list.
    ///
    /// # Returns
    /// Zero if the conversion fails (as far as failure can be detected), and a nonzero value otherwise.
    /// See also: https://sicstus.sics.se/sicstus/docs/latest4/pdf/sicstus.pdf#Creating%20Prolog%20Terms
    pub fn SP_cons_list(term: SP_term_ref, head: SP_term_ref, tail: SP_term_ref) -> c_int;

    /// Create a Prolog stream that will call user defined functions to perform stream operations.
    ///
    /// # Arguments
    /// * user_data - This is a pointer to arbitrary user specified data. It is passed to all user defined stream methods. It must not be NULL.
    /// * user_class - Arbitrary pointer. This is used with [SP_get_stream_user_data], which see.
    /// * user_read - If non-NULL then this is an input stream.
    /// * user_write - If non-NULL then this is an output stream.
    /// * user_flush_output - Will be called to flush output on the stream. Ignored if user_write is NULL.
    ///   Can be NULL if the stream need not be flushed, e.g. if user_write always ensures that any output reaches its destination immediately.
    /// * user_seek - Reserved, should be NULL.
    /// * user_close - Closes the stream. See Section 12.3.104 [cpg-ref-user close], page 1394, for details.
    /// * user_interrupt - Reserved, should be NULL.
    /// * user_ioctl - Reserved, should be NULL.
    /// * create_stream_options - Reserved, should be 0.
    ///
    /// ## The following bits can be set:
    /// * SP_CREATE_STREAM_OPTION_BINARY
    ///    This is a binary stream. The user_read and user_write methods
    ///    transfer bytes.
    /// * SP_CREATE_STREAM_OPTION_TEXT
    ///    This is a TEXT stream. The user_read and user_write methods
    ///    transfer wide characters.
    /// * SP_CREATE_STREAM_OPTION_AUTOFLUSH
    ///    After writing to this stream prolog predicates will do a flush_
    ///    output/1. In essence this ensures that the stream behaves as if it
    ///    were unbuffered.
    /// * SP_CREATE_STREAM_OPTION_INTERACTIVE
    ///    Treat this stream as an interactive stream. Implies SP_CREATE_
    /// * STREAM_OPTION_AUTOFLUSH.
    /// * SP_CREATE_STREAM_OPTION_EOF_ON_EOF
    /// * SP_CREATE_STREAM_OPTION_RESET_ON_EOF
    ///    These correspond to the open/4 options eof_action(eof) and
    ///    eof_action(reset) respectively. The default is to give an error if
    ///    reading after reaching end of file.
    ///    Exactly one of SP_CREATE_STREAM_OPTION_BINARY and SP_CREATE_STREAM_
    ///    OPTION_TEXT must be set.
    /// * pstream - The new stream is returned here.
    ///
    /// # Returns
    ///
    /// On success, *pstream is assigned, and [SPIO_S_NOERR] or some other success code is returned.
    /// You should use the [SPIO_FAILED()] macro to determine if the return value signifies failure
    /// or success.
    /// See also: https://sicstus.sics.se/sicstus/docs/latest4/pdf/sicstus.pdf#Defining%20a%20New%20Stream
    pub fn SP_create_stream(
        user_data: *mut c_void,
        user_class: *const c_void,
        user_read: *mut bindings::spio_t_simple_device_read,
        user_write: *mut bindings::spio_t_simple_device_write,
        user_flush_output: *mut bindings::spio_t_simple_device_flush_output,
        user_seek: *mut bindings::spio_t_simple_device_seek,
        user_close: *mut bindings::spio_t_simple_device_close,
        user_interrupt: *mut bindings::spio_t_simple_device_interrupt,
        user_ioctl: *mut bindings::spio_t_simple_device_ioctl,
        create_stream_options: bindings::spio_t_bits,
        pstream: *mut *mut bindings::SP_stream,
    ) -> bindings::spio_t_error_code;

    /// Commit to the current solution to the given query, and close it.
    ///
    /// # Arguments
    /// * query - The query, created by [SP_open_query].
    ///
    /// # Return Value
    /// [SP_SUCCESS] for success, [SP_FAILURE] for failure, [SP_ERROR] if an error condition occurred.
    /// See also: https://sicstus.sics.se/sicstus/docs/latest4/pdf/sicstus.pdf#Finding%20Multiple%20Solutions%20of%20a%20Call
    pub fn SP_cut_query(query: SP_qid) -> c_int;

    /// Defines a Prolog predicate such that when the Prolog predicate is called it will call a
    /// C function with a term corresponding to the Prolog goal.
    ///
    /// # Arguments
    ///
    /// * name - The predicate name.
    /// * arity - The predicate arity.
    /// * module - The predicate module name.
    /// * proc - The C function to call.
    /// * stash - See below.
    ///
    /// # Return Value
    /// Nonzero on success, and 0 otherwise.
    ///
    /// # Description
    ///  The Prolog predicate module:name/arity will be defined (the module module must already exist).
    ///  The stash argument can be anything and is simply passed as the second argument to the C function proc.
    ///  The C function should return SP_SUCCESS for success and SP_FAILURE for failure. The C
    ///  function may also call SP_fail() or SP_raise_exception() in which case the return value will be ignored.
    pub fn SP_define_c_predicate(
        name: *mut c_char,
        arity: c_int,
        module: *mut c_char,
        proc: *mut bindings::SP_CPredFun,
        stash: *mut c_void,
    ) -> c_int;

    /// Shuts down the Prolog engine.
    ///
    /// # Description
    /// This will make a best effort to restore the system to the state it was in at the time of calling [SP_initialize].
    /// This involves unloading foreign resources, shutting down the emulator, and deallocating memory by Prolog.
    /// If SICStus has not been initialized, this function does nothing.
    /// See also: https://sicstus.sics.se/sicstus/docs/latest4/pdf/sicstus.pdf#Initializing%20the%20Prolog%20Engine
    pub fn SP_deinitialize();

    pub fn SP_fail();
    pub fn SP_get_list(list: SP_term_ref, head: SP_term_ref, tail: SP_term_ref) -> c_int;
    pub fn SP_new_term_ref() -> SP_term_ref;
    pub fn SP_get_integer(term: SP_term_ref, integer: *mut SP_integer) -> c_int;
    pub fn SP_get_arg(index: c_int, term: SP_term_ref, arg: SP_term_ref) -> c_int;
    pub fn SP_is_compound(term: SP_term_ref) -> c_int;
    pub fn SP_predicate(
        arg1: *const std::os::raw::c_char,
        arg2: SP_integer,
        arg3: *const ::std::os::raw::c_char,
    ) -> SP_pred_ref;
}
