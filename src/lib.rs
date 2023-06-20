use memory::Memory;
use scfia::Scfia;
use std::{fmt::Debug, rc::Weak};
use values::active_value::ActiveValue;

pub mod memory;
pub mod models;
pub mod scfia;
pub mod values;
pub mod z3_handle;

pub trait ScfiaComposition: Debug + Clone + Sized {
    type Model: Debug;
    type ForkSink: GenericForkSink<Self>;
}

pub trait GenericForkSink<SC: ScfiaComposition>: Debug {
    fn fork(&mut self, fork_symbol: ActiveValue<SC>);
    fn push_value(&mut self, value: ActiveValue<SC>);
}

struct StepContext<'a, SC: ScfiaComposition> {
    memory: *mut Memory<SC>,
    scfia: &'a Scfia<SC>,
    hints: Option<SymbolicHints>,
    fork_sink: Option<SC::ForkSink>,
}

#[derive(Clone)]
pub struct SymbolicHints {
    pub hints: Vec<Vec<u64>>,
}
