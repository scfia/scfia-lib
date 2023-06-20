use std::fmt::Debug;
use std::{
    cell::RefCell,
    collections::BTreeMap,
    rc::{Rc, Weak},
};

use log::trace;
use z3_sys::Z3_ast;

use crate::scfia::{Scfia};
use crate::ScfiaComposition;
use crate::z3_handle::Z3Ast;

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

#[derive(Clone, Debug)]
pub struct ValueComment {
    pub message: String,
}

impl ValueComment {
    pub fn new(message: String) -> Self {
        ValueComment { message }
    }
}

pub struct ActiveValueInner<SC: ScfiaComposition> {
    pub id: u64,
    pub z3_ast: Z3Ast<SC>,
    pub expression: ActiveExpression<SC>,
    pub inherited_asts: BTreeMap<u64, RetiredValue<SC>>,
    pub discovered_asts: BTreeMap<u64, ActiveValueWeak<SC>>,
    pub scfia: Weak<Scfia<SC>>,
    pub comment: Option<ValueComment>,
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

    pub fn is_concrete(&self) -> bool {
        match self.expression {
            ActiveExpression::BoolConcrete(_) => true,
            ActiveExpression::BVConcrete(_) => true,
            _ => false,
        }
    }

    pub fn monomorphize(&self, candidates: &mut Vec<u64>) {
        self.scfia.upgrade().unwrap().monomorphize_active(self, candidates)
    }

    pub fn assert(&mut self) {
        todo!()
    }

    pub(crate) fn clone_to_stdlib(&self, cloned_scfia: &Scfia<SC>, cloned_actives: &mut BTreeMap<u64, ActiveValue<SC>>, cloned_retired: &mut BTreeMap<u64, RetiredValue<SC>>) -> ActiveValue<SC> {
        if let Some(value) = cloned_actives.get(&self.id)  {
            return value.clone()
        }

        trace!("Cloning {:?}", self);
        let cloned_value = match &self.expression {
            ActiveExpression::BoolConcrete(e) => cloned_scfia.new_bool_concrete(e.value, Some(self.id), &mut None),
            ActiveExpression::BoolEqExpression(e) => {
                let s1 = e.s1.try_borrow().unwrap().clone_to_stdlib(cloned_scfia, cloned_actives, cloned_retired);
                let s2 = e.s2.try_borrow().unwrap().clone_to_stdlib(cloned_scfia, cloned_actives, cloned_retired);
                cloned_scfia.new_bool_eq(&s1, &s2, Some(self.id), e.is_assert, &mut None, self.comment.clone())
            },
            /* 
            ActiveExpression::BoolNotExpression(e) => {
                let s1 = e.s1.try_borrow().unwrap().clone_to_stdlib(cloned_scfia, cloned_actives, cloned_retired);
                cloned_scfia.new_bool_not(s1, Some(self.id), e.is_assert, &mut None, self.comment.clone())
            },
            ActiveExpression::BoolSignedLessThanExpression(e) => {
                let s1 = e.s1.try_borrow().unwrap().clone_to_stdlib(cloned_scfia, cloned_actives, cloned_retired);
                let s2 = e.s2.try_borrow().unwrap().clone_to_stdlib(cloned_scfia, cloned_actives, cloned_retired);
                cloned_scfia.new_bool_signed_less_than(s1, s2, Some(self.id), e.is_assert, &mut None, self.comment.clone())
            },
            ActiveExpression::BoolUnsignedLessThanExpression(e) => {
                let s1 = e.s1.try_borrow().unwrap().clone_to_stdlib(cloned_scfia, cloned_actives, cloned_retired);
                let s2: Rc<RefCell<ActiveValueInner<SC>>> = e.s2.try_borrow().unwrap().clone_to_stdlib(cloned_scfia, cloned_actives, cloned_retired);
                cloned_scfia.new_bool_unsigned_less_than(s1, s2, Some(self.id), e.is_assert, &mut None, self.comment.clone())
            },
            ActiveExpression::BVAddExpression(e) => {
                let s1 = e.s1.try_borrow().unwrap().clone_to_stdlib(cloned_scfia, cloned_actives, cloned_retired);
                let s2 = e.s2.try_borrow().unwrap().clone_to_stdlib(cloned_scfia, cloned_actives, cloned_retired);
                cloned_scfia.new_bv_add(s1, s2, e.width, Some(self.id), &mut None, self.comment.clone())
            },
            ActiveExpression::BVAndExpression(e) => {
                let s1 = e.s1.try_borrow().unwrap().clone_to_stdlib(cloned_scfia, cloned_actives, cloned_retired);
                let s2 = e.s2.try_borrow().unwrap().clone_to_stdlib(cloned_scfia, cloned_actives, cloned_retired);
                cloned_scfia.new_bv_and(s1, s2, e.width, Some(self.id), &mut None, self.comment.clone())
            },
            ActiveExpression::BVConcatExpression(e) => {
                let s1 = e.s1.try_borrow().unwrap().clone_to_stdlib(cloned_scfia, cloned_actives, cloned_retired);
                let s2 = e.s2.try_borrow().unwrap().clone_to_stdlib(cloned_scfia, cloned_actives, cloned_retired);
                cloned_scfia.new_bv_concat(s1, s2, e.width, Some(self.id), &mut None, self.comment.clone())
            },
            ActiveExpression::BVConcrete(e) => cloned_scfia.new_bv_concrete(e.value, e.width, Some(self.id), &mut None, self.comment.clone()),
            ActiveExpression::BVMultiplyExpression(e) => {
                let s1 = e.s1.try_borrow().unwrap().clone_to_stdlib(cloned_scfia, cloned_actives, cloned_retired);
                let s2 = e.s2.try_borrow().unwrap().clone_to_stdlib(cloned_scfia, cloned_actives, cloned_retired);
                cloned_scfia.new_bv_multiply(s1, s2, e.width, Some(self.id), &mut None, self.comment.clone())
            },
            ActiveExpression::BVOrExpression(e) => {
                let s1 = e.s1.try_borrow().unwrap().clone_to_stdlib(cloned_scfia, cloned_actives, cloned_retired);
                let s2 = e.s2.try_borrow().unwrap().clone_to_stdlib(cloned_scfia, cloned_actives, cloned_retired);
                cloned_scfia.new_bv_or(s1, s2, e.width, Some(self.id), &mut None, self.comment.clone())
            },
            ActiveExpression::BVSignExtendExpression(e) => {
                let s1 = e.s1.try_borrow().unwrap().clone_to_stdlib(cloned_scfia, cloned_actives, cloned_retired);
                cloned_scfia.new_bv_sign_extend(s1, e.input_width, e.width,Some(self.id), &mut None, self.comment.clone())
            },
            ActiveExpression::BVSliceExpression(e) => {
                let s1 = e.s1.try_borrow().unwrap().clone_to_stdlib(cloned_scfia, cloned_actives, cloned_retired);
                cloned_scfia.new_bv_slice(s1, e.high, e.low, Some(self.id), &mut None, self.comment.clone())
            },
            ActiveExpression::BVSllExpression(e) => {
                let s1 = e.s1.try_borrow().unwrap().clone_to_stdlib(cloned_scfia, cloned_actives, cloned_retired);
                let s2 = e.s2.try_borrow().unwrap().clone_to_stdlib(cloned_scfia, cloned_actives, cloned_retired);
                cloned_scfia.new_bv_sll(s1, s2, e.width, Some(self.id), &mut None, self.comment.clone())
            },
            ActiveExpression::BVSrlExpression(e) => {
                let s1 = e.s1.try_borrow().unwrap().clone_to_stdlib(cloned_scfia, cloned_actives, cloned_retired);
                let s2 = e.s2.try_borrow().unwrap().clone_to_stdlib(cloned_scfia, cloned_actives, cloned_retired);
                cloned_scfia.new_bv_srl(s1, s2, e.width, Some(self.id), &mut None, self.comment.clone())
            },
            ActiveExpression::BVSubExpression(e) => {
                let s1 = e.s1.try_borrow().unwrap().clone_to_stdlib(cloned_scfia, cloned_actives, cloned_retired);
                let s2 = e.s2.try_borrow().unwrap().clone_to_stdlib(cloned_scfia, cloned_actives, cloned_retired);
                cloned_scfia.new_bv_sub(s1, s2, e.width, Some(self.id), &mut None, self.comment.clone())
            },
            ActiveExpression::BVSymbol(e) => cloned_scfia.new_bv_symbol(e.width, Some(self.id), &mut None, self.comment.clone()),
            ActiveExpression::BVUnsignedRemainderExpression(e) => {
                let s1 = e.s1.try_borrow().unwrap().clone_to_stdlib(cloned_scfia, cloned_actives, cloned_retired);
                let s2 = e.s2.try_borrow().unwrap().clone_to_stdlib(cloned_scfia, cloned_actives, cloned_retired);
                cloned_scfia.new_bv_unsigned_remainder(s1, s2, e.width, Some(self.id), &mut None, self.comment.clone())
            },
            ActiveExpression::BVXorExpression(e) => {
                let s1 = e.s1.try_borrow().unwrap().clone_to_stdlib(cloned_scfia, cloned_actives, cloned_retired);
                let s2 = e.s2.try_borrow().unwrap().clone_to_stdlib(cloned_scfia, cloned_actives, cloned_retired);
                cloned_scfia.new_bv_xor(s1, s2, e.width, Some(self.id), &mut None, self.comment.clone())
            },
            */
            _ => unimplemented!()
        };

        cloned_actives.insert(self.id, cloned_value.clone());

        // Clone inherited values
        for (inherited_value_id, inherited_value) in &self.inherited_asts {
            let inherited_value_ref = inherited_value.try_borrow().unwrap();
            let cloned_inherited_value = inherited_value_ref.clone_to_stdlib(cloned_scfia, cloned_actives, cloned_retired);
            cloned_value.try_borrow_mut().unwrap().inherited_asts.insert(*inherited_value_id, cloned_inherited_value);
        }

        // Clone discovered values
        for (discovered_value_id, discovered_value) in &self.discovered_asts {
            let cloned_discovered_value = discovered_value.upgrade().unwrap().try_borrow().unwrap().clone_to_stdlib(cloned_scfia, cloned_actives, cloned_retired);
            cloned_value.try_borrow_mut().unwrap().discovered_asts.insert(*discovered_value_id, Rc::downgrade(&cloned_discovered_value));
        }

        cloned_value
    }
}

impl<SC: ScfiaComposition> Debug for ActiveValueInner<SC> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        self.expression.fmt(f)?;
        f.write_str(format!("[id={}]", self.id).as_str())?;
        if let Some(comment) = &self.comment {
            f.write_str(&format!("[{}]", comment.message))?;
        }
        if f.alternate() {
            f.write_str(&format!("[inherited={:#?}]", self.inherited_asts))?;
        }
        Ok(())
    }
}

impl<SC: ScfiaComposition> Drop for ActiveValueInner<SC> {
    fn drop(&mut self) {
        if let Some(scfia) = self.scfia.upgrade() {
            scfia.drop_active(self);
        }
    }
}

impl<SC: ScfiaComposition> Debug for ActiveExpression<SC> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        if f.alternate() {
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
        } else {
            match self {
                ActiveExpression::BoolConcrete(_) => f.write_str("BoolConcrete"),
                ActiveExpression::BoolEqExpression(_) => f.write_str("BoolEqExpression"),
                ActiveExpression::BoolNotExpression(_) => f.write_str("BoolNotExpression"),
                ActiveExpression::BoolSignedLessThanExpression(_) => f.write_str("BoolSignedLessThanExpression"),
                ActiveExpression::BoolUnsignedLessThanExpression(_) => f.write_str("BoolUnsignedLessThanExpression"),
                ActiveExpression::BVAddExpression(_) => f.write_str("BVAddExpression"),
                ActiveExpression::BVAndExpression(_) => f.write_str("BVAndExpression"),
                ActiveExpression::BVConcatExpression(_) => f.write_str("BVConcatExpression"),
                ActiveExpression::BVConcrete(_) => f.write_str("BVConcrete"),
                ActiveExpression::BVMultiplyExpression(_) => f.write_str("BVMultiplyExpression"),
                ActiveExpression::BVOrExpression(_) => f.write_str("BVOrExpression"),
                ActiveExpression::BVSignExtendExpression(_) => f.write_str("BVSignExtendExpression"),
                ActiveExpression::BVSliceExpression(_) => f.write_str("BVSliceExpression"),
                ActiveExpression::BVSllExpression(_) => f.write_str("BVSllExpression"),
                ActiveExpression::BVSrlExpression(_) => f.write_str("BVSrlExpression"),
                ActiveExpression::BVSubExpression(_) => f.write_str("BVSubExpression"),
                ActiveExpression::BVSymbol(_) => f.write_str("BVSymbol"),
                ActiveExpression::BVUnsignedRemainderExpression(_) => f.write_str("BVUnsignedRemainderExpression"),
                ActiveExpression::BVXorExpression(_) => f.write_str("BVXorExpression"),
            }
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
