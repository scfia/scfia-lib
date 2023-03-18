use std::{cell::RefCell, rc::{Rc, Weak}, collections::HashMap};

use crate::memory::memory32::Memory32;

pub struct Test {
    pub state: SystemState,
    pub memory: NewMemory,
    pub symbol_manager: Rc<RefCell<SymbolManager>>,
}

pub struct NewMemory {}

impl NewMemory {
    pub fn foo(&mut self, sm: &mut SymbolManager) {
        panic!()
    }
}

pub struct SystemState {
    pub pc: Rc<RefCell<NewActiveValue>>,
}

pub struct SymbolManager {
    next_symbol_id: u64,
    active_symbols: HashMap<u64, Weak<RefCell<SymbolManager>>>,
}

impl SymbolManager {
    pub fn drop_active_value(&mut self, value: &NewActiveValue) {
        assert!(self.active_symbols.remove(&value.get_id()).is_some())
    }
}

pub struct NewBitVectorConcrete {}

pub enum NewActiveValue {
    NewBitVectorConcrete(Rc<RefCell<SymbolManager>>, NewBitVectorConcrete),
}

impl NewActiveValue {
    fn get_symbol_manager(&self) -> Rc<RefCell<SymbolManager>> {
        match self {
            NewActiveValue::NewBitVectorConcrete(sm, _) => sm.clone(),
        }
    }

    fn get_id(&self) -> u64 {
        42
    }
}

impl Drop for NewActiveValue {
    fn drop(&mut self) {
        self.get_symbol_manager().try_borrow_mut().unwrap().drop_active_value(self)
    }
}

impl Test {
    fn foo() {
        
    }
}

impl Test {
    pub fn step(&mut self) {
        unsafe {
            self.memory.foo(&mut self.symbol_manager.try_borrow_mut().unwrap());
            self.state.pc = Rc::new(RefCell::new(NewActiveValue::NewBitVectorConcrete(
                self.symbol_manager.clone(),
                NewBitVectorConcrete {},
            )));
            Test::other_stuff(self, Test::do_stuff(self))
        }
    }

    pub unsafe fn do_stuff(selff: *mut Test) -> Rc<RefCell<NewActiveValue>> {
        (*selff).state.pc = Rc::new(RefCell::new(NewActiveValue::NewBitVectorConcrete(
            (*selff).symbol_manager.clone(),
            NewBitVectorConcrete {},
        )));
        Rc::new(RefCell::new(NewActiveValue::NewBitVectorConcrete(
            (*selff).symbol_manager.clone(),
            NewBitVectorConcrete {},
        )))
    }

    pub unsafe fn other_stuff(selff: *mut Test, some_value: Rc<RefCell<NewActiveValue>>) {
        (*selff).state.pc = some_value;
    }
}
