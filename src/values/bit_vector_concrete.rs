use z3_sys::Z3_context;
use z3_sys::Z3_inc_ref;
use z3_sys::Z3_dec_ref;
use z3_sys::Z3_mk_bv_sort;
use z3_sys::Z3_mk_unsigned_int64;
use z3_sys::Z3_ast;
use std::cell::RefCell;
use std::rc::Rc;
use crate::ScfiaStdlib;
use crate::traits::ast::Ast;
use crate::traits::bit_vector::BitVector;


#[derive(Debug)]
pub struct BitVectorConcrete {
    pub value: u64,
    pub width: u32,
    z3_context: Z3_context,
    z3_ast: Z3_ast,
}

impl BitVectorConcrete {
    pub fn new(value: u64, width: u32, stdlib: &ScfiaStdlib) -> BitVectorConcrete {
        unsafe {
            let sort = Z3_mk_bv_sort(stdlib.z3_context, width);
            // Z3_inc_ref(stdlib.z3_context, sort);

            let ast = Z3_mk_unsigned_int64(stdlib.z3_context, value, sort);
            Z3_inc_ref(stdlib.z3_context, ast);

            // let z3_ast = stdlib.z3_context
            let bvc = BitVectorConcrete {
                value: value,
                width: width,
                z3_context: stdlib.z3_context,
                z3_ast: ast,
            };
            bvc
        }
    }
}

impl Ast for BitVectorConcrete {
    fn get_id(&self) -> u64 {
        unreachable!()
    }    

    fn get_z3_ast(&self) -> Z3_ast {
        self.z3_ast
    }

    fn get_parents(&self, _list: &mut Vec<Rc<RefCell<dyn Ast>>>) {}

    fn inherit(&mut self, _ast: Rc<RefCell<dyn Ast>>) {}

    fn get_cloned(&self, clone_map: &mut std::collections::HashMap<u64, Rc<RefCell<dyn Ast>>>, cloned_stdlib: &mut ScfiaStdlib) -> Rc<RefCell<dyn Ast>> {
        todo!()
    }
}

impl BitVector for BitVectorConcrete {

}

impl Drop for BitVectorConcrete {
    fn drop(&mut self) {
        unsafe {
            Z3_dec_ref(self.z3_context, self.z3_ast)
        }
    }
}
