use std::{collections::{HashMap, VecDeque}, cell::RefCell, rc::Rc, fmt};

use crate::{values::{ActiveValue, bit_vector_concrete::BitVectorConcrete, bit_vector_symbol::BitVectorSymbol}, expressions::{bv_or_expression::BVOrExpression, bv_slice_expression::BVSliceExpression, bv_concat_expression::BVConcatExpression}, models::riscv::rv32i::ForkSink};

use super::{memory32::Memory32, MemoryRegion32};

pub struct VolatileMemoryRegion32 {
    pub start_address: u32,
    pub length: u32,
}

impl VolatileMemoryRegion32 {
    pub fn new(start_address: u32, length: u32) -> Self {
        VolatileMemoryRegion32 {
            start_address,
            length
        }
    }

    pub fn contains(&self, address: u32, width: u32) -> bool {
        address >= self.start_address && address + width <= self.start_address + self.length
    }
}

impl fmt::Debug for VolatileMemoryRegion32 {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(&format!("VolatileMemoryRegion32 {{ start_address: 0x{:x}, length: 0x{:x}}}", self.start_address, self.length))
    }
}

impl MemoryRegion32 for VolatileMemoryRegion32 {
    fn read(&mut self, address: u32, width: u32, stdlib: &mut crate::ScfiaStdlib, fork_sink: &mut Option<&mut ForkSink>,) -> Rc<RefCell<ActiveValue>> {
        let s = BitVectorSymbol::new(None, width, format!("<= 0x{:x}", address).into(), stdlib, fork_sink);
        // println!("<= 0x{:x} (volatile) yielding {}", address, s.try_borrow().unwrap().get_id());
        s
    }

    fn write(&mut self, _address: u32, _value: Rc<RefCell<ActiveValue>>, _width: u32, _stdlib: &mut crate::ScfiaStdlib, fork_sink: &mut Option<&mut ForkSink>) {
    }
}
