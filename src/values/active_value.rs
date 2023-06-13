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

    pub(crate) fn clone_to_stdlib(&self, cloned_scfia: &mut ScfiaInner<SC>, cloned_scfia_rc: Scfia<SC>) -> ActiveValue<SC> {
        if let Some(value) = cloned_scfia.active_symbols.get(&self.id)  {
            return value.upgrade().unwrap()
        }

        let cloned_value = match &self.expression {
            ActiveExpression::BoolConcrete(e) => cloned_scfia.new_bool_concrete(cloned_scfia_rc, e.value, Some(self.id), &mut None),
            ActiveExpression::BoolEqExpression(e) => {
                let s1 = e.s1.try_borrow().unwrap().clone_to_stdlib(cloned_scfia, cloned_scfia_rc.clone());
                let s2 = e.s2.try_borrow().unwrap().clone_to_stdlib(cloned_scfia, cloned_scfia_rc.clone());
                cloned_scfia.new_bool_eq(cloned_scfia_rc, s1, s2, Some(self.id), e.is_assert, &mut None)
            },
            ActiveExpression::BoolNotExpression(e) => {
                let s1 = e.s1.try_borrow().unwrap().clone_to_stdlib(cloned_scfia, cloned_scfia_rc.clone());
                cloned_scfia.new_bool_not(cloned_scfia_rc, s1, Some(self.id), e.is_assert, &mut None)
            },
            ActiveExpression::BoolSignedLessThanExpression(e) => {
                let s1 = e.s1.try_borrow().unwrap().clone_to_stdlib(cloned_scfia, cloned_scfia_rc.clone());
                let s2 = e.s2.try_borrow().unwrap().clone_to_stdlib(cloned_scfia, cloned_scfia_rc.clone());
                cloned_scfia.new_bool_signed_less_than(cloned_scfia_rc, s1, s2, Some(self.id), e.is_assert, &mut None)
            },
            ActiveExpression::BoolUnsignedLessThanExpression(e) => {
                let s1 = e.s1.try_borrow().unwrap().clone_to_stdlib(cloned_scfia, cloned_scfia_rc.clone());
                let s2: Rc<RefCell<ActiveValueInner<SC>>> = e.s2.try_borrow().unwrap().clone_to_stdlib(cloned_scfia, cloned_scfia_rc.clone());
                cloned_scfia.new_bool_unsigned_less_than(cloned_scfia_rc, s1, s2, Some(self.id), e.is_assert, &mut None)
            },
            ActiveExpression::BVAddExpression(e) => {
                let s1 = e.s1.try_borrow().unwrap().clone_to_stdlib(cloned_scfia, cloned_scfia_rc.clone());
                let s2 = e.s2.try_borrow().unwrap().clone_to_stdlib(cloned_scfia, cloned_scfia_rc.clone());
                cloned_scfia.new_bv_add(cloned_scfia_rc, s1, s2, e.width, Some(self.id), &mut None)
            },
            ActiveExpression::BVAndExpression(e) => {
                let s1 = e.s1.try_borrow().unwrap().clone_to_stdlib(cloned_scfia, cloned_scfia_rc.clone());
                let s2 = e.s2.try_borrow().unwrap().clone_to_stdlib(cloned_scfia, cloned_scfia_rc.clone());
                cloned_scfia.new_bv_and(cloned_scfia_rc, s1, s2, e.width, Some(self.id), &mut None)
            },
            ActiveExpression::BVConcatExpression(e) => {
                let s1 = e.s1.try_borrow().unwrap().clone_to_stdlib(cloned_scfia, cloned_scfia_rc.clone());
                let s2 = e.s2.try_borrow().unwrap().clone_to_stdlib(cloned_scfia, cloned_scfia_rc.clone());
                cloned_scfia.new_bv_concat(cloned_scfia_rc, s1, s2, e.width, Some(self.id), &mut None)
            },
            ActiveExpression::BVConcrete(e) => cloned_scfia.new_bv_concrete(cloned_scfia_rc, e.value, e.width, Some(self.id), &mut None),
            ActiveExpression::BVMultiplyExpression(e) => {
                let s1 = e.s1.try_borrow().unwrap().clone_to_stdlib(cloned_scfia, cloned_scfia_rc.clone());
                let s2 = e.s2.try_borrow().unwrap().clone_to_stdlib(cloned_scfia, cloned_scfia_rc.clone());
                cloned_scfia.new_bv_multiply(cloned_scfia_rc, s1, s2, e.width, Some(self.id), &mut None)
            },
            ActiveExpression::BVOrExpression(e) => {
                let s1 = e.s1.try_borrow().unwrap().clone_to_stdlib(cloned_scfia, cloned_scfia_rc.clone());
                let s2 = e.s2.try_borrow().unwrap().clone_to_stdlib(cloned_scfia, cloned_scfia_rc.clone());
                cloned_scfia.new_bv_or(cloned_scfia_rc, s1, s2, e.width, Some(self.id), &mut None)
            },
            ActiveExpression::BVSignExtendExpression(e) => {
                let s1 = e.s1.try_borrow().unwrap().clone_to_stdlib(cloned_scfia, cloned_scfia_rc.clone());
                cloned_scfia.new_bv_sign_extend(cloned_scfia_rc, s1, e.input_width, e.width,Some(self.id), &mut None)
            },
            ActiveExpression::BVSliceExpression(e) => {
                let s1 = e.s1.try_borrow().unwrap().clone_to_stdlib(cloned_scfia, cloned_scfia_rc.clone());
                cloned_scfia.new_bv_slice(cloned_scfia_rc, s1, e.high, e.low, Some(self.id), &mut None)
            },
            ActiveExpression::BVSllExpression(e) => {
                let s1 = e.s1.try_borrow().unwrap().clone_to_stdlib(cloned_scfia, cloned_scfia_rc.clone());
                let s2 = e.s2.try_borrow().unwrap().clone_to_stdlib(cloned_scfia, cloned_scfia_rc.clone());
                cloned_scfia.new_bv_sll(cloned_scfia_rc, s1, s2, e.width, e.shamt_width, Some(self.id), &mut None)
            },
            ActiveExpression::BVSrlExpression(e) => {
                let s1 = e.s1.try_borrow().unwrap().clone_to_stdlib(cloned_scfia, cloned_scfia_rc.clone());
                let s2 = e.s2.try_borrow().unwrap().clone_to_stdlib(cloned_scfia, cloned_scfia_rc.clone());
                cloned_scfia.new_bv_srl(cloned_scfia_rc, s1, s2, e.width, e.shamt, Some(self.id), &mut None)
            },
            ActiveExpression::BVSubExpression(e) => {
                let s1 = e.s1.try_borrow().unwrap().clone_to_stdlib(cloned_scfia, cloned_scfia_rc.clone());
                let s2 = e.s2.try_borrow().unwrap().clone_to_stdlib(cloned_scfia, cloned_scfia_rc.clone());
                cloned_scfia.new_bv_sub(cloned_scfia_rc, s1, s2, e.width, Some(self.id), &mut None)
            },
            ActiveExpression::BVSymbol(e) => cloned_scfia.new_bv_symbol(cloned_scfia_rc, e.width, Some(self.id), &mut None),
            ActiveExpression::BVUnsignedRemainderExpression(e) => {
                let s1 = e.s1.try_borrow().unwrap().clone_to_stdlib(cloned_scfia, cloned_scfia_rc.clone());
                let s2 = e.s2.try_borrow().unwrap().clone_to_stdlib(cloned_scfia, cloned_scfia_rc.clone());
                cloned_scfia.new_bv_unsigned_remainder(cloned_scfia_rc, s1, s2, e.width, Some(self.id), &mut None)
            },
            ActiveExpression::BVXorExpression(e) => {
                let s1 = e.s1.try_borrow().unwrap().clone_to_stdlib(cloned_scfia, cloned_scfia_rc.clone());
                let s2 = e.s2.try_borrow().unwrap().clone_to_stdlib(cloned_scfia, cloned_scfia_rc.clone());
                cloned_scfia.new_bv_xor(cloned_scfia_rc, s1, s2, e.width, Some(self.id), &mut None)
            },
        };
        // TODO fix relations
        cloned_value
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
            _ => Ok(()),
            /*
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
            */
        }
    }
}

pub trait ActiveValueImpl<SC: ScfiaComposition> {
    fn to_u64(&self) -> u64;
}

impl<SC: ScfiaComposition> ActiveValueImpl<SC> for ActiveValue<SC> {
    fn to_u64(&self) -> u64 {
        match &self.try_borrow().unwrap().expression {
            ActiveExpression::BVConcrete(bvc) => bvc.value,
            _ => panic!(),
        }
    }
}
