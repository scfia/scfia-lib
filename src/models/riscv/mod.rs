use std::{cell::RefCell, rc::Rc};

use crate::{
    memory::memory32::Memory32, values::bit_vector_concrete::BitVectorConcrete, ScfiaStdlib,
};

pub mod rv32i;

#[test]
fn test_system_state() {
    let mut stdlib = ScfiaStdlib::new();

    let mut rv32i_system_state = rv32i::SystemState {
        x0: BitVectorConcrete::new(0b0, 32, &mut stdlib).into(),
        x1: BitVectorConcrete::new(0b0, 32, &mut stdlib).into(),
        x2: BitVectorConcrete::new(0b0, 32, &mut stdlib).into(),
        x3: BitVectorConcrete::new(0b0, 32, &mut stdlib).into(),
        pc: BitVectorConcrete::new(0b0, 32, &mut stdlib).into(),
        x4: BitVectorConcrete::new(0b0, 32, &mut stdlib).into(),
        x5: BitVectorConcrete::new(0b0, 32, &mut stdlib).into(),
        x6: BitVectorConcrete::new(0b0, 32, &mut stdlib).into(),
        x7: BitVectorConcrete::new(0b0, 32, &mut stdlib).into(),
        x8: BitVectorConcrete::new(0b0, 32, &mut stdlib).into(),
        x9: BitVectorConcrete::new(0b0, 32, &mut stdlib).into(),
        x10: BitVectorConcrete::new(0b0, 32, &mut stdlib).into(),
        x11: BitVectorConcrete::new(0b0, 32, &mut stdlib).into(),
        x12: BitVectorConcrete::new(0b0, 32, &mut stdlib).into(),
        x13: BitVectorConcrete::new(0b0, 32, &mut stdlib).into(),
        x14: BitVectorConcrete::new(0b0, 32, &mut stdlib).into(),
        x15: BitVectorConcrete::new(0b0, 32, &mut stdlib).into(),
        x16: BitVectorConcrete::new(0b0, 32, &mut stdlib).into(),
        x17: BitVectorConcrete::new(0b0, 32, &mut stdlib).into(),
        x18: BitVectorConcrete::new(0b0, 32, &mut stdlib).into(),
        x19: BitVectorConcrete::new(0b0, 32, &mut stdlib).into(),
        x20: BitVectorConcrete::new(0b0, 32, &mut stdlib).into(),
        x21: BitVectorConcrete::new(0b0, 32, &mut stdlib).into(),
        x22: BitVectorConcrete::new(0b0, 32, &mut stdlib).into(),
        x23: BitVectorConcrete::new(0b0, 32, &mut stdlib).into(),
        x24: BitVectorConcrete::new(0b0, 32, &mut stdlib).into(),
        x25: BitVectorConcrete::new(0b0, 32, &mut stdlib).into(),
        x26: BitVectorConcrete::new(0b0, 32, &mut stdlib).into(),
        x27: BitVectorConcrete::new(0b0, 32, &mut stdlib).into(),
        x28: BitVectorConcrete::new(0b0, 32, &mut stdlib).into(),
        x29: BitVectorConcrete::new(0b0, 32, &mut stdlib).into(),
        x30: BitVectorConcrete::new(0b0, 32, &mut stdlib).into(),
        x31: BitVectorConcrete::new(0b0, 32, &mut stdlib).into(),
    };
    unsafe {
        let mut memory = Memory32 {};
        rv32i::step(&mut rv32i_system_state, &mut stdlib, None, &mut memory);
        // rv32i_system_state.x2 = BitVectorConcrete::new(0b0, 32, &mut stdlib).into();
        rv32i_system_state.pc = BitVectorConcrete::new(0b0, 32, &mut stdlib).into();
        println!("{:?}", &rv32i_system_state);
    };
}
