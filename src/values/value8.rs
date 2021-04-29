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

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum Value8 {
    Concrete(u8),
    Symbolic(ScfiaSymbolId),
}

impl Value8 {
    pub fn as_concrete(&self) -> u8 {
        match self {
            Value8::Concrete(value) => *value,
            Value8::Symbolic(_) => {
                panic!()
            }
        }
    }

    pub fn as_symbolic(&self) -> ScfiaSymbolId {
        match self {
            Value8::Symbolic(symbol_id) => *symbol_id,
            Value8::Concrete(_) => {
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
            Value8::Concrete(value) => Val::Bits(B64::new(*value as u64, 8)),
            Value8::Symbolic(symbol_id) => Val::Symbolic(symbol_manager.to_isla_sym(
                symbol_id,
                solver,
                scfia_symbol_to_isla_sym_mapping,
            )),
        }
    }
}

impl ScfiaValue for Value8 {
    fn new_concrete(value: u64, width: u32) -> Self {
        assert_eq!(8, width);
        Value8::Concrete(value as u8)
    }

    fn new_symbolic(symbol_id: ScfiaSymbolId) -> Self {
        Value8::Symbolic(symbol_id)
    }
}
