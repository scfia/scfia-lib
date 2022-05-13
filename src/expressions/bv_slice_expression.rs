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
use z3_sys::Z3_mk_extract;

#[derive(Debug)]
pub struct BVSliceExpression {
    pub id: u64,
    pub s1: Rc<RefCell<dyn Ast>>,
    pub high: u32,
    pub low: u32,
    inherited_asts: Vec<Rc<RefCell<dyn Ast>>>,
    discovered_asts: HashMap<u64, Weak<RefCell<dyn Ast>>>,
    z3_context: Z3_context,
    z3_ast: Z3_ast,
}

impl BVSliceExpression {
    pub fn new(
        s1: Rc<RefCell<dyn Ast>>,
        high: u32,
        low: u32,
        stdlib: &mut ScfiaStdlib,
    ) -> BVSliceExpression {
        Self::new_with_id(stdlib.get_symbol_id(), s1, high, low, stdlib)
    }

    pub fn new_with_id(
        id: u64,
        s1: Rc<RefCell<dyn Ast>>,
        high: u32,
        low: u32,
        stdlib: &mut ScfiaStdlib,
    ) -> BVSliceExpression {
        unsafe {
            let z3_context = stdlib.z3_context;
            let ast = Z3_mk_extract(
                stdlib.z3_context,
                high,
                low,
                s1.try_borrow().unwrap().get_z3_ast(),
            );
            Z3_inc_ref(z3_context, ast);
            BVSliceExpression {
                id: id,
                s1: s1,
                high: high,
                low: low,
                inherited_asts: vec![],
                discovered_asts: HashMap::new(),
                z3_context: z3_context,
                z3_ast: ast,
            }
        }
    }
}

impl Ast for BVSliceExpression {
    fn get_z3_ast(&self) -> Z3_ast {
        self.z3_ast
    }

    fn get_id(&self) -> u64 {
        self.id
    }

    fn get_parents(&self, list: &mut Vec<Rc<RefCell<dyn Ast>>>) {
        list.push(self.s1.clone());
    }

    fn inherit(&mut self, ast: Rc<RefCell<dyn Ast>>) {
        self.inherited_asts.push(ast)
    }

    fn get_cloned(
        &self,
        clone_map: &mut HashMap<u64, Rc<RefCell<dyn Ast>>>,
        cloned_stdlib: &mut ScfiaStdlib,
    ) -> Rc<RefCell<dyn Ast>> {
        if let Some(active_ast) = clone_map.get(&self.id) {
            return active_ast.clone();
        }

        // Rebuild expression
        let s1_clone = self
            .s1
            .try_borrow()
            .unwrap()
            .get_cloned(clone_map, cloned_stdlib);
        let mut selff = BVSliceExpression::new(s1_clone, self.high, self.low, cloned_stdlib);

        // Add retirees
        for retiree in &self.inherited_asts {
            selff.inherited_asts.push(
                retiree
                    .try_borrow()
                    .unwrap()
                    .get_cloned(clone_map, cloned_stdlib),
            )
        }

        // Add discoveries
        for discovered in self.discovered_asts.values() {
            let discovered = discovered.upgrade().unwrap();
            let discovered = discovered.try_borrow().unwrap();
            selff.discovered_asts.insert(
                discovered.get_id(),
                Rc::downgrade(&discovered.get_cloned(clone_map, cloned_stdlib)),
            );
        }

        Rc::new(RefCell::new(selff))
    }
}

impl BitVector for BVSliceExpression {}

impl Expression for BVSliceExpression {}

impl BitVectorExpression for BVSliceExpression {}

impl Drop for BVSliceExpression {
    fn drop(&mut self) {
        // Retire expression, maintain z3 ast refcount
        let retired_expression = Rc::new(RefCell::new(RetiredBVSliceExpression {
            id: self.id,
            s1: Rc::downgrade(&self.s1),
            high: self.high,
            low: self.low,
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
pub struct RetiredBVSliceExpression {
    id: u64,
    s1: Weak<RefCell<dyn Ast>>,
    high: u32,
    low: u32,
    z3_context: Z3_context,
    z3_ast: Z3_ast,
}

impl Ast for RetiredBVSliceExpression {
    fn get_id(&self) -> u64 {
        self.id
    }

    fn get_z3_ast(&self) -> Z3_ast {
        self.z3_ast
    }

    fn get_cloned(
        &self,
        clone_map: &mut HashMap<u64, Rc<RefCell<dyn Ast>>>,
        cloned_stdlib: &mut ScfiaStdlib,
    ) -> Rc<RefCell<dyn Ast>> {
        if let Some(ast) = clone_map.get(&self.id) {
            return ast.clone();
        }

        let s1 = self
            .s1
            .upgrade()
            .unwrap()
            .try_borrow()
            .unwrap()
            .get_cloned(clone_map, cloned_stdlib);
        let s1_ast = s1.try_borrow().unwrap().get_z3_ast();
        unsafe {
            Rc::new(RefCell::new(RetiredBVSliceExpression {
                id: self.id,
                s1: Rc::downgrade(&s1),
                high: self.high,
                low: self.low,
                z3_ast: Z3_mk_extract(
                    cloned_stdlib.z3_context,
                    self.high,
                    self.low,
                    s1_ast,
                ),
                z3_context: cloned_stdlib.z3_context,
            }))
        }
    }

    fn get_parents(&self, list: &mut Vec<Rc<RefCell<dyn Ast>>>) {
        unreachable!()
    }

    fn inherit(&mut self, ast: Rc<RefCell<dyn Ast>>) {
        unreachable!()
    }
}

impl Drop for RetiredBVSliceExpression {
    fn drop(&mut self) {
        unsafe { Z3_dec_ref(self.z3_context, self.z3_ast) }
    }
}
