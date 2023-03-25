use std::fmt::Debug;
use std::{
    cell::RefCell,
    collections::BTreeMap,
    rc::{Rc, Weak},
};

use log::{trace, warn};
use z3_sys::Z3_ast;

use crate::scfia::{Scfia, ScfiaInner};

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

pub type ActiveValue = Rc<RefCell<ActiveValueInner>>;

pub type ActiveValueWeak = Weak<RefCell<ActiveValueInner>>;

pub struct ActiveValueInner {
    pub id: u64,
    pub z3_ast: Z3_ast,
    pub expression: ActiveExpression,
    pub inherited_asts: BTreeMap<u64, RetiredValue>,
    pub discovered_asts: BTreeMap<u64, ActiveValueWeak>,
    pub scfia: Scfia,
}

pub enum ActiveExpression {
    BoolConcrete(BoolConcrete),
    BoolEqExpression(BoolEqExpression),
    BoolNotExpression(BoolNotExpression),
    BoolSignedLessThanExpression(BoolSignedLessThanExpression),
    BoolUnsignedLessThanExpression(BoolUnsignedLessThanExpression),
    BVAddExpression(BVAddExpression),
    BVAndExpression(BVAndExpression),
    BVConcatExpression(BVConcatExpression),
    BVConcrete(BVConcrete),
    BVMultiplyExpression(BVMultiplyExpression),
    BVOrExpression(BVOrExpression),
    BVSignExtendExpression(BVSignExtendExpression),
    BVSliceExpression(BVSliceExpression),
    BVSllExpression(BVSllExpression),
    BVSrlExpression(BVSrlExpression),
    BVSubExpression(BVSubExpression),
    BVSymbol(BVSymbol),
    BVUnsignedRemainderExpression(BVUnsignedRemainderExpression),
    BVXorExpression(BVXorExpression),
}

impl ActiveExpression {
    pub(crate) fn get_parents(&self, dest: &mut Vec<ActiveValue>) {
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

    pub(crate) fn clone_to(&self, own_scfia: &ScfiaInner, cloned_scfia: &mut ScfiaInner, cloned_scfia_rc: Scfia, id: u64) -> ActiveValue {
        trace!("cloning {:?}", self);
        match self {
            ActiveExpression::BoolConcrete(e) => cloned_scfia.new_bool_concrete(cloned_scfia_rc, e.value, Some(id)),
            ActiveExpression::BoolEqExpression(_) => todo!(),
            ActiveExpression::BoolNotExpression(_) => todo!(),
            ActiveExpression::BoolSignedLessThanExpression(_) => todo!(),
            ActiveExpression::BoolUnsignedLessThanExpression(_) => todo!(),
            ActiveExpression::BVAddExpression(_) => todo!(),
            ActiveExpression::BVAndExpression(_) => todo!(),
            ActiveExpression::BVConcatExpression(_) => todo!(),
            ActiveExpression::BVConcrete(e) => cloned_scfia.new_bv_concrete(cloned_scfia_rc, e.value, e.width, Some(id)),
            ActiveExpression::BVMultiplyExpression(_) => todo!(),
            ActiveExpression::BVOrExpression(_) => todo!(),
            ActiveExpression::BVSignExtendExpression(_) => todo!(),
            ActiveExpression::BVSliceExpression(_) => todo!(),
            ActiveExpression::BVSllExpression(_) => todo!(),
            ActiveExpression::BVSrlExpression(_) => todo!(),
            ActiveExpression::BVSubExpression(_) => todo!(),
            ActiveExpression::BVSymbol(e) => cloned_scfia.new_bv_symbol(cloned_scfia_rc, e.width, Some(id)),
            ActiveExpression::BVUnsignedRemainderExpression(_) => todo!(),
            ActiveExpression::BVXorExpression(_) => todo!(),
        }
    }
}

impl ActiveValueInner {
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
}

impl Debug for ActiveValueInner {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        self.expression.fmt(f)?;
        f.write_str(format!("[id={}]", self.id).as_str())
    }
}

impl Drop for ActiveValueInner {
    fn drop(&mut self) {
        self.scfia.drop_active(self);
    }
}

impl Debug for ActiveExpression {
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
