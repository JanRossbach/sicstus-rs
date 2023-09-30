//! Prolog IO functionalty including Streams.
//! Still only partially supported, meaning it just calls directly into the sys variant without making any safety checks.

use super::sys::*;

use core::ffi::{c_char, c_int, c_void};

pub fn sp_expand_file_name(
    relpath: *const c_char,
    cwd: *mut c_char,
    options: spio_t_bits,
    pabspath: *mut *mut c_char,
) -> c_int {
    unsafe { SP_expand_file_name(relpath, cwd, options, pabspath) }
}

// pub fn sp_get_byte(stream: *mut SP_stream) -> spio_t_error_code {
//     unsafe { SP_get_byte(stream) }
// }
// pub fn sp_get_code(stream: *mut SP_stream) -> spio_t_error_code {
//     unsafe { SP_get_code(stream) }
// }
pub fn sp_unget_byte(stream: *mut SP_stream, item: c_int) -> spio_t_error_code {
    unsafe { SP_unget_byte(stream, item) }
}
pub fn sp_unget_code(stream: *mut SP_stream, item: c_int) -> spio_t_error_code {
    unsafe { SP_unget_code(stream, item) }
}
// pub fn sp_fprintf(stream: *mut SP_stream, fmt: *const c_char, args: Vec<_>) -> spio_t_error_code {
//     unsafe { SP_fprintf(stream, fmt) }
// }

// pub fn sp_printf(fmt: *const c_char, ...) -> spio_t_error_code;
// pub fn sp_put_byte(stream: *mut SP_stream, item: c_int) -> spio_t_error_code {
//     unsafe { SP_put_byte(stream, item) }
// }
pub fn sp_put_bytes(
    strea: *mut SP_stream,
    codes: *const spio_t_uint8,
    byte_count: usize,
    options: spio_t_bits,
) -> spio_t_error_code {
    unsafe { SP_put_bytes(strea, codes, byte_count, options) }
}
// pub fn sp_put_code(stream: *mut SP_stream, item: c_int) -> spio_t_error_code {
//     unsafe { SP_put_code(stream, item) }
// }

pub fn sp_put_codes(
    strea: *mut SP_stream,
    codes: *const spio_t_wchar,
    code_count: usize,
    options: spio_t_bits,
) -> spio_t_error_code {
    unsafe { SP_put_codes(strea, codes, code_count, options) }
}
pub fn sp_put_encoded_string(
    stream: *mut SP_stream,
    encoded_string: *const c_char,
    options: spio_t_bits,
) -> spio_t_error_code {
    unsafe { SP_put_encoded_string(stream, encoded_string, options) }
}

pub fn sp_fclose(stream: *mut SP_stream, close_options: spio_t_bits) -> spio_t_error_code {
    unsafe { SP_fclose(stream, close_options) }
}

pub fn sp_fopen(
    pathname: *const c_char,
    reserved: *mut c_void,
    options: spio_t_bits,
    pstream: *mut *mut SP_stream,
) -> spio_t_error_code {
    unsafe { SP_fopen(pathname, reserved, options, pstream) }
}

// pub fn user_flush_output(user_data: *mut c_void, flush_options: spio_t_bits) -> spio_t_error_code {
//     unsafe { super::sys::user_flush_output(user_data, flush_options) }
// }

pub fn sp_load(filename: *const c_char) -> c_int {
    unsafe { SP_load(filename) }
}
pub fn sp_restore(filenmae: *const c_char) -> c_int {
    unsafe { SP_restore(filenmae) }
}
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
pub fn sp_create_stream(
    user_data: *mut c_void,
    user_class: *const c_void,
    user_read: spio_t_simple_device_read,
    user_write: spio_t_simple_device_write,
    user_flush_output: spio_t_simple_device_flush_output,
    user_seek: spio_t_simple_device_seek,
    user_close: spio_t_simple_device_close,
    user_interrupt: spio_t_simple_device_interrupt,
    user_ioctl: spio_t_simple_device_ioctl,
    args: *mut c_void,
    create_stream_options: spio_t_bits,
    pstream: *mut *mut SP_stream,
) -> spio_t_error_code {
    unsafe {
        SP_create_stream(
            user_data,
            user_class,
            user_read,
            user_write,
            user_flush_output,
            user_seek,
            user_close,
            user_interrupt,
            user_ioctl,
            args,
            create_stream_options,
            pstream,
        )
    }
}
pub fn sp_set_user_stream_hook(
    hook: SP_UserStreamHook,
    user_data: *mut c_void,
) -> SP_UserStreamHook {
    unsafe { SP_set_user_stream_hook(hook, user_data) }
}

pub fn sp_set_user_stream_post_hook(
    hook: SP_UserStreamPostHook,
    user_data: *mut c_void,
) -> SP_UserStreamPostHook {
    unsafe { SP_set_user_stream_post_hook(hook, user_data) }
}

pub fn sp_get_current_dir() -> *mut c_char {
    unsafe { SP_get_current_dir() }
}

pub fn sp_get_stream_counts(
    stream: *mut SP_stream,
    ptiem_count: *mut spio_t_offset,
    pnewline_count: *mut spio_t_offset,
    pline_length: *mut spio_t_offset,
    options: spio_t_bits,
) -> spio_t_error_code {
    unsafe { SP_get_stream_counts(stream, ptiem_count, pnewline_count, pline_length, options) }
}

pub fn sp_get_stream_user_data(
    stream: *mut SP_stream,
    user_class: *const c_void,
    puser_data: *mut *mut c_void,
) -> spio_t_error_code {
    unsafe { SP_get_stream_user_data(stream, user_class, puser_data) }
}

pub fn sp_getenv(name: *const c_char) -> *mut c_char {
    unsafe { SP_getenv(name) }
}

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
pub fn sp_event(func: SP_EventFun, arg: *mut c_void) -> c_int {
    unsafe { SP_event(func, arg) }
}
