use z3_sys::Z3_solver_assert;

use crate::{traits::bit_vector::BitVector, ScfiaStdlib};
use std::fmt;
use std::ops::Deref;
use std::rc::Rc;
use std::cell::RefCell;
use std::collections::HashMap;

use crate::Ast;
use crate::values::ActiveValue;
use crate::values::RetiredValue;
use crate::expressions::bv_add_expression::BVAddExpression;
use crate::expressions::bv_and_expression::BVAndExpression;
use crate::expressions::bv_shift_right_logical_expression::BVShiftRightLogicalExpression;
use crate::expressions::bv_shift_left_logical_expression::BVShiftLeftLogicalExpression;
use crate::expressions::bool_less_than_uint_expression::BoolLessThanUIntExpression;
use crate::expressions::bool_eq_expression::BoolEqExpression;
use crate::expressions::bool_neq_expression::BoolNEqExpression;
use crate::expressions::bv_concat_expression::BVConcatExpression;
use crate::expressions::bv_or_expression::BVOrExpression;
use crate::expressions::bv_sign_extend_expression::BVSignExtendExpression;
use crate::expressions::bv_slice_expression::BVSliceExpression;
use crate::memory::memory32::Memory32;
use crate::values::bit_vector_concrete::BitVectorConcrete;


pub struct RV32iSystemState {
    pub system_state: SystemState,
    pub memory: Memory32,
    pub stdlib: ScfiaStdlib,
}

#[derive(Debug)]
pub struct ForkSink {
    pub forks: Vec<RV32iSystemStateFork>,
}

#[derive(Debug)]
pub struct RV32iSystemStateFork {
    cloned_stdlib: ScfiaStdlib,
    cloned_active_values: HashMap<u64, Rc<RefCell<ActiveValue>>>,
    cloned_retired_values: HashMap<u64, Rc<RefCell<RetiredValue>>>
}

impl RV32iSystemStateFork {
    pub fn new(next_symbol_id: u64) -> Self {
        RV32iSystemStateFork {
            cloned_stdlib: ScfiaStdlib::new_with_next_id(next_symbol_id),
            cloned_active_values: HashMap::new(),
            cloned_retired_values: HashMap::new()
        }
    }
}

impl fmt::Debug for RV32iSystemState {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "RV32iSystemState {{ system_state = {:?} }}", &self.system_state)
    }
}

impl ForkSink {
    pub fn new() -> Self {
        ForkSink {
            forks: vec![]
        }
    }

    pub fn fork(&mut self, fork_condition: Rc<RefCell<ActiveValue>>) {
        let mut fork = RV32iSystemStateFork::new(fork_condition.try_borrow().unwrap().get_id());

        let cloned_condition = fork_condition.try_borrow().unwrap().clone_to_stdlib(
            &mut fork.cloned_active_values,
            &mut fork.cloned_retired_values,
            &mut fork.cloned_stdlib);

        cloned_condition.try_borrow_mut().unwrap().assert(&mut fork.cloned_stdlib);
        // cloned_condition will reside in cloned_active_values and thus stay active

        self.forks.push(fork);
    }
}


#[derive(Debug)]
pub struct SystemState {
    pub x0: Rc<RefCell<ActiveValue>>,
    pub x1: Rc<RefCell<ActiveValue>>,
    pub x2: Rc<RefCell<ActiveValue>>,
    pub x3: Rc<RefCell<ActiveValue>>,
    pub x4: Rc<RefCell<ActiveValue>>,
    pub x5: Rc<RefCell<ActiveValue>>,
    pub x6: Rc<RefCell<ActiveValue>>,
    pub x7: Rc<RefCell<ActiveValue>>,
    pub x8: Rc<RefCell<ActiveValue>>,
    pub x9: Rc<RefCell<ActiveValue>>,
    pub x10: Rc<RefCell<ActiveValue>>,
    pub x11: Rc<RefCell<ActiveValue>>,
    pub x12: Rc<RefCell<ActiveValue>>,
    pub x13: Rc<RefCell<ActiveValue>>,
    pub x14: Rc<RefCell<ActiveValue>>,
    pub x15: Rc<RefCell<ActiveValue>>,
    pub x16: Rc<RefCell<ActiveValue>>,
    pub x17: Rc<RefCell<ActiveValue>>,
    pub x18: Rc<RefCell<ActiveValue>>,
    pub x19: Rc<RefCell<ActiveValue>>,
    pub x20: Rc<RefCell<ActiveValue>>,
    pub x21: Rc<RefCell<ActiveValue>>,
    pub x22: Rc<RefCell<ActiveValue>>,
    pub x23: Rc<RefCell<ActiveValue>>,
    pub x24: Rc<RefCell<ActiveValue>>,
    pub x25: Rc<RefCell<ActiveValue>>,
    pub x26: Rc<RefCell<ActiveValue>>,
    pub x27: Rc<RefCell<ActiveValue>>,
    pub x28: Rc<RefCell<ActiveValue>>,
    pub x29: Rc<RefCell<ActiveValue>>,
    pub x30: Rc<RefCell<ActiveValue>>,
    pub x31: Rc<RefCell<ActiveValue>>,
    pub pc: Rc<RefCell<ActiveValue>>,
}

#[derive(Debug)]
pub struct ComplexSpecialStruct {
    pub some_flag: Rc<RefCell<ActiveValue>>,
}

pub unsafe fn reset(state: *mut SystemState, _stdlib: *mut ScfiaStdlib, _fork_sink: Option<Rc<RefCell<ForkSink>>>, _memory: &mut Memory32) {
    (*state).x0 = BitVectorConcrete::new(0b0, 32, _stdlib.as_mut().unwrap()).into();
    (*state).x1 = BitVectorConcrete::new(0b0, 32, _stdlib.as_mut().unwrap()).into();
    (*state).x2 = BitVectorConcrete::new(0b0, 32, _stdlib.as_mut().unwrap()).into();
    (*state).x3 = BitVectorConcrete::new(0b0, 32, _stdlib.as_mut().unwrap()).into();
    (*state).x4 = BitVectorConcrete::new(0b0, 32, _stdlib.as_mut().unwrap()).into();
    (*state).x5 = BitVectorConcrete::new(0b0, 32, _stdlib.as_mut().unwrap()).into();
    (*state).x6 = BitVectorConcrete::new(0b0, 32, _stdlib.as_mut().unwrap()).into();
    (*state).x7 = BitVectorConcrete::new(0b0, 32, _stdlib.as_mut().unwrap()).into();
    (*state).x8 = BitVectorConcrete::new(0b0, 32, _stdlib.as_mut().unwrap()).into();
    (*state).x9 = BitVectorConcrete::new(0b0, 32, _stdlib.as_mut().unwrap()).into();
    (*state).x10 = BitVectorConcrete::new(0b0, 32, _stdlib.as_mut().unwrap()).into();
    (*state).x11 = BitVectorConcrete::new(0b0, 32, _stdlib.as_mut().unwrap()).into();
    (*state).x12 = BitVectorConcrete::new(0b0, 32, _stdlib.as_mut().unwrap()).into();
    (*state).x13 = BitVectorConcrete::new(0b0, 32, _stdlib.as_mut().unwrap()).into();
    (*state).x14 = BitVectorConcrete::new(0b0, 32, _stdlib.as_mut().unwrap()).into();
    (*state).x15 = BitVectorConcrete::new(0b0, 32, _stdlib.as_mut().unwrap()).into();
    (*state).x16 = BitVectorConcrete::new(0b0, 32, _stdlib.as_mut().unwrap()).into();
    (*state).x17 = BitVectorConcrete::new(0b0, 32, _stdlib.as_mut().unwrap()).into();
    (*state).x18 = BitVectorConcrete::new(0b0, 32, _stdlib.as_mut().unwrap()).into();
    (*state).x19 = BitVectorConcrete::new(0b0, 32, _stdlib.as_mut().unwrap()).into();
    (*state).x20 = BitVectorConcrete::new(0b0, 32, _stdlib.as_mut().unwrap()).into();
    (*state).x21 = BitVectorConcrete::new(0b0, 32, _stdlib.as_mut().unwrap()).into();
    (*state).x22 = BitVectorConcrete::new(0b0, 32, _stdlib.as_mut().unwrap()).into();
    (*state).x23 = BitVectorConcrete::new(0b0, 32, _stdlib.as_mut().unwrap()).into();
    (*state).x24 = BitVectorConcrete::new(0b0, 32, _stdlib.as_mut().unwrap()).into();
    (*state).x25 = BitVectorConcrete::new(0b0, 32, _stdlib.as_mut().unwrap()).into();
    (*state).x26 = BitVectorConcrete::new(0b0, 32, _stdlib.as_mut().unwrap()).into();
    (*state).x27 = BitVectorConcrete::new(0b0, 32, _stdlib.as_mut().unwrap()).into();
    (*state).x28 = BitVectorConcrete::new(0b0, 32, _stdlib.as_mut().unwrap()).into();
    (*state).x29 = BitVectorConcrete::new(0b0, 32, _stdlib.as_mut().unwrap()).into();
    (*state).x30 = BitVectorConcrete::new(0b0, 32, _stdlib.as_mut().unwrap()).into();
    (*state).x31 = BitVectorConcrete::new(0b0, 32, _stdlib.as_mut().unwrap()).into();
}

pub unsafe fn sum(state: *mut SystemState, _stdlib: *mut ScfiaStdlib, _fork_sink: Option<Rc<RefCell<ForkSink>>>, _memory: &mut Memory32) -> Rc<RefCell<ActiveValue>> {
    return BVAddExpression::new((*state).x1.clone(), (*state).x2.clone(), _stdlib.as_mut().unwrap()).into();
}

pub unsafe fn test(state: *mut SystemState, _stdlib: *mut ScfiaStdlib, _fork_sink: Option<Rc<RefCell<ForkSink>>>, _memory: &mut Memory32) {
    (*state).x3 = BVAddExpression::new(sum(state, _stdlib, _fork_sink.clone(), _memory), sum(state, _stdlib, _fork_sink.clone(), _memory), _stdlib.as_mut().unwrap()).into();
}

pub unsafe fn step(state: *mut SystemState, _stdlib: *mut ScfiaStdlib, _fork_sink: Option<Rc<RefCell<ForkSink>>>, _memory: &mut Memory32) {
    let instruction_32: Rc<RefCell<ActiveValue>> = _memory.read((*state).pc.clone(), 32, _stdlib.as_mut().unwrap()).into();
    let opcode: Rc<RefCell<ActiveValue>> = BVSliceExpression::new(instruction_32.clone(), 6, 0, _stdlib.as_mut().unwrap()).into();
    if _stdlib.as_mut().unwrap().do_condition(BoolEqExpression::new(opcode.clone(), BitVectorConcrete::new(0b11, 7, _stdlib.as_mut().unwrap()).into(), _stdlib.as_mut().unwrap()).into(), _fork_sink.clone()) {
        let funct3: Rc<RefCell<ActiveValue>> = BVSliceExpression::new(instruction_32.clone(), 14, 12, _stdlib.as_mut().unwrap()).into();
        if _stdlib.as_mut().unwrap().do_condition(BoolEqExpression::new(funct3.clone(), BitVectorConcrete::new(0b10, 3, _stdlib.as_mut().unwrap()).into(), _stdlib.as_mut().unwrap()).into(), _fork_sink.clone()) {
            let rd: Rc<RefCell<ActiveValue>> = extract_rd_32(instruction_32.clone(), _stdlib, _fork_sink.clone(), _memory);
            let rs1: Rc<RefCell<ActiveValue>> = extract_rs1_32(instruction_32.clone(), _stdlib, _fork_sink.clone(), _memory);
            let imm: Rc<RefCell<ActiveValue>> = BVSliceExpression::new(instruction_32.clone(), 31, 20, _stdlib.as_mut().unwrap()).into();
            let imm32: Rc<RefCell<ActiveValue>> = BVSignExtendExpression::new(imm.clone(), 12, 32, _stdlib.as_mut().unwrap()).into();
            let base_address: Rc<RefCell<ActiveValue>> = register_read_BV32(state, rs1.clone(), _stdlib, _fork_sink.clone(), _memory);
            let address: Rc<RefCell<ActiveValue>> = BVAddExpression::new(base_address.clone(), imm32.clone(), _stdlib.as_mut().unwrap()).into();
            let value: Rc<RefCell<ActiveValue>> = _memory.read(address.clone(), 32, _stdlib.as_mut().unwrap()).into();
            register_write_BV32(state, rd.clone(), value.clone(), _stdlib, _fork_sink.clone(), _memory);
            progress_pc_4(state, _stdlib, _fork_sink.clone(), _memory);
        }
        else {
            unimplemented!();
        }
    }
    else if _stdlib.as_mut().unwrap().do_condition(BoolEqExpression::new(opcode.clone(), BitVectorConcrete::new(0b10011, 7, _stdlib.as_mut().unwrap()).into(), _stdlib.as_mut().unwrap()).into(), _fork_sink.clone()) {
        let funct3: Rc<RefCell<ActiveValue>> = BVSliceExpression::new(instruction_32.clone(), 14, 12, _stdlib.as_mut().unwrap()).into();
        if _stdlib.as_mut().unwrap().do_condition(BoolEqExpression::new(funct3.clone(), BitVectorConcrete::new(0b0, 3, _stdlib.as_mut().unwrap()).into(), _stdlib.as_mut().unwrap()).into(), _fork_sink.clone()) {
            let rd: Rc<RefCell<ActiveValue>> = extract_rd_32(instruction_32.clone(), _stdlib, _fork_sink.clone(), _memory);
            let rs1: Rc<RefCell<ActiveValue>> = extract_rs1_32(instruction_32.clone(), _stdlib, _fork_sink.clone(), _memory);
            let offset: Rc<RefCell<ActiveValue>> = BVSliceExpression::new(instruction_32.clone(), 31, 20, _stdlib.as_mut().unwrap()).into();
            let offset_32: Rc<RefCell<ActiveValue>> = BVSignExtendExpression::new(offset.clone(), 12, 32, _stdlib.as_mut().unwrap()).into();
            let result: Rc<RefCell<ActiveValue>> = BVAddExpression::new(offset_32.clone(), register_read_BV32(state, rs1.clone(), _stdlib, _fork_sink.clone(), _memory), _stdlib.as_mut().unwrap()).into();
            register_write_BV32(state, rd.clone(), result.clone(), _stdlib, _fork_sink.clone(), _memory);
            progress_pc_4(state, _stdlib, _fork_sink.clone(), _memory);
        }
        else if _stdlib.as_mut().unwrap().do_condition(BoolEqExpression::new(funct3.clone(), BitVectorConcrete::new(0b1, 3, _stdlib.as_mut().unwrap()).into(), _stdlib.as_mut().unwrap()).into(), _fork_sink.clone()) {
            let funct7: Rc<RefCell<ActiveValue>> = BVSliceExpression::new(instruction_32.clone(), 31, 25, _stdlib.as_mut().unwrap()).into();
            if _stdlib.as_mut().unwrap().do_condition(BoolEqExpression::new(funct7.clone(), BitVectorConcrete::new(0b0, 7, _stdlib.as_mut().unwrap()).into(), _stdlib.as_mut().unwrap()).into(), _fork_sink.clone()) {
                let rd: Rc<RefCell<ActiveValue>> = extract_rd_32(instruction_32.clone(), _stdlib, _fork_sink.clone(), _memory);
                let rs1: Rc<RefCell<ActiveValue>> = extract_rs1_32(instruction_32.clone(), _stdlib, _fork_sink.clone(), _memory);
                let shamt: Rc<RefCell<ActiveValue>> = BVSliceExpression::new(instruction_32.clone(), 24, 20, _stdlib.as_mut().unwrap()).into();
                let result: Rc<RefCell<ActiveValue>> = BVShiftLeftLogicalExpression::new(register_read_BV32(state, rs1.clone(), _stdlib, _fork_sink.clone(), _memory), shamt.clone(), 32, 5, _stdlib.as_mut().unwrap()).into();
                register_write_BV32(state, rd.clone(), result.clone(), _stdlib, _fork_sink.clone(), _memory);
                progress_pc_4(state, _stdlib, _fork_sink.clone(), _memory);
            }
            else {
                unimplemented!();
            }
        }
        else if _stdlib.as_mut().unwrap().do_condition(BoolEqExpression::new(funct3.clone(), BitVectorConcrete::new(0b101, 3, _stdlib.as_mut().unwrap()).into(), _stdlib.as_mut().unwrap()).into(), _fork_sink.clone()) {
            let funct7: Rc<RefCell<ActiveValue>> = BVSliceExpression::new(instruction_32.clone(), 31, 25, _stdlib.as_mut().unwrap()).into();
            if _stdlib.as_mut().unwrap().do_condition(BoolEqExpression::new(funct7.clone(), BitVectorConcrete::new(0b0, 7, _stdlib.as_mut().unwrap()).into(), _stdlib.as_mut().unwrap()).into(), _fork_sink.clone()) {
                let rd: Rc<RefCell<ActiveValue>> = extract_rd_32(instruction_32.clone(), _stdlib, _fork_sink.clone(), _memory);
                let rs1: Rc<RefCell<ActiveValue>> = extract_rs1_32(instruction_32.clone(), _stdlib, _fork_sink.clone(), _memory);
                let shamt: Rc<RefCell<ActiveValue>> = BVSliceExpression::new(instruction_32.clone(), 24, 20, _stdlib.as_mut().unwrap()).into();
                let result: Rc<RefCell<ActiveValue>> = BVShiftRightLogicalExpression::new(register_read_BV32(state, rs1.clone(), _stdlib, _fork_sink.clone(), _memory), shamt.clone(), 32, 5, _stdlib.as_mut().unwrap()).into();
                register_write_BV32(state, rd.clone(), result.clone(), _stdlib, _fork_sink.clone(), _memory);
                progress_pc_4(state, _stdlib, _fork_sink.clone(), _memory);
            }
            else {
                unimplemented!();
            }
        }
        else if _stdlib.as_mut().unwrap().do_condition(BoolEqExpression::new(funct3.clone(), BitVectorConcrete::new(0b110, 3, _stdlib.as_mut().unwrap()).into(), _stdlib.as_mut().unwrap()).into(), _fork_sink.clone()) {
            let rd: Rc<RefCell<ActiveValue>> = extract_rd_32(instruction_32.clone(), _stdlib, _fork_sink.clone(), _memory);
            let rs1: Rc<RefCell<ActiveValue>> = extract_rs1_32(instruction_32.clone(), _stdlib, _fork_sink.clone(), _memory);
            let imm: Rc<RefCell<ActiveValue>> = BVSliceExpression::new(instruction_32.clone(), 31, 20, _stdlib.as_mut().unwrap()).into();
            let imm_32: Rc<RefCell<ActiveValue>> = BVSignExtendExpression::new(imm.clone(), 12, 32, _stdlib.as_mut().unwrap()).into();
            let result: Rc<RefCell<ActiveValue>> = BVOrExpression::new(register_read_BV32(state, rs1.clone(), _stdlib, _fork_sink.clone(), _memory), imm_32.clone(), _stdlib.as_mut().unwrap()).into();
            register_write_BV32(state, rd.clone(), result.clone(), _stdlib, _fork_sink.clone(), _memory);
            progress_pc_4(state, _stdlib, _fork_sink.clone(), _memory);
        }
        else if _stdlib.as_mut().unwrap().do_condition(BoolEqExpression::new(funct3.clone(), BitVectorConcrete::new(0b111, 3, _stdlib.as_mut().unwrap()).into(), _stdlib.as_mut().unwrap()).into(), _fork_sink.clone()) {
            let rd: Rc<RefCell<ActiveValue>> = extract_rd_32(instruction_32.clone(), _stdlib, _fork_sink.clone(), _memory);
            let rs1: Rc<RefCell<ActiveValue>> = extract_rs1_32(instruction_32.clone(), _stdlib, _fork_sink.clone(), _memory);
            let imm: Rc<RefCell<ActiveValue>> = BVSliceExpression::new(instruction_32.clone(), 31, 20, _stdlib.as_mut().unwrap()).into();
            let imm_32: Rc<RefCell<ActiveValue>> = BVSignExtendExpression::new(imm.clone(), 12, 32, _stdlib.as_mut().unwrap()).into();
            let result: Rc<RefCell<ActiveValue>> = BVAndExpression::new(register_read_BV32(state, rs1.clone(), _stdlib, _fork_sink.clone(), _memory), imm_32.clone(), _stdlib.as_mut().unwrap()).into();
            register_write_BV32(state, rd.clone(), result.clone(), _stdlib, _fork_sink.clone(), _memory);
            progress_pc_4(state, _stdlib, _fork_sink.clone(), _memory);
        }
        else {
            unimplemented!();
        }
    }
    else if _stdlib.as_mut().unwrap().do_condition(BoolEqExpression::new(opcode.clone(), BitVectorConcrete::new(0b100011, 7, _stdlib.as_mut().unwrap()).into(), _stdlib.as_mut().unwrap()).into(), _fork_sink.clone()) {
        let funct3: Rc<RefCell<ActiveValue>> = BVSliceExpression::new(instruction_32.clone(), 14, 12, _stdlib.as_mut().unwrap()).into();
        if _stdlib.as_mut().unwrap().do_condition(BoolEqExpression::new(funct3.clone(), BitVectorConcrete::new(0b0, 3, _stdlib.as_mut().unwrap()).into(), _stdlib.as_mut().unwrap()).into(), _fork_sink.clone()) {
            let rs1: Rc<RefCell<ActiveValue>> = extract_rs1_32(instruction_32.clone(), _stdlib, _fork_sink.clone(), _memory);
            let rs2: Rc<RefCell<ActiveValue>> = extract_rs2_32(instruction_32.clone(), _stdlib, _fork_sink.clone(), _memory);
            let offset_11_5: Rc<RefCell<ActiveValue>> = BVSliceExpression::new(instruction_32.clone(), 31, 25, _stdlib.as_mut().unwrap()).into();
            let offset_4_0: Rc<RefCell<ActiveValue>> = BVSliceExpression::new(instruction_32.clone(), 11, 7, _stdlib.as_mut().unwrap()).into();
            let offset: Rc<RefCell<ActiveValue>> = BVConcatExpression::new(offset_11_5.clone(), offset_4_0.clone(), _stdlib.as_mut().unwrap()).into();
            let offset_32: Rc<RefCell<ActiveValue>> = BVSignExtendExpression::new(offset.clone(), 12, 32, _stdlib.as_mut().unwrap()).into();
            let base_address: Rc<RefCell<ActiveValue>> = register_read_BV32(state, rs1.clone(), _stdlib, _fork_sink.clone(), _memory);
            let address: Rc<RefCell<ActiveValue>> = BVAddExpression::new(base_address.clone(), offset_32.clone(), _stdlib.as_mut().unwrap()).into();
            let value_32: Rc<RefCell<ActiveValue>> = register_read_BV32(state, rs2.clone(), _stdlib, _fork_sink.clone(), _memory);
            let value: Rc<RefCell<ActiveValue>> = BVSliceExpression::new(value_32.clone(), 7, 0, _stdlib.as_mut().unwrap()).into();
            _memory.write(address.clone(), value.clone(), _stdlib.as_mut().unwrap());
            progress_pc_4(state, _stdlib, _fork_sink.clone(), _memory);
        }
        else if _stdlib.as_mut().unwrap().do_condition(BoolEqExpression::new(funct3.clone(), BitVectorConcrete::new(0b1, 3, _stdlib.as_mut().unwrap()).into(), _stdlib.as_mut().unwrap()).into(), _fork_sink.clone()) {
            unimplemented!();
        }
        else if _stdlib.as_mut().unwrap().do_condition(BoolEqExpression::new(funct3.clone(), BitVectorConcrete::new(0b10, 3, _stdlib.as_mut().unwrap()).into(), _stdlib.as_mut().unwrap()).into(), _fork_sink.clone()) {
            let rs1: Rc<RefCell<ActiveValue>> = extract_rs1_32(instruction_32.clone(), _stdlib, _fork_sink.clone(), _memory);
            let rs2: Rc<RefCell<ActiveValue>> = extract_rs2_32(instruction_32.clone(), _stdlib, _fork_sink.clone(), _memory);
            let offset_11_5: Rc<RefCell<ActiveValue>> = BVSliceExpression::new(instruction_32.clone(), 31, 25, _stdlib.as_mut().unwrap()).into();
            let offset_4_0: Rc<RefCell<ActiveValue>> = BVSliceExpression::new(instruction_32.clone(), 11, 7, _stdlib.as_mut().unwrap()).into();
            let offset: Rc<RefCell<ActiveValue>> = BVConcatExpression::new(offset_11_5.clone(), offset_4_0.clone(), _stdlib.as_mut().unwrap()).into();
            let offset_32: Rc<RefCell<ActiveValue>> = BVSignExtendExpression::new(offset.clone(), 12, 32, _stdlib.as_mut().unwrap()).into();
            let base_address: Rc<RefCell<ActiveValue>> = register_read_BV32(state, rs1.clone(), _stdlib, _fork_sink.clone(), _memory);
            let address: Rc<RefCell<ActiveValue>> = BVAddExpression::new(base_address.clone(), offset_32.clone(), _stdlib.as_mut().unwrap()).into();
            let value: Rc<RefCell<ActiveValue>> = register_read_BV32(state, rs2.clone(), _stdlib, _fork_sink.clone(), _memory);
            _memory.write(address.clone(), value.clone(), _stdlib.as_mut().unwrap());
            progress_pc_4(state, _stdlib, _fork_sink.clone(), _memory);
        }
        else {
            unimplemented!();
        }
    }
    else if _stdlib.as_mut().unwrap().do_condition(BoolEqExpression::new(opcode.clone(), BitVectorConcrete::new(0b110111, 7, _stdlib.as_mut().unwrap()).into(), _stdlib.as_mut().unwrap()).into(), _fork_sink.clone()) {
        let rd: Rc<RefCell<ActiveValue>> = extract_rd_32(instruction_32.clone(), _stdlib, _fork_sink.clone(), _memory);
        let rs: Rc<RefCell<ActiveValue>> = extract_rs1_32(instruction_32.clone(), _stdlib, _fork_sink.clone(), _memory);
        let imm: Rc<RefCell<ActiveValue>> = BVSliceExpression::new(instruction_32.clone(), 31, 12, _stdlib.as_mut().unwrap()).into();
        let value: Rc<RefCell<ActiveValue>> = BVConcatExpression::new(imm.clone(), BitVectorConcrete::new(0b0, 12, _stdlib.as_mut().unwrap()).into(), _stdlib.as_mut().unwrap()).into();
        register_write_BV32(state, rd.clone(), value.clone(), _stdlib, _fork_sink.clone(), _memory);
        progress_pc_4(state, _stdlib, _fork_sink.clone(), _memory);
    }
    else if _stdlib.as_mut().unwrap().do_condition(BoolEqExpression::new(opcode.clone(), BitVectorConcrete::new(0b10111, 7, _stdlib.as_mut().unwrap()).into(), _stdlib.as_mut().unwrap()).into(), _fork_sink.clone()) {
        let dst: Rc<RefCell<ActiveValue>> = extract_rd_32(instruction_32.clone(), _stdlib, _fork_sink.clone(), _memory);
        let imm: Rc<RefCell<ActiveValue>> = BVSliceExpression::new(instruction_32.clone(), 31, 12, _stdlib.as_mut().unwrap()).into();
        let imm32: Rc<RefCell<ActiveValue>> = BVConcatExpression::new(imm.clone(), BitVectorConcrete::new(0b0, 12, _stdlib.as_mut().unwrap()).into(), _stdlib.as_mut().unwrap()).into();
        let sum: Rc<RefCell<ActiveValue>> = BVAddExpression::new(imm32.clone(), (*state).pc.clone(), _stdlib.as_mut().unwrap()).into();
        register_write_BV32(state, dst.clone(), sum.clone(), _stdlib, _fork_sink.clone(), _memory);
        progress_pc_4(state, _stdlib, _fork_sink.clone(), _memory);
    }
    else if _stdlib.as_mut().unwrap().do_condition(BoolEqExpression::new(opcode.clone(), BitVectorConcrete::new(0b110011, 7, _stdlib.as_mut().unwrap()).into(), _stdlib.as_mut().unwrap()).into(), _fork_sink.clone()) {
        let funct3: Rc<RefCell<ActiveValue>> = BVSliceExpression::new(instruction_32.clone(), 14, 12, _stdlib.as_mut().unwrap()).into();
        let funct7: Rc<RefCell<ActiveValue>> = BVSliceExpression::new(instruction_32.clone(), 31, 25, _stdlib.as_mut().unwrap()).into();
        let rd: Rc<RefCell<ActiveValue>> = BVSliceExpression::new(instruction_32.clone(), 11, 7, _stdlib.as_mut().unwrap()).into();
        let rs1: Rc<RefCell<ActiveValue>> = BVSliceExpression::new(instruction_32.clone(), 19, 15, _stdlib.as_mut().unwrap()).into();
        let rs2: Rc<RefCell<ActiveValue>> = BVSliceExpression::new(instruction_32.clone(), 24, 20, _stdlib.as_mut().unwrap()).into();
        if _stdlib.as_mut().unwrap().do_condition(BoolEqExpression::new(funct3.clone(), BitVectorConcrete::new(0b0, 3, _stdlib.as_mut().unwrap()).into(), _stdlib.as_mut().unwrap()).into(), _fork_sink.clone()) {
            if _stdlib.as_mut().unwrap().do_condition(BoolEqExpression::new(funct7.clone(), BitVectorConcrete::new(0b0, 7, _stdlib.as_mut().unwrap()).into(), _stdlib.as_mut().unwrap()).into(), _fork_sink.clone()) {
                execute_add32(state, rd.clone(), rs1.clone(), rs2.clone(), _stdlib, _fork_sink.clone(), _memory);
                progress_pc_4(state, _stdlib, _fork_sink.clone(), _memory);
            }
            else if _stdlib.as_mut().unwrap().do_condition(BoolEqExpression::new(funct7.clone(), BitVectorConcrete::new(0b100000, 7, _stdlib.as_mut().unwrap()).into(), _stdlib.as_mut().unwrap()).into(), _fork_sink.clone()) {
                (*state).pc = BitVectorConcrete::new(0b0, 32, _stdlib.as_mut().unwrap()).into();
                unimplemented!();
            }
            else {
                unimplemented!();
            }
        }
        else if _stdlib.as_mut().unwrap().do_condition(BoolEqExpression::new(funct3.clone(), BitVectorConcrete::new(0b1, 3, _stdlib.as_mut().unwrap()).into(), _stdlib.as_mut().unwrap()).into(), _fork_sink.clone()) {
            if _stdlib.as_mut().unwrap().do_condition(BoolEqExpression::new(funct7.clone(), BitVectorConcrete::new(0b0, 7, _stdlib.as_mut().unwrap()).into(), _stdlib.as_mut().unwrap()).into(), _fork_sink.clone()) {
                unimplemented!();
            }
            else {
                unimplemented!();
            }
        }
        else if _stdlib.as_mut().unwrap().do_condition(BoolEqExpression::new(funct3.clone(), BitVectorConcrete::new(0b10, 3, _stdlib.as_mut().unwrap()).into(), _stdlib.as_mut().unwrap()).into(), _fork_sink.clone()) {
            if _stdlib.as_mut().unwrap().do_condition(BoolEqExpression::new(funct7.clone(), BitVectorConcrete::new(0b0, 7, _stdlib.as_mut().unwrap()).into(), _stdlib.as_mut().unwrap()).into(), _fork_sink.clone()) {
                unimplemented!();
            }
            else {
                unimplemented!();
            }
        }
        else if _stdlib.as_mut().unwrap().do_condition(BoolEqExpression::new(funct3.clone(), BitVectorConcrete::new(0b11, 3, _stdlib.as_mut().unwrap()).into(), _stdlib.as_mut().unwrap()).into(), _fork_sink.clone()) {
            if _stdlib.as_mut().unwrap().do_condition(BoolEqExpression::new(funct7.clone(), BitVectorConcrete::new(0b0, 7, _stdlib.as_mut().unwrap()).into(), _stdlib.as_mut().unwrap()).into(), _fork_sink.clone()) {
                unimplemented!();
            }
            else {
                unimplemented!();
            }
        }
        else if _stdlib.as_mut().unwrap().do_condition(BoolEqExpression::new(funct3.clone(), BitVectorConcrete::new(0b100, 3, _stdlib.as_mut().unwrap()).into(), _stdlib.as_mut().unwrap()).into(), _fork_sink.clone()) {
            if _stdlib.as_mut().unwrap().do_condition(BoolEqExpression::new(funct7.clone(), BitVectorConcrete::new(0b0, 7, _stdlib.as_mut().unwrap()).into(), _stdlib.as_mut().unwrap()).into(), _fork_sink.clone()) {
                unimplemented!();
            }
            else {
                unimplemented!();
            }
        }
        else if _stdlib.as_mut().unwrap().do_condition(BoolEqExpression::new(funct3.clone(), BitVectorConcrete::new(0b101, 3, _stdlib.as_mut().unwrap()).into(), _stdlib.as_mut().unwrap()).into(), _fork_sink.clone()) {
            if _stdlib.as_mut().unwrap().do_condition(BoolEqExpression::new(funct7.clone(), BitVectorConcrete::new(0b0, 7, _stdlib.as_mut().unwrap()).into(), _stdlib.as_mut().unwrap()).into(), _fork_sink.clone()) {
                unimplemented!();
            }
            else if _stdlib.as_mut().unwrap().do_condition(BoolEqExpression::new(funct7.clone(), BitVectorConcrete::new(0b100000, 7, _stdlib.as_mut().unwrap()).into(), _stdlib.as_mut().unwrap()).into(), _fork_sink.clone()) {
                unimplemented!();
            }
            else {
                unimplemented!();
            }
        }
        else if _stdlib.as_mut().unwrap().do_condition(BoolEqExpression::new(funct3.clone(), BitVectorConcrete::new(0b110, 3, _stdlib.as_mut().unwrap()).into(), _stdlib.as_mut().unwrap()).into(), _fork_sink.clone()) {
            if _stdlib.as_mut().unwrap().do_condition(BoolEqExpression::new(funct7.clone(), BitVectorConcrete::new(0b0, 7, _stdlib.as_mut().unwrap()).into(), _stdlib.as_mut().unwrap()).into(), _fork_sink.clone()) {
                unimplemented!();
            }
            else {
                unimplemented!();
            }
        }
        else if _stdlib.as_mut().unwrap().do_condition(BoolEqExpression::new(funct3.clone(), BitVectorConcrete::new(0b111, 3, _stdlib.as_mut().unwrap()).into(), _stdlib.as_mut().unwrap()).into(), _fork_sink.clone()) {
            if _stdlib.as_mut().unwrap().do_condition(BoolEqExpression::new(funct7.clone(), BitVectorConcrete::new(0b0, 7, _stdlib.as_mut().unwrap()).into(), _stdlib.as_mut().unwrap()).into(), _fork_sink.clone()) {
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
    else if _stdlib.as_mut().unwrap().do_condition(BoolEqExpression::new(opcode.clone(), BitVectorConcrete::new(0b1100011, 7, _stdlib.as_mut().unwrap()).into(), _stdlib.as_mut().unwrap()).into(), _fork_sink.clone()) {
        let funct3: Rc<RefCell<ActiveValue>> = BVSliceExpression::new(instruction_32.clone(), 14, 12, _stdlib.as_mut().unwrap()).into();
        if _stdlib.as_mut().unwrap().do_condition(BoolEqExpression::new(funct3.clone(), BitVectorConcrete::new(0b0, 3, _stdlib.as_mut().unwrap()).into(), _stdlib.as_mut().unwrap()).into(), _fork_sink.clone()) {
            let lhs: Rc<RefCell<ActiveValue>> = register_read_BV32(state, extract_rs1_32(instruction_32.clone(), _stdlib, _fork_sink.clone(), _memory), _stdlib, _fork_sink.clone(), _memory);
            let rhs: Rc<RefCell<ActiveValue>> = register_read_BV32(state, extract_rs2_32(instruction_32.clone(), _stdlib, _fork_sink.clone(), _memory), _stdlib, _fork_sink.clone(), _memory);
            if _stdlib.as_mut().unwrap().do_condition(BoolEqExpression::new(lhs.clone(), rhs.clone(), _stdlib.as_mut().unwrap()).into(), _fork_sink.clone()) {
                let imm_4_1: Rc<RefCell<ActiveValue>> = BVSliceExpression::new(instruction_32.clone(), 11, 8, _stdlib.as_mut().unwrap()).into();
                let imm_10_5: Rc<RefCell<ActiveValue>> = BVSliceExpression::new(instruction_32.clone(), 30, 25, _stdlib.as_mut().unwrap()).into();
                let imm11: Rc<RefCell<ActiveValue>> = BVSliceExpression::new(instruction_32.clone(), 7, 7, _stdlib.as_mut().unwrap()).into();
                let imm12: Rc<RefCell<ActiveValue>> = BVSliceExpression::new(instruction_32.clone(), 31, 31, _stdlib.as_mut().unwrap()).into();
                let imm_4_0: Rc<RefCell<ActiveValue>> = BVConcatExpression::new(imm_4_1.clone(), BitVectorConcrete::new(0b0, 1, _stdlib.as_mut().unwrap()).into(), _stdlib.as_mut().unwrap()).into();
                let imm_10_0: Rc<RefCell<ActiveValue>> = BVConcatExpression::new(imm_10_5.clone(), imm_4_0.clone(), _stdlib.as_mut().unwrap()).into();
                let imm_11_0: Rc<RefCell<ActiveValue>> = BVConcatExpression::new(imm11.clone(), imm_10_0.clone(), _stdlib.as_mut().unwrap()).into();
                let imm_12_0: Rc<RefCell<ActiveValue>> = BVConcatExpression::new(imm12.clone(), imm_11_0.clone(), _stdlib.as_mut().unwrap()).into();
                let offset: Rc<RefCell<ActiveValue>> = BVSignExtendExpression::new(imm_12_0.clone(), 13, 32, _stdlib.as_mut().unwrap()).into();
                let address: Rc<RefCell<ActiveValue>> = BVAddExpression::new((*state).pc.clone(), offset.clone(), _stdlib.as_mut().unwrap()).into();
                (*state).pc = address.clone();
            }
            else {
                progress_pc_4(state, _stdlib, _fork_sink.clone(), _memory);
            }
        }
        else if _stdlib.as_mut().unwrap().do_condition(BoolEqExpression::new(funct3.clone(), BitVectorConcrete::new(0b1, 3, _stdlib.as_mut().unwrap()).into(), _stdlib.as_mut().unwrap()).into(), _fork_sink.clone()) {
            let lhs: Rc<RefCell<ActiveValue>> = register_read_BV32(state, extract_rs1_32(instruction_32.clone(), _stdlib, _fork_sink.clone(), _memory), _stdlib, _fork_sink.clone(), _memory);
            let rhs: Rc<RefCell<ActiveValue>> = register_read_BV32(state, extract_rs2_32(instruction_32.clone(), _stdlib, _fork_sink.clone(), _memory), _stdlib, _fork_sink.clone(), _memory);
            if _stdlib.as_mut().unwrap().do_condition(BoolNEqExpression::new(lhs.clone(), rhs.clone(), _stdlib.as_mut().unwrap()).into(), _fork_sink.clone()) {
                let imm_4_1: Rc<RefCell<ActiveValue>> = BVSliceExpression::new(instruction_32.clone(), 11, 8, _stdlib.as_mut().unwrap()).into();
                let imm_10_5: Rc<RefCell<ActiveValue>> = BVSliceExpression::new(instruction_32.clone(), 30, 25, _stdlib.as_mut().unwrap()).into();
                let imm11: Rc<RefCell<ActiveValue>> = BVSliceExpression::new(instruction_32.clone(), 7, 7, _stdlib.as_mut().unwrap()).into();
                let imm12: Rc<RefCell<ActiveValue>> = BVSliceExpression::new(instruction_32.clone(), 31, 31, _stdlib.as_mut().unwrap()).into();
                let imm_4_0: Rc<RefCell<ActiveValue>> = BVConcatExpression::new(imm_4_1.clone(), BitVectorConcrete::new(0b0, 1, _stdlib.as_mut().unwrap()).into(), _stdlib.as_mut().unwrap()).into();
                let imm_10_0: Rc<RefCell<ActiveValue>> = BVConcatExpression::new(imm_10_5.clone(), imm_4_0.clone(), _stdlib.as_mut().unwrap()).into();
                let imm_11_0: Rc<RefCell<ActiveValue>> = BVConcatExpression::new(imm11.clone(), imm_10_0.clone(), _stdlib.as_mut().unwrap()).into();
                let imm_12_0: Rc<RefCell<ActiveValue>> = BVConcatExpression::new(imm12.clone(), imm_11_0.clone(), _stdlib.as_mut().unwrap()).into();
                let offset: Rc<RefCell<ActiveValue>> = BVSignExtendExpression::new(imm_12_0.clone(), 13, 32, _stdlib.as_mut().unwrap()).into();
                let address: Rc<RefCell<ActiveValue>> = BVAddExpression::new((*state).pc.clone(), offset.clone(), _stdlib.as_mut().unwrap()).into();
                (*state).pc = address.clone();
            }
            else {
                progress_pc_4(state, _stdlib, _fork_sink.clone(), _memory);
            }
        }
        else if _stdlib.as_mut().unwrap().do_condition(BoolEqExpression::new(funct3.clone(), BitVectorConcrete::new(0b110, 3, _stdlib.as_mut().unwrap()).into(), _stdlib.as_mut().unwrap()).into(), _fork_sink.clone()) {
            let lhs: Rc<RefCell<ActiveValue>> = register_read_BV32(state, extract_rs1_32(instruction_32.clone(), _stdlib, _fork_sink.clone(), _memory), _stdlib, _fork_sink.clone(), _memory);
            let rhs: Rc<RefCell<ActiveValue>> = register_read_BV32(state, extract_rs2_32(instruction_32.clone(), _stdlib, _fork_sink.clone(), _memory), _stdlib, _fork_sink.clone(), _memory);
            if _stdlib.as_mut().unwrap().do_condition(BoolLessThanUIntExpression::new(lhs.clone(), rhs.clone(), _stdlib.as_mut().unwrap()).into(), _fork_sink.clone()) {
                let imm_4_1: Rc<RefCell<ActiveValue>> = BVSliceExpression::new(instruction_32.clone(), 11, 8, _stdlib.as_mut().unwrap()).into();
                let imm_10_5: Rc<RefCell<ActiveValue>> = BVSliceExpression::new(instruction_32.clone(), 30, 25, _stdlib.as_mut().unwrap()).into();
                let imm11: Rc<RefCell<ActiveValue>> = BVSliceExpression::new(instruction_32.clone(), 7, 7, _stdlib.as_mut().unwrap()).into();
                let imm12: Rc<RefCell<ActiveValue>> = BVSliceExpression::new(instruction_32.clone(), 31, 31, _stdlib.as_mut().unwrap()).into();
                let imm_4_0: Rc<RefCell<ActiveValue>> = BVConcatExpression::new(imm_4_1.clone(), BitVectorConcrete::new(0b0, 1, _stdlib.as_mut().unwrap()).into(), _stdlib.as_mut().unwrap()).into();
                let imm_10_0: Rc<RefCell<ActiveValue>> = BVConcatExpression::new(imm_10_5.clone(), imm_4_0.clone(), _stdlib.as_mut().unwrap()).into();
                let imm_11_0: Rc<RefCell<ActiveValue>> = BVConcatExpression::new(imm11.clone(), imm_10_0.clone(), _stdlib.as_mut().unwrap()).into();
                let imm_12_0: Rc<RefCell<ActiveValue>> = BVConcatExpression::new(imm12.clone(), imm_11_0.clone(), _stdlib.as_mut().unwrap()).into();
                let offset: Rc<RefCell<ActiveValue>> = BVSignExtendExpression::new(imm_12_0.clone(), 13, 32, _stdlib.as_mut().unwrap()).into();
                let address: Rc<RefCell<ActiveValue>> = BVAddExpression::new((*state).pc.clone(), offset.clone(), _stdlib.as_mut().unwrap()).into();
                (*state).pc = address.clone();
            }
            else {
                progress_pc_4(state, _stdlib, _fork_sink.clone(), _memory);
            }
        }
        else if _stdlib.as_mut().unwrap().do_condition(BoolEqExpression::new(funct3.clone(), BitVectorConcrete::new(0b111, 3, _stdlib.as_mut().unwrap()).into(), _stdlib.as_mut().unwrap()).into(), _fork_sink.clone()) {
            let lhs: Rc<RefCell<ActiveValue>> = register_read_BV32(state, extract_rs1_32(instruction_32.clone(), _stdlib, _fork_sink.clone(), _memory), _stdlib, _fork_sink.clone(), _memory);
            let rhs: Rc<RefCell<ActiveValue>> = register_read_BV32(state, extract_rs2_32(instruction_32.clone(), _stdlib, _fork_sink.clone(), _memory), _stdlib, _fork_sink.clone(), _memory);
            if _stdlib.as_mut().unwrap().do_condition(BoolLessThanUIntExpression::new(rhs.clone(), lhs.clone(), _stdlib.as_mut().unwrap()).into(), _fork_sink.clone()) {
                let imm_4_1: Rc<RefCell<ActiveValue>> = BVSliceExpression::new(instruction_32.clone(), 11, 8, _stdlib.as_mut().unwrap()).into();
                let imm_10_5: Rc<RefCell<ActiveValue>> = BVSliceExpression::new(instruction_32.clone(), 30, 25, _stdlib.as_mut().unwrap()).into();
                let imm11: Rc<RefCell<ActiveValue>> = BVSliceExpression::new(instruction_32.clone(), 7, 7, _stdlib.as_mut().unwrap()).into();
                let imm12: Rc<RefCell<ActiveValue>> = BVSliceExpression::new(instruction_32.clone(), 31, 31, _stdlib.as_mut().unwrap()).into();
                let imm_4_0: Rc<RefCell<ActiveValue>> = BVConcatExpression::new(imm_4_1.clone(), BitVectorConcrete::new(0b0, 1, _stdlib.as_mut().unwrap()).into(), _stdlib.as_mut().unwrap()).into();
                let imm_10_0: Rc<RefCell<ActiveValue>> = BVConcatExpression::new(imm_10_5.clone(), imm_4_0.clone(), _stdlib.as_mut().unwrap()).into();
                let imm_11_0: Rc<RefCell<ActiveValue>> = BVConcatExpression::new(imm11.clone(), imm_10_0.clone(), _stdlib.as_mut().unwrap()).into();
                let imm_12_0: Rc<RefCell<ActiveValue>> = BVConcatExpression::new(imm12.clone(), imm_11_0.clone(), _stdlib.as_mut().unwrap()).into();
                let offset: Rc<RefCell<ActiveValue>> = BVSignExtendExpression::new(imm_12_0.clone(), 13, 32, _stdlib.as_mut().unwrap()).into();
                let address: Rc<RefCell<ActiveValue>> = BVAddExpression::new((*state).pc.clone(), offset.clone(), _stdlib.as_mut().unwrap()).into();
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
    else if _stdlib.as_mut().unwrap().do_condition(BoolEqExpression::new(opcode.clone(), BitVectorConcrete::new(0b1100111, 7, _stdlib.as_mut().unwrap()).into(), _stdlib.as_mut().unwrap()).into(), _fork_sink.clone()) {
        let funct3: Rc<RefCell<ActiveValue>> = BVSliceExpression::new(instruction_32.clone(), 14, 12, _stdlib.as_mut().unwrap()).into();
        if _stdlib.as_mut().unwrap().do_condition(BoolEqExpression::new(funct3.clone(), BitVectorConcrete::new(0b0, 3, _stdlib.as_mut().unwrap()).into(), _stdlib.as_mut().unwrap()).into(), _fork_sink.clone()) {
            let dst: Rc<RefCell<ActiveValue>> = extract_rd_32(instruction_32.clone(), _stdlib, _fork_sink.clone(), _memory);
            let rs1: Rc<RefCell<ActiveValue>> = extract_rs1_32(instruction_32.clone(), _stdlib, _fork_sink.clone(), _memory);
            let s1: Rc<RefCell<ActiveValue>> = register_read_BV32(state, rs1.clone(), _stdlib, _fork_sink.clone(), _memory);
            let offset: Rc<RefCell<ActiveValue>> = BVSliceExpression::new(instruction_32.clone(), 31, 20, _stdlib.as_mut().unwrap()).into();
            let offset_32: Rc<RefCell<ActiveValue>> = BVSignExtendExpression::new(offset.clone(), 12, 32, _stdlib.as_mut().unwrap()).into();
            let address: Rc<RefCell<ActiveValue>> = BVAddExpression::new(s1.clone(), offset_32.clone(), _stdlib.as_mut().unwrap()).into();
            let return_address: Rc<RefCell<ActiveValue>> = BVAddExpression::new((*state).pc.clone(), BitVectorConcrete::new(0b100, 32, _stdlib.as_mut().unwrap()).into(), _stdlib.as_mut().unwrap()).into();
            (*state).pc = address.clone();
            register_write_BV32(state, dst.clone(), return_address.clone(), _stdlib, _fork_sink.clone(), _memory);
        }
        else {
            unimplemented!();
        }
    }
    else if _stdlib.as_mut().unwrap().do_condition(BoolEqExpression::new(opcode.clone(), BitVectorConcrete::new(0b1101111, 7, _stdlib.as_mut().unwrap()).into(), _stdlib.as_mut().unwrap()).into(), _fork_sink.clone()) {
        let imm_20: Rc<RefCell<ActiveValue>> = BVSliceExpression::new(instruction_32.clone(), 31, 31, _stdlib.as_mut().unwrap()).into();
        let imm_10_1: Rc<RefCell<ActiveValue>> = BVSliceExpression::new(instruction_32.clone(), 30, 21, _stdlib.as_mut().unwrap()).into();
        let imm_11: Rc<RefCell<ActiveValue>> = BVSliceExpression::new(instruction_32.clone(), 20, 20, _stdlib.as_mut().unwrap()).into();
        let imm_19_12: Rc<RefCell<ActiveValue>> = BVSliceExpression::new(instruction_32.clone(), 19, 12, _stdlib.as_mut().unwrap()).into();
        let offset_10_0: Rc<RefCell<ActiveValue>> = BVConcatExpression::new(imm_10_1.clone(), BitVectorConcrete::new(0b0, 1, _stdlib.as_mut().unwrap()).into(), _stdlib.as_mut().unwrap()).into();
        let offset_11_0: Rc<RefCell<ActiveValue>> = BVConcatExpression::new(imm_11.clone(), offset_10_0.clone(), _stdlib.as_mut().unwrap()).into();
        let offset_19_0: Rc<RefCell<ActiveValue>> = BVConcatExpression::new(imm_19_12.clone(), offset_11_0.clone(), _stdlib.as_mut().unwrap()).into();
        let offset_20_0: Rc<RefCell<ActiveValue>> = BVConcatExpression::new(imm_20.clone(), offset_19_0.clone(), _stdlib.as_mut().unwrap()).into();
        let offset_32: Rc<RefCell<ActiveValue>> = BVSignExtendExpression::new(offset_20_0.clone(), 21, 32, _stdlib.as_mut().unwrap()).into();
        let address: Rc<RefCell<ActiveValue>> = BVAddExpression::new((*state).pc.clone(), offset_32.clone(), _stdlib.as_mut().unwrap()).into();
        let return_address: Rc<RefCell<ActiveValue>> = BVAddExpression::new((*state).pc.clone(), BitVectorConcrete::new(0b100, 32, _stdlib.as_mut().unwrap()).into(), _stdlib.as_mut().unwrap()).into();
        let dst: Rc<RefCell<ActiveValue>> = extract_rd_32(instruction_32.clone(), _stdlib, _fork_sink.clone(), _memory);
        register_write_BV32(state, dst.clone(), return_address.clone(), _stdlib, _fork_sink.clone(), _memory);
        (*state).pc = address.clone();
    }
    else {
        unimplemented!();
    }
}

pub unsafe fn extract_rd_32(op: Rc<RefCell<ActiveValue>>, _stdlib: *mut ScfiaStdlib, _fork_sink: Option<Rc<RefCell<ForkSink>>>, _memory: &mut Memory32) -> Rc<RefCell<ActiveValue>> {
    return BVSliceExpression::new(op.clone(), 11, 7, _stdlib.as_mut().unwrap()).into();
}

pub unsafe fn extract_rs1_32(op: Rc<RefCell<ActiveValue>>, _stdlib: *mut ScfiaStdlib, _fork_sink: Option<Rc<RefCell<ForkSink>>>, _memory: &mut Memory32) -> Rc<RefCell<ActiveValue>> {
    return BVSliceExpression::new(op.clone(), 19, 15, _stdlib.as_mut().unwrap()).into();
}

pub unsafe fn extract_rs2_32(op: Rc<RefCell<ActiveValue>>, _stdlib: *mut ScfiaStdlib, _fork_sink: Option<Rc<RefCell<ForkSink>>>, _memory: &mut Memory32) -> Rc<RefCell<ActiveValue>> {
    return BVSliceExpression::new(op.clone(), 24, 20, _stdlib.as_mut().unwrap()).into();
}

pub unsafe fn progress_pc_4(state: *mut SystemState, _stdlib: *mut ScfiaStdlib, _fork_sink: Option<Rc<RefCell<ForkSink>>>, _memory: &mut Memory32) {
    let old_pc: Rc<RefCell<ActiveValue>> = (*state).pc.clone();
    let new_pc: Rc<RefCell<ActiveValue>> = BVAddExpression::new(old_pc.clone(), BitVectorConcrete::new(0b100, 32, _stdlib.as_mut().unwrap()).into(), _stdlib.as_mut().unwrap()).into();
    (*state).pc = new_pc.clone();
}

pub unsafe fn register_write_BV32(state: *mut SystemState, register_id: Rc<RefCell<ActiveValue>>, value: Rc<RefCell<ActiveValue>>, _stdlib: *mut ScfiaStdlib, _fork_sink: Option<Rc<RefCell<ForkSink>>>, _memory: &mut Memory32) {
    if _stdlib.as_mut().unwrap().do_condition(BoolEqExpression::new(register_id.clone(), BitVectorConcrete::new(0b0, 5, _stdlib.as_mut().unwrap()).into(), _stdlib.as_mut().unwrap()).into(), _fork_sink.clone()) {
        (*state).x0 = value.clone();
    }
    else if _stdlib.as_mut().unwrap().do_condition(BoolEqExpression::new(register_id.clone(), BitVectorConcrete::new(0b1, 5, _stdlib.as_mut().unwrap()).into(), _stdlib.as_mut().unwrap()).into(), _fork_sink.clone()) {
        (*state).x1 = value.clone();
    }
    else if _stdlib.as_mut().unwrap().do_condition(BoolEqExpression::new(register_id.clone(), BitVectorConcrete::new(0b10, 5, _stdlib.as_mut().unwrap()).into(), _stdlib.as_mut().unwrap()).into(), _fork_sink.clone()) {
        (*state).x2 = value.clone();
    }
    else if _stdlib.as_mut().unwrap().do_condition(BoolEqExpression::new(register_id.clone(), BitVectorConcrete::new(0b11, 5, _stdlib.as_mut().unwrap()).into(), _stdlib.as_mut().unwrap()).into(), _fork_sink.clone()) {
        (*state).x3 = value.clone();
    }
    else if _stdlib.as_mut().unwrap().do_condition(BoolEqExpression::new(register_id.clone(), BitVectorConcrete::new(0b100, 5, _stdlib.as_mut().unwrap()).into(), _stdlib.as_mut().unwrap()).into(), _fork_sink.clone()) {
        (*state).x4 = value.clone();
    }
    else if _stdlib.as_mut().unwrap().do_condition(BoolEqExpression::new(register_id.clone(), BitVectorConcrete::new(0b101, 5, _stdlib.as_mut().unwrap()).into(), _stdlib.as_mut().unwrap()).into(), _fork_sink.clone()) {
        (*state).x5 = value.clone();
    }
    else if _stdlib.as_mut().unwrap().do_condition(BoolEqExpression::new(register_id.clone(), BitVectorConcrete::new(0b110, 5, _stdlib.as_mut().unwrap()).into(), _stdlib.as_mut().unwrap()).into(), _fork_sink.clone()) {
        (*state).x6 = value.clone();
    }
    else if _stdlib.as_mut().unwrap().do_condition(BoolEqExpression::new(register_id.clone(), BitVectorConcrete::new(0b111, 5, _stdlib.as_mut().unwrap()).into(), _stdlib.as_mut().unwrap()).into(), _fork_sink.clone()) {
        (*state).x7 = value.clone();
    }
    else if _stdlib.as_mut().unwrap().do_condition(BoolEqExpression::new(register_id.clone(), BitVectorConcrete::new(0b1000, 5, _stdlib.as_mut().unwrap()).into(), _stdlib.as_mut().unwrap()).into(), _fork_sink.clone()) {
        (*state).x8 = value.clone();
    }
    else if _stdlib.as_mut().unwrap().do_condition(BoolEqExpression::new(register_id.clone(), BitVectorConcrete::new(0b1001, 5, _stdlib.as_mut().unwrap()).into(), _stdlib.as_mut().unwrap()).into(), _fork_sink.clone()) {
        (*state).x9 = value.clone();
    }
    else if _stdlib.as_mut().unwrap().do_condition(BoolEqExpression::new(register_id.clone(), BitVectorConcrete::new(0b1010, 5, _stdlib.as_mut().unwrap()).into(), _stdlib.as_mut().unwrap()).into(), _fork_sink.clone()) {
        (*state).x10 = value.clone();
    }
    else if _stdlib.as_mut().unwrap().do_condition(BoolEqExpression::new(register_id.clone(), BitVectorConcrete::new(0b1011, 5, _stdlib.as_mut().unwrap()).into(), _stdlib.as_mut().unwrap()).into(), _fork_sink.clone()) {
        (*state).x11 = value.clone();
    }
    else if _stdlib.as_mut().unwrap().do_condition(BoolEqExpression::new(register_id.clone(), BitVectorConcrete::new(0b1100, 5, _stdlib.as_mut().unwrap()).into(), _stdlib.as_mut().unwrap()).into(), _fork_sink.clone()) {
        (*state).x12 = value.clone();
    }
    else if _stdlib.as_mut().unwrap().do_condition(BoolEqExpression::new(register_id.clone(), BitVectorConcrete::new(0b1101, 5, _stdlib.as_mut().unwrap()).into(), _stdlib.as_mut().unwrap()).into(), _fork_sink.clone()) {
        (*state).x13 = value.clone();
    }
    else if _stdlib.as_mut().unwrap().do_condition(BoolEqExpression::new(register_id.clone(), BitVectorConcrete::new(0b1110, 5, _stdlib.as_mut().unwrap()).into(), _stdlib.as_mut().unwrap()).into(), _fork_sink.clone()) {
        (*state).x14 = value.clone();
    }
    else if _stdlib.as_mut().unwrap().do_condition(BoolEqExpression::new(register_id.clone(), BitVectorConcrete::new(0b1111, 5, _stdlib.as_mut().unwrap()).into(), _stdlib.as_mut().unwrap()).into(), _fork_sink.clone()) {
        (*state).x15 = value.clone();
    }
    else if _stdlib.as_mut().unwrap().do_condition(BoolEqExpression::new(register_id.clone(), BitVectorConcrete::new(0b10000, 5, _stdlib.as_mut().unwrap()).into(), _stdlib.as_mut().unwrap()).into(), _fork_sink.clone()) {
        (*state).x16 = value.clone();
    }
    else if _stdlib.as_mut().unwrap().do_condition(BoolEqExpression::new(register_id.clone(), BitVectorConcrete::new(0b10001, 5, _stdlib.as_mut().unwrap()).into(), _stdlib.as_mut().unwrap()).into(), _fork_sink.clone()) {
        (*state).x17 = value.clone();
    }
    else if _stdlib.as_mut().unwrap().do_condition(BoolEqExpression::new(register_id.clone(), BitVectorConcrete::new(0b10010, 5, _stdlib.as_mut().unwrap()).into(), _stdlib.as_mut().unwrap()).into(), _fork_sink.clone()) {
        (*state).x18 = value.clone();
    }
    else if _stdlib.as_mut().unwrap().do_condition(BoolEqExpression::new(register_id.clone(), BitVectorConcrete::new(0b10011, 5, _stdlib.as_mut().unwrap()).into(), _stdlib.as_mut().unwrap()).into(), _fork_sink.clone()) {
        (*state).x19 = value.clone();
    }
    else if _stdlib.as_mut().unwrap().do_condition(BoolEqExpression::new(register_id.clone(), BitVectorConcrete::new(0b10100, 5, _stdlib.as_mut().unwrap()).into(), _stdlib.as_mut().unwrap()).into(), _fork_sink.clone()) {
        (*state).x20 = value.clone();
    }
    else if _stdlib.as_mut().unwrap().do_condition(BoolEqExpression::new(register_id.clone(), BitVectorConcrete::new(0b10101, 5, _stdlib.as_mut().unwrap()).into(), _stdlib.as_mut().unwrap()).into(), _fork_sink.clone()) {
        (*state).x21 = value.clone();
    }
    else if _stdlib.as_mut().unwrap().do_condition(BoolEqExpression::new(register_id.clone(), BitVectorConcrete::new(0b10110, 5, _stdlib.as_mut().unwrap()).into(), _stdlib.as_mut().unwrap()).into(), _fork_sink.clone()) {
        (*state).x22 = value.clone();
    }
    else if _stdlib.as_mut().unwrap().do_condition(BoolEqExpression::new(register_id.clone(), BitVectorConcrete::new(0b10111, 5, _stdlib.as_mut().unwrap()).into(), _stdlib.as_mut().unwrap()).into(), _fork_sink.clone()) {
        (*state).x23 = value.clone();
    }
    else if _stdlib.as_mut().unwrap().do_condition(BoolEqExpression::new(register_id.clone(), BitVectorConcrete::new(0b11000, 5, _stdlib.as_mut().unwrap()).into(), _stdlib.as_mut().unwrap()).into(), _fork_sink.clone()) {
        (*state).x24 = value.clone();
    }
    else if _stdlib.as_mut().unwrap().do_condition(BoolEqExpression::new(register_id.clone(), BitVectorConcrete::new(0b11001, 5, _stdlib.as_mut().unwrap()).into(), _stdlib.as_mut().unwrap()).into(), _fork_sink.clone()) {
        (*state).x25 = value.clone();
    }
    else if _stdlib.as_mut().unwrap().do_condition(BoolEqExpression::new(register_id.clone(), BitVectorConcrete::new(0b11010, 5, _stdlib.as_mut().unwrap()).into(), _stdlib.as_mut().unwrap()).into(), _fork_sink.clone()) {
        (*state).x26 = value.clone();
    }
    else if _stdlib.as_mut().unwrap().do_condition(BoolEqExpression::new(register_id.clone(), BitVectorConcrete::new(0b11011, 5, _stdlib.as_mut().unwrap()).into(), _stdlib.as_mut().unwrap()).into(), _fork_sink.clone()) {
        (*state).x27 = value.clone();
    }
    else if _stdlib.as_mut().unwrap().do_condition(BoolEqExpression::new(register_id.clone(), BitVectorConcrete::new(0b11100, 5, _stdlib.as_mut().unwrap()).into(), _stdlib.as_mut().unwrap()).into(), _fork_sink.clone()) {
        (*state).x28 = value.clone();
    }
    else if _stdlib.as_mut().unwrap().do_condition(BoolEqExpression::new(register_id.clone(), BitVectorConcrete::new(0b11101, 5, _stdlib.as_mut().unwrap()).into(), _stdlib.as_mut().unwrap()).into(), _fork_sink.clone()) {
        (*state).x29 = value.clone();
    }
    else if _stdlib.as_mut().unwrap().do_condition(BoolEqExpression::new(register_id.clone(), BitVectorConcrete::new(0b11110, 5, _stdlib.as_mut().unwrap()).into(), _stdlib.as_mut().unwrap()).into(), _fork_sink.clone()) {
        (*state).x30 = value.clone();
    }
    else if _stdlib.as_mut().unwrap().do_condition(BoolEqExpression::new(register_id.clone(), BitVectorConcrete::new(0b11111, 5, _stdlib.as_mut().unwrap()).into(), _stdlib.as_mut().unwrap()).into(), _fork_sink.clone()) {
        (*state).x31 = value.clone();
    }
    else {
        unimplemented!();
    }
}

pub unsafe fn register_read_BV32(state: *mut SystemState, register_id: Rc<RefCell<ActiveValue>>, _stdlib: *mut ScfiaStdlib, _fork_sink: Option<Rc<RefCell<ForkSink>>>, _memory: &mut Memory32) -> Rc<RefCell<ActiveValue>> {
    if _stdlib.as_mut().unwrap().do_condition(BoolEqExpression::new(register_id.clone(), BitVectorConcrete::new(0b0, 5, _stdlib.as_mut().unwrap()).into(), _stdlib.as_mut().unwrap()).into(), _fork_sink.clone()) {
        return (*state).x0.clone();
    }
    else if _stdlib.as_mut().unwrap().do_condition(BoolEqExpression::new(register_id.clone(), BitVectorConcrete::new(0b1, 5, _stdlib.as_mut().unwrap()).into(), _stdlib.as_mut().unwrap()).into(), _fork_sink.clone()) {
        return (*state).x1.clone();
    }
    else if _stdlib.as_mut().unwrap().do_condition(BoolEqExpression::new(register_id.clone(), BitVectorConcrete::new(0b10, 5, _stdlib.as_mut().unwrap()).into(), _stdlib.as_mut().unwrap()).into(), _fork_sink.clone()) {
        return (*state).x2.clone();
    }
    else if _stdlib.as_mut().unwrap().do_condition(BoolEqExpression::new(register_id.clone(), BitVectorConcrete::new(0b11, 5, _stdlib.as_mut().unwrap()).into(), _stdlib.as_mut().unwrap()).into(), _fork_sink.clone()) {
        return (*state).x3.clone();
    }
    else if _stdlib.as_mut().unwrap().do_condition(BoolEqExpression::new(register_id.clone(), BitVectorConcrete::new(0b100, 5, _stdlib.as_mut().unwrap()).into(), _stdlib.as_mut().unwrap()).into(), _fork_sink.clone()) {
        return (*state).x4.clone();
    }
    else if _stdlib.as_mut().unwrap().do_condition(BoolEqExpression::new(register_id.clone(), BitVectorConcrete::new(0b101, 5, _stdlib.as_mut().unwrap()).into(), _stdlib.as_mut().unwrap()).into(), _fork_sink.clone()) {
        return (*state).x5.clone();
    }
    else if _stdlib.as_mut().unwrap().do_condition(BoolEqExpression::new(register_id.clone(), BitVectorConcrete::new(0b110, 5, _stdlib.as_mut().unwrap()).into(), _stdlib.as_mut().unwrap()).into(), _fork_sink.clone()) {
        return (*state).x6.clone();
    }
    else if _stdlib.as_mut().unwrap().do_condition(BoolEqExpression::new(register_id.clone(), BitVectorConcrete::new(0b111, 5, _stdlib.as_mut().unwrap()).into(), _stdlib.as_mut().unwrap()).into(), _fork_sink.clone()) {
        return (*state).x7.clone();
    }
    else if _stdlib.as_mut().unwrap().do_condition(BoolEqExpression::new(register_id.clone(), BitVectorConcrete::new(0b1000, 5, _stdlib.as_mut().unwrap()).into(), _stdlib.as_mut().unwrap()).into(), _fork_sink.clone()) {
        return (*state).x8.clone();
    }
    else if _stdlib.as_mut().unwrap().do_condition(BoolEqExpression::new(register_id.clone(), BitVectorConcrete::new(0b1001, 5, _stdlib.as_mut().unwrap()).into(), _stdlib.as_mut().unwrap()).into(), _fork_sink.clone()) {
        return (*state).x9.clone();
    }
    else if _stdlib.as_mut().unwrap().do_condition(BoolEqExpression::new(register_id.clone(), BitVectorConcrete::new(0b1010, 5, _stdlib.as_mut().unwrap()).into(), _stdlib.as_mut().unwrap()).into(), _fork_sink.clone()) {
        return (*state).x10.clone();
    }
    else if _stdlib.as_mut().unwrap().do_condition(BoolEqExpression::new(register_id.clone(), BitVectorConcrete::new(0b1011, 5, _stdlib.as_mut().unwrap()).into(), _stdlib.as_mut().unwrap()).into(), _fork_sink.clone()) {
        return (*state).x11.clone();
    }
    else if _stdlib.as_mut().unwrap().do_condition(BoolEqExpression::new(register_id.clone(), BitVectorConcrete::new(0b1100, 5, _stdlib.as_mut().unwrap()).into(), _stdlib.as_mut().unwrap()).into(), _fork_sink.clone()) {
        return (*state).x12.clone();
    }
    else if _stdlib.as_mut().unwrap().do_condition(BoolEqExpression::new(register_id.clone(), BitVectorConcrete::new(0b1101, 5, _stdlib.as_mut().unwrap()).into(), _stdlib.as_mut().unwrap()).into(), _fork_sink.clone()) {
        return (*state).x13.clone();
    }
    else if _stdlib.as_mut().unwrap().do_condition(BoolEqExpression::new(register_id.clone(), BitVectorConcrete::new(0b1110, 5, _stdlib.as_mut().unwrap()).into(), _stdlib.as_mut().unwrap()).into(), _fork_sink.clone()) {
        return (*state).x14.clone();
    }
    else if _stdlib.as_mut().unwrap().do_condition(BoolEqExpression::new(register_id.clone(), BitVectorConcrete::new(0b1111, 5, _stdlib.as_mut().unwrap()).into(), _stdlib.as_mut().unwrap()).into(), _fork_sink.clone()) {
        return (*state).x15.clone();
    }
    else if _stdlib.as_mut().unwrap().do_condition(BoolEqExpression::new(register_id.clone(), BitVectorConcrete::new(0b10000, 5, _stdlib.as_mut().unwrap()).into(), _stdlib.as_mut().unwrap()).into(), _fork_sink.clone()) {
        return (*state).x16.clone();
    }
    else if _stdlib.as_mut().unwrap().do_condition(BoolEqExpression::new(register_id.clone(), BitVectorConcrete::new(0b10001, 5, _stdlib.as_mut().unwrap()).into(), _stdlib.as_mut().unwrap()).into(), _fork_sink.clone()) {
        return (*state).x17.clone();
    }
    else if _stdlib.as_mut().unwrap().do_condition(BoolEqExpression::new(register_id.clone(), BitVectorConcrete::new(0b10010, 5, _stdlib.as_mut().unwrap()).into(), _stdlib.as_mut().unwrap()).into(), _fork_sink.clone()) {
        return (*state).x18.clone();
    }
    else if _stdlib.as_mut().unwrap().do_condition(BoolEqExpression::new(register_id.clone(), BitVectorConcrete::new(0b10011, 5, _stdlib.as_mut().unwrap()).into(), _stdlib.as_mut().unwrap()).into(), _fork_sink.clone()) {
        return (*state).x19.clone();
    }
    else if _stdlib.as_mut().unwrap().do_condition(BoolEqExpression::new(register_id.clone(), BitVectorConcrete::new(0b10100, 5, _stdlib.as_mut().unwrap()).into(), _stdlib.as_mut().unwrap()).into(), _fork_sink.clone()) {
        return (*state).x20.clone();
    }
    else if _stdlib.as_mut().unwrap().do_condition(BoolEqExpression::new(register_id.clone(), BitVectorConcrete::new(0b10101, 5, _stdlib.as_mut().unwrap()).into(), _stdlib.as_mut().unwrap()).into(), _fork_sink.clone()) {
        return (*state).x21.clone();
    }
    else if _stdlib.as_mut().unwrap().do_condition(BoolEqExpression::new(register_id.clone(), BitVectorConcrete::new(0b10110, 5, _stdlib.as_mut().unwrap()).into(), _stdlib.as_mut().unwrap()).into(), _fork_sink.clone()) {
        return (*state).x22.clone();
    }
    else if _stdlib.as_mut().unwrap().do_condition(BoolEqExpression::new(register_id.clone(), BitVectorConcrete::new(0b10111, 5, _stdlib.as_mut().unwrap()).into(), _stdlib.as_mut().unwrap()).into(), _fork_sink.clone()) {
        return (*state).x23.clone();
    }
    else if _stdlib.as_mut().unwrap().do_condition(BoolEqExpression::new(register_id.clone(), BitVectorConcrete::new(0b11000, 5, _stdlib.as_mut().unwrap()).into(), _stdlib.as_mut().unwrap()).into(), _fork_sink.clone()) {
        return (*state).x24.clone();
    }
    else if _stdlib.as_mut().unwrap().do_condition(BoolEqExpression::new(register_id.clone(), BitVectorConcrete::new(0b11001, 5, _stdlib.as_mut().unwrap()).into(), _stdlib.as_mut().unwrap()).into(), _fork_sink.clone()) {
        return (*state).x25.clone();
    }
    else if _stdlib.as_mut().unwrap().do_condition(BoolEqExpression::new(register_id.clone(), BitVectorConcrete::new(0b11010, 5, _stdlib.as_mut().unwrap()).into(), _stdlib.as_mut().unwrap()).into(), _fork_sink.clone()) {
        return (*state).x26.clone();
    }
    else if _stdlib.as_mut().unwrap().do_condition(BoolEqExpression::new(register_id.clone(), BitVectorConcrete::new(0b11011, 5, _stdlib.as_mut().unwrap()).into(), _stdlib.as_mut().unwrap()).into(), _fork_sink.clone()) {
        return (*state).x27.clone();
    }
    else if _stdlib.as_mut().unwrap().do_condition(BoolEqExpression::new(register_id.clone(), BitVectorConcrete::new(0b11100, 5, _stdlib.as_mut().unwrap()).into(), _stdlib.as_mut().unwrap()).into(), _fork_sink.clone()) {
        return (*state).x28.clone();
    }
    else if _stdlib.as_mut().unwrap().do_condition(BoolEqExpression::new(register_id.clone(), BitVectorConcrete::new(0b11101, 5, _stdlib.as_mut().unwrap()).into(), _stdlib.as_mut().unwrap()).into(), _fork_sink.clone()) {
        return (*state).x29.clone();
    }
    else if _stdlib.as_mut().unwrap().do_condition(BoolEqExpression::new(register_id.clone(), BitVectorConcrete::new(0b11110, 5, _stdlib.as_mut().unwrap()).into(), _stdlib.as_mut().unwrap()).into(), _fork_sink.clone()) {
        return (*state).x30.clone();
    }
    else if _stdlib.as_mut().unwrap().do_condition(BoolEqExpression::new(register_id.clone(), BitVectorConcrete::new(0b11111, 5, _stdlib.as_mut().unwrap()).into(), _stdlib.as_mut().unwrap()).into(), _fork_sink.clone()) {
        return (*state).x31.clone();
    }
    else {
        unimplemented!();
    }
}

pub unsafe fn execute_add32(state: *mut SystemState, destination_id: Rc<RefCell<ActiveValue>>, source1_id: Rc<RefCell<ActiveValue>>, source2_id: Rc<RefCell<ActiveValue>>, _stdlib: *mut ScfiaStdlib, _fork_sink: Option<Rc<RefCell<ForkSink>>>, _memory: &mut Memory32) {
    let s1: Rc<RefCell<ActiveValue>> = register_read_BV32(state, source1_id.clone(), _stdlib, _fork_sink.clone(), _memory);
    let s2: Rc<RefCell<ActiveValue>> = register_read_BV32(state, source2_id.clone(), _stdlib, _fork_sink.clone(), _memory);
    let sum: Rc<RefCell<ActiveValue>> = BVAddExpression::new(s1.clone(), s2.clone(), _stdlib.as_mut().unwrap()).into();
    register_write_BV32(state, destination_id.clone(), sum.clone(), _stdlib, _fork_sink.clone(), _memory);
}

impl RV32iSystemState {
    pub fn step(&mut self) {
        unsafe {
            println!("stepping {:?}", self.system_state.pc);
            step(&mut self.system_state, &mut self.stdlib, None, &mut self.memory);
        }
    }

    pub fn step_forking(self) -> Vec<RV32iSystemState> {
        unsafe {
            println!("RV32iSystemState stepping {:?} (forking)", self.system_state.pc);
            let mut successors = vec![];
            let mut candidates = vec![self.clone_to_stdlib(RV32iSystemStateFork::new(self.stdlib.next_symbol_id))];
            println!("RV32iSystemState stepping candidate");
            
            while let Some(mut current) = candidates.pop() {
                // Step current candidate
                let fork_sink = Rc::new(RefCell::new(ForkSink::new()));
                println!("RV32iSystemState stepping candidate");
                step(&mut current.system_state, &mut current.stdlib, Some(fork_sink.clone()), &mut current.memory);

                // Convert forks to candidates
                let mut f = fork_sink.try_borrow_mut().unwrap();
                while let Some(fork) = f.forks.pop() {
                    candidates.push(self.clone_to_stdlib(fork));
                }

                successors.push(current)
            }

            successors
        }
    }

    fn clone_to_stdlib(&self, mut fork: RV32iSystemStateFork) -> RV32iSystemState {
        let cloned_memory = self.memory.clone_to_stdlib(&mut fork.cloned_active_values, &mut fork.cloned_retired_values, &mut fork.cloned_stdlib);

        RV32iSystemState {
            system_state: SystemState {
                x0: self.system_state.x0.try_borrow().unwrap().clone_to_stdlib(&mut fork.cloned_active_values, &mut fork.cloned_retired_values, &mut fork.cloned_stdlib),
                x1: self.system_state.x1.try_borrow().unwrap().clone_to_stdlib(&mut fork.cloned_active_values, &mut fork.cloned_retired_values, &mut fork.cloned_stdlib),
                x2: self.system_state.x2.try_borrow().unwrap().clone_to_stdlib(&mut fork.cloned_active_values, &mut fork.cloned_retired_values, &mut fork.cloned_stdlib),
                x3: self.system_state.x3.try_borrow().unwrap().clone_to_stdlib(&mut fork.cloned_active_values, &mut fork.cloned_retired_values, &mut fork.cloned_stdlib),
                x4: self.system_state.x4.try_borrow().unwrap().clone_to_stdlib(&mut fork.cloned_active_values, &mut fork.cloned_retired_values, &mut fork.cloned_stdlib),
                x5: self.system_state.x5.try_borrow().unwrap().clone_to_stdlib(&mut fork.cloned_active_values, &mut fork.cloned_retired_values, &mut fork.cloned_stdlib),
                x6: self.system_state.x6.try_borrow().unwrap().clone_to_stdlib(&mut fork.cloned_active_values, &mut fork.cloned_retired_values, &mut fork.cloned_stdlib),
                x7: self.system_state.x7.try_borrow().unwrap().clone_to_stdlib(&mut fork.cloned_active_values, &mut fork.cloned_retired_values, &mut fork.cloned_stdlib),
                x8: self.system_state.x8.try_borrow().unwrap().clone_to_stdlib(&mut fork.cloned_active_values, &mut fork.cloned_retired_values, &mut fork.cloned_stdlib),
                x9: self.system_state.x9.try_borrow().unwrap().clone_to_stdlib(&mut fork.cloned_active_values, &mut fork.cloned_retired_values, &mut fork.cloned_stdlib),
                x10: self.system_state.x10.try_borrow().unwrap().clone_to_stdlib(&mut fork.cloned_active_values, &mut fork.cloned_retired_values, &mut fork.cloned_stdlib),
                x11: self.system_state.x11.try_borrow().unwrap().clone_to_stdlib(&mut fork.cloned_active_values, &mut fork.cloned_retired_values, &mut fork.cloned_stdlib),
                x12: self.system_state.x12.try_borrow().unwrap().clone_to_stdlib(&mut fork.cloned_active_values, &mut fork.cloned_retired_values, &mut fork.cloned_stdlib),
                x13: self.system_state.x13.try_borrow().unwrap().clone_to_stdlib(&mut fork.cloned_active_values, &mut fork.cloned_retired_values, &mut fork.cloned_stdlib),
                x14: self.system_state.x14.try_borrow().unwrap().clone_to_stdlib(&mut fork.cloned_active_values, &mut fork.cloned_retired_values, &mut fork.cloned_stdlib),
                x15: self.system_state.x15.try_borrow().unwrap().clone_to_stdlib(&mut fork.cloned_active_values, &mut fork.cloned_retired_values, &mut fork.cloned_stdlib),
                x16: self.system_state.x16.try_borrow().unwrap().clone_to_stdlib(&mut fork.cloned_active_values, &mut fork.cloned_retired_values, &mut fork.cloned_stdlib),
                x17: self.system_state.x17.try_borrow().unwrap().clone_to_stdlib(&mut fork.cloned_active_values, &mut fork.cloned_retired_values, &mut fork.cloned_stdlib),
                x18: self.system_state.x18.try_borrow().unwrap().clone_to_stdlib(&mut fork.cloned_active_values, &mut fork.cloned_retired_values, &mut fork.cloned_stdlib),
                x19: self.system_state.x19.try_borrow().unwrap().clone_to_stdlib(&mut fork.cloned_active_values, &mut fork.cloned_retired_values, &mut fork.cloned_stdlib),
                x20: self.system_state.x20.try_borrow().unwrap().clone_to_stdlib(&mut fork.cloned_active_values, &mut fork.cloned_retired_values, &mut fork.cloned_stdlib),
                x21: self.system_state.x21.try_borrow().unwrap().clone_to_stdlib(&mut fork.cloned_active_values, &mut fork.cloned_retired_values, &mut fork.cloned_stdlib),
                x22: self.system_state.x22.try_borrow().unwrap().clone_to_stdlib(&mut fork.cloned_active_values, &mut fork.cloned_retired_values, &mut fork.cloned_stdlib),
                x23: self.system_state.x23.try_borrow().unwrap().clone_to_stdlib(&mut fork.cloned_active_values, &mut fork.cloned_retired_values, &mut fork.cloned_stdlib),
                x24: self.system_state.x24.try_borrow().unwrap().clone_to_stdlib(&mut fork.cloned_active_values, &mut fork.cloned_retired_values, &mut fork.cloned_stdlib),
                x25: self.system_state.x25.try_borrow().unwrap().clone_to_stdlib(&mut fork.cloned_active_values, &mut fork.cloned_retired_values, &mut fork.cloned_stdlib),
                x26: self.system_state.x26.try_borrow().unwrap().clone_to_stdlib(&mut fork.cloned_active_values, &mut fork.cloned_retired_values, &mut fork.cloned_stdlib),
                x27: self.system_state.x27.try_borrow().unwrap().clone_to_stdlib(&mut fork.cloned_active_values, &mut fork.cloned_retired_values, &mut fork.cloned_stdlib),
                x28: self.system_state.x28.try_borrow().unwrap().clone_to_stdlib(&mut fork.cloned_active_values, &mut fork.cloned_retired_values, &mut fork.cloned_stdlib),
                x29: self.system_state.x29.try_borrow().unwrap().clone_to_stdlib(&mut fork.cloned_active_values, &mut fork.cloned_retired_values, &mut fork.cloned_stdlib),
                x30: self.system_state.x30.try_borrow().unwrap().clone_to_stdlib(&mut fork.cloned_active_values, &mut fork.cloned_retired_values, &mut fork.cloned_stdlib),
                x31: self.system_state.x31.try_borrow().unwrap().clone_to_stdlib(&mut fork.cloned_active_values, &mut fork.cloned_retired_values, &mut fork.cloned_stdlib),
                pc: self.system_state.pc.try_borrow().unwrap().clone_to_stdlib(&mut fork.cloned_active_values, &mut fork.cloned_retired_values, &mut fork.cloned_stdlib),       },
            memory: cloned_memory,
            stdlib: fork.cloned_stdlib,
        }
    }
}
