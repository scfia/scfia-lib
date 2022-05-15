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

use crate::traits::ast::{Ast};
use crate::traits::bit_vector::BitVector;
use crate::traits::bit_vector_expression::BitVectorExpression;
use crate::traits::expression::{self, Expression};
use crate::{ScfiaStdlib};

use super::{ActiveValue, RetiredValue};

#[derive(Debug)]
pub struct BitVectorSymbol {
    pub id: u64,
    pub expression: Option<Rc<RefCell<ActiveValue>>>,
    pub inherited_asts: Vec<Rc<RefCell<RetiredValue>>>,
    pub discovered_asts: HashMap<u64, Weak<RefCell<ActiveValue>>>,
    pub z3_context: Z3_context,
    pub z3_ast: Z3_ast,
}

#[derive(Debug)]
pub struct RetiredBitvectorSymbol {
    id: u64,
    expression: Option<u64>,
    pub z3_context: Z3_context,
    pub z3_ast: Z3_ast,
}

impl BitVectorSymbol {
    pub fn new(
        expression: Option<Rc<RefCell<ActiveValue>>>,
        width: u32,
        stdlib: &mut ScfiaStdlib,
    ) -> Self {
        unsafe {
            let z3_ast;
            let z3_context = stdlib.z3_context;
            let symbol_id = stdlib.get_symbol_id();

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

            let bvs = BitVectorSymbol {
                id: symbol_id,
                expression,
                inherited_asts: vec![],
                discovered_asts: HashMap::new(),
                z3_context,
                z3_ast,
            };

            bvs
        }
    }
}

impl Drop for BitVectorSymbol {
    fn drop(&mut self) {
        // Retire expression, maintain z3 ast refcount
        let retired_expression = Rc::new(RefCell::new(RetiredValue::RetiredBitvectorSymbol(RetiredBitvectorSymbol {
            id: self.id,
            expression: if let Some(expression) = &self.expression { Some(expression.try_borrow().unwrap().get_id()) } else { None },
            z3_context: self.z3_context,
            z3_ast: self.z3_ast,
        })));

        // Heirs are parents and discovered symbols
        let mut heirs: Vec<Rc<RefCell<ActiveValue>>> = vec![];
        if let Some(expression) = &self.expression {
            heirs.push(expression.clone())
        }
        for discovered_symbol in self.discovered_asts.values() {
            let discovered_symbol = discovered_symbol.upgrade().unwrap();
            let mut discovered_symbol_ref = discovered_symbol.try_borrow_mut().unwrap();
            discovered_symbol_ref.forget(self.id);
            heirs.push(discovered_symbol.clone())
        }

        // For each heir...
        for heir in &heirs {
            let mut heir_ref = heir.try_borrow_mut().unwrap();

            // Pass on inherited symbols
            for inherited in &self.inherited_asts {
                heir_ref.inherit(inherited.clone())
            }

            // Inherit
            heir_ref.inherit(retired_expression.clone());

            // Acquaint all heirs
            for other_heir in &heirs {
                if let Ok(mut other_heir_ref) = other_heir.try_borrow_mut() {
                    heir_ref.discover(other_heir_ref.get_id(), Rc::downgrade(other_heir));
                    other_heir_ref.discover(heir_ref.get_id(), Rc::downgrade(heir));
                }                
            }
        }
    }
}

impl Drop for RetiredBitvectorSymbol {
    fn drop(&mut self) {
        unsafe { Z3_dec_ref(self.z3_context, self.z3_ast) }
    }
}
