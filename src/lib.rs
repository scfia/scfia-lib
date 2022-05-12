// #![feature(trait_upcasting)]
#![allow(unused_imports)]

pub mod expressions;
pub mod traits;
pub mod values;
pub mod models;

use std::cell::RefCell;
use std::rc::Rc;
use std::rc::Weak;

use crate::traits::symbol::Symbol;

use traits::bit_vector_expression::BitVectorExpression;
use traits::ast::ActiveAst;
use values::bit_vector_symbol::BitVectorSymbol;
use values::retired_bitvector_symbol::RetiredBitvectorSymbol;
use z3_sys::Z3_del_context;
use z3_sys::Z3_context;
use z3_sys::Z3_mk_context_rc;
use z3_sys::Z3_del_config;
use z3_sys::Z3_mk_config;
use z3_sys::Z3_solver;
use z3_sys::Z3_mk_solver;


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
            Z3_del_config(z3_config);
            stdlib
        }
    }

    pub fn get_symbol_id(&mut self) -> u64 {
        let symbol_id = self.next_symbol_id;
        self.next_symbol_id += 1;
        symbol_id
    }
}

impl Drop for ScfiaStdlib {
    fn drop(&mut self) {
        unsafe { Z3_del_context(self.z3_context) }
    }
}

#[cfg(test)]
mod tests {
    use crate::ScfiaStdlib;

    #[test]
    fn mk_context() {
        ScfiaStdlib::new();
    }
}
