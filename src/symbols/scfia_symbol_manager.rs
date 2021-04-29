use isla_lib::{
    concrete::bitvector64::B64,
    smt::{
        smtlib::{Def, Ty},
        Solver, Sym,
    },
};
use log::{debug, trace};

use crate::transformation_functions::transformation_function_expression::ScfiaTransformationFunctionExpression;
use crate::{
    asserts::{scfia_assert::ScfiaAssert, scfia_assert_expression::ScfiaAssertExpression},
    system_states::ScfiaSystemState,
};
use std::collections::{HashMap, HashSet};
use std::sync::Arc;
use std::sync::Mutex;

use super::{
    scfia_symbol::{ScfiaSymbol, ScfiaSymbolId, SymbolInnerVariant},
    scfia_symbol_expression::ScfiaSymbolExpression,
};

#[derive(Clone, Debug)]
pub struct ScfiaSymbolManager {
    pub next_symbol_id: ScfiaSymbolId,
    pub active_symbols: HashMap<u64, ScfiaSymbol>,
    pub retired_symbols: HashMap<u64, ScfiaSymbol>,
    next_assert_id: u64,
}

impl ScfiaSymbolManager {
    pub fn new() -> Self {
        ScfiaSymbolManager {
            next_symbol_id: 0,
            active_symbols: HashMap::new(),
            retired_symbols: HashMap::new(),
            next_assert_id: 0,
        }
    }

    pub fn get_symbol(&self, symbol_id: &ScfiaSymbolId) -> &ScfiaSymbol {
        if let Some(symbol) = self.active_symbols.get(symbol_id) {
            symbol
        } else if let Some(symbol) = self.retired_symbols.get(symbol_id) {
            symbol
        } else {
            panic!("coud not find symbol {}", symbol_id)
        }
    }

    /// Creates a new, unconstrained bitvector symbol.
    /// The Caller must ensure the initial reference is decremented.
    pub fn create_symbol_bv(&mut self, width: u32) -> ScfiaSymbolId {
        let next = self.next();
        let symbol = ScfiaSymbol {
            id: next,
            variant: SymbolInnerVariant::Bitvector(width, None),
            references: 1,
            asserts: vec![],
            retired_relatives: HashSet::new(),
            discovered_relatives: HashSet::new(),
            reclusive: false,
        };
        self.active_symbols.insert(next, symbol);
        next
    }

    pub fn create_reclusive_symbol_bv(&mut self, width: u32) -> ScfiaSymbolId {
        let next = self.next();
        let symbol = ScfiaSymbol {
            id: next,
            variant: SymbolInnerVariant::Bitvector(width, None),
            references: 1,
            asserts: vec![],
            retired_relatives: HashSet::new(),
            discovered_relatives: HashSet::new(),
            reclusive: true,
        };
        self.active_symbols.insert(next, symbol);
        next
    }

    /// Creates a new, defined symbol.
    /// The Caller must ensure the the parents' refcount is incremented, and the refcount of the new symbol is decremented later.
    pub fn create_symbol_defined(&mut self, expression: ScfiaSymbolExpression) -> ScfiaSymbolId {
        let next = self.next();
        let symbol = ScfiaSymbol {
            id: next,
            variant: SymbolInnerVariant::Defined(expression),
            references: 1,
            asserts: vec![],
            retired_relatives: HashSet::new(),
            discovered_relatives: HashSet::new(),
            reclusive: false,
        };
        self.active_symbols.insert(next, symbol);
        next
    }

    fn next(&mut self) -> ScfiaSymbolId {
        let next = self.next_symbol_id;
        self.next_symbol_id += 1;
        next
    }

    pub fn decrement_active_symbol_refcount(&mut self, symbol_id: &ScfiaSymbolId) {
        trace!("decrement_active_symbol_refcount({})", symbol_id);
        let retain = {
            let old_symbol_handle = self.active_symbols.get_mut(&symbol_id).unwrap();
            old_symbol_handle.references -= 1;
            old_symbol_handle.references > 0
        };
        if !retain {
            self.retire_symbol(symbol_id)
        }
    }

    pub fn increment_active_symbol_refcount(&mut self, symbol_id: ScfiaSymbolId) {
        trace!("increment_active_symbol_refcount({})", symbol_id);
        self.active_symbols.get_mut(&symbol_id).unwrap().references += 1;
    }

    pub fn decrement_retired_symbol_refcount(&mut self, symbol_id: &ScfiaSymbolId) {
        trace!("decrement_retired_symbol_refcount({})", symbol_id);
        let retain = {
            let old_symbol_handle = self.retired_symbols.get_mut(symbol_id).unwrap();
            old_symbol_handle.references -= 1;
            trace!(
                "decrement_retired_symbol_refcount new count {}",
                old_symbol_handle.references
            );
            old_symbol_handle.references > 0
        };
        if !retain {
            self.decommission_symbol(symbol_id);
        }
    }

    fn retire_symbol(&mut self, symbol_id: &ScfiaSymbolId) {
        trace!("retire_symbol {}", symbol_id);
        // (1) Remove the symbol from the active set
        let mut symbol = self.active_symbols.remove(symbol_id).unwrap();
        symbol.references = 0;

        // (2) Determine all heirs
        let mut heirs = symbol.discovered_relatives.clone();
        let mut parents = vec![];
        match &symbol.variant {
            SymbolInnerVariant::Defined(expression) => {
                expression.get_parents(&mut parents);
                // assert!(parents.len() > 0);
                for parent in &parents {
                    let heir = self.active_symbols.get_mut(parent).unwrap();
                    if !heir.reclusive {
                        heirs.insert(*parent);
                    }
                }
            }
            _ => {}
        };

        // (3) For each heir:
        for heir_id in &heirs {
            let heir = self.active_symbols.get_mut(heir_id).unwrap();

            // Remove the symbol from the discovered relatives lists
            heir.discovered_relatives.remove(&symbol_id);

            // Acquaint all heirs
            for other_heir_id in &heirs {
                if heir_id != other_heir_id {
                    heir.discovered_relatives.insert(*other_heir_id);
                }
            }

            // Inherit retired relatives
            for retired_relative_id in &symbol.retired_relatives {
                if heir.retired_relatives.insert(*retired_relative_id) {
                    let retired_relative =
                        self.retired_symbols.get_mut(&retired_relative_id).unwrap();
                    retired_relative.references += 1;
                }
            }

            // Inherit symbol
            if heir.retired_relatives.insert(*symbol_id) {
                symbol.references += 1;
            }
        }

        // (4) Decrement retired set refcount on retired relatives
        for retired_relative_id in &symbol.retired_relatives {
            self.decrement_retired_symbol_refcount(retired_relative_id);
        }

        if heirs.len() > 0 {
            symbol.retired_relatives.clear();
            symbol.discovered_relatives.clear();
            self.retired_symbols.insert(symbol.id, symbol);
        }

        // (5) Decrement active set refcount on parents
        for parent_id in &parents {
            self.decrement_active_symbol_refcount(parent_id);
        }
    }

    fn decommission_symbol(&mut self, symbol_id: &ScfiaSymbolId) {
        trace!("decommission_symbol {}", symbol_id);
        self.retired_symbols.remove(symbol_id).unwrap();
    }

    pub fn add_assert(
        &mut self,
        scfia_assert_expression: ScfiaAssertExpression,
        affected_symbol_ids: Vec<ScfiaSymbolId>,
    ) {
        trace!("add_assert to symbol {}", affected_symbol_ids[0]);
        let assert = ScfiaAssert {
            id: self.next_assert_id,
            expression: scfia_assert_expression,
        };

        self.active_symbols
            .get_mut(&affected_symbol_ids[0])
            .unwrap()
            .asserts
            .push(assert);
        for symbol_id in &affected_symbol_ids {
            let symbol = self.active_symbols.get_mut(symbol_id).unwrap();

            for other_symbol_id in &affected_symbol_ids {
                if symbol_id != other_symbol_id {
                    symbol.discovered_relatives.insert(*other_symbol_id);
                }
            }
        }

        // TODO to_assert_expression increments the refcount, so this hack decrements it again
        for symbol_id in &affected_symbol_ids {
            self.decrement_active_symbol_refcount(symbol_id);
        }

        self.next_assert_id += 1;
    }

    pub fn to_isla_sym(
        &self,
        symbol_id: &ScfiaSymbolId,
        solver: &mut Solver<B64>,
        scfia_symbol_to_isla_sym_mapping: &mut HashMap<ScfiaSymbolId, Sym>,
    ) -> Sym {
        if let Some(sym) = scfia_symbol_to_isla_sym_mapping.get(symbol_id) {
            *sym
        } else {
            let symbol = self.get_symbol(symbol_id);
            let sym =
                self.translate_to_isla_symbol(&symbol, solver, scfia_symbol_to_isla_sym_mapping);
            //*
            // These relatives might already have been translated if they were connected to the transitive parents.
            for retired_relative in &symbol.retired_relatives {
                if !scfia_symbol_to_isla_sym_mapping.contains_key(retired_relative) {
                    self.to_isla_sym(retired_relative, solver, scfia_symbol_to_isla_sym_mapping);
                }
            }
            for discovered_relative in &symbol.discovered_relatives {
                if !scfia_symbol_to_isla_sym_mapping.contains_key(discovered_relative) {
                    self.to_isla_sym(
                        discovered_relative,
                        solver,
                        scfia_symbol_to_isla_sym_mapping,
                    );
                }
            }
            for assert in &symbol.asserts {
                trace!("translating assert {:?}", assert);
                let isla_assert = Def::Assert(assert.expression.to_isla_exp(
                    solver,
                    self,
                    scfia_symbol_to_isla_sym_mapping,
                ));
                solver.add(isla_assert);
            }
            sym
        }
    }

    fn translate_to_isla_symbol(
        &self,
        symbol: &ScfiaSymbol,
        solver: &mut Solver<B64>,
        scfia_symbol_to_isla_sym_mapping: &mut HashMap<ScfiaSymbolId, Sym>,
    ) -> Sym {
        assert!(!scfia_symbol_to_isla_sym_mapping.contains_key(&symbol.id));
        let sym = match &symbol.variant {
            SymbolInnerVariant::Bitvector(width, _) => {
                let sym = solver.declare_const(Ty::BitVec(*width));
                scfia_symbol_to_isla_sym_mapping.insert(symbol.id, sym);
                sym
            }
            SymbolInnerVariant::Bool => {
                let sym = solver.declare_const(Ty::Bool);
                scfia_symbol_to_isla_sym_mapping.insert(symbol.id, sym);
                sym
            }
            SymbolInnerVariant::Defined(expression) => {
                let exp = expression.to_isla_exp(solver, self, scfia_symbol_to_isla_sym_mapping);
                let sym = if let Some(sym) = scfia_symbol_to_isla_sym_mapping.get(&symbol.id) {
                    *sym
                } else {
                    let sym = solver.define_const(exp);
                    scfia_symbol_to_isla_sym_mapping.insert(symbol.id, sym);
                    sym
                };
                sym
            }
        };
        sym
    }
}
