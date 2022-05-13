use crate::traits::ast::Ast;
use crate::traits::bit_vector::BitVector;
use crate::traits::bit_vector_expression::BitVectorExpression;
use crate::traits::bool::Bool;
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
use z3_sys::Z3_mk_eq;

#[derive(Debug)]
pub struct BoolEqExpression {
    pub id: u64,
    pub s1: Rc<RefCell<dyn Ast>>,
    pub s2: Rc<RefCell<dyn Ast>>,
    inherited_asts: Vec<Rc<RefCell<dyn Ast>>>,
    discovered_asts: HashMap<u64, Weak<RefCell<dyn Ast>>>,
    z3_context: Z3_context,
    z3_ast: Z3_ast,
}

impl BoolEqExpression {
    pub fn new(
        s1: Rc<RefCell<dyn Ast>>,
        s2: Rc<RefCell<dyn Ast>>,
        stdlib: &mut ScfiaStdlib,
    ) -> BoolEqExpression {
        unsafe {
            let z3_context = stdlib.z3_context;
            let ast = Z3_mk_eq(
                stdlib.z3_context,
                s1.try_borrow().unwrap().get_z3_ast(),
                s2.try_borrow().unwrap().get_z3_ast(),
            );
            Z3_inc_ref(z3_context, ast);
            BoolEqExpression {
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

impl Ast for BoolEqExpression {
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

impl Bool for BoolEqExpression {}

impl Expression for BoolEqExpression {}

impl Drop for BoolEqExpression {
    fn drop(&mut self) {
        // Retire expression, maintain z3 ast refcount
        unsafe { Z3_dec_ref(self.z3_context, self.z3_ast) }
        return;
        let retired_expression = Rc::new(RefCell::new(RetiredBoolEqExpression {
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
pub struct RetiredBoolEqExpression {
    id: u64,
    _s1: Weak<RefCell<dyn Ast>>,
    _s2: Weak<RefCell<dyn Ast>>,
    z3_context: Z3_context,
    z3_ast: Z3_ast,
}

impl Ast for RetiredBoolEqExpression {
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

impl Drop for RetiredBoolEqExpression {
    fn drop(&mut self) {
        unsafe { Z3_dec_ref(self.z3_context, self.z3_ast) }
    }
}
