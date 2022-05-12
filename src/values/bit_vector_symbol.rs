use std::borrow::Borrow;
use std::cell::Ref;
use std::collections::{HashMap, HashSet};
use std::hash::Hash;
use std::rc::Weak;
use std::{borrow::BorrowMut, cell::RefCell, rc::Rc};

use z3_sys::{
    Z3_ast, Z3_context, Z3_dec_ref, Z3_inc_ref, Z3_mk_bool_sort, Z3_mk_const, Z3_mk_fresh_const,
    Z3_mk_string, Z3_solver_assert, _Z3_symbol, Z3_mk_bv_sort,
};

use crate::traits::ast::{ActiveAst, Ast};
use crate::traits::bit_vector::BitVector;
use crate::traits::bit_vector_expression::BitVectorExpression;
use crate::traits::expression::{self, Expression};
use crate::Symbol;
use crate::{traits::bool::Bool, ScfiaStdlib};

use super::retired_bitvector_symbol::RetiredBitvectorSymbol;

#[derive(Debug)]
pub struct BitVectorSymbol {
    pub symbol_id: u64,
    pub expression: Option<Rc<RefCell<dyn BitVectorExpression>>>,
    z3_context: Z3_context,
    z3_ast: Z3_ast,
    discovered_relatives: HashMap<u64, Weak<RefCell<dyn ActiveAst>>>,
    retired_relatives: Vec<Rc<RefCell<dyn Ast>>>,
}

impl BitVectorSymbol {
    pub fn new(
        expression: Option<Rc<RefCell<dyn BitVectorExpression>>>,
        width: u32,
        stdlib: &mut ScfiaStdlib,
    ) -> Self {
        unsafe {
            let symbol_id;
            let z3_context;
            let z3_ast;
            {
                z3_context = stdlib.z3_context;
                symbol_id = stdlib.get_symbol_id();

                z3_ast = Z3_mk_const(
                    z3_context,
                    0 as *mut _Z3_symbol,
                    Z3_mk_bv_sort(stdlib.z3_context, width),
                );
                Z3_inc_ref(z3_context, z3_ast);
                if let Some(ref expression) = expression {
                    Z3_solver_assert(
                        stdlib.z3_context,
                        stdlib.z3_solver,
                        expression.try_borrow().unwrap().get_z3_ast(),
                    );
                }
            }

            let bvs = BitVectorSymbol {
                symbol_id,
                z3_context,
                z3_ast,
                expression,
                discovered_relatives: HashMap::new(),
                retired_relatives: vec![],
            };

            bvs
        }
    }
}

impl Ast for BitVectorSymbol {
    fn get_z3_ast(&self) -> Z3_ast {
        self.z3_ast
    }
}

impl ActiveAst for BitVectorSymbol {
    fn get_parents(&self, list: &mut Vec<Rc<RefCell<dyn ActiveAst>>>) {
        if let Some(expression) = &self.expression {
            expression.try_borrow().unwrap().get_parents(list)
        }
    }

    fn inherit(&mut self, ast: Rc<RefCell<dyn Ast>>) {
        self.retired_relatives.push(ast)
    }
}

impl BitVector for BitVectorSymbol {}

impl Symbol for BitVectorSymbol {
    fn get_symbol_id(&self) -> u64 {
        self.symbol_id
    }
}

impl Drop for BitVectorSymbol {
    fn drop(&mut self) {
        // Retire symbol, maintain z3 ast refcount
        let retired_symbol = Rc::new(RefCell::new(RetiredBitvectorSymbol::new(
            self.symbol_id,
            self.expression
                .as_ref()
                .map_or(None, |e| Some(Rc::downgrade(e))),
            self.z3_context,
            self.z3_ast,
        )));

        // Heirs are parents and discovered symbols
        let mut heirs: Vec<Rc<RefCell<dyn ActiveAst>>> = vec![];
        self.get_parents(&mut heirs);
        for discovered_symbol in self.discovered_relatives.values() {
            heirs.push(discovered_symbol.upgrade().unwrap())
        }

        // For each heir...
        for parent in &heirs {
            let mut parent_ref = parent.try_borrow_mut().unwrap();

            // Pass on inherited symbols
            for inherited in &self.retired_relatives {
                parent_ref.inherit(inherited.clone())
            }

            // Inherit
            parent_ref.inherit(retired_symbol.clone());
        }
    }
}

#[cfg(test)]
mod tests {
    use z3_sys::{
        Z3_dec_ref, Z3_inc_ref, Z3_mk_app, Z3_mk_bool_sort, Z3_mk_const, Z3_mk_fresh_const,
        Z3_mk_fresh_func_decl, _Z3_ast, _Z3_sort, _Z3_symbol,
    };

    use crate::ScfiaStdlib;
    use std::{cell::RefCell, iter::StepBy, rc::Rc};

    use super::BitVectorSymbol;

    #[test]
    fn mk_bvs() {
        let mut stdlib = ScfiaStdlib::new();
        let _bvc = BitVectorSymbol::new(None, 32, &mut stdlib);
    }
}
