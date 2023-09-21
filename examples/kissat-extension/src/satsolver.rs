extern crate kissat;
use kissat::Solver;


static solvers: Arc<Mutex<Vec<Solver>>>  = Arc::new(Mutex::new(Vec::new()));


pub fn new_solver() -> usize {
    let mut solvers = solvers.lock().unwrap();
    solvers.push(Solver::new());
    Solver::new()
}

pub fn delete_solver(id: usize) {
    let mut solvers = solvers.lock().unwrap();
    solvers.remove(id);
}

pub fn add_clause(id: usize, list: SP_term_ref) {
    let head: SP_term_ref = SP_new_term_ref();
    let lits: Vec<kissat::Var> = Vec::new();
}
