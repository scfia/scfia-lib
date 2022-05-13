use crate::traits::ast::Ast;
use crate::traits::bit_vector::BitVector;
use crate::traits::bit_vector_expression::BitVectorExpression;
use crate::traits::expression::Expression;
use crate::ScfiaStdlib;
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

#[derive(Debug)]
pub struct BVAddExpression {
    pub id: u64,
    pub s1: Rc<RefCell<dyn Ast>>,
    pub s2: Rc<RefCell<dyn Ast>>,
    inherited_asts: Vec<Rc<RefCell<dyn Ast>>>,
    discovered_asts: HashMap<u64, Weak<RefCell<dyn Ast>>>,
    z3_context: Z3_context,
    z3_ast: Z3_ast,
}

impl BVAddExpression {
    pub fn new(
        s1: Rc<RefCell<dyn Ast>>,
        s2: Rc<RefCell<dyn Ast>>,
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
                id: stdlib.get_symbol_id(),
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

impl Ast for BVAddExpression {
    fn get_id(&self) -> u64 {
        self.id
    }

    fn get_z3_ast(&self) -> Z3_ast {
        self.z3_ast
    }

    fn get_parents(&self, list: &mut Vec<Rc<RefCell<dyn Ast>>>) {
        list.push(self.s1.clone());
        list.push(self.s2.clone());
    }

    fn inherit(&mut self, ast: Rc<RefCell<dyn Ast>>) {
        self.inherited_asts.push(ast)
    }

    fn get_cloned(&self, clone_map: &mut HashMap<u64, Rc<RefCell<dyn Ast>>>, cloned_stdlib: &mut ScfiaStdlib) -> Rc<RefCell<dyn Ast>> {
        todo!()
    }
}

impl BitVector for BVAddExpression {}

impl Expression for BVAddExpression {}

impl BitVectorExpression for BVAddExpression {}

impl Drop for BVAddExpression {
    fn drop(&mut self) {
        // Retire expression, maintain z3 ast refcount
        let retired_expression = Rc::new(RefCell::new(RetiredBVAddExpression {
            id: self.id,
            _s1: Rc::downgrade(&self.s1),
            _s2: Rc::downgrade(&self.s2),
            z3_context: self.z3_context,
            z3_ast: self.z3_ast,
        }));

        // Heirs are paraents and discovered symbols
        let mut heirs: Vec<Rc<RefCell<dyn Ast>>> = vec![];
        self.get_parents(&mut heirs);
        for discovered_symbol in self.discovered_asts.values() {
            heirs.push(discovered_symbol.upgrade().unwrap())
        }

        // For each heir...
        for parent in &heirs {
            let mut parent_ref = parent.try_borrow_mut().unwrap();

            // Pass on inherited symbols
            for inherited in &self.inherited_asts {
                parent_ref.inherit(inherited.clone())
            }

            // Inherit
            parent_ref.inherit(retired_expression.clone());
        }
    }
}

#[derive(Debug)]
pub struct RetiredBVAddExpression {
    id: u64,
    _s1: Weak<RefCell<dyn Ast>>,
    _s2: Weak<RefCell<dyn Ast>>,
    z3_context: Z3_context,
    z3_ast: Z3_ast,
}

impl Ast for RetiredBVAddExpression {
    fn get_id(&self) -> u64 {
        self.id
    }

    fn get_z3_ast(&self) -> Z3_ast {
        self.z3_ast
    }

    fn get_parents(&self, list: &mut Vec<Rc<RefCell<dyn Ast>>>) {
        todo!()
    }

    fn inherit(&mut self, ast: Rc<RefCell<dyn Ast>>) {
        todo!()
    }

    fn get_cloned(&self, clone_map: &mut HashMap<u64, Rc<RefCell<dyn Ast>>>, cloned_stdlib: &mut ScfiaStdlib) -> Rc<RefCell<dyn Ast>> {
        todo!()
    }
}

impl Drop for RetiredBVAddExpression {
    fn drop(&mut self) {
        unsafe { Z3_dec_ref(self.z3_context, self.z3_ast) }
    }
}

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
