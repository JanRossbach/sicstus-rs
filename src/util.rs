use core::ffi::c_char;

use alloc::string::String;
use regex::Regex;

/// Create a new [String] from a *const pointer to a C string.
/// This does not take ownership of the pointer. The caller is responsible for freeing the memory.
/// # Safety
/// The pointer must be valid and point to a null terminated C string.
pub unsafe fn string_from_ref(sp: *const c_char) -> String {
    let mut result = String::new();
    let mut cp: *const c_char = sp;
    loop {
        let c: c_char = *cp;
        let c = c as u8 as char;
        if c == '\0' {
            break;
        }
        result.push(c as u8 as char);
        cp = cp.add(1);
    }
    result
}

#[cfg(test)]
#[test]
fn test_string_copy() {
    use alloc::{ffi::CString, string::ToString};
    let s: CString = CString::new("Hello, World!").expect("CString::new failed");
    let s: *const c_char = s.as_ptr();
    unsafe {
        let pp: *const c_char = s;
        let pp = pp.add(1);
        assert_eq!('l', *(pp.add(1)) as u8 as char);
    }
    let copied_string: String = unsafe { string_from_ref(s) };
    assert_eq!(copied_string, "Hello, World!".to_string());
}

pub fn is_valid_atom_name(name: &str) -> bool {
    // TODO Check for correct bracket pairs
    let re = Regex::new(r"^[a-z][a-zA-Z0-9_\+\-\*/\\\^<>=~:.\?@#$&!;\[\]\{\}]*$").unwrap();
    let re2 = Regex::new(r"^'.*'$").unwrap();
    re.is_match(name) || re2.is_match(name)
}

#[cfg(test)]
#[test]
fn test_is_valid_atom_name() {
    assert!(!is_valid_atom_name(""));
    assert!(is_valid_atom_name("a"));
    assert!(is_valid_atom_name("a+-*/\\^<>=~:.?@#$&!;[]{}"));
    assert!(!is_valid_atom_name("X1"));
    assert!(!is_valid_atom_name("_X1"));
    assert!(is_valid_atom_name("'_X1'"));
}

pub fn is_valid_variable_name(name: &str) -> bool {
    let re = Regex::new(r"^[A-Z_][a-zA-Z0-9_]*$").unwrap();
    re.is_match(name)
}

#[cfg(test)]
#[test]
fn test_is_valid_variable_name() {
    assert!(!is_valid_variable_name(""));
    assert!(is_valid_variable_name("X"));
    assert!(!is_valid_variable_name("a+-*/\\^<>=~:.?@#$&!;[]{}"));
    assert!(is_valid_variable_name("X1"));
    assert!(is_valid_variable_name("_X1"));
    assert!(!is_valid_variable_name("'_X1'"));
}
