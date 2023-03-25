use memory::Memory;
use scfia::Scfia;

pub mod memory;
pub mod models;
pub mod scfia;
pub mod values;

struct StepContext {
    memory: *mut Memory,
    scfia: Scfia,
    hints: Option<SymbolicHints>,
}

#[derive(Clone)]
pub struct SymbolicHints {
    pub hints: Vec<Vec<u64>>,
}
