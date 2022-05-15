use crate::{traits::bit_vector::BitVector, ScfiaStdlib};
use std::rc::Rc;
use std::cell::RefCell;

use crate::Ast;
use crate::expressions::bv_add_expression::BVAddExpression;
use crate::expressions::bv_concat_expression::BVConcatExpression;
use crate::expressions::bv_or_expression::BVOrExpression;
use crate::expressions::bv_sign_extend_expression::BVSignExtendExpression;
use crate::expressions::bv_slice_expression::BVSliceExpression;
use crate::memory::memory32::Memory32;
use crate::values::bit_vector_concrete::BitVectorConcrete;

pub struct ForkSink {
    pub predecessor: SystemState,
    pub forks: Vec<Rc<RefCell<dyn Ast>>>,
}

pub struct SystemState {
    pub x0: Rc<RefCell<dyn Ast>>,
    pub x1: Rc<RefCell<dyn Ast>>,
    pub x2: Rc<RefCell<dyn Ast>>,
    pub x3: Rc<RefCell<dyn Ast>>,
    pub x4: Rc<RefCell<dyn Ast>>,
    pub x5: Rc<RefCell<dyn Ast>>,
    pub x6: Rc<RefCell<dyn Ast>>,
    pub x7: Rc<RefCell<dyn Ast>>,
    pub x8: Rc<RefCell<dyn Ast>>,
    pub x9: Rc<RefCell<dyn Ast>>,
    pub x10: Rc<RefCell<dyn Ast>>,
    pub x11: Rc<RefCell<dyn Ast>>,
    pub x12: Rc<RefCell<dyn Ast>>,
    pub x13: Rc<RefCell<dyn Ast>>,
    pub x14: Rc<RefCell<dyn Ast>>,
    pub x15: Rc<RefCell<dyn Ast>>,
    pub x16: Rc<RefCell<dyn Ast>>,
    pub x17: Rc<RefCell<dyn Ast>>,
    pub x18: Rc<RefCell<dyn Ast>>,
    pub x19: Rc<RefCell<dyn Ast>>,
    pub x20: Rc<RefCell<dyn Ast>>,
    pub x21: Rc<RefCell<dyn Ast>>,
    pub x22: Rc<RefCell<dyn Ast>>,
    pub x23: Rc<RefCell<dyn Ast>>,
    pub x24: Rc<RefCell<dyn Ast>>,
    pub x25: Rc<RefCell<dyn Ast>>,
    pub x26: Rc<RefCell<dyn Ast>>,
    pub x27: Rc<RefCell<dyn Ast>>,
    pub x28: Rc<RefCell<dyn Ast>>,
    pub x29: Rc<RefCell<dyn Ast>>,
    pub x30: Rc<RefCell<dyn Ast>>,
    pub x31: Rc<RefCell<dyn Ast>>,
    pub pc: Rc<RefCell<dyn Ast>>,
}

pub struct ComplexSpecialStruct {
    pub some_flag: Rc<RefCell<dyn Ast>>,
}

pub unsafe fn reset(state: *mut SystemState, _stdlib: &mut ScfiaStdlib, _fork_sink: Option<Rc<RefCell<ForkSink>>>, _memory: &mut Memory32) {
    (*state).x0 = Rc::new(RefCell::new(BitVectorConcrete::new(0b0, 32, _stdlib)));
    (*state).x1 = Rc::new(RefCell::new(BitVectorConcrete::new(0b0, 32, _stdlib)));
    (*state).x2 = Rc::new(RefCell::new(BitVectorConcrete::new(0b0, 32, _stdlib)));
    (*state).x3 = Rc::new(RefCell::new(BitVectorConcrete::new(0b0, 32, _stdlib)));
    (*state).x4 = Rc::new(RefCell::new(BitVectorConcrete::new(0b0, 32, _stdlib)));
    (*state).x5 = Rc::new(RefCell::new(BitVectorConcrete::new(0b0, 32, _stdlib)));
    (*state).x6 = Rc::new(RefCell::new(BitVectorConcrete::new(0b0, 32, _stdlib)));
    (*state).x7 = Rc::new(RefCell::new(BitVectorConcrete::new(0b0, 32, _stdlib)));
    (*state).x8 = Rc::new(RefCell::new(BitVectorConcrete::new(0b0, 32, _stdlib)));
    (*state).x9 = Rc::new(RefCell::new(BitVectorConcrete::new(0b0, 32, _stdlib)));
    (*state).x10 = Rc::new(RefCell::new(BitVectorConcrete::new(0b0, 32, _stdlib)));
    (*state).x11 = Rc::new(RefCell::new(BitVectorConcrete::new(0b0, 32, _stdlib)));
    (*state).x12 = Rc::new(RefCell::new(BitVectorConcrete::new(0b0, 32, _stdlib)));
    (*state).x13 = Rc::new(RefCell::new(BitVectorConcrete::new(0b0, 32, _stdlib)));
    (*state).x14 = Rc::new(RefCell::new(BitVectorConcrete::new(0b0, 32, _stdlib)));
    (*state).x15 = Rc::new(RefCell::new(BitVectorConcrete::new(0b0, 32, _stdlib)));
    (*state).x16 = Rc::new(RefCell::new(BitVectorConcrete::new(0b0, 32, _stdlib)));
    (*state).x17 = Rc::new(RefCell::new(BitVectorConcrete::new(0b0, 32, _stdlib)));
    (*state).x18 = Rc::new(RefCell::new(BitVectorConcrete::new(0b0, 32, _stdlib)));
    (*state).x19 = Rc::new(RefCell::new(BitVectorConcrete::new(0b0, 32, _stdlib)));
    (*state).x20 = Rc::new(RefCell::new(BitVectorConcrete::new(0b0, 32, _stdlib)));
    (*state).x21 = Rc::new(RefCell::new(BitVectorConcrete::new(0b0, 32, _stdlib)));
    (*state).x22 = Rc::new(RefCell::new(BitVectorConcrete::new(0b0, 32, _stdlib)));
    (*state).x23 = Rc::new(RefCell::new(BitVectorConcrete::new(0b0, 32, _stdlib)));
    (*state).x24 = Rc::new(RefCell::new(BitVectorConcrete::new(0b0, 32, _stdlib)));
    (*state).x25 = Rc::new(RefCell::new(BitVectorConcrete::new(0b0, 32, _stdlib)));
    (*state).x26 = Rc::new(RefCell::new(BitVectorConcrete::new(0b0, 32, _stdlib)));
    (*state).x27 = Rc::new(RefCell::new(BitVectorConcrete::new(0b0, 32, _stdlib)));
    (*state).x28 = Rc::new(RefCell::new(BitVectorConcrete::new(0b0, 32, _stdlib)));
    (*state).x29 = Rc::new(RefCell::new(BitVectorConcrete::new(0b0, 32, _stdlib)));
    (*state).x30 = Rc::new(RefCell::new(BitVectorConcrete::new(0b0, 32, _stdlib)));
    (*state).x31 = Rc::new(RefCell::new(BitVectorConcrete::new(0b0, 32, _stdlib)));
}

pub unsafe fn sum(state: *mut SystemState, _stdlib: &mut ScfiaStdlib, _fork_sink: Option<Rc<RefCell<ForkSink>>>, _memory: &mut Memory32) -> Rc<RefCell<dyn Ast>> {
    return Rc::new(RefCell::new(BVAddExpression::new((*state).x1.clone(), (*state).x2.clone(), _stdlib)));
}

pub unsafe fn test(state: *mut SystemState, _stdlib: &mut ScfiaStdlib, _fork_sink: Option<Rc<RefCell<ForkSink>>>, _memory: &mut Memory32) {
    (*state).x3 = Rc::new(RefCell::new(BVAddExpression::new(sum(state, _stdlib, _fork_sink.clone(), _memory), sum(state, _stdlib, _fork_sink.clone(), _memory), _stdlib)));
}

pub unsafe fn step(state: *mut SystemState, _stdlib: &mut ScfiaStdlib, _fork_sink: Option<Rc<RefCell<ForkSink>>>, _memory: &mut Memory32) {
    let instruction_32: Rc<RefCell<dyn Ast>> = _memory.read((*state).pc.clone(), 32, _stdlib);
    let opcode: Rc<RefCell<dyn Ast>> = Rc::new(RefCell::new(BVSliceExpression::new(instruction_32.clone(), 6, 0, _stdlib)));
    if _stdlib.do_eq(opcode.clone(), Rc::new(RefCell::new(BitVectorConcrete::new(0b11, 7, _stdlib))), _fork_sink.clone()) {
        let funct3: Rc<RefCell<dyn Ast>> = Rc::new(RefCell::new(BVSliceExpression::new(instruction_32.clone(), 14, 12, _stdlib)));
        if _stdlib.do_eq(funct3.clone(), Rc::new(RefCell::new(BitVectorConcrete::new(0b10, 3, _stdlib))), _fork_sink.clone()) {
            let rd: Rc<RefCell<dyn Ast>> = extract_rd_32(instruction_32.clone(), _stdlib, _fork_sink.clone(), _memory);
            let rs1: Rc<RefCell<dyn Ast>> = extract_rs1_32(instruction_32.clone(), _stdlib, _fork_sink.clone(), _memory);
            let imm: Rc<RefCell<dyn Ast>> = Rc::new(RefCell::new(BVSliceExpression::new(instruction_32.clone(), 31, 20, _stdlib)));
            let imm32: Rc<RefCell<dyn Ast>> = Rc::new(RefCell::new(BVSignExtendExpression::new(imm.clone(), 12, 32, _stdlib)));
            let base_address: Rc<RefCell<dyn Ast>> = register_read_BV32(state, rs1.clone(), _stdlib, _fork_sink.clone(), _memory);
            let address: Rc<RefCell<dyn Ast>> = Rc::new(RefCell::new(BVAddExpression::new(base_address.clone(), imm32.clone(), _stdlib)));
            let value: Rc<RefCell<dyn Ast>> = _memory.read(address.clone(), 32, _stdlib);
            register_write_BV32(state, rd.clone(), value.clone(), _stdlib, _fork_sink.clone(), _memory);
            progress_pc_4(state, _stdlib, _fork_sink.clone(), _memory);
        }
        else {
            unimplemented!();
        }
    }
    else if _stdlib.do_eq(opcode.clone(), Rc::new(RefCell::new(BitVectorConcrete::new(0b10011, 7, _stdlib))), _fork_sink.clone()) {
        let funct3: Rc<RefCell<dyn Ast>> = Rc::new(RefCell::new(BVSliceExpression::new(instruction_32.clone(), 14, 12, _stdlib)));
        if _stdlib.do_eq(funct3.clone(), Rc::new(RefCell::new(BitVectorConcrete::new(0b0, 3, _stdlib))), _fork_sink.clone()) {
            let rd: Rc<RefCell<dyn Ast>> = extract_rd_32(instruction_32.clone(), _stdlib, _fork_sink.clone(), _memory);
            let rs1: Rc<RefCell<dyn Ast>> = extract_rs1_32(instruction_32.clone(), _stdlib, _fork_sink.clone(), _memory);
            let offset: Rc<RefCell<dyn Ast>> = Rc::new(RefCell::new(BVSliceExpression::new(instruction_32.clone(), 31, 20, _stdlib)));
            let offset_32: Rc<RefCell<dyn Ast>> = Rc::new(RefCell::new(BVSignExtendExpression::new(offset.clone(), 12, 32, _stdlib)));
            let result: Rc<RefCell<dyn Ast>> = Rc::new(RefCell::new(BVAddExpression::new(offset_32.clone(), register_read_BV32(state, rs1.clone(), _stdlib, _fork_sink.clone(), _memory), _stdlib)));
            register_write_BV32(state, rd.clone(), result.clone(), _stdlib, _fork_sink.clone(), _memory);
            progress_pc_4(state, _stdlib, _fork_sink.clone(), _memory);
        }
        else if _stdlib.do_eq(funct3.clone(), Rc::new(RefCell::new(BitVectorConcrete::new(0b110, 3, _stdlib))), _fork_sink.clone()) {
            let rd: Rc<RefCell<dyn Ast>> = extract_rd_32(instruction_32.clone(), _stdlib, _fork_sink.clone(), _memory);
            let rs1: Rc<RefCell<dyn Ast>> = extract_rs1_32(instruction_32.clone(), _stdlib, _fork_sink.clone(), _memory);
            let imm: Rc<RefCell<dyn Ast>> = Rc::new(RefCell::new(BVSliceExpression::new(instruction_32.clone(), 31, 20, _stdlib)));
            let imm_32: Rc<RefCell<dyn Ast>> = Rc::new(RefCell::new(BVSignExtendExpression::new(imm.clone(), 12, 32, _stdlib)));
            let result: Rc<RefCell<dyn Ast>> = Rc::new(RefCell::new(BVOrExpression::new(register_read_BV32(state, rs1.clone(), _stdlib, _fork_sink.clone(), _memory), imm_32.clone(), _stdlib)));
            register_write_BV32(state, rd.clone(), result.clone(), _stdlib, _fork_sink.clone(), _memory);
            progress_pc_4(state, _stdlib, _fork_sink.clone(), _memory);
        }
        else {
            unimplemented!();
        }
    }
    else if _stdlib.do_eq(opcode.clone(), Rc::new(RefCell::new(BitVectorConcrete::new(0b100011, 7, _stdlib))), _fork_sink.clone()) {
        let funct3: Rc<RefCell<dyn Ast>> = Rc::new(RefCell::new(BVSliceExpression::new(instruction_32.clone(), 14, 12, _stdlib)));
        if _stdlib.do_eq(funct3.clone(), Rc::new(RefCell::new(BitVectorConcrete::new(0b10, 3, _stdlib))), _fork_sink.clone()) {
            let rs1: Rc<RefCell<dyn Ast>> = extract_rs1_32(instruction_32.clone(), _stdlib, _fork_sink.clone(), _memory);
            let rs2: Rc<RefCell<dyn Ast>> = extract_rs2_32(instruction_32.clone(), _stdlib, _fork_sink.clone(), _memory);
            let offset_11_5: Rc<RefCell<dyn Ast>> = Rc::new(RefCell::new(BVSliceExpression::new(instruction_32.clone(), 31, 25, _stdlib)));
            let offset_4_0: Rc<RefCell<dyn Ast>> = Rc::new(RefCell::new(BVSliceExpression::new(instruction_32.clone(), 11, 7, _stdlib)));
            let offset: Rc<RefCell<dyn Ast>> = Rc::new(RefCell::new(BVConcatExpression::new(offset_11_5.clone(), offset_4_0.clone(), _stdlib)));
            let offset_32: Rc<RefCell<dyn Ast>> = Rc::new(RefCell::new(BVSignExtendExpression::new(offset.clone(), 12, 32, _stdlib)));
            let base_address: Rc<RefCell<dyn Ast>> = register_read_BV32(state, rs1.clone(), _stdlib, _fork_sink.clone(), _memory);
            let address: Rc<RefCell<dyn Ast>> = Rc::new(RefCell::new(BVAddExpression::new(base_address.clone(), offset_32.clone(), _stdlib)));
            let value: Rc<RefCell<dyn Ast>> = register_read_BV32(state, rs2.clone(), _stdlib, _fork_sink.clone(), _memory);
            _memory.write(address.clone(), value.clone(), _stdlib);
            progress_pc_4(state, _stdlib, _fork_sink.clone(), _memory);
        }
        else {
            unimplemented!();
        }
    }
    else if _stdlib.do_eq(opcode.clone(), Rc::new(RefCell::new(BitVectorConcrete::new(0b110111, 7, _stdlib))), _fork_sink.clone()) {
        let rd: Rc<RefCell<dyn Ast>> = extract_rd_32(instruction_32.clone(), _stdlib, _fork_sink.clone(), _memory);
        let rs: Rc<RefCell<dyn Ast>> = extract_rs1_32(instruction_32.clone(), _stdlib, _fork_sink.clone(), _memory);
        let imm: Rc<RefCell<dyn Ast>> = Rc::new(RefCell::new(BVSliceExpression::new(instruction_32.clone(), 31, 12, _stdlib)));
        let value: Rc<RefCell<dyn Ast>> = Rc::new(RefCell::new(BVConcatExpression::new(imm.clone(), Rc::new(RefCell::new(BitVectorConcrete::new(0b0, 12, _stdlib))), _stdlib)));
        register_write_BV32(state, rd.clone(), value.clone(), _stdlib, _fork_sink.clone(), _memory);
        progress_pc_4(state, _stdlib, _fork_sink.clone(), _memory);
    }
    else if _stdlib.do_eq(opcode.clone(), Rc::new(RefCell::new(BitVectorConcrete::new(0b10111, 7, _stdlib))), _fork_sink.clone()) {
        let dst: Rc<RefCell<dyn Ast>> = extract_rd_32(instruction_32.clone(), _stdlib, _fork_sink.clone(), _memory);
        let imm: Rc<RefCell<dyn Ast>> = Rc::new(RefCell::new(BVSliceExpression::new(instruction_32.clone(), 31, 12, _stdlib)));
        let imm32: Rc<RefCell<dyn Ast>> = Rc::new(RefCell::new(BVConcatExpression::new(imm.clone(), Rc::new(RefCell::new(BitVectorConcrete::new(0b0, 12, _stdlib))), _stdlib)));
        let sum: Rc<RefCell<dyn Ast>> = Rc::new(RefCell::new(BVAddExpression::new(imm32.clone(), (*state).pc.clone(), _stdlib)));
        register_write_BV32(state, dst.clone(), sum.clone(), _stdlib, _fork_sink.clone(), _memory);
        progress_pc_4(state, _stdlib, _fork_sink.clone(), _memory);
    }
    else if _stdlib.do_eq(opcode.clone(), Rc::new(RefCell::new(BitVectorConcrete::new(0b110011, 7, _stdlib))), _fork_sink.clone()) {
        let funct3: Rc<RefCell<dyn Ast>> = Rc::new(RefCell::new(BVSliceExpression::new(instruction_32.clone(), 14, 12, _stdlib)));
        let funct7: Rc<RefCell<dyn Ast>> = Rc::new(RefCell::new(BVSliceExpression::new(instruction_32.clone(), 31, 25, _stdlib)));
        let rd: Rc<RefCell<dyn Ast>> = Rc::new(RefCell::new(BVSliceExpression::new(instruction_32.clone(), 11, 7, _stdlib)));
        let rs1: Rc<RefCell<dyn Ast>> = Rc::new(RefCell::new(BVSliceExpression::new(instruction_32.clone(), 19, 15, _stdlib)));
        let rs2: Rc<RefCell<dyn Ast>> = Rc::new(RefCell::new(BVSliceExpression::new(instruction_32.clone(), 24, 20, _stdlib)));
        if _stdlib.do_eq(funct3.clone(), Rc::new(RefCell::new(BitVectorConcrete::new(0b0, 3, _stdlib))), _fork_sink.clone()) {
            if _stdlib.do_eq(funct7.clone(), Rc::new(RefCell::new(BitVectorConcrete::new(0b0, 7, _stdlib))), _fork_sink.clone()) {
                execute_add32(state, rd.clone(), rs2.clone(), rs2.clone(), _stdlib, _fork_sink.clone(), _memory);
                progress_pc_4(state, _stdlib, _fork_sink.clone(), _memory);
            }
            else if _stdlib.do_eq(funct7.clone(), Rc::new(RefCell::new(BitVectorConcrete::new(0b100000, 7, _stdlib))), _fork_sink.clone()) {
                (*state).pc = Rc::new(RefCell::new(BitVectorConcrete::new(0b0, 32, _stdlib)));
                unimplemented!();
            }
            else {
                unimplemented!();
            }
        }
        else if _stdlib.do_eq(funct3.clone(), Rc::new(RefCell::new(BitVectorConcrete::new(0b1, 3, _stdlib))), _fork_sink.clone()) {
            if _stdlib.do_eq(funct7.clone(), Rc::new(RefCell::new(BitVectorConcrete::new(0b0, 7, _stdlib))), _fork_sink.clone()) {
                unimplemented!();
            }
            else {
                unimplemented!();
            }
        }
        else if _stdlib.do_eq(funct3.clone(), Rc::new(RefCell::new(BitVectorConcrete::new(0b10, 3, _stdlib))), _fork_sink.clone()) {
            if _stdlib.do_eq(funct7.clone(), Rc::new(RefCell::new(BitVectorConcrete::new(0b0, 7, _stdlib))), _fork_sink.clone()) {
                unimplemented!();
            }
            else {
                unimplemented!();
            }
        }
        else if _stdlib.do_eq(funct3.clone(), Rc::new(RefCell::new(BitVectorConcrete::new(0b11, 3, _stdlib))), _fork_sink.clone()) {
            if _stdlib.do_eq(funct7.clone(), Rc::new(RefCell::new(BitVectorConcrete::new(0b0, 7, _stdlib))), _fork_sink.clone()) {
                unimplemented!();
            }
            else {
                unimplemented!();
            }
        }
        else if _stdlib.do_eq(funct3.clone(), Rc::new(RefCell::new(BitVectorConcrete::new(0b100, 3, _stdlib))), _fork_sink.clone()) {
            if _stdlib.do_eq(funct7.clone(), Rc::new(RefCell::new(BitVectorConcrete::new(0b0, 7, _stdlib))), _fork_sink.clone()) {
                unimplemented!();
            }
            else {
                unimplemented!();
            }
        }
        else if _stdlib.do_eq(funct3.clone(), Rc::new(RefCell::new(BitVectorConcrete::new(0b101, 3, _stdlib))), _fork_sink.clone()) {
            if _stdlib.do_eq(funct7.clone(), Rc::new(RefCell::new(BitVectorConcrete::new(0b0, 7, _stdlib))), _fork_sink.clone()) {
                unimplemented!();
            }
            else if _stdlib.do_eq(funct7.clone(), Rc::new(RefCell::new(BitVectorConcrete::new(0b100000, 7, _stdlib))), _fork_sink.clone()) {
                unimplemented!();
            }
            else {
                unimplemented!();
            }
        }
        else if _stdlib.do_eq(funct3.clone(), Rc::new(RefCell::new(BitVectorConcrete::new(0b110, 3, _stdlib))), _fork_sink.clone()) {
            if _stdlib.do_eq(funct7.clone(), Rc::new(RefCell::new(BitVectorConcrete::new(0b0, 7, _stdlib))), _fork_sink.clone()) {
                unimplemented!();
            }
            else {
                unimplemented!();
            }
        }
        else if _stdlib.do_eq(funct3.clone(), Rc::new(RefCell::new(BitVectorConcrete::new(0b111, 3, _stdlib))), _fork_sink.clone()) {
            if _stdlib.do_eq(funct7.clone(), Rc::new(RefCell::new(BitVectorConcrete::new(0b0, 7, _stdlib))), _fork_sink.clone()) {
                unimplemented!();
            }
            else {
                unimplemented!();
            }
        }
        else {
            unimplemented!();
        }
    }
    else if _stdlib.do_eq(opcode.clone(), Rc::new(RefCell::new(BitVectorConcrete::new(0b1100011, 7, _stdlib))), _fork_sink.clone()) {
        let funct3: Rc<RefCell<dyn Ast>> = Rc::new(RefCell::new(BVSliceExpression::new(instruction_32.clone(), 14, 12, _stdlib)));
        if _stdlib.do_eq(funct3.clone(), Rc::new(RefCell::new(BitVectorConcrete::new(0b0, 3, _stdlib))), _fork_sink.clone()) {
            unimplemented!();
        }
        else if _stdlib.do_eq(funct3.clone(), Rc::new(RefCell::new(BitVectorConcrete::new(0b1, 3, _stdlib))), _fork_sink.clone()) {
            let lhs: Rc<RefCell<dyn Ast>> = register_read_BV32(state, extract_rs1_32(instruction_32.clone(), _stdlib, _fork_sink.clone(), _memory), _stdlib, _fork_sink.clone(), _memory);
            let rhs: Rc<RefCell<dyn Ast>> = register_read_BV32(state, extract_rs2_32(instruction_32.clone(), _stdlib, _fork_sink.clone(), _memory), _stdlib, _fork_sink.clone(), _memory);
            if _stdlib.do_neq(lhs.clone(), rhs.clone(), _fork_sink.clone()) {
                let imm_4_1: Rc<RefCell<dyn Ast>> = Rc::new(RefCell::new(BVSliceExpression::new(instruction_32.clone(), 11, 8, _stdlib)));
                let imm_10_5: Rc<RefCell<dyn Ast>> = Rc::new(RefCell::new(BVSliceExpression::new(instruction_32.clone(), 30, 25, _stdlib)));
                let imm11: Rc<RefCell<dyn Ast>> = Rc::new(RefCell::new(BVSliceExpression::new(instruction_32.clone(), 7, 7, _stdlib)));
                let imm12: Rc<RefCell<dyn Ast>> = Rc::new(RefCell::new(BVSliceExpression::new(instruction_32.clone(), 31, 31, _stdlib)));
                let imm_4_0: Rc<RefCell<dyn Ast>> = Rc::new(RefCell::new(BVConcatExpression::new(imm_4_1.clone(), Rc::new(RefCell::new(BitVectorConcrete::new(0b0, 1, _stdlib))), _stdlib)));
                let imm_10_0: Rc<RefCell<dyn Ast>> = Rc::new(RefCell::new(BVConcatExpression::new(imm_10_5.clone(), imm_4_0.clone(), _stdlib)));
                let imm_11_0: Rc<RefCell<dyn Ast>> = Rc::new(RefCell::new(BVConcatExpression::new(imm11.clone(), imm_10_0.clone(), _stdlib)));
                let imm_12_0: Rc<RefCell<dyn Ast>> = Rc::new(RefCell::new(BVConcatExpression::new(imm12.clone(), imm_11_0.clone(), _stdlib)));
                let offset: Rc<RefCell<dyn Ast>> = Rc::new(RefCell::new(BVSignExtendExpression::new(imm_12_0.clone(), 13, 32, _stdlib)));
                let address: Rc<RefCell<dyn Ast>> = Rc::new(RefCell::new(BVAddExpression::new((*state).pc.clone(), offset.clone(), _stdlib)));
                (*state).pc = address.clone();
            }
            else {
                progress_pc_4(state, _stdlib, _fork_sink.clone(), _memory);
            }
        }
        else {
            unimplemented!();
        }
    }
    else if _stdlib.do_eq(opcode.clone(), Rc::new(RefCell::new(BitVectorConcrete::new(0b1100111, 7, _stdlib))), _fork_sink.clone()) {
        let funct3: Rc<RefCell<dyn Ast>> = Rc::new(RefCell::new(BVSliceExpression::new(instruction_32.clone(), 14, 12, _stdlib)));
        if _stdlib.do_eq(funct3.clone(), Rc::new(RefCell::new(BitVectorConcrete::new(0b0, 3, _stdlib))), _fork_sink.clone()) {
            let dst: Rc<RefCell<dyn Ast>> = extract_rd_32(instruction_32.clone(), _stdlib, _fork_sink.clone(), _memory);
            let rs1: Rc<RefCell<dyn Ast>> = extract_rs1_32(instruction_32.clone(), _stdlib, _fork_sink.clone(), _memory);
            let s1: Rc<RefCell<dyn Ast>> = register_read_BV32(state, rs1.clone(), _stdlib, _fork_sink.clone(), _memory);
            let offset: Rc<RefCell<dyn Ast>> = Rc::new(RefCell::new(BVSliceExpression::new(instruction_32.clone(), 31, 20, _stdlib)));
            let offset_32: Rc<RefCell<dyn Ast>> = Rc::new(RefCell::new(BVSignExtendExpression::new(offset.clone(), 12, 32, _stdlib)));
            let address: Rc<RefCell<dyn Ast>> = Rc::new(RefCell::new(BVAddExpression::new(s1.clone(), offset_32.clone(), _stdlib)));
            let return_address: Rc<RefCell<dyn Ast>> = Rc::new(RefCell::new(BVAddExpression::new((*state).pc.clone(), Rc::new(RefCell::new(BitVectorConcrete::new(0b100, 32, _stdlib))), _stdlib)));
            (*state).pc = address.clone();
            register_write_BV32(state, dst.clone(), return_address.clone(), _stdlib, _fork_sink.clone(), _memory);
        }
        else {
            unimplemented!();
        }
    }
    else if _stdlib.do_eq(opcode.clone(), Rc::new(RefCell::new(BitVectorConcrete::new(0b1101111, 7, _stdlib))), _fork_sink.clone()) {
        let imm_20: Rc<RefCell<dyn Ast>> = Rc::new(RefCell::new(BVSliceExpression::new(instruction_32.clone(), 31, 31, _stdlib)));
        let imm_10_1: Rc<RefCell<dyn Ast>> = Rc::new(RefCell::new(BVSliceExpression::new(instruction_32.clone(), 30, 21, _stdlib)));
        let imm_11: Rc<RefCell<dyn Ast>> = Rc::new(RefCell::new(BVSliceExpression::new(instruction_32.clone(), 20, 20, _stdlib)));
        let imm_19_12: Rc<RefCell<dyn Ast>> = Rc::new(RefCell::new(BVSliceExpression::new(instruction_32.clone(), 19, 12, _stdlib)));
        let offset_10_0: Rc<RefCell<dyn Ast>> = Rc::new(RefCell::new(BVConcatExpression::new(imm_10_1.clone(), Rc::new(RefCell::new(BitVectorConcrete::new(0b0, 1, _stdlib))), _stdlib)));
        let offset_11_0: Rc<RefCell<dyn Ast>> = Rc::new(RefCell::new(BVConcatExpression::new(imm_11.clone(), offset_10_0.clone(), _stdlib)));
        let offset_19_0: Rc<RefCell<dyn Ast>> = Rc::new(RefCell::new(BVConcatExpression::new(imm_19_12.clone(), offset_11_0.clone(), _stdlib)));
        let offset_20_0: Rc<RefCell<dyn Ast>> = Rc::new(RefCell::new(BVConcatExpression::new(imm_20.clone(), offset_19_0.clone(), _stdlib)));
        let offset_32: Rc<RefCell<dyn Ast>> = Rc::new(RefCell::new(BVSignExtendExpression::new(offset_20_0.clone(), 21, 32, _stdlib)));
        let address: Rc<RefCell<dyn Ast>> = Rc::new(RefCell::new(BVAddExpression::new((*state).pc.clone(), offset_32.clone(), _stdlib)));
        let return_address: Rc<RefCell<dyn Ast>> = Rc::new(RefCell::new(BVAddExpression::new((*state).pc.clone(), Rc::new(RefCell::new(BitVectorConcrete::new(0b100, 32, _stdlib))), _stdlib)));
        let dst: Rc<RefCell<dyn Ast>> = extract_rd_32(instruction_32.clone(), _stdlib, _fork_sink.clone(), _memory);
        register_write_BV32(state, dst.clone(), return_address.clone(), _stdlib, _fork_sink.clone(), _memory);
        (*state).pc = address.clone();
    }
    else {
        unimplemented!();
    }
}

pub unsafe fn extract_rd_32(op: Rc<RefCell<dyn Ast>>, _stdlib: &mut ScfiaStdlib, _fork_sink: Option<Rc<RefCell<ForkSink>>>, _memory: &mut Memory32) -> Rc<RefCell<dyn Ast>> {
    return Rc::new(RefCell::new(BVSliceExpression::new(op.clone(), 11, 7, _stdlib)));
}

pub unsafe fn extract_rs1_32(op: Rc<RefCell<dyn Ast>>, _stdlib: &mut ScfiaStdlib, _fork_sink: Option<Rc<RefCell<ForkSink>>>, _memory: &mut Memory32) -> Rc<RefCell<dyn Ast>> {
    return Rc::new(RefCell::new(BVSliceExpression::new(op.clone(), 19, 15, _stdlib)));
}

pub unsafe fn extract_rs2_32(op: Rc<RefCell<dyn Ast>>, _stdlib: &mut ScfiaStdlib, _fork_sink: Option<Rc<RefCell<ForkSink>>>, _memory: &mut Memory32) -> Rc<RefCell<dyn Ast>> {
    return Rc::new(RefCell::new(BVSliceExpression::new(op.clone(), 24, 20, _stdlib)));
}

pub unsafe fn progress_pc_4(state: *mut SystemState, _stdlib: &mut ScfiaStdlib, _fork_sink: Option<Rc<RefCell<ForkSink>>>, _memory: &mut Memory32) {
    let old_pc: Rc<RefCell<dyn Ast>> = (*state).pc.clone();
    let new_pc: Rc<RefCell<dyn Ast>> = Rc::new(RefCell::new(BVAddExpression::new(old_pc.clone(), Rc::new(RefCell::new(BitVectorConcrete::new(0b100, 32, _stdlib))), _stdlib)));
    (*state).pc = new_pc.clone();
}

pub unsafe fn register_write_BV32(state: *mut SystemState, register_id: Rc<RefCell<dyn Ast>>, value: Rc<RefCell<dyn Ast>>, _stdlib: &mut ScfiaStdlib, _fork_sink: Option<Rc<RefCell<ForkSink>>>, _memory: &mut Memory32) {
    if _stdlib.do_eq(register_id.clone(), Rc::new(RefCell::new(BitVectorConcrete::new(0b0, 5, _stdlib))), _fork_sink.clone()) {
        (*state).x0 = value.clone();
    }
    else if _stdlib.do_eq(register_id.clone(), Rc::new(RefCell::new(BitVectorConcrete::new(0b1, 5, _stdlib))), _fork_sink.clone()) {
        (*state).x1 = value.clone();
    }
    else if _stdlib.do_eq(register_id.clone(), Rc::new(RefCell::new(BitVectorConcrete::new(0b10, 5, _stdlib))), _fork_sink.clone()) {
        (*state).x2 = value.clone();
    }
    else if _stdlib.do_eq(register_id.clone(), Rc::new(RefCell::new(BitVectorConcrete::new(0b11, 5, _stdlib))), _fork_sink.clone()) {
        (*state).x3 = value.clone();
    }
    else if _stdlib.do_eq(register_id.clone(), Rc::new(RefCell::new(BitVectorConcrete::new(0b100, 5, _stdlib))), _fork_sink.clone()) {
        (*state).x4 = value.clone();
    }
    else if _stdlib.do_eq(register_id.clone(), Rc::new(RefCell::new(BitVectorConcrete::new(0b101, 5, _stdlib))), _fork_sink.clone()) {
        (*state).x5 = value.clone();
    }
    else if _stdlib.do_eq(register_id.clone(), Rc::new(RefCell::new(BitVectorConcrete::new(0b110, 5, _stdlib))), _fork_sink.clone()) {
        (*state).x6 = value.clone();
    }
    else if _stdlib.do_eq(register_id.clone(), Rc::new(RefCell::new(BitVectorConcrete::new(0b111, 5, _stdlib))), _fork_sink.clone()) {
        (*state).x7 = value.clone();
    }
    else if _stdlib.do_eq(register_id.clone(), Rc::new(RefCell::new(BitVectorConcrete::new(0b1000, 5, _stdlib))), _fork_sink.clone()) {
        (*state).x8 = value.clone();
    }
    else if _stdlib.do_eq(register_id.clone(), Rc::new(RefCell::new(BitVectorConcrete::new(0b1001, 5, _stdlib))), _fork_sink.clone()) {
        (*state).x9 = value.clone();
    }
    else if _stdlib.do_eq(register_id.clone(), Rc::new(RefCell::new(BitVectorConcrete::new(0b1010, 5, _stdlib))), _fork_sink.clone()) {
        (*state).x10 = value.clone();
    }
    else if _stdlib.do_eq(register_id.clone(), Rc::new(RefCell::new(BitVectorConcrete::new(0b1011, 5, _stdlib))), _fork_sink.clone()) {
        (*state).x11 = value.clone();
    }
    else if _stdlib.do_eq(register_id.clone(), Rc::new(RefCell::new(BitVectorConcrete::new(0b1100, 5, _stdlib))), _fork_sink.clone()) {
        (*state).x12 = value.clone();
    }
    else if _stdlib.do_eq(register_id.clone(), Rc::new(RefCell::new(BitVectorConcrete::new(0b1101, 5, _stdlib))), _fork_sink.clone()) {
        (*state).x13 = value.clone();
    }
    else if _stdlib.do_eq(register_id.clone(), Rc::new(RefCell::new(BitVectorConcrete::new(0b1110, 5, _stdlib))), _fork_sink.clone()) {
        (*state).x14 = value.clone();
    }
    else if _stdlib.do_eq(register_id.clone(), Rc::new(RefCell::new(BitVectorConcrete::new(0b1111, 5, _stdlib))), _fork_sink.clone()) {
        (*state).x15 = value.clone();
    }
    else if _stdlib.do_eq(register_id.clone(), Rc::new(RefCell::new(BitVectorConcrete::new(0b10000, 5, _stdlib))), _fork_sink.clone()) {
        (*state).x16 = value.clone();
    }
    else if _stdlib.do_eq(register_id.clone(), Rc::new(RefCell::new(BitVectorConcrete::new(0b10001, 5, _stdlib))), _fork_sink.clone()) {
        (*state).x17 = value.clone();
    }
    else if _stdlib.do_eq(register_id.clone(), Rc::new(RefCell::new(BitVectorConcrete::new(0b10010, 5, _stdlib))), _fork_sink.clone()) {
        (*state).x18 = value.clone();
    }
    else if _stdlib.do_eq(register_id.clone(), Rc::new(RefCell::new(BitVectorConcrete::new(0b10011, 5, _stdlib))), _fork_sink.clone()) {
        (*state).x19 = value.clone();
    }
    else if _stdlib.do_eq(register_id.clone(), Rc::new(RefCell::new(BitVectorConcrete::new(0b10100, 5, _stdlib))), _fork_sink.clone()) {
        (*state).x20 = value.clone();
    }
    else if _stdlib.do_eq(register_id.clone(), Rc::new(RefCell::new(BitVectorConcrete::new(0b10101, 5, _stdlib))), _fork_sink.clone()) {
        (*state).x21 = value.clone();
    }
    else if _stdlib.do_eq(register_id.clone(), Rc::new(RefCell::new(BitVectorConcrete::new(0b10110, 5, _stdlib))), _fork_sink.clone()) {
        (*state).x22 = value.clone();
    }
    else if _stdlib.do_eq(register_id.clone(), Rc::new(RefCell::new(BitVectorConcrete::new(0b10111, 5, _stdlib))), _fork_sink.clone()) {
        (*state).x23 = value.clone();
    }
    else if _stdlib.do_eq(register_id.clone(), Rc::new(RefCell::new(BitVectorConcrete::new(0b11000, 5, _stdlib))), _fork_sink.clone()) {
        (*state).x24 = value.clone();
    }
    else if _stdlib.do_eq(register_id.clone(), Rc::new(RefCell::new(BitVectorConcrete::new(0b11001, 5, _stdlib))), _fork_sink.clone()) {
        (*state).x25 = value.clone();
    }
    else if _stdlib.do_eq(register_id.clone(), Rc::new(RefCell::new(BitVectorConcrete::new(0b11010, 5, _stdlib))), _fork_sink.clone()) {
        (*state).x26 = value.clone();
    }
    else if _stdlib.do_eq(register_id.clone(), Rc::new(RefCell::new(BitVectorConcrete::new(0b11011, 5, _stdlib))), _fork_sink.clone()) {
        (*state).x27 = value.clone();
    }
    else if _stdlib.do_eq(register_id.clone(), Rc::new(RefCell::new(BitVectorConcrete::new(0b11100, 5, _stdlib))), _fork_sink.clone()) {
        (*state).x28 = value.clone();
    }
    else if _stdlib.do_eq(register_id.clone(), Rc::new(RefCell::new(BitVectorConcrete::new(0b11101, 5, _stdlib))), _fork_sink.clone()) {
        (*state).x29 = value.clone();
    }
    else if _stdlib.do_eq(register_id.clone(), Rc::new(RefCell::new(BitVectorConcrete::new(0b11110, 5, _stdlib))), _fork_sink.clone()) {
        (*state).x30 = value.clone();
    }
    else if _stdlib.do_eq(register_id.clone(), Rc::new(RefCell::new(BitVectorConcrete::new(0b11111, 5, _stdlib))), _fork_sink.clone()) {
        (*state).x31 = value.clone();
    }
    else {
        unimplemented!();
    }
}

pub unsafe fn register_read_BV32(state: *mut SystemState, register_id: Rc<RefCell<dyn Ast>>, _stdlib: &mut ScfiaStdlib, _fork_sink: Option<Rc<RefCell<ForkSink>>>, _memory: &mut Memory32) -> Rc<RefCell<dyn Ast>> {
    if _stdlib.do_eq(register_id.clone(), Rc::new(RefCell::new(BitVectorConcrete::new(0b0, 5, _stdlib))), _fork_sink.clone()) {
        return (*state).x0.clone();
    }
    else if _stdlib.do_eq(register_id.clone(), Rc::new(RefCell::new(BitVectorConcrete::new(0b1, 5, _stdlib))), _fork_sink.clone()) {
        return (*state).x1.clone();
    }
    else if _stdlib.do_eq(register_id.clone(), Rc::new(RefCell::new(BitVectorConcrete::new(0b10, 5, _stdlib))), _fork_sink.clone()) {
        return (*state).x2.clone();
    }
    else if _stdlib.do_eq(register_id.clone(), Rc::new(RefCell::new(BitVectorConcrete::new(0b11, 5, _stdlib))), _fork_sink.clone()) {
        return (*state).x3.clone();
    }
    else if _stdlib.do_eq(register_id.clone(), Rc::new(RefCell::new(BitVectorConcrete::new(0b100, 5, _stdlib))), _fork_sink.clone()) {
        return (*state).x4.clone();
    }
    else if _stdlib.do_eq(register_id.clone(), Rc::new(RefCell::new(BitVectorConcrete::new(0b101, 5, _stdlib))), _fork_sink.clone()) {
        return (*state).x5.clone();
    }
    else if _stdlib.do_eq(register_id.clone(), Rc::new(RefCell::new(BitVectorConcrete::new(0b110, 5, _stdlib))), _fork_sink.clone()) {
        return (*state).x6.clone();
    }
    else if _stdlib.do_eq(register_id.clone(), Rc::new(RefCell::new(BitVectorConcrete::new(0b111, 5, _stdlib))), _fork_sink.clone()) {
        return (*state).x7.clone();
    }
    else if _stdlib.do_eq(register_id.clone(), Rc::new(RefCell::new(BitVectorConcrete::new(0b1000, 5, _stdlib))), _fork_sink.clone()) {
        return (*state).x8.clone();
    }
    else if _stdlib.do_eq(register_id.clone(), Rc::new(RefCell::new(BitVectorConcrete::new(0b1001, 5, _stdlib))), _fork_sink.clone()) {
        return (*state).x9.clone();
    }
    else if _stdlib.do_eq(register_id.clone(), Rc::new(RefCell::new(BitVectorConcrete::new(0b1010, 5, _stdlib))), _fork_sink.clone()) {
        return (*state).x10.clone();
    }
    else if _stdlib.do_eq(register_id.clone(), Rc::new(RefCell::new(BitVectorConcrete::new(0b1011, 5, _stdlib))), _fork_sink.clone()) {
        return (*state).x11.clone();
    }
    else if _stdlib.do_eq(register_id.clone(), Rc::new(RefCell::new(BitVectorConcrete::new(0b1100, 5, _stdlib))), _fork_sink.clone()) {
        return (*state).x12.clone();
    }
    else if _stdlib.do_eq(register_id.clone(), Rc::new(RefCell::new(BitVectorConcrete::new(0b1101, 5, _stdlib))), _fork_sink.clone()) {
        return (*state).x13.clone();
    }
    else if _stdlib.do_eq(register_id.clone(), Rc::new(RefCell::new(BitVectorConcrete::new(0b1110, 5, _stdlib))), _fork_sink.clone()) {
        return (*state).x14.clone();
    }
    else if _stdlib.do_eq(register_id.clone(), Rc::new(RefCell::new(BitVectorConcrete::new(0b1111, 5, _stdlib))), _fork_sink.clone()) {
        return (*state).x15.clone();
    }
    else if _stdlib.do_eq(register_id.clone(), Rc::new(RefCell::new(BitVectorConcrete::new(0b10000, 5, _stdlib))), _fork_sink.clone()) {
        return (*state).x16.clone();
    }
    else if _stdlib.do_eq(register_id.clone(), Rc::new(RefCell::new(BitVectorConcrete::new(0b10001, 5, _stdlib))), _fork_sink.clone()) {
        return (*state).x17.clone();
    }
    else if _stdlib.do_eq(register_id.clone(), Rc::new(RefCell::new(BitVectorConcrete::new(0b10010, 5, _stdlib))), _fork_sink.clone()) {
        return (*state).x18.clone();
    }
    else if _stdlib.do_eq(register_id.clone(), Rc::new(RefCell::new(BitVectorConcrete::new(0b10011, 5, _stdlib))), _fork_sink.clone()) {
        return (*state).x19.clone();
    }
    else if _stdlib.do_eq(register_id.clone(), Rc::new(RefCell::new(BitVectorConcrete::new(0b10100, 5, _stdlib))), _fork_sink.clone()) {
        return (*state).x20.clone();
    }
    else if _stdlib.do_eq(register_id.clone(), Rc::new(RefCell::new(BitVectorConcrete::new(0b10101, 5, _stdlib))), _fork_sink.clone()) {
        return (*state).x21.clone();
    }
    else if _stdlib.do_eq(register_id.clone(), Rc::new(RefCell::new(BitVectorConcrete::new(0b10110, 5, _stdlib))), _fork_sink.clone()) {
        return (*state).x22.clone();
    }
    else if _stdlib.do_eq(register_id.clone(), Rc::new(RefCell::new(BitVectorConcrete::new(0b10111, 5, _stdlib))), _fork_sink.clone()) {
        return (*state).x23.clone();
    }
    else if _stdlib.do_eq(register_id.clone(), Rc::new(RefCell::new(BitVectorConcrete::new(0b11000, 5, _stdlib))), _fork_sink.clone()) {
        return (*state).x24.clone();
    }
    else if _stdlib.do_eq(register_id.clone(), Rc::new(RefCell::new(BitVectorConcrete::new(0b11001, 5, _stdlib))), _fork_sink.clone()) {
        return (*state).x25.clone();
    }
    else if _stdlib.do_eq(register_id.clone(), Rc::new(RefCell::new(BitVectorConcrete::new(0b11010, 5, _stdlib))), _fork_sink.clone()) {
        return (*state).x26.clone();
    }
    else if _stdlib.do_eq(register_id.clone(), Rc::new(RefCell::new(BitVectorConcrete::new(0b11011, 5, _stdlib))), _fork_sink.clone()) {
        return (*state).x27.clone();
    }
    else if _stdlib.do_eq(register_id.clone(), Rc::new(RefCell::new(BitVectorConcrete::new(0b11100, 5, _stdlib))), _fork_sink.clone()) {
        return (*state).x28.clone();
    }
    else if _stdlib.do_eq(register_id.clone(), Rc::new(RefCell::new(BitVectorConcrete::new(0b11101, 5, _stdlib))), _fork_sink.clone()) {
        return (*state).x29.clone();
    }
    else if _stdlib.do_eq(register_id.clone(), Rc::new(RefCell::new(BitVectorConcrete::new(0b11110, 5, _stdlib))), _fork_sink.clone()) {
        return (*state).x30.clone();
    }
    else if _stdlib.do_eq(register_id.clone(), Rc::new(RefCell::new(BitVectorConcrete::new(0b11111, 5, _stdlib))), _fork_sink.clone()) {
        return (*state).x31.clone();
    }
    else {
        unimplemented!();
    }
}

pub unsafe fn execute_add32(state: *mut SystemState, destination_id: Rc<RefCell<dyn Ast>>, source1_id: Rc<RefCell<dyn Ast>>, source2_id: Rc<RefCell<dyn Ast>>, _stdlib: &mut ScfiaStdlib, _fork_sink: Option<Rc<RefCell<ForkSink>>>, _memory: &mut Memory32) {
    let s1: Rc<RefCell<dyn Ast>> = register_read_BV32(state, source1_id.clone(), _stdlib, _fork_sink.clone(), _memory);
    let s2: Rc<RefCell<dyn Ast>> = register_read_BV32(state, source2_id.clone(), _stdlib, _fork_sink.clone(), _memory);
    let sum: Rc<RefCell<dyn Ast>> = Rc::new(RefCell::new(BVAddExpression::new(s1.clone(), s2.clone(), _stdlib)));
    register_write_BV32(state, destination_id.clone(), sum.clone(), _stdlib, _fork_sink.clone(), _memory);
}
