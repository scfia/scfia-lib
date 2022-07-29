use crate::ScfiaStdlib;
use crate::models::riscv::rv32i::ForkSink;
use crate::values::ActiveValue;
use crate::values::RetiredValue;
use crate::values::bit_vector_concrete::BitVectorConcrete;
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

use super::finish_clone;
use super::inherit;

#[derive(Debug)]
pub struct BVAddExpression {
    pub id: u64,
    pub s1: Rc<RefCell<ActiveValue>>,
    pub s2: Rc<RefCell<ActiveValue>>,
    pub inherited_asts: BTreeMap<u64, Rc<RefCell<RetiredValue>>>,
    pub discovered_asts: BTreeMap<u64, Weak<RefCell<ActiveValue>>>,
    pub z3_context: Z3_context,
    pub z3_ast: Z3_ast,
    pub depth: u64,
}

#[derive(Debug)]
pub struct RetiredBVAddExpression {
    pub id: u64,
    s1_id: u64,
    s1: Weak<RefCell<ActiveValue>>,
    s2_id: u64,
    s2: Weak<RefCell<ActiveValue>>,
    pub z3_context: Z3_context,
    pub z3_ast: Z3_ast,
}

impl BVAddExpression {
    pub fn new(
        s1: Rc<RefCell<ActiveValue>>,
        s2: Rc<RefCell<ActiveValue>>,
        stdlib: &mut ScfiaStdlib,
        fork_sink: &mut Option<&mut ForkSink>,
    ) -> Rc<RefCell<ActiveValue>> {
        let value: Rc<RefCell<ActiveValue>> = Self::new_try_concretize(s1, s2, stdlib, fork_sink);
        if let Some(fork_sink) = fork_sink {
            fork_sink.new_values.push(value.clone());
        }
        value
    }

    pub fn new_try_concretize(
        s1: Rc<RefCell<ActiveValue>>,
        s2: Rc<RefCell<ActiveValue>>,
        stdlib: &mut ScfiaStdlib,
        fork_sink: &mut Option<&mut ForkSink>,
    ) -> Rc<RefCell<ActiveValue>> {
        match s1.try_borrow().unwrap().deref() {
            ActiveValue::BitvectorConcrete(e1) => {
                match s2.try_borrow().unwrap().deref() {
                    ActiveValue::BitvectorConcrete(e2) => {
                        // TODO check
                        let one: u64 = 1;
                        let mask = one.rotate_left(e2.width).overflowing_sub(1).0;
                        let sum = e1.value.overflowing_add(e2.value).0; 
                        let value = mask & sum;
                        return BitVectorConcrete::new(value, e2.width, stdlib, fork_sink);
                    },
                    _ => {}
                }
            }
            _ => {}
        }
        ActiveValue::BitvectorAddExpression(Self::new_with_id(stdlib.get_symbol_id(), s1,  s2, stdlib)).into()
    }

    pub fn new_with_id(
        id: u64,
        s1: Rc<RefCell<ActiveValue>>,
        s2: Rc<RefCell<ActiveValue>>,
        stdlib: &mut ScfiaStdlib,
    ) -> BVAddExpression {
        unsafe {
            let z3_context = stdlib.z3_context;
            let ast = Z3_mk_bvadd(
                stdlib.z3_context,
                s1.try_borrow().unwrap().get_z3_ast(),
                s2.try_borrow().unwrap().get_z3_ast(),
            );
            Z3_inc_ref(z3_context, ast);
            let depth = 1 + std::cmp::max(s1.try_borrow().unwrap().get_depth(), s2.try_borrow().unwrap().get_depth());
            if depth > super::MAX_DEPTH {
                println!("s1={}", s1.try_borrow().unwrap().to_json());
                println!("s2={}", s2.try_borrow().unwrap().to_json());
                panic!("Depth too big:\n{:?}\n{:?}", s1, s2)
            }
            BVAddExpression {
                id: id,
                s1: s1,
                s2: s2,
                inherited_asts: BTreeMap::new(),
                discovered_asts: BTreeMap::new(),
                z3_context: z3_context,
                z3_ast: ast,
                depth,
            }
        }
    }

    pub fn clone_to_stdlib(
        &self,
        cloned_active_values: &mut BTreeMap<u64, Rc<RefCell<ActiveValue>>>,
        cloned_retired_values: &mut BTreeMap<u64, Rc<RefCell<RetiredValue>>>,
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
}

impl Drop for BVAddExpression {
    fn drop(&mut self) {
        // Retire expression, maintain z3 ast refcount
        let s1_id = self.s1.try_borrow().unwrap().get_id();
        let s2_id = self.s2.try_borrow().unwrap().get_id();
        debug_assert!(s1_id < self.id);
        debug_assert!(s2_id < self.id);

        let retired_expression = Rc::new(RefCell::new(RetiredValue::RetiredBitvectorAddExpression(RetiredBVAddExpression {
            id: self.id,
            s1_id,
            s1: Rc::downgrade(&self.s1),
            s2_id,
            s2: Rc::downgrade(&self.s2),
            z3_context: self.z3_context,
            z3_ast: self.z3_ast,
        })));

        let parents = vec![
            (s1_id, self.s1.clone()),
            (s2_id, self.s2.clone()),
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

impl RetiredBVAddExpression {
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

        let cloned_s2_ast;
        let cloned_s2;
        if let Some(s2_rc) = cloned_active_values.get(&self.s2_id) {
            let s2 = s2_rc.try_borrow().unwrap();
            cloned_s2_ast = s2.get_z3_ast();
            cloned_s2 = Rc::downgrade(s2_rc);
        } else if let Some(s2_rc) = self.s2.upgrade() {
            let cloned_s2_rc = s2_rc.try_borrow().unwrap().clone_to_stdlib(cloned_active_values, cloned_retired_values, cloned_stdlib);
            cloned_s2_ast = cloned_s2_rc.try_borrow().unwrap().get_z3_ast();
            cloned_s2 = Rc::downgrade(&cloned_s2_rc);
        } else {
            let cloned_s2_rc = cloned_retired_values.get(&self.s2_id).unwrap();
            cloned_s2_ast = cloned_s2_rc.try_borrow().unwrap().get_z3_ast();
            cloned_s2 = Weak::new();
        };

        let cloned: Rc<RefCell<RetiredValue>> = unsafe {
            let z3_ast = Z3_mk_bvadd(
                cloned_stdlib.z3_context,
                cloned_s1_ast,
                cloned_s2_ast,
            );
            Z3_inc_ref(cloned_stdlib.z3_context, z3_ast);
            RetiredBVAddExpression {
                id: self.id,
                s1_id: self.s1_id,
                s1: cloned_s1,
                s2_id: self.s2_id,
                s2: cloned_s2,
                z3_context: cloned_stdlib.z3_context,
                z3_ast
            }
        }.into();

        cloned_retired_values.insert(self.id, cloned.clone());
        cloned
    }
}

impl Drop for RetiredBVAddExpression {
    fn drop(&mut self) {
        // unsafe { Z3_dec_ref(self.z3_context, self.z3_ast) }
    }
}
