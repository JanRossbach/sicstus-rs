pub use sicstus_sys::{SP_integer, SP_term_ref};
use std::os::raw::c_int;

extern "C" {
    fn SP_new_term_ref() -> SP_term_ref;
    fn SP_get_integer(term: SP_term_ref, integer: *mut SP_integer) -> c_int;
    fn SP_get_arg(index: c_int, term: SP_term_ref, arg: SP_term_ref) -> c_int;
    fn SP_is_compound(term: SP_term_ref) -> c_int;
}

pub fn sp_new_term_ref() -> SP_term_ref {
    unsafe { SP_new_term_ref() }
}


fn write_path(path: SP_term_ref) {
    let tail = sp_new_term_ref();
    let via = sp_new_term_ref();
}
