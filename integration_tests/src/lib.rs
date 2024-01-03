mod sys;
mod list;
mod query;

use crate::sys::sys_tests;
use crate::list::test_list;
use crate::query::test_queries;


#[no_mangle]
pub extern "C" fn rust_main() {
    run_tests();
}

fn run_tests() {
    sys_tests();
    test_list();
    test_queries();
}
