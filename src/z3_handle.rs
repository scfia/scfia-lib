use std::{cell::Cell, rc::Weak};

use z3_sys::{Z3_ast, Z3_context, Z3_solver, Z3_dec_ref};

pub struct Z3Handle {
    pub context: Z3_context,
    pub solver: Z3_solver,
    pub actives: Cell<u64>,
    pub inactives: Cell<u64>,
}

pub struct Z3Ast {
    pub ast: Z3_ast,
    pub z3: Weak<Z3Handle>,
}

impl Drop for Z3Ast {
    fn drop(&mut self) {
        unsafe {
            if let Some(z3) = self.z3.upgrade() {
                Z3_dec_ref(z3.context, self.ast);
                z3.actives.replace(z3.actives.get() - 1);
            }
        }
    }
}
