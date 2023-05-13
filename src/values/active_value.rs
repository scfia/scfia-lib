use std::fmt::Debug;
use std::{
    cell::RefCell,
    collections::BTreeMap,
    rc::{Rc, Weak},
};

use log::trace;
use z3_sys::Z3_ast;

use crate::scfia::{Scfia, ScfiaInner};
use crate::ScfiaComposition;

use super::bool_concrete::BoolConcrete;
use super::bool_eq_expression::BoolEqExpression;
use super::bool_not_expresssion::BoolNotExpression;
use super::bool_signed_less_than_expression::BoolSignedLessThanExpression;
use super::bool_unsigned_less_than_expression::BoolUnsignedLessThanExpression;
use super::bv_add_expression::BVAddExpression;
use super::bv_and_expression::BVAndExpression;
use super::bv_concat_expression::BVConcatExpression;
use super::bv_multiply_expression::BVMultiplyExpression;
use super::bv_or_expression::BVOrExpression;
use super::bv_sign_extend_expression::BVSignExtendExpression;
use super::bv_slice_expression::BVSliceExpression;
use super::bv_sll_expression::BVSllExpression;
use super::bv_srl_expression::BVSrlExpression;
use super::bv_sub_expression::BVSubExpression;
use super::bv_unsigned_remainder_expression::BVUnsignedRemainderExpression;
use super::bv_xor_expression::BVXorExpression;
use super::{bv_concrete::BVConcrete, bv_symbol::BVSymbol, retired_value::RetiredValue};

pub type ActiveValue<SC> = Rc<RefCell<ActiveValueInner<SC>>>;
pub type ActiveValueWeak<SC> = Weak<RefCell<ActiveValueInner<SC>>>;

pub struct ActiveValueInner<SC: ScfiaComposition> {
    pub id: u64,
    pub z3_ast: Z3_ast,
    pub expression: ActiveExpression<SC>,
    pub inherited_asts: BTreeMap<u64, RetiredValue<SC>>,
    pub discovered_asts: BTreeMap<u64, ActiveValueWeak<SC>>,
    pub scfia: Scfia<SC>,
}

pub enum ActiveExpression<SC: ScfiaComposition> {
    BoolConcrete(BoolConcrete),
    BoolEqExpression(BoolEqExpression<SC>),
    BoolNotExpression(BoolNotExpression<SC>),
    BoolSignedLessThanExpression(BoolSignedLessThanExpression<SC>),
    BoolUnsignedLessThanExpression(BoolUnsignedLessThanExpression<SC>),
    BVAddExpression(BVAddExpression<SC>),
    BVAndExpression(BVAndExpression<SC>),
    BVConcatExpression(BVConcatExpression<SC>),
    BVConcrete(BVConcrete),
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

impl<SC: ScfiaComposition> ActiveExpression<SC> {
    pub(crate) fn get_parents(&self, dest: &mut Vec<ActiveValue<SC>>) {
        match self {
            ActiveExpression::BVConcrete(_) => {}
            ActiveExpression::BVSymbol(_) => {}
            ActiveExpression::BVAddExpression(e) => {
                dest.push(e.s1.clone());
                dest.push(e.s2.clone());
            }
            ActiveExpression::BoolConcrete(_) => {}
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
        }
    }
}

impl<SC: ScfiaComposition> ActiveValueInner<SC> {
    pub fn try_as_concrete_bv(&self) -> Option<u64> {
        match &self.expression {
            ActiveExpression::BVConcrete(bvc) => Some(bvc.value),
            _ => None,
        }
    }

    pub fn try_as_concrete_bool(&self) -> Option<bool> {
        match &self.expression {
            ActiveExpression::BoolConcrete(bvc) => Some(bvc.value),
            _ => None,
        }
    }

    pub fn monomorphize(&self, candidates: &mut Vec<u64>) {
        self.scfia.monomorphize_active(self, candidates)
    }

    pub fn assert(&mut self) {
        self.scfia.clone().assert(self);
    }

    pub(crate) fn clone_to(&self, own_scfia: &ScfiaInner<SC>, cloned_scfia: &mut ScfiaInner<SC>, cloned_scfia_rc: Scfia<SC>, id: Option<u64>) -> ActiveValue<SC> {
        trace!("cloning {:?}", self);
        let cloned_value = match &self.expression {
            ActiveExpression::BoolConcrete(e) => cloned_scfia.new_bool_concrete(cloned_scfia_rc, e.value, id, &mut None),
            ActiveExpression::BoolEqExpression(e) => cloned_scfia.new_bool_eq(cloned_scfia_rc, e.s1.clone(), e.s2.clone(), id, &mut None),
            ActiveExpression::BoolNotExpression(_) => todo!(),
            ActiveExpression::BoolSignedLessThanExpression(_) => todo!(),
            ActiveExpression::BoolUnsignedLessThanExpression(_) => todo!(),
            ActiveExpression::BVAddExpression(_) => todo!(),
            ActiveExpression::BVAndExpression(_) => todo!(),
            ActiveExpression::BVConcatExpression(_) => todo!(),
            ActiveExpression::BVConcrete(e) => cloned_scfia.new_bv_concrete(cloned_scfia_rc, e.value, e.width, id, &mut None),
            ActiveExpression::BVMultiplyExpression(_) => todo!(),
            ActiveExpression::BVOrExpression(_) => todo!(),
            ActiveExpression::BVSignExtendExpression(_) => todo!(),
            ActiveExpression::BVSliceExpression(_) => todo!(),
            ActiveExpression::BVSllExpression(_) => todo!(),
            ActiveExpression::BVSrlExpression(_) => todo!(),
            ActiveExpression::BVSubExpression(_) => todo!(),
            ActiveExpression::BVSymbol(e) => cloned_scfia.new_bv_symbol(cloned_scfia_rc, e.width, id, &mut None),
            ActiveExpression::BVUnsignedRemainderExpression(_) => todo!(),
            ActiveExpression::BVXorExpression(_) => todo!(),
        };
        let mut cloned_value_inner = cloned_value.borrow_mut();
        // Fix references to inherited RetiredValues
        for inherited_value in &self.inherited_asts {
            let old = cloned_value_inner.inherited_asts.insert(*inherited_value.0, cloned_scfia.retired_symbols.get(inherited_value.0).unwrap().upgrade().unwrap());
            debug_assert!(old.is_none());
        }
        // Fix references to discovered values
        for discovered_value in &self.discovered_asts {
            panic!("welp with recursive clones this would work")
        }
        cloned_value.clone()
    }
}

impl<SC: ScfiaComposition> Debug for ActiveValueInner<SC> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        self.expression.fmt(f)?;
        f.write_str(format!("[id={}]", self.id).as_str())
    }
}

impl<SC: ScfiaComposition> Drop for ActiveValueInner<SC> {
    fn drop(&mut self) {
        self.scfia.drop_active(self);
    }
}

impl<SC: ScfiaComposition> Debug for ActiveExpression<SC> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ActiveExpression::BVConcrete(e) => e.fmt(f),
            ActiveExpression::BVSymbol(e) => e.fmt(f),
            ActiveExpression::BVAddExpression(e) => e.fmt(f),
            ActiveExpression::BoolConcrete(e) => e.fmt(f),
            ActiveExpression::BoolEqExpression(e) => e.fmt(f),
            ActiveExpression::BoolNotExpression(e) => e.fmt(f),
            ActiveExpression::BoolSignedLessThanExpression(e) => e.fmt(f),
            ActiveExpression::BoolUnsignedLessThanExpression(e) => e.fmt(f),
            ActiveExpression::BVAndExpression(e) => e.fmt(f),
            ActiveExpression::BVConcatExpression(e) => e.fmt(f),
            ActiveExpression::BVMultiplyExpression(e) => e.fmt(f),
            ActiveExpression::BVOrExpression(e) => e.fmt(f),
            ActiveExpression::BVSignExtendExpression(e) => e.fmt(f),
            ActiveExpression::BVSliceExpression(e) => e.fmt(f),
            ActiveExpression::BVSllExpression(e) => e.fmt(f),
            ActiveExpression::BVSrlExpression(e) => e.fmt(f),
            ActiveExpression::BVSubExpression(e) => e.fmt(f),
            ActiveExpression::BVUnsignedRemainderExpression(e) => e.fmt(f),
            ActiveExpression::BVXorExpression(e) => e.fmt(f),
        }
    }
}
