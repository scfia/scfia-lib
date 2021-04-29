use std::{collections::HashMap, sync::Arc};

use isla_lib::smt::{smtlib::Exp, Sym};

use crate::{
    system_states::ScfiaSystemState,
    transformation_functions::transformation_function_expression::ScfiaTransformationFunctionExpression,
};

pub fn map_isla_exp_to_transformation_expression<SystemState: ScfiaSystemState>(
    exp: &Exp,
    isla_sym_to_symbol_definition_mapping: &mut HashMap<
        Sym,
        Arc<SystemState::TransformationDefinition>,
    >,
) -> Option<ScfiaTransformationFunctionExpression<SystemState>> {
    match exp {
        Exp::Bvor(a, b) => {
            let a = match map_isla_exp_to_transformation_expression(
                &**a,
                isla_sym_to_symbol_definition_mapping,
            ) {
                Some(expression) => Arc::new(expression),
                None => return None,
            };
            let b = match map_isla_exp_to_transformation_expression(
                &**b,
                isla_sym_to_symbol_definition_mapping,
            ) {
                Some(expression) => Arc::new(expression),
                None => return None,
            };
            Some(ScfiaTransformationFunctionExpression::Bvor(a, b))
        }
        Exp::Extract(a, b, exp) => {
            let expression = match map_isla_exp_to_transformation_expression(
                exp,
                isla_sym_to_symbol_definition_mapping,
            ) {
                Some(expression) => expression,
                None => return None,
            };
            Some(ScfiaTransformationFunctionExpression::Extract(
                *a,
                *b,
                Arc::new(expression),
            ))
        }
        Exp::Var(s) => match isla_sym_to_symbol_definition_mapping.get(s) {
            Some(transformation_definition) => Some(ScfiaTransformationFunctionExpression::Var(
                transformation_definition.clone(),
            )),
            None => None,
        },
        Exp::Concat(a, b) => {
            let a = match map_isla_exp_to_transformation_expression(
                &**a,
                isla_sym_to_symbol_definition_mapping,
            ) {
                Some(expression) => Arc::new(expression),
                None => return None,
            };
            let b = match map_isla_exp_to_transformation_expression(
                &**b,
                isla_sym_to_symbol_definition_mapping,
            ) {
                Some(expression) => Arc::new(expression),
                None => return None,
            };
            Some(ScfiaTransformationFunctionExpression::Concat(a, b))
        }
        Exp::Bvnot(a) => {
            let a = match map_isla_exp_to_transformation_expression(
                &**a,
                isla_sym_to_symbol_definition_mapping,
            ) {
                Some(expression) => Arc::new(expression),
                None => return None,
            };
            Some(ScfiaTransformationFunctionExpression::Bvnot(a))
        }
        Exp::Bits64(a, b) => Some(ScfiaTransformationFunctionExpression::Bits64(*a, *b)),
        Exp::Bvand(a, b) => {
            let a = match map_isla_exp_to_transformation_expression(
                &**a,
                isla_sym_to_symbol_definition_mapping,
            ) {
                Some(expression) => Arc::new(expression),
                None => return None,
            };
            let b = match map_isla_exp_to_transformation_expression(
                &**b,
                isla_sym_to_symbol_definition_mapping,
            ) {
                Some(expression) => Arc::new(expression),
                None => return None,
            };
            Some(ScfiaTransformationFunctionExpression::Bvand(a, b))
        }
        Exp::Bvlshr(a, b) => {
            let a = match map_isla_exp_to_transformation_expression(
                &**a,
                isla_sym_to_symbol_definition_mapping,
            ) {
                Some(expression) => Arc::new(expression),
                None => return None,
            };
            let b = match map_isla_exp_to_transformation_expression(
                &**b,
                isla_sym_to_symbol_definition_mapping,
            ) {
                Some(expression) => Arc::new(expression),
                None => return None,
            };
            Some(ScfiaTransformationFunctionExpression::Bvlshr(a, b))
        }
        Exp::Bits(a) => Some(ScfiaTransformationFunctionExpression::Bits(a.clone())),
        Exp::Eq(a, b) => {
            let a = match map_isla_exp_to_transformation_expression(
                &**a,
                isla_sym_to_symbol_definition_mapping,
            ) {
                Some(expression) => Arc::new(expression),
                None => return None,
            };
            let b = match map_isla_exp_to_transformation_expression(
                &**b,
                isla_sym_to_symbol_definition_mapping,
            ) {
                Some(expression) => Arc::new(expression),
                None => return None,
            };
            Some(ScfiaTransformationFunctionExpression::Eq(a, b))
        }
        Exp::Ite(a, b, c) => {
            let a = match map_isla_exp_to_transformation_expression(
                &**a,
                isla_sym_to_symbol_definition_mapping,
            ) {
                Some(expression) => Arc::new(expression),
                None => return None,
            };
            let b = match map_isla_exp_to_transformation_expression(
                &**b,
                isla_sym_to_symbol_definition_mapping,
            ) {
                Some(expression) => Arc::new(expression),
                None => return None,
            };
            let c = match map_isla_exp_to_transformation_expression(
                &**c,
                isla_sym_to_symbol_definition_mapping,
            ) {
                Some(expression) => Arc::new(expression),
                None => return None,
            };
            Some(ScfiaTransformationFunctionExpression::Ite(a, b, c))
        }
        Exp::Enum(enum_member) => Some(ScfiaTransformationFunctionExpression::Enum(
            enum_member.enum_id,
            enum_member.member,
        )),
        Exp::Not(a) => {
            let a = match map_isla_exp_to_transformation_expression(
                &**a,
                isla_sym_to_symbol_definition_mapping,
            ) {
                Some(expression) => Arc::new(expression),
                None => return None,
            };
            Some(ScfiaTransformationFunctionExpression::Not(a))
        }
        Exp::ZeroExtend(a, b) => {
            let b = match map_isla_exp_to_transformation_expression(
                &**b,
                isla_sym_to_symbol_definition_mapping,
            ) {
                Some(expression) => Arc::new(expression),
                None => return None,
            };
            Some(ScfiaTransformationFunctionExpression::ZeroExtend(*a, b))
        }
        Exp::Bvshl(a, b) => {
            let a = match map_isla_exp_to_transformation_expression(
                &**a,
                isla_sym_to_symbol_definition_mapping,
            ) {
                Some(expression) => Arc::new(expression),
                None => return None,
            };
            let b = match map_isla_exp_to_transformation_expression(
                &**b,
                isla_sym_to_symbol_definition_mapping,
            ) {
                Some(expression) => Arc::new(expression),
                None => return None,
            };
            Some(ScfiaTransformationFunctionExpression::Bvshl(a, b))
        }
        Exp::Bool(a) => Some(ScfiaTransformationFunctionExpression::Bool(*a)),
        Exp::Neq(a, b) => {
            let a = match map_isla_exp_to_transformation_expression(
                &**a,
                isla_sym_to_symbol_definition_mapping,
            ) {
                Some(expression) => Arc::new(expression),
                None => return None,
            };
            let b = match map_isla_exp_to_transformation_expression(
                &**b,
                isla_sym_to_symbol_definition_mapping,
            ) {
                Some(expression) => Arc::new(expression),
                None => return None,
            };
            Some(ScfiaTransformationFunctionExpression::Neq(a, b))
        }
        Exp::And(a, b) => {
            let a = match map_isla_exp_to_transformation_expression(
                &**a,
                isla_sym_to_symbol_definition_mapping,
            ) {
                Some(expression) => Arc::new(expression),
                None => return None,
            };
            let b = match map_isla_exp_to_transformation_expression(
                &**b,
                isla_sym_to_symbol_definition_mapping,
            ) {
                Some(expression) => Arc::new(expression),
                None => return None,
            };
            Some(ScfiaTransformationFunctionExpression::And(a, b))
        }
        Exp::Or(a, b) => {
            let a = match map_isla_exp_to_transformation_expression(
                &**a,
                isla_sym_to_symbol_definition_mapping,
            ) {
                Some(expression) => Arc::new(expression),
                None => return None,
            };
            let b = match map_isla_exp_to_transformation_expression(
                &**b,
                isla_sym_to_symbol_definition_mapping,
            ) {
                Some(expression) => Arc::new(expression),
                None => return None,
            };
            Some(ScfiaTransformationFunctionExpression::Or(a, b))
        }
        Exp::Bvxor(a, b) => {
            let a = match map_isla_exp_to_transformation_expression(
                &**a,
                isla_sym_to_symbol_definition_mapping,
            ) {
                Some(expression) => Arc::new(expression),
                None => return None,
            };
            let b = match map_isla_exp_to_transformation_expression(
                &**b,
                isla_sym_to_symbol_definition_mapping,
            ) {
                Some(expression) => Arc::new(expression),
                None => return None,
            };
            Some(ScfiaTransformationFunctionExpression::Bvxor(a, b))
        }
        Exp::Bvnand(a, b) => {
            let a = match map_isla_exp_to_transformation_expression(
                &**a,
                isla_sym_to_symbol_definition_mapping,
            ) {
                Some(expression) => Arc::new(expression),
                None => return None,
            };
            let b = match map_isla_exp_to_transformation_expression(
                &**b,
                isla_sym_to_symbol_definition_mapping,
            ) {
                Some(expression) => Arc::new(expression),
                None => return None,
            };
            Some(ScfiaTransformationFunctionExpression::Bvnand(a, b))
        }
        Exp::Bvnor(a, b) => {
            let a = match map_isla_exp_to_transformation_expression(
                &**a,
                isla_sym_to_symbol_definition_mapping,
            ) {
                Some(expression) => Arc::new(expression),
                None => return None,
            };
            let b = match map_isla_exp_to_transformation_expression(
                &**b,
                isla_sym_to_symbol_definition_mapping,
            ) {
                Some(expression) => Arc::new(expression),
                None => return None,
            };
            Some(ScfiaTransformationFunctionExpression::Bvnor(a, b))
        }
        Exp::Bvxnor(a, b) => {
            let a = match map_isla_exp_to_transformation_expression(
                &**a,
                isla_sym_to_symbol_definition_mapping,
            ) {
                Some(expression) => Arc::new(expression),
                None => return None,
            };
            let b = match map_isla_exp_to_transformation_expression(
                &**b,
                isla_sym_to_symbol_definition_mapping,
            ) {
                Some(expression) => Arc::new(expression),
                None => return None,
            };
            Some(ScfiaTransformationFunctionExpression::Bvxnor(a, b))
        }
        Exp::Bvneg(a) => {
            let a = match map_isla_exp_to_transformation_expression(
                &**a,
                isla_sym_to_symbol_definition_mapping,
            ) {
                Some(expression) => Arc::new(expression),
                None => return None,
            };
            Some(ScfiaTransformationFunctionExpression::Bvneg(a))
        }
        Exp::Bvadd(a, b) => {
            let a = match map_isla_exp_to_transformation_expression(
                &**a,
                isla_sym_to_symbol_definition_mapping,
            ) {
                Some(expression) => Arc::new(expression),
                None => return None,
            };
            let b = match map_isla_exp_to_transformation_expression(
                &**b,
                isla_sym_to_symbol_definition_mapping,
            ) {
                Some(expression) => Arc::new(expression),
                None => return None,
            };
            Some(ScfiaTransformationFunctionExpression::Bvadd(a, b))
        }
        Exp::Bvsub(a, b) => {
            let a = match map_isla_exp_to_transformation_expression(
                &**a,
                isla_sym_to_symbol_definition_mapping,
            ) {
                Some(expression) => Arc::new(expression),
                None => return None,
            };
            let b = match map_isla_exp_to_transformation_expression(
                &**b,
                isla_sym_to_symbol_definition_mapping,
            ) {
                Some(expression) => Arc::new(expression),
                None => return None,
            };
            Some(ScfiaTransformationFunctionExpression::Bvsub(a, b))
        }
        Exp::Bvmul(a, b) => {
            let a = match map_isla_exp_to_transformation_expression(
                &**a,
                isla_sym_to_symbol_definition_mapping,
            ) {
                Some(expression) => Arc::new(expression),
                None => return None,
            };
            let b = match map_isla_exp_to_transformation_expression(
                &**b,
                isla_sym_to_symbol_definition_mapping,
            ) {
                Some(expression) => Arc::new(expression),
                None => return None,
            };
            Some(ScfiaTransformationFunctionExpression::Bvmul(a, b))
        }
        Exp::Bvudiv(a, b) => {
            let a = match map_isla_exp_to_transformation_expression(
                &**a,
                isla_sym_to_symbol_definition_mapping,
            ) {
                Some(expression) => Arc::new(expression),
                None => return None,
            };
            let b = match map_isla_exp_to_transformation_expression(
                &**b,
                isla_sym_to_symbol_definition_mapping,
            ) {
                Some(expression) => Arc::new(expression),
                None => return None,
            };
            Some(ScfiaTransformationFunctionExpression::Bvudiv(a, b))
        }
        Exp::Bvsdiv(a, b) => {
            let a = match map_isla_exp_to_transformation_expression(
                &**a,
                isla_sym_to_symbol_definition_mapping,
            ) {
                Some(expression) => Arc::new(expression),
                None => return None,
            };
            let b = match map_isla_exp_to_transformation_expression(
                &**b,
                isla_sym_to_symbol_definition_mapping,
            ) {
                Some(expression) => Arc::new(expression),
                None => return None,
            };
            Some(ScfiaTransformationFunctionExpression::Bvsdiv(a, b))
        }
        Exp::Bvurem(a, b) => {
            let a = match map_isla_exp_to_transformation_expression(
                &**a,
                isla_sym_to_symbol_definition_mapping,
            ) {
                Some(expression) => Arc::new(expression),
                None => return None,
            };
            let b = match map_isla_exp_to_transformation_expression(
                &**b,
                isla_sym_to_symbol_definition_mapping,
            ) {
                Some(expression) => Arc::new(expression),
                None => return None,
            };
            Some(ScfiaTransformationFunctionExpression::Bvurem(a, b))
        }
        Exp::Bvsrem(a, b) => {
            let a = match map_isla_exp_to_transformation_expression(
                &**a,
                isla_sym_to_symbol_definition_mapping,
            ) {
                Some(expression) => Arc::new(expression),
                None => return None,
            };
            let b = match map_isla_exp_to_transformation_expression(
                &**b,
                isla_sym_to_symbol_definition_mapping,
            ) {
                Some(expression) => Arc::new(expression),
                None => return None,
            };
            Some(ScfiaTransformationFunctionExpression::Bvsrem(a, b))
        }
        Exp::Bvsmod(a, b) => {
            let a = match map_isla_exp_to_transformation_expression(
                &**a,
                isla_sym_to_symbol_definition_mapping,
            ) {
                Some(expression) => Arc::new(expression),
                None => return None,
            };
            let b = match map_isla_exp_to_transformation_expression(
                &**b,
                isla_sym_to_symbol_definition_mapping,
            ) {
                Some(expression) => Arc::new(expression),
                None => return None,
            };
            Some(ScfiaTransformationFunctionExpression::Bvsmod(a, b))
        }
        Exp::Bvult(a, b) => {
            let a = match map_isla_exp_to_transformation_expression(
                &**a,
                isla_sym_to_symbol_definition_mapping,
            ) {
                Some(expression) => Arc::new(expression),
                None => return None,
            };
            let b = match map_isla_exp_to_transformation_expression(
                &**b,
                isla_sym_to_symbol_definition_mapping,
            ) {
                Some(expression) => Arc::new(expression),
                None => return None,
            };
            Some(ScfiaTransformationFunctionExpression::Bvult(a, b))
        }
        Exp::Bvslt(a, b) => {
            let a = match map_isla_exp_to_transformation_expression(
                &**a,
                isla_sym_to_symbol_definition_mapping,
            ) {
                Some(expression) => Arc::new(expression),
                None => return None,
            };
            let b = match map_isla_exp_to_transformation_expression(
                &**b,
                isla_sym_to_symbol_definition_mapping,
            ) {
                Some(expression) => Arc::new(expression),
                None => return None,
            };
            Some(ScfiaTransformationFunctionExpression::Bvslt(a, b))
        }
        Exp::Bvule(a, b) => {
            let a = match map_isla_exp_to_transformation_expression(
                &**a,
                isla_sym_to_symbol_definition_mapping,
            ) {
                Some(expression) => Arc::new(expression),
                None => return None,
            };
            let b = match map_isla_exp_to_transformation_expression(
                &**b,
                isla_sym_to_symbol_definition_mapping,
            ) {
                Some(expression) => Arc::new(expression),
                None => return None,
            };
            Some(ScfiaTransformationFunctionExpression::Bvule(a, b))
        }
        Exp::Bvsle(a, b) => {
            let a = match map_isla_exp_to_transformation_expression(
                &**a,
                isla_sym_to_symbol_definition_mapping,
            ) {
                Some(expression) => Arc::new(expression),
                None => return None,
            };
            let b = match map_isla_exp_to_transformation_expression(
                &**b,
                isla_sym_to_symbol_definition_mapping,
            ) {
                Some(expression) => Arc::new(expression),
                None => return None,
            };
            Some(ScfiaTransformationFunctionExpression::Bvsle(a, b))
        }
        Exp::Bvuge(a, b) => {
            let a = match map_isla_exp_to_transformation_expression(
                &**a,
                isla_sym_to_symbol_definition_mapping,
            ) {
                Some(expression) => Arc::new(expression),
                None => return None,
            };
            let b = match map_isla_exp_to_transformation_expression(
                &**b,
                isla_sym_to_symbol_definition_mapping,
            ) {
                Some(expression) => Arc::new(expression),
                None => return None,
            };
            Some(ScfiaTransformationFunctionExpression::Bvuge(a, b))
        }
        Exp::Bvsge(a, b) => {
            let a = match map_isla_exp_to_transformation_expression(
                &**a,
                isla_sym_to_symbol_definition_mapping,
            ) {
                Some(expression) => Arc::new(expression),
                None => return None,
            };
            let b = match map_isla_exp_to_transformation_expression(
                &**b,
                isla_sym_to_symbol_definition_mapping,
            ) {
                Some(expression) => Arc::new(expression),
                None => return None,
            };
            Some(ScfiaTransformationFunctionExpression::Bvsge(a, b))
        }
        Exp::Bvugt(a, b) => {
            let a = match map_isla_exp_to_transformation_expression(
                &**a,
                isla_sym_to_symbol_definition_mapping,
            ) {
                Some(expression) => Arc::new(expression),
                None => return None,
            };
            let b = match map_isla_exp_to_transformation_expression(
                &**b,
                isla_sym_to_symbol_definition_mapping,
            ) {
                Some(expression) => Arc::new(expression),
                None => return None,
            };
            Some(ScfiaTransformationFunctionExpression::Bvugt(a, b))
        }
        Exp::Bvsgt(a, b) => {
            let a = match map_isla_exp_to_transformation_expression(
                &**a,
                isla_sym_to_symbol_definition_mapping,
            ) {
                Some(expression) => Arc::new(expression),
                None => return None,
            };
            let b = match map_isla_exp_to_transformation_expression(
                &**b,
                isla_sym_to_symbol_definition_mapping,
            ) {
                Some(expression) => Arc::new(expression),
                None => return None,
            };
            Some(ScfiaTransformationFunctionExpression::Bvsgt(a, b))
        }
        Exp::SignExtend(a, b) => {
            let b = match map_isla_exp_to_transformation_expression(
                &**b,
                isla_sym_to_symbol_definition_mapping,
            ) {
                Some(expression) => Arc::new(expression),
                None => return None,
            };
            Some(ScfiaTransformationFunctionExpression::SignExtend(*a, b))
        }
        Exp::Bvashr(a, b) => {
            let a = match map_isla_exp_to_transformation_expression(
                &**a,
                isla_sym_to_symbol_definition_mapping,
            ) {
                Some(expression) => Arc::new(expression),
                None => return None,
            };
            let b = match map_isla_exp_to_transformation_expression(
                &**b,
                isla_sym_to_symbol_definition_mapping,
            ) {
                Some(expression) => Arc::new(expression),
                None => return None,
            };
            Some(ScfiaTransformationFunctionExpression::Bvashr(a, b))
        }
        Exp::App(a, b) => {
            unimplemented!("{:?} {:?}", a, b)
        }
        Exp::Select(a, b) => {
            let a = match map_isla_exp_to_transformation_expression(
                &**a,
                isla_sym_to_symbol_definition_mapping,
            ) {
                Some(expression) => Arc::new(expression),
                None => return None,
            };
            let b = match map_isla_exp_to_transformation_expression(
                &**b,
                isla_sym_to_symbol_definition_mapping,
            ) {
                Some(expression) => Arc::new(expression),
                None => return None,
            };
            Some(ScfiaTransformationFunctionExpression::Select(a, b))
        }
        Exp::Store(a, b, c) => {
            let a = match map_isla_exp_to_transformation_expression(
                &**a,
                isla_sym_to_symbol_definition_mapping,
            ) {
                Some(expression) => Arc::new(expression),
                None => return None,
            };
            let b = match map_isla_exp_to_transformation_expression(
                &**b,
                isla_sym_to_symbol_definition_mapping,
            ) {
                Some(expression) => Arc::new(expression),
                None => return None,
            };
            let c = match map_isla_exp_to_transformation_expression(
                &**c,
                isla_sym_to_symbol_definition_mapping,
            ) {
                Some(expression) => Arc::new(expression),
                None => return None,
            };
            Some(ScfiaTransformationFunctionExpression::Store(a, b, c))
        }
    }
}
