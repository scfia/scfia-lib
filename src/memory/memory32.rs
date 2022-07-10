use std::{rc::Rc, cell::RefCell, ops::Deref, collections::{HashMap, BTreeMap}};

use crate::{ScfiaStdlib, values::{bit_vector_concrete::BitVectorConcrete, ActiveValue, RetiredValue}, models::riscv::rv32i::ForkSink};
use crate::memory::MemoryRegion32;

use super::{stable_memory_region32::StableMemoryRegion32, volatile_memory_region::VolatileMemoryRegion32};

#[derive(Debug)]
pub struct Memory32 {
    pub stable_memory_regions: Vec<StableMemoryRegion32>,
    pub volatile_memory_regions: Vec<VolatileMemoryRegion32>,
}

impl Memory32 {
    pub fn new() -> Self {
        Memory32 {
            stable_memory_regions: vec![],
            volatile_memory_regions: vec![],
        }
    }

    pub fn read(&mut self, address: Rc<RefCell<ActiveValue>>, width: u32, stdlib: &mut ScfiaStdlib, fork_sink: &mut Option<&mut ForkSink>) -> Rc<RefCell<ActiveValue>> {
        let address = address.try_borrow().unwrap();
        match &*address {
            ActiveValue::BitvectorConcrete(e) => {
                self.read_concrete(e.value as u32, width, stdlib, fork_sink)
            },
            _ => panic!(),
        }
    }
    
    pub fn write(&mut self, address: Rc<RefCell<ActiveValue>>, value: Rc<RefCell<ActiveValue>>, stdlib: &mut ScfiaStdlib, fork_sink: &mut Option<&mut ForkSink>) {
        let address = address.try_borrow().unwrap();
        match &*address {
            ActiveValue::BitvectorConcrete(e) => {
                self.write_concrete(e.value as u32, value, stdlib, fork_sink)
            },
            _ => unimplemented!("{:?}", address),
        }
    }

    fn read_concrete(&mut self, address: u32, width: u32, stdlib: &mut ScfiaStdlib, fork_sink: &mut Option<&mut ForkSink>,) -> Rc<RefCell<ActiveValue>> {
        for region in &mut self.stable_memory_regions {
            if address >= region.start_address && address < region.start_address + region.length {
                return region.read(address, width, stdlib, fork_sink)
            }
        }

        for region in &mut self.volatile_memory_regions {
            if address >= region.start_address && address < region.start_address + region.length {
                return region.read(address, width, stdlib, fork_sink)
            }
        }

        panic!("0x{:x} {:?}", address, self.volatile_memory_regions);
    }

    fn write_concrete(&mut self, address: u32, value: Rc<RefCell<ActiveValue>>, stdlib: &mut ScfiaStdlib, fork_sink: &mut Option<&mut ForkSink>) {
        // TODO ensure read fits in region
        for region in &mut self.stable_memory_regions {
            if address >= region.start_address && address < region.start_address + region.length {
                return region.write(address, value, stdlib, fork_sink)
            }
        }

        for region in &mut self.volatile_memory_regions {
            if address >= region.start_address && address < region.start_address + region.length {
                return region.write(address, value, stdlib, fork_sink)
            }
        }
    }

    pub fn clone_to_stdlib(
        &self,
        cloned_active_values: &mut BTreeMap<u64, Rc<RefCell<ActiveValue>>>,
        cloned_retired_values: &mut BTreeMap<u64, Rc<RefCell<RetiredValue>>>,
        cloned_stdlib: &mut ScfiaStdlib
    ) -> Self {
        let mut cloned_memory = Self::new();

        for region in &self.stable_memory_regions {
            let cloned_region = region.clone_to_stdlib(cloned_active_values, cloned_retired_values, cloned_stdlib);
            cloned_memory.stable_memory_regions.push(cloned_region);
        }

        for region in &self.volatile_memory_regions {
            cloned_memory.volatile_memory_regions.push(VolatileMemoryRegion32 { start_address: region.start_address, length: region.length })
        }

        cloned_memory
    }
}
