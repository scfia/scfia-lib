use std::{
    cell::OnceCell,
    rc::{Rc, Weak},
};

use z3_sys::{
    Z3_ast, Z3_context, Z3_dec_ref, Z3_del_config, Z3_del_context, Z3_inc_ref, Z3_mk_bv_sort, Z3_mk_bvadd, Z3_mk_bvand, Z3_mk_bvlshr, Z3_mk_bvmul, Z3_mk_bvor,
    Z3_mk_bvshl, Z3_mk_bvslt, Z3_mk_bvsub, Z3_mk_bvult, Z3_mk_bvurem, Z3_mk_concat, Z3_mk_config, Z3_mk_context_rc, Z3_mk_eq, Z3_mk_extract, Z3_mk_false,
    Z3_mk_not, Z3_mk_sign_ext, Z3_mk_solver, Z3_mk_true, Z3_mk_unsigned_int64, Z3_solver, Z3_solver_assert, Z3_solver_check, Z3_solver_inc_ref, Z3_L_TRUE, Z3_mk_bvxor, Z3_mk_fresh_const,
};

pub const PREFIX: [i8; 4] = ['p' as i8, 'r' as i8, 'e' as i8, 0];

#[derive(Debug)]
pub struct Z3Handle {
    pub context: Z3_context,
    pub solver: Z3_solver,
    pub selff: OnceCell<Weak<Self>>,
}

#[derive(Debug)]
// TODO clone or don't drop for actives
pub struct Z3Ast {
    pub ast: Z3_ast,
    pub z3: Weak<Z3Handle>,
}

impl Z3Handle {
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

    pub fn new_bool_concrete(&self, value: bool) -> Z3Ast {
        unsafe {
            let ast = if value { Z3_mk_true(self.context) } else { Z3_mk_false(self.context) };
            Z3_inc_ref(self.context, ast);
            Z3Ast {
                ast,
                z3: self.selff.get().unwrap().clone(),
            }
        }
    }

    pub fn new_eq(&self, s1: &Z3Ast, s2: &Z3Ast, is_assert: bool) -> Z3Ast {
        unsafe {
            let ast = Z3_mk_eq(self.context, s1.ast, s2.ast);
            Z3_inc_ref(self.context, ast);
            if is_assert {
                Z3_solver_assert(self.context, self.solver, ast);
            }
            Z3Ast {
                ast,
                z3: self.selff.get().unwrap().clone(),
            }
        }
    }

    pub fn new_not(&self, s1: &Z3Ast, is_assert: bool) -> Z3Ast {
        unsafe {
            let ast = Z3_mk_not(self.context, s1.ast);
            Z3_inc_ref(self.context, ast);
            if is_assert {
                Z3_solver_assert(self.context, self.solver, ast);
            }
            Z3Ast {
                ast,
                z3: self.selff.get().unwrap().clone(),
            }
        }
    }

    pub fn new_bvslt(&self, s1: &Z3Ast, s2: &Z3Ast, is_assert: bool) -> Z3Ast {
        unsafe {
            let ast = Z3_mk_bvslt(self.context, s1.ast, s2.ast);
            Z3_inc_ref(self.context, ast);
            if is_assert {
                Z3_solver_assert(self.context, self.solver, ast);
            }
            Z3Ast {
                ast,
                z3: self.selff.get().unwrap().clone(),
            }
        }
    }

    pub fn new_bvult(&self, s1: &Z3Ast, s2: &Z3Ast, is_assert: bool) -> Z3Ast {
        unsafe {
            let ast = Z3_mk_bvult(self.context, s1.ast, s2.ast);
            Z3_inc_ref(self.context, ast);
            if is_assert {
                Z3_solver_assert(self.context, self.solver, ast);
            }
            Z3Ast {
                ast,
                z3: self.selff.get().unwrap().clone(),
            }
        }
    }

    pub fn new_bvadd(&self, s1: &Z3Ast, s2: &Z3Ast) -> Z3Ast {
        unsafe {
            let ast = Z3_mk_bvadd(self.context, s1.ast, s2.ast);
            Z3_inc_ref(self.context, ast);
            Z3Ast {
                ast,
                z3: self.selff.get().unwrap().clone(),
            }
        }
    }

    pub fn new_bvand(&self, s1: &Z3Ast, s2: &Z3Ast) -> Z3Ast {
        unsafe {
            let ast = Z3_mk_bvand(self.context, s1.ast, s2.ast);
            Z3_inc_ref(self.context, ast);
            Z3Ast {
                ast,
                z3: self.selff.get().unwrap().clone(),
            }
        }
    }

    pub fn new_bvconcat(&self, s1: &Z3Ast, s2: &Z3Ast) -> Z3Ast {
        unsafe {
            let ast = Z3_mk_concat(self.context, s1.ast, s2.ast);
            Z3_inc_ref(self.context, ast);
            Z3Ast {
                ast,
                z3: self.selff.get().unwrap().clone(),
            }
        }
    }

    pub fn new_bv_concrete(&self, value: u64, width: u32) -> Z3Ast {
        unsafe {
            let sort = Z3_mk_bv_sort(self.context, width);
            let ast = Z3_mk_unsigned_int64(self.context, value, sort);
            Z3_inc_ref(self.context, ast);
            Z3Ast {
                ast,
                z3: self.selff.get().unwrap().clone(),
            }
        }
    }

    pub fn new_bvmul(&self, s1: &Z3Ast, s2: &Z3Ast) -> Z3Ast {
        unsafe {
            let ast = Z3_mk_bvmul(self.context, s1.ast, s2.ast);
            Z3_inc_ref(self.context, ast);
            Z3Ast {
                ast,
                z3: self.selff.get().unwrap().clone(),
            }
        }
    }

    pub fn new_bvor(&self, s1: &Z3Ast, s2: &Z3Ast) -> Z3Ast {
        unsafe {
            let ast = Z3_mk_bvor(self.context, s1.ast, s2.ast);
            Z3_inc_ref(self.context, ast);
            Z3Ast {
                ast,
                z3: self.selff.get().unwrap().clone(),
            }
        }
    }

    pub fn new_sign_ext(&self, extension_width: u32, s1: &Z3Ast) -> Z3Ast {
        unsafe {
            let ast = Z3_mk_sign_ext(self.context, extension_width, s1.ast);
            Z3_inc_ref(self.context, ast);
            Z3Ast {
                ast,
                z3: self.selff.get().unwrap().clone(),
            }
        }
    }

    pub fn new_extract(&self, high: u32, low: u32, s1: &Z3Ast) -> Z3Ast {
        unsafe {
            let ast = Z3_mk_extract(self.context, high, low, s1.ast);
            Z3_inc_ref(self.context, ast);
            Z3Ast {
                ast,
                z3: self.selff.get().unwrap().clone(),
            }
        }
    }

    pub fn new_bvshl(&self, s1: &Z3Ast, s2: &Z3Ast) -> Z3Ast {
        unsafe {
            let ast = Z3_mk_bvshl(self.context, s1.ast, s2.ast);
            Z3_inc_ref(self.context, ast);
            Z3Ast {
                ast,
                z3: self.selff.get().unwrap().clone(),
            }
        }
    }

    pub fn new_bvlshr(&self, s1: &Z3Ast, s2: &Z3Ast) -> Z3Ast {
        unsafe {
            let ast = Z3_mk_bvlshr(self.context, s1.ast, s2.ast);
            Z3_inc_ref(self.context, ast);
            Z3Ast {
                ast,
                z3: self.selff.get().unwrap().clone(),
            }
        }
    }

    pub fn new_bvsub(&self, s1: &Z3Ast, s2: &Z3Ast) -> Z3Ast {
        unsafe {
            let ast = Z3_mk_bvsub(self.context, s1.ast, s2.ast);
            Z3_inc_ref(self.context, ast);
            Z3Ast {
                ast,
                z3: self.selff.get().unwrap().clone(),
            }
        }
    }

    pub fn new_fresh_const(&self, width: u32) -> Z3Ast {
        unsafe {
            let ast = Z3_mk_fresh_const(self.context, PREFIX.as_ptr(), Z3_mk_bv_sort(self.context, width));
            Z3_inc_ref(self.context, ast);
            Z3Ast {
                ast,
                z3: self.selff.get().unwrap().clone(),
            }
        }
    }

    pub fn new_bvurem(&self, s1: &Z3Ast, s2: &Z3Ast) -> Z3Ast {
        unsafe {
            let ast = Z3_mk_bvurem(self.context, s1.ast, s2.ast);
            Z3_inc_ref(self.context, ast);
            Z3Ast {
                ast,
                z3: self.selff.get().unwrap().clone(),
            }
        }
    }

    pub fn new_bvxor(&self, s1: &Z3Ast, s2: &Z3Ast) -> Z3Ast {
        unsafe {
            let ast = Z3_mk_bvxor(self.context, s1.ast, s2.ast);
            Z3_inc_ref(self.context, ast);
            Z3Ast {
                ast,
                z3: self.selff.get().unwrap().clone(),
            }
        }
    }
}

impl Z3Ast {
    pub fn assert(&self) {
        unsafe {
            let z3 = self.z3.upgrade().unwrap();
            Z3_solver_assert(z3.context, z3.solver, self.ast);
        }
    }
}

impl Drop for Z3Handle {
    fn drop(&mut self) {
        unsafe { Z3_del_context(self.context) }
    }
}

impl Drop for Z3Ast {
    fn drop(&mut self) {
        unsafe {
            if let Some(z3) = self.z3.upgrade() {
                Z3_dec_ref(z3.context, self.ast);
            }
        }
    }
}

impl Clone for Z3Ast {
    fn clone(&self) -> Self {
        unsafe {
            Z3_inc_ref(self.z3.upgrade().unwrap().context, self.ast)
        }
        Self { ast: self.ast.clone(), z3: self.z3.clone() }
    }
}
