use std::{rc::{Rc, Weak}, cell::RefCell};

use z3_sys::{Z3_ast, Z3_context, Z3_dec_ref, Z3_inc_ref};

use crate::{traits::{expression::{Expression, self}, bit_vector_expression::BitVectorExpression, ast::Ast}, ScfiaStdlib};

#[derive(Debug)]
pub struct RetiredBitvectorSymbol {
    symbol_id: u64,
    _expression: Option<Weak<RefCell<dyn BitVectorExpression>>>,
    z3_context: Z3_context,
    z3_ast: Z3_ast,
}

impl RetiredBitvectorSymbol {
    pub fn new(
        symbol_id: u64,
        expression: Option<Weak<RefCell<dyn BitVectorExpression>>>,
        z3_context: Z3_context,
        z3_ast: Z3_ast,
    ) -> RetiredBitvectorSymbol {
        RetiredBitvectorSymbol {
            symbol_id,
            _expression: expression,
            z3_context,
            z3_ast,
        }
    }
}

impl Ast for RetiredBitvectorSymbol {
    fn get_id(&self) -> u64 {
        self.symbol_id
    }

    fn get_z3_ast(&self) -> Z3_ast {
        self.z3_ast
    }

    fn get_parents(&self, list: &mut Vec<Rc<RefCell<dyn Ast>>>) {
        unreachable!()
    }

    fn inherit(&mut self, ast: Rc<RefCell<dyn Ast>>) {
        unreachable!()
    }

    fn get_cloned(&self, clone_map: &mut std::collections::HashMap<u64, Rc<RefCell<dyn Ast>>>, cloned_stdlib: &mut ScfiaStdlib) -> Rc<RefCell<dyn Ast>> {
        todo!()
    }
}

impl Drop for RetiredBitvectorSymbol {
    fn drop(&mut self) {
        unsafe { Z3_dec_ref(self.z3_context, self.z3_ast) }
    }
}
