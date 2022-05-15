use std::{rc::Rc, cell::RefCell};

use crate::{traits::{bit_vector::BitVector, ast::Ast}, ScfiaStdlib, values::{bit_vector_concrete::BitVectorConcrete, ActiveValue}};

pub struct Memory32 {
    
}

impl Memory32 {
    pub fn read(&mut self, address: Rc<RefCell<ActiveValue>>, width: u32, stdlib: &mut ScfiaStdlib) -> ActiveValue {
        //let address: &mut dyn Ast = &mut *address.try_borrow_mut().unwrap();
        // let bv = address.downcast_ref::<BitVectorConcrete>().unwrap();
        // Rc::new(RefCell::new(BitVectorConcrete::new(0xfd010113, 32, stdlib)))
        ActiveValue::BitvectorConcrete(BitVectorConcrete::new(0xfd010113, 32, stdlib))
    }
    
    pub fn write(&mut self, address: Rc<RefCell<ActiveValue>>, value: Rc<RefCell<ActiveValue>>, stdlib: &mut ScfiaStdlib) {
        unimplemented!()
    }
}