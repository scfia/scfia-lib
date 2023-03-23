use std::{
    cell::RefCell,
    fmt::Debug,
    rc::{Rc, Weak},
};

use z3_sys::Z3_ast;

use crate::scfia::Scfia;

use super::{
    bool_concrete::RetiredBoolConcrete, bool_eq_expression::RetiredBoolEqExpression, bool_not_expresssion::RetiredBoolNotExpression,
    bool_signed_less_than_expression::RetiredBoolSignedLessThanExpression, bool_unsigned_less_than_expression::RetiredBoolUnsignedLessThanExpression,
    bv_add_expression::RetiredBVAddExpression, bv_and_expression::RetiredBVAndExpression, bv_concat_expression::RetiredBVConcatExpression,
    bv_concrete::RetiredBVConcrete, bv_multiply_expression::RetiredBVMultiplyExpression, bv_or_expression::RetiredBVOrExpression,
    bv_sign_extend_expression::RetiredBVSignExtendExpression, bv_slice_expression::RetiredBVSliceExpression, bv_sll_expression::RetiredBVSllExpression,
    bv_srl_expression::RetiredBVSrlExpression, bv_sub_expression::RetiredBVSubExpression, bv_symbol::RetiredBVSymbol,
    bv_unsigned_remainder_expression::RetiredBVUnsignedRemainderExpression, bv_xor_expression::RetiredBVXorExpression,
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
    BoolEqExpression(RetiredBoolEqExpression),
    BoolNotExpression(RetiredBoolNotExpression),
    BoolSignedLessThanExpression(RetiredBoolSignedLessThanExpression),
    BoolUnsignedLessThanExpression(RetiredBoolUnsignedLessThanExpression),
    BVAddExpression(RetiredBVAddExpression),
    BVAndExpression(RetiredBVAndExpression),
    BVConcatExpression(RetiredBVConcatExpression),
    BVConcrete(RetiredBVConcrete),
    BVMultiplyExpression(RetiredBVMultiplyExpression),
    BVOrExpression(RetiredBVOrExpression),
    BVSignExtendExpression(RetiredBVSignExtendExpression),
    BVSliceExpression(RetiredBVSliceExpression),
    BVSllExpression(RetiredBVSllExpression),
    BVSrlExpression(RetiredBVSrlExpression),
    BVSubExpression(RetiredBVSubExpression),
    BVSymbol(RetiredBVSymbol),
    BVUnsignedRemainderExpression(RetiredBVUnsignedRemainderExpression),
    BVXorExpression(RetiredBVXorExpression),
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
            RetiredExpression::BoolSignedLessThanExpression(e) => e.fmt(f),
            RetiredExpression::BoolUnsignedLessThanExpression(e) => e.fmt(f),
            RetiredExpression::BVAndExpression(e) => e.fmt(f),
            RetiredExpression::BVConcatExpression(e) => e.fmt(f),
            RetiredExpression::BVMultiplyExpression(e) => e.fmt(f),
            RetiredExpression::BVOrExpression(e) => e.fmt(f),
            RetiredExpression::BVSignExtendExpression(e) => e.fmt(f),
            RetiredExpression::BVSliceExpression(e) => e.fmt(f),
            RetiredExpression::BVSllExpression(e) => e.fmt(f),
            RetiredExpression::BVSrlExpression(e) => e.fmt(f),
            RetiredExpression::BVSubExpression(e) => e.fmt(f),
            RetiredExpression::BVUnsignedRemainderExpression(e) => e.fmt(f),
            RetiredExpression::BVXorExpression(e) => e.fmt(f),
        }
    }
}
