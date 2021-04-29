use crate::system_states::ScfiaSystemState;

use super::scfia_assert_expression::ScfiaAssertExpression;

/// An assertion over ScfiaSymbols.
/// Assertions are owned by their directly referenced symbols.
#[derive(Debug, Clone)]
pub struct ScfiaAssert {
    pub id: u64,
    pub expression: ScfiaAssertExpression,
}
