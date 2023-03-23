pub mod memory;
pub mod models;
pub mod scfia;
pub mod values;

#[derive(Clone)]
pub struct SymbolicHints {
    pub hints: Vec<Vec<u64>>,
}
