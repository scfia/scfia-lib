use std::cell::Ref;
use std::rc::Weak;
use std::{rc::Rc, cell::RefCell};
use std::fmt::Debug;

use crate::values::retired_bitvector_symbol::RetiredBitvectorSymbol;

use super::ast::ActiveAst;

pub trait Symbol: ActiveAst + Debug {
    fn get_symbol_id(&self) -> u64;
}
