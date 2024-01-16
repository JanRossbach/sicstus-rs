use core::cmp::Ordering;
use sicstus_rs::sys::*;

pub fn sys_tests() {
    test_sp_atom_from_string();
    test_sp_atom_length();
    test_sp_compare();
    test_print();
}

fn test_sp_atom_from_string() {
    let atom = sp_atom_from_string("hello").unwrap();
    sicstus_rs::println!("test_sp_atom_from_string: {:?}, Ok", atom);
    let atom2 = sp_atom_from_string("_ÖALKSDFJÖLK-").unwrap(); // Wierd atom name gets accepted by prolog. I think it treats it like it is in '' quotes.
    sicstus_rs::println!("test_sp_atom_from_string: {:?}, Ok", atom2);
}

fn test_sp_atom_length() {
    let atom = sp_atom_from_string("hello").unwrap();
    let len = sp_atom_length(atom);
    let s = sp_string_from_atom(atom);
    assert_eq!(s, "hello");
    assert_eq!(len, 5);
    sicstus_rs::println!("test_sp_atom_length, Ok");
}

fn test_sp_compare() {
    let a1 = sp_atom_from_string("hello").unwrap();
    let t1 = sp_new_term_ref();
    sp_put_atom(t1, a1).unwrap();
    let a2 = sp_atom_from_string("hollo").unwrap();
    let t2 = sp_new_term_ref();
    sp_put_atom(t2, a2).unwrap();
    assert_eq!(Ordering::Less, sp_compare(t1,t2));
    sicstus_rs::println!("test_sp_compare, Ok");
}

fn test_print() {
    sicstus_rs::print!("Hello, world from sicstus_rs print macro!\n");
    //println!("???");
    sicstus_rs::println!("Hello, world from sicstus_rs println macro!");
    println!("test_print, Ok");
}
