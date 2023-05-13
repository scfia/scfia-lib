use memory::Memory;
use scfia::Scfia;
use std::fmt::Debug;
use values::active_value::ActiveValue;

pub mod memory;
pub mod models;
pub mod scfia;
pub mod values;

pub trait ScfiaComposition: Debug + Clone + Sized {
    type Model: Debug;
    type ForkSink: GenericForkSink<Self>;
}

pub trait GenericForkSink<SC: ScfiaComposition>: Debug {
    fn fork(&self, fork_symbol: ActiveValue<SC>);
    fn push_value(&mut self, value: ActiveValue<SC>);
}

struct StepContext<SC: ScfiaComposition> {
    memory: *mut Memory<SC>,
    scfia: Scfia<SC>,
    hints: Option<SymbolicHints>,
    fork_sink: Option<SC::ForkSink>,
}

#[derive(Clone)]
pub struct SymbolicHints {
    pub hints: Vec<Vec<u64>>,
}
