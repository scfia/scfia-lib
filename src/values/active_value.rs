use std::fmt::Debug;
use std::{
    cell::RefCell,
    collections::BTreeMap,
    rc::{Rc, Weak},
};

use z3_sys::Z3_ast;

use crate::scfia::Scfia;

use super::bool_concrete::BoolConcrete;
use super::bool_eq_expression::BoolEqExpression;
use super::bool_not_expresssion::BoolNotExpression;
use super::bv_add_expression::BVAddExpression;
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
    BoolNotExpression(BoolNotExpression),
    BoolEqExpression(BoolEqExpression),
    BVConcrete(BVConcrete),
    BVSymbol(BVSymbol),
    BVAddExpression(BVAddExpression),
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
        }
    }
}
