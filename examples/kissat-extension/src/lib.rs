use sicstus_rs::sys::{SP_integer, SP_term_ref};

mod satsolver;

#[no_mangle]
pub extern "C" fn c1(a: SP_integer) -> SP_integer {
    a + 9
}

#[no_mangle]
pub extern "C" fn delete_solver(id: SP_integer) {
    satsolver::delete_solver(id);
}

#[no_mangle]
pub extern "C" fn new_solver() -> SP_integer {
    satsolver::new_solver();
}
#[no_mangle]
pub extern "C" fn add_clause(solver_id: SP_integer, list: SP_term_ref) {
    satsolver::add_clause(solver_id, list);
}
#[no_mangle]
pub extern "C" fn solve(solver_id: SP_integer) {
    satsolver::solve(solver_id);
}
#[no_mangle]
pub extern "C" fn get_model(solver_id: SP_integer, model: SP_term_ref) {
    satsolver::get_model(solver_id, model);
}
#[no_mangle]
pub extern "C" fn assign_model(solver_id: SP_integer, asgnList: SP_term_ref) {
    satsolver::get_model(solver_id, model);
}
#[no_mangle]
pub extern "C" fn toDimacs(solver_id: SP_integer, filename: std::ffi::CString) {
    satsolver::toDimacs(solver_id, filename);
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_kissat_crate_is_working() {
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
}
