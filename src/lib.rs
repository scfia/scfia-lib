// #![feature(trait_upcasting)]
#![allow(unused_imports)]

pub mod expressions;
pub mod traits;
pub mod values;
pub mod memory;
pub mod models;

use std::borrow::Borrow;
use std::cell::RefCell;
use std::collections::HashMap;
use std::fmt;
use std::ops::Deref;
use std::ops::DerefMut;
use std::rc::Rc;
use std::rc::Weak;
use std::time::SystemTime;
use std::time::UNIX_EPOCH;
use std::vec;

use expressions::bool_eq_expression::BoolEqExpression;
use expressions::bool_neq_expression::BoolNEqExpression;
use expressions::bool_not_expression::BoolNotExpression;
use models::riscv::rv32i::ForkSink;
use values::ActiveValue;
use values::RetiredValue;
use values::bit_vector_symbol::BitVectorSymbol;
use z3_sys::Z3_L_FALSE;
use z3_sys::Z3_L_TRUE;
use z3_sys::Z3_ast_vector_dec_ref;
use z3_sys::Z3_ast_vector_inc_ref;
use z3_sys::Z3_ast_vector_size;
use z3_sys::Z3_dec_ref;
use z3_sys::Z3_del_context;
use z3_sys::Z3_context;
use z3_sys::Z3_finalize_memory;
use z3_sys::Z3_inc_ref;
use z3_sys::Z3_mk_context_rc;
use z3_sys::Z3_del_config;
use z3_sys::Z3_mk_config;
use z3_sys::Z3_mk_eq;
use z3_sys::Z3_mk_model;
use z3_sys::Z3_mk_not;
use z3_sys::Z3_model_dec_ref;
use z3_sys::Z3_model_inc_ref;
use z3_sys::Z3_solver;
use z3_sys::Z3_mk_solver;
use z3_sys::Z3_solver_assert;
use z3_sys::Z3_solver_assert_and_track;
use z3_sys::Z3_solver_check;
use z3_sys::Z3_solver_check_assumptions;
use z3_sys::Z3_solver_get_assertions;
use z3_sys::Z3_solver_get_model;
use z3_sys::Z3_solver_get_unsat_core;
use z3_sys::Z3_solver_inc_ref;
use z3_sys::Z3_solver_pop;
use z3_sys::Z3_solver_push;
use z3_sys::Z3_solver_reset;

use crate::values::bit_vector_concrete::BitVectorConcrete;


#[derive(Debug)]
pub struct ScfiaStdlib {
    pub id: String,
    next_clone_id: u64,
    next_symbol_id: u64,
    pub retired_symbols_map: HashMap<u64, Rc<RefCell<RetiredValue>>>,
    pub z3_context: Z3_context,
    pub z3_solver: Z3_solver,
}

impl ScfiaStdlib {
    pub fn new(id: String) -> ScfiaStdlib {
        Self::new_with_next_id(id, 0)
    }

    pub fn new_with_next_id(id: String, next_symbol_id: u64) -> ScfiaStdlib {
        unsafe {
            let z3_config = Z3_mk_config();
            let z3_context = Z3_mk_context_rc(z3_config);
            let stdlib = ScfiaStdlib {
                id: id,
                next_clone_id: 1,
                z3_context,
                z3_solver: Z3_mk_solver(z3_context),
                next_symbol_id,
                retired_symbols_map: HashMap::new(),
            };
            Z3_solver_inc_ref(stdlib.z3_context, stdlib.z3_solver);
            Z3_del_config(z3_config);
            stdlib
        }
    }

    pub fn get_clone_id(&mut self) -> u64 {
        let clone_id = self.next_clone_id;
        self.next_clone_id += 1;
        clone_id
    }

    pub fn get_symbol_id(&mut self) -> u64 {
        let symbol_id = self.next_symbol_id;
        self.next_symbol_id += 1;
        symbol_id
    }

    pub fn assert_consistency(&mut self) {
        unsafe {
            assert_eq!(Z3_solver_check(self.z3_context, self.z3_solver), Z3_L_TRUE);
        }
    }

    pub fn do_condition(
        &mut self,
        expression: Rc<RefCell<ActiveValue>>,
        fork_sink: &mut Option<&mut ForkSink>,
    ) -> bool {
        unsafe {
            // debug_assert_eq!(Z3_solver_check(self.z3_context, self.z3_solver), Z3_L_TRUE);

            match expression.try_borrow().unwrap().deref() {
                ActiveValue::BoolConcrete(e1) => {
                    return e1.value
                }
                _ => {}
            }

            let mut can_be_true = false;
            let mut can_be_false = false;

            let condition_ast = {
                let condition_symbol = expression.try_borrow_mut().unwrap();
                condition_symbol.get_z3_ast()
            };
            let neg_condition_symbol = BoolNotExpression::new(expression.clone(), self, fork_sink);
            let neg_condition_ast = neg_condition_symbol.try_borrow().unwrap().get_z3_ast();

            if Z3_solver_check_assumptions(self.z3_context, self.z3_solver, 1, &condition_ast) != Z3_L_FALSE {
                can_be_true = true
            }

            if Z3_solver_check_assumptions(self.z3_context, self.z3_solver, 1, &neg_condition_ast) != Z3_L_FALSE {
                can_be_false = true
            }

            if can_be_true && can_be_false {
                // Create fork with negative condition
                // println!("FORK!");
                // println!("negcondition={:?}", &neg_condition_symbol);
                // println!("condition={:?}", &expression);
                if let Some(fork_sink) = fork_sink {
                    fork_sink.fork(neg_condition_symbol.into());
                    expression.try_borrow_mut().unwrap().assert(self);
                    true
                } else {
                    panic!()
                }
            } else if can_be_true {
                true
            } else if can_be_false {
                false
            } else {
                unreachable!("expression= {:?}, can_be_true={}, can_be_false={}", expression, can_be_true, can_be_false)
            }
        }
    }
}

impl Drop for ScfiaStdlib {
    fn drop(&mut self) {
        println!("dropping context {:x}", self.z3_context as u64);
        unsafe { Z3_del_context(self.z3_context) }
    }
}
