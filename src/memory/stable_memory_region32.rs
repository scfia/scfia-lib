use std::{collections::{HashMap, VecDeque, BTreeMap}, cell::RefCell, rc::Rc};

use crate::{values::{ActiveValue, bit_vector_concrete::BitVectorConcrete, bit_vector_symbol::BitVectorSymbol}, expressions::{bv_or_expression::BVOrExpression, bv_slice_expression::BVSliceExpression, bv_concat_expression::BVConcatExpression}, models::riscv::rv32i::ForkSink};

use super::{memory32::Memory32, MemoryRegion32};

#[derive(Debug)]
pub struct StableMemoryRegion32 {
    pub memory: BTreeMap<u32, Rc<RefCell<ActiveValue>>>,
    pub start_address: u32,
    pub length: u32,
}

impl StableMemoryRegion32 {
    pub fn new(start_address: u32, length: u32) -> Self {
        StableMemoryRegion32 {
            memory: BTreeMap::new(),
            start_address,
            length
        }
    }

    pub fn clone_to_stdlib(
        &self,
        cloned_active_values: &mut BTreeMap<u64, Rc<RefCell<ActiveValue>>>,
        cloned_retired_values: &mut BTreeMap<u64, Rc<RefCell<crate::values::RetiredValue>>>,
        cloned_stdlib: &mut crate::ScfiaStdlib
    ) -> Self {
        let mut cloned_region = StableMemoryRegion32::new(self.start_address, self.length);

        for (key, value) in self.memory.iter() {
            let cloned_value = value.try_borrow().unwrap().clone_to_stdlib(cloned_active_values, cloned_retired_values, cloned_stdlib);
            cloned_region.memory.insert(*key, cloned_value);
        }

        cloned_region
    }
}

impl MemoryRegion32 for StableMemoryRegion32 {
    fn read(&mut self, address: u32, width: u32, stdlib: &mut crate::ScfiaStdlib, fork_sink: &mut Option<&mut ForkSink>,) -> Rc<RefCell<ActiveValue>> {
        assert_eq!(width % 8, 0);
        // println!("<= 0x{:x} ({} bytes)", address, width/8);
        let bytes = width / 8;
        let mut byte_values = VecDeque::new();
        for i in 0..bytes {
            if let Some(byte) = self.memory.get(&(address + i)) {
                byte_values.push_back(byte.clone());
            } else {
                println!("Warning: Reading from uninitialized 0x{:x}", address + i);
                byte_values.push_back(BitVectorSymbol::new(None, 8, stdlib, fork_sink))
            }
        }

        let mut value = byte_values.pop_front().unwrap();

        // Little endian
        while byte_values.len() > 0 {
            let rhs = byte_values.pop_front().unwrap();
            value = BVConcatExpression::new(rhs, value, stdlib, fork_sink).into();
        }

        value
    }

    fn write(&mut self, address: u32, value: Rc<RefCell<ActiveValue>>, stdlib: &mut crate::ScfiaStdlib, fork_sink: &mut Option<&mut ForkSink>) {
        let value_ref = value.try_borrow().unwrap();
        match &*value_ref {
            ActiveValue::BitvectorConcrete(e) => {
                // Split concrete value into bytes
                // println!("0x{:x} => 0x{} ({} bytes)", address, e.value, e.width/8);
                assert_eq!(e.width % 8, 0);
                let bytes = e.width / 8;
                for byte in 0..bytes {
                    let v = BVSliceExpression::new(value.clone(), (byte * 8) + 7, byte * 8, stdlib, fork_sink);
                    self.memory.insert(address + byte, v.into());
                }
            },
            _ => unimplemented!("{:?}", value),
        }
    }
}
