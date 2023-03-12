// #![feature(trait_upcasting)]
#![allow(unused_imports)]

pub mod expressions;
pub mod memory;
pub mod models;
pub mod traits;
pub mod values;

use std::borrow::Borrow;
use std::cell::RefCell;
use std::collections::BTreeSet;
use std::collections::HashMap;
use std::fmt;
use std::ops::Deref;
use std::ops::DerefMut;
use std::ptr;
use std::rc::Rc;
use std::rc::Weak;
use std::time::SystemTime;
use std::time::UNIX_EPOCH;
use std::vec;

use expressions::bool_eq_expression::BoolEqExpression;
use expressions::bool_neq_expression::BoolNEqExpression;
use expressions::bool_not_expression::BoolNotExpression;
use models::riscv::rv32i::ForkSink;
use values::bit_vector_symbol::BitVectorSymbol;
use values::ActiveValue;
use values::RetiredValue;
use z3_sys::AstKind;
use z3_sys::SortKind;
use z3_sys::Z3_ast;
use z3_sys::Z3_ast_vector_dec_ref;
use z3_sys::Z3_ast_vector_inc_ref;
use z3_sys::Z3_ast_vector_size;
use z3_sys::Z3_context;
use z3_sys::Z3_dec_ref;
use z3_sys::Z3_del_config;
use z3_sys::Z3_del_context;
use z3_sys::Z3_finalize_memory;
use z3_sys::Z3_get_app_decl;
use z3_sys::Z3_get_ast_kind;
use z3_sys::Z3_get_bv_sort_size;
use z3_sys::Z3_get_numeral_uint64;
use z3_sys::Z3_get_sort;
use z3_sys::Z3_get_sort_kind;
use z3_sys::Z3_inc_ref;
use z3_sys::Z3_mk_config;
use z3_sys::Z3_mk_context_rc;
use z3_sys::Z3_mk_eq;
use z3_sys::Z3_mk_model;
use z3_sys::Z3_mk_not;
use z3_sys::Z3_mk_solver;
use z3_sys::Z3_mk_unsigned_int64;
use z3_sys::Z3_model_dec_ref;
use z3_sys::Z3_model_eval;
use z3_sys::Z3_model_inc_ref;
use z3_sys::Z3_solver;
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
use z3_sys::Z3_L_FALSE;
use z3_sys::Z3_L_TRUE;

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
                ActiveValue::BoolConcrete(e1) => return e1.value,
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

            if Z3_solver_check_assumptions(self.z3_context, self.z3_solver, 1, &condition_ast)
                != Z3_L_FALSE
            {
                can_be_true = true
            }

            if Z3_solver_check_assumptions(self.z3_context, self.z3_solver, 1, &neg_condition_ast)
                != Z3_L_FALSE
            {
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
                    panic!("unexpected fork")
                }
            } else if can_be_true {
                true
            } else if can_be_false {
                false
            } else {
                unreachable!(
                    "expression= {:?}, can_be_true={}, can_be_false={}",
                    expression, can_be_true, can_be_false
                )
            }
        }
    }

    pub fn are_equal(&mut self, lhs: Z3_ast, rhs: Z3_ast) -> bool {
        unsafe {
            let assumptions = vec![Z3_mk_not(
                self.z3_context,
                Z3_mk_eq(self.z3_context, lhs, rhs),
            )];
            if Z3_solver_check_assumptions(self.z3_context, self.z3_solver, 1, assumptions.as_ptr())
                == Z3_L_FALSE
            {
                true
            } else {
                false
            }
        }
    }

    pub fn monomorphize(&mut self, ast: Z3_ast, candidates: &mut Vec<u64>) {
        unsafe {
            let mut assumptions: Vec<Z3_ast> = vec![];
            let sort = Z3_get_sort(self.z3_context, ast);
            let sort_kind = Z3_get_sort_kind(self.z3_context, sort);
            assert_eq!(SortKind::BV, sort_kind);

            // Fill assumptions with known candidates
            for candidate in candidates.iter() {
                let assumption = Z3_mk_not(
                    self.z3_context,
                    Z3_mk_eq(
                        self.z3_context,
                        Z3_mk_unsigned_int64(self.z3_context, *candidate, sort),
                        ast,
                    ),
                );
                Z3_inc_ref(self.z3_context, assumption);
                assumptions.push(assumption)
            }

            // Find all remaining candidates
            loop {
                let assumptions_count = assumptions.len().try_into().unwrap();
                if Z3_solver_check_assumptions(
                    self.z3_context,
                    self.z3_solver,
                    assumptions_count,
                    assumptions.as_ptr(),
                ) == Z3_L_FALSE
                {
                    break;
                }

                let model = Z3_solver_get_model(self.z3_context, self.z3_solver);

                let mut z3_ast: Z3_ast = ptr::null_mut();
                assert!(Z3_model_eval(
                    self.z3_context,
                    model,
                    ast,
                    true,
                    &mut z3_ast
                ));

                let ast_kind = Z3_get_ast_kind(self.z3_context, z3_ast);
                let mut v: u64 = 0;
                if ast_kind == AstKind::Numeral {
                    let size =
                        Z3_get_bv_sort_size(self.z3_context, Z3_get_sort(self.z3_context, z3_ast));
                    assert_eq!(32, size);
                    assert!(Z3_get_numeral_uint64(self.z3_context, z3_ast, &mut v));
                } else {
                    panic!("{:?}", ast_kind)
                }

                println!("WARNING: Unpredicted monomorphization candidate 0x{:x} ({} assumptions, {} candidates)", v, assumptions.len(), candidates.len());
                candidates.push(v);

                let assumption = Z3_mk_not(
                    self.z3_context,
                    Z3_mk_eq(
                        self.z3_context,
                        Z3_mk_unsigned_int64(self.z3_context, v, sort),
                        ast,
                    ),
                );
                Z3_inc_ref(self.z3_context, assumption);
                assumptions.push(assumption)
            }

            for assumption in assumptions {
                Z3_dec_ref(self.z3_context, assumption);
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

pub struct SymbolicHints {
    pub hints: Vec<Vec<u64>>,
}
