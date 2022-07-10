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
use z3_sys::Z3_mk_sign_ext;

use super::inherit;

#[derive(Debug)]
pub struct BVSignExtendExpression {
    pub id: u64,
    pub s1_id: u64,
    pub s1: Rc<RefCell<ActiveValue>>,
    pub input_width: u32,
    pub output_width: u32,
    pub inherited_asts: BTreeMap<u64, Rc<RefCell<RetiredValue>>>,
    pub discovered_asts: BTreeMap<u64, Weak<RefCell<ActiveValue>>>,
    pub z3_context: Z3_context,
    pub z3_ast: Z3_ast,
}

#[derive(Debug)]
pub struct RetiredBVSignExtendExpression {
    pub id: u64,
    s1: u64,
    input_width: u32,
    output_width: u32,
    pub z3_context: Z3_context,
    pub z3_ast: Z3_ast,
}

impl BVSignExtendExpression {
    pub fn new(
        s1: Rc<RefCell<ActiveValue>>,
        input_width: u32,
        output_width: u32,
        stdlib: &mut ScfiaStdlib,
        fork_sink: &mut Option<&mut ForkSink>,
    ) -> Rc<RefCell<ActiveValue>> {
        let value: Rc<RefCell<ActiveValue>> = Self::new_try_concretize(s1, input_width, output_width, stdlib, fork_sink).into();
        if let Some(fork_sink) = fork_sink {
            fork_sink.new_values.push(value.clone());
        }
        value
    }

    pub fn new_try_concretize(
        s1: Rc<RefCell<ActiveValue>>,
        input_width: u32,
        output_width: u32,
        stdlib: &mut ScfiaStdlib,
        fork_sink: &mut Option<&mut ForkSink>,
    ) -> Rc<RefCell<ActiveValue>> {
        match s1.try_borrow().unwrap().deref() {
            ActiveValue::BitvectorConcrete(e) => {
                // https://graphics.stanford.edu/~seander/bithacks.html#VariableSignExtend
                let m: u64 = 1 << (e.width - 1);
                let x = e.value & ((1 << e.width) - 1);
                let value = (x ^ m).overflowing_sub(m).0;
                return BitVectorConcrete::new(value, output_width, stdlib, fork_sink);
            }
            _ => {}
        }
        ActiveValue::BitvectorSignExtendExpression(Self::new_with_id(stdlib.get_symbol_id(), s1, input_width, output_width, stdlib)).into()
    }

    pub fn new_with_id(
        id: u64,
        s1: Rc<RefCell<ActiveValue>>,
        input_width: u32,
        output_width: u32,
        stdlib: &mut ScfiaStdlib,
    ) -> BVSignExtendExpression {
        unsafe {
            let z3_context = stdlib.z3_context;
            let s1_ast;
            let s1_id;
            {
                let s1 = s1.try_borrow().unwrap();
                s1_ast = s1.get_z3_ast();
                s1_id = s1.get_id();
            }
            let ast = Z3_mk_sign_ext(
                stdlib.z3_context,
                output_width - input_width,
                s1_ast,
            );
            Z3_inc_ref(z3_context, ast);
            BVSignExtendExpression {
                id: id,
                s1_id,
                s1: s1,
                input_width,
                output_width,
                inherited_asts: BTreeMap::new(),
                discovered_asts: BTreeMap::new(),
                z3_context: z3_context,
                z3_ast: ast,
            }
        }
    }
}

impl Drop for BVSignExtendExpression {
    fn drop(&mut self) {
        // Retire expression, maintain z3 ast refcount
        let s1_id = self.s1.try_borrow().unwrap().get_id();
        debug_assert!(s1_id < self.id);
        let retired_expression = Rc::new(RefCell::new(RetiredValue::RetiredBitvectorSignExtendExpression(RetiredBVSignExtendExpression {
            id: self.id,
            s1: s1_id,
            input_width: self.input_width,
            output_width: self.output_width,
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

impl Drop for RetiredBVSignExtendExpression {
    fn drop(&mut self) {
        // unsafe { Z3_dec_ref(self.z3_context, self.z3_ast) }
    }
}

