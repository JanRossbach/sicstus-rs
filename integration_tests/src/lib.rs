mod sys;

use sicstus_rs::terms::{Atom, Var};
use crate::sys::sys_tests;


#[no_mangle]
pub extern "C" fn rust_main() {
    run_tests();
}

fn run_tests() {
    sys_tests();
    test_create_variables();
    test_create_atoms();
}

fn test_create_variables() {
    let _ = Var::new();
}

fn test_create_atoms() {
    let hello_atom = Atom::from("hello");
    println!("hello atom: {:?}", hello_atom);
}
