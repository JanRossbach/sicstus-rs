use super::sys::*;

use std::ffi::c_char;
use std::os::raw::c_int;

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
