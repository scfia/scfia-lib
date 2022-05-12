use z3_sys::Z3_ast;
use std::{fmt::Debug, rc::Rc, cell::RefCell, sync::Arc, any::Any};

use crate::expressions::bv_add_expression::BVAddExpression;

use super::{expression::Expression, symbol::Symbol};

pub trait ActiveAst: Ast {
    fn get_parents(&self, list: &mut Vec<Rc<RefCell<dyn ActiveAst>>>);
    fn inherit(&mut self, ast: Rc<RefCell<dyn Ast>>);
}

pub trait Ast: Any + Debug + 'static {
    fn get_z3_ast(&self) -> Z3_ast;
}
