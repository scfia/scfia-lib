use std::{fmt, rc::Rc, cell::RefCell};
use std::fmt::Debug;

use z3_sys::{
    AstKind, Z3_ast, Z3_get_ast_kind, Z3_get_bv_sort_size, Z3_get_numeral_uint64, Z3_get_sort,
    Z3_inc_ref, Z3_simplify,
};

use crate::{values::bit_vector_concrete::BitVectorConcrete, ScfiaStdlib};

use super::ast::ActiveAst;

pub trait BitVector: ActiveAst {
    // fn to_ast(&self, this: Rc<RefCell<dyn BitVector>>) -> Rc<RefCell<dyn ActiveAst>>;

    fn try_simplify_conrete(&self, stdlib: &mut ScfiaStdlib) -> Result<BitVectorConcrete, AstKind> {
        unsafe {
            let simplified = Z3_simplify(stdlib.z3_context, self.get_z3_ast());
            // TODO when do we have to call inc?
            let kind = Z3_get_ast_kind(stdlib.z3_context, simplified);
            match kind {
                AstKind::Numeral => {
                    let mut val: u64 = 0;
                    if !Z3_get_numeral_uint64(stdlib.z3_context, simplified, &mut val) {
                        panic!("Z3_get_numeral_uint64 failed")
                    }
                    let sort = Z3_get_sort(stdlib.z3_context, simplified);
                    let width = Z3_get_bv_sort_size(stdlib.z3_context, sort);
                    Ok(BitVectorConcrete::new(val, width, stdlib))
                }
                k => Err(k),
            }
        }
    }
}
