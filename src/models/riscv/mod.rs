use std::{rc::Rc, cell::RefCell};

use crate::{ScfiaStdlib, values::bit_vector_concrete::BitVectorConcrete};

pub mod rv32i;


#[test]
fn test_system_state() {
    let mut stdlib = ScfiaStdlib::new();
    let mut rv32i_system_state = rv32i::SystemState {
        x0:  Rc::new(RefCell::new(BitVectorConcrete::new(0b0, 32, &mut stdlib))),
        x1:  Rc::new(RefCell::new(BitVectorConcrete::new(0b0, 32, &mut stdlib))),
        x2:  Rc::new(RefCell::new(BitVectorConcrete::new(0b0, 32, &mut stdlib))),
        x3:  Rc::new(RefCell::new(BitVectorConcrete::new(0b0, 32, &mut stdlib))),
        pc:  Rc::new(RefCell::new(BitVectorConcrete::new(0b0, 32, &mut stdlib))),
        x4:  Rc::new(RefCell::new(BitVectorConcrete::new(0b0, 32, &mut stdlib))),
        x5:  Rc::new(RefCell::new(BitVectorConcrete::new(0b0, 32, &mut stdlib))),
        x6:  Rc::new(RefCell::new(BitVectorConcrete::new(0b0, 32, &mut stdlib))),
        x7:  Rc::new(RefCell::new(BitVectorConcrete::new(0b0, 32, &mut stdlib))),
        x8:  Rc::new(RefCell::new(BitVectorConcrete::new(0b0, 32, &mut stdlib))),
        x9:  Rc::new(RefCell::new(BitVectorConcrete::new(0b0, 32, &mut stdlib))),
        x10:  Rc::new(RefCell::new(BitVectorConcrete::new(0b0, 32, &mut stdlib))),
        x11:  Rc::new(RefCell::new(BitVectorConcrete::new(0b0, 32, &mut stdlib))),
        x12:  Rc::new(RefCell::new(BitVectorConcrete::new(0b0, 32, &mut stdlib))),
        x13:  Rc::new(RefCell::new(BitVectorConcrete::new(0b0, 32, &mut stdlib))),
        x14:  Rc::new(RefCell::new(BitVectorConcrete::new(0b0, 32, &mut stdlib))),
        x15:  Rc::new(RefCell::new(BitVectorConcrete::new(0b0, 32, &mut stdlib))),
        x16:  Rc::new(RefCell::new(BitVectorConcrete::new(0b0, 32, &mut stdlib))),
        x17:  Rc::new(RefCell::new(BitVectorConcrete::new(0b0, 32, &mut stdlib))),
        x18:  Rc::new(RefCell::new(BitVectorConcrete::new(0b0, 32, &mut stdlib))),
        x19:  Rc::new(RefCell::new(BitVectorConcrete::new(0b0, 32, &mut stdlib))),
        x20:  Rc::new(RefCell::new(BitVectorConcrete::new(0b0, 32, &mut stdlib))),
        x21:  Rc::new(RefCell::new(BitVectorConcrete::new(0b0, 32, &mut stdlib))),
        x22:  Rc::new(RefCell::new(BitVectorConcrete::new(0b0, 32, &mut stdlib))),
        x23:  Rc::new(RefCell::new(BitVectorConcrete::new(0b0, 32, &mut stdlib))),
        x24:  Rc::new(RefCell::new(BitVectorConcrete::new(0b0, 32, &mut stdlib))),
        x25:  Rc::new(RefCell::new(BitVectorConcrete::new(0b0, 32, &mut stdlib))),
        x26:  Rc::new(RefCell::new(BitVectorConcrete::new(0b0, 32, &mut stdlib))),
        x27:  Rc::new(RefCell::new(BitVectorConcrete::new(0b0, 32, &mut stdlib))),
        x28:  Rc::new(RefCell::new(BitVectorConcrete::new(0b0, 32, &mut stdlib))),
        x29:  Rc::new(RefCell::new(BitVectorConcrete::new(0b0, 32, &mut stdlib))),
        x30:  Rc::new(RefCell::new(BitVectorConcrete::new(0b0, 32, &mut stdlib))),
        x31:  Rc::new(RefCell::new(BitVectorConcrete::new(0b0, 32, &mut stdlib))),
    };
    unsafe {
        rv32i::test(&mut rv32i_system_state, &mut stdlib)
    };
}