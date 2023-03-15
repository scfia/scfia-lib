use crate::memory::symbolic_volatile_memory_region::SymbolicVolatileMemoryRegion32;
use crate::BitVectorSymbol;
use crate::SymbolicHints;
use crate::Z3_dec_ref;
use crate::Z3_inc_ref;
use crate::Z3_mk_unsigned_int64;
use crate::Z3_solver_check_assumptions;
use crate::Z3_L_FALSE;
use std::{
    cell::RefCell,
    collections::{BTreeMap, BTreeSet, HashMap},
    ops::Deref,
    rc::Rc,
};
use z3_sys::Z3_get_bv_sort_size;
use z3_sys::Z3_get_sort;
use z3_sys::Z3_mk_add;
use z3_sys::Z3_mk_bv_sort;
use z3_sys::Z3_mk_bvadd;
use z3_sys::Z3_mk_bvuge;
use z3_sys::Z3_mk_bvult;
use z3_sys::Z3_mk_eq;
use z3_sys::Z3_mk_or;

use crate::memory::MemoryRegion32;
use crate::{
    models::riscv::rv32i::ForkSink,
    values::{bit_vector_concrete::BitVectorConcrete, ActiveValue, RetiredValue},
    ScfiaStdlib,
};

use super::{stable_memory_region32::StableMemoryRegion32, volatile_memory_region::VolatileMemoryRegion32};

#[derive(Debug)]
pub struct Memory32 {
    pub stable_memory_regions: Vec<StableMemoryRegion32>,
    pub volatile_memory_regions: Vec<VolatileMemoryRegion32>,
    pub symbolic_volatile_memory_regions: Vec<SymbolicVolatileMemoryRegion32>,
}

impl Memory32 {
    pub fn new() -> Self {
        Memory32 {
            stable_memory_regions: vec![],
            volatile_memory_regions: vec![],
            symbolic_volatile_memory_regions: vec![],
        }
    }

    pub fn read(
        &mut self,
        address: Rc<RefCell<ActiveValue>>,
        width: u32,
        stdlib: &mut ScfiaStdlib,
        fork_sink: &mut Option<&mut ForkSink>,
        hints: &mut Option<SymbolicHints>,
    ) -> Rc<RefCell<ActiveValue>> {
        let address = address.try_borrow().unwrap();
        match &*address {
            ActiveValue::BitvectorConcrete(e) => self.read_concrete(e.value as u32, width, stdlib, fork_sink),
            x => self.read_symbolic(x, width, stdlib, fork_sink, hints),
        }
    }

    pub fn write(
        &mut self,
        address: Rc<RefCell<ActiveValue>>,
        value: Rc<RefCell<ActiveValue>>,
        width: u32,
        stdlib: &mut ScfiaStdlib,
        fork_sink: &mut Option<&mut ForkSink>,
        hints: &mut Option<SymbolicHints>,
    ) {
        let address = address.try_borrow().unwrap();
        match &*address {
            ActiveValue::BitvectorConcrete(e) => self.write_concrete(e.value as u32, value, width, stdlib, fork_sink),
            x => {
                // A symbolic write is occuring! These can be
                // - unanimous conrete writes, i.e. the symbol points to exactly one conrete memory location
                // - irrelevant writes, i.e. the symbol points to volatile memory locations only
                // - symbolic offset writes, i.e. the symbol points to a concrete offset of a symbolic region
                // Symbolic Offset Read
                println!(
                    "Attempting to perform Symbolic Offset Read through {} regions",
                    self.symbolic_volatile_memory_regions.len()
                );
                for symbolic_volatile_memory_region in &self.symbolic_volatile_memory_regions {
                    let symbolic_volatile_memory_region_base_ast = symbolic_volatile_memory_region.base_symbol.try_borrow().unwrap().get_z3_ast();
                    unsafe {
                        // address < base_address
                        let lt = Z3_mk_bvult(stdlib.z3_context, address.get_z3_ast(), symbolic_volatile_memory_region_base_ast);
                        Z3_inc_ref(stdlib.z3_context, lt);

                        // address >= base_address + length
                        let sort = Z3_mk_bv_sort(stdlib.z3_context, 32);
                        let add_ast = Z3_mk_unsigned_int64(stdlib.z3_context, symbolic_volatile_memory_region.length.into(), sort);
                        let ge = Z3_mk_bvuge(
                            stdlib.z3_context,
                            address.get_z3_ast(),
                            Z3_mk_bvadd(stdlib.z3_context, symbolic_volatile_memory_region_base_ast, add_ast),
                        );
                        Z3_inc_ref(stdlib.z3_context, ge);

                        let assumptions = [lt, ge];
                        println!("Z3_solver_check_assumptions");
                        let check_result = Z3_solver_check_assumptions(stdlib.z3_context, stdlib.z3_solver, 2, assumptions.as_ptr());
                        if check_result == Z3_L_FALSE {
                            // If the address CAN NOT be outside the symbolic volatile region, we can return a new BVS
                            println!("Symbolic Offset Write covered");
                            return;
                        } else {
                            println!("Z3_solver_check_assumptions returned {:?}", check_result);
                            println!("{:?}", Z3_solver_check_assumptions(stdlib.z3_context, stdlib.z3_solver, 1, [lt].as_ptr()));
                            println!("{:?}", Z3_solver_check_assumptions(stdlib.z3_context, stdlib.z3_solver, 1, [ge].as_ptr()));
                        }

                        Z3_dec_ref(stdlib.z3_context, lt);
                        Z3_dec_ref(stdlib.z3_context, ge);
                    }
                }

                let mut candidates = vec![];

                if let Some(hints) = hints {
                    if hints.hints.len() > 0 {
                        candidates = hints.hints.pop().unwrap();
                        println!("Using initial candiates {:x?}", candidates)
                    }
                }

                stdlib.monomorphize(x.get_z3_ast(), &mut candidates);
                println!("Write monomorphized addresses {:?}", &candidates);
                if candidates.len() == 1 {
                    // unanimous concrete write
                    let address = candidates.iter().next().unwrap();
                    self.write_concrete(*address as u32, value, width, stdlib, fork_sink)
                } else if self.is_volatile(&candidates, width) {
                    // irrelevant write
                } else {
                    panic!("Symbolic write is neither unanimous nor irrelevant{:x?}", candidates)
                }
            }
        }
    }

    fn read_concrete(&mut self, address: u32, width: u32, stdlib: &mut ScfiaStdlib, fork_sink: &mut Option<&mut ForkSink>) -> Rc<RefCell<ActiveValue>> {
        for region in &mut self.volatile_memory_regions {
            if address >= region.start_address && address < region.start_address + region.length {
                return region.read(address, width, stdlib, fork_sink);
            }
        }

        for region in &mut self.stable_memory_regions {
            if address >= region.start_address && address < region.start_address + region.length {
                return region.read(address, width, stdlib, fork_sink);
            }
        }

        panic!("0x{:x} {:?}", address, self.volatile_memory_regions);
    }

    fn read_symbolic(
        &mut self,
        address: &ActiveValue,
        width: u32,
        stdlib: &mut ScfiaStdlib,
        fork_sink: &mut Option<&mut ForkSink>,
        hints: &mut Option<SymbolicHints>,
    ) -> Rc<RefCell<ActiveValue>> {
        // A symbolic read is occuring! These can be
        // - unanimous address reads, i.e. the symbol points to exactly one memory location
        // - unanimous value reads, i.e. the symbol points to several memory locations which all contain the same value
        // - symbolic offset reads, i.e. the symbol points to a concrete offset of a symbolic region
        let hints = if let Some(hints) = hints { hints.hints.pop().unwrap() } else { vec![] };

        // Symbolic Offset Read
        println!(
            "Attempting to perform Symbolic Offset Read through {} regions",
            self.symbolic_volatile_memory_regions.len()
        );
        for symbolic_volatile_memory_region in &self.symbolic_volatile_memory_regions {
            let symbolic_volatile_memory_region_base_ast = symbolic_volatile_memory_region.base_symbol.try_borrow().unwrap().get_z3_ast();
            unsafe {
                // address < base_address
                let lt = Z3_mk_bvult(stdlib.z3_context, address.get_z3_ast(), symbolic_volatile_memory_region_base_ast);
                Z3_inc_ref(stdlib.z3_context, lt);

                // address >= base_address + length
                let sort = Z3_mk_bv_sort(stdlib.z3_context, 32);
                let add_ast = Z3_mk_unsigned_int64(stdlib.z3_context, symbolic_volatile_memory_region.length.into(), sort);
                let ge = Z3_mk_bvuge(
                    stdlib.z3_context,
                    address.get_z3_ast(),
                    Z3_mk_bvadd(stdlib.z3_context, symbolic_volatile_memory_region_base_ast, add_ast),
                );
                Z3_inc_ref(stdlib.z3_context, ge);
                let asts = [lt, ge];
                let assumption = Z3_mk_or(stdlib.z3_context, 2, asts.as_ptr());
                Z3_inc_ref(stdlib.z3_context, assumption);

                println!("Z3_solver_check_assumptions");
                let check_result = Z3_solver_check_assumptions(stdlib.z3_context, stdlib.z3_solver, 1, asts.as_ptr());
                if check_result == Z3_L_FALSE {
                    // If the address CAN NOT be outside the symbolic volatile region, we can return a new BVS
                    println!("Symbolic Offset Read returning new symbol");
                    return BitVectorSymbol::new(None, width, Some("<= [symbolic volatile region]".into()), stdlib, fork_sink);
                } else {
                    println!("Z3_solver_check_assumptions returned {:?}", check_result);
                    println!("{:?}", Z3_solver_check_assumptions(stdlib.z3_context, stdlib.z3_solver, 1, [lt].as_ptr()));
                    println!("{:?}", Z3_solver_check_assumptions(stdlib.z3_context, stdlib.z3_solver, 1, [ge].as_ptr()));
                }

                Z3_dec_ref(stdlib.z3_context, lt);
                Z3_dec_ref(stdlib.z3_context, ge);
                Z3_dec_ref(stdlib.z3_context, assumption);
            }
        }

        // Unanimous Read
        println!("Attempting to perform Unanimous Value Read");
        let mut addresses = hints.clone();
        stdlib.monomorphize(address.get_z3_ast(), &mut addresses);
        assert!(addresses.len() > 0);
        println!("Read monomorphized addresses {:?}", &addresses);
        let unanimous_address = addresses.windows(2).all(|w| w[0] == w[1]);
        if unanimous_address {
            // The addresses are unanimous
            println!("Read was unanimous");
            return self.read_concrete(addresses[0].try_into().unwrap(), width, stdlib, fork_sink);
        } else {
            // The addresses are not unanimous, but the values might still be
            let value = self.read_concrete(addresses[0].try_into().unwrap(), width, stdlib, fork_sink);
            for address in &addresses {
                let other_value = self.read_concrete((*address).try_into().unwrap(), width, stdlib, fork_sink);
                if !stdlib.are_equal(value.try_borrow().unwrap().get_z3_ast(), other_value.try_borrow().unwrap().get_z3_ast()) {
                    println!("*0x{:x} != *0x{:x}", addresses[0], address);
                    println!("lhs={:?}", value);
                    println!("rhs={:?}", other_value);
                    panic!("Could not resolve symbolic read");
                }
            }

            println!("Unamimous value read returning {:?}", value);
            return value;
        }
    }

    pub fn write_concrete(
        &mut self,
        address: u32,
        value: Rc<RefCell<ActiveValue>>,
        width: u32,
        stdlib: &mut ScfiaStdlib,
        fork_sink: &mut Option<&mut ForkSink>,
    ) {
        // TODO ensure write fits in region
        if address == 0x46000004 || address == 0x46000084 {
            println!("### write_concrete 0x{:x} = {:?} ({})", address, value, width)
        }

        for region in &mut self.stable_memory_regions {
            if address >= region.start_address && address < region.start_address + region.length {
                return region.write(address, value, width, stdlib, fork_sink);
            }
        }

        for region in &mut self.volatile_memory_regions {
            if address >= region.start_address && address < region.start_address + region.length {
                return region.write(address, value, width, stdlib, fork_sink);
            }
        }
    }

    fn is_volatile(&self, candidates: &[u64], width: u32) -> bool {
        for candidate in candidates {
            let mut covered = false;
            for volatile_region in &self.volatile_memory_regions {
                if volatile_region.contains(*candidate as u32, width) {
                    covered = true;
                    break;
                }
            }

            if !covered {
                println!("0x{:x} not covered by volatile regions", candidate);
                return false;
            }
        }

        true
    }

    pub fn clone_to_stdlib(
        &self,
        cloned_active_values: &mut BTreeMap<u64, Rc<RefCell<ActiveValue>>>,
        cloned_retired_values: &mut BTreeMap<u64, Rc<RefCell<RetiredValue>>>,
        cloned_stdlib: &mut ScfiaStdlib,
    ) -> Self {
        let mut cloned_memory = Self::new();

        for region in &self.stable_memory_regions {
            let cloned_region = region.clone_to_stdlib(cloned_active_values, cloned_retired_values, cloned_stdlib);
            cloned_memory.stable_memory_regions.push(cloned_region);
        }

        for region in &self.volatile_memory_regions {
            cloned_memory.volatile_memory_regions.push(VolatileMemoryRegion32 {
                start_address: region.start_address,
                length: region.length,
            })
        }

        for region in &self.symbolic_volatile_memory_regions {
            cloned_memory.symbolic_volatile_memory_regions.push(SymbolicVolatileMemoryRegion32 {
                base_symbol: region
                    .base_symbol
                    .try_borrow()
                    .unwrap()
                    .clone_to_stdlib(cloned_active_values, cloned_retired_values, cloned_stdlib),
                length: region.length,
            })
        }

        cloned_memory
    }
}
