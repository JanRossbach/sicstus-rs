mod sys;
mod list;

use crate::sys::sys_tests;
use crate::list::test_list;


#[no_mangle]
pub extern "C" fn rust_main() {
    run_tests();
}

fn run_tests() {
    sys_tests();
    test_list();
}
