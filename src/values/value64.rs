use std::{
    collections::HashMap,
    fmt::Display,
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
pub enum Value64 {
    Concrete(u64),
    Symbolic(ScfiaSymbolId),
}

impl Value64 {
    pub fn as_concrete(&self) -> u64 {
        match self {
            Value64::Concrete(value) => *value,
            Value64::Symbolic(_) => {
                panic!()
            }
        }
    }

    pub fn as_symbolic(&self) -> ScfiaSymbolId {
        match self {
            Value64::Concrete(_) => {
                panic!()
            }
            Value64::Symbolic(symbol_id) => *symbol_id,
        }
    }

    pub fn to_isla_val(
        &self,
        symbol_manager: &ScfiaSymbolManager,
        solver: &mut Solver<B64>,
        scfia_symbol_to_isla_sym_mapping: &mut HashMap<ScfiaSymbolId, Sym>,
    ) -> Val<B64> {
        match self {
            Value64::Concrete(value) => Val::Bits(B64::new(*value, 64)),
            Value64::Symbolic(symbol_id) => Val::Symbolic(symbol_manager.to_isla_sym(
                symbol_id,
                solver,
                scfia_symbol_to_isla_sym_mapping,
            )),
        }
    }
}

impl ScfiaValue for Value64 {
    fn new_concrete(value: u64, width: u32) -> Self {
        assert_eq!(64, width);
        Value64::Concrete(value)
    }

    fn new_symbolic(symbol_id: ScfiaSymbolId) -> Self {
        Value64::Symbolic(symbol_id)
    }
}

impl Display for Value64 {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Value64::Concrete(val) => f.pad(&format!("{:x}", val))?,
            Value64::Symbolic(_sym) => f.pad("Symbol")?,
        }
        Ok(())
    }
}
