use z3_sys::Z3_ast;
use std::{fmt::Debug, rc::{Rc, Weak}, cell::RefCell, sync::Arc, any::Any, collections::HashMap};

use crate::{expressions::bv_add_expression::BVAddExpression, ScfiaStdlib, values::{ActiveValue, RetiredValue, Value}};

use super::{expression::Expression};


pub trait Ast: Any + Debug + 'static {
    fn get_id(&self) -> u64;
    fn get_z3_ast(&self) -> Z3_ast;
    fn inherit(&mut self, ast: Rc<RefCell<RetiredValue>>);
    fn discover(&mut self, ast_id: u64, ast: Weak<RefCell<ActiveValue>>);
    fn forget(&mut self, id: u64);
}
