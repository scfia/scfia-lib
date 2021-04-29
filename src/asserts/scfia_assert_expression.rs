use std::{
    collections::HashMap,
    sync::{Arc, Mutex},
};

use isla_lib::{
    concrete::bitvector64::B64,
    ir::EnumMember,
    smt::{smtlib::Exp, Solver, Sym},
};

use crate::{
    symbols::{scfia_symbol::ScfiaSymbolId, scfia_symbol_manager::ScfiaSymbolManager},
    system_states::ScfiaSystemState,
};

#[derive(Clone, Debug)]
pub enum ScfiaAssertExpression {
    Var(ScfiaSymbolId),
    Bits(Vec<bool>),
    Bits64(u64, u32),
    Enum(usize, usize), // enum_id, member
    Bool(bool),
    Eq(Arc<ScfiaAssertExpression>, Arc<ScfiaAssertExpression>),
    Neq(Arc<ScfiaAssertExpression>, Arc<ScfiaAssertExpression>),
    And(Arc<ScfiaAssertExpression>, Arc<ScfiaAssertExpression>),
    Or(Arc<ScfiaAssertExpression>, Arc<ScfiaAssertExpression>),
    Not(Arc<ScfiaAssertExpression>),
    Bvnot(Arc<ScfiaAssertExpression>),
    Bvand(Arc<ScfiaAssertExpression>, Arc<ScfiaAssertExpression>),
    Bvor(Arc<ScfiaAssertExpression>, Arc<ScfiaAssertExpression>),
    Bvxor(Arc<ScfiaAssertExpression>, Arc<ScfiaAssertExpression>),
    Bvnand(Arc<ScfiaAssertExpression>, Arc<ScfiaAssertExpression>),
    Bvnor(Arc<ScfiaAssertExpression>, Arc<ScfiaAssertExpression>),
    Bvxnor(Arc<ScfiaAssertExpression>, Arc<ScfiaAssertExpression>),
    Bvneg(Arc<ScfiaAssertExpression>),
    Bvadd(Arc<ScfiaAssertExpression>, Arc<ScfiaAssertExpression>),
    Bvsub(Arc<ScfiaAssertExpression>, Arc<ScfiaAssertExpression>),
    Bvmul(Arc<ScfiaAssertExpression>, Arc<ScfiaAssertExpression>),
    Bvudiv(Arc<ScfiaAssertExpression>, Arc<ScfiaAssertExpression>),
    Bvsdiv(Arc<ScfiaAssertExpression>, Arc<ScfiaAssertExpression>),
    Bvurem(Arc<ScfiaAssertExpression>, Arc<ScfiaAssertExpression>),
    Bvsrem(Arc<ScfiaAssertExpression>, Arc<ScfiaAssertExpression>),
    Bvsmod(Arc<ScfiaAssertExpression>, Arc<ScfiaAssertExpression>),
    Bvult(Arc<ScfiaAssertExpression>, Arc<ScfiaAssertExpression>),
    Bvslt(Arc<ScfiaAssertExpression>, Arc<ScfiaAssertExpression>),
    Bvule(Arc<ScfiaAssertExpression>, Arc<ScfiaAssertExpression>),
    Bvsle(Arc<ScfiaAssertExpression>, Arc<ScfiaAssertExpression>),
    Bvuge(Arc<ScfiaAssertExpression>, Arc<ScfiaAssertExpression>),
    Bvsge(Arc<ScfiaAssertExpression>, Arc<ScfiaAssertExpression>),
    Bvugt(Arc<ScfiaAssertExpression>, Arc<ScfiaAssertExpression>),
    Bvsgt(Arc<ScfiaAssertExpression>, Arc<ScfiaAssertExpression>),
    Extract(u32, u32, Arc<ScfiaAssertExpression>),
    ZeroExtend(u32, Arc<ScfiaAssertExpression>),
    SignExtend(u32, Arc<ScfiaAssertExpression>),
    Bvshl(Arc<ScfiaAssertExpression>, Arc<ScfiaAssertExpression>),
    Bvlshr(Arc<ScfiaAssertExpression>, Arc<ScfiaAssertExpression>),
    Bvashr(Arc<ScfiaAssertExpression>, Arc<ScfiaAssertExpression>),
    Concat(Arc<ScfiaAssertExpression>, Arc<ScfiaAssertExpression>),
    Ite(
        Arc<ScfiaAssertExpression>,
        Arc<ScfiaAssertExpression>,
        Arc<ScfiaAssertExpression>,
    ),
    Select(Arc<ScfiaAssertExpression>, Arc<ScfiaAssertExpression>),
    Store(
        Arc<ScfiaAssertExpression>,
        Arc<ScfiaAssertExpression>,
        Arc<ScfiaAssertExpression>,
    ),
}

impl ScfiaAssertExpression {
    pub fn to_isla_exp(
        &self,
        solver: &mut Solver<B64>,
        symbol_manager: &ScfiaSymbolManager,
        scfia_symbol_to_isla_sym_mapping: &mut HashMap<ScfiaSymbolId, Sym>,
    ) -> Exp {
        match self {
            ScfiaAssertExpression::Var(symbol_id) => Exp::Var(symbol_manager.to_isla_sym(
                symbol_id,
                solver,
                scfia_symbol_to_isla_sym_mapping,
            )),
            ScfiaAssertExpression::Bits(b) => Exp::Bits(b.clone()),
            ScfiaAssertExpression::Bits64(value, width) => Exp::Bits64(*value, *width),
            ScfiaAssertExpression::Enum(enum_id, enum_member) => Exp::Enum(EnumMember {
                enum_id: *enum_id,
                member: *enum_member,
            }),
            ScfiaAssertExpression::Bool(b) => Exp::Bool(*b),
            ScfiaAssertExpression::Eq(a, b) => Exp::Eq(
                Box::new(a.to_isla_exp(solver, symbol_manager, scfia_symbol_to_isla_sym_mapping)),
                Box::new(b.to_isla_exp(solver, symbol_manager, scfia_symbol_to_isla_sym_mapping)),
            ),
            ScfiaAssertExpression::Neq(a, b) => Exp::Neq(
                Box::new(a.to_isla_exp(solver, symbol_manager, scfia_symbol_to_isla_sym_mapping)),
                Box::new(b.to_isla_exp(solver, symbol_manager, scfia_symbol_to_isla_sym_mapping)),
            ),
            ScfiaAssertExpression::And(a, b) => Exp::And(
                Box::new(a.to_isla_exp(solver, symbol_manager, scfia_symbol_to_isla_sym_mapping)),
                Box::new(b.to_isla_exp(solver, symbol_manager, scfia_symbol_to_isla_sym_mapping)),
            ),
            ScfiaAssertExpression::Or(a, b) => Exp::Or(
                Box::new(a.to_isla_exp(solver, symbol_manager, scfia_symbol_to_isla_sym_mapping)),
                Box::new(b.to_isla_exp(solver, symbol_manager, scfia_symbol_to_isla_sym_mapping)),
            ),
            ScfiaAssertExpression::Not(a) => Exp::Not(Box::new(a.to_isla_exp(
                solver,
                symbol_manager,
                scfia_symbol_to_isla_sym_mapping,
            ))),
            ScfiaAssertExpression::Bvnot(a) => Exp::Bvnot(Box::new(a.to_isla_exp(
                solver,
                symbol_manager,
                scfia_symbol_to_isla_sym_mapping,
            ))),
            ScfiaAssertExpression::Bvand(a, b) => Exp::Bvand(
                Box::new(a.to_isla_exp(solver, symbol_manager, scfia_symbol_to_isla_sym_mapping)),
                Box::new(b.to_isla_exp(solver, symbol_manager, scfia_symbol_to_isla_sym_mapping)),
            ),
            ScfiaAssertExpression::Bvor(a, b) => Exp::Bvor(
                Box::new(a.to_isla_exp(solver, symbol_manager, scfia_symbol_to_isla_sym_mapping)),
                Box::new(b.to_isla_exp(solver, symbol_manager, scfia_symbol_to_isla_sym_mapping)),
            ),
            ScfiaAssertExpression::Bvxor(a, b) => Exp::Bvxor(
                Box::new(a.to_isla_exp(solver, symbol_manager, scfia_symbol_to_isla_sym_mapping)),
                Box::new(b.to_isla_exp(solver, symbol_manager, scfia_symbol_to_isla_sym_mapping)),
            ),
            ScfiaAssertExpression::Bvnand(a, b) => Exp::Bvnand(
                Box::new(a.to_isla_exp(solver, symbol_manager, scfia_symbol_to_isla_sym_mapping)),
                Box::new(b.to_isla_exp(solver, symbol_manager, scfia_symbol_to_isla_sym_mapping)),
            ),
            ScfiaAssertExpression::Bvnor(a, b) => Exp::Bvnor(
                Box::new(a.to_isla_exp(solver, symbol_manager, scfia_symbol_to_isla_sym_mapping)),
                Box::new(b.to_isla_exp(solver, symbol_manager, scfia_symbol_to_isla_sym_mapping)),
            ),
            ScfiaAssertExpression::Bvxnor(a, b) => Exp::Bvxnor(
                Box::new(a.to_isla_exp(solver, symbol_manager, scfia_symbol_to_isla_sym_mapping)),
                Box::new(b.to_isla_exp(solver, symbol_manager, scfia_symbol_to_isla_sym_mapping)),
            ),
            ScfiaAssertExpression::Bvneg(a) => Exp::Bvneg(Box::new(a.to_isla_exp(
                solver,
                symbol_manager,
                scfia_symbol_to_isla_sym_mapping,
            ))),
            ScfiaAssertExpression::Bvadd(a, b) => Exp::Bvadd(
                Box::new(a.to_isla_exp(solver, symbol_manager, scfia_symbol_to_isla_sym_mapping)),
                Box::new(b.to_isla_exp(solver, symbol_manager, scfia_symbol_to_isla_sym_mapping)),
            ),
            ScfiaAssertExpression::Bvsub(a, b) => Exp::Bvsub(
                Box::new(a.to_isla_exp(solver, symbol_manager, scfia_symbol_to_isla_sym_mapping)),
                Box::new(b.to_isla_exp(solver, symbol_manager, scfia_symbol_to_isla_sym_mapping)),
            ),
            ScfiaAssertExpression::Bvmul(a, b) => Exp::Bvmul(
                Box::new(a.to_isla_exp(solver, symbol_manager, scfia_symbol_to_isla_sym_mapping)),
                Box::new(b.to_isla_exp(solver, symbol_manager, scfia_symbol_to_isla_sym_mapping)),
            ),
            ScfiaAssertExpression::Bvudiv(a, b) => Exp::Bvudiv(
                Box::new(a.to_isla_exp(solver, symbol_manager, scfia_symbol_to_isla_sym_mapping)),
                Box::new(b.to_isla_exp(solver, symbol_manager, scfia_symbol_to_isla_sym_mapping)),
            ),
            ScfiaAssertExpression::Bvsdiv(a, b) => Exp::Bvsdiv(
                Box::new(a.to_isla_exp(solver, symbol_manager, scfia_symbol_to_isla_sym_mapping)),
                Box::new(b.to_isla_exp(solver, symbol_manager, scfia_symbol_to_isla_sym_mapping)),
            ),
            ScfiaAssertExpression::Bvurem(a, b) => Exp::Bvurem(
                Box::new(a.to_isla_exp(solver, symbol_manager, scfia_symbol_to_isla_sym_mapping)),
                Box::new(b.to_isla_exp(solver, symbol_manager, scfia_symbol_to_isla_sym_mapping)),
            ),
            ScfiaAssertExpression::Bvsrem(a, b) => Exp::Bvsrem(
                Box::new(a.to_isla_exp(solver, symbol_manager, scfia_symbol_to_isla_sym_mapping)),
                Box::new(b.to_isla_exp(solver, symbol_manager, scfia_symbol_to_isla_sym_mapping)),
            ),
            ScfiaAssertExpression::Bvsmod(a, b) => Exp::Bvsmod(
                Box::new(a.to_isla_exp(solver, symbol_manager, scfia_symbol_to_isla_sym_mapping)),
                Box::new(b.to_isla_exp(solver, symbol_manager, scfia_symbol_to_isla_sym_mapping)),
            ),
            ScfiaAssertExpression::Bvult(a, b) => Exp::Bvult(
                Box::new(a.to_isla_exp(solver, symbol_manager, scfia_symbol_to_isla_sym_mapping)),
                Box::new(b.to_isla_exp(solver, symbol_manager, scfia_symbol_to_isla_sym_mapping)),
            ),
            ScfiaAssertExpression::Bvslt(a, b) => Exp::Bvslt(
                Box::new(a.to_isla_exp(solver, symbol_manager, scfia_symbol_to_isla_sym_mapping)),
                Box::new(b.to_isla_exp(solver, symbol_manager, scfia_symbol_to_isla_sym_mapping)),
            ),
            ScfiaAssertExpression::Bvule(a, b) => Exp::Bvule(
                Box::new(a.to_isla_exp(solver, symbol_manager, scfia_symbol_to_isla_sym_mapping)),
                Box::new(b.to_isla_exp(solver, symbol_manager, scfia_symbol_to_isla_sym_mapping)),
            ),
            ScfiaAssertExpression::Bvsle(a, b) => Exp::Bvsle(
                Box::new(a.to_isla_exp(solver, symbol_manager, scfia_symbol_to_isla_sym_mapping)),
                Box::new(b.to_isla_exp(solver, symbol_manager, scfia_symbol_to_isla_sym_mapping)),
            ),
            ScfiaAssertExpression::Bvuge(a, b) => Exp::Bvuge(
                Box::new(a.to_isla_exp(solver, symbol_manager, scfia_symbol_to_isla_sym_mapping)),
                Box::new(b.to_isla_exp(solver, symbol_manager, scfia_symbol_to_isla_sym_mapping)),
            ),
            ScfiaAssertExpression::Bvsge(a, b) => Exp::Bvsge(
                Box::new(a.to_isla_exp(solver, symbol_manager, scfia_symbol_to_isla_sym_mapping)),
                Box::new(b.to_isla_exp(solver, symbol_manager, scfia_symbol_to_isla_sym_mapping)),
            ),
            ScfiaAssertExpression::Bvugt(a, b) => Exp::Bvugt(
                Box::new(a.to_isla_exp(solver, symbol_manager, scfia_symbol_to_isla_sym_mapping)),
                Box::new(b.to_isla_exp(solver, symbol_manager, scfia_symbol_to_isla_sym_mapping)),
            ),
            ScfiaAssertExpression::Bvsgt(a, b) => Exp::Bvsgt(
                Box::new(a.to_isla_exp(solver, symbol_manager, scfia_symbol_to_isla_sym_mapping)),
                Box::new(b.to_isla_exp(solver, symbol_manager, scfia_symbol_to_isla_sym_mapping)),
            ),
            ScfiaAssertExpression::Extract(a, b, c) => Exp::Extract(
                *a,
                *b,
                Box::new(c.to_isla_exp(solver, symbol_manager, scfia_symbol_to_isla_sym_mapping)),
            ),
            ScfiaAssertExpression::ZeroExtend(a, b) => Exp::ZeroExtend(
                *a,
                Box::new(b.to_isla_exp(solver, symbol_manager, scfia_symbol_to_isla_sym_mapping)),
            ),
            ScfiaAssertExpression::SignExtend(a, b) => Exp::SignExtend(
                *a,
                Box::new(b.to_isla_exp(solver, symbol_manager, scfia_symbol_to_isla_sym_mapping)),
            ),
            ScfiaAssertExpression::Bvshl(a, b) => Exp::Bvshl(
                Box::new(a.to_isla_exp(solver, symbol_manager, scfia_symbol_to_isla_sym_mapping)),
                Box::new(b.to_isla_exp(solver, symbol_manager, scfia_symbol_to_isla_sym_mapping)),
            ),
            ScfiaAssertExpression::Bvlshr(a, b) => Exp::Bvlshr(
                Box::new(a.to_isla_exp(solver, symbol_manager, scfia_symbol_to_isla_sym_mapping)),
                Box::new(b.to_isla_exp(solver, symbol_manager, scfia_symbol_to_isla_sym_mapping)),
            ),
            ScfiaAssertExpression::Bvashr(a, b) => Exp::Bvashr(
                Box::new(a.to_isla_exp(solver, symbol_manager, scfia_symbol_to_isla_sym_mapping)),
                Box::new(b.to_isla_exp(solver, symbol_manager, scfia_symbol_to_isla_sym_mapping)),
            ),
            ScfiaAssertExpression::Concat(a, b) => Exp::Concat(
                Box::new(a.to_isla_exp(solver, symbol_manager, scfia_symbol_to_isla_sym_mapping)),
                Box::new(b.to_isla_exp(solver, symbol_manager, scfia_symbol_to_isla_sym_mapping)),
            ),
            ScfiaAssertExpression::Ite(a, b, c) => Exp::Ite(
                Box::new(a.to_isla_exp(solver, symbol_manager, scfia_symbol_to_isla_sym_mapping)),
                Box::new(b.to_isla_exp(solver, symbol_manager, scfia_symbol_to_isla_sym_mapping)),
                Box::new(c.to_isla_exp(solver, symbol_manager, scfia_symbol_to_isla_sym_mapping)),
            ),
            ScfiaAssertExpression::Select(a, b) => Exp::Select(
                Box::new(a.to_isla_exp(solver, symbol_manager, scfia_symbol_to_isla_sym_mapping)),
                Box::new(b.to_isla_exp(solver, symbol_manager, scfia_symbol_to_isla_sym_mapping)),
            ),
            ScfiaAssertExpression::Store(a, b, c) => Exp::Store(
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
            ScfiaAssertExpression::Var(symbol_id) => {
                Exp::Var(*scfia_symbol_to_isla_sym_mapping.get(symbol_id).unwrap())
            }
            ScfiaAssertExpression::Bits(b) => Exp::Bits(b.clone()),
            ScfiaAssertExpression::Bits64(value, width) => Exp::Bits64(*value, *width),
            ScfiaAssertExpression::Enum(enum_id, enum_member) => Exp::Enum(EnumMember {
                enum_id: *enum_id,
                member: *enum_member,
            }),
            ScfiaAssertExpression::Bool(b) => Exp::Bool(*b),
            ScfiaAssertExpression::Eq(a, b) => Exp::Eq(
                Box::new(
                    a.to_isla_exp_from_existing_vars(solver, scfia_symbol_to_isla_sym_mapping),
                ),
                Box::new(
                    b.to_isla_exp_from_existing_vars(solver, scfia_symbol_to_isla_sym_mapping),
                ),
            ),
            ScfiaAssertExpression::Neq(a, b) => Exp::Neq(
                Box::new(
                    a.to_isla_exp_from_existing_vars(solver, scfia_symbol_to_isla_sym_mapping),
                ),
                Box::new(
                    b.to_isla_exp_from_existing_vars(solver, scfia_symbol_to_isla_sym_mapping),
                ),
            ),
            ScfiaAssertExpression::And(a, b) => Exp::And(
                Box::new(
                    a.to_isla_exp_from_existing_vars(solver, scfia_symbol_to_isla_sym_mapping),
                ),
                Box::new(
                    b.to_isla_exp_from_existing_vars(solver, scfia_symbol_to_isla_sym_mapping),
                ),
            ),
            ScfiaAssertExpression::Or(a, b) => Exp::Or(
                Box::new(
                    a.to_isla_exp_from_existing_vars(solver, scfia_symbol_to_isla_sym_mapping),
                ),
                Box::new(
                    b.to_isla_exp_from_existing_vars(solver, scfia_symbol_to_isla_sym_mapping),
                ),
            ),
            ScfiaAssertExpression::Not(a) => Exp::Not(Box::new(
                a.to_isla_exp_from_existing_vars(solver, scfia_symbol_to_isla_sym_mapping),
            )),
            ScfiaAssertExpression::Bvnot(a) => Exp::Bvnot(Box::new(
                a.to_isla_exp_from_existing_vars(solver, scfia_symbol_to_isla_sym_mapping),
            )),
            ScfiaAssertExpression::Bvand(a, b) => Exp::Bvand(
                Box::new(
                    a.to_isla_exp_from_existing_vars(solver, scfia_symbol_to_isla_sym_mapping),
                ),
                Box::new(
                    b.to_isla_exp_from_existing_vars(solver, scfia_symbol_to_isla_sym_mapping),
                ),
            ),
            ScfiaAssertExpression::Bvor(a, b) => Exp::Bvor(
                Box::new(
                    a.to_isla_exp_from_existing_vars(solver, scfia_symbol_to_isla_sym_mapping),
                ),
                Box::new(
                    b.to_isla_exp_from_existing_vars(solver, scfia_symbol_to_isla_sym_mapping),
                ),
            ),
            ScfiaAssertExpression::Bvxor(a, b) => Exp::Bvxor(
                Box::new(
                    a.to_isla_exp_from_existing_vars(solver, scfia_symbol_to_isla_sym_mapping),
                ),
                Box::new(
                    b.to_isla_exp_from_existing_vars(solver, scfia_symbol_to_isla_sym_mapping),
                ),
            ),
            ScfiaAssertExpression::Bvnand(a, b) => Exp::Bvnand(
                Box::new(
                    a.to_isla_exp_from_existing_vars(solver, scfia_symbol_to_isla_sym_mapping),
                ),
                Box::new(
                    b.to_isla_exp_from_existing_vars(solver, scfia_symbol_to_isla_sym_mapping),
                ),
            ),
            ScfiaAssertExpression::Bvnor(a, b) => Exp::Bvnor(
                Box::new(
                    a.to_isla_exp_from_existing_vars(solver, scfia_symbol_to_isla_sym_mapping),
                ),
                Box::new(
                    b.to_isla_exp_from_existing_vars(solver, scfia_symbol_to_isla_sym_mapping),
                ),
            ),
            ScfiaAssertExpression::Bvxnor(a, b) => Exp::Bvxnor(
                Box::new(
                    a.to_isla_exp_from_existing_vars(solver, scfia_symbol_to_isla_sym_mapping),
                ),
                Box::new(
                    b.to_isla_exp_from_existing_vars(solver, scfia_symbol_to_isla_sym_mapping),
                ),
            ),
            ScfiaAssertExpression::Bvneg(a) => Exp::Bvneg(Box::new(
                a.to_isla_exp_from_existing_vars(solver, scfia_symbol_to_isla_sym_mapping),
            )),
            ScfiaAssertExpression::Bvadd(a, b) => Exp::Bvadd(
                Box::new(
                    a.to_isla_exp_from_existing_vars(solver, scfia_symbol_to_isla_sym_mapping),
                ),
                Box::new(
                    b.to_isla_exp_from_existing_vars(solver, scfia_symbol_to_isla_sym_mapping),
                ),
            ),
            ScfiaAssertExpression::Bvsub(a, b) => Exp::Bvsub(
                Box::new(
                    a.to_isla_exp_from_existing_vars(solver, scfia_symbol_to_isla_sym_mapping),
                ),
                Box::new(
                    b.to_isla_exp_from_existing_vars(solver, scfia_symbol_to_isla_sym_mapping),
                ),
            ),
            ScfiaAssertExpression::Bvmul(a, b) => Exp::Bvmul(
                Box::new(
                    a.to_isla_exp_from_existing_vars(solver, scfia_symbol_to_isla_sym_mapping),
                ),
                Box::new(
                    b.to_isla_exp_from_existing_vars(solver, scfia_symbol_to_isla_sym_mapping),
                ),
            ),
            ScfiaAssertExpression::Bvudiv(a, b) => Exp::Bvudiv(
                Box::new(
                    a.to_isla_exp_from_existing_vars(solver, scfia_symbol_to_isla_sym_mapping),
                ),
                Box::new(
                    b.to_isla_exp_from_existing_vars(solver, scfia_symbol_to_isla_sym_mapping),
                ),
            ),
            ScfiaAssertExpression::Bvsdiv(a, b) => Exp::Bvsdiv(
                Box::new(
                    a.to_isla_exp_from_existing_vars(solver, scfia_symbol_to_isla_sym_mapping),
                ),
                Box::new(
                    b.to_isla_exp_from_existing_vars(solver, scfia_symbol_to_isla_sym_mapping),
                ),
            ),
            ScfiaAssertExpression::Bvurem(a, b) => Exp::Bvurem(
                Box::new(
                    a.to_isla_exp_from_existing_vars(solver, scfia_symbol_to_isla_sym_mapping),
                ),
                Box::new(
                    b.to_isla_exp_from_existing_vars(solver, scfia_symbol_to_isla_sym_mapping),
                ),
            ),
            ScfiaAssertExpression::Bvsrem(a, b) => Exp::Bvsrem(
                Box::new(
                    a.to_isla_exp_from_existing_vars(solver, scfia_symbol_to_isla_sym_mapping),
                ),
                Box::new(
                    b.to_isla_exp_from_existing_vars(solver, scfia_symbol_to_isla_sym_mapping),
                ),
            ),
            ScfiaAssertExpression::Bvsmod(a, b) => Exp::Bvsmod(
                Box::new(
                    a.to_isla_exp_from_existing_vars(solver, scfia_symbol_to_isla_sym_mapping),
                ),
                Box::new(
                    b.to_isla_exp_from_existing_vars(solver, scfia_symbol_to_isla_sym_mapping),
                ),
            ),
            ScfiaAssertExpression::Bvult(a, b) => Exp::Bvult(
                Box::new(
                    a.to_isla_exp_from_existing_vars(solver, scfia_symbol_to_isla_sym_mapping),
                ),
                Box::new(
                    b.to_isla_exp_from_existing_vars(solver, scfia_symbol_to_isla_sym_mapping),
                ),
            ),
            ScfiaAssertExpression::Bvslt(a, b) => Exp::Bvslt(
                Box::new(
                    a.to_isla_exp_from_existing_vars(solver, scfia_symbol_to_isla_sym_mapping),
                ),
                Box::new(
                    b.to_isla_exp_from_existing_vars(solver, scfia_symbol_to_isla_sym_mapping),
                ),
            ),
            ScfiaAssertExpression::Bvule(a, b) => Exp::Bvule(
                Box::new(
                    a.to_isla_exp_from_existing_vars(solver, scfia_symbol_to_isla_sym_mapping),
                ),
                Box::new(
                    b.to_isla_exp_from_existing_vars(solver, scfia_symbol_to_isla_sym_mapping),
                ),
            ),
            ScfiaAssertExpression::Bvsle(a, b) => Exp::Bvsle(
                Box::new(
                    a.to_isla_exp_from_existing_vars(solver, scfia_symbol_to_isla_sym_mapping),
                ),
                Box::new(
                    b.to_isla_exp_from_existing_vars(solver, scfia_symbol_to_isla_sym_mapping),
                ),
            ),
            ScfiaAssertExpression::Bvuge(a, b) => Exp::Bvuge(
                Box::new(
                    a.to_isla_exp_from_existing_vars(solver, scfia_symbol_to_isla_sym_mapping),
                ),
                Box::new(
                    b.to_isla_exp_from_existing_vars(solver, scfia_symbol_to_isla_sym_mapping),
                ),
            ),
            ScfiaAssertExpression::Bvsge(a, b) => Exp::Bvsge(
                Box::new(
                    a.to_isla_exp_from_existing_vars(solver, scfia_symbol_to_isla_sym_mapping),
                ),
                Box::new(
                    b.to_isla_exp_from_existing_vars(solver, scfia_symbol_to_isla_sym_mapping),
                ),
            ),
            ScfiaAssertExpression::Bvugt(a, b) => Exp::Bvugt(
                Box::new(
                    a.to_isla_exp_from_existing_vars(solver, scfia_symbol_to_isla_sym_mapping),
                ),
                Box::new(
                    b.to_isla_exp_from_existing_vars(solver, scfia_symbol_to_isla_sym_mapping),
                ),
            ),
            ScfiaAssertExpression::Bvsgt(a, b) => Exp::Bvsgt(
                Box::new(
                    a.to_isla_exp_from_existing_vars(solver, scfia_symbol_to_isla_sym_mapping),
                ),
                Box::new(
                    b.to_isla_exp_from_existing_vars(solver, scfia_symbol_to_isla_sym_mapping),
                ),
            ),
            ScfiaAssertExpression::Extract(a, b, c) => Exp::Extract(
                *a,
                *b,
                Box::new(
                    c.to_isla_exp_from_existing_vars(solver, scfia_symbol_to_isla_sym_mapping),
                ),
            ),
            ScfiaAssertExpression::ZeroExtend(a, b) => Exp::ZeroExtend(
                *a,
                Box::new(
                    b.to_isla_exp_from_existing_vars(solver, scfia_symbol_to_isla_sym_mapping),
                ),
            ),
            ScfiaAssertExpression::SignExtend(a, b) => Exp::SignExtend(
                *a,
                Box::new(
                    b.to_isla_exp_from_existing_vars(solver, scfia_symbol_to_isla_sym_mapping),
                ),
            ),
            ScfiaAssertExpression::Bvshl(a, b) => Exp::Bvshl(
                Box::new(
                    a.to_isla_exp_from_existing_vars(solver, scfia_symbol_to_isla_sym_mapping),
                ),
                Box::new(
                    b.to_isla_exp_from_existing_vars(solver, scfia_symbol_to_isla_sym_mapping),
                ),
            ),
            ScfiaAssertExpression::Bvlshr(a, b) => Exp::Bvlshr(
                Box::new(
                    a.to_isla_exp_from_existing_vars(solver, scfia_symbol_to_isla_sym_mapping),
                ),
                Box::new(
                    b.to_isla_exp_from_existing_vars(solver, scfia_symbol_to_isla_sym_mapping),
                ),
            ),
            ScfiaAssertExpression::Bvashr(a, b) => Exp::Bvashr(
                Box::new(
                    a.to_isla_exp_from_existing_vars(solver, scfia_symbol_to_isla_sym_mapping),
                ),
                Box::new(
                    b.to_isla_exp_from_existing_vars(solver, scfia_symbol_to_isla_sym_mapping),
                ),
            ),
            ScfiaAssertExpression::Concat(a, b) => Exp::Concat(
                Box::new(
                    a.to_isla_exp_from_existing_vars(solver, scfia_symbol_to_isla_sym_mapping),
                ),
                Box::new(
                    b.to_isla_exp_from_existing_vars(solver, scfia_symbol_to_isla_sym_mapping),
                ),
            ),
            ScfiaAssertExpression::Ite(a, b, c) => Exp::Ite(
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
            ScfiaAssertExpression::Select(a, b) => Exp::Select(
                Box::new(
                    a.to_isla_exp_from_existing_vars(solver, scfia_symbol_to_isla_sym_mapping),
                ),
                Box::new(
                    b.to_isla_exp_from_existing_vars(solver, scfia_symbol_to_isla_sym_mapping),
                ),
            ),
            ScfiaAssertExpression::Store(a, b, c) => Exp::Store(
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
            ScfiaAssertExpression::Var(symbol_id) => parents_vec.push(*symbol_id),
            ScfiaAssertExpression::Bits(_b) => {}
            ScfiaAssertExpression::Bits64(_value, _width) => {}
            ScfiaAssertExpression::Enum(_id, _member) => {}
            ScfiaAssertExpression::Bool(_b) => {}
            ScfiaAssertExpression::Eq(a, b) => {
                a.get_parents(parents_vec);
                b.get_parents(parents_vec);
            }
            ScfiaAssertExpression::Neq(a, b) => {
                a.get_parents(parents_vec);
                b.get_parents(parents_vec);
            }
            ScfiaAssertExpression::And(a, b) => {
                a.get_parents(parents_vec);
                b.get_parents(parents_vec);
            }
            ScfiaAssertExpression::Or(a, b) => {
                a.get_parents(parents_vec);
                b.get_parents(parents_vec);
            }
            ScfiaAssertExpression::Not(a) => {
                a.get_parents(parents_vec);
            }
            ScfiaAssertExpression::Bvnot(a) => {
                a.get_parents(parents_vec);
            }
            ScfiaAssertExpression::Bvand(a, b) => {
                a.get_parents(parents_vec);
                b.get_parents(parents_vec);
            }
            ScfiaAssertExpression::Bvor(a, b) => {
                a.get_parents(parents_vec);
                b.get_parents(parents_vec);
            }
            ScfiaAssertExpression::Bvxor(a, b) => {
                a.get_parents(parents_vec);
                b.get_parents(parents_vec);
            }
            ScfiaAssertExpression::Bvnand(a, b) => {
                a.get_parents(parents_vec);
                b.get_parents(parents_vec);
            }
            ScfiaAssertExpression::Bvnor(a, b) => {
                a.get_parents(parents_vec);
                b.get_parents(parents_vec);
            }
            ScfiaAssertExpression::Bvxnor(a, b) => {
                a.get_parents(parents_vec);
                b.get_parents(parents_vec);
            }
            ScfiaAssertExpression::Bvneg(a) => {
                a.get_parents(parents_vec);
            }
            ScfiaAssertExpression::Bvadd(a, b) => {
                a.get_parents(parents_vec);
                b.get_parents(parents_vec);
            }
            ScfiaAssertExpression::Bvsub(a, b) => {
                a.get_parents(parents_vec);
                b.get_parents(parents_vec);
            }
            ScfiaAssertExpression::Bvmul(a, b) => {
                a.get_parents(parents_vec);
                b.get_parents(parents_vec);
            }
            ScfiaAssertExpression::Bvudiv(a, b) => {
                a.get_parents(parents_vec);
                b.get_parents(parents_vec);
            }
            ScfiaAssertExpression::Bvsdiv(a, b) => {
                a.get_parents(parents_vec);
                b.get_parents(parents_vec);
            }
            ScfiaAssertExpression::Bvurem(a, b) => {
                a.get_parents(parents_vec);
                b.get_parents(parents_vec);
            }
            ScfiaAssertExpression::Bvsrem(a, b) => {
                a.get_parents(parents_vec);
                b.get_parents(parents_vec);
            }
            ScfiaAssertExpression::Bvsmod(a, b) => {
                a.get_parents(parents_vec);
                b.get_parents(parents_vec);
            }
            ScfiaAssertExpression::Bvult(a, b) => {
                a.get_parents(parents_vec);
                b.get_parents(parents_vec);
            }
            ScfiaAssertExpression::Bvslt(a, b) => {
                a.get_parents(parents_vec);
                b.get_parents(parents_vec);
            }
            ScfiaAssertExpression::Bvule(a, b) => {
                a.get_parents(parents_vec);
                b.get_parents(parents_vec);
            }
            ScfiaAssertExpression::Bvsle(a, b) => {
                a.get_parents(parents_vec);
                b.get_parents(parents_vec);
            }
            ScfiaAssertExpression::Bvuge(a, b) => {
                a.get_parents(parents_vec);
                b.get_parents(parents_vec);
            }
            ScfiaAssertExpression::Bvsge(a, b) => {
                a.get_parents(parents_vec);
                b.get_parents(parents_vec);
            }
            ScfiaAssertExpression::Bvugt(a, b) => {
                a.get_parents(parents_vec);
                b.get_parents(parents_vec);
            }
            ScfiaAssertExpression::Bvsgt(a, b) => {
                a.get_parents(parents_vec);
                b.get_parents(parents_vec);
            }
            ScfiaAssertExpression::Extract(_a, _b, c) => {
                c.get_parents(parents_vec);
            }
            ScfiaAssertExpression::ZeroExtend(_a, b) => {
                b.get_parents(parents_vec);
            }
            ScfiaAssertExpression::SignExtend(_a, b) => {
                b.get_parents(parents_vec);
            }
            ScfiaAssertExpression::Bvshl(a, b) => {
                a.get_parents(parents_vec);
                b.get_parents(parents_vec);
            }
            ScfiaAssertExpression::Bvlshr(a, b) => {
                a.get_parents(parents_vec);
                b.get_parents(parents_vec);
            }
            ScfiaAssertExpression::Bvashr(a, b) => {
                a.get_parents(parents_vec);
                b.get_parents(parents_vec);
            }
            ScfiaAssertExpression::Concat(a, b) => {
                a.get_parents(parents_vec);
                b.get_parents(parents_vec);
            }
            ScfiaAssertExpression::Ite(a, b, c) => {
                a.get_parents(parents_vec);
                b.get_parents(parents_vec);
                c.get_parents(parents_vec);
            }
            ScfiaAssertExpression::Select(a, b) => {
                a.get_parents(parents_vec);
                b.get_parents(parents_vec);
            }
            ScfiaAssertExpression::Store(a, b, c) => {
                a.get_parents(parents_vec);
                b.get_parents(parents_vec);
                c.get_parents(parents_vec);
            }
        }
    }
}
