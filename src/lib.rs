use memory::Memory;
use scfia::Scfia;
use std::fmt::Debug;
use values::active_value::ActiveValue;

pub mod memory;
pub mod models;
pub mod scfia;
pub mod values;

struct StepContext<SC: ScfiaComposition> {
    memory: *mut Memory<SC>,
    scfia: Scfia<SC>,
    hints: Option<SymbolicHints>,
    fork_sink: Option<ForkSink<SC>>,
}

#[derive(Clone)]
pub struct SymbolicHints {
    pub hints: Vec<Vec<u64>>,
}

pub struct ForkSink<SC: ScfiaComposition> {
    new_values_history: Vec<ActiveValue<SC>>,
    forks: Vec<SC>,
}
impl<SC: ScfiaComposition> ForkSink<SC> {
    fn fork(&self, fork_symbol: ActiveValue<SC>) {
        todo!()
    }
}

pub trait ScfiaComposition: Clone + Debug {
    type Model: Debug;
    type ForkSink: Debug;
}
