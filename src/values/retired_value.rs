use std::{
    cell::RefCell,
    fmt::Debug,
    rc::{Rc, Weak},
};

use crate::scfia::Scfia;

use super::{retired_bv_concrete::RetiredBVConcrete, retired_bv_symbol::RetiredBVSymbol};

pub type RetiredValue = Rc<RefCell<RetiredValueInner>>;
pub type RetiredValueWeak = Weak<RefCell<RetiredValueInner>>;

pub struct RetiredValueInner {
    pub id: u64,
    pub scfia: Scfia,
    pub expression: RetiredExpression,
}

pub enum RetiredExpression {
    BVConcrete(RetiredBVConcrete),
    BVSymbol(RetiredBVSymbol),
    //BVAddExpression(RetiredBVAddExpression),
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
            //RetiredExpression::BVAddExpression(e) => e.fmt(f),
        }
    }
}
