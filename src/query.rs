// use alloc::vec::Vec;
// use sicstus_sys::SP_qid;

// use crate::{predicate::Predicate, term::Term};

// pub struct Query {
//     predicate: Predicate,
//     args: Vec<Term>,
// }

// struct QueryIterator {
//     qid: SP_qid,
// }

// impl Iterator for Query {
//     type Item = Term;

//     fn next(&mut self) -> Option<Self::Item> {
//         let term = Term::new();
//         let success = sp_next_solution(self.qid, term.term_ref);
//         if success {
//             Some(term)
//         } else {
//             None
//         }
//     }
// }

// impl Query {
//     pub fn new(predicate: Predicate, args: Vec<Term>) -> Self {
//         Query { predicate, args }
//     }
// }

// impl IntoIterator for Query {
//     type Item = Term;
//     type IntoIter = QueryIterator;

//     fn into_iter(self) -> Self::IntoIter {
//         let qid = sp_open_query(self.predicate.pred_ref, self.args);
//         QueryIterator { qid }
//     }
// }
