use z3_sys::Z3_context;
use z3_sys::Z3_inc_ref;
use z3_sys::Z3_dec_ref;
use z3_sys::Z3_mk_bv_sort;
use z3_sys::Z3_mk_unsigned_int64;
use z3_sys::Z3_ast;
use std::cell::RefCell;
use std::collections::BTreeMap;
use std::collections::HashMap;
use std::fmt;
use std::rc::Rc;
use std::rc::Weak;
use crate::ScfiaStdlib;
use crate::expressions::finish_clone;
use crate::expressions::inherit;

use super::ActiveValue;
use super::RetiredValue;


pub struct BitVectorConcrete {
    pub id: u64,
    pub value: u64,
    pub width: u32,
    pub inherited_asts: BTreeMap<u64, Rc<RefCell<RetiredValue>>>,
    pub discovered_asts: HashMap<u64, Weak<RefCell<ActiveValue>>>,
    pub z3_context: Z3_context,
    pub z3_ast: Z3_ast,
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
    pub fn new(value: u64, width: u32, stdlib: &mut ScfiaStdlib) -> BitVectorConcrete {
        Self::new_with_id(stdlib.get_symbol_id(), value, width, stdlib)
    }

    pub fn new_with_id(id: u64, value: u64, width: u32, stdlib: &mut ScfiaStdlib) -> BitVectorConcrete {
        unsafe {
            let sort = Z3_mk_bv_sort(stdlib.z3_context, width);
            let ast = Z3_mk_unsigned_int64(stdlib.z3_context, value, sort);
            Z3_inc_ref(stdlib.z3_context, ast);

            let bvc = BitVectorConcrete {
                id,
                value: value,
                width: width,
                inherited_asts: BTreeMap::new(),
                discovered_asts: HashMap::new(),
                z3_context: stdlib.z3_context,
                z3_ast: ast,
            };
            bvc
        }
    }

    pub fn clone_to_stdlib(
        &self,
        cloned_active_values: &mut HashMap<u64, Rc<RefCell<ActiveValue>>>,
        cloned_retired_values: &mut HashMap<u64, Rc<RefCell<RetiredValue>>>,
        cloned_stdlib: &mut ScfiaStdlib,
    ) -> Rc<RefCell<ActiveValue>> {
        let clone = Self::new_with_id(self.id, self.value, self.width, cloned_stdlib);
        if self.id == 12920 {
            println!("#### {:?}", self.inherited_asts)
        }
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
        _cloned_active_values: &mut HashMap<u64, Rc<RefCell<ActiveValue>>>,
        cloned_retired_values: &mut HashMap<u64, Rc<RefCell<RetiredValue>>>,
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

        if self.id == 1982653 {
            println!("### Drop BitVectorConcrete {} with inheritance {:?}", self.id, &self.inherited_asts)
        }

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
        unsafe { Z3_dec_ref(self.z3_context, self.z3_ast) }
    }
}
