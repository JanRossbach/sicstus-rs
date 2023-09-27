//! Prolog IO functionalty including Streams.
//! Still only partially supported, meaning it just calls directly into the sys variant without making any safety checks.

use super::sys::*;

use std::ffi::c_char;
use std::os::raw::{c_int, c_void};

pub fn sp_expand_file_name(
    relpath: *const c_char,
    cwd: *mut c_char,
    options: spio_t_bits,
    pabspath: *mut *mut c_char,
) -> c_int {
    unsafe { SP_expand_file_name(relpath, cwd, options, pabspath) }
}

pub fn sp_get_byte(stream: *mut SP_stream) -> spio_t_error_code {
    unsafe { SP_get_byte(stream) }
}
pub fn sp_get_code(stream: *mut SP_stream) -> spio_t_error_code {
    unsafe { SP_get_code(stream) }
}
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
pub fn sp_put_byte(stream: *mut SP_stream, item: c_int) -> spio_t_error_code {
    unsafe { SP_put_byte(stream, item) }
}
pub fn sp_put_bytes(
    strea: *mut SP_stream,
    codes: *const spio_t_uint8,
    byte_count: usize,
    options: spio_t_bits,
) -> spio_t_error_code {
    unsafe { SP_put_bytes(strea, codes, byte_count, options) }
}
pub fn sp_put_code(stream: *mut SP_stream, item: c_int) -> spio_t_error_code {
    unsafe { SP_put_code(stream, item) }
}

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
    encoded_string: *const spio_t_wchar,
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
pub fn user_flush_output(user_data: *mut c_void, flush_options: spio_t_bits) -> spio_t_error_code {
    unsafe { super::sys::user_flush_output(user_data, flush_options) }
}
pub fn sp_load(filename: *const c_char) -> c_int {
    unsafe { SP_load(filename) }
}
pub fn sp_restore(filenmae: *const c_char) -> c_int {
    unsafe { SP_restore(filenmae) }
}
pub fn sp_create_stream(
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
            create_stream_options,
            pstream,
        )
    }
}
pub fn sp_set_user_stream_hook(
    hook: *mut SP_UserStreamHook,
    user_data: *mut c_void,
) -> *mut SP_UserStreamHook {
    unsafe { SP_set_user_stream_hook(hook, user_data) }
}
pub fn sp_set_user_stream_post_hook(
    hook: *mut SP_UserStreamPostHook,
    user_data: *mut c_void,
) -> *mut SP_UserStreamPostHook {
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

pub fn sp_getenv(name: *const c_char) -> *mut c_void {
    unsafe { SP_getenv(name) }
}
