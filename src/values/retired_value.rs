use std::{
    cell::RefCell,
    fmt::Debug,
    rc::{Rc, Weak},
};

use z3_sys::Z3_ast;

use crate::scfia::Scfia;

use super::{
    retired_bool_concrete::RetiredBoolConcrete, retired_bool_eq_expression::RetiredBoolEqExpression, retired_bool_not_expresssion::RetiredBoolNotExpression,
    retired_bool_signed_less_than_expression::RetiredBoolSignedLessThanExpression,
    retired_bool_unsigned_less_than_expression::RetiredBoolUnsignedLessThanExpression, retired_bv_add_expression::RetiredBVAddExpression,
    retired_bv_and_expression::RetiredBVAndExpression, retired_bv_concat_expression::RetiredBVConcatExpression, retired_bv_concrete::RetiredBVConcrete,
    retired_bv_multiply_expression::RetiredBVMultiplyExpression, retired_bv_or_expression::RetiredBVOrExpression,
    retired_bv_sign_extend_expression::RetiredBVSignExtendExpression, retired_bv_slice_expression::RetiredBVSliceExpression,
    retired_bv_sll_expression::RetiredBVSllExpression, retired_bv_srl_expression::RetiredBVSrlExpression, retired_bv_sub_expression::RetiredBVSubExpression,
    retired_bv_symbol::RetiredBVSymbol, retired_bv_unsigned_remainder_expression::RetiredBVUnsignedRemainderExpression,
    retired_bv_xor_expression::RetiredBVXorExpression,
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
