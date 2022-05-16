// #![feature(trait_upcasting)]
#![allow(unused_imports)]

pub mod expressions;
pub mod traits;
pub mod values;
pub mod memory;
pub mod models;

use std::cell::RefCell;
use std::ops::Deref;
use std::rc::Rc;
use std::rc::Weak;
use std::vec;

use expressions::bool_eq_expression::BoolEqExpression;
use expressions::bool_neq_expression::BoolNEqExpression;
use models::riscv::rv32i::ForkSink;
use traits::ast::Ast;
use traits::bit_vector_expression::BitVectorExpression;
use values::ActiveValue;
use values::bit_vector_symbol::BitVectorSymbol;
use z3_sys::Z3_L_FALSE;
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


#[derive(Debug)]
pub struct ScfiaStdlib {
    pub z3_context: Z3_context,
    z3_solver: Z3_solver,
    next_symbol_id: u64,
}

impl ScfiaStdlib {
    pub fn new() -> ScfiaStdlib {
        unsafe {
            let z3_config = Z3_mk_config();
            let z3_context = Z3_mk_context_rc(z3_config);
            let stdlib = ScfiaStdlib {
                z3_context,
                z3_solver: Z3_mk_solver(z3_context),
                next_symbol_id: 0,
            };
            Z3_solver_inc_ref(stdlib.z3_context, stdlib.z3_solver);
            Z3_del_config(z3_config);
            stdlib
        }
    }

    pub fn get_symbol_id(&mut self) -> u64 {
        let symbol_id = self.next_symbol_id;
        self.next_symbol_id += 1;
        symbol_id
    }

    
    pub fn do_neq(&mut self, left: Rc<RefCell<ActiveValue>>, right: Rc<RefCell<ActiveValue>>, fork_sink: Option<Rc<RefCell<ForkSink>>>) -> bool {
        unsafe {
            let mut can_be_true = false;
            let mut can_be_false = false;

            let condition_symbol = ActiveValue::BoolNEqExpression(BoolNEqExpression::new(left.clone(), right.clone(), self));
            let neg_condition_symbol = ActiveValue::BoolEqExpression(BoolEqExpression::new(left.clone(), right.clone(), self));

            let condition_ast = condition_symbol.get_z3_ast();
            if Z3_solver_check_assumptions(self.z3_context, self.z3_solver, 1, &condition_ast) != -1 {
                can_be_true = true
            }

            let neg_condition_ast = neg_condition_symbol.get_z3_ast();
            if Z3_solver_check_assumptions(self.z3_context, self.z3_solver, 1, &neg_condition_ast) != -1 {
                can_be_false = true
            }

            if can_be_true && can_be_false {
                // Create fork with negative condition
                fork_sink.unwrap().try_borrow_mut().unwrap().forks.push(neg_condition_symbol.into());

                // continue with positive condition
                // TODO: Add this assert to our data structures
                Z3_solver_assert(self.z3_context, self.z3_solver, condition_ast);

                true
            } else if can_be_true {
                true
            } else if can_be_false {
                false
            } else {
                unreachable!()
            }
        }
    }

    pub fn do_eq(&mut self, left: Rc<RefCell<ActiveValue>>, right: Rc<RefCell<ActiveValue>>, fork_sink: Option<Rc<RefCell<ForkSink>>>) -> bool {
        unsafe {
            let mut can_be_true = false;
            let mut can_be_false = false;

            let condition_symbol = ActiveValue::BoolEqExpression(BoolEqExpression::new(left.clone(), right.clone(), self));
            let condition_ast = condition_symbol.get_z3_ast();
            let neg_condition_symbol = ActiveValue::BoolNEqExpression(BoolNEqExpression::new(left, right, self));
            let neg_condition_ast = neg_condition_symbol.get_z3_ast();

            if Z3_solver_check_assumptions(self.z3_context, self.z3_solver, 1, &condition_ast) != Z3_L_FALSE {
                can_be_true = true
            }

            if Z3_solver_check_assumptions(self.z3_context, self.z3_solver, 1, &neg_condition_ast) != Z3_L_FALSE {
                can_be_false = true
            }

            if can_be_true && can_be_false {
                // Create fork with negative condition
                fork_sink.unwrap().try_borrow_mut().unwrap().forks.push(neg_condition_symbol.into());

                // continue with positive condition
                // TODO: Add this assert to our data structures
                Z3_solver_assert(self.z3_context, self.z3_solver, condition_ast);

                true
            } else if can_be_true {
                true
            } else if can_be_false {
                false
            } else {
                unreachable!()
            }
        }
    }
}

impl Drop for ScfiaStdlib {
    fn drop(&mut self) {
        unsafe { Z3_del_context(self.z3_context) }
    }
}
