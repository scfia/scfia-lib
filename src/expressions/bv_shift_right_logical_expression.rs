use crate::ScfiaStdlib;
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
use z3_sys::Z3_mk_bvand;
use z3_sys::Z3_mk_bvlshr;
use z3_sys::Z3_mk_zero_ext;

use super::finish_clone;
use super::inherit;

#[derive(Debug)]
pub struct BVShiftRightLogicalExpression {
    pub id: u64,
    pub s1: Rc<RefCell<ActiveValue>>,
    pub s2: Rc<RefCell<ActiveValue>>,
    pub input_width: u32,
    pub shamt_width: u32,
    pub inherited_asts: BTreeMap<u64, Rc<RefCell<RetiredValue>>>,
    pub discovered_asts: BTreeMap<u64, Weak<RefCell<ActiveValue>>>,
    pub z3_context: Z3_context,
    pub z3_ast: Z3_ast,
}

#[derive(Debug)]
pub struct RetiredBVShiftRightLogicalExpression {
    pub id: u64,
    s1_id: u64,
    s1: Weak<RefCell<ActiveValue>>,
    s2_id: u64,
    s2: Weak<RefCell<ActiveValue>>,
    input_width: u32,
    shamt_width: u32,
    pub z3_context: Z3_context,
    pub z3_ast: Z3_ast,
}

impl BVShiftRightLogicalExpression {
    pub fn new(
        s1: Rc<RefCell<ActiveValue>>,
        s2: Rc<RefCell<ActiveValue>>,
        input_width: u32,
        shamt_width: u32,
        stdlib: &mut ScfiaStdlib,
    ) -> ActiveValue {
        match s1.try_borrow().unwrap().deref() {
            ActiveValue::BitvectorConcrete(e1) => {
                match s2.try_borrow().unwrap().deref() {
                    ActiveValue::BitvectorConcrete(e2) => {
                        let value = e1.value >> e2.value;
                        return ActiveValue::BitvectorConcrete(BitVectorConcrete::new(value, e1.width, stdlib));
                    },
                    _ => {}
                }
            }
            _ => {}
        }
        ActiveValue::BitvectorShiftRightLogicalExpression(Self::new_with_id(stdlib.get_symbol_id(), s1, s2, input_width, shamt_width, stdlib))
    }

    pub fn new_with_id(
        id: u64,
        s1: Rc<RefCell<ActiveValue>>,
        s2: Rc<RefCell<ActiveValue>>,
        input_width: u32,
        shamt_width: u32,
        stdlib: &mut ScfiaStdlib,
    ) -> BVShiftRightLogicalExpression {
        unsafe {
            let z3_context = stdlib.z3_context;
            let ast = Z3_mk_bvlshr(
                stdlib.z3_context,
                s1.try_borrow().unwrap().get_z3_ast(),
                Z3_mk_zero_ext(
                    stdlib.z3_context,
                    input_width - shamt_width,
                    s2.try_borrow().unwrap().get_z3_ast()),
            );
            Z3_inc_ref(z3_context, ast);
            BVShiftRightLogicalExpression {
                id: id,
                s1: s1,
                s2: s2,
                input_width,
                shamt_width,
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
        let s2 = self.s2.try_borrow().unwrap().clone_to_stdlib(cloned_active_values, cloned_retired_values, cloned_stdlib);
        if let Some(e) = cloned_active_values.get(&self.id) {
            return e.clone()
        }

        // Build clone
        let cloned_expression = Self::new_with_id(self.id, s1, s2, self.input_width, self.shamt_width, cloned_stdlib);

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

impl RetiredBVShiftRightLogicalExpression {
    pub fn clone_to_stdlib(
        &self,
        cloned_active_values: &mut BTreeMap<u64, Rc<RefCell<ActiveValue>>>,
        cloned_retired_values: &mut BTreeMap<u64, Rc<RefCell<RetiredValue>>>,
        cloned_stdlib: &mut ScfiaStdlib
    ) -> Rc<RefCell<RetiredValue>> {
        // println!("RetiredBoolNotExpression getting parent ast {}", self.s1_id);
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
            let z3_ast = Z3_mk_bvlshr(
                cloned_stdlib.z3_context,
                s1_ast,
                Z3_mk_zero_ext(
                    cloned_stdlib.z3_context,
                    self.input_width - self.shamt_width,
                    s2_ast),
            );
            Z3_inc_ref(cloned_stdlib.z3_context, z3_ast);
            RetiredBVShiftRightLogicalExpression {
                id: self.id,
                s1_id: self.s1_id,
                s1: self.s1.clone(),
                s2_id: self.s2_id,
                s2: self.s2.clone(),
                input_width: self.input_width,
                shamt_width: self.shamt_width,
                z3_context: cloned_stdlib.z3_context,
                z3_ast,
            }
        }.into();

        cloned_retired_values.insert(self.id, cloned.clone());
        cloned
    }
}

impl Drop for BVShiftRightLogicalExpression {
    fn drop(&mut self) {
        // Retire expression, maintain z3 ast refcount
        let s1_id = self.s1.try_borrow().unwrap().get_id();
        let s2_id = self.s2.try_borrow().unwrap().get_id();
        debug_assert!(s1_id < self.id);
        debug_assert!(s2_id < self.id);

        let retired_expression = Rc::new(RefCell::new(RetiredValue::RetiredBitvectorShiftRightLogicalExpression(RetiredBVShiftRightLogicalExpression {
            id: self.id,
            s1_id,
            s1: Rc::downgrade(&self.s1),
            s2_id,
            s2: Rc::downgrade(&self.s2),
            input_width: self.input_width,
            shamt_width: self.shamt_width,
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

impl Drop for RetiredBVShiftRightLogicalExpression {
    fn drop(&mut self) {
        unsafe { Z3_dec_ref(self.z3_context, self.z3_ast) }
    }
}
