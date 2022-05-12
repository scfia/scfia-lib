use crate::traits::ast::ActiveAst;
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
use z3_sys::Z3_mk_concat;

#[derive(Debug)]
pub struct BVConcatExpression {
    pub id: u64,
    pub s1: Rc<RefCell<dyn ActiveAst>>,
    pub s2: Rc<RefCell<dyn ActiveAst>>,
    inherited_asts: Vec<Rc<RefCell<dyn Ast>>>,
    discovered_asts: HashMap<u64, Weak<RefCell<dyn ActiveAst>>>,
    z3_context: Z3_context,
    z3_ast: Z3_ast,
}

impl BVConcatExpression {
    pub fn new(
        s1: Rc<RefCell<dyn ActiveAst>>,
        s2: Rc<RefCell<dyn ActiveAst>>,
        stdlib: &mut ScfiaStdlib,
    ) -> BVConcatExpression {
        unsafe {
            let z3_context = stdlib.z3_context;
            let ast = Z3_mk_concat(
                stdlib.z3_context,
                s1.try_borrow().unwrap().get_z3_ast(),
                s2.try_borrow().unwrap().get_z3_ast(),
            );
            Z3_inc_ref(z3_context, ast);
            BVConcatExpression {
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

impl Ast for BVConcatExpression {
    fn get_z3_ast(&self) -> Z3_ast {
        self.z3_ast
    }
}

impl ActiveAst for BVConcatExpression {
    fn get_parents(&self, list: &mut Vec<Rc<RefCell<dyn ActiveAst>>>) {
        list.push(self.s1.clone());
        list.push(self.s2.clone());
    }

    fn inherit(&mut self, ast: Rc<RefCell<dyn Ast>>) {
        self.inherited_asts.push(ast)
    }
}

impl BitVector for BVConcatExpression {}

impl Expression for BVConcatExpression {}

impl BitVectorExpression for BVConcatExpression {}

impl Drop for BVConcatExpression {
    fn drop(&mut self) {
        // Retire expression, maintain z3 ast refcount
        let retired_expression = Rc::new(RefCell::new(RetiredBVConcatExpression {
            _id: self.id,
            _s1: Rc::downgrade(&self.s1),
            _s2: Rc::downgrade(&self.s2),
            z3_context: self.z3_context,
            z3_ast: self.z3_ast,
        }));

        // Heirs are paraents and discovered symbols
        let mut heirs: Vec<Rc<RefCell<dyn ActiveAst>>> = vec![];
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
pub struct RetiredBVConcatExpression {
    _id: u64,
    _s1: Weak<RefCell<dyn ActiveAst>>,
    _s2: Weak<RefCell<dyn ActiveAst>>,
    z3_context: Z3_context,
    z3_ast: Z3_ast,
}

impl Ast for RetiredBVConcatExpression {
    fn get_z3_ast(&self) -> Z3_ast {
        self.z3_ast
    }
}

impl Drop for RetiredBVConcatExpression {
    fn drop(&mut self) {
        unsafe { Z3_dec_ref(self.z3_context, self.z3_ast) }
    }
}
