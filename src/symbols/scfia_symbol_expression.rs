use std::{collections::HashMap, sync::Arc};

use isla_lib::{
    concrete::bitvector64::B64,
    ir::EnumMember,
    smt::{smtlib::Exp, Solver, Sym},
};

use super::{
    scfia_symbol::{ScfiaSymbol, ScfiaSymbolId},
    scfia_symbol_manager::ScfiaSymbolManager,
};

#[derive(Clone, Debug)]
pub enum ScfiaSymbolExpression {
    Var(ScfiaSymbolId),
    Bits(Vec<bool>),
    Bits64(u64, u32),
    Enum(usize, usize), // enum_id, member
    Bool(bool),
    Eq(Arc<ScfiaSymbolExpression>, Arc<ScfiaSymbolExpression>),
    Neq(Arc<ScfiaSymbolExpression>, Arc<ScfiaSymbolExpression>),
    And(Arc<ScfiaSymbolExpression>, Arc<ScfiaSymbolExpression>),
    Or(Arc<ScfiaSymbolExpression>, Arc<ScfiaSymbolExpression>),
    Not(Arc<ScfiaSymbolExpression>),
    Bvnot(Arc<ScfiaSymbolExpression>),
    Bvand(Arc<ScfiaSymbolExpression>, Arc<ScfiaSymbolExpression>),
    Bvor(Arc<ScfiaSymbolExpression>, Arc<ScfiaSymbolExpression>),
    Bvxor(Arc<ScfiaSymbolExpression>, Arc<ScfiaSymbolExpression>),
    Bvnand(Arc<ScfiaSymbolExpression>, Arc<ScfiaSymbolExpression>),
    Bvnor(Arc<ScfiaSymbolExpression>, Arc<ScfiaSymbolExpression>),
    Bvxnor(Arc<ScfiaSymbolExpression>, Arc<ScfiaSymbolExpression>),
    Bvneg(Arc<ScfiaSymbolExpression>),
    Bvadd(Arc<ScfiaSymbolExpression>, Arc<ScfiaSymbolExpression>),
    Bvsub(Arc<ScfiaSymbolExpression>, Arc<ScfiaSymbolExpression>),
    Bvmul(Arc<ScfiaSymbolExpression>, Arc<ScfiaSymbolExpression>),
    Bvudiv(Arc<ScfiaSymbolExpression>, Arc<ScfiaSymbolExpression>),
    Bvsdiv(Arc<ScfiaSymbolExpression>, Arc<ScfiaSymbolExpression>),
    Bvurem(Arc<ScfiaSymbolExpression>, Arc<ScfiaSymbolExpression>),
    Bvsrem(Arc<ScfiaSymbolExpression>, Arc<ScfiaSymbolExpression>),
    Bvsmod(Arc<ScfiaSymbolExpression>, Arc<ScfiaSymbolExpression>),
    Bvult(Arc<ScfiaSymbolExpression>, Arc<ScfiaSymbolExpression>),
    Bvslt(Arc<ScfiaSymbolExpression>, Arc<ScfiaSymbolExpression>),
    Bvule(Arc<ScfiaSymbolExpression>, Arc<ScfiaSymbolExpression>),
    Bvsle(Arc<ScfiaSymbolExpression>, Arc<ScfiaSymbolExpression>),
    Bvuge(Arc<ScfiaSymbolExpression>, Arc<ScfiaSymbolExpression>),
    Bvsge(Arc<ScfiaSymbolExpression>, Arc<ScfiaSymbolExpression>),
    Bvugt(Arc<ScfiaSymbolExpression>, Arc<ScfiaSymbolExpression>),
    Bvsgt(Arc<ScfiaSymbolExpression>, Arc<ScfiaSymbolExpression>),
    Extract(u32, u32, Arc<ScfiaSymbolExpression>),
    ZeroExtend(u32, Arc<ScfiaSymbolExpression>),
    SignExtend(u32, Arc<ScfiaSymbolExpression>),
    Bvshl(Arc<ScfiaSymbolExpression>, Arc<ScfiaSymbolExpression>),
    Bvlshr(Arc<ScfiaSymbolExpression>, Arc<ScfiaSymbolExpression>),
    Bvashr(Arc<ScfiaSymbolExpression>, Arc<ScfiaSymbolExpression>),
    Concat(Arc<ScfiaSymbolExpression>, Arc<ScfiaSymbolExpression>),
    Ite(
        Arc<ScfiaSymbolExpression>,
        Arc<ScfiaSymbolExpression>,
        Arc<ScfiaSymbolExpression>,
    ),
    Select(Arc<ScfiaSymbolExpression>, Arc<ScfiaSymbolExpression>),
    Store(
        Arc<ScfiaSymbolExpression>,
        Arc<ScfiaSymbolExpression>,
        Arc<ScfiaSymbolExpression>,
    ),
}

impl ScfiaSymbolExpression {
    pub fn to_isla_exp(
        &self,
        solver: &mut Solver<B64>,
        symbol_manager: &ScfiaSymbolManager,
        scfia_symbol_to_isla_sym_mapping: &mut HashMap<ScfiaSymbolId, Sym>,
    ) -> Exp {
        match self {
            ScfiaSymbolExpression::Var(symbol_id) => Exp::Var(symbol_manager.to_isla_sym(
                symbol_id,
                solver,
                scfia_symbol_to_isla_sym_mapping,
            )),
            ScfiaSymbolExpression::Bits(b) => Exp::Bits(b.clone()),
            ScfiaSymbolExpression::Bits64(value, width) => Exp::Bits64(*value, *width),
            ScfiaSymbolExpression::Enum(enum_id, enum_member) => Exp::Enum(EnumMember {
                enum_id: *enum_id,
                member: *enum_member,
            }),
            ScfiaSymbolExpression::Bool(b) => Exp::Bool(*b),
            ScfiaSymbolExpression::Eq(a, b) => Exp::Eq(
                Box::new(a.to_isla_exp(solver, symbol_manager, scfia_symbol_to_isla_sym_mapping)),
                Box::new(b.to_isla_exp(solver, symbol_manager, scfia_symbol_to_isla_sym_mapping)),
            ),
            ScfiaSymbolExpression::Neq(a, b) => Exp::Neq(
                Box::new(a.to_isla_exp(solver, symbol_manager, scfia_symbol_to_isla_sym_mapping)),
                Box::new(b.to_isla_exp(solver, symbol_manager, scfia_symbol_to_isla_sym_mapping)),
            ),
            ScfiaSymbolExpression::And(a, b) => Exp::And(
                Box::new(a.to_isla_exp(solver, symbol_manager, scfia_symbol_to_isla_sym_mapping)),
                Box::new(b.to_isla_exp(solver, symbol_manager, scfia_symbol_to_isla_sym_mapping)),
            ),
            ScfiaSymbolExpression::Or(a, b) => Exp::Or(
                Box::new(a.to_isla_exp(solver, symbol_manager, scfia_symbol_to_isla_sym_mapping)),
                Box::new(b.to_isla_exp(solver, symbol_manager, scfia_symbol_to_isla_sym_mapping)),
            ),
            ScfiaSymbolExpression::Not(a) => Exp::Not(Box::new(a.to_isla_exp(
                solver,
                symbol_manager,
                scfia_symbol_to_isla_sym_mapping,
            ))),
            ScfiaSymbolExpression::Bvnot(a) => Exp::Bvnot(Box::new(a.to_isla_exp(
                solver,
                symbol_manager,
                scfia_symbol_to_isla_sym_mapping,
            ))),
            ScfiaSymbolExpression::Bvand(a, b) => Exp::Bvand(
                Box::new(a.to_isla_exp(solver, symbol_manager, scfia_symbol_to_isla_sym_mapping)),
                Box::new(b.to_isla_exp(solver, symbol_manager, scfia_symbol_to_isla_sym_mapping)),
            ),
            ScfiaSymbolExpression::Bvor(a, b) => Exp::Bvor(
                Box::new(a.to_isla_exp(solver, symbol_manager, scfia_symbol_to_isla_sym_mapping)),
                Box::new(b.to_isla_exp(solver, symbol_manager, scfia_symbol_to_isla_sym_mapping)),
            ),
            ScfiaSymbolExpression::Bvxor(a, b) => Exp::Bvxor(
                Box::new(a.to_isla_exp(solver, symbol_manager, scfia_symbol_to_isla_sym_mapping)),
                Box::new(b.to_isla_exp(solver, symbol_manager, scfia_symbol_to_isla_sym_mapping)),
            ),
            ScfiaSymbolExpression::Bvnand(a, b) => Exp::Bvnand(
                Box::new(a.to_isla_exp(solver, symbol_manager, scfia_symbol_to_isla_sym_mapping)),
                Box::new(b.to_isla_exp(solver, symbol_manager, scfia_symbol_to_isla_sym_mapping)),
            ),
            ScfiaSymbolExpression::Bvnor(a, b) => Exp::Bvnor(
                Box::new(a.to_isla_exp(solver, symbol_manager, scfia_symbol_to_isla_sym_mapping)),
                Box::new(b.to_isla_exp(solver, symbol_manager, scfia_symbol_to_isla_sym_mapping)),
            ),
            ScfiaSymbolExpression::Bvxnor(a, b) => Exp::Bvxnor(
                Box::new(a.to_isla_exp(solver, symbol_manager, scfia_symbol_to_isla_sym_mapping)),
                Box::new(b.to_isla_exp(solver, symbol_manager, scfia_symbol_to_isla_sym_mapping)),
            ),
            ScfiaSymbolExpression::Bvneg(a) => Exp::Bvneg(Box::new(a.to_isla_exp(
                solver,
                symbol_manager,
                scfia_symbol_to_isla_sym_mapping,
            ))),
            ScfiaSymbolExpression::Bvadd(a, b) => Exp::Bvadd(
                Box::new(a.to_isla_exp(solver, symbol_manager, scfia_symbol_to_isla_sym_mapping)),
                Box::new(b.to_isla_exp(solver, symbol_manager, scfia_symbol_to_isla_sym_mapping)),
            ),
            ScfiaSymbolExpression::Bvsub(a, b) => Exp::Bvsub(
                Box::new(a.to_isla_exp(solver, symbol_manager, scfia_symbol_to_isla_sym_mapping)),
                Box::new(b.to_isla_exp(solver, symbol_manager, scfia_symbol_to_isla_sym_mapping)),
            ),
            ScfiaSymbolExpression::Bvmul(a, b) => Exp::Bvmul(
                Box::new(a.to_isla_exp(solver, symbol_manager, scfia_symbol_to_isla_sym_mapping)),
                Box::new(b.to_isla_exp(solver, symbol_manager, scfia_symbol_to_isla_sym_mapping)),
            ),
            ScfiaSymbolExpression::Bvudiv(a, b) => Exp::Bvudiv(
                Box::new(a.to_isla_exp(solver, symbol_manager, scfia_symbol_to_isla_sym_mapping)),
                Box::new(b.to_isla_exp(solver, symbol_manager, scfia_symbol_to_isla_sym_mapping)),
            ),
            ScfiaSymbolExpression::Bvsdiv(a, b) => Exp::Bvsdiv(
                Box::new(a.to_isla_exp(solver, symbol_manager, scfia_symbol_to_isla_sym_mapping)),
                Box::new(b.to_isla_exp(solver, symbol_manager, scfia_symbol_to_isla_sym_mapping)),
            ),
            ScfiaSymbolExpression::Bvurem(a, b) => Exp::Bvurem(
                Box::new(a.to_isla_exp(solver, symbol_manager, scfia_symbol_to_isla_sym_mapping)),
                Box::new(b.to_isla_exp(solver, symbol_manager, scfia_symbol_to_isla_sym_mapping)),
            ),
            ScfiaSymbolExpression::Bvsrem(a, b) => Exp::Bvsrem(
                Box::new(a.to_isla_exp(solver, symbol_manager, scfia_symbol_to_isla_sym_mapping)),
                Box::new(b.to_isla_exp(solver, symbol_manager, scfia_symbol_to_isla_sym_mapping)),
            ),
            ScfiaSymbolExpression::Bvsmod(a, b) => Exp::Bvsmod(
                Box::new(a.to_isla_exp(solver, symbol_manager, scfia_symbol_to_isla_sym_mapping)),
                Box::new(b.to_isla_exp(solver, symbol_manager, scfia_symbol_to_isla_sym_mapping)),
            ),
            ScfiaSymbolExpression::Bvult(a, b) => Exp::Bvult(
                Box::new(a.to_isla_exp(solver, symbol_manager, scfia_symbol_to_isla_sym_mapping)),
                Box::new(b.to_isla_exp(solver, symbol_manager, scfia_symbol_to_isla_sym_mapping)),
            ),
            ScfiaSymbolExpression::Bvslt(a, b) => Exp::Bvslt(
                Box::new(a.to_isla_exp(solver, symbol_manager, scfia_symbol_to_isla_sym_mapping)),
                Box::new(b.to_isla_exp(solver, symbol_manager, scfia_symbol_to_isla_sym_mapping)),
            ),
            ScfiaSymbolExpression::Bvule(a, b) => Exp::Bvule(
                Box::new(a.to_isla_exp(solver, symbol_manager, scfia_symbol_to_isla_sym_mapping)),
                Box::new(b.to_isla_exp(solver, symbol_manager, scfia_symbol_to_isla_sym_mapping)),
            ),
            ScfiaSymbolExpression::Bvsle(a, b) => Exp::Bvsle(
                Box::new(a.to_isla_exp(solver, symbol_manager, scfia_symbol_to_isla_sym_mapping)),
                Box::new(b.to_isla_exp(solver, symbol_manager, scfia_symbol_to_isla_sym_mapping)),
            ),
            ScfiaSymbolExpression::Bvuge(a, b) => Exp::Bvuge(
                Box::new(a.to_isla_exp(solver, symbol_manager, scfia_symbol_to_isla_sym_mapping)),
                Box::new(b.to_isla_exp(solver, symbol_manager, scfia_symbol_to_isla_sym_mapping)),
            ),
            ScfiaSymbolExpression::Bvsge(a, b) => Exp::Bvsge(
                Box::new(a.to_isla_exp(solver, symbol_manager, scfia_symbol_to_isla_sym_mapping)),
                Box::new(b.to_isla_exp(solver, symbol_manager, scfia_symbol_to_isla_sym_mapping)),
            ),
            ScfiaSymbolExpression::Bvugt(a, b) => Exp::Bvugt(
                Box::new(a.to_isla_exp(solver, symbol_manager, scfia_symbol_to_isla_sym_mapping)),
                Box::new(b.to_isla_exp(solver, symbol_manager, scfia_symbol_to_isla_sym_mapping)),
            ),
            ScfiaSymbolExpression::Bvsgt(a, b) => Exp::Bvsgt(
                Box::new(a.to_isla_exp(solver, symbol_manager, scfia_symbol_to_isla_sym_mapping)),
                Box::new(b.to_isla_exp(solver, symbol_manager, scfia_symbol_to_isla_sym_mapping)),
            ),
            ScfiaSymbolExpression::Extract(a, b, c) => Exp::Extract(
                *a,
                *b,
                Box::new(c.to_isla_exp(solver, symbol_manager, scfia_symbol_to_isla_sym_mapping)),
            ),
            ScfiaSymbolExpression::ZeroExtend(a, b) => Exp::ZeroExtend(
                *a,
                Box::new(b.to_isla_exp(solver, symbol_manager, scfia_symbol_to_isla_sym_mapping)),
            ),
            ScfiaSymbolExpression::SignExtend(a, b) => Exp::SignExtend(
                *a,
                Box::new(b.to_isla_exp(solver, symbol_manager, scfia_symbol_to_isla_sym_mapping)),
            ),
            ScfiaSymbolExpression::Bvshl(a, b) => Exp::Bvshl(
                Box::new(a.to_isla_exp(solver, symbol_manager, scfia_symbol_to_isla_sym_mapping)),
                Box::new(b.to_isla_exp(solver, symbol_manager, scfia_symbol_to_isla_sym_mapping)),
            ),
            ScfiaSymbolExpression::Bvlshr(a, b) => Exp::Bvlshr(
                Box::new(a.to_isla_exp(solver, symbol_manager, scfia_symbol_to_isla_sym_mapping)),
                Box::new(b.to_isla_exp(solver, symbol_manager, scfia_symbol_to_isla_sym_mapping)),
            ),
            ScfiaSymbolExpression::Bvashr(a, b) => Exp::Bvashr(
                Box::new(a.to_isla_exp(solver, symbol_manager, scfia_symbol_to_isla_sym_mapping)),
                Box::new(b.to_isla_exp(solver, symbol_manager, scfia_symbol_to_isla_sym_mapping)),
            ),
            ScfiaSymbolExpression::Concat(a, b) => Exp::Concat(
                Box::new(a.to_isla_exp(solver, symbol_manager, scfia_symbol_to_isla_sym_mapping)),
                Box::new(b.to_isla_exp(solver, symbol_manager, scfia_symbol_to_isla_sym_mapping)),
            ),
            ScfiaSymbolExpression::Ite(a, b, c) => Exp::Ite(
                Box::new(a.to_isla_exp(solver, symbol_manager, scfia_symbol_to_isla_sym_mapping)),
                Box::new(b.to_isla_exp(solver, symbol_manager, scfia_symbol_to_isla_sym_mapping)),
                Box::new(c.to_isla_exp(solver, symbol_manager, scfia_symbol_to_isla_sym_mapping)),
            ),
            ScfiaSymbolExpression::Select(a, b) => Exp::Select(
                Box::new(a.to_isla_exp(solver, symbol_manager, scfia_symbol_to_isla_sym_mapping)),
                Box::new(b.to_isla_exp(solver, symbol_manager, scfia_symbol_to_isla_sym_mapping)),
            ),
            ScfiaSymbolExpression::Store(a, b, c) => Exp::Store(
                Box::new(a.to_isla_exp(solver, symbol_manager, scfia_symbol_to_isla_sym_mapping)),
                Box::new(b.to_isla_exp(solver, symbol_manager, scfia_symbol_to_isla_sym_mapping)),
                Box::new(c.to_isla_exp(solver, symbol_manager, scfia_symbol_to_isla_sym_mapping)),
            ),
        }
    }

    pub fn to_isla_exp_from_existing_vars(
        &self,
        solver: &mut Solver<B64>,
        scfia_symbol_to_isla_sym_mapping: &mut HashMap<ScfiaSymbolId, Sym>,
    ) -> Exp {
        match self {
            ScfiaSymbolExpression::Var(symbol_id) => {
                Exp::Var(*scfia_symbol_to_isla_sym_mapping.get(symbol_id).unwrap())
            }
            ScfiaSymbolExpression::Bits(b) => Exp::Bits(b.clone()),
            ScfiaSymbolExpression::Bits64(value, width) => Exp::Bits64(*value, *width),
            ScfiaSymbolExpression::Enum(enum_id, enum_member) => Exp::Enum(EnumMember {
                enum_id: *enum_id,
                member: *enum_member,
            }),
            ScfiaSymbolExpression::Bool(b) => Exp::Bool(*b),
            ScfiaSymbolExpression::Eq(a, b) => Exp::Eq(
                Box::new(
                    a.to_isla_exp_from_existing_vars(solver, scfia_symbol_to_isla_sym_mapping),
                ),
                Box::new(
                    b.to_isla_exp_from_existing_vars(solver, scfia_symbol_to_isla_sym_mapping),
                ),
            ),
            ScfiaSymbolExpression::Neq(a, b) => Exp::Neq(
                Box::new(
                    a.to_isla_exp_from_existing_vars(solver, scfia_symbol_to_isla_sym_mapping),
                ),
                Box::new(
                    b.to_isla_exp_from_existing_vars(solver, scfia_symbol_to_isla_sym_mapping),
                ),
            ),
            ScfiaSymbolExpression::And(a, b) => Exp::And(
                Box::new(
                    a.to_isla_exp_from_existing_vars(solver, scfia_symbol_to_isla_sym_mapping),
                ),
                Box::new(
                    b.to_isla_exp_from_existing_vars(solver, scfia_symbol_to_isla_sym_mapping),
                ),
            ),
            ScfiaSymbolExpression::Or(a, b) => Exp::Or(
                Box::new(
                    a.to_isla_exp_from_existing_vars(solver, scfia_symbol_to_isla_sym_mapping),
                ),
                Box::new(
                    b.to_isla_exp_from_existing_vars(solver, scfia_symbol_to_isla_sym_mapping),
                ),
            ),
            ScfiaSymbolExpression::Not(a) => Exp::Not(Box::new(
                a.to_isla_exp_from_existing_vars(solver, scfia_symbol_to_isla_sym_mapping),
            )),
            ScfiaSymbolExpression::Bvnot(a) => Exp::Bvnot(Box::new(
                a.to_isla_exp_from_existing_vars(solver, scfia_symbol_to_isla_sym_mapping),
            )),
            ScfiaSymbolExpression::Bvand(a, b) => Exp::Bvand(
                Box::new(
                    a.to_isla_exp_from_existing_vars(solver, scfia_symbol_to_isla_sym_mapping),
                ),
                Box::new(
                    b.to_isla_exp_from_existing_vars(solver, scfia_symbol_to_isla_sym_mapping),
                ),
            ),
            ScfiaSymbolExpression::Bvor(a, b) => Exp::Bvor(
                Box::new(
                    a.to_isla_exp_from_existing_vars(solver, scfia_symbol_to_isla_sym_mapping),
                ),
                Box::new(
                    b.to_isla_exp_from_existing_vars(solver, scfia_symbol_to_isla_sym_mapping),
                ),
            ),
            ScfiaSymbolExpression::Bvxor(a, b) => Exp::Bvxor(
                Box::new(
                    a.to_isla_exp_from_existing_vars(solver, scfia_symbol_to_isla_sym_mapping),
                ),
                Box::new(
                    b.to_isla_exp_from_existing_vars(solver, scfia_symbol_to_isla_sym_mapping),
                ),
            ),
            ScfiaSymbolExpression::Bvnand(a, b) => Exp::Bvnand(
                Box::new(
                    a.to_isla_exp_from_existing_vars(solver, scfia_symbol_to_isla_sym_mapping),
                ),
                Box::new(
                    b.to_isla_exp_from_existing_vars(solver, scfia_symbol_to_isla_sym_mapping),
                ),
            ),
            ScfiaSymbolExpression::Bvnor(a, b) => Exp::Bvnor(
                Box::new(
                    a.to_isla_exp_from_existing_vars(solver, scfia_symbol_to_isla_sym_mapping),
                ),
                Box::new(
                    b.to_isla_exp_from_existing_vars(solver, scfia_symbol_to_isla_sym_mapping),
                ),
            ),
            ScfiaSymbolExpression::Bvxnor(a, b) => Exp::Bvxnor(
                Box::new(
                    a.to_isla_exp_from_existing_vars(solver, scfia_symbol_to_isla_sym_mapping),
                ),
                Box::new(
                    b.to_isla_exp_from_existing_vars(solver, scfia_symbol_to_isla_sym_mapping),
                ),
            ),
            ScfiaSymbolExpression::Bvneg(a) => Exp::Bvneg(Box::new(
                a.to_isla_exp_from_existing_vars(solver, scfia_symbol_to_isla_sym_mapping),
            )),
            ScfiaSymbolExpression::Bvadd(a, b) => Exp::Bvadd(
                Box::new(
                    a.to_isla_exp_from_existing_vars(solver, scfia_symbol_to_isla_sym_mapping),
                ),
                Box::new(
                    b.to_isla_exp_from_existing_vars(solver, scfia_symbol_to_isla_sym_mapping),
                ),
            ),
            ScfiaSymbolExpression::Bvsub(a, b) => Exp::Bvsub(
                Box::new(
                    a.to_isla_exp_from_existing_vars(solver, scfia_symbol_to_isla_sym_mapping),
                ),
                Box::new(
                    b.to_isla_exp_from_existing_vars(solver, scfia_symbol_to_isla_sym_mapping),
                ),
            ),
            ScfiaSymbolExpression::Bvmul(a, b) => Exp::Bvmul(
                Box::new(
                    a.to_isla_exp_from_existing_vars(solver, scfia_symbol_to_isla_sym_mapping),
                ),
                Box::new(
                    b.to_isla_exp_from_existing_vars(solver, scfia_symbol_to_isla_sym_mapping),
                ),
            ),
            ScfiaSymbolExpression::Bvudiv(a, b) => Exp::Bvudiv(
                Box::new(
                    a.to_isla_exp_from_existing_vars(solver, scfia_symbol_to_isla_sym_mapping),
                ),
                Box::new(
                    b.to_isla_exp_from_existing_vars(solver, scfia_symbol_to_isla_sym_mapping),
                ),
            ),
            ScfiaSymbolExpression::Bvsdiv(a, b) => Exp::Bvsdiv(
                Box::new(
                    a.to_isla_exp_from_existing_vars(solver, scfia_symbol_to_isla_sym_mapping),
                ),
                Box::new(
                    b.to_isla_exp_from_existing_vars(solver, scfia_symbol_to_isla_sym_mapping),
                ),
            ),
            ScfiaSymbolExpression::Bvurem(a, b) => Exp::Bvurem(
                Box::new(
                    a.to_isla_exp_from_existing_vars(solver, scfia_symbol_to_isla_sym_mapping),
                ),
                Box::new(
                    b.to_isla_exp_from_existing_vars(solver, scfia_symbol_to_isla_sym_mapping),
                ),
            ),
            ScfiaSymbolExpression::Bvsrem(a, b) => Exp::Bvsrem(
                Box::new(
                    a.to_isla_exp_from_existing_vars(solver, scfia_symbol_to_isla_sym_mapping),
                ),
                Box::new(
                    b.to_isla_exp_from_existing_vars(solver, scfia_symbol_to_isla_sym_mapping),
                ),
            ),
            ScfiaSymbolExpression::Bvsmod(a, b) => Exp::Bvsmod(
                Box::new(
                    a.to_isla_exp_from_existing_vars(solver, scfia_symbol_to_isla_sym_mapping),
                ),
                Box::new(
                    b.to_isla_exp_from_existing_vars(solver, scfia_symbol_to_isla_sym_mapping),
                ),
            ),
            ScfiaSymbolExpression::Bvult(a, b) => Exp::Bvult(
                Box::new(
                    a.to_isla_exp_from_existing_vars(solver, scfia_symbol_to_isla_sym_mapping),
                ),
                Box::new(
                    b.to_isla_exp_from_existing_vars(solver, scfia_symbol_to_isla_sym_mapping),
                ),
            ),
            ScfiaSymbolExpression::Bvslt(a, b) => Exp::Bvslt(
                Box::new(
                    a.to_isla_exp_from_existing_vars(solver, scfia_symbol_to_isla_sym_mapping),
                ),
                Box::new(
                    b.to_isla_exp_from_existing_vars(solver, scfia_symbol_to_isla_sym_mapping),
                ),
            ),
            ScfiaSymbolExpression::Bvule(a, b) => Exp::Bvule(
                Box::new(
                    a.to_isla_exp_from_existing_vars(solver, scfia_symbol_to_isla_sym_mapping),
                ),
                Box::new(
                    b.to_isla_exp_from_existing_vars(solver, scfia_symbol_to_isla_sym_mapping),
                ),
            ),
            ScfiaSymbolExpression::Bvsle(a, b) => Exp::Bvsle(
                Box::new(
                    a.to_isla_exp_from_existing_vars(solver, scfia_symbol_to_isla_sym_mapping),
                ),
                Box::new(
                    b.to_isla_exp_from_existing_vars(solver, scfia_symbol_to_isla_sym_mapping),
                ),
            ),
            ScfiaSymbolExpression::Bvuge(a, b) => Exp::Bvuge(
                Box::new(
                    a.to_isla_exp_from_existing_vars(solver, scfia_symbol_to_isla_sym_mapping),
                ),
                Box::new(
                    b.to_isla_exp_from_existing_vars(solver, scfia_symbol_to_isla_sym_mapping),
                ),
            ),
            ScfiaSymbolExpression::Bvsge(a, b) => Exp::Bvsge(
                Box::new(
                    a.to_isla_exp_from_existing_vars(solver, scfia_symbol_to_isla_sym_mapping),
                ),
                Box::new(
                    b.to_isla_exp_from_existing_vars(solver, scfia_symbol_to_isla_sym_mapping),
                ),
            ),
            ScfiaSymbolExpression::Bvugt(a, b) => Exp::Bvugt(
                Box::new(
                    a.to_isla_exp_from_existing_vars(solver, scfia_symbol_to_isla_sym_mapping),
                ),
                Box::new(
                    b.to_isla_exp_from_existing_vars(solver, scfia_symbol_to_isla_sym_mapping),
                ),
            ),
            ScfiaSymbolExpression::Bvsgt(a, b) => Exp::Bvsgt(
                Box::new(
                    a.to_isla_exp_from_existing_vars(solver, scfia_symbol_to_isla_sym_mapping),
                ),
                Box::new(
                    b.to_isla_exp_from_existing_vars(solver, scfia_symbol_to_isla_sym_mapping),
                ),
            ),
            ScfiaSymbolExpression::Extract(a, b, c) => Exp::Extract(
                *a,
                *b,
                Box::new(
                    c.to_isla_exp_from_existing_vars(solver, scfia_symbol_to_isla_sym_mapping),
                ),
            ),
            ScfiaSymbolExpression::ZeroExtend(a, b) => Exp::ZeroExtend(
                *a,
                Box::new(
                    b.to_isla_exp_from_existing_vars(solver, scfia_symbol_to_isla_sym_mapping),
                ),
            ),
            ScfiaSymbolExpression::SignExtend(a, b) => Exp::SignExtend(
                *a,
                Box::new(
                    b.to_isla_exp_from_existing_vars(solver, scfia_symbol_to_isla_sym_mapping),
                ),
            ),
            ScfiaSymbolExpression::Bvshl(a, b) => Exp::Bvshl(
                Box::new(
                    a.to_isla_exp_from_existing_vars(solver, scfia_symbol_to_isla_sym_mapping),
                ),
                Box::new(
                    b.to_isla_exp_from_existing_vars(solver, scfia_symbol_to_isla_sym_mapping),
                ),
            ),
            ScfiaSymbolExpression::Bvlshr(a, b) => Exp::Bvlshr(
                Box::new(
                    a.to_isla_exp_from_existing_vars(solver, scfia_symbol_to_isla_sym_mapping),
                ),
                Box::new(
                    b.to_isla_exp_from_existing_vars(solver, scfia_symbol_to_isla_sym_mapping),
                ),
            ),
            ScfiaSymbolExpression::Bvashr(a, b) => Exp::Bvashr(
                Box::new(
                    a.to_isla_exp_from_existing_vars(solver, scfia_symbol_to_isla_sym_mapping),
                ),
                Box::new(
                    b.to_isla_exp_from_existing_vars(solver, scfia_symbol_to_isla_sym_mapping),
                ),
            ),
            ScfiaSymbolExpression::Concat(a, b) => Exp::Concat(
                Box::new(
                    a.to_isla_exp_from_existing_vars(solver, scfia_symbol_to_isla_sym_mapping),
                ),
                Box::new(
                    b.to_isla_exp_from_existing_vars(solver, scfia_symbol_to_isla_sym_mapping),
                ),
            ),
            ScfiaSymbolExpression::Ite(a, b, c) => Exp::Ite(
                Box::new(
                    a.to_isla_exp_from_existing_vars(solver, scfia_symbol_to_isla_sym_mapping),
                ),
                Box::new(
                    b.to_isla_exp_from_existing_vars(solver, scfia_symbol_to_isla_sym_mapping),
                ),
                Box::new(
                    c.to_isla_exp_from_existing_vars(solver, scfia_symbol_to_isla_sym_mapping),
                ),
            ),
            ScfiaSymbolExpression::Select(a, b) => Exp::Select(
                Box::new(
                    a.to_isla_exp_from_existing_vars(solver, scfia_symbol_to_isla_sym_mapping),
                ),
                Box::new(
                    b.to_isla_exp_from_existing_vars(solver, scfia_symbol_to_isla_sym_mapping),
                ),
            ),
            ScfiaSymbolExpression::Store(a, b, c) => Exp::Store(
                Box::new(
                    a.to_isla_exp_from_existing_vars(solver, scfia_symbol_to_isla_sym_mapping),
                ),
                Box::new(
                    b.to_isla_exp_from_existing_vars(solver, scfia_symbol_to_isla_sym_mapping),
                ),
                Box::new(
                    c.to_isla_exp_from_existing_vars(solver, scfia_symbol_to_isla_sym_mapping),
                ),
            ),
        }
    }

    pub fn get_parents(&self, parents_vec: &mut Vec<ScfiaSymbolId>) {
        match self {
            ScfiaSymbolExpression::Var(symbol_id) => parents_vec.push(*symbol_id),
            ScfiaSymbolExpression::Bits(_b) => {}
            ScfiaSymbolExpression::Bits64(_value, _width) => {}
            ScfiaSymbolExpression::Enum(_id, _member) => {}
            ScfiaSymbolExpression::Bool(_b) => {}
            ScfiaSymbolExpression::Eq(a, b) => {
                a.get_parents(parents_vec);
                b.get_parents(parents_vec);
            }
            ScfiaSymbolExpression::Neq(a, b) => {
                a.get_parents(parents_vec);
                b.get_parents(parents_vec);
            }
            ScfiaSymbolExpression::And(a, b) => {
                a.get_parents(parents_vec);
                b.get_parents(parents_vec);
            }
            ScfiaSymbolExpression::Or(a, b) => {
                a.get_parents(parents_vec);
                b.get_parents(parents_vec);
            }
            ScfiaSymbolExpression::Not(a) => {
                a.get_parents(parents_vec);
            }
            ScfiaSymbolExpression::Bvnot(a) => {
                a.get_parents(parents_vec);
            }
            ScfiaSymbolExpression::Bvand(a, b) => {
                a.get_parents(parents_vec);
                b.get_parents(parents_vec);
            }
            ScfiaSymbolExpression::Bvor(a, b) => {
                a.get_parents(parents_vec);
                b.get_parents(parents_vec);
            }
            ScfiaSymbolExpression::Bvxor(a, b) => {
                a.get_parents(parents_vec);
                b.get_parents(parents_vec);
            }
            ScfiaSymbolExpression::Bvnand(a, b) => {
                a.get_parents(parents_vec);
                b.get_parents(parents_vec);
            }
            ScfiaSymbolExpression::Bvnor(a, b) => {
                a.get_parents(parents_vec);
                b.get_parents(parents_vec);
            }
            ScfiaSymbolExpression::Bvxnor(a, b) => {
                a.get_parents(parents_vec);
                b.get_parents(parents_vec);
            }
            ScfiaSymbolExpression::Bvneg(a) => {
                a.get_parents(parents_vec);
            }
            ScfiaSymbolExpression::Bvadd(a, b) => {
                a.get_parents(parents_vec);
                b.get_parents(parents_vec);
            }
            ScfiaSymbolExpression::Bvsub(a, b) => {
                a.get_parents(parents_vec);
                b.get_parents(parents_vec);
            }
            ScfiaSymbolExpression::Bvmul(a, b) => {
                a.get_parents(parents_vec);
                b.get_parents(parents_vec);
            }
            ScfiaSymbolExpression::Bvudiv(a, b) => {
                a.get_parents(parents_vec);
                b.get_parents(parents_vec);
            }
            ScfiaSymbolExpression::Bvsdiv(a, b) => {
                a.get_parents(parents_vec);
                b.get_parents(parents_vec);
            }
            ScfiaSymbolExpression::Bvurem(a, b) => {
                a.get_parents(parents_vec);
                b.get_parents(parents_vec);
            }
            ScfiaSymbolExpression::Bvsrem(a, b) => {
                a.get_parents(parents_vec);
                b.get_parents(parents_vec);
            }
            ScfiaSymbolExpression::Bvsmod(a, b) => {
                a.get_parents(parents_vec);
                b.get_parents(parents_vec);
            }
            ScfiaSymbolExpression::Bvult(a, b) => {
                a.get_parents(parents_vec);
                b.get_parents(parents_vec);
            }
            ScfiaSymbolExpression::Bvslt(a, b) => {
                a.get_parents(parents_vec);
                b.get_parents(parents_vec);
            }
            ScfiaSymbolExpression::Bvule(a, b) => {
                a.get_parents(parents_vec);
                b.get_parents(parents_vec);
            }
            ScfiaSymbolExpression::Bvsle(a, b) => {
                a.get_parents(parents_vec);
                b.get_parents(parents_vec);
            }
            ScfiaSymbolExpression::Bvuge(a, b) => {
                a.get_parents(parents_vec);
                b.get_parents(parents_vec);
            }
            ScfiaSymbolExpression::Bvsge(a, b) => {
                a.get_parents(parents_vec);
                b.get_parents(parents_vec);
            }
            ScfiaSymbolExpression::Bvugt(a, b) => {
                a.get_parents(parents_vec);
                b.get_parents(parents_vec);
            }
            ScfiaSymbolExpression::Bvsgt(a, b) => {
                a.get_parents(parents_vec);
                b.get_parents(parents_vec);
            }
            ScfiaSymbolExpression::Extract(_a, _b, c) => {
                c.get_parents(parents_vec);
            }
            ScfiaSymbolExpression::ZeroExtend(_a, b) => {
                b.get_parents(parents_vec);
            }
            ScfiaSymbolExpression::SignExtend(_a, b) => {
                b.get_parents(parents_vec);
            }
            ScfiaSymbolExpression::Bvshl(a, b) => {
                a.get_parents(parents_vec);
                b.get_parents(parents_vec);
            }
            ScfiaSymbolExpression::Bvlshr(a, b) => {
                a.get_parents(parents_vec);
                b.get_parents(parents_vec);
            }
            ScfiaSymbolExpression::Bvashr(a, b) => {
                a.get_parents(parents_vec);
                b.get_parents(parents_vec);
            }
            ScfiaSymbolExpression::Concat(a, b) => {
                a.get_parents(parents_vec);
                b.get_parents(parents_vec);
            }
            ScfiaSymbolExpression::Ite(a, b, c) => {
                a.get_parents(parents_vec);
                b.get_parents(parents_vec);
                c.get_parents(parents_vec);
            }
            ScfiaSymbolExpression::Select(a, b) => {
                a.get_parents(parents_vec);
                b.get_parents(parents_vec);
            }
            ScfiaSymbolExpression::Store(a, b, c) => {
                a.get_parents(parents_vec);
                b.get_parents(parents_vec);
                c.get_parents(parents_vec);
            }
        }
    }
}
