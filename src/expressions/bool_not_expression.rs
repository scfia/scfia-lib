use crate::ScfiaStdlib;
use crate::models::riscv::rv32i::ForkSink;
use crate::values::ActiveValue;
use crate::values::RetiredValue;
use crate::values::Value;
use crate::values::bool_concrete::BoolConcrete;
use std::cell::Ref;
use std::cell::RefCell;
use std::collections::BTreeMap;
use std::collections::HashMap;
use std::ops::Deref;
use std::rc::Rc;
use std::rc::Weak;
use z3_sys::Z3_ast;
use z3_sys::Z3_context;
use z3_sys::Z3_dec_ref;
use z3_sys::Z3_inc_ref;
use z3_sys::Z3_mk_bvadd;
use z3_sys::Z3_mk_eq;
use z3_sys::Z3_mk_not;
use z3_sys::Z3_solver_assert;

use super::finish_clone;
use super::inherit;

#[derive(Debug)]
pub struct BoolNotExpression {
    pub id: u64,
    pub s1: Rc<RefCell<ActiveValue>>,
    pub inherited_asts: BTreeMap<u64, Rc<RefCell<RetiredValue>>>,
    pub discovered_asts: BTreeMap<u64, Weak<RefCell<ActiveValue>>>,
    pub is_assert: bool,
    pub z3_context: Z3_context,
    pub z3_ast: Z3_ast,
}

#[derive(Debug)]
pub struct RetiredBoolNotExpression {
    pub id: u64,
    s1_id: u64,
    s1: Weak<RefCell<ActiveValue>>,
    pub is_assert: bool,
    pub z3_context: Z3_context,
    pub z3_ast: Z3_ast,
}

impl BoolNotExpression {
    pub fn new(
        s1: Rc<RefCell<ActiveValue>>,
        stdlib: &mut ScfiaStdlib,
        fork_sink: &mut Option<&mut ForkSink>,
    ) -> Rc<RefCell<ActiveValue>> {
        let value: Rc<RefCell<ActiveValue>> = Self::new_try_concretize(s1, stdlib, fork_sink).into();
        if let Some(fork_sink) = fork_sink {
            fork_sink.new_values.push(value.clone());
        }
        value
    }

    pub fn new_try_concretize(
        s1: Rc<RefCell<ActiveValue>>,
        stdlib: &mut ScfiaStdlib,
        fork_sink: &mut Option<&mut ForkSink>,
    ) -> Rc<RefCell<ActiveValue>> {
        match s1.try_borrow().unwrap().deref() {
            ActiveValue::BoolConcrete(e1) => {
                return BoolConcrete::new(!e1.value, stdlib, fork_sink)
            }
            _ => {}
        }
        ActiveValue::BoolNotExpression(Self::new_with_id(stdlib.get_symbol_id(), s1, stdlib)).into()
    }

    pub fn new_with_id(
        id: u64,
        s1: Rc<RefCell<ActiveValue>>,
        stdlib: &mut ScfiaStdlib,
    ) -> BoolNotExpression {
        unsafe {
            let z3_context = stdlib.z3_context;
            let ast = Z3_mk_not(
                stdlib.z3_context,
                s1.try_borrow().unwrap().get_z3_ast(),
            );
            Z3_inc_ref(z3_context, ast);
            BoolNotExpression {
                id,
                s1: s1,
                inherited_asts: BTreeMap::new(),
                discovered_asts: BTreeMap::new(),
                is_assert: false,
                z3_context: z3_context,
                z3_ast: ast,
            }
        }
    }

    pub fn clone_to_stdlib(
        &self,
        cloned_active_values: &mut BTreeMap<u64, Rc<RefCell<ActiveValue>>>,
        cloned_retired_values: &mut BTreeMap<u64, Rc<RefCell<RetiredValue>>>,
        cloned_stdlib: &mut ScfiaStdlib
    ) -> Rc<RefCell<ActiveValue>> {
        // Clone s1
        let s1 = self.s1.try_borrow().unwrap().clone_to_stdlib(cloned_active_values, cloned_retired_values, cloned_stdlib);
        if let Some(e) = cloned_active_values.get(&self.id) {
            return e.clone()
        }

        // Build clone
        let cloned_expression = Self::new_with_id(self.id, s1, cloned_stdlib);
        if self.is_assert {
            unsafe {
                Z3_solver_assert(cloned_stdlib.z3_context, cloned_stdlib.z3_solver, cloned_expression.z3_ast);
            }
        }

        finish_clone(
            self.id,
            &self.inherited_asts,
            &self.discovered_asts,
            cloned_expression.into(),
            cloned_active_values,
            cloned_retired_values,
            cloned_stdlib
        )
    }

    pub fn assert(&mut self, stdlib: &mut ScfiaStdlib) {
        self.is_assert = true;
        unsafe {
            Z3_solver_assert(stdlib.z3_context, stdlib.z3_solver, self.z3_ast);
        }
    }
}

impl Drop for BoolNotExpression {
    fn drop(&mut self) {
        // Retire expression, maintain z3 ast refcount
        let s1_id = self.s1.try_borrow().unwrap().get_id();
        debug_assert!(s1_id < self.id);
        let retired_expression = Rc::new(RefCell::new(RetiredValue::RetiredBoolNotExpression(RetiredBoolNotExpression {
            id: self.id,
            s1_id: s1_id,
            s1: Rc::downgrade(&self.s1),
            is_assert: self.is_assert,
            z3_context: self.z3_context,
            z3_ast: self.z3_ast,
        })));

        let parents = vec![
            (s1_id, self.s1.clone()),
        ];
        inherit(
            self.id,
            retired_expression,
            parents,
            &self.inherited_asts,
            &self.discovered_asts
        );
    }
}

impl RetiredBoolNotExpression {
    pub fn clone_to_stdlib(
        &self,
        cloned_active_values: &mut BTreeMap<u64, Rc<RefCell<ActiveValue>>>,
        cloned_retired_values: &mut BTreeMap<u64, Rc<RefCell<RetiredValue>>>,
        cloned_stdlib: &mut ScfiaStdlib
    ) -> Rc<RefCell<RetiredValue>> {
        let cloned_s1_ast;
        let cloned_s1;
        if let Some(s1_rc) = cloned_active_values.get(&self.s1_id) {
            let s1 = s1_rc.try_borrow().unwrap();
            cloned_s1_ast = s1.get_z3_ast();
            cloned_s1 = Rc::downgrade(s1_rc);
        } else if let Some(s1_rc) = self.s1.upgrade() {
            let cloned_s1_rc = s1_rc.try_borrow().unwrap().clone_to_stdlib(cloned_active_values, cloned_retired_values, cloned_stdlib);
            cloned_s1_ast = cloned_s1_rc.try_borrow().unwrap().get_z3_ast();
            cloned_s1 = Rc::downgrade(&cloned_s1_rc);
        } else {
            let cloned_s1_rc = cloned_retired_values.get(&self.s1_id).unwrap();
            cloned_s1_ast = cloned_s1_rc.try_borrow().unwrap().get_z3_ast();
            cloned_s1 = Weak::new();
        };

        let cloned: Rc<RefCell<RetiredValue>> = unsafe {
            let z3_ast = Z3_mk_not(
                cloned_stdlib.z3_context,
                cloned_s1_ast,
            );
            Z3_inc_ref(cloned_stdlib.z3_context, z3_ast);

            if self.is_assert {
                Z3_solver_assert(cloned_stdlib.z3_context, cloned_stdlib.z3_solver, z3_ast);
            }

            RetiredBoolNotExpression {
                id: self.id,
                is_assert: self.is_assert,
                s1_id: self.s1_id,
                s1: cloned_s1,
                z3_context: cloned_stdlib.z3_context,
                z3_ast
            }
        }.into();

        cloned_retired_values.insert(self.id, cloned.clone());
        cloned
    }
}

impl Drop for RetiredBoolNotExpression {
    fn drop(&mut self) {
        // unsafe { Z3_dec_ref(self.z3_context, self.z3_ast) }
    }
}
