use std::collections::{BTreeMap, VecDeque};

use log::{debug, trace, warn};

use crate::{scfia::Scfia, values::active_value::ActiveValue, ForkSink, ScfiaComposition, StepContext};

#[derive(Debug)]
pub struct StableMemoryRegion<SC: ScfiaComposition> {
    pub memory: BTreeMap<u64, ActiveValue<SC>>,
    pub start_address: u64,
    pub length: u64,
}

#[derive(Clone, Debug)]
pub struct VolatileMemoryRegion {
    pub start_address: u64,
    pub length: u64,
}

#[derive(Debug)]
pub struct SymbolicVolatileMemoryRegion<SC: ScfiaComposition> {
    pub base_symbol: ActiveValue<SC>,
    pub length: u64,
}
impl<SC: ScfiaComposition> SymbolicVolatileMemoryRegion<SC> {
    pub(crate) fn clone_to(&self, cloned_scfia: Scfia<SC>) -> SymbolicVolatileMemoryRegion<SC> {
        SymbolicVolatileMemoryRegion {
            base_symbol: cloned_scfia
                .inner
                .try_borrow_mut()
                .unwrap()
                .active_symbols
                .get(&self.base_symbol.try_borrow().unwrap().id)
                .unwrap()
                .upgrade()
                .unwrap(),
            length: self.length,
        }
    }
}

impl<SC: ScfiaComposition> StableMemoryRegion<SC> {
    pub fn new(start_address: u64, length: u64) -> Self {
        StableMemoryRegion {
            memory: BTreeMap::default(),
            start_address,
            length,
        }
    }

    pub(crate) fn read(&self, address: u64, width: u32, scfia: Scfia<SC>, fork_sink: &mut Option<ForkSink<SC>>) -> ActiveValue<SC> {
        assert_eq!(width % 8, 0);
        let bytes = width / 8;
        let mut byte_values = VecDeque::new();
        for i in 0..bytes {
            if let Some(byte) = self.memory.get(&(address + i as u64)) {
                byte_values.push_back(byte.clone());
            } else {
                warn!("Reading from uninitialized 0x{:x}", address + i as u64);
                byte_values.push_back(scfia.new_bv_symbol(8, fork_sink))
            }
        }

        let mut value = byte_values.pop_front().unwrap();

        // Little endian
        while !byte_values.is_empty() {
            let rhs = byte_values.pop_front().unwrap();
            value = scfia.new_bv_concat(rhs, value, width - (byte_values.len() * 8) as u32, fork_sink);
        }

        value
    }

    pub(crate) fn write(&mut self, address: u64, value: ActiveValue<SC>, width: u32, scfia: Scfia<SC>, fork_sink: &mut Option<ForkSink<SC>>) {
        let bytes = width / 8;
        for byte in 0..bytes {
            let v = scfia.new_bv_slice(value.clone(), (byte * 8) + 7, byte * 8, fork_sink);
            debug!("*{:x} = {:?}", address, v);
            self.memory.insert(address + byte as u64, v);
        }
    }

    pub(crate) fn clone_to(&self, cloned_scfia: Scfia<SC>) -> StableMemoryRegion<SC> {
        let mut clone = StableMemoryRegion {
            length: self.length,
            memory: BTreeMap::new(),
            start_address: self.start_address,
        };

        for (offset, value) in &self.memory {
            trace!("memcloning {:?}", value);
            clone.memory.insert(
                *offset,
                cloned_scfia
                    .inner
                    .try_borrow_mut()
                    .unwrap()
                    .active_symbols
                    .get(&value.try_borrow().unwrap().id)
                    .unwrap()
                    .upgrade()
                    .unwrap()
                    .clone(),
            );
        }

        clone
    }
}
