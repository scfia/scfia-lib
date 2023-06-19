use std::collections::{BTreeMap, VecDeque};

use log::{debug, trace, warn};

use crate::{values::{active_value::{ActiveValue, ValueComment}, retired_value::RetiredValue}, ScfiaComposition, StepContext, scfia::Scfia};

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
    pub(crate) fn clone_to_stdlib(&self, cloned_scfia_rc: &Scfia<SC>, cloned_actives: &mut BTreeMap<u64, ActiveValue<SC>>, cloned_retired: &mut BTreeMap<u64, RetiredValue<SC>>) -> SymbolicVolatileMemoryRegion<SC> {
        /*
        SymbolicVolatileMemoryRegion {
            base_symbol: self
                .base_symbol
                .try_borrow()
                .unwrap()
                .clone_to_stdlib(&mut cloned_scfia_rc.inner.try_borrow_mut().unwrap(), cloned_scfia_rc.clone(), cloned_actives, cloned_retired),
            length: self.length,
        }
        */
        todo!()
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
        /*
        assert_eq!(width % 8, 0);
        let bytes = width / 8;
        let mut byte_values = VecDeque::new();
        let mut uninitialized = false;
        for i in 0..bytes {
            if let Some(byte) = self.memory.get(&(address + i as u64)) {
                byte_values.push_back(byte.clone());
            } else {
                warn!("Region {:#x} (len={:#x}) reading from uninitialized {:#x}", self.start_address, self.length, address + i as u64);
                uninitialized = true;
                byte_values.push_back(scfia.inner.try_borrow_mut().unwrap().new_bv_symbol(scfia.clone(), 8, None, fork_sink, Some(ValueComment::new(format!("Read from uninitialized address {:#x}", address + i as u64).to_string()))))
            }
        }

        // Little endian
        let mut value = byte_values.pop_front().unwrap();
        while !byte_values.is_empty() {
            let rhs = byte_values.pop_front().unwrap();
            let new_value = if uninitialized {
                scfia.inner.try_borrow_mut().unwrap().new_bv_concat(scfia.clone(), rhs.clone(), value.clone(), width - (byte_values.len() * 8) as u32, None, fork_sink, Some(ValueComment::new("StableMemmoryRegion::read concat".to_string())))
            } else {
                scfia.new_bv_concat(rhs.clone(), value.clone(), width - (byte_values.len() * 8) as u32, fork_sink)
            };
            // Seriously, fuck Rust's drop behaviour/lifetimes.
            // If we directly assign, the drop (of value's previous content) will be executed **before** the release of scfia.inner.try_borrow_mut().
            value = new_value;
        }

        //debug!("? = *{:x} ({:?})", address, value);

        value
        */
        todo!()
    }

    pub(crate) fn write(&mut self, address: u64, value: ActiveValue<SC>, width: u32, scfia: &Scfia<SC>, fork_sink: &mut Option<SC::ForkSink>) {
        /*
        assert_eq!(width % 8, 0);
        debug!("*{:x} = {:?}", address, value);
        let bytes = width / 8;
        for byte in 0..bytes {
            let v = scfia.new_bv_slice(value.clone(), (byte * 8) + 7, byte * 8, fork_sink);
            //debug!("*{:x} = {:?}", address, v);
            self.memory.insert(address + byte as u64, v);
        }
        */
    }

    pub(crate) fn clone_to_stdlib(&self, cloned_scfia_rc: &Scfia<SC>, cloned_actives: &mut BTreeMap<u64, ActiveValue<SC>>, cloned_retired: &mut BTreeMap<u64, RetiredValue<SC>>) -> StableMemoryRegion<SC> {
        /*
        let mut clone = StableMemoryRegion {
            length: self.length,
            memory: BTreeMap::new(),
            start_address: self.start_address,
        };

        for (offset, value) in &self.memory {
            trace!("memcloning {:?}", value);
            clone.memory.insert(
                *offset,
                value
                    .try_borrow()
                    .unwrap()
                    .clone_to_stdlib(&mut cloned_scfia_rc.inner.try_borrow_mut().unwrap(), cloned_scfia_rc.clone(), cloned_actives, cloned_retired),
            );
        }

        clone
        */
        todo!()
    }
}
