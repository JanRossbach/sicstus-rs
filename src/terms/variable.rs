use core::marker::PhantomData;

use crate::util::is_valid_variable_name;
use crate::sys::*;

use super::Term;

#[derive(Debug)]
pub struct Var;
