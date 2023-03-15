use std::{cell::RefCell, rc::Rc};

use crate::{models::riscv::rv32i::ForkSink, values::ActiveValue, ScfiaStdlib};

pub mod memory32;
pub mod stable_memory_region32;
pub mod symbolic_volatile_memory_region;
pub mod volatile_memory_region;

pub trait MemoryRegion32 {
    fn read(&mut self, address: u32, width: u32, stdlib: &mut ScfiaStdlib, fork_sink: &mut Option<&mut ForkSink>) -> Rc<RefCell<ActiveValue>>;
    fn write(&mut self, address: u32, value: Rc<RefCell<ActiveValue>>, width: u32, stdlib: &mut ScfiaStdlib, fork_sink: &mut Option<&mut ForkSink>);
}
