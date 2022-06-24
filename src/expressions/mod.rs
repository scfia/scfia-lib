use std::{rc::{Rc, Weak}, cell::RefCell, collections::{HashMap, BTreeMap}};

use crate::values::{ActiveValue, RetiredValue};

pub mod bool_eq_expression;
pub mod bool_less_than_uint_expression;
pub mod bool_neq_expression;
pub mod bool_not_expression;
pub mod bv_add_expression;
pub mod bv_and_expression;
pub mod bv_concat_expression;
pub mod bv_or_expression;
pub mod bv_sign_extend_expression;
pub mod bv_slice_expression;
pub mod bv_shift_right_logical_expression;
pub mod bv_shift_left_logical_expression;


pub(crate) fn inherit(
    id: u64,
    retired_expression: Rc<RefCell<RetiredValue>>,
    mut heirs: Vec<(u64, Rc<RefCell<ActiveValue>>)>,
    inherited_asts: &BTreeMap<u64, Rc<RefCell<RetiredValue>>>,
    discovered_asts: &HashMap<u64, Weak<RefCell<ActiveValue>>>,
) {
    // Heirs are parents and discovered symbols
    for (discovered_symbol_id, discovered_symbol) in discovered_asts {
        let discovered_symbol = discovered_symbol.upgrade().unwrap();
        let mut discovered_symbol_ref = discovered_symbol.try_borrow_mut().unwrap();
        discovered_symbol_ref.forget(id);
        heirs.push((*discovered_symbol_id, discovered_symbol.clone()))
    }

    // For each heir...
    for (heir_id, heir) in &heirs {
        let mut heir_ref = heir.try_borrow_mut().unwrap();

        // Inherit
        heir_ref.inherit(id, retired_expression.clone());

        // Pass on inherited symbols
        for (inherited_id, inherited) in inherited_asts {
            heir_ref.inherit(*inherited_id, inherited.clone())
        }

        // Acquaint all heirs
        for (other_heir_id, other_heir) in &heirs {
            if other_heir_id != heir_id {
                let mut other_heir_ref = other_heir.try_borrow_mut().unwrap();
                heir_ref.discover(other_heir_ref.get_id(), Rc::downgrade(other_heir));
                other_heir_ref.discover(heir_ref.get_id(), Rc::downgrade(heir));
            }
        }
    }
}
