use std::{
    cell::RefCell,
    fmt::Debug,
    rc::{Rc, Weak},
};

use z3_sys::Z3_ast;

use crate::scfia::Scfia;

use super::{
    retired_bool_concrete::RetiredBoolConcrete, retired_bool_eq_expression::RetiredBoolEqExpression, retired_bool_not_expresssion::RetiredBoolNotExpression,
    retired_bv_add_expression::RetiredBVAddExpression, retired_bv_concrete::RetiredBVConcrete, retired_bv_symbol::RetiredBVSymbol,
};

pub type RetiredValue = Rc<RefCell<RetiredValueInner>>;
pub type RetiredValueWeak = Weak<RefCell<RetiredValueInner>>;

pub struct RetiredValueInner {
    pub id: u64,
    pub z3_ast: Z3_ast,
    pub expression: RetiredExpression,
    pub scfia: Scfia,
}

pub enum RetiredExpression {
    BoolConcrete(RetiredBoolConcrete),
    BoolNotExpression(RetiredBoolNotExpression),
    BoolEqExpression(RetiredBoolEqExpression),
    BVConcrete(RetiredBVConcrete),
    BVSymbol(RetiredBVSymbol),
    BVAddExpression(RetiredBVAddExpression),
}

impl Debug for RetiredValueInner {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        self.expression.fmt(f)?;
        f.write_str(format!("[id={}]", self.id).as_str())
    }
}

impl Drop for RetiredValueInner {
    fn drop(&mut self) {
        self.scfia.drop_retired(self)
    }
}

impl Debug for RetiredExpression {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            RetiredExpression::BVConcrete(e) => e.fmt(f),
            RetiredExpression::BVSymbol(e) => e.fmt(f),
            RetiredExpression::BVAddExpression(e) => e.fmt(f),
            RetiredExpression::BoolConcrete(e) => e.fmt(f),
            RetiredExpression::BoolEqExpression(e) => e.fmt(f),
            RetiredExpression::BoolNotExpression(e) => e.fmt(f),
        }
    }
}
