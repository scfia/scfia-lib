use std::{cell::RefCell, rc::{Rc, Weak}, collections::BTreeMap};
use std::collections::HashMap;
use z3_sys::Z3_solver_assert;

use crate::{ScfiaStdlib, expressions::{bool_less_than_uint_expression::{BoolLessThanUIntExpression, RetiredBoolLessThanUIntExpression}, bool_not_expression::{RetiredBoolNotExpression, BoolNotExpression}, bv_and_expression::{BVAndExpression, RetiredBVAndExpression}, bv_shift_right_logical_expression::{BVShiftRightLogicalExpression, RetiredBVShiftRightLogicalExpression}, bv_shift_left_logical_expression::{BVShiftLeftLogicalExpression, RetiredBVShiftLeftLogicalExpression}, bv_xor_expression::{BVXorExpression, RetiredBVXorExpression}, bv_sub_expression::{BVSubExpression, RetiredBVSubExpression}, bool_less_than_signed_expression::{BoolLessThanSignedExpression, RetiredBoolLessThanSignedExpression}}};

use crate::{expressions::{bool_eq_expression::{BoolEqExpression, RetiredBoolEqExpression}, bool_neq_expression::{BoolNEqExpression, RetiredBoolNEqExpression}, bv_add_expression::{BVAddExpression, RetiredBVAddExpression}, bv_concat_expression::{BVConcatExpression, RetiredBVConcatExpression}, bv_or_expression::{BVOrExpression, RetiredBVOrExpression}, bv_sign_extend_expression::{BVSignExtendExpression, RetiredBVSignExtendExpression}, bv_slice_expression::{BVSliceExpression, RetiredBVSliceExpression}}};

use self::{bit_vector_concrete::{BitVectorConcrete, RetiredBitvectorConcrete}, bit_vector_symbol::{BitVectorSymbol, RetiredBitvectorSymbol}, bool_concrete::{BoolConcrete, RetiredBoolConcrete}};

pub mod bit_vector_concrete;
pub mod bit_vector_symbol;
pub mod bool_concrete;

#[derive(Debug)]
pub enum Value {
    Active(ActiveValue),
    Ref(RetiredValue),
}

#[derive(Debug)]
pub enum ActiveValue {
    BitvectorConcrete(BitVectorConcrete),
    BitvectorSymbol(BitVectorSymbol),
    BoolEqExpression(BoolEqExpression),
    BoolLessThanUIntExpression(BoolLessThanUIntExpression),
    BoolLessThanSignedExpression(BoolLessThanSignedExpression),
    BoolNEqExpression(BoolNEqExpression),
    BoolNotExpression(BoolNotExpression),
    BoolConcrete(BoolConcrete),
    BitvectorAddExpression(BVAddExpression),
    BitvectorAndExpression(BVAndExpression),
    BitvectorSubExpression(BVSubExpression),
    BitvectorXorExpression(BVXorExpression),
    BitvectorConcatExpression(BVConcatExpression),
    BitvectorOrExpression(BVOrExpression),
    BitvectorSignExtendExpression(BVSignExtendExpression),
    BitvectorSliceExpression(BVSliceExpression),
    BitvectorShiftRightLogicalExpression(BVShiftRightLogicalExpression),
    BitvectorShiftLeftLogicalExpression(BVShiftLeftLogicalExpression),
}

#[derive(Debug)]
pub enum RetiredValue {
    RetiredBitvectorConcrete(RetiredBitvectorConcrete),
    RetiredBitvectorSymbol(RetiredBitvectorSymbol),
    RetiredBoolEqExpression(RetiredBoolEqExpression),
    RetiredBoolLessThanUIntExpression(RetiredBoolLessThanUIntExpression),
    RetiredBoolLessThanSignedExpression(RetiredBoolLessThanSignedExpression),
    RetiredBoolNEqExpression(RetiredBoolNEqExpression),
    RetiredBoolNotExpression(RetiredBoolNotExpression),
    RetiredBoolConcrete(RetiredBoolConcrete),
    RetiredBitvectorAddExpression(RetiredBVAddExpression),
    RetiredBitvectorSubExpression(RetiredBVSubExpression),
    RetiredBitvectorAndExpression(RetiredBVAndExpression),
    RetiredBitvectorXorExpression(RetiredBVXorExpression),
    RetiredBitvectorConcatExpression(RetiredBVConcatExpression),
    RetiredBitvectorOrExpression(RetiredBVOrExpression),
    RetiredBitvectorSignExtendExpression(RetiredBVSignExtendExpression),
    RetiredBitvectorSliceExpression(RetiredBVSliceExpression),
    RetiredBitvectorShiftRightLogicalExpression(RetiredBVShiftRightLogicalExpression),
    RetiredBitvectorShiftLeftLogicalExpression(RetiredBVShiftLeftLogicalExpression),
}

impl ActiveValue {
    pub fn as_concrete_bitvector(&self) -> &BitVectorConcrete {
        match self {
            ActiveValue::BitvectorConcrete(e) => e,
            _ => panic!()
        }
    }

    pub fn clone_to_stdlib(
        &self,
        cloned_active_values: &mut BTreeMap<u64, Rc<RefCell<ActiveValue>>>,
        cloned_retired_values: &mut BTreeMap<u64, Rc<RefCell<RetiredValue>>>,
        cloned_stdlib: &mut ScfiaStdlib
    ) -> Rc<RefCell<ActiveValue>> {
        if let Some(cloned_active_value) = cloned_active_values.get(&self.get_id()) {
            let clone = cloned_active_value.clone();
            debug_assert_eq!(clone.try_borrow().unwrap().get_id(), self.get_id());
            return clone
        }

        let clone = match self {
            ActiveValue::BitvectorConcrete(e) => e.clone_to_stdlib(cloned_active_values, cloned_retired_values, cloned_stdlib),
            ActiveValue::BitvectorSymbol(e) => e.clone_to_stdlib(cloned_active_values, cloned_retired_values, cloned_stdlib),
            ActiveValue::BoolEqExpression(e) => e.clone_to_stdlib(cloned_active_values, cloned_retired_values, cloned_stdlib),
            ActiveValue::BoolNEqExpression(e) => e.clone_to_stdlib(cloned_active_values, cloned_retired_values, cloned_stdlib),
            ActiveValue::BitvectorAddExpression(e) => e.clone_to_stdlib(cloned_active_values, cloned_retired_values, cloned_stdlib),
            ActiveValue::BitvectorConcatExpression(e) => e.clone_to_stdlib(cloned_active_values, cloned_retired_values, cloned_stdlib),
            ActiveValue::BitvectorOrExpression(e) => e.clone_to_stdlib(cloned_active_values, cloned_retired_values, cloned_stdlib),
            ActiveValue::BitvectorSignExtendExpression(_) => unimplemented!(),
            ActiveValue::BitvectorSliceExpression(e) => e.clone_to_stdlib(cloned_active_values, cloned_retired_values, cloned_stdlib),
            ActiveValue::BoolLessThanUIntExpression(e) => e.clone_to_stdlib(cloned_active_values, cloned_retired_values, cloned_stdlib),
            ActiveValue::BoolNotExpression(e) => e.clone_to_stdlib(cloned_active_values, cloned_retired_values, cloned_stdlib),
            ActiveValue::BitvectorAndExpression(_) => todo!(),
            ActiveValue::BitvectorShiftRightLogicalExpression(e) => e.clone_to_stdlib(cloned_active_values, cloned_retired_values, cloned_stdlib),
            ActiveValue::BitvectorShiftLeftLogicalExpression(e) => e.clone_to_stdlib(cloned_active_values, cloned_retired_values, cloned_stdlib),
            ActiveValue::BoolConcrete(e) => e.clone_to_stdlib(cloned_active_values, cloned_retired_values, cloned_stdlib),
            ActiveValue::BitvectorXorExpression(e) => e.clone_to_stdlib(cloned_active_values, cloned_retired_values, cloned_stdlib),
            ActiveValue::BitvectorSubExpression(e) => e.clone_to_stdlib(cloned_active_values, cloned_retired_values, cloned_stdlib),
            ActiveValue::BoolLessThanSignedExpression(e) => e.clone_to_stdlib(cloned_active_values, cloned_retired_values, cloned_stdlib),
        };
        clone
    }

    pub fn assert(&mut self, stdlib: &mut ScfiaStdlib) {
        match self {
            ActiveValue::BoolEqExpression(x) => x.assert(stdlib),
            ActiveValue::BoolNEqExpression(x) => x.assert(stdlib),
            ActiveValue::BoolNotExpression(x) => x.assert(stdlib),
            ActiveValue::BoolLessThanUIntExpression(x) => x.assert(stdlib),
            ActiveValue::BoolLessThanSignedExpression(x) => x.assert(stdlib),
            x => panic!("{:?}", &x),
        }
    }
}

impl RetiredValue {
    pub fn clone_to_stdlib(
        &self,
        cloned_active_values: &mut BTreeMap<u64, Rc<RefCell<ActiveValue>>>,
        cloned_retired_values: &mut BTreeMap<u64, Rc<RefCell<RetiredValue>>>,
        cloned_stdlib: &mut ScfiaStdlib
    ) -> Rc<RefCell<RetiredValue>> {
        if let Some(cloned_retired_value) = cloned_retired_values.get(&self.get_id()) {
            let clone = cloned_retired_value.clone();
            debug_assert_eq!(clone.try_borrow().unwrap().get_id(), self.get_id());
            return clone
        }

        match self {
            RetiredValue::RetiredBitvectorConcrete(_) => {},
            x => {}, //println!("clone_to_stdlib {} RetiredValue {:?}", cloned_stdlib.id, &x),
        }

        match self {
            RetiredValue::RetiredBitvectorConcrete(x) => x.clone_to_stdlib(cloned_active_values, cloned_retired_values, cloned_stdlib),
            RetiredValue::RetiredBitvectorSymbol(x) => x.clone_to_stdlib(cloned_active_values, cloned_retired_values, cloned_stdlib),
            RetiredValue::RetiredBoolEqExpression(x) => x.clone_to_stdlib(cloned_active_values, cloned_retired_values, cloned_stdlib),
            RetiredValue::RetiredBoolNEqExpression(x) => x.clone_to_stdlib(cloned_active_values, cloned_retired_values, cloned_stdlib),
            RetiredValue::RetiredBitvectorAddExpression(x) => x.clone_to_stdlib(cloned_active_values, cloned_retired_values, cloned_stdlib),
            RetiredValue::RetiredBitvectorConcatExpression(_) => todo!(),
            RetiredValue::RetiredBitvectorOrExpression(_) => todo!(),
            RetiredValue::RetiredBitvectorSignExtendExpression(_) => todo!(),
            RetiredValue::RetiredBitvectorSliceExpression(_) => todo!(),
            RetiredValue::RetiredBoolLessThanUIntExpression(x) => x.clone_to_stdlib(cloned_active_values, cloned_retired_values, cloned_stdlib),
            RetiredValue::RetiredBoolNotExpression(x) => x.clone_to_stdlib(cloned_active_values, cloned_retired_values, cloned_stdlib),
            RetiredValue::RetiredBitvectorAndExpression(_) => todo!(),
            RetiredValue::RetiredBitvectorShiftRightLogicalExpression(x) => x.clone_to_stdlib(cloned_active_values, cloned_retired_values, cloned_stdlib),
            RetiredValue::RetiredBitvectorShiftLeftLogicalExpression(_) => todo!(),
            RetiredValue::RetiredBoolConcrete(_) => todo!(),
            RetiredValue::RetiredBitvectorXorExpression(_) => todo!(),
            RetiredValue::RetiredBitvectorSubExpression(_) => todo!(),
            RetiredValue::RetiredBoolLessThanSignedExpression(x) => x.clone_to_stdlib(cloned_active_values, cloned_retired_values, cloned_stdlib),
        }
    }

    pub fn get_id(&self) -> u64 {
        match self {
            RetiredValue::RetiredBitvectorConcrete(e) => e.id,
            RetiredValue::RetiredBitvectorSymbol(e) => e.id,
            RetiredValue::RetiredBoolEqExpression(e) => e.id,
            RetiredValue::RetiredBoolNEqExpression(e) => e.id,
            RetiredValue::RetiredBitvectorAddExpression(e) => e.id,
            RetiredValue::RetiredBitvectorConcatExpression(e) => e.id,
            RetiredValue::RetiredBitvectorOrExpression(e) => e.id,
            RetiredValue::RetiredBitvectorSignExtendExpression(e) => e.id,
            RetiredValue::RetiredBitvectorSliceExpression(e) => e.id,
            RetiredValue::RetiredBoolLessThanUIntExpression(e) => e.id,
            RetiredValue::RetiredBoolNotExpression(e) => e.id,
            RetiredValue::RetiredBitvectorAndExpression(e) => e.id,
            RetiredValue::RetiredBitvectorShiftRightLogicalExpression(e) => e.id,
            RetiredValue::RetiredBitvectorShiftLeftLogicalExpression(e) => e.id,
            RetiredValue::RetiredBoolConcrete(e) => e.id,
            RetiredValue::RetiredBitvectorXorExpression(e) => e.id,
            RetiredValue::RetiredBitvectorSubExpression(e) => e.id,
            RetiredValue::RetiredBoolLessThanSignedExpression(e) => e.id,
        }
    }

    pub fn get_z3_ast(&self) -> z3_sys::Z3_ast {
        match self {
            RetiredValue::RetiredBitvectorConcrete(e) => e.z3_ast,
            RetiredValue::RetiredBitvectorSymbol(e) => e.z3_ast,
            RetiredValue::RetiredBoolEqExpression(e) => e.z3_ast,
            RetiredValue::RetiredBoolNEqExpression(e) => e.z3_ast,
            RetiredValue::RetiredBitvectorAddExpression(e) => e.z3_ast,
            RetiredValue::RetiredBitvectorConcatExpression(e) => e.z3_ast,
            RetiredValue::RetiredBitvectorOrExpression(e) => e.z3_ast,
            RetiredValue::RetiredBitvectorSignExtendExpression(e) => e.z3_ast,
            RetiredValue::RetiredBitvectorSliceExpression(e) => e.z3_ast,
            RetiredValue::RetiredBoolLessThanUIntExpression(e) => e.z3_ast,
            RetiredValue::RetiredBoolNotExpression(e) => e.z3_ast,
            RetiredValue::RetiredBitvectorAndExpression(e) => e.z3_ast,
            RetiredValue::RetiredBitvectorShiftRightLogicalExpression(e) => e.z3_ast,
            RetiredValue::RetiredBitvectorShiftLeftLogicalExpression(e) => e.z3_ast,
            RetiredValue::RetiredBoolConcrete(e) => e.z3_ast,
            RetiredValue::RetiredBitvectorXorExpression(e) => e.z3_ast,
            RetiredValue::RetiredBitvectorSubExpression(e) => e.z3_ast,
            RetiredValue::RetiredBoolLessThanSignedExpression(e) => e.z3_ast,
        }
    }
}

impl ActiveValue {
    pub fn get_id(&self) -> u64 {
        match self {
            ActiveValue::BitvectorConcrete(e) => e.id,
            ActiveValue::BitvectorSymbol(e) => e.id,
            ActiveValue::BoolEqExpression(e) => e.id,
            ActiveValue::BoolNEqExpression(e) => e.id,
            ActiveValue::BitvectorAddExpression(e) => e.id,
            ActiveValue::BitvectorConcatExpression(e) => e.id,
            ActiveValue::BitvectorOrExpression(e) => e.id,
            ActiveValue::BitvectorSignExtendExpression(e) => e.id,
            ActiveValue::BitvectorSliceExpression(e) => e.id,
            ActiveValue::BoolLessThanUIntExpression(e) => e.id,
            ActiveValue::BoolNotExpression(e) => e.id,
            ActiveValue::BitvectorAndExpression(e) => e.id,
            ActiveValue::BitvectorShiftRightLogicalExpression(e) => e.id,
            ActiveValue::BitvectorShiftLeftLogicalExpression(e) => e.id,
            ActiveValue::BoolConcrete(e) => e.id,
            ActiveValue::BitvectorXorExpression(e) => e.id,
            ActiveValue::BitvectorSubExpression(e) => e.id,
            ActiveValue::BoolLessThanSignedExpression(e) => e.id,
        }
    }

    pub fn get_z3_ast(&self) -> z3_sys::Z3_ast {
        match self {
            ActiveValue::BitvectorConcrete(e) => e.z3_ast,
            ActiveValue::BitvectorSymbol(e) => e.z3_ast,
            ActiveValue::BoolEqExpression(e) => e.z3_ast,
            ActiveValue::BoolNEqExpression(e) => e.z3_ast,
            ActiveValue::BitvectorAddExpression(e) => e.z3_ast,
            ActiveValue::BitvectorConcatExpression(e) => e.z3_ast,
            ActiveValue::BitvectorOrExpression(e) => e.z3_ast,
            ActiveValue::BitvectorSignExtendExpression(e) => e.z3_ast,
            ActiveValue::BitvectorSliceExpression(e) => e.z3_ast,
            ActiveValue::BoolLessThanUIntExpression(e) => e.z3_ast,
            ActiveValue::BoolNotExpression(e) => e.z3_ast,
            ActiveValue::BitvectorAndExpression(e) => e.z3_ast,
            ActiveValue::BitvectorShiftRightLogicalExpression(e) => e.z3_ast,
            ActiveValue::BitvectorShiftLeftLogicalExpression(e) => e.z3_ast,
            ActiveValue::BoolConcrete(e) => e.z3_ast,
            ActiveValue::BitvectorXorExpression(e) => e.z3_ast,
            ActiveValue::BitvectorSubExpression(e) => e.z3_ast,
            ActiveValue::BoolLessThanSignedExpression(e) => e.z3_ast,
        }
    }

    pub fn inherit(&mut self, ast_id: u64, ast: Rc<RefCell<RetiredValue>>) {
        debug_assert_ne!(self.get_id(), ast_id);
        match self {
            ActiveValue::BitvectorConcrete(e) => e.inherit(ast_id, ast),
            ActiveValue::BitvectorSymbol(e) => { e.inherited_asts.insert(ast_id, ast); },
            ActiveValue::BoolEqExpression(e) => { e.inherited_asts.insert(ast_id, ast); },
            ActiveValue::BoolNEqExpression(e) => { e.inherited_asts.insert(ast_id, ast); },
            ActiveValue::BitvectorAddExpression(e) => { e.inherited_asts.insert(ast_id, ast); },
            ActiveValue::BitvectorConcatExpression(e) => { e.inherited_asts.insert(ast_id, ast); },
            ActiveValue::BitvectorOrExpression(e) => { e.inherited_asts.insert(ast_id, ast); },
            ActiveValue::BitvectorSignExtendExpression(e) =>{ e.inherited_asts.insert(ast_id, ast); },
            ActiveValue::BitvectorSliceExpression(e) => { e.inherited_asts.insert(ast_id, ast); },
            ActiveValue::BoolLessThanUIntExpression(e) => { e.inherited_asts.insert(ast_id, ast); },
            ActiveValue::BoolNotExpression(e) => { e.inherited_asts.insert(ast_id, ast); },
            ActiveValue::BitvectorAndExpression(e) => { e.inherited_asts.insert(ast_id, ast); },
            ActiveValue::BitvectorShiftRightLogicalExpression(e) => { e.inherited_asts.insert(ast_id, ast); },
            ActiveValue::BitvectorShiftLeftLogicalExpression(e) => { e.inherited_asts.insert(ast_id, ast); },
            ActiveValue::BoolConcrete(e) => { e.inherited_asts.insert(ast_id, ast); },
            ActiveValue::BitvectorXorExpression(e) => { e.inherited_asts.insert(ast_id, ast); },
            ActiveValue::BitvectorSubExpression(e) => { e.inherited_asts.insert(ast_id, ast); },
            ActiveValue::BoolLessThanSignedExpression(e) => { e.inherited_asts.insert(ast_id, ast); },
        }
    }

    pub fn forget(&mut self, id: u64) {
        match self {
            ActiveValue::BitvectorConcrete(e) => { assert!(e.discovered_asts.remove(&id).is_some()); },
            ActiveValue::BitvectorSymbol(e) => { assert!(e.discovered_asts.remove(&id).is_some()); },
            ActiveValue::BoolEqExpression(e) => { assert!(e.discovered_asts.remove(&id).is_some()); },
            ActiveValue::BoolNEqExpression(e) => { assert!(e.discovered_asts.remove(&id).is_some()); },
            ActiveValue::BitvectorAddExpression(e) => { assert!(e.discovered_asts.remove(&id).is_some()); },
            ActiveValue::BitvectorConcatExpression(e) => { assert!(e.discovered_asts.remove(&id).is_some()); },
            ActiveValue::BitvectorOrExpression(e) => { assert!(e.discovered_asts.remove(&id).is_some()); },
            ActiveValue::BitvectorSignExtendExpression(e) => { assert!(e.discovered_asts.remove(&id).is_some()); },
            ActiveValue::BitvectorSliceExpression(e) => { assert!(e.discovered_asts.remove(&id).is_some()); },
            ActiveValue::BoolLessThanUIntExpression(e) => { assert!(e.discovered_asts.remove(&id).is_some()); },
            ActiveValue::BoolNotExpression(e) => { assert!(e.discovered_asts.remove(&id).is_some()); },
            ActiveValue::BitvectorAndExpression(e) => { assert!(e.discovered_asts.remove(&id).is_some()); },
            ActiveValue::BitvectorShiftRightLogicalExpression(e) => { assert!(e.discovered_asts.remove(&id).is_some()); },
            ActiveValue::BitvectorShiftLeftLogicalExpression(e) => { assert!(e.discovered_asts.remove(&id).is_some()); },
            ActiveValue::BoolConcrete(e) => { assert!(e.discovered_asts.remove(&id).is_some()); },
            ActiveValue::BitvectorXorExpression(e) => { assert!(e.discovered_asts.remove(&id).is_some()); },
            ActiveValue::BitvectorSubExpression(e) => { assert!(e.discovered_asts.remove(&id).is_some()); },
            ActiveValue::BoolLessThanSignedExpression(e) => { assert!(e.discovered_asts.remove(&id).is_some()); },
        }
    }

    pub fn discover(&mut self, ast_id: u64, ast: Weak<RefCell<ActiveValue>>) {
        if self.get_id() == 16012 {
            println!("{} discovering {}", self.get_id(), ast_id);
        }
        match self {
            ActiveValue::BitvectorConcrete(e) => { e.discovered_asts.insert(ast_id, ast); },
            ActiveValue::BitvectorSymbol(e) => { e.discovered_asts.insert(ast_id, ast); },
            ActiveValue::BoolEqExpression(e) => { e.discovered_asts.insert(ast_id, ast); },
            ActiveValue::BoolNEqExpression(e) => { e.discovered_asts.insert(ast_id, ast); },
            ActiveValue::BitvectorAddExpression(e) => { e.discovered_asts.insert(ast_id, ast); },
            ActiveValue::BitvectorConcatExpression(e) => { e.discovered_asts.insert(ast_id, ast); },
            ActiveValue::BitvectorOrExpression(e) => { e.discovered_asts.insert(ast_id, ast); },
            ActiveValue::BitvectorSignExtendExpression(e) => { e.discovered_asts.insert(ast_id, ast); },
            ActiveValue::BitvectorSliceExpression(e) => { e.discovered_asts.insert(ast_id, ast); },
            ActiveValue::BoolLessThanUIntExpression(e) => { e.discovered_asts.insert(ast_id, ast); },
            ActiveValue::BoolNotExpression(e) => { e.discovered_asts.insert(ast_id, ast); },
            ActiveValue::BitvectorAndExpression(e) => { e.discovered_asts.insert(ast_id, ast); },
            ActiveValue::BitvectorShiftRightLogicalExpression(e) => { e.discovered_asts.insert(ast_id, ast); },
            ActiveValue::BitvectorShiftLeftLogicalExpression(e) => { e.discovered_asts.insert(ast_id, ast); },
            ActiveValue::BoolConcrete(e) => { e.discovered_asts.insert(ast_id, ast); },
            ActiveValue::BitvectorXorExpression(e) => { e.discovered_asts.insert(ast_id, ast); },
            ActiveValue::BitvectorSubExpression(e) => { e.discovered_asts.insert(ast_id, ast); },
            ActiveValue::BoolLessThanSignedExpression(e) => { e.discovered_asts.insert(ast_id, ast); },
        }
    }
}

impl From<ActiveValue> for Rc<RefCell<ActiveValue>> {
    fn from(s: ActiveValue) -> Self {
        Rc::new(RefCell::new(s))
    }
}

impl From<BitVectorConcrete> for Rc<RefCell<ActiveValue>> {
    fn from(s: BitVectorConcrete) -> Self {
        Rc::new(RefCell::new(ActiveValue::BitvectorConcrete(s)))
    }
}

impl From<BitVectorSymbol> for Rc<RefCell<ActiveValue>> {
    fn from(s: BitVectorSymbol) -> Self {
        Rc::new(RefCell::new(ActiveValue::BitvectorSymbol(s)))
    }
}

impl From<BoolEqExpression> for Rc<RefCell<ActiveValue>> {
    fn from(s: BoolEqExpression) -> Self {
        Rc::new(RefCell::new(ActiveValue::BoolEqExpression(s)))
    }
}

impl From<BoolLessThanUIntExpression> for Rc<RefCell<ActiveValue>> {
    fn from(s: BoolLessThanUIntExpression) -> Self {
        Rc::new(RefCell::new(ActiveValue::BoolLessThanUIntExpression(s)))
    }
}

impl From<BoolLessThanSignedExpression> for Rc<RefCell<ActiveValue>> {
    fn from(s: BoolLessThanSignedExpression) -> Self {
        Rc::new(RefCell::new(ActiveValue::BoolLessThanSignedExpression(s)))
    }
}

impl From<BoolNEqExpression> for Rc<RefCell<ActiveValue>> {
    fn from(s: BoolNEqExpression) -> Self {
        Rc::new(RefCell::new(ActiveValue::BoolNEqExpression(s)))
    }
}

impl From<BoolNotExpression> for Rc<RefCell<ActiveValue>> {
    fn from(s: BoolNotExpression) -> Self {
        Rc::new(RefCell::new(ActiveValue::BoolNotExpression(s)))
    }
}

impl From<BoolConcrete> for Rc<RefCell<ActiveValue>> {
    fn from(s: BoolConcrete) -> Self {
        Rc::new(RefCell::new(ActiveValue::BoolConcrete(s)))
    }
}

impl From<BVAddExpression> for Rc<RefCell<ActiveValue>> {
    fn from(s: BVAddExpression) -> Self {
        Rc::new(RefCell::new(ActiveValue::BitvectorAddExpression(s)))
    }
}

impl From<BVSubExpression> for Rc<RefCell<ActiveValue>> {
    fn from(s: BVSubExpression) -> Self {
        Rc::new(RefCell::new(ActiveValue::BitvectorSubExpression(s)))
    }
}

impl From<BVConcatExpression> for Rc<RefCell<ActiveValue>> {
    fn from(s: BVConcatExpression) -> Self {
        Rc::new(RefCell::new(ActiveValue::BitvectorConcatExpression(s)))
    }
}

impl From<BVOrExpression> for Rc<RefCell<ActiveValue>> {
    fn from(s: BVOrExpression) -> Self {
        Rc::new(RefCell::new(ActiveValue::BitvectorOrExpression(s)))
    }
}

impl From<BVSignExtendExpression> for Rc<RefCell<ActiveValue>> {
    fn from(s: BVSignExtendExpression) -> Self {
        Rc::new(RefCell::new(ActiveValue::BitvectorSignExtendExpression(s)))
    }
}

impl From<BVShiftRightLogicalExpression> for Rc<RefCell<ActiveValue>> {
    fn from(s: BVShiftRightLogicalExpression) -> Self {
        Rc::new(RefCell::new(ActiveValue::BitvectorShiftRightLogicalExpression(s)))
    }
}

impl From<BVShiftLeftLogicalExpression> for Rc<RefCell<ActiveValue>> {
    fn from(s: BVShiftLeftLogicalExpression) -> Self {
        Rc::new(RefCell::new(ActiveValue::BitvectorShiftLeftLogicalExpression(s)))
    }
}

impl From<BVSliceExpression> for Rc<RefCell<ActiveValue>> {
    fn from(s: BVSliceExpression) -> Self {
        Rc::new(RefCell::new(ActiveValue::BitvectorSliceExpression(s)))
    }
}

impl From<BVXorExpression> for Rc<RefCell<ActiveValue>> {
    fn from(s: BVXorExpression) -> Self {
        Rc::new(RefCell::new(ActiveValue::BitvectorXorExpression(s)))
    }
}

impl From<RetiredValue> for Rc<RefCell<RetiredValue>> {
    fn from(s: RetiredValue) -> Self {
        Rc::new(RefCell::new(s))
    }
}

impl From<RetiredBitvectorConcrete> for Rc<RefCell<RetiredValue>> {
    fn from(s: RetiredBitvectorConcrete) -> Self {
        Rc::new(RefCell::new(RetiredValue::RetiredBitvectorConcrete(s)))
    }
}

impl From<RetiredBitvectorSymbol> for Rc<RefCell<RetiredValue>> {
    fn from(s: RetiredBitvectorSymbol) -> Self {
        Rc::new(RefCell::new(RetiredValue::RetiredBitvectorSymbol(s)))
    }
}

impl From<RetiredBoolEqExpression> for Rc<RefCell<RetiredValue>> {
    fn from(s: RetiredBoolEqExpression) -> Self {
        Rc::new(RefCell::new(RetiredValue::RetiredBoolEqExpression(s)))
    }
}

impl From<RetiredBoolLessThanUIntExpression> for Rc<RefCell<RetiredValue>> {
    fn from(s: RetiredBoolLessThanUIntExpression) -> Self {
        Rc::new(RefCell::new(RetiredValue::RetiredBoolLessThanUIntExpression(s)))
    }
}

impl From<RetiredBoolLessThanSignedExpression> for Rc<RefCell<RetiredValue>> {
    fn from(s: RetiredBoolLessThanSignedExpression) -> Self {
        Rc::new(RefCell::new(RetiredValue::RetiredBoolLessThanSignedExpression(s)))
    }
}

impl From<RetiredBoolNEqExpression> for Rc<RefCell<RetiredValue>> {
    fn from(s: RetiredBoolNEqExpression) -> Self {
        Rc::new(RefCell::new(RetiredValue::RetiredBoolNEqExpression(s)))
    }
}

impl From<RetiredBoolNotExpression> for Rc<RefCell<RetiredValue>> {
    fn from(s: RetiredBoolNotExpression) -> Self {
        Rc::new(RefCell::new(RetiredValue::RetiredBoolNotExpression(s)))
    }
}

impl From<RetiredBoolConcrete> for Rc<RefCell<RetiredValue>> {
    fn from(s: RetiredBoolConcrete) -> Self {
        Rc::new(RefCell::new(RetiredValue::RetiredBoolConcrete(s)))
    }
}

impl From<RetiredBVAddExpression> for Rc<RefCell<RetiredValue>> {
    fn from(s: RetiredBVAddExpression) -> Self {
        Rc::new(RefCell::new(RetiredValue::RetiredBitvectorAddExpression(s)))
    }
}

impl From<RetiredBVSubExpression> for Rc<RefCell<RetiredValue>> {
    fn from(s: RetiredBVSubExpression) -> Self {
        Rc::new(RefCell::new(RetiredValue::RetiredBitvectorSubExpression(s)))
    }
}

impl From<RetiredBVConcatExpression> for Rc<RefCell<RetiredValue>> {
    fn from(s: RetiredBVConcatExpression) -> Self {
        Rc::new(RefCell::new(RetiredValue::RetiredBitvectorConcatExpression(s)))
    }
}

impl From<RetiredBVOrExpression> for Rc<RefCell<RetiredValue>> {
    fn from(s: RetiredBVOrExpression) -> Self {
        Rc::new(RefCell::new(RetiredValue::RetiredBitvectorOrExpression(s)))
    }
}

impl From<RetiredBVSignExtendExpression> for Rc<RefCell<RetiredValue>> {
    fn from(s: RetiredBVSignExtendExpression) -> Self {
        Rc::new(RefCell::new(RetiredValue::RetiredBitvectorSignExtendExpression(s)))
    }
}

impl From<RetiredBVSliceExpression> for Rc<RefCell<RetiredValue>> {
    fn from(s: RetiredBVSliceExpression) -> Self {
        Rc::new(RefCell::new(RetiredValue::RetiredBitvectorSliceExpression(s)))
    }
}

impl From<RetiredBVXorExpression> for Rc<RefCell<RetiredValue>> {
    fn from(s: RetiredBVXorExpression) -> Self {
        Rc::new(RefCell::new(RetiredValue::RetiredBitvectorXorExpression(s)))
    }
}

impl From<RetiredBVShiftRightLogicalExpression> for Rc<RefCell<RetiredValue>> {
    fn from(s: RetiredBVShiftRightLogicalExpression) -> Self {
        Rc::new(RefCell::new(RetiredValue::RetiredBitvectorShiftRightLogicalExpression(s)))
    }
}

impl From<RetiredBVShiftLeftLogicalExpression> for Rc<RefCell<RetiredValue>> {
    fn from(s: RetiredBVShiftLeftLogicalExpression) -> Self {
        Rc::new(RefCell::new(RetiredValue::RetiredBitvectorShiftLeftLogicalExpression(s)))
    }
}
