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
    test_create_variables();
    test_create_atoms();
    test_list();
}

fn test_create_variables() {
}

fn test_create_atoms() {
    // let hello_atom = Atom::from("hello");
    // println!("hello atom: {:?}", hello_atom);
}
