use std::fmt::Debug;
use std::{
    cell::RefCell,
    collections::BTreeMap,
    rc::{Rc, Weak},
};

use z3_sys::Z3_ast;

use crate::scfia::Scfia;

use super::bv_add_expression::BVAddExpression;
use super::{bv_concrete::BVConcrete, bv_symbol::BVSymbol, retired_value::RetiredValue};

pub type ActiveValue = Rc<RefCell<ActiveValueInner>>;

pub type ActiveValueWeak = Weak<RefCell<ActiveValueInner>>;

pub struct ActiveValueInner {
    pub id: u64,
    pub z3_ast: Z3_ast,
    pub expression: ActiveExpression,
    pub inherited_asts: BTreeMap<u64, RetiredValue>,
    pub discovered_asts: BTreeMap<u64, Weak<RefCell<ActiveValue>>>,
    pub scfia: Scfia,
}

pub enum ActiveExpression {
    BVConcrete(BVConcrete),
    BVSymbol(BVSymbol),
    BVAddExpression(BVAddExpression),
}

impl ActiveValueInner {
    pub fn try_as_concrete(&self) -> Option<u64> {
        match &self.expression {
            ActiveExpression::BVConcrete(bvc) => Some(bvc.value),
            _ => None,
        }
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
        self.scfia.drop_active(self)
    }
}

impl Debug for ActiveExpression {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ActiveExpression::BVConcrete(e) => e.fmt(f),
            ActiveExpression::BVSymbol(e) => e.fmt(f),
            ActiveExpression::BVAddExpression(e) => e.fmt(f),
        }
    }
}
