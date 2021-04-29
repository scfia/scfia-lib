use std::sync::Arc;

use crate::{
    asserts::scfia_assert_expression::ScfiaAssertExpression,
    symbols::{
        scfia_symbol_expression::ScfiaSymbolExpression, scfia_symbol_manager::ScfiaSymbolManager,
    },
    system_states::{aarch64::ScfiaAarch64SystemState, ScfiaSystemState},
    values::{value1::Value1, value64::Value64, value8::Value8, ScfiaValue},
};

use super::aarch64::transformation_definition;
use crate::transformation_functions::ScfiaTransformationDefinition;

#[derive(Clone, Debug)]
pub enum ScfiaTransformationFunctionExpression<SystemState: ScfiaSystemState> {
    Var(Arc<SystemState::TransformationDefinition>),
    Bits(Vec<bool>),
    Bits64(u64, u32),
    Enum(usize, usize), // enum_id, member
    Bool(bool),
    Eq(
        Arc<ScfiaTransformationFunctionExpression<SystemState>>,
        Arc<ScfiaTransformationFunctionExpression<SystemState>>,
    ),
    Neq(
        Arc<ScfiaTransformationFunctionExpression<SystemState>>,
        Arc<ScfiaTransformationFunctionExpression<SystemState>>,
    ),
    And(
        Arc<ScfiaTransformationFunctionExpression<SystemState>>,
        Arc<ScfiaTransformationFunctionExpression<SystemState>>,
    ),
    Or(
        Arc<ScfiaTransformationFunctionExpression<SystemState>>,
        Arc<ScfiaTransformationFunctionExpression<SystemState>>,
    ),
    Not(Arc<ScfiaTransformationFunctionExpression<SystemState>>),
    Bvnot(Arc<ScfiaTransformationFunctionExpression<SystemState>>),
    Bvand(
        Arc<ScfiaTransformationFunctionExpression<SystemState>>,
        Arc<ScfiaTransformationFunctionExpression<SystemState>>,
    ),
    Bvor(
        Arc<ScfiaTransformationFunctionExpression<SystemState>>,
        Arc<ScfiaTransformationFunctionExpression<SystemState>>,
    ),
    Bvxor(
        Arc<ScfiaTransformationFunctionExpression<SystemState>>,
        Arc<ScfiaTransformationFunctionExpression<SystemState>>,
    ),
    Bvnand(
        Arc<ScfiaTransformationFunctionExpression<SystemState>>,
        Arc<ScfiaTransformationFunctionExpression<SystemState>>,
    ),
    Bvnor(
        Arc<ScfiaTransformationFunctionExpression<SystemState>>,
        Arc<ScfiaTransformationFunctionExpression<SystemState>>,
    ),
    Bvxnor(
        Arc<ScfiaTransformationFunctionExpression<SystemState>>,
        Arc<ScfiaTransformationFunctionExpression<SystemState>>,
    ),
    Bvneg(Arc<ScfiaTransformationFunctionExpression<SystemState>>),
    Bvadd(
        Arc<ScfiaTransformationFunctionExpression<SystemState>>,
        Arc<ScfiaTransformationFunctionExpression<SystemState>>,
    ),
    Bvsub(
        Arc<ScfiaTransformationFunctionExpression<SystemState>>,
        Arc<ScfiaTransformationFunctionExpression<SystemState>>,
    ),
    Bvmul(
        Arc<ScfiaTransformationFunctionExpression<SystemState>>,
        Arc<ScfiaTransformationFunctionExpression<SystemState>>,
    ),
    Bvudiv(
        Arc<ScfiaTransformationFunctionExpression<SystemState>>,
        Arc<ScfiaTransformationFunctionExpression<SystemState>>,
    ),
    Bvsdiv(
        Arc<ScfiaTransformationFunctionExpression<SystemState>>,
        Arc<ScfiaTransformationFunctionExpression<SystemState>>,
    ),
    Bvurem(
        Arc<ScfiaTransformationFunctionExpression<SystemState>>,
        Arc<ScfiaTransformationFunctionExpression<SystemState>>,
    ),
    Bvsrem(
        Arc<ScfiaTransformationFunctionExpression<SystemState>>,
        Arc<ScfiaTransformationFunctionExpression<SystemState>>,
    ),
    Bvsmod(
        Arc<ScfiaTransformationFunctionExpression<SystemState>>,
        Arc<ScfiaTransformationFunctionExpression<SystemState>>,
    ),
    Bvult(
        Arc<ScfiaTransformationFunctionExpression<SystemState>>,
        Arc<ScfiaTransformationFunctionExpression<SystemState>>,
    ),
    Bvslt(
        Arc<ScfiaTransformationFunctionExpression<SystemState>>,
        Arc<ScfiaTransformationFunctionExpression<SystemState>>,
    ),
    Bvule(
        Arc<ScfiaTransformationFunctionExpression<SystemState>>,
        Arc<ScfiaTransformationFunctionExpression<SystemState>>,
    ),
    Bvsle(
        Arc<ScfiaTransformationFunctionExpression<SystemState>>,
        Arc<ScfiaTransformationFunctionExpression<SystemState>>,
    ),
    Bvuge(
        Arc<ScfiaTransformationFunctionExpression<SystemState>>,
        Arc<ScfiaTransformationFunctionExpression<SystemState>>,
    ),
    Bvsge(
        Arc<ScfiaTransformationFunctionExpression<SystemState>>,
        Arc<ScfiaTransformationFunctionExpression<SystemState>>,
    ),
    Bvugt(
        Arc<ScfiaTransformationFunctionExpression<SystemState>>,
        Arc<ScfiaTransformationFunctionExpression<SystemState>>,
    ),
    Bvsgt(
        Arc<ScfiaTransformationFunctionExpression<SystemState>>,
        Arc<ScfiaTransformationFunctionExpression<SystemState>>,
    ),
    Extract(
        u32,
        u32,
        Arc<ScfiaTransformationFunctionExpression<SystemState>>,
    ),
    ZeroExtend(u32, Arc<ScfiaTransformationFunctionExpression<SystemState>>),
    SignExtend(u32, Arc<ScfiaTransformationFunctionExpression<SystemState>>),
    Bvshl(
        Arc<ScfiaTransformationFunctionExpression<SystemState>>,
        Arc<ScfiaTransformationFunctionExpression<SystemState>>,
    ),
    Bvlshr(
        Arc<ScfiaTransformationFunctionExpression<SystemState>>,
        Arc<ScfiaTransformationFunctionExpression<SystemState>>,
    ),
    Bvashr(
        Arc<ScfiaTransformationFunctionExpression<SystemState>>,
        Arc<ScfiaTransformationFunctionExpression<SystemState>>,
    ),
    Concat(
        Arc<ScfiaTransformationFunctionExpression<SystemState>>,
        Arc<ScfiaTransformationFunctionExpression<SystemState>>,
    ),
    Ite(
        Arc<ScfiaTransformationFunctionExpression<SystemState>>,
        Arc<ScfiaTransformationFunctionExpression<SystemState>>,
        Arc<ScfiaTransformationFunctionExpression<SystemState>>,
    ),
    Select(
        Arc<ScfiaTransformationFunctionExpression<SystemState>>,
        Arc<ScfiaTransformationFunctionExpression<SystemState>>,
    ),
    Store(
        Arc<ScfiaTransformationFunctionExpression<SystemState>>,
        Arc<ScfiaTransformationFunctionExpression<SystemState>>,
        Arc<ScfiaTransformationFunctionExpression<SystemState>>,
    ),
}

impl<SystemState: ScfiaSystemState> ScfiaTransformationFunctionExpression<SystemState> {
    pub fn to_value_64(
        &self,
        old_state: &SystemState,
        new_symbol_manager: &mut ScfiaSymbolManager,
    ) -> Value64 {
        self.to_value(old_state, new_symbol_manager)
    }

    pub fn to_value_1(
        &self,
        old_state: &SystemState,
        new_symbol_manager: &mut ScfiaSymbolManager,
    ) -> Value1 {
        self.to_value(old_state, new_symbol_manager)
    }

    pub fn to_value_8(
        &self,
        old_state: &SystemState,
        new_symbol_manager: &mut ScfiaSymbolManager,
    ) -> Value8 {
        self.to_value(old_state, new_symbol_manager)
    }

    pub fn to_value<Value: ScfiaValue>(
        &self,
        old_state: &SystemState,
        new_symbol_manager: &mut ScfiaSymbolManager,
    ) -> Value {
        match self {
            ScfiaTransformationFunctionExpression::Bits64(value, width) => {
                Value::new_concrete(*value, *width)
            }
            ScfiaTransformationFunctionExpression::Var(transformation_definition) => {
                Value::new_symbolic(
                    transformation_definition.to_symbol(old_state, new_symbol_manager),
                )
            }
            ScfiaTransformationFunctionExpression::Bits(a) => {
                todo!()
            }
            ScfiaTransformationFunctionExpression::Enum(a, b) => {
                panic!()
            }
            ScfiaTransformationFunctionExpression::Bool(a) => {
                panic!()
            }
            ScfiaTransformationFunctionExpression::Eq(a, b) => {
                let a = Arc::new(a.to_scfia_symbol_expression(old_state, new_symbol_manager));
                let b = Arc::new(b.to_scfia_symbol_expression(old_state, new_symbol_manager));
                Value::new_symbolic(
                    new_symbol_manager.create_symbol_defined(ScfiaSymbolExpression::Eq(a, b)),
                )
            }
            ScfiaTransformationFunctionExpression::Neq(a, b) => {
                let a = Arc::new(a.to_scfia_symbol_expression(old_state, new_symbol_manager));
                let b = Arc::new(b.to_scfia_symbol_expression(old_state, new_symbol_manager));
                Value::new_symbolic(
                    new_symbol_manager.create_symbol_defined(ScfiaSymbolExpression::Neq(a, b)),
                )
            }
            ScfiaTransformationFunctionExpression::And(a, b) => {
                let a = Arc::new(a.to_scfia_symbol_expression(old_state, new_symbol_manager));
                let b = Arc::new(b.to_scfia_symbol_expression(old_state, new_symbol_manager));
                Value::new_symbolic(
                    new_symbol_manager.create_symbol_defined(ScfiaSymbolExpression::And(a, b)),
                )
            }
            ScfiaTransformationFunctionExpression::Or(a, b) => {
                let a = Arc::new(a.to_scfia_symbol_expression(old_state, new_symbol_manager));
                let b = Arc::new(b.to_scfia_symbol_expression(old_state, new_symbol_manager));
                Value::new_symbolic(
                    new_symbol_manager.create_symbol_defined(ScfiaSymbolExpression::Or(a, b)),
                )
            }
            ScfiaTransformationFunctionExpression::Not(a) => {
                let a = Arc::new(a.to_scfia_symbol_expression(old_state, new_symbol_manager));
                Value::new_symbolic(
                    new_symbol_manager.create_symbol_defined(ScfiaSymbolExpression::Not(a)),
                )
            }
            ScfiaTransformationFunctionExpression::Bvnot(a) => {
                let a = Arc::new(a.to_scfia_symbol_expression(old_state, new_symbol_manager));
                Value::new_symbolic(
                    new_symbol_manager.create_symbol_defined(ScfiaSymbolExpression::Bvnot(a)),
                )
            }
            ScfiaTransformationFunctionExpression::Bvand(a, b) => {
                let a = Arc::new(a.to_scfia_symbol_expression(old_state, new_symbol_manager));
                let b = Arc::new(b.to_scfia_symbol_expression(old_state, new_symbol_manager));
                Value::new_symbolic(
                    new_symbol_manager.create_symbol_defined(ScfiaSymbolExpression::Bvand(a, b)),
                )
            }
            ScfiaTransformationFunctionExpression::Bvor(a, b) => {
                let a = Arc::new(a.to_scfia_symbol_expression(old_state, new_symbol_manager));
                let b = Arc::new(b.to_scfia_symbol_expression(old_state, new_symbol_manager));
                Value::new_symbolic(
                    new_symbol_manager.create_symbol_defined(ScfiaSymbolExpression::Bvor(a, b)),
                )
            }
            ScfiaTransformationFunctionExpression::Bvxor(a, b) => {
                let a = Arc::new(a.to_scfia_symbol_expression(old_state, new_symbol_manager));
                let b = Arc::new(b.to_scfia_symbol_expression(old_state, new_symbol_manager));
                Value::new_symbolic(
                    new_symbol_manager.create_symbol_defined(ScfiaSymbolExpression::Bvxor(a, b)),
                )
            }
            ScfiaTransformationFunctionExpression::Bvnand(a, b) => {
                let a = Arc::new(a.to_scfia_symbol_expression(old_state, new_symbol_manager));
                let b = Arc::new(b.to_scfia_symbol_expression(old_state, new_symbol_manager));
                Value::new_symbolic(
                    new_symbol_manager.create_symbol_defined(ScfiaSymbolExpression::Bvnand(a, b)),
                )
            }
            ScfiaTransformationFunctionExpression::Bvnor(a, b) => {
                let a = Arc::new(a.to_scfia_symbol_expression(old_state, new_symbol_manager));
                let b = Arc::new(b.to_scfia_symbol_expression(old_state, new_symbol_manager));
                Value::new_symbolic(
                    new_symbol_manager.create_symbol_defined(ScfiaSymbolExpression::Bvnor(a, b)),
                )
            }
            ScfiaTransformationFunctionExpression::Bvxnor(a, b) => {
                let a = Arc::new(a.to_scfia_symbol_expression(old_state, new_symbol_manager));
                let b = Arc::new(b.to_scfia_symbol_expression(old_state, new_symbol_manager));
                Value::new_symbolic(
                    new_symbol_manager.create_symbol_defined(ScfiaSymbolExpression::Bvxnor(a, b)),
                )
            }
            ScfiaTransformationFunctionExpression::Bvneg(a) => {
                let a = Arc::new(a.to_scfia_symbol_expression(old_state, new_symbol_manager));
                Value::new_symbolic(
                    new_symbol_manager.create_symbol_defined(ScfiaSymbolExpression::Bvneg(a)),
                )
            }
            ScfiaTransformationFunctionExpression::Bvadd(a, b) => {
                let a = Arc::new(a.to_scfia_symbol_expression(old_state, new_symbol_manager));
                let b = Arc::new(b.to_scfia_symbol_expression(old_state, new_symbol_manager));
                Value::new_symbolic(
                    new_symbol_manager.create_symbol_defined(ScfiaSymbolExpression::Bvadd(a, b)),
                )
            }
            ScfiaTransformationFunctionExpression::Bvsub(a, b) => {
                let a = Arc::new(a.to_scfia_symbol_expression(old_state, new_symbol_manager));
                let b = Arc::new(b.to_scfia_symbol_expression(old_state, new_symbol_manager));
                Value::new_symbolic(
                    new_symbol_manager.create_symbol_defined(ScfiaSymbolExpression::Bvsub(a, b)),
                )
            }
            ScfiaTransformationFunctionExpression::Bvmul(a, b) => {
                let a = Arc::new(a.to_scfia_symbol_expression(old_state, new_symbol_manager));
                let b = Arc::new(b.to_scfia_symbol_expression(old_state, new_symbol_manager));
                Value::new_symbolic(
                    new_symbol_manager.create_symbol_defined(ScfiaSymbolExpression::Bvmul(a, b)),
                )
            }
            ScfiaTransformationFunctionExpression::Bvudiv(a, b) => {
                let a = Arc::new(a.to_scfia_symbol_expression(old_state, new_symbol_manager));
                let b = Arc::new(b.to_scfia_symbol_expression(old_state, new_symbol_manager));
                Value::new_symbolic(
                    new_symbol_manager.create_symbol_defined(ScfiaSymbolExpression::Bvudiv(a, b)),
                )
            }
            ScfiaTransformationFunctionExpression::Bvsdiv(a, b) => {
                let a = Arc::new(a.to_scfia_symbol_expression(old_state, new_symbol_manager));
                let b = Arc::new(b.to_scfia_symbol_expression(old_state, new_symbol_manager));
                Value::new_symbolic(
                    new_symbol_manager.create_symbol_defined(ScfiaSymbolExpression::Bvsdiv(a, b)),
                )
            }
            ScfiaTransformationFunctionExpression::Bvurem(a, b) => {
                let a = Arc::new(a.to_scfia_symbol_expression(old_state, new_symbol_manager));
                let b = Arc::new(b.to_scfia_symbol_expression(old_state, new_symbol_manager));
                Value::new_symbolic(
                    new_symbol_manager.create_symbol_defined(ScfiaSymbolExpression::Bvurem(a, b)),
                )
            }
            ScfiaTransformationFunctionExpression::Bvsrem(a, b) => {
                let a = Arc::new(a.to_scfia_symbol_expression(old_state, new_symbol_manager));
                let b = Arc::new(b.to_scfia_symbol_expression(old_state, new_symbol_manager));
                Value::new_symbolic(
                    new_symbol_manager.create_symbol_defined(ScfiaSymbolExpression::Bvsrem(a, b)),
                )
            }
            ScfiaTransformationFunctionExpression::Bvsmod(a, b) => {
                let a = Arc::new(a.to_scfia_symbol_expression(old_state, new_symbol_manager));
                let b = Arc::new(b.to_scfia_symbol_expression(old_state, new_symbol_manager));
                Value::new_symbolic(
                    new_symbol_manager.create_symbol_defined(ScfiaSymbolExpression::Bvsmod(a, b)),
                )
            }
            ScfiaTransformationFunctionExpression::Bvult(a, b) => {
                let a = Arc::new(a.to_scfia_symbol_expression(old_state, new_symbol_manager));
                let b = Arc::new(b.to_scfia_symbol_expression(old_state, new_symbol_manager));
                Value::new_symbolic(
                    new_symbol_manager.create_symbol_defined(ScfiaSymbolExpression::Bvult(a, b)),
                )
            }
            ScfiaTransformationFunctionExpression::Bvslt(a, b) => {
                let a = Arc::new(a.to_scfia_symbol_expression(old_state, new_symbol_manager));
                let b = Arc::new(b.to_scfia_symbol_expression(old_state, new_symbol_manager));
                Value::new_symbolic(
                    new_symbol_manager.create_symbol_defined(ScfiaSymbolExpression::Bvslt(a, b)),
                )
            }
            ScfiaTransformationFunctionExpression::Bvule(a, b) => {
                let a = Arc::new(a.to_scfia_symbol_expression(old_state, new_symbol_manager));
                let b = Arc::new(b.to_scfia_symbol_expression(old_state, new_symbol_manager));
                Value::new_symbolic(
                    new_symbol_manager.create_symbol_defined(ScfiaSymbolExpression::Bvule(a, b)),
                )
            }
            ScfiaTransformationFunctionExpression::Bvsle(a, b) => {
                let a = Arc::new(a.to_scfia_symbol_expression(old_state, new_symbol_manager));
                let b = Arc::new(b.to_scfia_symbol_expression(old_state, new_symbol_manager));
                Value::new_symbolic(
                    new_symbol_manager.create_symbol_defined(ScfiaSymbolExpression::Bvsle(a, b)),
                )
            }
            ScfiaTransformationFunctionExpression::Bvuge(a, b) => {
                let a = Arc::new(a.to_scfia_symbol_expression(old_state, new_symbol_manager));
                let b = Arc::new(b.to_scfia_symbol_expression(old_state, new_symbol_manager));
                Value::new_symbolic(
                    new_symbol_manager.create_symbol_defined(ScfiaSymbolExpression::Bvuge(a, b)),
                )
            }
            ScfiaTransformationFunctionExpression::Bvsge(a, b) => {
                let a = Arc::new(a.to_scfia_symbol_expression(old_state, new_symbol_manager));
                let b = Arc::new(b.to_scfia_symbol_expression(old_state, new_symbol_manager));
                Value::new_symbolic(
                    new_symbol_manager.create_symbol_defined(ScfiaSymbolExpression::Bvsge(a, b)),
                )
            }
            ScfiaTransformationFunctionExpression::Bvugt(a, b) => {
                let a = Arc::new(a.to_scfia_symbol_expression(old_state, new_symbol_manager));
                let b = Arc::new(b.to_scfia_symbol_expression(old_state, new_symbol_manager));
                Value::new_symbolic(
                    new_symbol_manager.create_symbol_defined(ScfiaSymbolExpression::Bvugt(a, b)),
                )
            }
            ScfiaTransformationFunctionExpression::Bvsgt(a, b) => {
                let a = Arc::new(a.to_scfia_symbol_expression(old_state, new_symbol_manager));
                let b = Arc::new(b.to_scfia_symbol_expression(old_state, new_symbol_manager));
                Value::new_symbolic(
                    new_symbol_manager.create_symbol_defined(ScfiaSymbolExpression::Bvsgt(a, b)),
                )
            }
            ScfiaTransformationFunctionExpression::Extract(a, b, c) => {
                let c = Arc::new(c.to_scfia_symbol_expression(old_state, new_symbol_manager));
                Value::new_symbolic(
                    new_symbol_manager
                        .create_symbol_defined(ScfiaSymbolExpression::Extract(*a, *b, c)),
                )
            }
            ScfiaTransformationFunctionExpression::ZeroExtend(a, b) => {
                let b = Arc::new(b.to_scfia_symbol_expression(old_state, new_symbol_manager));
                Value::new_symbolic(
                    new_symbol_manager
                        .create_symbol_defined(ScfiaSymbolExpression::ZeroExtend(*a, b)),
                )
            }
            ScfiaTransformationFunctionExpression::SignExtend(a, b) => {
                let b = Arc::new(b.to_scfia_symbol_expression(old_state, new_symbol_manager));
                Value::new_symbolic(
                    new_symbol_manager
                        .create_symbol_defined(ScfiaSymbolExpression::SignExtend(*a, b)),
                )
            }
            ScfiaTransformationFunctionExpression::Bvshl(a, b) => {
                let a = Arc::new(a.to_scfia_symbol_expression(old_state, new_symbol_manager));
                let b = Arc::new(b.to_scfia_symbol_expression(old_state, new_symbol_manager));
                Value::new_symbolic(
                    new_symbol_manager.create_symbol_defined(ScfiaSymbolExpression::Bvshl(a, b)),
                )
            }
            ScfiaTransformationFunctionExpression::Bvlshr(a, b) => {
                let a = Arc::new(a.to_scfia_symbol_expression(old_state, new_symbol_manager));
                let b = Arc::new(b.to_scfia_symbol_expression(old_state, new_symbol_manager));
                Value::new_symbolic(
                    new_symbol_manager.create_symbol_defined(ScfiaSymbolExpression::Bvlshr(a, b)),
                )
            }
            ScfiaTransformationFunctionExpression::Bvashr(a, b) => {
                let a = Arc::new(a.to_scfia_symbol_expression(old_state, new_symbol_manager));
                let b = Arc::new(b.to_scfia_symbol_expression(old_state, new_symbol_manager));
                Value::new_symbolic(
                    new_symbol_manager.create_symbol_defined(ScfiaSymbolExpression::Bvashr(a, b)),
                )
            }
            ScfiaTransformationFunctionExpression::Concat(a, b) => {
                let a = Arc::new(a.to_scfia_symbol_expression(old_state, new_symbol_manager));
                let b = Arc::new(b.to_scfia_symbol_expression(old_state, new_symbol_manager));
                Value::new_symbolic(
                    new_symbol_manager.create_symbol_defined(ScfiaSymbolExpression::Concat(a, b)),
                )
            }
            ScfiaTransformationFunctionExpression::Ite(a, b, c) => {
                let a = Arc::new(a.to_scfia_symbol_expression(old_state, new_symbol_manager));
                let b = Arc::new(b.to_scfia_symbol_expression(old_state, new_symbol_manager));
                let c = Arc::new(c.to_scfia_symbol_expression(old_state, new_symbol_manager));
                Value::new_symbolic(
                    new_symbol_manager.create_symbol_defined(ScfiaSymbolExpression::Ite(a, b, c)),
                )
            }
            ScfiaTransformationFunctionExpression::Select(a, b) => {
                let a = Arc::new(a.to_scfia_symbol_expression(old_state, new_symbol_manager));
                let b = Arc::new(b.to_scfia_symbol_expression(old_state, new_symbol_manager));
                Value::new_symbolic(
                    new_symbol_manager.create_symbol_defined(ScfiaSymbolExpression::Select(a, b)),
                )
            }
            ScfiaTransformationFunctionExpression::Store(a, b, c) => {
                let a = Arc::new(a.to_scfia_symbol_expression(old_state, new_symbol_manager));
                let b = Arc::new(b.to_scfia_symbol_expression(old_state, new_symbol_manager));
                let c = Arc::new(c.to_scfia_symbol_expression(old_state, new_symbol_manager));
                Value::new_symbolic(
                    new_symbol_manager.create_symbol_defined(ScfiaSymbolExpression::Store(a, b, c)),
                )
            }
        }
    }

    pub fn to_scfia_symbol_expression(
        &self,
        old_state: &SystemState,
        new_symbol_manager: &mut ScfiaSymbolManager,
    ) -> ScfiaSymbolExpression {
        match self {
            ScfiaTransformationFunctionExpression::Bits64(value, width) => {
                ScfiaSymbolExpression::Bits64(*value, *width)
            }
            ScfiaTransformationFunctionExpression::Var(transformation_definition) => {
                ScfiaSymbolExpression::Var(
                    transformation_definition.to_symbol(old_state, new_symbol_manager),
                )
            }
            ScfiaTransformationFunctionExpression::Bits(a) => {
                ScfiaSymbolExpression::Bits(a.clone())
            }
            ScfiaTransformationFunctionExpression::Enum(a, b) => {
                panic!()
            }
            ScfiaTransformationFunctionExpression::Bool(a) => ScfiaSymbolExpression::Bool(*a),
            ScfiaTransformationFunctionExpression::Eq(a, b) => {
                let a = Arc::new(a.to_scfia_symbol_expression(old_state, new_symbol_manager));
                let b = Arc::new(b.to_scfia_symbol_expression(old_state, new_symbol_manager));
                ScfiaSymbolExpression::Eq(a, b)
            }
            ScfiaTransformationFunctionExpression::Neq(a, b) => {
                let a = Arc::new(a.to_scfia_symbol_expression(old_state, new_symbol_manager));
                let b = Arc::new(b.to_scfia_symbol_expression(old_state, new_symbol_manager));
                ScfiaSymbolExpression::Neq(a, b)
            }
            ScfiaTransformationFunctionExpression::And(a, b) => {
                let a = Arc::new(a.to_scfia_symbol_expression(old_state, new_symbol_manager));
                let b = Arc::new(b.to_scfia_symbol_expression(old_state, new_symbol_manager));
                ScfiaSymbolExpression::And(a, b)
            }
            ScfiaTransformationFunctionExpression::Or(a, b) => {
                let a = Arc::new(a.to_scfia_symbol_expression(old_state, new_symbol_manager));
                let b = Arc::new(b.to_scfia_symbol_expression(old_state, new_symbol_manager));
                ScfiaSymbolExpression::Or(a, b)
            }
            ScfiaTransformationFunctionExpression::Not(a) => {
                let a = Arc::new(a.to_scfia_symbol_expression(old_state, new_symbol_manager));
                ScfiaSymbolExpression::Not(a)
            }
            ScfiaTransformationFunctionExpression::Bvnot(a) => {
                let a = Arc::new(a.to_scfia_symbol_expression(old_state, new_symbol_manager));
                ScfiaSymbolExpression::Bvnot(a)
            }
            ScfiaTransformationFunctionExpression::Bvand(a, b) => {
                let a = Arc::new(a.to_scfia_symbol_expression(old_state, new_symbol_manager));
                let b = Arc::new(b.to_scfia_symbol_expression(old_state, new_symbol_manager));
                ScfiaSymbolExpression::Bvand(a, b)
            }
            ScfiaTransformationFunctionExpression::Bvor(a, b) => {
                let a = Arc::new(a.to_scfia_symbol_expression(old_state, new_symbol_manager));
                let b = Arc::new(b.to_scfia_symbol_expression(old_state, new_symbol_manager));
                ScfiaSymbolExpression::Bvor(a, b)
            }
            ScfiaTransformationFunctionExpression::Bvxor(a, b) => {
                let a = Arc::new(a.to_scfia_symbol_expression(old_state, new_symbol_manager));
                let b = Arc::new(b.to_scfia_symbol_expression(old_state, new_symbol_manager));
                ScfiaSymbolExpression::Bvxor(a, b)
            }
            ScfiaTransformationFunctionExpression::Bvnand(a, b) => {
                let a = Arc::new(a.to_scfia_symbol_expression(old_state, new_symbol_manager));
                let b = Arc::new(b.to_scfia_symbol_expression(old_state, new_symbol_manager));
                ScfiaSymbolExpression::Bvnand(a, b)
            }
            ScfiaTransformationFunctionExpression::Bvnor(a, b) => {
                let a = Arc::new(a.to_scfia_symbol_expression(old_state, new_symbol_manager));
                let b = Arc::new(b.to_scfia_symbol_expression(old_state, new_symbol_manager));
                ScfiaSymbolExpression::Bvnor(a, b)
            }
            ScfiaTransformationFunctionExpression::Bvxnor(a, b) => {
                let a = Arc::new(a.to_scfia_symbol_expression(old_state, new_symbol_manager));
                let b = Arc::new(b.to_scfia_symbol_expression(old_state, new_symbol_manager));
                ScfiaSymbolExpression::Bvxnor(a, b)
            }
            ScfiaTransformationFunctionExpression::Bvneg(a) => {
                let a = Arc::new(a.to_scfia_symbol_expression(old_state, new_symbol_manager));
                ScfiaSymbolExpression::Bvneg(a)
            }
            ScfiaTransformationFunctionExpression::Bvadd(a, b) => {
                let a = Arc::new(a.to_scfia_symbol_expression(old_state, new_symbol_manager));
                let b = Arc::new(b.to_scfia_symbol_expression(old_state, new_symbol_manager));
                ScfiaSymbolExpression::Bvadd(a, b)
            }
            ScfiaTransformationFunctionExpression::Bvsub(a, b) => {
                let a = Arc::new(a.to_scfia_symbol_expression(old_state, new_symbol_manager));
                let b = Arc::new(b.to_scfia_symbol_expression(old_state, new_symbol_manager));
                ScfiaSymbolExpression::Bvsub(a, b)
            }
            ScfiaTransformationFunctionExpression::Bvmul(a, b) => {
                let a = Arc::new(a.to_scfia_symbol_expression(old_state, new_symbol_manager));
                let b = Arc::new(b.to_scfia_symbol_expression(old_state, new_symbol_manager));
                ScfiaSymbolExpression::Bvmul(a, b)
            }
            ScfiaTransformationFunctionExpression::Bvudiv(a, b) => {
                let a = Arc::new(a.to_scfia_symbol_expression(old_state, new_symbol_manager));
                let b = Arc::new(b.to_scfia_symbol_expression(old_state, new_symbol_manager));
                ScfiaSymbolExpression::Bvudiv(a, b)
            }
            ScfiaTransformationFunctionExpression::Bvsdiv(a, b) => {
                let a = Arc::new(a.to_scfia_symbol_expression(old_state, new_symbol_manager));
                let b = Arc::new(b.to_scfia_symbol_expression(old_state, new_symbol_manager));
                ScfiaSymbolExpression::Bvsdiv(a, b)
            }
            ScfiaTransformationFunctionExpression::Bvurem(a, b) => {
                let a = Arc::new(a.to_scfia_symbol_expression(old_state, new_symbol_manager));
                let b = Arc::new(b.to_scfia_symbol_expression(old_state, new_symbol_manager));
                ScfiaSymbolExpression::Bvurem(a, b)
            }
            ScfiaTransformationFunctionExpression::Bvsrem(a, b) => {
                let a = Arc::new(a.to_scfia_symbol_expression(old_state, new_symbol_manager));
                let b = Arc::new(b.to_scfia_symbol_expression(old_state, new_symbol_manager));
                ScfiaSymbolExpression::Bvsrem(a, b)
            }
            ScfiaTransformationFunctionExpression::Bvsmod(a, b) => {
                let a = Arc::new(a.to_scfia_symbol_expression(old_state, new_symbol_manager));
                let b = Arc::new(b.to_scfia_symbol_expression(old_state, new_symbol_manager));
                ScfiaSymbolExpression::Bvsmod(a, b)
            }
            ScfiaTransformationFunctionExpression::Bvult(a, b) => {
                let a = Arc::new(a.to_scfia_symbol_expression(old_state, new_symbol_manager));
                let b = Arc::new(b.to_scfia_symbol_expression(old_state, new_symbol_manager));
                ScfiaSymbolExpression::Bvult(a, b)
            }
            ScfiaTransformationFunctionExpression::Bvslt(a, b) => {
                let a = Arc::new(a.to_scfia_symbol_expression(old_state, new_symbol_manager));
                let b = Arc::new(b.to_scfia_symbol_expression(old_state, new_symbol_manager));
                ScfiaSymbolExpression::Bvslt(a, b)
            }
            ScfiaTransformationFunctionExpression::Bvule(a, b) => {
                let a = Arc::new(a.to_scfia_symbol_expression(old_state, new_symbol_manager));
                let b = Arc::new(b.to_scfia_symbol_expression(old_state, new_symbol_manager));
                ScfiaSymbolExpression::Bvule(a, b)
            }
            ScfiaTransformationFunctionExpression::Bvsle(a, b) => {
                let a = Arc::new(a.to_scfia_symbol_expression(old_state, new_symbol_manager));
                let b = Arc::new(b.to_scfia_symbol_expression(old_state, new_symbol_manager));
                ScfiaSymbolExpression::Bvsle(a, b)
            }
            ScfiaTransformationFunctionExpression::Bvuge(a, b) => {
                let a = Arc::new(a.to_scfia_symbol_expression(old_state, new_symbol_manager));
                let b = Arc::new(b.to_scfia_symbol_expression(old_state, new_symbol_manager));
                ScfiaSymbolExpression::Bvuge(a, b)
            }
            ScfiaTransformationFunctionExpression::Bvsge(a, b) => {
                let a = Arc::new(a.to_scfia_symbol_expression(old_state, new_symbol_manager));
                let b = Arc::new(b.to_scfia_symbol_expression(old_state, new_symbol_manager));
                ScfiaSymbolExpression::Bvsge(a, b)
            }
            ScfiaTransformationFunctionExpression::Bvugt(a, b) => {
                let a = Arc::new(a.to_scfia_symbol_expression(old_state, new_symbol_manager));
                let b = Arc::new(b.to_scfia_symbol_expression(old_state, new_symbol_manager));
                ScfiaSymbolExpression::Bvugt(a, b)
            }
            ScfiaTransformationFunctionExpression::Bvsgt(a, b) => {
                let a = Arc::new(a.to_scfia_symbol_expression(old_state, new_symbol_manager));
                let b = Arc::new(b.to_scfia_symbol_expression(old_state, new_symbol_manager));
                ScfiaSymbolExpression::Bvsgt(a, b)
            }
            ScfiaTransformationFunctionExpression::Extract(a, b, c) => {
                let c = Arc::new(c.to_scfia_symbol_expression(old_state, new_symbol_manager));
                ScfiaSymbolExpression::Extract(*a, *b, c)
            }
            ScfiaTransformationFunctionExpression::ZeroExtend(a, b) => {
                let b = Arc::new(b.to_scfia_symbol_expression(old_state, new_symbol_manager));
                ScfiaSymbolExpression::ZeroExtend(*a, b)
            }
            ScfiaTransformationFunctionExpression::SignExtend(a, b) => {
                let b = Arc::new(b.to_scfia_symbol_expression(old_state, new_symbol_manager));
                ScfiaSymbolExpression::SignExtend(*a, b)
            }
            ScfiaTransformationFunctionExpression::Bvshl(a, b) => {
                let a = Arc::new(a.to_scfia_symbol_expression(old_state, new_symbol_manager));
                let b = Arc::new(b.to_scfia_symbol_expression(old_state, new_symbol_manager));
                ScfiaSymbolExpression::Bvshl(a, b)
            }
            ScfiaTransformationFunctionExpression::Bvlshr(a, b) => {
                let a = Arc::new(a.to_scfia_symbol_expression(old_state, new_symbol_manager));
                let b = Arc::new(b.to_scfia_symbol_expression(old_state, new_symbol_manager));
                ScfiaSymbolExpression::Bvlshr(a, b)
            }
            ScfiaTransformationFunctionExpression::Bvashr(a, b) => {
                let a = Arc::new(a.to_scfia_symbol_expression(old_state, new_symbol_manager));
                let b = Arc::new(b.to_scfia_symbol_expression(old_state, new_symbol_manager));
                ScfiaSymbolExpression::Bvashr(a, b)
            }
            ScfiaTransformationFunctionExpression::Concat(a, b) => {
                let a = Arc::new(a.to_scfia_symbol_expression(old_state, new_symbol_manager));
                let b = Arc::new(b.to_scfia_symbol_expression(old_state, new_symbol_manager));
                ScfiaSymbolExpression::Concat(a, b)
            }
            ScfiaTransformationFunctionExpression::Ite(a, b, c) => {
                let a = Arc::new(a.to_scfia_symbol_expression(old_state, new_symbol_manager));
                let b = Arc::new(b.to_scfia_symbol_expression(old_state, new_symbol_manager));
                let c = Arc::new(c.to_scfia_symbol_expression(old_state, new_symbol_manager));
                ScfiaSymbolExpression::Ite(a, b, c)
            }
            ScfiaTransformationFunctionExpression::Select(a, b) => {
                let a = Arc::new(a.to_scfia_symbol_expression(old_state, new_symbol_manager));
                let b = Arc::new(b.to_scfia_symbol_expression(old_state, new_symbol_manager));
                ScfiaSymbolExpression::Select(a, b)
            }
            ScfiaTransformationFunctionExpression::Store(a, b, c) => {
                let a = Arc::new(a.to_scfia_symbol_expression(old_state, new_symbol_manager));
                let b = Arc::new(b.to_scfia_symbol_expression(old_state, new_symbol_manager));
                let c = Arc::new(c.to_scfia_symbol_expression(old_state, new_symbol_manager));
                ScfiaSymbolExpression::Store(a, b, c)
            }
        }
    }

    pub fn to_assert_expression(
        &self,
        old_state: &SystemState,
        new_symbol_manager: &mut ScfiaSymbolManager,
        affected_symbol_ids: &mut Vec<u64>,
    ) -> ScfiaAssertExpression {
        match self {
            ScfiaTransformationFunctionExpression::Var(s) => {
                let symbol_id = s.to_symbol(old_state, new_symbol_manager);
                affected_symbol_ids.push(symbol_id);
                ScfiaAssertExpression::Var(symbol_id)
            }
            ScfiaTransformationFunctionExpression::Bits(a) => {
                ScfiaAssertExpression::Bits(a.clone())
            }
            ScfiaTransformationFunctionExpression::Bits64(a, b) => {
                ScfiaAssertExpression::Bits64(*a, *b)
            }
            ScfiaTransformationFunctionExpression::Enum(a, b) => {
                ScfiaAssertExpression::Enum(*a, *b)
            }
            ScfiaTransformationFunctionExpression::Bool(a) => ScfiaAssertExpression::Bool(*a),
            ScfiaTransformationFunctionExpression::Eq(a, b) => {
                let a = Arc::new(a.to_assert_expression(
                    old_state,
                    new_symbol_manager,
                    affected_symbol_ids,
                ));
                let b = Arc::new(b.to_assert_expression(
                    old_state,
                    new_symbol_manager,
                    affected_symbol_ids,
                ));
                ScfiaAssertExpression::Eq(a, b)
            }
            ScfiaTransformationFunctionExpression::Neq(a, b) => {
                let a = Arc::new(a.to_assert_expression(
                    old_state,
                    new_symbol_manager,
                    affected_symbol_ids,
                ));
                let b = Arc::new(b.to_assert_expression(
                    old_state,
                    new_symbol_manager,
                    affected_symbol_ids,
                ));
                ScfiaAssertExpression::Neq(a, b)
            }
            ScfiaTransformationFunctionExpression::And(a, b) => {
                let a = Arc::new(a.to_assert_expression(
                    old_state,
                    new_symbol_manager,
                    affected_symbol_ids,
                ));
                let b = Arc::new(b.to_assert_expression(
                    old_state,
                    new_symbol_manager,
                    affected_symbol_ids,
                ));
                ScfiaAssertExpression::And(a, b)
            }
            ScfiaTransformationFunctionExpression::Or(a, b) => {
                let a = Arc::new(a.to_assert_expression(
                    old_state,
                    new_symbol_manager,
                    affected_symbol_ids,
                ));
                let b = Arc::new(b.to_assert_expression(
                    old_state,
                    new_symbol_manager,
                    affected_symbol_ids,
                ));
                ScfiaAssertExpression::Or(a, b)
            }
            ScfiaTransformationFunctionExpression::Not(a) => {
                let a = Arc::new(a.to_assert_expression(
                    old_state,
                    new_symbol_manager,
                    affected_symbol_ids,
                ));
                ScfiaAssertExpression::Not(a)
            }
            ScfiaTransformationFunctionExpression::Bvnot(a) => {
                let a = Arc::new(a.to_assert_expression(
                    old_state,
                    new_symbol_manager,
                    affected_symbol_ids,
                ));
                ScfiaAssertExpression::Bvnot(a)
            }
            ScfiaTransformationFunctionExpression::Bvand(a, b) => {
                let a = Arc::new(a.to_assert_expression(
                    old_state,
                    new_symbol_manager,
                    affected_symbol_ids,
                ));
                let b = Arc::new(b.to_assert_expression(
                    old_state,
                    new_symbol_manager,
                    affected_symbol_ids,
                ));
                ScfiaAssertExpression::Bvand(a, b)
            }
            ScfiaTransformationFunctionExpression::Bvor(a, b) => {
                let a = Arc::new(a.to_assert_expression(
                    old_state,
                    new_symbol_manager,
                    affected_symbol_ids,
                ));
                let b = Arc::new(b.to_assert_expression(
                    old_state,
                    new_symbol_manager,
                    affected_symbol_ids,
                ));
                ScfiaAssertExpression::Bvor(a, b)
            }
            ScfiaTransformationFunctionExpression::Bvxor(a, b) => {
                let a = Arc::new(a.to_assert_expression(
                    old_state,
                    new_symbol_manager,
                    affected_symbol_ids,
                ));
                let b = Arc::new(b.to_assert_expression(
                    old_state,
                    new_symbol_manager,
                    affected_symbol_ids,
                ));
                ScfiaAssertExpression::Bvxor(a, b)
            }
            ScfiaTransformationFunctionExpression::Bvnand(a, b) => {
                let a = Arc::new(a.to_assert_expression(
                    old_state,
                    new_symbol_manager,
                    affected_symbol_ids,
                ));
                let b = Arc::new(b.to_assert_expression(
                    old_state,
                    new_symbol_manager,
                    affected_symbol_ids,
                ));
                ScfiaAssertExpression::Bvnand(a, b)
            }
            ScfiaTransformationFunctionExpression::Bvnor(a, b) => {
                let a = Arc::new(a.to_assert_expression(
                    old_state,
                    new_symbol_manager,
                    affected_symbol_ids,
                ));
                let b = Arc::new(b.to_assert_expression(
                    old_state,
                    new_symbol_manager,
                    affected_symbol_ids,
                ));
                ScfiaAssertExpression::Bvnor(a, b)
            }
            ScfiaTransformationFunctionExpression::Bvxnor(a, b) => {
                let a = Arc::new(a.to_assert_expression(
                    old_state,
                    new_symbol_manager,
                    affected_symbol_ids,
                ));
                let b = Arc::new(b.to_assert_expression(
                    old_state,
                    new_symbol_manager,
                    affected_symbol_ids,
                ));
                ScfiaAssertExpression::Bvxnor(a, b)
            }
            ScfiaTransformationFunctionExpression::Bvneg(a) => {
                let a = Arc::new(a.to_assert_expression(
                    old_state,
                    new_symbol_manager,
                    affected_symbol_ids,
                ));
                ScfiaAssertExpression::Bvneg(a)
            }
            ScfiaTransformationFunctionExpression::Bvadd(a, b) => {
                let a = Arc::new(a.to_assert_expression(
                    old_state,
                    new_symbol_manager,
                    affected_symbol_ids,
                ));
                let b = Arc::new(b.to_assert_expression(
                    old_state,
                    new_symbol_manager,
                    affected_symbol_ids,
                ));
                ScfiaAssertExpression::Bvadd(a, b)
            }
            ScfiaTransformationFunctionExpression::Bvsub(a, b) => {
                let a = Arc::new(a.to_assert_expression(
                    old_state,
                    new_symbol_manager,
                    affected_symbol_ids,
                ));
                let b = Arc::new(b.to_assert_expression(
                    old_state,
                    new_symbol_manager,
                    affected_symbol_ids,
                ));
                ScfiaAssertExpression::Bvsub(a, b)
            }
            ScfiaTransformationFunctionExpression::Bvmul(a, b) => {
                let a = Arc::new(a.to_assert_expression(
                    old_state,
                    new_symbol_manager,
                    affected_symbol_ids,
                ));
                let b = Arc::new(b.to_assert_expression(
                    old_state,
                    new_symbol_manager,
                    affected_symbol_ids,
                ));
                ScfiaAssertExpression::Bvmul(a, b)
            }
            ScfiaTransformationFunctionExpression::Bvudiv(a, b) => {
                let a = Arc::new(a.to_assert_expression(
                    old_state,
                    new_symbol_manager,
                    affected_symbol_ids,
                ));
                let b = Arc::new(b.to_assert_expression(
                    old_state,
                    new_symbol_manager,
                    affected_symbol_ids,
                ));
                ScfiaAssertExpression::Bvudiv(a, b)
            }
            ScfiaTransformationFunctionExpression::Bvsdiv(a, b) => {
                let a = Arc::new(a.to_assert_expression(
                    old_state,
                    new_symbol_manager,
                    affected_symbol_ids,
                ));
                let b = Arc::new(b.to_assert_expression(
                    old_state,
                    new_symbol_manager,
                    affected_symbol_ids,
                ));
                ScfiaAssertExpression::Bvsdiv(a, b)
            }
            ScfiaTransformationFunctionExpression::Bvurem(a, b) => {
                let a = Arc::new(a.to_assert_expression(
                    old_state,
                    new_symbol_manager,
                    affected_symbol_ids,
                ));
                let b = Arc::new(b.to_assert_expression(
                    old_state,
                    new_symbol_manager,
                    affected_symbol_ids,
                ));
                ScfiaAssertExpression::Bvurem(a, b)
            }
            ScfiaTransformationFunctionExpression::Bvsrem(a, b) => {
                let a = Arc::new(a.to_assert_expression(
                    old_state,
                    new_symbol_manager,
                    affected_symbol_ids,
                ));
                let b = Arc::new(b.to_assert_expression(
                    old_state,
                    new_symbol_manager,
                    affected_symbol_ids,
                ));
                ScfiaAssertExpression::Bvsrem(a, b)
            }
            ScfiaTransformationFunctionExpression::Bvsmod(a, b) => {
                let a = Arc::new(a.to_assert_expression(
                    old_state,
                    new_symbol_manager,
                    affected_symbol_ids,
                ));
                let b = Arc::new(b.to_assert_expression(
                    old_state,
                    new_symbol_manager,
                    affected_symbol_ids,
                ));
                ScfiaAssertExpression::Bvsmod(a, b)
            }
            ScfiaTransformationFunctionExpression::Bvult(a, b) => {
                let a = Arc::new(a.to_assert_expression(
                    old_state,
                    new_symbol_manager,
                    affected_symbol_ids,
                ));
                let b = Arc::new(b.to_assert_expression(
                    old_state,
                    new_symbol_manager,
                    affected_symbol_ids,
                ));
                ScfiaAssertExpression::Bvult(a, b)
            }
            ScfiaTransformationFunctionExpression::Bvslt(a, b) => {
                let a = Arc::new(a.to_assert_expression(
                    old_state,
                    new_symbol_manager,
                    affected_symbol_ids,
                ));
                let b = Arc::new(b.to_assert_expression(
                    old_state,
                    new_symbol_manager,
                    affected_symbol_ids,
                ));
                ScfiaAssertExpression::Bvslt(a, b)
            }
            ScfiaTransformationFunctionExpression::Bvule(a, b) => {
                let a = Arc::new(a.to_assert_expression(
                    old_state,
                    new_symbol_manager,
                    affected_symbol_ids,
                ));
                let b = Arc::new(b.to_assert_expression(
                    old_state,
                    new_symbol_manager,
                    affected_symbol_ids,
                ));
                ScfiaAssertExpression::Bvule(a, b)
            }
            ScfiaTransformationFunctionExpression::Bvsle(a, b) => {
                let a = Arc::new(a.to_assert_expression(
                    old_state,
                    new_symbol_manager,
                    affected_symbol_ids,
                ));
                let b = Arc::new(b.to_assert_expression(
                    old_state,
                    new_symbol_manager,
                    affected_symbol_ids,
                ));
                ScfiaAssertExpression::Bvsle(a, b)
            }
            ScfiaTransformationFunctionExpression::Bvuge(a, b) => {
                let a = Arc::new(a.to_assert_expression(
                    old_state,
                    new_symbol_manager,
                    affected_symbol_ids,
                ));
                let b = Arc::new(b.to_assert_expression(
                    old_state,
                    new_symbol_manager,
                    affected_symbol_ids,
                ));
                ScfiaAssertExpression::Bvuge(a, b)
            }
            ScfiaTransformationFunctionExpression::Bvsge(a, b) => {
                let a = Arc::new(a.to_assert_expression(
                    old_state,
                    new_symbol_manager,
                    affected_symbol_ids,
                ));
                let b = Arc::new(b.to_assert_expression(
                    old_state,
                    new_symbol_manager,
                    affected_symbol_ids,
                ));
                ScfiaAssertExpression::Bvsge(a, b)
            }
            ScfiaTransformationFunctionExpression::Bvugt(a, b) => {
                let a = Arc::new(a.to_assert_expression(
                    old_state,
                    new_symbol_manager,
                    affected_symbol_ids,
                ));
                let b = Arc::new(b.to_assert_expression(
                    old_state,
                    new_symbol_manager,
                    affected_symbol_ids,
                ));
                ScfiaAssertExpression::Bvugt(a, b)
            }
            ScfiaTransformationFunctionExpression::Bvsgt(a, b) => {
                let a = Arc::new(a.to_assert_expression(
                    old_state,
                    new_symbol_manager,
                    affected_symbol_ids,
                ));
                let b = Arc::new(b.to_assert_expression(
                    old_state,
                    new_symbol_manager,
                    affected_symbol_ids,
                ));
                ScfiaAssertExpression::Bvsgt(a, b)
            }
            ScfiaTransformationFunctionExpression::Extract(a, b, c) => {
                let c = Arc::new(c.to_assert_expression(
                    old_state,
                    new_symbol_manager,
                    affected_symbol_ids,
                ));
                ScfiaAssertExpression::Extract(*a, *b, c)
            }
            ScfiaTransformationFunctionExpression::ZeroExtend(a, b) => {
                let b = Arc::new(b.to_assert_expression(
                    old_state,
                    new_symbol_manager,
                    affected_symbol_ids,
                ));
                ScfiaAssertExpression::ZeroExtend(*a, b)
            }
            ScfiaTransformationFunctionExpression::SignExtend(a, b) => {
                let b = Arc::new(b.to_assert_expression(
                    old_state,
                    new_symbol_manager,
                    affected_symbol_ids,
                ));
                ScfiaAssertExpression::SignExtend(*a, b)
            }
            ScfiaTransformationFunctionExpression::Bvshl(a, b) => {
                let a = Arc::new(a.to_assert_expression(
                    old_state,
                    new_symbol_manager,
                    affected_symbol_ids,
                ));
                let b = Arc::new(b.to_assert_expression(
                    old_state,
                    new_symbol_manager,
                    affected_symbol_ids,
                ));
                ScfiaAssertExpression::Bvshl(a, b)
            }
            ScfiaTransformationFunctionExpression::Bvlshr(a, b) => {
                let a = Arc::new(a.to_assert_expression(
                    old_state,
                    new_symbol_manager,
                    affected_symbol_ids,
                ));
                let b = Arc::new(b.to_assert_expression(
                    old_state,
                    new_symbol_manager,
                    affected_symbol_ids,
                ));
                ScfiaAssertExpression::Bvlshr(a, b)
            }
            ScfiaTransformationFunctionExpression::Bvashr(a, b) => {
                let a = Arc::new(a.to_assert_expression(
                    old_state,
                    new_symbol_manager,
                    affected_symbol_ids,
                ));
                let b = Arc::new(b.to_assert_expression(
                    old_state,
                    new_symbol_manager,
                    affected_symbol_ids,
                ));
                ScfiaAssertExpression::Bvashr(a, b)
            }
            ScfiaTransformationFunctionExpression::Concat(a, b) => {
                let a = Arc::new(a.to_assert_expression(
                    old_state,
                    new_symbol_manager,
                    affected_symbol_ids,
                ));
                let b = Arc::new(b.to_assert_expression(
                    old_state,
                    new_symbol_manager,
                    affected_symbol_ids,
                ));
                ScfiaAssertExpression::Concat(a, b)
            }
            ScfiaTransformationFunctionExpression::Ite(a, b, c) => {
                let a = Arc::new(a.to_assert_expression(
                    old_state,
                    new_symbol_manager,
                    affected_symbol_ids,
                ));
                let b = Arc::new(b.to_assert_expression(
                    old_state,
                    new_symbol_manager,
                    affected_symbol_ids,
                ));
                let c = Arc::new(c.to_assert_expression(
                    old_state,
                    new_symbol_manager,
                    affected_symbol_ids,
                ));
                ScfiaAssertExpression::Ite(a, b, c)
            }
            ScfiaTransformationFunctionExpression::Select(a, b) => {
                let a = Arc::new(a.to_assert_expression(
                    old_state,
                    new_symbol_manager,
                    affected_symbol_ids,
                ));
                let b = Arc::new(b.to_assert_expression(
                    old_state,
                    new_symbol_manager,
                    affected_symbol_ids,
                ));
                ScfiaAssertExpression::Select(a, b)
            }
            ScfiaTransformationFunctionExpression::Store(a, b, c) => {
                let a = Arc::new(a.to_assert_expression(
                    old_state,
                    new_symbol_manager,
                    affected_symbol_ids,
                ));
                let b = Arc::new(b.to_assert_expression(
                    old_state,
                    new_symbol_manager,
                    affected_symbol_ids,
                ));
                let c = Arc::new(c.to_assert_expression(
                    old_state,
                    new_symbol_manager,
                    affected_symbol_ids,
                ));
                ScfiaAssertExpression::Store(a, b, c)
            }
        }
    }
}
