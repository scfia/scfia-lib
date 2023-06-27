use std::{
    cell::{Cell, OnceCell},
    ptr,
    rc::{Rc, Weak},
    time::Instant,
};

use log::{debug, error, info, warn};
use z3_sys::{
    Z3_ast, Z3_context, Z3_dec_ref, Z3_del_config, Z3_del_context, Z3_get_numeral_uint64, Z3_inc_ref, Z3_lbool, Z3_mk_bv_sort, Z3_mk_bvadd, Z3_mk_bvand,
    Z3_mk_bvlshr, Z3_mk_bvmul, Z3_mk_bvor, Z3_mk_bvshl, Z3_mk_bvslt, Z3_mk_bvsub, Z3_mk_bvuge, Z3_mk_bvult, Z3_mk_bvurem, Z3_mk_bvxor, Z3_mk_concat,
    Z3_mk_config, Z3_mk_context_rc, Z3_mk_eq, Z3_mk_extract, Z3_mk_false, Z3_mk_fresh_const, Z3_mk_not, Z3_mk_or, Z3_mk_sign_ext, Z3_mk_solver, Z3_mk_true,
    Z3_mk_unsigned_int64, Z3_model_eval, Z3_solver, Z3_solver_assert, Z3_solver_check, Z3_solver_check_assumptions, Z3_solver_get_model, Z3_solver_inc_ref,
    Z3_string, Z3_L_FALSE, Z3_L_TRUE,
};

use crate::{
    scfia::Scfia,
    values::active_value::{ActiveValue},
    GenericForkSink, ScfiaComposition,
};

pub const PREFIX: [i8; 4] = ['p' as i8, 'r' as i8, 'e' as i8, 0];

#[derive(Debug)]
pub struct Z3Handle<SC: ScfiaComposition> {
    pub context: Z3_context,
    pub solver: Z3_solver,
    pub ast_refs: Cell<i64>,
    pub selff: OnceCell<Weak<Self>>,
}

#[derive(Debug)]
pub struct Z3Ast<SC: ScfiaComposition> {
    pub ast: Z3_ast,
    pub z3: Weak<Z3Handle<SC>>,
}

impl<SC: ScfiaComposition> Z3Handle<SC> {
    pub fn new() -> Rc<Self> {
        unsafe {
            let config = Z3_mk_config();
            let context = Z3_mk_context_rc(config);
            let solver = Z3_mk_solver(context);
            Z3_solver_inc_ref(context, solver);
            Z3_del_config(config);
            let selff = Rc::new(Z3Handle {
                context,
                solver,
                ast_refs: Cell::new(0),
                selff: OnceCell::new(),
            });
            selff.selff.set(Rc::downgrade(&selff)).unwrap();
            selff
        }
    }

    pub fn assert_consistency(&self) {
        unsafe {
            assert_eq!(Z3_solver_check(self.context, self.solver), Z3_L_TRUE);
        }
    }

    pub fn new_bool_concrete(&self, value: bool) -> Z3Ast<SC> {
        unsafe {
            let ast = if value { Z3_mk_true(self.context) } else { Z3_mk_false(self.context) };
            Z3_inc_ref(self.context, ast);
            self.ast_refs.set(self.ast_refs.get() + 1);
            Z3Ast {
                ast,
                z3: self.selff.get().unwrap().clone(),
            }
        }
    }

    pub fn new_eq(&self, s1: &Z3Ast<SC>, s2: &Z3Ast<SC>, is_assert: bool) -> Z3Ast<SC> {
        unsafe {
            let ast = Z3_mk_eq(self.context, s1.ast, s2.ast);
            Z3_inc_ref(self.context, ast);
            self.ast_refs.set(self.ast_refs.get() + 1);
            if is_assert {
                Z3_solver_assert(self.context, self.solver, ast);
            }
            Z3Ast {
                ast,
                z3: self.selff.get().unwrap().clone(),
            }
        }
    }

    pub fn new_not(&self, s1: &Z3Ast<SC>, is_assert: bool) -> Z3Ast<SC> {
        unsafe {
            let ast = Z3_mk_not(self.context, s1.ast);
            Z3_inc_ref(self.context, ast);
            self.ast_refs.set(self.ast_refs.get() + 1);
            if is_assert {
                Z3_solver_assert(self.context, self.solver, ast);
            }
            Z3Ast {
                ast,
                z3: self.selff.get().unwrap().clone(),
            }
        }
    }

    pub fn new_bvslt(&self, s1: &Z3Ast<SC>, s2: &Z3Ast<SC>, is_assert: bool) -> Z3Ast<SC> {
        unsafe {
            let ast = Z3_mk_bvslt(self.context, s1.ast, s2.ast);
            Z3_inc_ref(self.context, ast);
            self.ast_refs.set(self.ast_refs.get() + 1);
            if is_assert {
                Z3_solver_assert(self.context, self.solver, ast);
            }
            Z3Ast {
                ast,
                z3: self.selff.get().unwrap().clone(),
            }
        }
    }

    pub fn new_bvult(&self, s1: &Z3Ast<SC>, s2: &Z3Ast<SC>, is_assert: bool) -> Z3Ast<SC> {
        unsafe {
            let ast = Z3_mk_bvult(self.context, s1.ast, s2.ast);
            Z3_inc_ref(self.context, ast);
            self.ast_refs.set(self.ast_refs.get() + 1);
            if is_assert {
                Z3_solver_assert(self.context, self.solver, ast);
            }
            Z3Ast {
                ast,
                z3: self.selff.get().unwrap().clone(),
            }
        }
    }

    pub fn new_bvuge(&self, s1: &Z3Ast<SC>, s2: &Z3Ast<SC>, is_assert: bool) -> Z3Ast<SC> {
        unsafe {
            let ast = Z3_mk_bvuge(self.context, s1.ast, s2.ast);
            Z3_inc_ref(self.context, ast);
            self.ast_refs.set(self.ast_refs.get() + 1);
            if is_assert {
                Z3_solver_assert(self.context, self.solver, ast);
            }
            Z3Ast {
                ast,
                z3: self.selff.get().unwrap().clone(),
            }
        }
    }

    pub fn new_bvadd(&self, s1: &Z3Ast<SC>, s2: &Z3Ast<SC>) -> Z3Ast<SC> {
        unsafe {
            let ast = Z3_mk_bvadd(self.context, s1.ast, s2.ast);
            Z3_inc_ref(self.context, ast);
            self.ast_refs.set(self.ast_refs.get() + 1);
            Z3Ast {
                ast,
                z3: self.selff.get().unwrap().clone(),
            }
        }
    }

    pub fn new_bvand(&self, s1: &Z3Ast<SC>, s2: &Z3Ast<SC>) -> Z3Ast<SC> {
        unsafe {
            let ast = Z3_mk_bvand(self.context, s1.ast, s2.ast);
            Z3_inc_ref(self.context, ast);
            self.ast_refs.set(self.ast_refs.get() + 1);
            Z3Ast {
                ast,
                z3: self.selff.get().unwrap().clone(),
            }
        }
    }

    pub fn new_bvconcat(&self, s1: &Z3Ast<SC>, s2: &Z3Ast<SC>) -> Z3Ast<SC> {
        unsafe {
            let ast = Z3_mk_concat(self.context, s1.ast, s2.ast);
            Z3_inc_ref(self.context, ast);
            self.ast_refs.set(self.ast_refs.get() + 1);
            Z3Ast {
                ast,
                z3: self.selff.get().unwrap().clone(),
            }
        }
    }

    pub fn new_bv_concrete(&self, value: u64, width: u32) -> Z3Ast<SC> {
        unsafe {
            let sort = Z3_mk_bv_sort(self.context, width);
            let ast = Z3_mk_unsigned_int64(self.context, value, sort);
            Z3_inc_ref(self.context, ast);
            self.ast_refs.set(self.ast_refs.get() + 1);
            Z3Ast {
                ast,
                z3: self.selff.get().unwrap().clone(),
            }
        }
    }

    pub fn new_bvmul(&self, s1: &Z3Ast<SC>, s2: &Z3Ast<SC>) -> Z3Ast<SC> {
        unsafe {
            let ast = Z3_mk_bvmul(self.context, s1.ast, s2.ast);
            Z3_inc_ref(self.context, ast);
            self.ast_refs.set(self.ast_refs.get() + 1);
            Z3Ast {
                ast,
                z3: self.selff.get().unwrap().clone(),
            }
        }
    }

    pub fn new_bvor(&self, s1: &Z3Ast<SC>, s2: &Z3Ast<SC>) -> Z3Ast<SC> {
        unsafe {
            let ast = Z3_mk_bvor(self.context, s1.ast, s2.ast);
            Z3_inc_ref(self.context, ast);
            self.ast_refs.set(self.ast_refs.get() + 1);
            Z3Ast {
                ast,
                z3: self.selff.get().unwrap().clone(),
            }
        }
    }

    pub fn new_sign_ext(&self, extension_width: u32, s1: &Z3Ast<SC>) -> Z3Ast<SC> {
        unsafe {
            let ast = Z3_mk_sign_ext(self.context, extension_width, s1.ast);
            Z3_inc_ref(self.context, ast);
            self.ast_refs.set(self.ast_refs.get() + 1);
            Z3Ast {
                ast,
                z3: self.selff.get().unwrap().clone(),
            }
        }
    }

    pub fn new_extract(&self, high: u32, low: u32, s1: &Z3Ast<SC>) -> Z3Ast<SC> {
        unsafe {
            let ast = Z3_mk_extract(self.context, high, low, s1.ast);
            Z3_inc_ref(self.context, ast);
            self.ast_refs.set(self.ast_refs.get() + 1);
            Z3Ast {
                ast,
                z3: self.selff.get().unwrap().clone(),
            }
        }
    }

    pub fn new_bvshl(&self, s1: &Z3Ast<SC>, s2: &Z3Ast<SC>) -> Z3Ast<SC> {
        unsafe {
            let ast = Z3_mk_bvshl(self.context, s1.ast, s2.ast);
            Z3_inc_ref(self.context, ast);
            self.ast_refs.set(self.ast_refs.get() + 1);
            Z3Ast {
                ast,
                z3: self.selff.get().unwrap().clone(),
            }
        }
    }

    pub fn new_bvlshr(&self, s1: &Z3Ast<SC>, s2: &Z3Ast<SC>) -> Z3Ast<SC> {
        unsafe {
            let ast = Z3_mk_bvlshr(self.context, s1.ast, s2.ast);
            Z3_inc_ref(self.context, ast);
            self.ast_refs.set(self.ast_refs.get() + 1);
            Z3Ast {
                ast,
                z3: self.selff.get().unwrap().clone(),
            }
        }
    }

    pub fn new_bvsub(&self, s1: &Z3Ast<SC>, s2: &Z3Ast<SC>) -> Z3Ast<SC> {
        unsafe {
            let ast = Z3_mk_bvsub(self.context, s1.ast, s2.ast);
            Z3_inc_ref(self.context, ast);
            self.ast_refs.set(self.ast_refs.get() + 1);
            Z3Ast {
                ast,
                z3: self.selff.get().unwrap().clone(),
            }
        }
    }

    pub fn new_fresh_const(&self, width: u32) -> Z3Ast<SC> {
        unsafe {
            let ast = Z3_mk_fresh_const(self.context, 0 as Z3_string, Z3_mk_bv_sort(self.context, width));
            Z3_inc_ref(self.context, ast);
            self.ast_refs.set(self.ast_refs.get() + 1);
            Z3Ast {
                ast,
                z3: self.selff.get().unwrap().clone(),
            }
        }
    }

    pub fn new_bvurem(&self, s1: &Z3Ast<SC>, s2: &Z3Ast<SC>) -> Z3Ast<SC> {
        unsafe {
            let ast = Z3_mk_bvurem(self.context, s1.ast, s2.ast);
            Z3_inc_ref(self.context, ast);
            self.ast_refs.set(self.ast_refs.get() + 1);
            Z3Ast {
                ast,
                z3: self.selff.get().unwrap().clone(),
            }
        }
    }

    pub fn new_bvxor(&self, s1: &Z3Ast<SC>, s2: &Z3Ast<SC>) -> Z3Ast<SC> {
        unsafe {
            let ast = Z3_mk_bvxor(self.context, s1.ast, s2.ast);
            Z3_inc_ref(self.context, ast);
            self.ast_refs.set(self.ast_refs.get() + 1);
            Z3Ast {
                ast,
                z3: self.selff.get().unwrap().clone(),
            }
        }
    }

    pub fn new_or(&self, s1: &Z3Ast<SC>, s2: &Z3Ast<SC>) -> Z3Ast<SC> {
        unsafe {
            let ast = Z3_mk_or(self.context, 2, [s1.ast, s2.ast].as_ptr());
            Z3_inc_ref(self.context, ast);
            self.ast_refs.set(self.ast_refs.get() + 1);
            Z3Ast {
                ast,
                z3: self.selff.get().unwrap().clone(),
            }
        }
    }

    pub fn check_assumptions(&self, assumptions: &[&Z3Ast<SC>]) -> Z3_lbool {
        unsafe {
            let mut assumptions_asts = Vec::with_capacity(assumptions.len());
            for assumption in assumptions {
                assumptions_asts.push(assumption.ast)
            }
            Z3_solver_check_assumptions(self.context, self.solver, assumptions_asts.len().try_into().unwrap(), assumptions_asts.as_ptr())
        }
    }

    pub fn check_condition(&self, scfia: &Scfia<SC>, condition: &ActiveValue<SC>, fork_sink: &mut Option<SC::ForkSink>) -> bool {
        unsafe {
            match condition {
                ActiveValue::BoolConcrete(e) => *e,
                ActiveValue::BVConcrete(_, _) => panic!("condition was bitvector"),
                ActiveValue::Expression(expression) => {
                    let mut can_be_true = false;
                    let mut can_be_false = false;

                    let condition_ast = condition.get_z3_ast().ast;
                    debug!("building neg_condition_symbol");
                    let neg_condition_symbol = scfia.new_bool_not(condition, None, false, fork_sink, None);
                    let neg_condition_ast = neg_condition_symbol.get_z3_ast().ast;

                    if Z3_solver_check_assumptions(self.context, self.solver, 1, &condition_ast) != Z3_L_FALSE {
                        can_be_true = true
                    }

                    if Z3_solver_check_assumptions(self.context, self.solver, 1, &neg_condition_ast) != Z3_L_FALSE {
                        can_be_false = true
                    }

                    if can_be_true && can_be_false {
                        if let Some(fork_sink) = fork_sink {
                            info!("Forking over {:?}", expression);
                            fork_sink.fork(neg_condition_symbol);
                            debug!("asserting condition in current branch");
                            condition.assert(scfia);
                            true
                        } else {
                            error!("unexpected fork");
                            panic!("unexpected fork")
                        }
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
    }

    pub fn monomorphize(&self, value: &Z3Ast<SC>, candidates: &mut Vec<u64>) {
        unsafe {
            let begin = Instant::now();
            debug!("monomorphize");

            // Fill assumptions with known candidates
            let mut assumptions = Vec::with_capacity(candidates.len());
            let mut assumptions_asts = Vec::with_capacity(candidates.len());
            for candidate in candidates.iter() {
                let candidate_ast = self.new_bv_concrete(*candidate, 32); // TODO don't hardcode width
                let eq = self.new_eq(&candidate_ast, value, false); // TODO this is unsafe - we need to ensure these are not freed
                let assumption = self.new_not(&eq, false);
                assumptions_asts.push(assumption.ast);
                assumptions.push(assumption);
            }

            // Find all remaining candidates
            loop {
                let assumptions_count = assumptions.len().try_into().unwrap();
                if Z3_solver_check_assumptions(self.context, self.solver, assumptions_count, assumptions_asts.as_ptr()) == Z3_L_FALSE {
                    break;
                }

                // Get unpredicted new candidate
                let model = Z3_solver_get_model(self.context, self.solver);
                let mut z3_ast_result: Z3_ast = ptr::null_mut();
                assert!(Z3_model_eval(self.context, model, value.ast, true, &mut z3_ast_result));
                Z3_inc_ref(self.context, z3_ast_result);
                let mut candidate: u64 = 0;
                assert!(Z3_get_numeral_uint64(self.context, z3_ast_result, &mut candidate));
                Z3_dec_ref(self.context, z3_ast_result);

                warn!("Unpredicted monomorphization candidate 0x{:x} ", candidate);
                candidates.push(candidate);

                let candidate_ast = self.new_bv_concrete(candidate, 32); // TODO don't hardcode width
                let eq = self.new_eq(&candidate_ast, value, false); // TODO this is unsafe - we need to ensure these are not freed
                let assumption = self.new_not(&eq, false);
                assumptions_asts.push(assumption.ast);
                assumptions.push(assumption);
            }

            debug!("monomorphize_active done after {} ms", begin.elapsed().as_millis());
        }
    }
}

impl<SC: ScfiaComposition> Z3Ast<SC> {
    pub fn assert(&self) {
        unsafe {
            let z3 = self.z3.upgrade().unwrap();
            Z3_solver_assert(z3.context, z3.solver, self.ast);
        }
    }
}

impl<SC: ScfiaComposition> Drop for Z3Handle<SC> {
    fn drop(&mut self) {
        unsafe { Z3_del_context(self.context) }
    }
}

impl<SC: ScfiaComposition> Drop for Z3Ast<SC> {
    fn drop(&mut self) {
        unsafe {
            if let Some(z3) = self.z3.upgrade() {
                Z3_dec_ref(z3.context, self.ast);
                z3.ast_refs.set(z3.ast_refs.get() - 1);
                assert!(z3.ast_refs.get() >= 0);
            }
        }
    }
}

impl<SC: ScfiaComposition> Clone for Z3Ast<SC> {
    fn clone(&self) -> Self {
        unsafe { Z3_inc_ref(self.z3.upgrade().unwrap().context, self.ast) }
        let z3 = self.z3.upgrade().unwrap();
        z3.ast_refs.set(z3.ast_refs.get() + 1);
        Self {
            ast: self.ast,
            z3: self.z3.clone(),
        }
    }
}

/*TODO
#[cfg(test)]
mod tests {
    use std::rc::Rc;

    use crate::models::riscv::rv32i::RV32iScfiaComposition;

    use super::Z3Handle;

    #[test]
    pub fn test() {
        let z3: Rc<Z3Handle<RV32iScfiaComposition>> = Z3Handle::new();
        {
            let b1 = z3.new_bool_concrete(true);
            assert_eq!(z3.ast_refs.get(), 1);
            {
                let b2 = b1.clone();
                assert!(Rc::ptr_eq(&b2.z3.upgrade().unwrap(), &b1.z3.upgrade().unwrap()));
                assert_eq!(z3.ast_refs.get(), 2);
            }
            assert_eq!(z3.ast_refs.get(), 1);
        }
        assert_eq!(z3.ast_refs.get(), 0)
    }
}
*/
