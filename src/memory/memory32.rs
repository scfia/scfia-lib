use std::{rc::Rc, cell::RefCell};

use crate::{traits::{bit_vector::BitVector, ast::ActiveAst}, ScfiaStdlib};

pub struct Memory32 {
    
}

impl Memory32 {
    pub fn read(&mut self, address: Rc<RefCell<dyn ActiveAst>>, width: u32, stdlib: &mut ScfiaStdlib) -> Rc<RefCell<dyn ActiveAst>> {
        unimplemented!()
    }
    
    pub fn write(&mut self, address: Rc<RefCell<dyn ActiveAst>>, value: Rc<RefCell<dyn ActiveAst>>, stdlib: &mut ScfiaStdlib) {
        unimplemented!()
    }
}