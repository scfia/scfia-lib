pub mod regions;

use log::{error, trace};
use z3_sys::{
    Z3_dec_ref, Z3_inc_ref, Z3_mk_bv_sort, Z3_mk_bvadd, Z3_mk_bvuge, Z3_mk_bvult, Z3_mk_eq, Z3_mk_not, Z3_mk_or, Z3_mk_unsigned_int64,
    Z3_solver_check_assumptions, Z3_L_FALSE,
};

use crate::{
    scfia::Scfia,
    values::active_value::{ActiveExpression, ActiveValue, ActiveValueInner},
    ForkSink, ScfiaComposition, StepContext, SymbolicHints,
};

use self::regions::{StableMemoryRegion, SymbolicVolatileMemoryRegion, VolatileMemoryRegion};

pub struct Memory<SC: ScfiaComposition> {
    pub stables: Vec<StableMemoryRegion<SC>>,
    pub volatiles: Vec<VolatileMemoryRegion>,
    pub symbolic_volatiles: Vec<SymbolicVolatileMemoryRegion<SC>>,
}

impl<SC: ScfiaComposition> Memory<SC> {
    pub fn new() -> Self {
        Memory {
            stables: vec![],
            volatiles: vec![],
            symbolic_volatiles: vec![],
        }
    }

    pub fn read(
        &mut self,
        address: ActiveValue<SC>,
        width: u32,
        scfia: Scfia<SC>,
        hints: &mut Option<SymbolicHints>,
        fork_sink: &mut Option<ForkSink<SC>>,
    ) -> ActiveValue<SC> {
        let address_inner = address.try_borrow().unwrap();
        if let ActiveExpression::BVConcrete(e) = &address_inner.expression {
            self.read_concrete(e.value, width, scfia, fork_sink)
        } else {
            self.read_symbolic(&address_inner, width, scfia, hints, fork_sink)
        }
    }

    pub fn write(
        &mut self,
        address: ActiveValue<SC>,
        value: ActiveValue<SC>,
        width: u32,
        scfia: Scfia<SC>,
        hints: &mut Option<SymbolicHints>,
        fork_sink: &mut Option<ForkSink<SC>>,
    ) {
        let address_inner = address.try_borrow().unwrap();
        if let ActiveExpression::BVConcrete(e) = &address_inner.expression {
            self.write_concrete(e.value, value, width, scfia, fork_sink)
        } else {
            self.write_symbolic(&address_inner, value, width)
        }
    }

    fn read_symbolic(
        &mut self,
        address: &ActiveValueInner<SC>,
        width: u32,
        scfia: Scfia<SC>,
        hints: &mut Option<SymbolicHints>,
        fork_sink: &mut Option<ForkSink<SC>>,
    ) -> ActiveValue<SC> {
        unsafe {
            // Symbolic reads can be symbolic volatile region reads or unanimous reads
            let z3_context = scfia.inner.try_borrow().unwrap().z3_context;
            let z3_solver = scfia.inner.try_borrow().unwrap().z3_solver;

            // Check for symbolic volatile region read
            for region in &self.symbolic_volatiles {
                // TODO refcounting for the intermediate refs
                let base_ast = region.base_symbol.try_borrow().unwrap().z3_ast;
                // address < base_address
                let lt = Z3_mk_bvult(z3_context, address.z3_ast, base_ast);
                Z3_inc_ref(z3_context, lt);

                // address >= base_address + length
                let sort = Z3_mk_bv_sort(z3_context, 32);
                let add_ast = Z3_mk_unsigned_int64(z3_context, region.length, sort);
                let ge = Z3_mk_bvuge(z3_context, address.z3_ast, Z3_mk_bvadd(z3_context, base_ast, add_ast));
                Z3_inc_ref(z3_context, ge);
                let asts = [lt, ge];
                let assumption = Z3_mk_or(z3_context, 2, asts.as_ptr());
                Z3_inc_ref(z3_context, assumption);

                let check_result = Z3_solver_check_assumptions(z3_context, z3_solver, 1, asts.as_ptr());
                if check_result == Z3_L_FALSE {
                    // If the address CAN NOT be outside the symbolic volatile region, we can return a new BVS
                    trace!("Symbolic volatile region returning new symbol");
                    return scfia.new_bv_symbol(width, fork_sink);
                }

                Z3_dec_ref(z3_context, lt);
                Z3_dec_ref(z3_context, ge);
                Z3_dec_ref(z3_context, assumption);
            }
            // Check for unamimous read
            let mut candidates = if let Some(hints) = hints { (*hints).hints.pop().unwrap() } else { vec![] };
            scfia.monomorphize_active(address, &mut candidates);
            let unanimous_address = candidates.windows(2).all(|w| w[0] == w[1]);
            assert!(!candidates.is_empty());
            if unanimous_address {
                self.read_concrete(candidates[0], width, scfia, fork_sink)
            } else {
                // The addresses are not unanimous, but the values might still be
                // TODO refcounting for the intermediate refs
                let value = self.read_concrete(candidates[0], width, scfia.clone(), fork_sink);
                for address in &candidates {
                    let other_value = self.read_concrete(*address, width, scfia.clone(), fork_sink);
                    let assumptions = vec![Z3_mk_not(
                        z3_context,
                        Z3_mk_eq(z3_context, value.try_borrow().unwrap().z3_ast, other_value.try_borrow().unwrap().z3_ast),
                    )];
                    if !Z3_solver_check_assumptions(z3_context, z3_solver, 1, assumptions.as_ptr()) == Z3_L_FALSE {
                        error!("Unequal values behind unimous read: *0x{:x} != *0x{:x}", candidates[0], address);
                        panic!("Could not resolve symbolic read: {:?} != {:?}", value, other_value);
                    }
                }

                println!("Unamimous value read returning {:?}", value);
                // TODO we must return a fresh symbol/value with the same monomorphization here, not just a random one!
                value
            }
        }
    }

    fn write_symbolic(&mut self, _address: &ActiveValueInner<SC>, _value: ActiveValue<SC>, _width: u32) {
        todo!()
    }

    fn read_concrete(&mut self, address: u64, width: u32, scfia: Scfia<SC>, fork_sink: &mut Option<ForkSink<SC>>) -> ActiveValue<SC> {
        for region in &self.stables {
            if address >= region.start_address && address < region.start_address + region.length {
                return region.read(address, width, scfia, fork_sink);
            }
        }
        for region in &self.volatiles {
            if address >= region.start_address && address < region.start_address + region.length {
                trace!("Volatile region 0x{:x} yielding fresh symbol", region.start_address);
                return scfia.new_bv_symbol(width, fork_sink);
            }
        }
        panic!("read_concrete failed to resolve 0x{:x}", address)
    }

    fn write_concrete(&mut self, address: u64, value: ActiveValue<SC>, width: u32, scfia: Scfia<SC>, fork_sink: &mut Option<ForkSink<SC>>) {
        for region in &mut self.stables {
            if address >= region.start_address && address < region.start_address + region.length {
                return region.write(address, value, width, scfia.clone(), fork_sink);
            }
        }
        for region in &self.volatiles {
            if address >= region.start_address && address < region.start_address + region.length {
                trace!("Volatile region 0x{:x} ignoring write", region.start_address);
                return;
            }
        }

        panic!("{:?}", address)
    }

    pub(crate) fn clone_to(&self, cloned_scfia: Scfia<SC>) -> Memory<SC> {
        let mut cloned_stables = vec![];
        let mut symbolic_volatiles = vec![];

        for stable in &self.stables {
            cloned_stables.push(stable.clone_to(cloned_scfia.clone()))
        }

        for symbolic_volatile in &self.symbolic_volatiles {
            symbolic_volatiles.push(symbolic_volatile.clone_to(cloned_scfia.clone()))
        }

        Memory {
            stables: cloned_stables,
            volatiles: self.volatiles.clone(),
            symbolic_volatiles,
        }
    }
}

impl<SC: ScfiaComposition> Default for Memory<SC> {
    fn default() -> Self {
        Self::new()
    }
}
