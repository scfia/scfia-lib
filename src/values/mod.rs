use crate::symbols::scfia_symbol::ScfiaSymbolId;

pub mod value1;
pub mod value64;
pub mod value8;

pub trait ScfiaValue {
    fn new_concrete(value: u64, width: u32) -> Self;
    fn new_symbolic(symbol_id: ScfiaSymbolId) -> Self;
}
