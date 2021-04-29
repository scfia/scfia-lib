use std::{
    collections::HashMap,
    sync::{Arc, Mutex},
};

use isla_lib::{
    concrete::{bitvector64::B64, BV},
    ir::{UVal, Val},
    smt::{Solver, Sym},
};

use crate::{
    symbols::{scfia_symbol::ScfiaSymbolId, scfia_symbol_manager::ScfiaSymbolManager},
    system_states::ScfiaSystemState,
};

use super::ScfiaValue;

#[derive(Clone, Debug)]
pub enum Value1 {
    Concrete(u64),
    Symbolic(ScfiaSymbolId),
}

impl Value1 {
    pub fn as_concrete(&self) -> u64 {
        match self {
            Value1::Concrete(value) => *value,
            Value1::Symbolic(_) => {
                panic!()
            }
        }
    }

    pub fn as_symbolic(&self) -> ScfiaSymbolId {
        match self {
            Value1::Symbolic(symbol_id) => *symbol_id,
            Value1::Concrete(_) => {
                panic!()
            }
        }
    }

    pub fn to_isla_val(
        &self,
        symbol_manager: &ScfiaSymbolManager,
        solver: &mut Solver<B64>,
        scfia_symbol_to_isla_sym_mapping: &mut HashMap<ScfiaSymbolId, Sym>,
    ) -> Val<B64> {
        match self {
            Value1::Concrete(value) => Val::Bits(B64::new(*value, 1)),
            Value1::Symbolic(symbol_id) => Val::Symbolic(symbol_manager.to_isla_sym(
                symbol_id,
                solver,
                scfia_symbol_to_isla_sym_mapping,
            )),
        }
    }
}

impl ScfiaValue for Value1 {
    fn new_concrete(value: u64, width: u32) -> Self {
        assert_eq!(1, width);
        Value1::Concrete(value)
    }

    fn new_symbolic(symbol_id: ScfiaSymbolId) -> Self {
        Value1::Symbolic(symbol_id)
    }
}
