use std::borrow::Borrow;
use std::cell::Ref;
use std::collections::{HashMap, HashSet};
use std::hash::Hash;
use std::rc::Weak;
use std::{borrow::BorrowMut, cell::RefCell, rc::Rc};

use z3_sys::{
    Z3_ast, Z3_context, Z3_dec_ref, Z3_inc_ref, Z3_mk_bool_sort, Z3_mk_const, Z3_mk_fresh_const,
    Z3_mk_string, Z3_solver_assert, _Z3_symbol, Z3_mk_bv_sort, Z3_string,
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
    pub width: u32,
    pub expression: Option<Rc<RefCell<ActiveValue>>>,
    pub inherited_asts: Vec<Rc<RefCell<RetiredValue>>>,
    pub discovered_asts: HashMap<u64, Weak<RefCell<ActiveValue>>>,
    pub z3_context: Z3_context,
    pub z3_ast: Z3_ast,
}

#[derive(Debug)]
pub struct RetiredBitvectorSymbol {
    pub id: u64,
    pub expression_id: Option<u64>,
    pub expression: Option<Weak<RefCell<ActiveValue>>>,
    pub z3_context: Z3_context,
    pub z3_ast: Z3_ast,
}

impl BitVectorSymbol {
    pub fn new(
        expression: Option<Rc<RefCell<ActiveValue>>>,
        width: u32,
        stdlib: &mut ScfiaStdlib,
    ) -> Self {
        Self::new_with_id(stdlib.get_symbol_id(), width, expression, stdlib)
    }

    pub fn new_with_id(
        id: u64,
        width: u32,
        expression: Option<Rc<RefCell<ActiveValue>>>,
        stdlib: &mut ScfiaStdlib,
    ) -> Self {
        unsafe {
            let z3_ast;
            let z3_context = stdlib.z3_context;

            z3_ast = Z3_mk_fresh_const(
                z3_context,
                0 as Z3_string,
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
                id,
                width,
                expression,
                inherited_asts: vec![],
                discovered_asts: HashMap::new(),
                z3_context,
                z3_ast,
            };

            bvs
        }
    }

    pub fn clone_to_stdlib(
        &self,
        cloned_active_values: &mut HashMap<u64, Rc<RefCell<ActiveValue>>>,
        cloned_retired_values: &mut HashMap<u64, Rc<RefCell<RetiredValue>>>,
        cloned_stdlib: &mut ScfiaStdlib,
    ) -> Rc<RefCell<ActiveValue>> {
        if let Some(value) = cloned_active_values.get(&self.id) {
            return value.clone();
        }

        let cloned_expression = if let Some(expression) = &self.expression {
            Some(expression.try_borrow().unwrap().clone_to_stdlib(cloned_active_values, cloned_retired_values, cloned_stdlib))
        } else {
            None
        };

        let mut cloned_symbol = Self::new_with_id(
            self.id,
            self.width,
            cloned_expression,
            cloned_stdlib
        );

        let cloned_symbol: Rc<RefCell<ActiveValue>> = cloned_symbol.into();
        cloned_active_values.insert(self.id, cloned_symbol.clone());

        for inherited_ast in self.inherited_asts.iter() {
            let cloned_inherited_ast = inherited_ast.try_borrow().unwrap().clone_to_stdlib(cloned_active_values, cloned_retired_values, cloned_stdlib);
            cloned_symbol.try_borrow_mut().unwrap().inherit(cloned_inherited_ast);
        }

        // TODO clone discovered values

        cloned_symbol
    }

}

impl Drop for BitVectorSymbol {
    fn drop(&mut self) {
        // Retire expression, maintain z3 ast refcount
        let retired_expression = Rc::new(RefCell::new(RetiredValue::RetiredBitvectorSymbol(RetiredBitvectorSymbol {
            id: self.id,
            expression_id: if let Some(expression) = &self.expression { Some(expression.try_borrow().unwrap().get_id()) } else { None },
            expression: if let Some(expression) = &self.expression { Some(Rc::downgrade(expression)) } else { None },
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

            // Inherit
            heir_ref.inherit(retired_expression.clone());

            // Pass on inherited symbols
            for inherited in &self.inherited_asts {
                heir_ref.inherit(inherited.clone())
            }

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
