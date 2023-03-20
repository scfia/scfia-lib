use std::collections::{BTreeMap, VecDeque};

use log::warn;

use crate::{scfia::Scfia, values::active_value::ActiveValue};

#[derive(Debug)]
pub struct StableMemoryRegion {
    pub memory: BTreeMap<u64, ActiveValue>,
    pub start_address: u64,
    pub length: u64,
}

#[derive(Debug)]
pub struct VolatileMemoryRegion {
    pub start_address: u64,
    pub length: u64,
}

#[derive(Debug)]
pub struct SymbolicVolatileMemoryRegion {
    pub base_symbol: ActiveValue,
    pub length: u64,
}

impl StableMemoryRegion {
    pub fn new(start_address: u64, length: u64) -> Self {
        StableMemoryRegion {
            memory: BTreeMap::default(),
            start_address,
            length,
        }
    }

    pub(crate) fn read(&self, address: u64, width: u32, scfia: Scfia) -> ActiveValue {
        assert_eq!(width % 8, 0);
        let bytes = width / 8;
        let mut byte_values = VecDeque::new();
        for i in 0..bytes {
            if let Some(byte) = self.memory.get(&(address + i as u64)) {
                byte_values.push_back(byte.clone());
            } else {
                warn!("Reading from uninitialized 0x{:x}", address + i as u64);
                byte_values.push_back(scfia.new_bv_symbol(8))
            }
        }

        let mut value = byte_values.pop_front().unwrap();

        // Little endian
        while !byte_values.is_empty() {
            let rhs = byte_values.pop_front().unwrap();
            value = scfia.new_bv_concat(rhs, value, width - (byte_values.len() * 8) as u32);
        }

        value
    }

    pub(crate) fn write(&mut self, address: u64, value: ActiveValue, width: u32, scfia: Scfia) {
        let bytes = width / 8;
        for byte in 0..bytes {
            let v = scfia.new_bv_slice(value.clone(), (byte * 8) + 7, byte * 8);
            self.memory.insert(address + byte as u64, v);
        }
    }
}
