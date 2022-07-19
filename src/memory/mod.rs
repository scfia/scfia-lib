use std::{cell::RefCell, rc::Rc};

use crate::{values::ActiveValue, ScfiaStdlib, models::riscv::rv32i::ForkSink};

pub mod memory32;
pub mod stable_memory_region32;
pub mod volatile_memory_region;

pub trait MemoryRegion32 {
    fn read(&mut self, address: u32, width: u32, stdlib: &mut ScfiaStdlib, fork_sink: &mut Option<&mut ForkSink>,) -> Rc<RefCell<ActiveValue>>;
    fn write(&mut self, address: u32, value: Rc<RefCell<ActiveValue>>, width: u32, stdlib: &mut ScfiaStdlib, fork_sink: &mut Option<&mut ForkSink>,);
}
