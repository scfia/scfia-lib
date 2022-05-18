use std::{rc::Rc, cell::RefCell, ops::Deref, collections::HashMap};

use crate::{traits::{bit_vector::BitVector, ast::Ast}, ScfiaStdlib, values::{bit_vector_concrete::BitVectorConcrete, ActiveValue, RetiredValue}};
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

    pub fn read(&mut self, address: Rc<RefCell<ActiveValue>>, width: u32, stdlib: &mut ScfiaStdlib) -> Rc<RefCell<ActiveValue>> {
        let address = address.try_borrow().unwrap();
        match &*address {
            ActiveValue::BitvectorConcrete(e) => {
                self.read_concrete(e.value as u32, width, stdlib)
            },
            _ => panic!(),
        }
    }
    
    pub fn write(&mut self, address: Rc<RefCell<ActiveValue>>, value: Rc<RefCell<ActiveValue>>, stdlib: &mut ScfiaStdlib) {
        let address = address.try_borrow().unwrap();
        match &*address {
            ActiveValue::BitvectorConcrete(e) => {
                self.write_concrete(e.value as u32, value, stdlib)
            },
            _ => panic!(),
        }
    }

    fn read_concrete(&mut self, address: u32, width: u32, stdlib: &mut ScfiaStdlib) -> Rc<RefCell<ActiveValue>> {
        for region in &mut self.stable_memory_regions {
            if address >= region.start_address && address < region.start_address + region.length {
                return region.read(address, width, stdlib)
            }
        }

        for region in &mut self.volatile_memory_regions {
            if address >= region.start_address && address < region.start_address + region.length {
                return region.read(address, width, stdlib)
            }
        }

        panic!("0x{:x} {:?}", address, self.volatile_memory_regions);
    }

    fn write_concrete(&mut self, address: u32, value: Rc<RefCell<ActiveValue>>, stdlib: &mut ScfiaStdlib) {
        for region in &mut self.stable_memory_regions {
            if address >= region.start_address && address < region.start_address + region.length {
                return region.write(address, value, stdlib)
            }
        }

        for region in &mut self.volatile_memory_regions {
            if address >= region.start_address && address < region.start_address + region.length {
                return region.write(address, value, stdlib)
            }
        }
    }

    pub fn clone_to_stdlib(
        &self,
        cloned_active_values: &mut HashMap<u64, Rc<RefCell<ActiveValue>>>,
        cloned_retired_values: &mut HashMap<u64, Rc<RefCell<RetiredValue>>>,
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
