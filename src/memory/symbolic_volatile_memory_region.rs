use crate::memory::MemoryRegion32;
use crate::ActiveValue;
use crate::BitVectorSymbol;
use crate::ForkSink;
use crate::Rc;
use core::cell::RefCell;
use z3_sys::Z3_dec_ref;
use z3_sys::Z3_inc_ref;
use z3_sys::Z3_mk_bv_sort;
use z3_sys::Z3_mk_bvadd;
use z3_sys::Z3_mk_bvuge;
use z3_sys::Z3_mk_bvult;
use z3_sys::Z3_mk_unsigned_int64;
use z3_sys::Z3_solver_check_assumptions;
use z3_sys::Z3_L_FALSE;

#[derive(Debug)]
pub struct SymbolicVolatileMemoryRegion32 {
    pub base_symbol: Rc<RefCell<ActiveValue>>,
    pub length: u32,
}

impl MemoryRegion32 for SymbolicVolatileMemoryRegion32 {
    fn read(&mut self, address: u32, width: u32, stdlib: &mut crate::ScfiaStdlib, fork_sink: &mut Option<&mut ForkSink>) -> Rc<RefCell<ActiveValue>> {
        let s = BitVectorSymbol::new(None, width, format!("<= 0x{:x}", address).into(), stdlib, fork_sink);
        // println!("<= 0x{:x} (volatile) yielding {}", address, s.try_borrow().unwrap().get_id());
        s
    }

    fn write(
        &mut self,
        _address: u32,
        _value: Rc<RefCell<ActiveValue>>,
        _width: u32,
        _stdlib: &mut crate::ScfiaStdlib,
        _fork_sink: &mut Option<&mut ForkSink>,
    ) {
    }
}

impl SymbolicVolatileMemoryRegion32 {
    pub fn contains(self, address: &ActiveValue, stdlib: &mut crate::ScfiaStdlib) -> bool {
        unsafe {
            let base_ast = self.base_symbol.try_borrow().unwrap().get_z3_ast();

            // address < base_address
            let lt = Z3_mk_bvult(stdlib.z3_context, address.get_z3_ast(), base_ast);
            Z3_inc_ref(stdlib.z3_context, lt);

            // address >= base_address + length
            let sort = Z3_mk_bv_sort(stdlib.z3_context, 32);
            let add_ast = Z3_mk_unsigned_int64(stdlib.z3_context, self.length.into(), sort);
            let ge = Z3_mk_bvuge(stdlib.z3_context, address.get_z3_ast(), Z3_mk_bvadd(stdlib.z3_context, base_ast, add_ast));
            Z3_inc_ref(stdlib.z3_context, ge);

            let assumptions = [lt, ge];

            if Z3_solver_check_assumptions(stdlib.z3_context, stdlib.z3_solver, 2, assumptions.as_ptr()) == Z3_L_FALSE {
                // If the address CAN NOT be outside the symbolic volatile region, we can return a new BVS
                return true;
            }

            Z3_dec_ref(stdlib.z3_context, lt);
            Z3_dec_ref(stdlib.z3_context, ge);
        }

        false
    }
}
