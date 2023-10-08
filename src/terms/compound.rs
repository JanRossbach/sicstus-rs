#[derive(Debug)]
pub struct List {}

#[macro_export]
macro_rules! term {
    ($functor:ident($($arg:expr),+)$rest:tt) => {
        term!($rest)
    };
    ($const:expr) => {
        "constant"
    };
    (.) => {
        "compound"
    }
}

#[cfg(test)]
#[test]
fn test_macro() {
    let t = term!(foo(1,2,3).);
    assert_eq!(t, "compound");
}
