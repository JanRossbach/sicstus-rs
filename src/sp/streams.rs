use super::sys::*;

pub fn sp_fclose(stream: *mut SP_stream, close_options: spio_t_bits) -> spio_t_error_code {
    unsafe { SP_fclose(stream, close_options) }
}

use std::ffi::c_char;
use std::os::raw::{c_int, c_void};

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
