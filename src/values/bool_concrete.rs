use z3_sys::Z3_context;
use z3_sys::Z3_inc_ref;
use z3_sys::Z3_dec_ref;
use z3_sys::Z3_mk_bool_sort;
use z3_sys::Z3_mk_bv_sort;
use z3_sys::Z3_mk_false;
use z3_sys::Z3_mk_fresh_const;
use z3_sys::Z3_mk_true;
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


pub struct BoolConcrete {
    pub id: u64,
    pub value: bool,
    pub inherited_asts: BTreeMap<u64, Rc<RefCell<RetiredValue>>>,
    pub discovered_asts: HashMap<u64, Weak<RefCell<ActiveValue>>>,
    pub z3_context: Z3_context,
    pub z3_ast: Z3_ast,
}

#[derive(Debug)]
pub struct RetiredBoolConcrete {
    pub id: u64,
    pub value: bool,
    pub z3_context: Z3_context,
    pub z3_ast: Z3_ast,
}

impl BoolConcrete {
    pub fn new(value: bool, stdlib: &mut ScfiaStdlib) -> BoolConcrete {
        Self::new_with_id(stdlib.get_symbol_id(), value, stdlib)
    }

    pub fn new_with_id(id: u64, value: bool, stdlib: &mut ScfiaStdlib) -> BoolConcrete {
        unsafe {
            let ast = if value {
                Z3_mk_true(stdlib.z3_context)
            } else {
                Z3_mk_false(stdlib.z3_context)
            };
            
            Z3_inc_ref(stdlib.z3_context, ast);

            // let z3_ast = stdlib.z3_context
            let bvc = BoolConcrete {
                id,
                value: value,
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
        let clone = Self::new_with_id(self.id, self.value, cloned_stdlib);
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

impl fmt::Debug for BoolConcrete {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(&format!("BoolConcrete {{ value={}, id={} }}", self.value, self.id))
    }
}

impl Drop for BoolConcrete {
    fn drop(&mut self) {
        // Retire expression, maintain z3 ast refcount
        let retired_expression = Rc::new(RefCell::new(RetiredValue::RetiredBoolConcrete(RetiredBoolConcrete {
            id: self.id,
            value: self.value,
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

impl Drop for RetiredBoolConcrete {
    fn drop(&mut self) {
        unsafe { Z3_dec_ref(self.z3_context, self.z3_ast) }
    }
}