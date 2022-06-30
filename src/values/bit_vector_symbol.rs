use std::borrow::Borrow;
use std::cell::Ref;
use std::collections::{HashMap, HashSet, BTreeMap};
use std::hash::Hash;
use std::rc::Weak;
use std::{borrow::BorrowMut, cell::RefCell, rc::Rc};

use z3_sys::{
    Z3_ast, Z3_context, Z3_dec_ref, Z3_inc_ref, Z3_mk_bool_sort, Z3_mk_const, Z3_mk_fresh_const,
    Z3_mk_string, Z3_solver_assert, _Z3_symbol, Z3_mk_bv_sort, Z3_string,
};

use crate::expressions::{finish_clone, inherit};
use crate::{ScfiaStdlib};

use super::{ActiveValue, RetiredValue};

#[derive(Debug)]
pub struct BitVectorSymbol {
    pub id: u64,
    pub width: u32,
    pub expression: Option<(u64, Rc<RefCell<ActiveValue>>)>,
    pub inherited_asts: BTreeMap<u64, Rc<RefCell<RetiredValue>>>,
    pub discovered_asts: HashMap<u64, Weak<RefCell<ActiveValue>>>,
    pub z3_context: Z3_context,
    pub z3_ast: Z3_ast,
}

#[derive(Debug)]
pub struct RetiredBitvectorSymbol {
    pub id: u64,
    pub expression: Option<(u64, Weak<RefCell<ActiveValue>>)>,
    pub z3_context: Z3_context,
    pub z3_ast: Z3_ast,
}

impl BitVectorSymbol {
    pub fn new(
        expression: Option<(u64, Rc<RefCell<ActiveValue>>)>,
        width: u32,
        stdlib: &mut ScfiaStdlib,
    ) -> Self {
        Self::new_with_id(stdlib.get_symbol_id(), width, expression, stdlib)
    }

    pub fn new_with_id(
        id: u64,
        width: u32,
        expression: Option<(u64, Rc<RefCell<ActiveValue>>)>,
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
                    expression.1.try_borrow().unwrap().get_z3_ast(),
                );
            }

            let bvs = BitVectorSymbol {
                id,
                width,
                expression,
                inherited_asts: BTreeMap::new(),
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
        let cloned_expression = if let Some(expression) = &self.expression {
            Some((expression.0, expression.1.try_borrow().unwrap().clone_to_stdlib(cloned_active_values, cloned_retired_values, cloned_stdlib)))
        } else {
            None
        };

        let cloned_symbol = Self::new_with_id(
            self.id,
            self.width,
            cloned_expression,
            cloned_stdlib
        );

        finish_clone(
            self.id,
            &self.inherited_asts,
            &self.discovered_asts,
            cloned_symbol.into(),
            cloned_active_values,
            cloned_retired_values,
            cloned_stdlib
        )
    }

}

impl Drop for BitVectorSymbol {
    fn drop(&mut self) {
        // Retire expression, maintain z3 ast refcount
        let retired_expression = Rc::new(RefCell::new(RetiredValue::RetiredBitvectorSymbol(RetiredBitvectorSymbol {
            id: self.id,
            expression: if let Some(expression) = &self.expression { Some((expression.0, Rc::downgrade(&expression.1))) } else { None },
            z3_context: self.z3_context,
            z3_ast: self.z3_ast,
        })));

        // Heirs are parents and discovered symbols
        let mut parents: Vec<(u64, Rc<RefCell<ActiveValue>>)> = vec![];
        if let Some(expression) = &self.expression {
            parents.push(expression.clone())
        }

        inherit(
            self.id,
            retired_expression,
            parents,
            &self.inherited_asts,
            &self.discovered_asts
        );
    }
}

impl Drop for RetiredBitvectorSymbol {
    fn drop(&mut self) {
        println!("Dropping RetiredBitvectorSymbol {}", self.id);
        unsafe { Z3_dec_ref(self.z3_context, self.z3_ast) }
    }
}
