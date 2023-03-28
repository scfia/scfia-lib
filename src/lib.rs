use memory::Memory;
use scfia::Scfia;
use values::active_value::ActiveValue;

pub mod memory;
pub mod models;
pub mod scfia;
pub mod values;

struct StepContext {
    memory: *mut Memory,
    scfia: Scfia,
    hints: Option<SymbolicHints>,
    fork_sink: Option<ForkSink>,
}

#[derive(Clone)]
pub struct SymbolicHints {
    pub hints: Vec<Vec<u64>>,
}

pub struct ForkSink {
    new_values_history: Vec<ActiveValue>,
}
