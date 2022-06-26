use crate::ScfiaStdlib;
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
use z3_sys::Z3_mk_bvult;
use z3_sys::Z3_mk_lt;
use z3_sys::Z3_solver_assert;

use super::finish_clone;
use super::inherit;

#[derive(Debug)]
pub struct BoolLessThanUIntExpression {
    pub id: u64,
    pub s1: Rc<RefCell<ActiveValue>>,
    pub s2: Rc<RefCell<ActiveValue>>,
    pub inherited_asts: BTreeMap<u64, Rc<RefCell<RetiredValue>>>,
    pub discovered_asts: HashMap<u64, Weak<RefCell<ActiveValue>>>,
    pub is_assert: bool,
    pub z3_context: Z3_context,
    pub z3_ast: Z3_ast,
}

#[derive(Debug)]
pub struct RetiredBoolLessThanUIntExpression {
    pub id: u64,
    s1_id: u64,
    s1: Weak<RefCell<ActiveValue>>,
    s2_id: u64,
    s2: Weak<RefCell<ActiveValue>>,
    pub is_assert: bool,
    pub z3_context: Z3_context,
    pub z3_ast: Z3_ast,
}

impl BoolLessThanUIntExpression {
    pub fn new(
        s1: Rc<RefCell<ActiveValue>>,
        s2: Rc<RefCell<ActiveValue>>,
        stdlib: &mut ScfiaStdlib,
    ) -> ActiveValue {
        match s1.try_borrow().unwrap().deref() {
            ActiveValue::BitvectorConcrete(e1) => {
                match s2.try_borrow().unwrap().deref() {
                    ActiveValue::BitvectorConcrete(e2) => {
                        return ActiveValue::BoolConcrete(BoolConcrete::new(e1.value < e2.value, stdlib))
                    },
                    _ => {}
                }
            }
            _ => {}
        }
        ActiveValue::BoolLessThanUIntExpression(Self::new_with_id(stdlib.get_symbol_id(), s1, s2, stdlib))
    }

    pub fn new_with_id(
        id: u64,
        s1: Rc<RefCell<ActiveValue>>,
        s2: Rc<RefCell<ActiveValue>>,
        stdlib: &mut ScfiaStdlib,
    ) -> BoolLessThanUIntExpression {
        unsafe {
            let z3_context = stdlib.z3_context;
            let ast = Z3_mk_bvult(
                stdlib.z3_context,
                s1.try_borrow().unwrap().get_z3_ast(),
                s2.try_borrow().unwrap().get_z3_ast(),
            );
            if id == 1983037 {
                println!("##### {} (s1={}, s2={})",
                    id,
                    s1.try_borrow().unwrap().get_id(),
                    s2.try_borrow().unwrap().get_id(),
                );
            }
            
            debug_assert!(s1.try_borrow().unwrap().get_id() < id);
            debug_assert!(s2.try_borrow().unwrap().get_id() < id);
            Z3_inc_ref(z3_context, ast);
            BoolLessThanUIntExpression {
                id,
                s1: s1,
                s2: s2,
                inherited_asts: BTreeMap::new(),
                discovered_asts: HashMap::new(),
                is_assert: false,
                z3_context: z3_context,
                z3_ast: ast,
            }
        }
    }

    pub fn clone_to_stdlib(
        &self,
        cloned_active_values: &mut HashMap<u64, Rc<RefCell<ActiveValue>>>,
        cloned_retired_values: &mut HashMap<u64, Rc<RefCell<RetiredValue>>>,
        cloned_stdlib: &mut ScfiaStdlib
    ) -> Rc<RefCell<ActiveValue>> {
        // Clone s1, s2
        let s1 = self.s1.try_borrow().unwrap().clone_to_stdlib(cloned_active_values, cloned_retired_values, cloned_stdlib);
        let s2 = self.s2.try_borrow().unwrap().clone_to_stdlib(cloned_active_values, cloned_retired_values, cloned_stdlib);
        if let Some(e) = cloned_active_values.get(&self.id) {
            return e.clone()
        }

        // Build clone
        let cloned_expression = Self::new_with_id(self.id, s1, s2, cloned_stdlib);
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

impl Drop for BoolLessThanUIntExpression {
    fn drop(&mut self) {
        // Retire expression, maintain z3 ast refcount
        let s1_id = self.s1.try_borrow().unwrap().get_id();
        let s2_id = self.s2.try_borrow().unwrap().get_id();
        debug_assert!(s1_id < self.id);
        debug_assert!(s2_id < self.id);
        let retired_expression = Rc::new(RefCell::new(RetiredValue::RetiredBoolLessThanUIntExpression(RetiredBoolLessThanUIntExpression {
            id: self.id,
            s1_id,
            s1: Rc::downgrade(&self.s1),
            s2_id,
            s2: Rc::downgrade(&self.s2),
            is_assert: self.is_assert,
            z3_context: self.z3_context,
            z3_ast: self.z3_ast,
        })));

        let parents = vec![
            (s1_id, self.s1.clone()),
            (s2_id, self.s2.clone()),
        ];

        if self.id == 1983033 {
            println!("### passing on 1983033 to parent {:?}", self.s1)
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

impl RetiredBoolLessThanUIntExpression {
    pub fn clone_to_stdlib(
        &self,
        cloned_active_values: &mut HashMap<u64, Rc<RefCell<ActiveValue>>>,
        cloned_retired_values: &mut HashMap<u64, Rc<RefCell<RetiredValue>>>,
        cloned_stdlib: &mut ScfiaStdlib
    ) -> Rc<RefCell<RetiredValue>> {
        debug_assert!(self.s1_id < self.id);
        debug_assert!(self.s2_id < self.id);
        let s1_ast = if let Some(s1) = cloned_active_values.get(&self.s1_id) {
            s1.try_borrow().unwrap().get_z3_ast()
        } else if let Some(s1) = self.s1.upgrade() {
            s1.try_borrow().unwrap().clone_to_stdlib(cloned_active_values, cloned_retired_values, cloned_stdlib).try_borrow().unwrap().get_z3_ast()
        } else if let Some(s1) = cloned_retired_values.get(&self.s1_id) {
            s1.try_borrow().unwrap().get_z3_ast()
        } else {
            panic!("Retired parent {} not found in cloned_retired_values\n{:?}", self.s1_id, self);
        };

        let s2_ast = if let Some(s2) = cloned_active_values.get(&self.s2_id) {
            s2.try_borrow().unwrap().get_z3_ast()
        } else if let Some(s2) = self.s2.upgrade() {
            s2.try_borrow().unwrap().clone_to_stdlib(cloned_active_values, cloned_retired_values, cloned_stdlib).try_borrow().unwrap().get_z3_ast()
        } else if let Some(s2) = cloned_retired_values.get(&self.s2_id) {
            s2.try_borrow().unwrap().get_z3_ast()
        } else {
            panic!("Retired parent {} not found in cloned_retired_values\n{:?}", self.s2_id, self);
        };

        let cloned: Rc<RefCell<RetiredValue>> = unsafe {
            let z3_ast = Z3_mk_bvult(
                cloned_stdlib.z3_context,
                s1_ast,
                s2_ast,
            );
            Z3_inc_ref(cloned_stdlib.z3_context, z3_ast);
            RetiredBoolLessThanUIntExpression {
                id: self.id,
                is_assert: self.is_assert,
                s1_id: self.s1_id,
                s1: self.s1.clone(),
                s2_id: self.s2_id,
                s2: self.s2.clone(),
                z3_context: cloned_stdlib.z3_context,
                z3_ast
            }
        }.into();

        println!("inserting {} into cloned_retired_values", self.id);
        cloned_retired_values.insert(self.id, cloned.clone());
        cloned
    }
}

impl Drop for RetiredBoolLessThanUIntExpression {
    fn drop(&mut self) {
        unsafe { Z3_dec_ref(self.z3_context, self.z3_ast) }
    }
}
