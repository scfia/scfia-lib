use std::{rc::Rc, cell::RefCell};

use crate::{traits::{bit_vector::BitVector, ast::Ast}, ScfiaStdlib, values::bit_vector_concrete::BitVectorConcrete};

pub struct Memory32 {
    
}

impl Memory32 {
    pub fn read(&mut self, address: Rc<RefCell<dyn Ast>>, width: u32, stdlib: &mut ScfiaStdlib) -> Rc<RefCell<dyn Ast>> {
        Rc::new(RefCell::new(BitVectorConcrete::new(0xfd010113, 32, stdlib)))
    }
    
    pub fn write(&mut self, address: Rc<RefCell<dyn Ast>>, value: Rc<RefCell<dyn Ast>>, stdlib: &mut ScfiaStdlib) {
        unimplemented!()
    }
}