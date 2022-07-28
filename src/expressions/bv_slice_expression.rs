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
use z3_sys::Z3_mk_extract;

use super::finish_clone;
use super::inherit;

#[derive(Debug)]
pub struct BVSliceExpression {
    pub id: u64,
    pub s1_id: u64,
    pub s1: Rc<RefCell<ActiveValue>>,
    pub high: u32,
    pub low: u32,
    pub inherited_asts: BTreeMap<u64, Rc<RefCell<RetiredValue>>>,
    pub discovered_asts: BTreeMap<u64, Weak<RefCell<ActiveValue>>>,
    pub z3_context: Z3_context,
    pub z3_ast: Z3_ast,
}

#[derive(Debug)]
pub struct RetiredBVSliceExpression {
    pub id: u64,
    s1_id: u64,
    s1: Weak<RefCell<ActiveValue>>,
    high: u32,
    low: u32,
    pub z3_context: Z3_context,
    pub z3_ast: Z3_ast,
}

impl BVSliceExpression {
    pub fn new(
        s1: Rc<RefCell<ActiveValue>>,
        high: u32,
        low: u32,
        stdlib: &mut ScfiaStdlib,
        fork_sink: &mut Option<&mut ForkSink>,
    ) -> Rc<RefCell<ActiveValue>> {
        let value: Rc<RefCell<ActiveValue>> = Self::new_try_concretize(s1, high, low, stdlib, fork_sink).into();
        if let Some(fork_sink) = fork_sink {
            fork_sink.new_values.push(value.clone());
        }
        value
    }

    pub fn new_try_concretize(
        s1: Rc<RefCell<ActiveValue>>,
        high: u32,
        low: u32,
        stdlib: &mut ScfiaStdlib,
        fork_sink: &mut Option<&mut ForkSink>,
    ) -> Rc<RefCell<ActiveValue>> {
        match s1.try_borrow().unwrap().deref() {
            ActiveValue::BitvectorConcrete(e) => {
                let shifted = e.value >> low;
                let width = high - low + 1;
                let mask = (1 << width) - 1;
                let value = shifted & mask;
                return BitVectorConcrete::new(value, width, stdlib, fork_sink);
            }
            _ => {}
        }
        ActiveValue::BitvectorSliceExpression(Self::new_with_id(stdlib.get_symbol_id(), s1, high, low, stdlib)).into()
    }

    pub fn new_with_id(
        id: u64,
        s1: Rc<RefCell<ActiveValue>>,
        high: u32,
        low: u32,
        stdlib: &mut ScfiaStdlib,
    ) -> BVSliceExpression {
        unsafe {
            let z3_context = stdlib.z3_context;
            let s1_ast;
            let s1_id;
            {
                let s1 = s1.try_borrow().unwrap();
                s1_ast = s1.get_z3_ast();
                s1_id = s1.get_id();
            }
            let ast = Z3_mk_extract(
                stdlib.z3_context,
                high,
                low,
                s1_ast,
            );
            Z3_inc_ref(z3_context, ast);
            BVSliceExpression {
                id,
                s1_id,
                s1,
                high,
                low,
                inherited_asts: BTreeMap::new(),
                discovered_asts: BTreeMap::new(),
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
        // Clone s1, s2
        let s1 = self.s1.try_borrow().unwrap().clone_to_stdlib(cloned_active_values, cloned_retired_values, cloned_stdlib);
        if let Some(e) = cloned_active_values.get(&self.id) {
            return e.clone()
        }

        // Build clone
        let cloned_expression = Self::new_with_id(self.id, s1, self.high, self.low, cloned_stdlib);

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

impl Drop for BVSliceExpression {
    fn drop(&mut self) {
        // Retire expression, maintain z3 ast refcount
        let s1_id = self.s1.try_borrow().unwrap().get_id();
        debug_assert!(s1_id < self.id);
        let retired_expression = Rc::new(RefCell::new(RetiredValue::RetiredBitvectorSliceExpression(RetiredBVSliceExpression {
            id: self.id,
            s1_id,
            s1: Rc::downgrade(&self.s1),
            high: self.high,
            low: self.low,
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

impl RetiredBVSliceExpression {
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
            let z3_ast = Z3_mk_extract(
                cloned_stdlib.z3_context,
                self.high,
                self.low,
                cloned_s1_ast,
            );
            Z3_inc_ref(cloned_stdlib.z3_context, z3_ast);
            RetiredBVSliceExpression {
                id: self.id,
                s1_id: self.s1_id,
                s1: cloned_s1,
                high: self.high,
                low: self.low,
                z3_context: cloned_stdlib.z3_context,
                z3_ast
            }
        }.into();

        cloned_retired_values.insert(self.id, cloned.clone());
        cloned
    }
}

impl Drop for RetiredBVSliceExpression {
    fn drop(&mut self) {
        // unsafe { Z3_dec_ref(self.z3_context, self.z3_ast) }
    }
}
