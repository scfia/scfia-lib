use z3_sys::Z3_context;
use z3_sys::Z3_inc_ref;
use z3_sys::Z3_dec_ref;
use z3_sys::Z3_mk_bv_sort;
use z3_sys::Z3_mk_unsigned_int64;
use z3_sys::Z3_ast;
use std::cell::RefCell;
use std::collections::HashMap;
use std::fmt;
use std::rc::Rc;
use std::rc::Weak;
use crate::ScfiaStdlib;
use crate::traits::ast::Ast;
use crate::traits::bit_vector::BitVector;

use super::ActiveValue;
use super::RetiredValue;


pub struct BitVectorConcrete {
    pub id: u64,
    pub value: u64,
    pub width: u32,
    pub inherited_asts: Vec<Rc<RefCell<RetiredValue>>>,
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
            // Z3_inc_ref(stdlib.z3_context, sort);

            let ast = Z3_mk_unsigned_int64(stdlib.z3_context, value, sort);
            Z3_inc_ref(stdlib.z3_context, ast);

            // let z3_ast = stdlib.z3_context
            let bvc = BitVectorConcrete {
                id,
                value: value,
                width: width,
                inherited_asts: vec![],
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
        let clone: Rc<RefCell<ActiveValue>> = Self::new_with_id(self.id, self.value, self.width, cloned_stdlib).into();
        cloned_active_values.insert(self.id, clone.clone());
        clone
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

        // Heirs are parents and discovered symbols
        let mut heirs: Vec<Rc<RefCell<ActiveValue>>> = vec![];
        for discovered_symbol in self.discovered_asts.values() {
            let discovered_symbol = discovered_symbol.upgrade().unwrap();
            let mut discovered_symbol_ref = discovered_symbol.try_borrow_mut().unwrap();
            discovered_symbol_ref.forget(self.id);
            heirs.push(discovered_symbol.clone())
        }

        // For each heir...
        for heir in &heirs {
            let mut heir_ref = heir.try_borrow_mut().unwrap();
            
            // Inherit
            heir_ref.inherit(retired_expression.clone());

            // Pass on inherited symbols
            for inherited in &self.inherited_asts {
                heir_ref.inherit(inherited.clone())
            }

            // Acquaint all heirs
            for other_heir in &heirs {
                if let Ok(mut other_heir_ref) = other_heir.try_borrow_mut() {
                    heir_ref.discover(other_heir_ref.get_id(), Rc::downgrade(other_heir));
                    other_heir_ref.discover(heir_ref.get_id(), Rc::downgrade(heir));
                }                
            }
        }
    }
}

impl Drop for RetiredBitvectorConcrete {
    fn drop(&mut self) {
        unsafe { Z3_dec_ref(self.z3_context, self.z3_ast) }
    }
}
