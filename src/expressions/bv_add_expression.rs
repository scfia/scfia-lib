use crate::traits::ast::Ast;
use crate::traits::bit_vector::BitVector;
use crate::traits::bit_vector_expression::BitVectorExpression;
use crate::traits::expression::Expression;
use crate::ScfiaStdlib;
use crate::values::ActiveValue;
use crate::values::RetiredValue;
use crate::values::bit_vector_concrete::BitVectorConcrete;
use std::cell::Ref;
use std::cell::RefCell;
use std::collections::HashMap;
use std::ops::Deref;
use std::rc::Rc;
use std::rc::Weak;
use z3_sys::Z3_ast;
use z3_sys::Z3_context;
use z3_sys::Z3_dec_ref;
use z3_sys::Z3_inc_ref;
use z3_sys::Z3_mk_bvadd;

#[derive(Debug)]
pub struct BVAddExpression {
    pub id: u64,
    pub s1: Rc<RefCell<ActiveValue>>,
    pub s2: Rc<RefCell<ActiveValue>>,
    pub inherited_asts: Vec<Rc<RefCell<RetiredValue>>>,
    pub discovered_asts: HashMap<u64, Weak<RefCell<ActiveValue>>>,
    pub z3_context: Z3_context,
    pub z3_ast: Z3_ast,
}

#[derive(Debug)]
pub struct RetiredBVAddExpression {
    pub id: u64,
    s1: u64,
    s2: u64,
    pub z3_context: Z3_context,
    pub z3_ast: Z3_ast,
}

impl BVAddExpression {
    pub fn new(
        s1: Rc<RefCell<ActiveValue>>,
        s2: Rc<RefCell<ActiveValue>>,
        stdlib: &mut ScfiaStdlib,
    ) -> ActiveValue {
        match s1.try_borrow().unwrap().deref() {
            ActiveValue::BitvectorConcrete(e1) => {
                match s2.try_borrow().unwrap().deref() {
                    ActiveValue::BitvectorConcrete(e2) => {
                        let one: u64 = 1;
                        let mask = one.rotate_left(e2.width).overflowing_sub(1).0;
                        let sum = e1.value.overflowing_add(e2.value).0; 
                        let value = mask & sum;
                        return ActiveValue::BitvectorConcrete(BitVectorConcrete::new(value, e2.width, stdlib));
                    },
                    _ => {}
                }
            }
            _ => {}
        }
        ActiveValue::BitvectorAddExpression(Self::new_with_id(stdlib.get_symbol_id(), s1,  s2, stdlib))
    }

    pub fn new_with_id(
        id: u64,
        s1: Rc<RefCell<ActiveValue>>,
        s2: Rc<RefCell<ActiveValue>>,
        stdlib: &mut ScfiaStdlib,
    ) -> BVAddExpression {
        unsafe {
            let z3_context = stdlib.z3_context;
            let ast = Z3_mk_bvadd(
                stdlib.z3_context,
                s1.try_borrow().unwrap().get_z3_ast(),
                s2.try_borrow().unwrap().get_z3_ast(),
            );
            Z3_inc_ref(z3_context, ast);
            BVAddExpression {
                id: id,
                s1: s1,
                s2: s2,
                inherited_asts: vec![],
                discovered_asts: HashMap::new(),
                z3_context: z3_context,
                z3_ast: ast,
            }
        }
    }
}

impl Drop for BVAddExpression {
    fn drop(&mut self) {
        // Retire expression, maintain z3 ast refcount
        let retired_expression = Rc::new(RefCell::new(RetiredValue::RetiredBitvectorAddExpression(RetiredBVAddExpression {
            id: self.id,
            s1: self.s1.try_borrow().unwrap().get_id(),
            s2: self.s2.try_borrow().unwrap().get_id(),
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

impl Drop for RetiredBVAddExpression {
    fn drop(&mut self) {
        unsafe { Z3_dec_ref(self.z3_context, self.z3_ast) }
    }
}

/*
#[cfg(test)]
mod tests {
    use std::cell::RefCell;
    use std::rc::Rc;

    use z3_sys::{
        AstKind, Z3_get_ast_kind, Z3_inc_ref, Z3_mk_app, Z3_mk_bv_sort, Z3_mk_unsigned_int64,
    };

    use crate::traits::bit_vector::BitVector;
    use crate::values::bit_vector_concrete::BitVectorConcrete;
    use crate::values::bit_vector_symbol::BitVectorSymbol;
    use crate::ScfiaStdlib;

    use super::BVAddExpression;

    #[test]
    fn add_concrete() {
        let mut stdlib = ScfiaStdlib::new();
        let i = 42;
        let s1 = Rc::new(RefCell::new(BitVectorConcrete::new(i, 32, &mut stdlib)));
        let s2 = Rc::new(RefCell::new(BitVectorConcrete::new(i, 32, &mut stdlib)));
        let add = BVAddExpression::new(s1, s2, &mut stdlib);
        let simplified = add.try_simplify_conrete(&mut stdlib).unwrap();
        assert_eq!(simplified.value, 2 * i);
        assert_eq!(simplified.width, 32);
    }

    #[test]
    fn add_nested() {
        let mut stdlib = ScfiaStdlib::new();
        let i = 42;
        let bv0 = Rc::new(RefCell::new(BitVectorConcrete::new(i, 32, &mut stdlib)));
        let bv1 = Rc::new(RefCell::new(BitVectorConcrete::new(i + 1, 32, &mut stdlib)));
        let s0 = Rc::new(RefCell::new(BitVectorSymbol::new(None, 32, &mut stdlib))); // 0
        let s1 = Rc::new(RefCell::new(BitVectorSymbol::new(None, 32, &mut stdlib))); // 1

        let expr1 = Rc::new(RefCell::new(BVAddExpression::new(bv0, s0, &mut stdlib))); // 2
        let expr2 = Rc::new(RefCell::new(BVAddExpression::new(bv1, s1, &mut stdlib))); // 3
        let _expr3 = BVAddExpression::new(expr1, expr2, &mut stdlib); // 4
    }
}
 */