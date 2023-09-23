/// Re export of the sicstus_sys crate low level bindings for SICStus Prolog.
/// This is purely for convenience and in order to opt out of the high level API if
/// you need fine grained control over the C API.
pub mod sys {
    pub use sicstus_sys::*;
}


pub enum Term {
    Atom(String),
    Integer(i64),
    Float(f64),
    Compound(String, Vec<Term>),
    List(Vec<Term>),
    Var(String),
    EmptyList,
}

pub struct Prolog {}
impl Prolog {
    pub fn new() -> Self {
        Prolog {}
    }

    pub fn
}

#[test]
fn test_term() {
    let term = Term::Compound("foo".to_string(), vec![Term::Atom("bar".to_string())]);
    println!("{:?}", term);
}
