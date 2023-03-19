use std::cell::RefCell;
use std::collections::BTreeMap;

use std::rc::Rc;

use log::trace;

use z3_sys::Z3_ast;

use z3_sys::Z3_context;

use z3_sys::Z3_del_config;
use z3_sys::Z3_del_context;

use z3_sys::Z3_inc_ref;
use z3_sys::Z3_mk_bv_sort;
use z3_sys::Z3_mk_bvadd;
use z3_sys::Z3_mk_config;
use z3_sys::Z3_mk_context_rc;

use z3_sys::Z3_mk_solver;
use z3_sys::Z3_mk_unsigned_int64;

use z3_sys::Z3_solver;

use z3_sys::Z3_solver_inc_ref;

use crate::values::active_value::ActiveExpression;
use crate::values::active_value::ActiveValue;
use crate::values::active_value::ActiveValueInner;
use crate::values::active_value::ActiveValueWeak;
use crate::values::bv_add_expression::BVAddExpression;
use crate::values::bv_concrete::BVConcrete;

use crate::values::retired_value::RetiredValueInner;
use crate::values::retired_value::RetiredValueWeak;

#[derive(Clone)]
pub struct Scfia {
    pub(crate) inner: Rc<RefCell<ScfiaInner>>,
}

pub struct ScfiaInner {
    next_symbol_id: u64,
    active_symbols: BTreeMap<u64, ActiveValueWeak>,
    retired_symbols: BTreeMap<u64, RetiredValueWeak>,
    z3_context: Z3_context,
    z3_solver: Z3_solver,
}

impl Scfia {
    pub fn new() -> Scfia {
        unsafe {
            let z3_config = Z3_mk_config();
            let z3_context = Z3_mk_context_rc(z3_config);
            let z3_solver = Z3_mk_solver(z3_context);
            Z3_solver_inc_ref(z3_context, z3_solver);
            Z3_del_config(z3_config);
            Scfia {
                inner: Rc::new(RefCell::new(ScfiaInner {
                    next_symbol_id: 0,
                    active_symbols: BTreeMap::new(),
                    retired_symbols: BTreeMap::new(),
                    z3_context,
                    z3_solver,
                })),
            }
        }
    }

    pub fn new_bv_concrete(&self, value: u64, width: u32) -> ActiveValue {
        unsafe {
            let mut selff = self.inner.try_borrow_mut().unwrap();
            let sort = Z3_mk_bv_sort(selff.z3_context, width);
            let z3_ast = Z3_mk_unsigned_int64(selff.z3_context, value, sort);
            Z3_inc_ref(selff.z3_context, z3_ast);
            let id = selff.next_symbol_id();

            selff.insert_active(ActiveExpression::BVConcrete(BVConcrete { value, width }), z3_ast, id, self.clone())
        }
    }

    pub fn new_bv_add(&self, s1: ActiveValue, s2: ActiveValue, width: u32) -> ActiveValue {
        unsafe {
            let mut selff = self.inner.try_borrow_mut().unwrap();
            let id = selff.next_symbol_id();
            if let ActiveExpression::BVConcrete(s1) = &s1.try_borrow().unwrap().expression {
                if let ActiveExpression::BVConcrete(s2) = &s2.try_borrow().unwrap().expression {
                    let one: u64 = 1;
                    let mask = one.rotate_left(s2.width).overflowing_sub(1).0;
                    let sum = s1.value.overflowing_add(s2.value).0;
                    let value = mask & sum;
                    let sort = Z3_mk_bv_sort(selff.z3_context, width);
                    let z3_ast = Z3_mk_unsigned_int64(selff.z3_context, value, sort);
                    return selff.insert_active(ActiveExpression::BVConcrete(BVConcrete { value, width }), z3_ast, id, self.clone());
                }
            };

            let z3_ast = Z3_mk_bvadd(selff.z3_context, s1.try_borrow().unwrap().z3_ast, s2.try_borrow().unwrap().z3_ast);
            Z3_inc_ref(selff.z3_context, z3_ast);
            selff.insert_active(ActiveExpression::BVAddExpression(BVAddExpression { s1, s2, width }), z3_ast, id, self.clone())
        }
    }

    pub fn drop_active(&self, value: &ActiveValueInner) {
        trace!("Retiring {:?}", value);
        let mut selff = self.inner.try_borrow_mut().unwrap();
        debug_assert!(selff.active_symbols.remove(&value.id).is_some())
    }

    pub fn drop_retired(&self, value: &RetiredValueInner) {
        trace!("Dropping {:?}", value);
        let mut selff = self.inner.try_borrow_mut().unwrap();
        debug_assert!(selff.active_symbols.remove(&value.id).is_some())
    }
}

impl Default for Scfia {
    fn default() -> Self {
        Self::new()
    }
}

impl ScfiaInner {
    fn next_symbol_id(&mut self) -> u64 {
        let id = self.next_symbol_id;
        self.next_symbol_id = id + 1;
        id
    }

    fn insert_active(&mut self, expression: ActiveExpression, z3_ast: Z3_ast, id: u64, scfia: Scfia) -> ActiveValue {
        let value = Rc::new(RefCell::new(ActiveValueInner {
            id,
            z3_ast,
            expression,
            inherited_asts: BTreeMap::new(),
            discovered_asts: BTreeMap::new(),
            scfia,
        }));
        trace!("Creating new Expression ({:?})", value.try_borrow().unwrap());
        self.active_symbols.insert(id, Rc::downgrade(&value));
        value
    }
}

impl Drop for ScfiaInner {
    fn drop(&mut self) {
        unsafe { Z3_del_context(self.z3_context) }
    }
}
