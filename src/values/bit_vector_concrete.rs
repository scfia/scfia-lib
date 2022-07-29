use z3_sys::Z3_context;
use z3_sys::Z3_inc_ref;
use z3_sys::Z3_dec_ref;
use z3_sys::Z3_mk_bv_sort;
use z3_sys::Z3_mk_unsigned_int64;
use z3_sys::Z3_ast;
use std::cell::RefCell;
use std::collections::BTreeMap;
use std::fmt;
use std::rc::Rc;
use std::rc::Weak;
use std::time::SystemTime;
use std::time::UNIX_EPOCH;
use crate::ScfiaStdlib;
use crate::expressions::finish_clone;
use crate::expressions::inherit;
use crate::models::riscv::rv32i::ForkSink;

use super::ActiveValue;
use super::RetiredValue;


pub struct BitVectorConcrete {
    pub id: u64,
    pub stdlib_id: String,
    pub value: u64,
    pub width: u32,
    pub inherited_asts: BTreeMap<u64, Rc<RefCell<RetiredValue>>>,
    pub discovered_asts: BTreeMap<u64, Weak<RefCell<ActiveValue>>>,
    pub z3_context: Z3_context,
    pub z3_ast: Z3_ast,
    pub depth: u64,
}

#[derive(Debug)]
pub struct RetiredBitvectorConcrete {
    pub id: u64,
    pub value: u64,
    pub width: u32,
    pub z3_context: Z3_context,
    pub z3_ast: Z3_ast,
}

impl BitVectorConcrete {
    pub fn new(
        value: u64,
        width: u32,
        stdlib: &mut ScfiaStdlib,
        fork_sink: &mut Option<&mut ForkSink>
    ) -> Rc<RefCell<ActiveValue>> {
        let value: Rc<RefCell<ActiveValue>> = Self::new_with_id(stdlib.get_symbol_id(), stdlib.id.clone(), value, width, stdlib).into();
        if let Some(fork_sink) = fork_sink {
            fork_sink.new_values.push(value.clone());
        }
        value
    }

    pub fn new_with_id(
        id: u64,
        stdlib_id: String,
        value: u64,
        width: u32,
        stdlib: &mut ScfiaStdlib
    ) -> BitVectorConcrete {
        unsafe {
            let sort = Z3_mk_bv_sort(stdlib.z3_context, width);
            let ast = Z3_mk_unsigned_int64(stdlib.z3_context, value, sort);
            Z3_inc_ref(stdlib.z3_context, ast);

            let bvc = BitVectorConcrete {
                id,
                stdlib_id,
                value: value,
                width: width,
                inherited_asts: BTreeMap::new(),
                discovered_asts: BTreeMap::new(),
                z3_context: stdlib.z3_context,
                z3_ast: ast,
                depth: 1,
            };
            bvc
        }
    }

    pub fn inherit(&mut self, ast_id: u64, ast: Rc<RefCell<RetiredValue>>) {
        // self.inherited_asts.insert(ast_id, ast);
    }

    pub fn clone_to_stdlib(
        &self,
        cloned_active_values: &mut BTreeMap<u64, Rc<RefCell<ActiveValue>>>,
        cloned_retired_values: &mut BTreeMap<u64, Rc<RefCell<RetiredValue>>>,
        cloned_stdlib: &mut ScfiaStdlib,
    ) -> Rc<RefCell<ActiveValue>> {
        let clone = Self::new_with_id(self.id, cloned_stdlib.id.clone(), self.value, self.width, cloned_stdlib);
        finish_clone(
            self.id,
            &self.inherited_asts,
            &self.discovered_asts,
            clone.into(),
            cloned_active_values,
            cloned_retired_values,
            cloned_stdlib
        )
    }
}

impl RetiredBitvectorConcrete {
    pub fn clone_to_stdlib(
        &self,
        _cloned_active_values: &mut BTreeMap<u64, Rc<RefCell<ActiveValue>>>,
        cloned_retired_values: &mut BTreeMap<u64, Rc<RefCell<RetiredValue>>>,
        cloned_stdlib: &mut ScfiaStdlib
    ) -> Rc<RefCell<RetiredValue>> {
        unsafe {
            let sort = Z3_mk_bv_sort(cloned_stdlib.z3_context, self.width);
            let ast = Z3_mk_unsigned_int64(cloned_stdlib.z3_context, self.value, sort);
            Z3_inc_ref(cloned_stdlib.z3_context, ast);

            let cloned: Rc<RefCell<RetiredValue>> = RetiredBitvectorConcrete {
                id: self.id,
                value: self.value,
                width: self.width,
                z3_context: cloned_stdlib.z3_context,
                z3_ast: ast,
            }.into();

            cloned_retired_values.insert(self.id, cloned.clone());
            cloned
        }
    }
}

impl fmt::Debug for BitVectorConcrete {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(&format!("BitVectorConcrete {{ value=0x{:x}, width={}, id={} }}", self.value, self.width, self.id))
    }
}

impl Drop for BitVectorConcrete {
    fn drop(&mut self) {
        // Retire expression, maintain z3 ast refcount
        let retired_expression = Rc::new(RefCell::new(RetiredValue::RetiredBitvectorConcrete(RetiredBitvectorConcrete {
            id: self.id,
            value: self.value,
            width: self.width,
            z3_context: self.z3_context,
            z3_ast: self.z3_ast,
        })));

        inherit(
            self.id,
            retired_expression,
            vec![],
            &self.inherited_asts,
            &self.discovered_asts
        );
    }
}

impl Drop for RetiredBitvectorConcrete {
    fn drop(&mut self) {
        // unsafe { Z3_dec_ref(self.z3_context, self.z3_ast) }
    }
}
