use std::{collections::HashSet, fmt::Display, sync::Arc};

use log::trace;

use crate::asserts::scfia_assert::ScfiaAssert;

use super::scfia_symbol_expression::ScfiaSymbolExpression;

pub type ScfiaSymbolId = u64;

#[derive(Clone, Debug)]
pub struct ScfiaSymbol {
    pub id: ScfiaSymbolId,
    pub variant: SymbolInnerVariant,
    pub references: u64,
    pub asserts: Vec<ScfiaAssert>,
    pub retired_relatives: HashSet<ScfiaSymbolId>,
    pub discovered_relatives: HashSet<ScfiaSymbolId>,
    pub reclusive: bool,
}

#[derive(Clone, Debug)]
pub enum SymbolInnerVariant {
    Bitvector(u32, Option<String>),
    Bool,
    Defined(ScfiaSymbolExpression),
}

impl Display for ScfiaSymbol {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(&format!("ScfiaSymbol({})", self.id))
    }
}
