use z3_sys::Z3_ast;
use std::{fmt::Debug, rc::Rc, cell::RefCell, sync::Arc, any::Any, collections::HashMap};

use crate::{expressions::bv_add_expression::BVAddExpression, ScfiaStdlib};

use super::{expression::Expression};


pub trait Ast: Any + Debug + 'static {
    fn get_id(&self) -> u64;
    fn get_z3_ast(&self) -> Z3_ast;
    fn get_parents(&self, list: &mut Vec<Rc<RefCell<dyn Ast>>>);
    fn inherit(&mut self, ast: Rc<RefCell<dyn Ast>>);
    fn get_cloned(&self, clone_map: &mut HashMap<u64, Rc<RefCell<dyn Ast>>>, cloned_stdlib: &mut ScfiaStdlib) -> Rc<RefCell<dyn Ast>>;
}
