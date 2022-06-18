use crate::traits::ast::Ast;
use crate::traits::bit_vector::BitVector;
use crate::traits::bit_vector_expression::BitVectorExpression;
use crate::traits::expression::Expression;
use crate::ScfiaStdlib;
use crate::values::ActiveValue;
use crate::values::RetiredValue;
use crate::values::Value;
use std::cell::Ref;
use std::cell::RefCell;
use std::collections::HashMap;
use std::rc::Rc;
use std::rc::Weak;
use z3_sys::Z3_ast;
use z3_sys::Z3_context;
use z3_sys::Z3_dec_ref;
use z3_sys::Z3_inc_ref;
use z3_sys::Z3_mk_bvadd;
use z3_sys::Z3_mk_eq;
use z3_sys::Z3_mk_not;
use z3_sys::Z3_solver_assert;

#[derive(Debug)]
pub struct BoolNEqExpression {
    pub id: u64,
    pub s1: Rc<RefCell<ActiveValue>>,
    pub s2: Rc<RefCell<ActiveValue>>,
    pub inherited_asts: Vec<Rc<RefCell<RetiredValue>>>,
    pub discovered_asts: HashMap<u64, Weak<RefCell<ActiveValue>>>,
    pub is_assert: bool,
    pub z3_context: Z3_context,
    pub z3_ast: Z3_ast,
}

#[derive(Debug)]
pub struct RetiredBoolNEqExpression {
    pub id: u64,
    s1_id: u64,
    s1: Weak<RefCell<ActiveValue>>,
    s2_id: u64,
    s2: Weak<RefCell<ActiveValue>>,
    pub is_assert: bool,
    pub z3_context: Z3_context,
    pub z3_ast: Z3_ast,
}

impl BoolNEqExpression {
    pub fn new(
        s1: Rc<RefCell<ActiveValue>>,
        s2: Rc<RefCell<ActiveValue>>,
        stdlib: &mut ScfiaStdlib,
    ) -> BoolNEqExpression {
        Self::new_with_id(stdlib.get_symbol_id(), s1, s2, stdlib)
    }

    pub fn new_with_id(
        id: u64,
        s1: Rc<RefCell<ActiveValue>>,
        s2: Rc<RefCell<ActiveValue>>,
        stdlib: &mut ScfiaStdlib,
    ) -> BoolNEqExpression {
        unsafe {
            let z3_context = stdlib.z3_context;
            let ast = Z3_mk_not(z3_context,
                Z3_mk_eq(
                    z3_context,
                    s1.try_borrow().unwrap().get_z3_ast(),
                    s2.try_borrow().unwrap().get_z3_ast(),
                ));
            Z3_inc_ref(z3_context, ast);
            BoolNEqExpression {
                id,
                s1: s1,
                s2: s2,
                inherited_asts: vec![],
                discovered_asts: HashMap::new(),
                is_assert: false,
                z3_context: z3_context,
                z3_ast: ast,
            }
        }
    }

    pub fn clone_to_stdlib(
        &self,
        cloned_active_values: &mut HashMap<u64, Rc<RefCell<ActiveValue>>>,
        cloned_retired_values: &mut HashMap<u64, Rc<RefCell<RetiredValue>>>,
        cloned_stdlib: &mut ScfiaStdlib
    ) -> Rc<RefCell<ActiveValue>> {
        // Check translated values map
        if let Some(eq_expression) = cloned_active_values.get(&self.id) {
            return eq_expression.clone()
        }

        // Clone s1, s2
        let s1 = self.s1.try_borrow().unwrap().clone_to_stdlib(cloned_active_values, cloned_retired_values, cloned_stdlib);
        let s2 = self.s2.try_borrow().unwrap().clone_to_stdlib(cloned_active_values, cloned_retired_values, cloned_stdlib);

        // Build clone
        let cloned_expression = Self::new_with_id(self.id, s1, s2, cloned_stdlib);
        if self.is_assert {
            unsafe {
                Z3_solver_assert(cloned_stdlib.z3_context, cloned_stdlib.z3_solver, cloned_expression.z3_ast);
            }
        }
        let cloned_expression: Rc<RefCell<ActiveValue>> = cloned_expression.into();
        cloned_active_values.insert(self.id, cloned_expression.clone());

        // Clone inherited values
        let mut cloned_inherited = vec![];
        for inherited_ast in self.inherited_asts.iter().rev() {
            cloned_inherited.push(inherited_ast.try_borrow().unwrap().clone_to_stdlib(cloned_active_values, cloned_retired_values, cloned_stdlib));
        }

        // Clone discovered values
        let mut cloned_discovered = vec![];
        for discovered_ast in self.discovered_asts.values() {
            cloned_discovered.push(discovered_ast.upgrade().unwrap().try_borrow().unwrap().clone_to_stdlib(cloned_active_values, cloned_retired_values, cloned_stdlib));
        }

        // Update clone with retirees and discoveries
        let mut cloned_expression_ref = cloned_expression.try_borrow_mut().unwrap();
        for cloned_inherited_ast in cloned_inherited {
            cloned_expression_ref.inherit(cloned_inherited_ast)
        }
        for cloned_discovered_value in cloned_discovered {
            cloned_expression_ref.discover(cloned_discovered_value.try_borrow().unwrap().get_id(), Rc::downgrade(&cloned_discovered_value))
        }

        cloned_expression.clone()
    }

    pub fn assert(&mut self, stdlib: &mut ScfiaStdlib) {
        self.is_assert = true;
        unsafe {
            Z3_solver_assert(stdlib.z3_context, stdlib.z3_solver, self.z3_ast);
        }
    }
}

impl Drop for BoolNEqExpression {
    fn drop(&mut self) {
        // Retire expression, maintain z3 ast refcount
        let retired_expression = Rc::new(RefCell::new(RetiredValue::RetiredBoolNEqExpression(RetiredBoolNEqExpression {
            id: self.id,
            s1_id: self.s1.try_borrow().unwrap().get_id(),
            s1: Rc::downgrade(&self.s1),
            s2_id: self.s2.try_borrow().unwrap().get_id(),
            s2: Rc::downgrade(&self.s2),
            is_assert: self.is_assert,
            z3_context: self.z3_context,
            z3_ast: self.z3_ast,
        })));

        // Heirs are parents and discovered symbols
        let mut heirs: Vec<Rc<RefCell<ActiveValue>>> = vec![self.s1.clone(), self.s2.clone()];
        for discovered_symbol in self.discovered_asts.values() {
            let discovered_symbol = discovered_symbol.upgrade().unwrap();
            let mut discovered_symbol_ref = discovered_symbol.try_borrow_mut().unwrap();
            discovered_symbol_ref.forget(self.id);
            heirs.push(discovered_symbol.clone())
        }

        // For each heir...
        for heir in &heirs {
            let mut heir_ref = heir.try_borrow_mut().unwrap();

            // Pass on inherited symbols
            for inherited in &self.inherited_asts {
                heir_ref.inherit(inherited.clone())
            }

            // Inherit
            heir_ref.inherit(retired_expression.clone());

            // Acquaint all heirs
            for other_heir in &heirs {
                if let Ok(mut other_heir_ref) = other_heir.try_borrow_mut() {
                    heir_ref.discover(other_heir_ref.get_id(), Rc::downgrade(other_heir));
                    other_heir_ref.discover(heir_ref.get_id(), Rc::downgrade(heir));
                }                
            }
        }
    }
}

impl RetiredBoolNEqExpression {
    pub fn clone_to_stdlib(
        &self,
        cloned_active_values: &mut HashMap<u64, Rc<RefCell<ActiveValue>>>,
        cloned_retired_values: &mut HashMap<u64, Rc<RefCell<RetiredValue>>>,
        cloned_stdlib: &mut ScfiaStdlib
    ) -> Rc<RefCell<RetiredValue>> {
        let s1_ast = if let Some(s1) = cloned_active_values.get(&self.s1_id) {
            s1.try_borrow().unwrap().get_z3_ast()
        } else if let Some(s1) = self.s1.upgrade() {
            s1.try_borrow().unwrap().clone_to_stdlib(cloned_active_values, cloned_retired_values, cloned_stdlib).try_borrow().unwrap().get_z3_ast()
        } else {
            cloned_retired_values.get(&self.s1_id).unwrap().try_borrow().unwrap().get_z3_ast()
        };

        let s2_ast = if let Some(s2) = cloned_active_values.get(&self.s2_id) {
            s2.try_borrow().unwrap().get_z3_ast()
        } else if let Some(s2) = self.s2.upgrade() {
            s2.try_borrow().unwrap().clone_to_stdlib(cloned_active_values, cloned_retired_values, cloned_stdlib).try_borrow().unwrap().get_z3_ast()
        } else {
            cloned_retired_values.get(&self.s2_id).unwrap().try_borrow().unwrap().get_z3_ast()
        };

        let neg_expression: Rc<RefCell<RetiredValue>> = unsafe {
            let z3_ast = Z3_mk_not(cloned_stdlib.z3_context,
                Z3_mk_eq(
                    cloned_stdlib.z3_context,
                    s1_ast,
                    s2_ast,
                )
            );
            Z3_inc_ref(cloned_stdlib.z3_context, z3_ast);
            RetiredBoolNEqExpression {
                id: self.id,
                is_assert: self.is_assert,
                s1_id: self.s1_id,
                s1: self.s1.clone(),
                s2_id: self.s2_id,
                s2: self.s2.clone(),
                z3_context: cloned_stdlib.z3_context,
                z3_ast
            }
        }.into();

        cloned_retired_values.insert(self.id, neg_expression.clone());
        neg_expression
    }
}

impl Drop for RetiredBoolNEqExpression {
    fn drop(&mut self) {
        unsafe { Z3_dec_ref(self.z3_context, self.z3_ast) }
    }
}
