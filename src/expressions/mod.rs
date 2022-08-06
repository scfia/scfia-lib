use std::{rc::{Rc, Weak}, cell::RefCell, collections::{BTreeMap}};

use crate::{values::{ActiveValue, RetiredValue}, ScfiaStdlib};

pub mod bool_eq_expression;
pub mod bool_less_than_signed_expression;
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
pub mod bv_sub_expression;
pub mod bv_xor_expression;
pub mod bv_multiply_expression;
pub mod bv_unsigned_remainder_expression;

pub const MAX_DEPTH: u64 = 1000000;


pub(crate) fn inherit(
    id: u64,
    retired_expression: Rc<RefCell<RetiredValue>>,
    mut heirs: Vec<(u64, Rc<RefCell<ActiveValue>>)>,
    inherited_asts: &BTreeMap<u64, Rc<RefCell<RetiredValue>>>,
    discovered_asts: &BTreeMap<u64, Weak<RefCell<ActiveValue>>>,
) {
    // Heirs are parents and discovered symbols
    for (discovered_symbol_id, discovered_symbol) in discovered_asts {
        let discovered_symbol = discovered_symbol.upgrade().unwrap();
        let mut discovered_symbol_ref = discovered_symbol.try_borrow_mut().unwrap();
        // println!("adding acquaintance {} ({:?}) to heir list of {}", discovered_symbol_id, discovered_symbol_ref, id);
        discovered_symbol_ref.forget(id);
        heirs.push((*discovered_symbol_id, discovered_symbol.clone()))
    }

    // For each heir...
    for (heir_id, heir) in &heirs {
        let mut heir_ref = heir.try_borrow_mut().unwrap();
        debug_assert_eq!(*heir_id, heir_ref.get_id());

        // Inherit if parent is not concrete
        match *heir_ref {
            ActiveValue::BitvectorConcrete(_) => {},
            ActiveValue::BoolConcrete(_) => {},
            _ => {heir_ref.inherit(id, retired_expression.clone())}
        }

        // Pass on inherited symbols
        for (inherited_id, inherited) in inherited_asts {
            // println!("passing on {} to {}", inherited_id, &heir_ref.get_id());
            match *heir_ref {
                ActiveValue::BitvectorConcrete(_) => {},
                ActiveValue::BoolConcrete(_) => {},
                _ => { heir_ref.inherit(*inherited_id, inherited.clone()); }
            }
        }

        // Acquaint all heirs
        for (other_heir_id, other_heir) in &heirs {
            if other_heir_id != heir_id {
                let mut other_heir_ref = other_heir.try_borrow_mut().unwrap();
                debug_assert_eq!(*other_heir_id, other_heir_ref.get_id());
                heir_ref.discover(other_heir_ref.get_id(), Rc::downgrade(other_heir));
                other_heir_ref.discover(heir_ref.get_id(), Rc::downgrade(heir));
            }
        }
    }
}

pub(crate) fn finish_clone(
    id: u64,
    inherited_asts: &BTreeMap<u64, Rc<RefCell<RetiredValue>>>,
    discovered_asts: &BTreeMap<u64, Weak<RefCell<ActiveValue>>>,
    clone: Rc<RefCell<ActiveValue>>,
    cloned_active_values: &mut BTreeMap<u64, Rc<RefCell<ActiveValue>>>,
    cloned_retired_values: &mut BTreeMap<u64, Rc<RefCell<RetiredValue>>>,
    cloned_stdlib: &mut ScfiaStdlib
) -> Rc<RefCell<ActiveValue>> {
    if let Some(undesirable) = cloned_active_values.insert(id, clone.clone()) {
        panic!("{:?}", undesirable)
    }
    assert_eq!(id, clone.try_borrow().unwrap().get_id());

    // Clone inherited values
    let mut cloned_inherited = vec![];
    for (inherited_ast_id, inherited_ast) in inherited_asts {
        cloned_inherited.push((inherited_ast_id, inherited_ast.try_borrow().unwrap().clone_to_stdlib(cloned_active_values, cloned_retired_values, cloned_stdlib)));
    }
    {
        debug_assert_eq!(inherited_asts.len(), cloned_inherited.len());
        let mut cloned_expression_ref = clone.try_borrow_mut().unwrap();
        for (cloned_inherited_ast_id, cloned_inherited_ast) in cloned_inherited {
            cloned_expression_ref.inherit(*cloned_inherited_ast_id, cloned_inherited_ast)
        }
    }

    // Clone discovered values
    let mut cloned_discovered = vec![];
    for discovered_ast in discovered_asts.values() {
        cloned_discovered.push(discovered_ast.upgrade().unwrap().try_borrow().unwrap().clone_to_stdlib(cloned_active_values, cloned_retired_values, cloned_stdlib));
    }

    // Update clone with retirees and discoveries
    {
        assert_eq!(discovered_asts.len(), cloned_discovered.len());
        let mut cloned_expression_ref = clone.try_borrow_mut().unwrap();
        for cloned_discovered_value in cloned_discovered {
            cloned_expression_ref.discover(cloned_discovered_value.try_borrow().unwrap().get_id(), Rc::downgrade(&cloned_discovered_value))
        }        
    }

    clone
}
