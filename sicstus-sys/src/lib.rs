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

use core::ffi::c_void;
use core::ffi::{c_char, c_int};

pub use bindings::{
    spio_t_bits, spio_t_error_code, spio_t_error_code_, spio_t_offset, spio_t_simple_device_close,
    spio_t_simple_device_flush_output, spio_t_simple_device_interrupt, spio_t_simple_device_ioctl,
    spio_t_simple_device_read, spio_t_simple_device_seek, spio_t_simple_device_write, spio_t_wchar,
    SP_CPredFun, SP_SigFun, SP_UserStreamHook, SP_UserStreamPostHook, SP_atom,
    SP_get_dispatch_type, SP_integer, SP_mutex, SP_options, SP_pred_ref, SP_qid, SP_stream,
    SP_term, SP_term_ref, SP_uinteger, SICSTUS_API_STRUCT, SP_ERROR, SP_FAILURE, SP_SUCCESS,
};

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
    /// See also: <https://sicstus.sics.se/sicstus/docs/latest4/pdf/sicstus.pdf#Creating%20and%20Manipulating%20SP_term_refs>
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
    /// See also: <https://sicstus.sics.se/sicstus/docs/latest4/pdf/sicstus.pdf#Creating%20and%20Manipulating%20SP_term_refs>
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
    /// See also: <https://sicstus.sics.se/sicstus/docs/latest4/pdf/sicstus.pdf#OS%20Memory%20Management>
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
    /// See also: <https://sicstus.sics.se/sicstus/docs/latest4/pdf/sicstus.pdf#Finding%20Multiple%20Solutions%20of%20a%20Call>
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
    /// See also: <https://sicstus.sics.se/sicstus/docs/latest4/pdf/sicstus.pdf#ref-lte-cte>
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
    /// See also: <https://sicstus.sics.se/sicstus/docs/latest4/pdf/sicstus.pdf#Creating%20Prolog%20Terms>
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
    /// See also: <https://sicstus.sics.se/sicstus/docs/latest4/pdf/sicstus.pdf#Creating%20Prolog%20Terms>
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
    /// See also: <https://sicstus.sics.se/sicstus/docs/latest4/pdf/sicstus.pdf#Creating%20Prolog%20Terms>
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
    /// See also: <https://sicstus.sics.se/sicstus/docs/latest4/pdf/sicstus.pdf#Defining%20a%20New%20Stream>
    pub fn SP_create_stream(
        user_data: *mut c_void,
        user_class: *const c_void,
        user_read: *mut spio_t_simple_device_read,
        user_write: *mut spio_t_simple_device_write,
        user_flush_output: *mut spio_t_simple_device_flush_output,
        user_seek: *mut spio_t_simple_device_seek,
        user_close: *mut spio_t_simple_device_close,
        user_interrupt: *mut spio_t_simple_device_interrupt,
        user_ioctl: *mut spio_t_simple_device_ioctl,
        create_stream_options: spio_t_bits,
        pstream: *mut *mut SP_stream,
    ) -> spio_t_error_code;

    /// Commit to the current solution to the given query, and close it.
    ///
    /// # Arguments
    /// * query - The query, created by [SP_open_query].
    ///
    /// # Return Value
    /// [SP_SUCCESS] for success, [SP_FAILURE] for failure, [SP_ERROR] if an error condition occurred.
    /// See also: <https://sicstus.sics.se/sicstus/docs/latest4/pdf/sicstus.pdf#Finding%20Multiple%20Solutions%20of%20a%20Call>
    ///
    /// # Description
    /// This will discard the choices created since the corresponding SP_open_query(), like the goal
    /// !. The current solution is retained in the arguments until backtracking into any enclosing
    /// query. The given argument does not have to be the innermost open query; any open queries
    /// in its scope will also be cut.
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
        proc: *mut SP_CPredFun,
        stash: *mut c_void,
    ) -> c_int;

    /// Shuts down the Prolog engine.
    ///
    /// # Description
    /// This will make a best effort to restore the system to the state it was in at the time of calling [SP_initialize].
    /// This involves unloading foreign resources, shutting down the emulator, and deallocating memory by Prolog.
    /// If SICStus has not been initialized, this function does nothing.
    /// See also: <https://sicstus.sics.se/sicstus/docs/latest4/pdf/sicstus.pdf#Initializing%20the%20Prolog%20Engine>
    pub fn SP_deinitialize();

    /// Obtains a pointer to the diagnostic message corresponding to a specified error number.
    ///
    /// # Arguments
    /// * errnum - The error number.
    ///
    /// # Returns
    /// A pointer to the diagnostic message corresponding to errnum.
    /// See also: <https://sicstus.sics.se/sicstus/docs/latest4/pdf/sicstus.pdf#CPL%20Notes>
    pub fn SP_error_message(errnum: c_int) -> *const c_char;

    /// Schedules a function for execution in the main thread contexts where queries cannot be issued.
    ///
    /// # Arguments
    /// * func - The function to call.
    /// * arg - The argument to pass to the function.
    ///
    /// # Returns
    /// Nonzero on success, and 0 otherwise.
    ///
    /// # Description
    /// If you wish to call Prolog back from a signal handler that has been installed with [SP_signal]
    /// or a thread other than the thread that called [SP_initialize], that is, the main thread,
    /// you cannot use [SP_query] etc. directly. The call to Prolog has to be delayed until such
    /// time that the Prolog execution can accept an interrupt and the call has to be performed
    /// from the main thread (the Prolog execution thread). This function serves this purpose, and
    /// installs func to be called from Prolog (in the main thread) when the execution can accept
    /// a callback.
    /// A queue of functions, with corresponding arguments, is maintained; that is, if several calls
    /// to [SP_event] occur before Prolog can accept an interrupt, the functions are queued and
    /// executed in turn at the next possible opportunity. A func installed with SP_event() will
    /// not be called until SICStus is actually running. One way of ensuring that all pending
    /// functions installed with [SP_event] are run is to call, from the main thread, some dummy
    /// goal, such as,
    /// ```c
    /// SP_query_cut_fail(SP_predicate("true",0,"user"));
    /// ```
    /// While SP_event() is safe to call from any thread, it is not safe to call from arbitrary signal
    /// handlers. If you want to call SP_event() when a signal is delivered, you need to install
    /// your signal handler with SP_signal().
    /// Note that SP_event() is one of the very few functions in the SICStus API that can safely
    /// be called from another thread than the main thread.
    /// Depending on the value returned from func, the interrupted Prolog execution will just
    /// continue (SP_SUCCESS) or backtrack (SP_FAILURE or SP_ERROR). An exception raised by
    /// func, using SP_raise_exception(), will be processed in the interrupted Prolog execution.
    /// If func calls SP_fail() or SP_raise_exception() the return value from func is ignored
    /// and handled as if func returned SP_FAILURE or SP_ERROR, respectively. In case of failure
    /// or exception, the event queue is flushed.
    /// It is generally not robust to let func raise an exception or (even worse) fail. The reason is
    /// that not all Prolog code is written such that it gracefully handles being interrupted. If you
    /// want to interrupt some long-running Prolog code, it is better to let the event handler set a
    /// flag (in C) and let your Prolog code test the flag (using a foreign predicate) in some part
    /// of your code that is executed repeatedly.
    /// See also: <https://sicstus.sics.se/sicstus/docs/latest4/pdf/sicstus.pdf#Calling%20Prolog%20Asynchronously>
    pub fn SP_event(func: extern "C" fn(arg1: *mut c_void) -> c_int, arg: *mut c_void) -> c_int;

    /// Retracts the current pending exception term, if it exits, and assigns it to term.
    ///
    /// # Arguments
    /// * term - The [SP_term_ref] to assign.
    ///
    /// # Returns
    /// 1 if an exception term was retracted and assigned, and 0 otherwise.
    /// See also: <https://sicstus.sics.se/sicstus/docs/latest4/pdf/sicstus.pdf#Exception%20Handling%20in%20C>
    pub fn SP_exception_term(term: SP_term_ref) -> c_int;

    /// Expand a file name into an absolute path.
    ///
    /// # Arguments
    /// * relpath - The relative path to expand. It should be an encoded string. The path is subject to syntactic rewriting, as if by absolute_file_name/2.
    /// * cwd - If the [relpath] is a relative path, it is expanded relative to cwd, unless cwd is NULL.
    ///         If cwd is NULL, a relative relpath is expanded relative to teh SICStus working directory (as returned by [SP_get_current_dir]).
    /// * options - The following option bits can be set:
    ///     SP_EXPAND_FILE_NAME_OPTION_DIR
    /// The relpath is expanded as a directory, i.e. *pabspath will be
    /// slash terminated.
    ///     SP_EXPAND_FILE_NAME_OPTION_NO_CWD
    /// An error is returned if the relpath is not an absolute path after
    /// syntactic rewriting.
    ///     SP_EXPAND_FILE_NAME_OPTION_NO_ENV
    /// Do not expand system properties and environment variables during
    /// syntactic rewriting.
    ///     SP_EXPAND_FILE_NAME_OPTION_NO_HOME
    /// Do not expand ‘~’ and ‘~user’ during syntactic rewriting.
    ///     SP_EXPAND_FILE_NAME_OPTION_ROOT_SLASH
    /// If the expanded value would refer to the root directory, return
    /// a slash terminated absolute path, as if SP_EXPAND_FILE_NAME_
    /// OPTION_DIR had been set. By default, an error is returned if the
    /// expanded absolute path would refer to a root directory and SP_EXPAND_FILE_NAME_OPTION_DIR is not set.
    ///     SP_EXPAND_FILE_NAME_OPTION_ROOT_DOT
    /// If the expanded value would refer to the root directory, return an
    /// absolute path terminated with ‘/.’. By default, an error is returned
    /// if the expanded absolute path would refer to a root directory and
    /// SP_EXPAND_FILE_NAME_OPTION_DIR is not set
    pub fn SP_expand_file_name(
        relpath: *const c_char,
        cwd: *mut c_char,
        options: spio_t_bits,
        pabspath: *mut *mut c_char,
    ) -> c_int;

    /// Fails in the scope of Prolog calling C.
    ///
    /// # Arguments
    /// * term - The [SP_term_ref] to assign.
    ///
    /// # Description
    /// This function is normally used in the context of a call from Prolog to C,
    /// and will cause Prolog to backtrack on return from the call.
    /// *Please note*: this should only be called right before returning to Prolog.
    /// See also: <https://sicstus.sics.se/sicstus/docs/latest4/pdf/sicstus.pdf#Exception%20Handling%20in%20C>
    pub fn SP_fail(term: SP_term_ref);

    /// Close the given stream.
    ///
    /// # Arguments
    /// * stream - The stream to close unless the SP_F_CLOSE_OPTION_USER_STREAMS is set, see below.
    /// * close_options - The following option bits can be set:
    ///    SP_FCLOSE_OPTION_READ
    ///    SP_FCLOSE_OPTION_WRITE
    ///        Close the specified directions. If neither of these options is specified,
    ///        the stream is closed in all opened directions, i.e. as if both options
    ///        were specified. If the stream is not opened in a direction specified
    ///        by an option, that option is ignored.
    ///        Note that it is possible to close only one direction of a bidirectional
    ///        stream. The return value will tell whether the stream is still open; see below
    ///
    /// SP_FCLOSE_OPTION_FORCE
    ///       Close the specified direction forcibly, i.e. without flushing buffers
    ///       etc. This also ensures that the close finishes quickly, i.e. does not
    ///       block.
    /// SP_FCLOSE_OPTION_NO_FSYNC
    ///       Do not use OS fclose() or similar when closing a stream in write
    ///       direction, i.e. do not wait for written data to reach the disk.
    ///       By default, closing a stream will try to ensure that all written
    ///       data have been stored on disk before the call returns. This makes
    ///       stream handling more robust, e.g. if the process crashes shortly
    ///       after closing the stream. However, waiting for data to reach the
    ///       disk is sometimes very slow (e.g. on some Linux configurations), in
    ///       which case this flag can be used to speed things up, at the cost of
    ///       somewhat reduced robustness.
    /// SP_FCLOSE_OPTION_NONBLOCKING
    ///       You should avoid using this option.
    ///       Pass non-blocking option to lower level routines, including the call
    ///       to SP_flush_output() that is issued when non-forcibly closing
    ///       write direction.
    ///       Chapter 12: C Reference Pages 1299
    ///       One possible use for this option is to perform a best effort close,
    ///       which falls back to using SP_FCLOSE_OPTION_FORCE only if ordinary
    ///       close would block.
    /// SP_FCLOSE_OPTION_USER_STREAMS
    ///       In this case the stream should not be a stream but instead
    ///       be the user_class of a user defined stream. When this op-
    ///       tion is passed, all currently opened streams of that class is
    ///       closed, using the remaining option flags. E.g. to close all
    ///       user defined streams of class my class in the read direction
    ///       only do: SP_fclose((SP_stream*)my_class,SP_FCLOSE_OPTION_
    ///       USER_STREAMS|SP_FCLOSE_OPTION_READ).
    /// # Returns
    /// On success, all specified directions have been closed. Since some direction may still be open, there are two possible return values on success:
    /// SPIO_S_NOERR: The stream is valid, some direction is not closed.
    /// SPIO_S_DEALLOCATED: The stream has been deallocated, and cannot be used further. All directions have been closed.
    /// On failure, returns an SPIO error code. Error codes with special meaning for [SP_fclose] are the same as for [SP_flush_output].
    /// Other error codes may also be returned. See also: <https://sicstus.sics.se/sicstus/docs/latest4/pdf/sicstus.pdf#cpg-ref-SP_flush_output>, <https://sicstus.sics.se/sicstus/docs/latest4/pdf/sicstus.pdf#Prolog%20Streams>
    pub fn SP_fclose(stream: *mut SP_stream, close_options: spio_t_bits) -> spio_t_error_code;

    pub fn SP_flush_output(stream: *mut SP_stream, flush_options: spio_t_bits)
        -> spio_t_error_code;

    pub fn SP_fopen(
        pathname: *const c_char,
        reserved: *mut c_void,
        options: spio_t_bits,
        pstream: *mut *mut SP_stream,
    ) -> spio_t_error_code;

    pub fn SP_foreign_stash() -> *mut c_void;
    pub fn SP_fprintf(stream: *mut SP_stream, fmt: *const c_char, ...) -> spio_t_error_code;
    pub fn SP_free(ptr: *mut c_void);
    pub fn SP_get_address(term: SP_term_ref, p: *mut *mut c_void) -> c_int;
    pub fn SP_get_arg(index: c_int, term: SP_term_ref, arg: SP_term_ref) -> c_int;
    pub fn SP_get_atom(term: SP_term_ref, atom: *mut SP_atom) -> c_int;
    pub fn SP_get_byte(stream: *mut SP_stream) -> spio_t_error_code;
    pub fn SP_get_code(stream: *mut SP_stream) -> spio_t_error_code;
    pub fn SP_get_current_dir() -> *mut c_char;
    pub fn SP_get_dispatch(reserved: *mut c_void) -> *mut SICSTUS_API_STRUCT;
    pub fn SP_get_float(term: SP_term_ref, f: *mut f64) -> c_int;
    pub fn SP_get_functor(term: SP_term_ref, name: *mut SP_atom, arity: *mut c_int) -> c_int;
    pub fn SP_get_integer(term: SP_term_ref, integer: *mut SP_integer) -> c_int;
    pub fn SP_get_integer_bytes(
        term: SP_term_ref,
        buf: *mut c_void,
        pbuf_size: *mut usize,
        native: c_int,
    ) -> c_int;
    pub fn SP_get_list(list: SP_term_ref, head: SP_term_ref, tail: SP_term_ref) -> c_int;
    pub fn SP_get_list_codes(term: SP_term_ref, s: *mut *const c_char) -> c_int;
    pub fn SP_get_list_n_bytes(
        term: SP_term_ref,
        tail: SP_term_ref,
        n: usize,
        w: *mut usize,
        s: *mut c_char,
    ) -> c_int;
    pub fn SP_get_list_n_codes(
        term: SP_term_ref,
        tail: SP_term_ref,
        n: usize,
        w: *mut usize,
        s: *mut c_char,
    ) -> c_int;
    pub fn SP_get_number_codes(term: SP_term_ref, s: *mut *const c_char) -> c_int;
    pub fn SP_get_stream_counts(
        stream: *mut SP_stream,
        ptiem_count: *mut spio_t_offset,
        pnewline_count: *mut spio_t_offset,
        pline_length: *mut spio_t_offset,
        options: spio_t_bits,
    ) -> spio_t_error_code;

    pub fn SP_get_stream_user_data(
        stream: *mut SP_stream,
        user_class: *const c_void,
        puser_data: *mut *mut c_void,
    ) -> spio_t_error_code;

    pub fn SP_get_string(term: SP_term_ref, s: *mut *const c_char) -> c_int;
    pub fn SP_getenv(name: *const c_char) -> *mut c_void;
    pub fn SP_initialize(argc: c_int, argv: *mut *mut c_char, options: *mut SP_options) -> c_int;
    pub fn SP_is_atom(term: SP_term_ref) -> c_int;
    pub fn SP_is_atomic(term: SP_term_ref) -> c_int;
    pub fn SP_is_compount(term: SP_term_ref) -> c_int;
    pub fn SP_is_float(term: SP_term_ref) -> c_int;
    pub fn SP_is_integer(term: SP_term_ref) -> c_int;
    pub fn SP_is_list(term: SP_term_ref) -> c_int;
    pub fn SP_is_number(term: SP_term_ref) -> c_int;
    pub fn SP_is_variable(term: SP_term_ref) -> c_int;
    pub fn SP_load(filename: *const c_char) -> c_int;
    pub fn SP_load_sicstus_run_time(
        pfuncp: *mut *mut SP_get_dispatch_type,
        reserved: *mut c_void,
    ) -> c_int;
    pub fn SP_malloc(size: usize) -> *mut c_void;
    pub fn SP_mutex_lock(pmx: *mut SP_mutex) -> c_int;
    pub fn SP_mutex_unlock(pmx: *mut SP_mutex) -> c_int;
    pub fn SP_new_term_ref() -> SP_term_ref;
    pub fn SP_next_solution(query: SP_qid) -> c_int;
    pub fn SP_next_stream(stream: *mut SP_stream, pnext: *mut *mut SP_stream) -> spio_t_error_code;
    pub fn SP_open_query(predicate: SP_pred_ref, arg1: SP_term_ref, ...) -> SP_qid;
    pub fn SP_pred(name_atom: SP_atom, arity: SP_integer, module_atom: SP_atom) -> SP_pred_ref;
    pub fn SP_predicate(arg1: *const c_char, arg2: SP_integer, arg3: *const c_char) -> SP_pred_ref;
    pub fn SP_printf(fmt: *const c_char, ...) -> spio_t_error_code;
    pub fn SP_put_address(term: SP_term_ref, pointer: *mut c_void) -> c_int;
    pub fn SP_put_atom(term: SP_term_ref, atom: SP_atom) -> c_int;
    pub fn SP_put_byte(stream: *mut SP_stream, item: c_int) -> spio_t_error_code;
    pub fn SP_put_code(stream: *mut SP_stream, item: c_int) -> spio_t_error_code;
    pub fn SP_put_codes(
        strea: *mut SP_stream,
        codes: *const spio_t_wchar,
        code_count: usize,
        options: spio_t_bits,
    ) -> spio_t_error_code;

    pub fn SP_put_encoded_string(
        stream: *mut SP_stream,
        encoded_string: *const spio_t_wchar,
        options: spio_t_bits,
    ) -> spio_t_error_code;

    pub fn SP_put_float(term: SP_term_ref, f: f64) -> c_int;
    pub fn SP_put_functor(term: SP_term_ref, name: SP_atom, arity: SP_integer) -> c_int;
    pub fn SP_put_integer(term: SP_term_ref, integer: SP_integer) -> c_int;
    pub fn SP_put_integer_bytes(
        term: SP_term_ref,
        buf: *const c_void,
        buf_size: usize,
        native: c_int,
    ) -> c_int;

    pub fn SP_put_list(term: SP_term_ref) -> c_int;
    pub fn SP_put_list_codes(term: SP_term_ref, tail: SP_term_ref, s: *const c_char) -> c_int;
    pub fn SP_put_list_n_bytes(
        term: SP_term_ref,
        tail: SP_term_ref,
        n: usize,
        s: *const c_char,
    ) -> c_int;

    pub fn SP_put_list_n_codes(
        term: SP_term_ref,
        tail: SP_term_ref,
        n: usize,
        s: *const c_char,
    ) -> c_int;

    pub fn SP_put_number_codes(term: SP_term_ref, s: *const c_char) -> c_int;
    pub fn SP_put_string(term: SP_term_ref, s: *const c_char) -> c_int;
    pub fn SP_put_term(to: SP_term_ref, from: SP_term_ref) -> c_int;
    pub fn SP_put_variable(term: SP_term_ref) -> c_int;
    pub fn SP_query(predicate: SP_pred_ref, arg1: SP_term_ref, ...) -> c_int;
    pub fn SP_query_cut_fail(predicate: SP_pred_ref, arg1: SP_term_ref, ...) -> c_int;
    pub fn SP_raise_exception(term: SP_term_ref);
    pub fn SP_read_from_string(
        t: SP_term_ref,
        string: *const c_char,
        values: *mut SP_term_ref,
    ) -> c_int;
    pub fn SP_realloc(ptr: *mut c_void, size: usize) -> *mut c_void;
    pub fn SP_register_atom(name: *const c_char, atom: *mut SP_atom) -> c_int;
    pub fn SP_restore(filenmae: *const c_char) -> c_int;
    pub fn SP_set_argv(argc: c_int, argv: *mut *mut c_char, options: spio_t_bits) -> c_int;
    pub fn SP_set_current_dir(dir: *const c_char) -> c_int;
    pub fn SP_set_user_stream_hook(
        hook: *mut SP_UserStreamHook,
        user_data: *mut c_void,
    ) -> *mut SP_UserStreamHook;
    pub fn SP_set_user_stream_post_hook(
        hook: *mut SP_UserStreamPostHook,
        user_data: *mut c_void,
    ) -> *mut SP_UserStreamPostHook;
    pub fn SP_signal(sig: c_int, fun: SP_SigFun, user_data: *mut c_void) -> c_int;
    pub fn SP_strdup(str: *const c_char) -> *mut c_void;
    pub fn SP_string_from_atom(atom: SP_atom) -> *const c_char;
    pub fn SP_term_type(term: SP_term_ref) -> c_int;
    pub fn SP_unget_byte(SP_stream: *mut SP_stream, item: c_int) -> spio_t_error_code;
    pub fn SP_unget_code(SP_stream: *mut SP_stream, item: c_int) -> spio_t_error_code;
    pub fn SP_unify(term1: SP_term_ref, term2: SP_term_ref) -> c_int;
    pub fn SP_unregister_atom(atom: SP_atom) -> c_int;
    pub fn SU_initialize(argc: c_int, argv: *mut *mut c_char) -> c_int;
    pub fn user_close(
        puser_data: *mut *mut c_void,
        close_options: spio_t_bits,
    ) -> spio_t_error_code;
    pub fn user_flush_output(
        user_data: *mut c_void,
        flush_options: spio_t_bits,
    ) -> spio_t_error_code;
    pub fn user_read(
        user_data: *mut c_void,
        buf: *mut c_void,
        pbuf_size: *mut usize,
        read_options: spio_t_bits,
    ) -> spio_t_error_code;
    pub fn user_write(
        user_data: *mut c_void,
        buf: *const c_void,
        pbuf_size: *mut usize,
        write_options: spio_t_bits,
    ) -> spio_t_error_code;
}
