use std::collections::{BTreeMap, VecDeque};

use log::{debug, trace, warn};

use crate::{
    scfia::Scfia,
    values::{
        active_value::{ActiveValue, ValueComment},
        retired_value::RetiredValue,
    },
    ScfiaComposition,
};

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
    pub(crate) fn clone_to_stdlib(
        &self,
        cloned_scfia: &Scfia<SC>,
        cloned_actives: &mut BTreeMap<u64, ActiveValue<SC>>,
        cloned_retired: &mut BTreeMap<u64, RetiredValue<SC>>,
    ) -> SymbolicVolatileMemoryRegion<SC> {
        SymbolicVolatileMemoryRegion {
            base_symbol: self
                .base_symbol
                .clone_to_stdlib(cloned_scfia, cloned_actives, cloned_retired),
            length: self.length,
        }
    }
}

impl<SC: ScfiaComposition> StableMemoryRegion<SC> {
    pub fn new(start_address: u64, length: u64) -> Self {
        debug!("StableMemoryRegion(start_address={:#x}, {:#x})", start_address, length);
        StableMemoryRegion {
            memory: BTreeMap::default(),
            start_address,
            length,
        }
    }

    pub(crate) fn read(&self, address: u64, width: u32, scfia: &Scfia<SC>, fork_sink: &mut Option<SC::ForkSink>) -> ActiveValue<SC> {
        assert_eq!(width % 8, 0);
        let bytes = width / 8;
        let mut byte_values = VecDeque::new();
        let mut uninitialized = false;
        for i in 0..bytes {
            if let Some(byte) = self.memory.get(&(address + i as u64)) {
                byte_values.push_back(byte.clone());
            } else {
                warn!(
                    "Region {:#x} (len={:#x}) reading from uninitialized {:#x}",
                    self.start_address,
                    self.length,
                    address + i as u64
                );
                uninitialized = true;
                byte_values.push_back(scfia.new_bv_symbol(
                    8,
                    None,
                    fork_sink,
                    Some(ValueComment::new(
                        format!("Read from uninitialized address {:#x}", address + i as u64).to_string(),
                    )),
                ))
            }
        }

        // Little endian
        let mut value = byte_values.pop_front().unwrap();
        while !byte_values.is_empty() {
            let rhs = byte_values.pop_front().unwrap();
            value = if uninitialized {
                scfia.new_bv_concat(
                    &rhs,
                    &value,
                    width - (byte_values.len() * 8) as u32,
                    None,
                    fork_sink,
                    Some(ValueComment::new("StableMemmoryRegion::read concat".to_string())),
                )
            } else {
                scfia.new_bv_concat(&rhs, &value, width - (byte_values.len() * 8) as u32, None, fork_sink, None)
            };
        }

        trace!("? = *{:x} ({:?})", address, value);
        value
    }

    pub(crate) fn write(&mut self, address: u64, value: &ActiveValue<SC>, width: u32, scfia: &Scfia<SC>, fork_sink: &mut Option<SC::ForkSink>) {
        assert_eq!(width % 8, 0);
        trace!("*{:x} = {:?}", address, value);
        let bytes = width / 8;
        for byte in 0..bytes {
            let v = scfia.new_bv_slice(value, (byte * 8) + 7, byte * 8, None, fork_sink, None);
            trace!("*{:x} = {:?}", address, v);
            self.memory.insert(address + byte as u64, v);
        }
    }

    pub(crate) fn clone_to_stdlib(
        &self,
        cloned_scfia: &Scfia<SC>,
        cloned_actives: &mut BTreeMap<u64, ActiveValue<SC>>,
        cloned_retired: &mut BTreeMap<u64, RetiredValue<SC>>,
    ) -> StableMemoryRegion<SC> {
        let mut clone = StableMemoryRegion {
            length: self.length,
            memory: BTreeMap::new(),
            start_address: self.start_address,
        };

        for (offset, value) in &self.memory {
            trace!("memcloning {:?}", value);
            clone.memory.insert(
                *offset,
                value.clone_to_stdlib(cloned_scfia, cloned_actives, cloned_retired),
            );
        }

        clone
    }
}
