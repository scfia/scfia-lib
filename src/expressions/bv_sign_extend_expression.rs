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
use z3_sys::Z3_mk_sign_ext;

#[derive(Debug)]
pub struct BVSignExtendExpression {
    pub id: u64,
    pub s1: Rc<RefCell<dyn Ast>>,
    pub input_width: u32,
    pub output_width: u32,
    inherited_asts: Vec<Rc<RefCell<dyn Ast>>>,
    discovered_asts: HashMap<u64, Weak<RefCell<dyn Ast>>>,
    z3_context: Z3_context,
    z3_ast: Z3_ast,
}

impl BVSignExtendExpression {
    pub fn new(
        s1: Rc<RefCell<dyn Ast>>,
        input_width: u32,
        output_width: u32,
        stdlib: &mut ScfiaStdlib,
    ) -> BVSignExtendExpression {
        unsafe {
            let z3_context = stdlib.z3_context;
            let ast = Z3_mk_sign_ext(
                stdlib.z3_context,
                output_width - input_width,
                s1.try_borrow().unwrap().get_z3_ast(),
            );
            Z3_inc_ref(z3_context, ast);
            BVSignExtendExpression {
                id: stdlib.get_symbol_id(),
                s1: s1,
                input_width,
                output_width,
                inherited_asts: vec![],
                discovered_asts: HashMap::new(),
                z3_context: z3_context,
                z3_ast: ast,
            }
        }
    }
}

impl Ast for BVSignExtendExpression {
    fn get_id(&self) -> u64 {
        self.id
    }

    fn get_z3_ast(&self) -> Z3_ast {
        self.z3_ast
    }

    fn get_parents(&self, list: &mut Vec<Rc<RefCell<dyn Ast>>>) {
        list.push(self.s1.clone());
    }

    fn inherit(&mut self, ast: Rc<RefCell<dyn Ast>>) {
        self.inherited_asts.push(ast)
    }

    fn get_cloned(&self, clone_map: &mut HashMap<u64, Rc<RefCell<dyn Ast>>>, cloned_stdlib: &mut ScfiaStdlib) -> Rc<RefCell<dyn Ast>> {
        todo!()
    }
}

impl BitVector for BVSignExtendExpression {}

impl Expression for BVSignExtendExpression {}

impl BitVectorExpression for BVSignExtendExpression {}

impl Drop for BVSignExtendExpression {
    fn drop(&mut self) {
        // Retire expression, maintain z3 ast refcount
        let retired_expression = Rc::new(RefCell::new(RetiredBVSignExtendExpression {
            id: self.id,
            _s1: Rc::downgrade(&self.s1),
            input_width: self.input_width,
            output_width: self.output_width,
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
pub struct RetiredBVSignExtendExpression {
    id: u64,
    _s1: Weak<RefCell<dyn Ast>>,
    input_width: u32,
    output_width: u32,
    z3_context: Z3_context,
    z3_ast: Z3_ast,
}

impl Ast for RetiredBVSignExtendExpression {
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

impl Drop for RetiredBVSignExtendExpression {
    fn drop(&mut self) {
        unsafe { Z3_dec_ref(self.z3_context, self.z3_ast) }
    }
}

