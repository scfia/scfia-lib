use crate::{
    symbols::{
        scfia_symbol::{ScfiaSymbol, ScfiaSymbolId},
        scfia_symbol_manager::ScfiaSymbolManager,
    },
    system_states::ScfiaSystemState,
    values::value64::Value64,
};

pub mod aarch64;
pub mod transformation_function_expression;

pub trait ScfiaTransformationDefinition<SystemState: ScfiaSystemState> {
    /// Convert the transformation definition into a symbol.
    /// This function increases the refcount of the symbol, the caller must properly decrement it.
    fn to_symbol(
        &self,
        old_state: &SystemState,
        new_symbol_manager: &mut ScfiaSymbolManager,
    ) -> ScfiaSymbolId;
}
