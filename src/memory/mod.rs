pub mod regions;

use std::{collections::BTreeMap, time::Instant};

use log::{debug, error, trace};
use z3_sys::Z3_L_FALSE;

use crate::{
    scfia::Scfia,
    values::{
        active_value::{ActiveValue, ActiveValueZ3},
        retired_value::RetiredValue,
    },
    ScfiaComposition, SymbolicHints,
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

    pub fn get_highest_depth(&self) -> Option<(u64, usize)> {
        todo!()
        /* 
        let mut highest = None;
        for stable in &self.stables {
            for (address, value) in &stable.memory {
                if let Some((_, highest_depth)) = highest {
                    if value.get_depth() > highest_depth {
                        highest = Some((*address, value.try_borrow().unwrap().get_depth()))
                    }
                } else {
                    highest = Some((*address, value.try_borrow().unwrap().get_depth()))
                }
            }
        }
        highest
        */
    }

    pub fn read(
        &mut self,
        address: &ActiveValue<SC>,
        width: u32,
        scfia: &Scfia<SC>,
        hints: &mut Option<SymbolicHints>,
        fork_sink: &mut Option<SC::ForkSink>,
    ) -> ActiveValue<SC> {
        match address {
            ActiveValue::BoolConcrete(_) => panic!(),
            ActiveValue::BVConcrete(address, _) => self.read_concrete(*address, width, scfia, fork_sink),
            ActiveValue::Expression(e) => self.read_symbolic(&e.try_borrow().unwrap(), width, scfia, hints, fork_sink)
        }
    }

    pub fn write(
        &mut self,
        address: &ActiveValue<SC>,
        value: &ActiveValue<SC>,
        width: u32,
        scfia: &Scfia<SC>,
        hints: &mut Option<SymbolicHints>,
        fork_sink: &mut Option<SC::ForkSink>,
    ) {
        match address {
            ActiveValue::BoolConcrete(_) => panic!(),
            ActiveValue::BVConcrete(address, width) => self.write_concrete(*address, value, *width, scfia, fork_sink),
            ActiveValue::Expression(e) => self.write_symbolic(&e.try_borrow().unwrap(), value, width, scfia, hints)
        }
    }

    fn read_symbolic(
        &mut self,
        address: &ActiveValueZ3<SC>,
        width: u32,
        scfia: &Scfia<SC>,
        hints: &mut Option<SymbolicHints>,
        fork_sink: &mut Option<SC::ForkSink>,
    ) -> ActiveValue<SC> {
        // Symbolic reads can be symbolic volatile region reads or unanimous reads.
        // Check for symbolic volatile region read:
        for region in &self.symbolic_volatiles {
            let begin = Instant::now();
            let base_ast = region.base_symbol.get_z3_ast().clone();
            // address < base_address
            let lt = scfia.z3.new_bvult(&address.z3_ast, &base_ast, false);

            // address >= base_address + length
            let length_ast = scfia.z3.new_bv_concrete(region.length, 32); // TODO don't hardcode 32
            let add_ast = scfia.z3.new_bvadd(&base_ast, &length_ast);
            let ge = scfia.z3.new_bvuge(&address.z3_ast, &add_ast, false);
            let assumption = scfia.z3.new_or(&lt, &ge);

            debug!("checking symbolic offset read assumptions");
            let check_result = scfia.z3.check_assumptions(&[&assumption]);
            if check_result == Z3_L_FALSE {
                // If the address CAN NOT be outside the symbolic volatile region, we can return a new BVS
                debug!("Symbolic volatile region returning new symbol after {}ms", begin.elapsed().as_millis());
                return scfia.new_bv_symbol(width, None, fork_sink, None);
            }
        }
        // Check for unamimous read
        let mut candidates = if let Some(hints) = hints { hints.hints.pop().unwrap() } else { vec![] };
        scfia.z3.monomorphize(&address.z3_ast, &mut candidates);
        let unanimous_address = candidates.windows(2).all(|w| w[0] == w[1]);
        assert!(!candidates.is_empty());
        if unanimous_address {
            self.read_concrete(candidates[0], width, scfia, fork_sink)
        } else {
            // The addresses are not unanimous, but the values might still be
            let value = self.read_concrete(candidates[0], width, scfia, fork_sink);
            for address in &candidates {
                let other_value = self.read_concrete(*address, width, scfia, fork_sink);
                let eq = scfia
                    .z3
                    .new_eq(&value.get_z3_ast(), &other_value.get_z3_ast(), false);
                let not = scfia.z3.new_not(&eq, false);
                if scfia.z3.check_assumptions(&[&not]) != Z3_L_FALSE {
                    error!("Unequal values behind unimous read: *0x{:x} != *0x{:x}", candidates[0], address);
                    panic!("Could not resolve symbolic read: {:?} != {:?}", value, other_value);
                };
            }

            println!("Unamimous value read returning {:?}", value);
            // TODO we must return a fresh symbol/value with the same monomorphization here, not just a random one!
            value
        }
    }

    fn write_symbolic(&mut self, address: &ActiveValueZ3<SC>, _value: &ActiveValue<SC>, width: u32, scfia: &Scfia<SC>, hints: &mut Option<SymbolicHints>) {
        debug!("write_symbolic");
        // Symbolic writes can be symbolic volatile region writes or unanimous writes

        // Check for symbolic volatile region write
        for region in &self.symbolic_volatiles {
            let region_base = region.base_symbol.get_z3_ast();
            // address < base_address
            let lt = scfia.z3.new_bvult(&address.z3_ast, &region_base, false);

            // address >= base_address + length
            let region_length = scfia.z3.new_bv_concrete(region.length, 32);
            let region_end = scfia.z3.new_bvadd(&region_base, &region_length);
            let ge = scfia.z3.new_bvult(&region_end, &address.z3_ast, false);

            let assumptions = [&lt, &ge];
            debug!("checking symbolic offset write assumptions");
            if scfia.z3.check_assumptions(&assumptions) == Z3_L_FALSE {
                // If the address CAN NOT be outside the symbolic volatile region, we can skip the write
                debug!("Symbolic offset write covered");
                return;
            }
        }

        let mut candidates = if let Some(hints) = hints { hints.hints.pop().unwrap() } else { vec![] };
        scfia.z3.monomorphize(&address.z3_ast, &mut candidates);
        let _unanimous_address = candidates.windows(2).all(|w| w[0] == w[1]);
        assert!(!candidates.is_empty());
        if candidates.len() == 1 {
            // unanimous concrete write
            todo!()
        } else if self.is_volatile(&candidates, width) {
            // Irrelevant write, all candidates point to volatile memory
            debug!("Irrelevant write covered");
        } else {
            panic!("Symbolic write is neither unanimous nor irrelevant{:x?}", candidates)
        }
    }

    fn read_concrete(&mut self, address: u64, width: u32, scfia: &Scfia<SC>, fork_sink: &mut Option<SC::ForkSink>) -> ActiveValue<SC> {
        // Volatile regions may be inside larger stable regions, so we check them first
        //debug!("*{:x}", address);
        for region in &self.volatiles {
            if address >= region.start_address && address < region.start_address + region.length {
                trace!("Volatile region 0x{:x} yielding fresh symbol", region.start_address);
                return scfia.new_bv_symbol(width, None, fork_sink, None);
            }
        }
        for region in &self.stables {
            if address >= region.start_address && address < region.start_address + region.length {
                return region.read(address, width, scfia, fork_sink);
            }
        }
        panic!("read_concrete failed to resolve 0x{:x}", address)
    }

    pub fn write_concrete(&mut self, address: u64, value: &ActiveValue<SC>, width: u32, scfia: &Scfia<SC>, fork_sink: &mut Option<SC::ForkSink>) {
        //debug!("*{:x} = {:?}", address, value);
        for region in &mut self.stables {
            if address >= region.start_address && address < region.start_address + region.length {
                // TODO add width
                return region.write(address, value, width, scfia, fork_sink);
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

    pub(crate) fn clone_to_stdlib(
        &self,
        cloned_scfia: &Scfia<SC>,
        cloned_actives: &mut BTreeMap<u64, ActiveValue<SC>>,
        cloned_retired: &mut BTreeMap<u64, RetiredValue<SC>>,
    ) -> Memory<SC> {
        let mut cloned_stables = vec![];
        let mut symbolic_volatiles = vec![];

        for stable in &self.stables {
            cloned_stables.push(stable.clone_to_stdlib(cloned_scfia, cloned_actives, cloned_retired))
        }

        for symbolic_volatile in &self.symbolic_volatiles {
            symbolic_volatiles.push(symbolic_volatile.clone_to_stdlib(cloned_scfia, cloned_actives, cloned_retired))
        }

        Memory {
            stables: cloned_stables,
            volatiles: self.volatiles.clone(),
            symbolic_volatiles,
        }
    }

    fn is_volatile(&self, candidates: &[u64], width: u32) -> bool {
        for candidate in candidates {
            let mut covered = false;
            for volatile_region in &self.volatiles {
                if *candidate >= volatile_region.start_address && *candidate + width as u64 / 8 < volatile_region.start_address + volatile_region.length {
                    covered = true;
                    break;
                }
            }

            if !covered {
                debug!("0x{:x} not covered by volatile regions", candidate);
                return false;
            }
        }

        true
    }
}

impl<SC: ScfiaComposition> Default for Memory<SC> {
    fn default() -> Self {
        Self::new()
    }
}
