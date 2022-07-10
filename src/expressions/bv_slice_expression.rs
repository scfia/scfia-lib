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
use z3_sys::Z3_mk_extract;

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
    s1: u64,
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
}

impl Drop for BVSliceExpression {
    fn drop(&mut self) {
        // Retire expression, maintain z3 ast refcount
        let s1_id = self.s1.try_borrow().unwrap().get_id();
        debug_assert!(s1_id < self.id);
        let retired_expression = Rc::new(RefCell::new(RetiredValue::RetiredBitvectorSliceExpression(RetiredBVSliceExpression {
            id: self.id,
            s1: s1_id,
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

impl Drop for RetiredBVSliceExpression {
    fn drop(&mut self) {
        // unsafe { Z3_dec_ref(self.z3_context, self.z3_ast) }
    }
}
