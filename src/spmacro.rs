// use crate::{
//     sp::{terms::sp_new_term_ref, SP_term_ref},
//     term::{Atom, Term},
// };
// use core::marker::PhantomData;

// #[macro_export]
// macro_rules! plterm {
//     ($name: ident) => {{
//         let term_ref: SP_term_ref = sp_new_term_ref();
//         Term {
//             term_ref,
//             kind: PhantomData::<Atom>,
//         }
//     }};
//     ($name: ident, $value: expr) => {{
//         let term_ref: SP_term_ref = sp_new_term_ref();
//         Term {
//             term_ref,
//             kind: PhantomData::<Atom>,
//         }
//     }};
// }


// macro pl {
//     ($prolog_code:expr) => {{
//         let tmref = sp_new_term_ref();
//         sp_term_from_string(tmref, $prolog_code, ).unwrap();
//         "hello"
//     }}
// }

// fn test_macro() {
//     let result: &str = pl!("test");
// }
