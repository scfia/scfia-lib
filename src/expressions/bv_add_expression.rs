use crate::ScfiaStdlib;
use crate::values::ActiveValue;
use crate::values::RetiredValue;
use crate::values::bit_vector_concrete::BitVectorConcrete;
use std::cell::Ref;
use std::cell::RefCell;
use std::collections::BTreeMap;
use std::collections::HashMap;
use std::ops::Deref;
use std::rc::Rc;
use std::rc::Weak;
use z3_sys::Z3_ast;
use z3_sys::Z3_context;
use z3_sys::Z3_dec_ref;
use z3_sys::Z3_inc_ref;
use z3_sys::Z3_mk_bvadd;

use super::inherit;

#[derive(Debug)]
pub struct BVAddExpression {
    pub id: u64,
    pub s1: Rc<RefCell<ActiveValue>>,
    pub s2: Rc<RefCell<ActiveValue>>,
    pub inherited_asts: BTreeMap<u64, Rc<RefCell<RetiredValue>>>,
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
                        // TODO check
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
                inherited_asts: BTreeMap::new(),
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
        let s1_id = self.s1.try_borrow().unwrap().get_id();
        let s2_id = self.s2.try_borrow().unwrap().get_id();

        let retired_expression = Rc::new(RefCell::new(RetiredValue::RetiredBitvectorAddExpression(RetiredBVAddExpression {
            id: self.id,
            s1: s1_id,
            s2: s2_id,
            z3_context: self.z3_context,
            z3_ast: self.z3_ast,
        })));

        let parents = vec![
            (s1_id, self.s1.clone()),
            (s2_id, self.s2.clone()),
        ];

        inherit(
            self.id,
            retired_expression,
            parents,
            &self.inherited_asts,
            &self.discovered_asts
        );
    }
}

impl Drop for RetiredBVAddExpression {
    fn drop(&mut self) {
        unsafe { Z3_dec_ref(self.z3_context, self.z3_ast) }
    }
}
