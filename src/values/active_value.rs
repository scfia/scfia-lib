use std::cmp::max;
use std::fmt::Debug;
use std::{
    cell::RefCell,
    collections::BTreeMap,
    rc::{Rc, Weak},
};

use log::{debug, trace};

use crate::scfia::Scfia;
use crate::z3_handle::Z3Ast;
use crate::ScfiaComposition;

use super::bool_eq_expression::BoolEqExpression;
use super::bool_not_expresssion::BoolNotExpression;
use super::bool_signed_less_than_expression::BoolSignedLessThanExpression;
use super::bool_unsigned_less_than_expression::BoolUnsignedLessThanExpression;
use super::bv_add_expression::BVAddExpression;
use super::bv_and_expression::BVAndExpression;
use super::bv_concat_expression::BVConcatExpression;
use super::bv_concrete_expression::BVConcreteExpression;
use super::bv_multiply_expression::BVMultiplyExpression;
use super::bv_or_expression::BVOrExpression;
use super::bv_sign_extend_expression::BVSignExtendExpression;
use super::bv_slice_expression::BVSliceExpression;
use super::bv_sll_expression::BVSllExpression;
use super::bv_srl_expression::BVSrlExpression;
use super::bv_sub_expression::BVSubExpression;
use super::bv_symbol::BVSymbol;
use super::bv_unsigned_remainder_expression::BVUnsignedRemainderExpression;
use super::bv_xor_expression::BVXorExpression;

use super::retired_value::RetiredValue;

#[derive(Clone, Debug)]
pub struct ValueComment {
    pub message: String,
}

impl ValueComment {
    pub fn new(message: String) -> Self {
        ValueComment { message }
    }
}

#[derive(Clone, Debug)]
pub enum ActiveValue<SC: ScfiaComposition> {
    BoolConcrete(bool),
    BVConcrete(u64, u32),
    Expression(Rc<RefCell<ActiveValueZ3<SC>>>),
}

#[derive(Debug)]
pub struct ActiveValueZ3<SC: ScfiaComposition> {
    pub id: u64,
    pub z3_ast: Z3Ast<SC>,
    pub expression: ActiveExpression<SC>,
    pub inherited_asts: BTreeMap<u64, RetiredValue<SC>>,
    pub discovered_asts: BTreeMap<u64, Weak<RefCell<ActiveValueZ3<SC>>>>,
    pub scfia: Weak<Scfia<SC>>,
    pub comment: Option<ValueComment>,
    pub can_inherit: bool,
}

#[derive(Debug)]
pub enum ActiveExpression<SC: ScfiaComposition> {
    BoolEqExpression(BoolEqExpression<SC>),
    BoolNotExpression(BoolNotExpression<SC>),
    BoolSignedLessThanExpression(BoolSignedLessThanExpression<SC>),
    BoolUnsignedLessThanExpression(BoolUnsignedLessThanExpression<SC>),
    BVAddExpression(BVAddExpression<SC>),
    BVAndExpression(BVAndExpression<SC>),
    BVConcatExpression(BVConcatExpression<SC>),
    BVConcreteExpression(BVConcreteExpression<SC>),
    BVMultiplyExpression(BVMultiplyExpression<SC>),
    BVOrExpression(BVOrExpression<SC>),
    BVSignExtendExpression(BVSignExtendExpression<SC>),
    BVSliceExpression(BVSliceExpression<SC>),
    BVSllExpression(BVSllExpression<SC>),
    BVSrlExpression(BVSrlExpression<SC>),
    BVSubExpression(BVSubExpression<SC>),
    BVSymbol(BVSymbol),
    BVUnsignedRemainderExpression(BVUnsignedRemainderExpression<SC>),
    BVXorExpression(BVXorExpression<SC>),
}

impl<SC: ScfiaComposition> ActiveValue<SC> {
    pub fn get_depth(&self) -> usize {
        match self {
            ActiveValue::Expression(e) => e.try_borrow().unwrap().get_depth(),
            _ => 1,
        }
    }

    pub fn get_z3_ast(&self) -> Z3Ast<SC> {
        match &self {
            ActiveValue::Expression(e) => e.try_borrow().unwrap().z3_ast.clone(),
            _ => panic!(),
        }
    }

    pub fn get_weak(&self) -> Weak<RefCell<ActiveValueZ3<SC>>> {
        if let ActiveValue::Expression(e) = &self {
            Rc::downgrade(e)
        } else {
            panic!()
        }
    }

    pub fn get_z3_value(&self) -> Rc<RefCell<ActiveValueZ3<SC>>> {
        if let ActiveValue::Expression(e) = &self {
            e.clone()
        } else {
            panic!()
        }
    }

    pub fn into_z3_value(&self, scfia: &Scfia<SC>, fork_sink: &mut Option<SC::ForkSink>) -> ActiveValue<SC> {
        match &self {
            ActiveValue::BoolConcrete(_) => todo!(), //TODO should this ever happen?
            ActiveValue::BVConcrete(value, width) => scfia.new_bv_concrete_z3(*value, *width, None, fork_sink, None),
            ActiveValue::Expression(e) => ActiveValue::Expression(e.clone()),
        }
    }

    pub fn set_can_inherit(&self, value: bool) {
        if let ActiveValue::Expression(e) = self {
            e.try_borrow_mut().unwrap().can_inherit = value
        } else {
            panic!()
        }
    }

    pub fn inherit(&self, inactive_value_id: u64, inactive_value: RetiredValue<SC>) {
        if let ActiveValue::Expression(e) = &self {
            e.try_borrow_mut().unwrap().inherited_asts.insert(inactive_value_id, inactive_value);
        } else {
            panic!()
        }
    }

    pub fn discover(&self, active_value_id: u64, active_value: Weak<RefCell<ActiveValueZ3<SC>>>) {
        if let ActiveValue::Expression(e) = &self {
            e.try_borrow_mut().unwrap().discovered_asts.insert(active_value_id, active_value);
        } else {
            panic!()
        }
    }

    pub fn assert(&self, _scfia: &Scfia<SC>) {
        debug!("asserting {:?}", self); // TODO
        self.get_z3_ast().assert()
    }

    pub(crate) fn clone_to_stdlib(
        &self,
        cloned_scfia: &Scfia<SC>,
        cloned_actives: &mut BTreeMap<u64, ActiveValue<SC>>,
        cloned_retired: &mut BTreeMap<u64, RetiredValue<SC>>,
    ) -> ActiveValue<SC> {
        match self {
            ActiveValue::BoolConcrete(value) => ActiveValue::BoolConcrete(*value),
            ActiveValue::BVConcrete(value, width) => ActiveValue::BVConcrete(*value, *width),
            ActiveValue::Expression(expression) => expression.try_borrow().unwrap().clone_to_stdlib(cloned_scfia, cloned_actives, cloned_retired),
        }
    }
}

impl<SC: ScfiaComposition> ActiveValueZ3<SC> {
    pub(crate) fn get_parents(&self, dest: &mut Vec<Rc<RefCell<ActiveValueZ3<SC>>>>) {
        match &self.expression {
            ActiveExpression::BVSymbol(_) => {}
            ActiveExpression::BVAddExpression(e) => {
                dest.push(e.s1.clone());
                dest.push(e.s2.clone());
            }
            ActiveExpression::BoolEqExpression(e) => {
                dest.push(e.s1.clone());
                dest.push(e.s2.clone());
            }
            ActiveExpression::BoolNotExpression(e) => {
                dest.push(e.s1.clone());
            }
            ActiveExpression::BoolSignedLessThanExpression(e) => {
                dest.push(e.s1.clone());
                dest.push(e.s2.clone());
            }
            ActiveExpression::BoolUnsignedLessThanExpression(e) => {
                dest.push(e.s1.clone());
                dest.push(e.s2.clone());
            }
            ActiveExpression::BVAndExpression(e) => {
                dest.push(e.s1.clone());
                dest.push(e.s2.clone());
            }
            ActiveExpression::BVConcatExpression(e) => {
                dest.push(e.s1.clone());
                dest.push(e.s2.clone());
            }
            ActiveExpression::BVMultiplyExpression(e) => {
                dest.push(e.s1.clone());
                dest.push(e.s2.clone());
            }
            ActiveExpression::BVOrExpression(e) => {
                dest.push(e.s1.clone());
                dest.push(e.s2.clone());
            }
            ActiveExpression::BVSignExtendExpression(e) => {
                dest.push(e.s1.clone());
            }
            ActiveExpression::BVSliceExpression(e) => {
                dest.push(e.s1.clone());
            }
            ActiveExpression::BVSllExpression(e) => {
                dest.push(e.s1.clone());
                dest.push(e.s2.clone());
            }
            ActiveExpression::BVSrlExpression(e) => {
                dest.push(e.s1.clone());
                dest.push(e.s2.clone());
            }
            ActiveExpression::BVSubExpression(e) => {
                dest.push(e.s1.clone());
                dest.push(e.s2.clone());
            }
            ActiveExpression::BVUnsignedRemainderExpression(e) => {
                dest.push(e.s1.clone());
                dest.push(e.s2.clone());
            }
            ActiveExpression::BVXorExpression(e) => {
                dest.push(e.s1.clone());
                dest.push(e.s2.clone());
            }
            ActiveExpression::BVConcreteExpression(_) => {}
        }
    }

    pub fn get_depth(&self) -> usize {
        match &self.expression {
            ActiveExpression::BoolEqExpression(e) => 1 + max(e.s1.try_borrow().unwrap().get_depth(), e.s1.try_borrow().unwrap().get_depth()),
            ActiveExpression::BoolNotExpression(e) => 1 + e.s1.try_borrow().unwrap().get_depth(),
            ActiveExpression::BoolSignedLessThanExpression(e) => 1 + max(e.s1.try_borrow().unwrap().get_depth(), e.s1.try_borrow().unwrap().get_depth()),
            ActiveExpression::BoolUnsignedLessThanExpression(e) => 1 + max(e.s1.try_borrow().unwrap().get_depth(), e.s1.try_borrow().unwrap().get_depth()),
            ActiveExpression::BVAddExpression(e) => 1 + max(e.s1.try_borrow().unwrap().get_depth(), e.s1.try_borrow().unwrap().get_depth()),
            ActiveExpression::BVAndExpression(e) => 1 + max(e.s1.try_borrow().unwrap().get_depth(), e.s1.try_borrow().unwrap().get_depth()),
            ActiveExpression::BVConcatExpression(e) => 1 + max(e.s1.try_borrow().unwrap().get_depth(), e.s1.try_borrow().unwrap().get_depth()),
            ActiveExpression::BVMultiplyExpression(e) => 1 + max(e.s1.try_borrow().unwrap().get_depth(), e.s1.try_borrow().unwrap().get_depth()),
            ActiveExpression::BVOrExpression(e) => 1 + max(e.s1.try_borrow().unwrap().get_depth(), e.s1.try_borrow().unwrap().get_depth()),
            ActiveExpression::BVSignExtendExpression(e) => 1 + e.s1.try_borrow().unwrap().get_depth(),
            ActiveExpression::BVSliceExpression(e) => 1 + e.s1.try_borrow().unwrap().get_depth(),
            ActiveExpression::BVSllExpression(e) => 1 + max(e.s1.try_borrow().unwrap().get_depth(), e.s1.try_borrow().unwrap().get_depth()),
            ActiveExpression::BVSrlExpression(e) => 1 + max(e.s1.try_borrow().unwrap().get_depth(), e.s1.try_borrow().unwrap().get_depth()),
            ActiveExpression::BVSubExpression(e) => 1 + max(e.s1.try_borrow().unwrap().get_depth(), e.s1.try_borrow().unwrap().get_depth()),
            ActiveExpression::BVSymbol(_) => 1,
            ActiveExpression::BVUnsignedRemainderExpression(e) => 1 + max(e.s1.try_borrow().unwrap().get_depth(), e.s1.try_borrow().unwrap().get_depth()),
            ActiveExpression::BVXorExpression(e) => 1 + max(e.s1.try_borrow().unwrap().get_depth(), e.s1.try_borrow().unwrap().get_depth()),
            ActiveExpression::BVConcreteExpression(_) => 1,
        }
    }

    pub fn get_inactives(&self) -> usize {
        self.inherited_asts.len()
    }

    pub fn is_concrete(&self) -> bool {
        matches!(self.expression, ActiveExpression::BVConcreteExpression(_))
    }

    pub(crate) fn clone_to_stdlib(
        &self,
        cloned_scfia: &Scfia<SC>,
        cloned_actives: &mut BTreeMap<u64, ActiveValue<SC>>,
        cloned_retired: &mut BTreeMap<u64, RetiredValue<SC>>,
    ) -> ActiveValue<SC> {
        if let Some(value) = cloned_actives.get(&self.id) {
            return value.clone();
        }
        trace!("Cloning {:?}", self);
        let cloned_value = match &self.expression {
            ActiveExpression::BoolEqExpression(e) => {
                let s1 = e.s1.try_borrow().unwrap().clone_to_stdlib(cloned_scfia, cloned_actives, cloned_retired);
                let s2 = e.s2.try_borrow().unwrap().clone_to_stdlib(cloned_scfia, cloned_actives, cloned_retired);
                cloned_scfia.new_bool_eq(&s1, &s2, Some(self.id), e.is_assert, &mut None, self.comment.clone())
            }
            ActiveExpression::BoolNotExpression(e) => {
                let s1 = e.s1.try_borrow().unwrap().clone_to_stdlib(cloned_scfia, cloned_actives, cloned_retired);
                cloned_scfia.new_bool_not(&s1, Some(self.id), e.is_assert, &mut None, self.comment.clone())
            }
            ActiveExpression::BoolSignedLessThanExpression(e) => {
                let s1 = e.s1.try_borrow().unwrap().clone_to_stdlib(cloned_scfia, cloned_actives, cloned_retired);
                let s2 = e.s2.try_borrow().unwrap().clone_to_stdlib(cloned_scfia, cloned_actives, cloned_retired);
                cloned_scfia.new_bool_signed_less_than(&s1, &s2, Some(self.id), e.is_assert, &mut None, self.comment.clone())
            }
            ActiveExpression::BoolUnsignedLessThanExpression(e) => {
                let s1 = e.s1.try_borrow().unwrap().clone_to_stdlib(cloned_scfia, cloned_actives, cloned_retired);
                let s2 = e.s2.try_borrow().unwrap().clone_to_stdlib(cloned_scfia, cloned_actives, cloned_retired);
                cloned_scfia.new_bool_unsigned_less_than(&s1, &s2, Some(self.id), e.is_assert, &mut None, self.comment.clone())
            }
            ActiveExpression::BVAddExpression(e) => {
                let s1 = e.s1.try_borrow().unwrap().clone_to_stdlib(cloned_scfia, cloned_actives, cloned_retired);
                let s2 = e.s2.try_borrow().unwrap().clone_to_stdlib(cloned_scfia, cloned_actives, cloned_retired);
                cloned_scfia.new_bv_add(&s1, &s2, e.width, Some(self.id), &mut None, self.comment.clone())
            }
            ActiveExpression::BVAndExpression(e) => {
                let s1 = e.s1.try_borrow().unwrap().clone_to_stdlib(cloned_scfia, cloned_actives, cloned_retired);
                let s2 = e.s2.try_borrow().unwrap().clone_to_stdlib(cloned_scfia, cloned_actives, cloned_retired);
                cloned_scfia.new_bv_and(&s1, &s2, e.width, Some(self.id), &mut None, self.comment.clone())
            }
            ActiveExpression::BVConcatExpression(e) => {
                let s1 = e.s1.try_borrow().unwrap().clone_to_stdlib(cloned_scfia, cloned_actives, cloned_retired);
                let s2 = e.s2.try_borrow().unwrap().clone_to_stdlib(cloned_scfia, cloned_actives, cloned_retired);
                cloned_scfia.new_bv_concat(&s1, &s2, e.width, Some(self.id), &mut None, self.comment.clone())
            }
            ActiveExpression::BVMultiplyExpression(e) => {
                let s1 = e.s1.try_borrow().unwrap().clone_to_stdlib(cloned_scfia, cloned_actives, cloned_retired);
                let s2 = e.s2.try_borrow().unwrap().clone_to_stdlib(cloned_scfia, cloned_actives, cloned_retired);
                cloned_scfia.new_bv_multiply(&s1, &s2, e.width, Some(self.id), &mut None, self.comment.clone())
            }
            ActiveExpression::BVOrExpression(e) => {
                let s1 = e.s1.try_borrow().unwrap().clone_to_stdlib(cloned_scfia, cloned_actives, cloned_retired);
                let s2 = e.s2.try_borrow().unwrap().clone_to_stdlib(cloned_scfia, cloned_actives, cloned_retired);
                cloned_scfia.new_bv_or(&s1, &s2, e.width, Some(self.id), &mut None, self.comment.clone())
            }
            ActiveExpression::BVSignExtendExpression(e) => {
                let s1 = e.s1.try_borrow().unwrap().clone_to_stdlib(cloned_scfia, cloned_actives, cloned_retired);
                cloned_scfia.new_bv_sign_extend(&s1, e.input_width, e.width, Some(self.id), &mut None, self.comment.clone())
            }
            ActiveExpression::BVSliceExpression(e) => {
                let s1 = e.s1.try_borrow().unwrap().clone_to_stdlib(cloned_scfia, cloned_actives, cloned_retired);
                cloned_scfia.new_bv_slice(&s1, e.high, e.low, Some(self.id), &mut None, self.comment.clone())
            }
            ActiveExpression::BVSllExpression(e) => {
                let s1 = e.s1.try_borrow().unwrap().clone_to_stdlib(cloned_scfia, cloned_actives, cloned_retired);
                let s2 = e.s2.try_borrow().unwrap().clone_to_stdlib(cloned_scfia, cloned_actives, cloned_retired);
                cloned_scfia.new_bv_sll(&s1, &s2, e.width, Some(self.id), &mut None, self.comment.clone())
            }
            ActiveExpression::BVSrlExpression(e) => {
                let s1 = e.s1.try_borrow().unwrap().clone_to_stdlib(cloned_scfia, cloned_actives, cloned_retired);
                let s2 = e.s2.try_borrow().unwrap().clone_to_stdlib(cloned_scfia, cloned_actives, cloned_retired);
                cloned_scfia.new_bv_srl(&s1, &s2, e.width, Some(self.id), &mut None, self.comment.clone())
            }
            ActiveExpression::BVSubExpression(e) => {
                let s1 = e.s1.try_borrow().unwrap().clone_to_stdlib(cloned_scfia, cloned_actives, cloned_retired);
                let s2 = e.s2.try_borrow().unwrap().clone_to_stdlib(cloned_scfia, cloned_actives, cloned_retired);
                cloned_scfia.new_bv_sub(&s1, &s2, e.width, Some(self.id), &mut None, self.comment.clone())
            }
            ActiveExpression::BVSymbol(e) => cloned_scfia.new_bv_symbol(e.width, Some(self.id), &mut None, self.comment.clone()),
            ActiveExpression::BVUnsignedRemainderExpression(e) => {
                let s1 = e.s1.try_borrow().unwrap().clone_to_stdlib(cloned_scfia, cloned_actives, cloned_retired);
                let s2 = e.s2.try_borrow().unwrap().clone_to_stdlib(cloned_scfia, cloned_actives, cloned_retired);
                cloned_scfia.new_bv_unsigned_remainder(&s1, &s2, e.width, Some(self.id), &mut None, self.comment.clone())
            }
            ActiveExpression::BVXorExpression(e) => {
                let s1 = e.s1.try_borrow().unwrap().clone_to_stdlib(cloned_scfia, cloned_actives, cloned_retired);
                let s2 = e.s2.try_borrow().unwrap().clone_to_stdlib(cloned_scfia, cloned_actives, cloned_retired);
                cloned_scfia.new_bv_xor(&s1, &s2, e.width, Some(self.id), &mut None, self.comment.clone())
            }
            ActiveExpression::BVConcreteExpression(e) => cloned_scfia.new_bv_concrete_z3(e.value, e.width, Some(self.id), &mut None, self.comment.clone()),
        };

        cloned_value.set_can_inherit(self.can_inherit);
        cloned_actives.insert(self.id, cloned_value.clone());

        // Clone inherited values
        for (inherited_value_id, inherited_value) in &self.inherited_asts {
            let inherited_value_ref = inherited_value.try_borrow().unwrap();
            let cloned_inherited_value = inherited_value_ref.clone_to_stdlib(cloned_scfia, cloned_actives, cloned_retired);
            cloned_value.inherit(*inherited_value_id, cloned_inherited_value);
        }

        // Clone discovered values
        for (discovered_value_id, discovered_value) in &self.discovered_asts {
            let cloned_discovered_value =
                discovered_value
                    .upgrade()
                    .unwrap()
                    .try_borrow()
                    .unwrap()
                    .clone_to_stdlib(cloned_scfia, cloned_actives, cloned_retired);
            cloned_value.discover(*discovered_value_id, cloned_discovered_value.get_weak());
        }

        cloned_value
    }
}

impl<SC: ScfiaComposition> Drop for ActiveValueZ3<SC> {
    fn drop(&mut self) {
        if let Some(scfia) = self.scfia.upgrade() {
            scfia.drop_active_expression(self);
        }
    }
}

pub trait ActiveValueImpl<SC: ScfiaComposition> {
    fn to_u64(&self) -> u64;
}

impl<SC: ScfiaComposition> ActiveValueImpl<SC> for ActiveValue<SC> {
    fn to_u64(&self) -> u64 {
        if let ActiveValue::BVConcrete(e, _) = &self {
            *e
        } else {
            panic!()
        }
    }
}
