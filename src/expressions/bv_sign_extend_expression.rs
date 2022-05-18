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
use z3_sys::Z3_mk_sign_ext;

#[derive(Debug)]
pub struct BVSignExtendExpression {
    pub id: u64,
    pub s1: Rc<RefCell<ActiveValue>>,
    pub input_width: u32,
    pub output_width: u32,
    pub inherited_asts: Vec<Rc<RefCell<RetiredValue>>>,
    pub discovered_asts: HashMap<u64, Weak<RefCell<ActiveValue>>>,
    pub z3_context: Z3_context,
    pub z3_ast: Z3_ast,
}

#[derive(Debug)]
pub struct RetiredBVSignExtendExpression {
    pub id: u64,
    s1: u64,
    input_width: u32,
    output_width: u32,
    pub z3_context: Z3_context,
    pub z3_ast: Z3_ast,
}

impl BVSignExtendExpression {
    pub fn new(
        s1: Rc<RefCell<ActiveValue>>,
        input_width: u32,
        output_width: u32,
        stdlib: &mut ScfiaStdlib,
    ) -> ActiveValue {
        match s1.try_borrow().unwrap().deref() {
            ActiveValue::BitvectorConcrete(e) => {
                // https://graphics.stanford.edu/~seander/bithacks.html#VariableSignExtend
                let m: u64 = 1 << (e.width - 1);
                let x = e.value & ((1 << e.width) - 1);
                let value = (x ^ m).overflowing_sub(m).0;
                return ActiveValue::BitvectorConcrete(BitVectorConcrete::new(value, output_width, stdlib));
            }
            _ => {}
        }
        ActiveValue::BitvectorSignExtendExpression(Self::new_with_id(stdlib.get_symbol_id(), s1, input_width, output_width, stdlib))
    }

    pub fn new_with_id(
        id: u64,
        s1: Rc<RefCell<ActiveValue>>,
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
                id: id,
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

impl Drop for BVSignExtendExpression {
    fn drop(&mut self) {
        // Retire expression, maintain z3 ast refcount
        let retired_expression = Rc::new(RefCell::new(RetiredValue::RetiredBitvectorSignExtendExpression(RetiredBVSignExtendExpression {
            id: self.id,
            s1: self.s1.try_borrow().unwrap().get_id(),
            input_width: self.input_width,
            output_width: self.output_width,
            z3_context: self.z3_context,
            z3_ast: self.z3_ast,
        })));

        // Heirs are parents and discovered symbols
        let mut heirs: Vec<Rc<RefCell<ActiveValue>>> = vec![self.s1.clone()];
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

impl Drop for RetiredBVSignExtendExpression {
    fn drop(&mut self) {
        unsafe { Z3_dec_ref(self.z3_context, self.z3_ast) }
    }
}

