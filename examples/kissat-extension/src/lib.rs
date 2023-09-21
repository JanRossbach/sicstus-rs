extern crate kissat;
use sicstus_rs::{SP_integer, SP_term_ref};

#[no_mangle]
pub extern "C" fn c1(a: SP_integer) -> SP_integer {
    a + 9
}

pub fn add(left: usize, right: usize) -> usize {
    left + right
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result = add(2, 2);
        assert_eq!(result, 4);
    }
}

#[test]
fn test_kissat() {
    let mut solver = kissat::Solver::new();
    let a = solver.var();
    let b = solver.var();
    solver.add2(a, !b);
    solver.add1(b);
    let result = solver.sat();
    assert!(result.is_some());
    let solution = result.unwrap();
    assert_eq!(solution.get(a), Some(true));
    assert_eq!(solution.get(b), Some(true));
}
