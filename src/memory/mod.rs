use crate::{values::active_value::ActiveValue, SymbolicHints};

pub struct Memory32 {}

impl Memory32 {
    pub fn read(&mut self, _address: ActiveValue, _width: u32, _hints: &mut Option<SymbolicHints>) -> ActiveValue {
        todo!()
    }

    pub fn write(&mut self, _address: ActiveValue, _value: ActiveValue, _width: u32, _hints: &mut Option<SymbolicHints>) {
        todo!()
    }
}
