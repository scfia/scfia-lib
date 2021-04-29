use std::sync::Arc;

use log::info;

use crate::asserts::scfia_assert_expression::ScfiaAssertExpression;
use crate::symbols::{
    scfia_symbol::{ScfiaSymbol, ScfiaSymbolId},
    scfia_symbol_expression::ScfiaSymbolExpression,
    scfia_symbol_manager::ScfiaSymbolManager,
};

#[derive(Clone, Debug)]
pub struct SymbolicVolatileMemoryRegion {
    pub base_symbol: ScfiaSymbolId,
    pub length: u32,
    pub little_endian_base_symbols: Vec<ScfiaSymbolId>,
}

impl SymbolicVolatileMemoryRegion {
    pub fn new(length: u32, symbol_manager: &mut ScfiaSymbolManager) -> Self {
        let base_symbol = symbol_manager.create_reclusive_symbol_bv(64);
        info!("SymbolicVolatileMemoryRegion base {}", base_symbol);
        //TODO!!! remove this fixation

        symbol_manager.increment_active_symbol_refcount(base_symbol);
        symbol_manager.add_assert(
            ScfiaAssertExpression::And(
                Arc::new(ScfiaAssertExpression::Bvult(
                    Arc::new(ScfiaAssertExpression::Var(base_symbol)),
                    Arc::new(ScfiaAssertExpression::Bits64(0x47020000, 64)),
                )),
                Arc::new(ScfiaAssertExpression::Eq(
                    Arc::new(ScfiaAssertExpression::Extract(
                        2,
                        0,
                        Arc::new(ScfiaAssertExpression::Var(base_symbol)),
                    )),
                    Arc::new(ScfiaAssertExpression::Bits64(0x0, 3)),
                )),
            ),
            vec![base_symbol],
        );
        

        let mut little_endian_base_symbols = vec![];
        // TODO put this into util and write tests for it
        for i in 0..8 {
            little_endian_base_symbols.push(symbol_manager.create_symbol_defined(
                ScfiaSymbolExpression::Extract(
                    (i * 8) + 7,
                    i * 8,
                    Arc::new(ScfiaSymbolExpression::Var(base_symbol.clone())),
                ),
            ));
        }
        SymbolicVolatileMemoryRegion {
            base_symbol,
            little_endian_base_symbols,
            length,
        }
    }
}
