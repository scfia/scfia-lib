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
use crate::models::riscv::rv32i::ForkSink;
use crate::{ScfiaStdlib};

use super::{ActiveValue, RetiredValue};

#[derive(Debug)]
pub struct BitVectorSymbol {
    pub id: u64,
    pub stdlib_id: String,
    pub width: u32,
    pub expression: Option<(u64, Rc<RefCell<ActiveValue>>)>,
    pub inherited_asts: BTreeMap<u64, Rc<RefCell<RetiredValue>>>,
    pub discovered_asts: BTreeMap<u64, Weak<RefCell<ActiveValue>>>,
    pub z3_context: Z3_context,
    pub z3_ast: Z3_ast,
    pub depth: u64,
    pub comment: Option<String>,
}

#[derive(Debug)]
pub struct RetiredBitvectorSymbol {
    pub id: u64,
    pub width: u32,
    pub expression: Option<(u64, Weak<RefCell<ActiveValue>>)>,
    pub z3_context: Z3_context,
    pub z3_ast: Z3_ast,
}

impl BitVectorSymbol {
    pub fn new(
        expression: Option<(u64, Rc<RefCell<ActiveValue>>)>,
        width: u32,
        comment: Option<String>,
        stdlib: &mut ScfiaStdlib,
        fork_sink: &mut Option<&mut ForkSink>
    ) -> Rc<RefCell<ActiveValue>> {
        let value: Rc<RefCell<ActiveValue>> = Self::new_with_id(stdlib.get_symbol_id(), stdlib.id.clone(), width, expression, comment, stdlib).into();
        if let Some(fork_sink) = fork_sink {
            fork_sink.new_values.push(value.clone());
        }
        value
    }

    pub fn new_with_id(
        id: u64,
        stdlib_id: String,
        width: u32,
        expression: Option<(u64, Rc<RefCell<ActiveValue>>)>,
        comment: Option<String>,
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
                unimplemented!()
            }

            let bvs = BitVectorSymbol {
                id,
                stdlib_id,
                width,
                expression,
                inherited_asts: BTreeMap::new(),
                discovered_asts: BTreeMap::new(),
                z3_context,
                z3_ast,
                depth: 1,
                comment,
            };

            bvs
        }
    }

    pub fn clone_to_stdlib(
        &self,
        cloned_active_values: &mut BTreeMap<u64, Rc<RefCell<ActiveValue>>>,
        cloned_retired_values: &mut BTreeMap<u64, Rc<RefCell<RetiredValue>>>,
        cloned_stdlib: &mut ScfiaStdlib,
    ) -> Rc<RefCell<ActiveValue>> {
        let cloned_expression = if let Some(expression) = &self.expression {
            Some((expression.0, expression.1.try_borrow().unwrap().clone_to_stdlib(cloned_active_values, cloned_retired_values, cloned_stdlib)))
        } else {
            None
        };

        let cloned_symbol = Self::new_with_id(
            self.id,
            cloned_stdlib.id.clone(),
            self.width,
            cloned_expression,
            self.comment.clone(),
            cloned_stdlib,
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
            width: self.width,
            expression: if let Some(expression) = &self.expression { Some((expression.0, Rc::downgrade(&expression.1))) } else { None },
            z3_context: self.z3_context,
            z3_ast: self.z3_ast,
        })));

        // Heirs are parents and discovered symbols
        let mut parents: Vec<(u64, Rc<RefCell<ActiveValue>>)> = vec![];
        if let Some(expression) = &self.expression {
            parents.push(expression.clone())
        }

        if self.id == 16012 {
            println!("dropping active BVS 16012 from {}", self.stdlib_id);
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

impl RetiredBitvectorSymbol {
    pub fn clone_to_stdlib(
        &self,
        _cloned_active_values: &mut BTreeMap<u64, Rc<RefCell<ActiveValue>>>,
        cloned_retired_values: &mut BTreeMap<u64, Rc<RefCell<RetiredValue>>>,
        cloned_stdlib: &mut ScfiaStdlib
    ) -> Rc<RefCell<RetiredValue>> {
        unsafe {
            let cloned_z3_ast;
            let cloned_z3_context = cloned_stdlib.z3_context;

            cloned_z3_ast = Z3_mk_fresh_const(
                cloned_z3_context,
                0 as Z3_string,
                Z3_mk_bv_sort(cloned_stdlib.z3_context, self.width),
            );
            Z3_inc_ref(cloned_z3_context, cloned_z3_ast);
            let cloned_expression = if let Some(ref expression) = self.expression {
                todo!()
            } else {
                None
            };

            let bvs: Rc<RefCell<RetiredValue>> = RetiredBitvectorSymbol {
                id: self.id,
                width: self.width,
                expression: cloned_expression,
                z3_context: cloned_z3_context,
                z3_ast: cloned_z3_ast,
            }.into();

            cloned_retired_values.insert(self.id, bvs.clone());
            bvs
        }
    }
}

impl Drop for RetiredBitvectorSymbol {
    fn drop(&mut self) {
        // unsafe { Z3_dec_ref(self.z3_context, self.z3_ast) }
    }
}
