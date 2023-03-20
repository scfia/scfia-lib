#![allow(clippy::all)]
#![allow(non_snake_case)]
#![allow(unused)]
use log::warn;

use crate::{memory::Memory, scfia::Scfia, values::active_value::ActiveValue, SymbolicHints};

pub struct RV32i {
    pub state: SystemState,
    pub memory: Memory,
    pub scfia: Scfia,
}

impl RV32i {
    pub fn step(&mut self, mut hints: Option<SymbolicHints>) {
        unsafe {
            _step(&mut self.state, &mut self.memory, self.scfia.clone(), &mut hints);
        }
    }
}

#[derive(Debug)]
pub struct SystemState {
    pub x0: ActiveValue,
    pub x1: ActiveValue,
    pub x2: ActiveValue,
    pub x3: ActiveValue,
    pub x4: ActiveValue,
    pub x5: ActiveValue,
    pub x6: ActiveValue,
    pub x7: ActiveValue,
    pub x8: ActiveValue,
    pub x9: ActiveValue,
    pub x10: ActiveValue,
    pub x11: ActiveValue,
    pub x12: ActiveValue,
    pub x13: ActiveValue,
    pub x14: ActiveValue,
    pub x15: ActiveValue,
    pub x16: ActiveValue,
    pub x17: ActiveValue,
    pub x18: ActiveValue,
    pub x19: ActiveValue,
    pub x20: ActiveValue,
    pub x21: ActiveValue,
    pub x22: ActiveValue,
    pub x23: ActiveValue,
    pub x24: ActiveValue,
    pub x25: ActiveValue,
    pub x26: ActiveValue,
    pub x27: ActiveValue,
    pub x28: ActiveValue,
    pub x29: ActiveValue,
    pub x30: ActiveValue,
    pub x31: ActiveValue,
    pub pc: ActiveValue,
}

#[derive(Debug)]
pub struct ComplexSpecialStruct {
    pub some_flag: ActiveValue,
}

pub unsafe fn _reset(state: *mut SystemState, memory: &mut Memory, scfia: Scfia, hints: &mut Option<SymbolicHints>) {
    (*state).x0 = scfia.new_bv_concrete(0b0, 32);
    (*state).x1 = scfia.new_bv_concrete(0b0, 32);
    (*state).x2 = scfia.new_bv_concrete(0b0, 32);
    (*state).x3 = scfia.new_bv_concrete(0b0, 32);
    (*state).x4 = scfia.new_bv_concrete(0b0, 32);
    (*state).x5 = scfia.new_bv_concrete(0b0, 32);
    (*state).x6 = scfia.new_bv_concrete(0b0, 32);
    (*state).x7 = scfia.new_bv_concrete(0b0, 32);
    (*state).x8 = scfia.new_bv_concrete(0b0, 32);
    (*state).x9 = scfia.new_bv_concrete(0b0, 32);
    (*state).x10 = scfia.new_bv_concrete(0b0, 32);
    (*state).x11 = scfia.new_bv_concrete(0b0, 32);
    (*state).x12 = scfia.new_bv_concrete(0b0, 32);
    (*state).x13 = scfia.new_bv_concrete(0b0, 32);
    (*state).x14 = scfia.new_bv_concrete(0b0, 32);
    (*state).x15 = scfia.new_bv_concrete(0b0, 32);
    (*state).x16 = scfia.new_bv_concrete(0b0, 32);
    (*state).x17 = scfia.new_bv_concrete(0b0, 32);
    (*state).x18 = scfia.new_bv_concrete(0b0, 32);
    (*state).x19 = scfia.new_bv_concrete(0b0, 32);
    (*state).x20 = scfia.new_bv_concrete(0b0, 32);
    (*state).x21 = scfia.new_bv_concrete(0b0, 32);
    (*state).x22 = scfia.new_bv_concrete(0b0, 32);
    (*state).x23 = scfia.new_bv_concrete(0b0, 32);
    (*state).x24 = scfia.new_bv_concrete(0b0, 32);
    (*state).x25 = scfia.new_bv_concrete(0b0, 32);
    (*state).x26 = scfia.new_bv_concrete(0b0, 32);
    (*state).x27 = scfia.new_bv_concrete(0b0, 32);
    (*state).x28 = scfia.new_bv_concrete(0b0, 32);
    (*state).x29 = scfia.new_bv_concrete(0b0, 32);
    (*state).x30 = scfia.new_bv_concrete(0b0, 32);
    (*state).x31 = scfia.new_bv_concrete(0b0, 32);
}

pub unsafe fn _sum(state: *mut SystemState, memory: &mut Memory, scfia: Scfia, hints: &mut Option<SymbolicHints>) -> ActiveValue {
    return scfia.new_bv_add((*state).x1.clone(), (*state).x2.clone(), 32);
}

pub unsafe fn _test(state: *mut SystemState, memory: &mut Memory, scfia: Scfia, hints: &mut Option<SymbolicHints>) {
    (*state).x3 = scfia.new_bv_add(_sum(state, memory, scfia.clone(), hints), _sum(state, memory, scfia.clone(), hints), 32);
}

pub unsafe fn _step(state: *mut SystemState, memory: &mut Memory, scfia: Scfia, hints: &mut Option<SymbolicHints>) {
    let instruction_32: ActiveValue = memory.read((*state).pc.clone(), 32, scfia.clone(), hints);
    let opcode: ActiveValue = scfia.new_bv_slice(instruction_32.clone(), 6, 0);
    if scfia.check_condition(scfia.new_bool_eq(opcode.clone(), scfia.new_bv_concrete(0b11, 7))) {
        let funct3: ActiveValue = scfia.new_bv_slice(instruction_32.clone(), 14, 12);
        if scfia.check_condition(scfia.new_bool_eq(funct3.clone(), scfia.new_bv_concrete(0b1, 3))) {
            let rd: ActiveValue = _extract_rd_32(instruction_32.clone(), memory, scfia.clone(), hints);
            let rs1: ActiveValue = _extract_rs1_32(instruction_32.clone(), memory, scfia.clone(), hints);
            let imm: ActiveValue = scfia.new_bv_slice(instruction_32.clone(), 31, 20);
            let imm32: ActiveValue = scfia.new_bv_sign_extend(imm.clone(), 12, 32);
            let base_address: ActiveValue = _register_read_BV32(state, rs1.clone(), memory, scfia.clone(), hints);
            let address: ActiveValue = scfia.new_bv_add(base_address.clone(), imm32.clone(), 32);
            let value16: ActiveValue = memory.read(address.clone(), 16, scfia.clone(), hints);
            let value: ActiveValue = scfia.new_bv_sign_extend(value16.clone(), 16, 32);
            _register_write_BV32(state, rd.clone(), value.clone(), memory, scfia.clone(), hints);
            _progress_pc_4(state, memory, scfia.clone(), hints);
        } else if scfia.check_condition(scfia.new_bool_eq(funct3.clone(), scfia.new_bv_concrete(0b10, 3))) {
            let rd: ActiveValue = _extract_rd_32(instruction_32.clone(), memory, scfia.clone(), hints);
            let rs1: ActiveValue = _extract_rs1_32(instruction_32.clone(), memory, scfia.clone(), hints);
            let imm: ActiveValue = scfia.new_bv_slice(instruction_32.clone(), 31, 20);
            let imm32: ActiveValue = scfia.new_bv_sign_extend(imm.clone(), 12, 32);
            let base_address: ActiveValue = _register_read_BV32(state, rs1.clone(), memory, scfia.clone(), hints);
            let address: ActiveValue = scfia.new_bv_add(base_address.clone(), imm32.clone(), 32);
            let value: ActiveValue = memory.read(address.clone(), 32, scfia.clone(), hints);
            _register_write_BV32(state, rd.clone(), value.clone(), memory, scfia.clone(), hints);
            _progress_pc_4(state, memory, scfia.clone(), hints);
        } else if scfia.check_condition(scfia.new_bool_eq(funct3.clone(), scfia.new_bv_concrete(0b100, 3))) {
            let rd: ActiveValue = _extract_rd_32(instruction_32.clone(), memory, scfia.clone(), hints);
            let rs1: ActiveValue = _extract_rs1_32(instruction_32.clone(), memory, scfia.clone(), hints);
            let imm: ActiveValue = scfia.new_bv_slice(instruction_32.clone(), 31, 20);
            let imm32: ActiveValue = scfia.new_bv_sign_extend(imm.clone(), 12, 32);
            let base_address: ActiveValue = _register_read_BV32(state, rs1.clone(), memory, scfia.clone(), hints);
            let address: ActiveValue = scfia.new_bv_add(base_address.clone(), imm32.clone(), 32);
            let value8: ActiveValue = memory.read(address.clone(), 8, scfia.clone(), hints);
            let value: ActiveValue = scfia.new_bv_concat(scfia.new_bv_concrete(0b0, 24), value8.clone(), 32);
            _register_write_BV32(state, rd.clone(), value.clone(), memory, scfia.clone(), hints);
            _progress_pc_4(state, memory, scfia.clone(), hints);
        } else if scfia.check_condition(scfia.new_bool_eq(funct3.clone(), scfia.new_bv_concrete(0b101, 3))) {
            let rd: ActiveValue = _extract_rd_32(instruction_32.clone(), memory, scfia.clone(), hints);
            let rs1: ActiveValue = _extract_rs1_32(instruction_32.clone(), memory, scfia.clone(), hints);
            let imm: ActiveValue = scfia.new_bv_slice(instruction_32.clone(), 31, 20);
            let imm32: ActiveValue = scfia.new_bv_sign_extend(imm.clone(), 12, 32);
            let base_address: ActiveValue = _register_read_BV32(state, rs1.clone(), memory, scfia.clone(), hints);
            let address: ActiveValue = scfia.new_bv_add(base_address.clone(), imm32.clone(), 32);
            let value16: ActiveValue = memory.read(address.clone(), 16, scfia.clone(), hints);
            let value: ActiveValue = scfia.new_bv_concat(scfia.new_bv_concrete(0b0, 16), value16.clone(), 32);
            _register_write_BV32(state, rd.clone(), value.clone(), memory, scfia.clone(), hints);
            _progress_pc_4(state, memory, scfia.clone(), hints);
        } else {
            unimplemented!();
        }
    } else if scfia.check_condition(scfia.new_bool_eq(opcode.clone(), scfia.new_bv_concrete(0b1111, 7))) {
        let rd_zeroes: ActiveValue = scfia.new_bv_slice(instruction_32.clone(), 11, 7);
        if scfia.check_condition(scfia.new_bool_eq(rd_zeroes.clone(), scfia.new_bv_concrete(0b0, 5))) {
            let funct3: ActiveValue = scfia.new_bv_slice(instruction_32.clone(), 14, 12);
            let rs1_zeroes: ActiveValue = scfia.new_bv_slice(instruction_32.clone(), 19, 15);
            if scfia.check_condition(scfia.new_bool_eq(rs1_zeroes.clone(), scfia.new_bv_concrete(0b0, 5))) {
                if scfia.check_condition(scfia.new_bool_eq(funct3.clone(), scfia.new_bv_concrete(0b0, 3))) {
                    _progress_pc_4(state, memory, scfia.clone(), hints);
                } else if scfia.check_condition(scfia.new_bool_eq(funct3.clone(), scfia.new_bv_concrete(0b1, 3))) {
                    _progress_pc_4(state, memory, scfia.clone(), hints);
                }
            } else {
                unimplemented!();
            }
        } else {
            unimplemented!();
        }
    } else if scfia.check_condition(scfia.new_bool_eq(opcode.clone(), scfia.new_bv_concrete(0b10011, 7))) {
        let funct3: ActiveValue = scfia.new_bv_slice(instruction_32.clone(), 14, 12);
        if scfia.check_condition(scfia.new_bool_eq(funct3.clone(), scfia.new_bv_concrete(0b0, 3))) {
            let rd: ActiveValue = _extract_rd_32(instruction_32.clone(), memory, scfia.clone(), hints);
            let rs1: ActiveValue = _extract_rs1_32(instruction_32.clone(), memory, scfia.clone(), hints);
            let offset: ActiveValue = scfia.new_bv_slice(instruction_32.clone(), 31, 20);
            let offset_32: ActiveValue = scfia.new_bv_sign_extend(offset.clone(), 12, 32);
            let result: ActiveValue = scfia.new_bv_add(offset_32.clone(), _register_read_BV32(state, rs1.clone(), memory, scfia.clone(), hints), 32);
            _register_write_BV32(state, rd.clone(), result.clone(), memory, scfia.clone(), hints);
            _progress_pc_4(state, memory, scfia.clone(), hints);
        } else if scfia.check_condition(scfia.new_bool_eq(funct3.clone(), scfia.new_bv_concrete(0b1, 3))) {
            let funct7: ActiveValue = scfia.new_bv_slice(instruction_32.clone(), 31, 25);
            if scfia.check_condition(scfia.new_bool_eq(funct7.clone(), scfia.new_bv_concrete(0b0, 7))) {
                let rd: ActiveValue = _extract_rd_32(instruction_32.clone(), memory, scfia.clone(), hints);
                let rs1: ActiveValue = _extract_rs1_32(instruction_32.clone(), memory, scfia.clone(), hints);
                let shamt: ActiveValue = scfia.new_bv_slice(instruction_32.clone(), 24, 20);
                let result: ActiveValue = scfia.new_bv_sll(_register_read_BV32(state, rs1.clone(), memory, scfia.clone(), hints), shamt.clone(), 32, 5);
                _register_write_BV32(state, rd.clone(), result.clone(), memory, scfia.clone(), hints);
                _progress_pc_4(state, memory, scfia.clone(), hints);
            } else {
                unimplemented!();
            }
        } else if scfia.check_condition(scfia.new_bool_eq(funct3.clone(), scfia.new_bv_concrete(0b100, 3))) {
            let rd: ActiveValue = _extract_rd_32(instruction_32.clone(), memory, scfia.clone(), hints);
            let rs1: ActiveValue = _extract_rs1_32(instruction_32.clone(), memory, scfia.clone(), hints);
            let imm: ActiveValue = scfia.new_bv_slice(instruction_32.clone(), 31, 20);
            let imm_32: ActiveValue = scfia.new_bv_sign_extend(imm.clone(), 12, 32);
            let result: ActiveValue = scfia.new_bv_xor(_register_read_BV32(state, rs1.clone(), memory, scfia.clone(), hints), imm_32.clone(), 32);
            _register_write_BV32(state, rd.clone(), result.clone(), memory, scfia.clone(), hints);
            _progress_pc_4(state, memory, scfia.clone(), hints);
        } else if scfia.check_condition(scfia.new_bool_eq(funct3.clone(), scfia.new_bv_concrete(0b101, 3))) {
            let funct7: ActiveValue = scfia.new_bv_slice(instruction_32.clone(), 31, 25);
            if scfia.check_condition(scfia.new_bool_eq(funct7.clone(), scfia.new_bv_concrete(0b0, 7))) {
                let rd: ActiveValue = _extract_rd_32(instruction_32.clone(), memory, scfia.clone(), hints);
                let rs1: ActiveValue = _extract_rs1_32(instruction_32.clone(), memory, scfia.clone(), hints);
                let shamt: ActiveValue = scfia.new_bv_slice(instruction_32.clone(), 24, 20);
                let result: ActiveValue = scfia.new_bv_srl(_register_read_BV32(state, rs1.clone(), memory, scfia.clone(), hints), shamt.clone(), 32, 5);
                _register_write_BV32(state, rd.clone(), result.clone(), memory, scfia.clone(), hints);
                _progress_pc_4(state, memory, scfia.clone(), hints);
            } else {
                unimplemented!();
            }
        } else if scfia.check_condition(scfia.new_bool_eq(funct3.clone(), scfia.new_bv_concrete(0b110, 3))) {
            let rd: ActiveValue = _extract_rd_32(instruction_32.clone(), memory, scfia.clone(), hints);
            let rs1: ActiveValue = _extract_rs1_32(instruction_32.clone(), memory, scfia.clone(), hints);
            let imm: ActiveValue = scfia.new_bv_slice(instruction_32.clone(), 31, 20);
            let imm_32: ActiveValue = scfia.new_bv_sign_extend(imm.clone(), 12, 32);
            let result: ActiveValue = scfia.new_bv_or(_register_read_BV32(state, rs1.clone(), memory, scfia.clone(), hints), imm_32.clone(), 32);
            _register_write_BV32(state, rd.clone(), result.clone(), memory, scfia.clone(), hints);
            _progress_pc_4(state, memory, scfia.clone(), hints);
        } else if scfia.check_condition(scfia.new_bool_eq(funct3.clone(), scfia.new_bv_concrete(0b111, 3))) {
            let rd: ActiveValue = _extract_rd_32(instruction_32.clone(), memory, scfia.clone(), hints);
            let rs1: ActiveValue = _extract_rs1_32(instruction_32.clone(), memory, scfia.clone(), hints);
            let imm: ActiveValue = scfia.new_bv_slice(instruction_32.clone(), 31, 20);
            let imm_32: ActiveValue = scfia.new_bv_sign_extend(imm.clone(), 12, 32);
            let result: ActiveValue = scfia.new_bv_add(_register_read_BV32(state, rs1.clone(), memory, scfia.clone(), hints), imm_32.clone(), 32);
            _register_write_BV32(state, rd.clone(), result.clone(), memory, scfia.clone(), hints);
            _progress_pc_4(state, memory, scfia.clone(), hints);
        } else {
            unimplemented!();
        }
    } else if scfia.check_condition(scfia.new_bool_eq(opcode.clone(), scfia.new_bv_concrete(0b100011, 7))) {
        let funct3: ActiveValue = scfia.new_bv_slice(instruction_32.clone(), 14, 12);
        if scfia.check_condition(scfia.new_bool_eq(funct3.clone(), scfia.new_bv_concrete(0b0, 3))) {
            let rs1: ActiveValue = _extract_rs1_32(instruction_32.clone(), memory, scfia.clone(), hints);
            let rs2: ActiveValue = _extract_rs2_32(instruction_32.clone(), memory, scfia.clone(), hints);
            let offset_11_5: ActiveValue = scfia.new_bv_slice(instruction_32.clone(), 31, 25);
            let offset_4_0: ActiveValue = scfia.new_bv_slice(instruction_32.clone(), 11, 7);
            let offset: ActiveValue = scfia.new_bv_concat(offset_11_5.clone(), offset_4_0.clone(), 12);
            let offset_32: ActiveValue = scfia.new_bv_sign_extend(offset.clone(), 12, 32);
            let base_address: ActiveValue = _register_read_BV32(state, rs1.clone(), memory, scfia.clone(), hints);
            let address: ActiveValue = scfia.new_bv_add(base_address.clone(), offset_32.clone(), 32);
            let value_32: ActiveValue = _register_read_BV32(state, rs2.clone(), memory, scfia.clone(), hints);
            let value: ActiveValue = scfia.new_bv_slice(value_32.clone(), 7, 0);
            memory.write(address.clone(), value.clone(), 8, scfia.clone(), hints);
            _progress_pc_4(state, memory, scfia.clone(), hints);
        } else if scfia.check_condition(scfia.new_bool_eq(funct3.clone(), scfia.new_bv_concrete(0b1, 3))) {
            let rs1: ActiveValue = _extract_rs1_32(instruction_32.clone(), memory, scfia.clone(), hints);
            let rs2: ActiveValue = _extract_rs2_32(instruction_32.clone(), memory, scfia.clone(), hints);
            let offset_11_5: ActiveValue = scfia.new_bv_slice(instruction_32.clone(), 31, 25);
            let offset_4_0: ActiveValue = scfia.new_bv_slice(instruction_32.clone(), 11, 7);
            let offset: ActiveValue = scfia.new_bv_concat(offset_11_5.clone(), offset_4_0.clone(), 12);
            let offset_32: ActiveValue = scfia.new_bv_sign_extend(offset.clone(), 12, 32);
            let base_address: ActiveValue = _register_read_BV32(state, rs1.clone(), memory, scfia.clone(), hints);
            let address: ActiveValue = scfia.new_bv_add(base_address.clone(), offset_32.clone(), 32);
            let value_32: ActiveValue = _register_read_BV32(state, rs2.clone(), memory, scfia.clone(), hints);
            let value: ActiveValue = scfia.new_bv_slice(value_32.clone(), 15, 0);
            memory.write(address.clone(), value.clone(), 16, scfia.clone(), hints);
            _progress_pc_4(state, memory, scfia.clone(), hints);
        } else if scfia.check_condition(scfia.new_bool_eq(funct3.clone(), scfia.new_bv_concrete(0b10, 3))) {
            let rs1: ActiveValue = _extract_rs1_32(instruction_32.clone(), memory, scfia.clone(), hints);
            let rs2: ActiveValue = _extract_rs2_32(instruction_32.clone(), memory, scfia.clone(), hints);
            let offset_11_5: ActiveValue = scfia.new_bv_slice(instruction_32.clone(), 31, 25);
            let offset_4_0: ActiveValue = scfia.new_bv_slice(instruction_32.clone(), 11, 7);
            let offset: ActiveValue = scfia.new_bv_concat(offset_11_5.clone(), offset_4_0.clone(), 12);
            let offset_32: ActiveValue = scfia.new_bv_sign_extend(offset.clone(), 12, 32);
            let base_address: ActiveValue = _register_read_BV32(state, rs1.clone(), memory, scfia.clone(), hints);
            let address: ActiveValue = scfia.new_bv_add(base_address.clone(), offset_32.clone(), 32);
            let value: ActiveValue = _register_read_BV32(state, rs2.clone(), memory, scfia.clone(), hints);
            memory.write(address.clone(), value.clone(), 32, scfia.clone(), hints);
            _progress_pc_4(state, memory, scfia.clone(), hints);
        } else {
            unimplemented!();
        }
    } else if scfia.check_condition(scfia.new_bool_eq(opcode.clone(), scfia.new_bv_concrete(0b110111, 7))) {
        let rd: ActiveValue = _extract_rd_32(instruction_32.clone(), memory, scfia.clone(), hints);
        let rs: ActiveValue = _extract_rs1_32(instruction_32.clone(), memory, scfia.clone(), hints);
        let imm: ActiveValue = scfia.new_bv_slice(instruction_32.clone(), 31, 12);
        let value: ActiveValue = scfia.new_bv_concat(imm.clone(), scfia.new_bv_concrete(0b0, 12), 32);
        _register_write_BV32(state, rd.clone(), value.clone(), memory, scfia.clone(), hints);
        _progress_pc_4(state, memory, scfia.clone(), hints);
    } else if scfia.check_condition(scfia.new_bool_eq(opcode.clone(), scfia.new_bv_concrete(0b10111, 7))) {
        let dst: ActiveValue = _extract_rd_32(instruction_32.clone(), memory, scfia.clone(), hints);
        let imm: ActiveValue = scfia.new_bv_slice(instruction_32.clone(), 31, 12);
        let imm32: ActiveValue = scfia.new_bv_concat(imm.clone(), scfia.new_bv_concrete(0b0, 12), 32);
        let sum: ActiveValue = scfia.new_bv_add(imm32.clone(), (*state).pc.clone(), 32);
        _register_write_BV32(state, dst.clone(), sum.clone(), memory, scfia.clone(), hints);
        _progress_pc_4(state, memory, scfia.clone(), hints);
    } else if scfia.check_condition(scfia.new_bool_eq(opcode.clone(), scfia.new_bv_concrete(0b110011, 7))) {
        let funct3: ActiveValue = scfia.new_bv_slice(instruction_32.clone(), 14, 12);
        let funct7: ActiveValue = scfia.new_bv_slice(instruction_32.clone(), 31, 25);
        let rd: ActiveValue = scfia.new_bv_slice(instruction_32.clone(), 11, 7);
        let rs1: ActiveValue = scfia.new_bv_slice(instruction_32.clone(), 19, 15);
        let rs2: ActiveValue = scfia.new_bv_slice(instruction_32.clone(), 24, 20);
        if scfia.check_condition(scfia.new_bool_eq(funct3.clone(), scfia.new_bv_concrete(0b0, 3))) {
            let s1: ActiveValue = _register_read_BV32(state, rs1.clone(), memory, scfia.clone(), hints);
            let s2: ActiveValue = _register_read_BV32(state, rs2.clone(), memory, scfia.clone(), hints);
            if scfia.check_condition(scfia.new_bool_eq(funct7.clone(), scfia.new_bv_concrete(0b0, 7))) {
                _execute_add32(state, rd.clone(), rs1.clone(), rs2.clone(), memory, scfia.clone(), hints);
                _progress_pc_4(state, memory, scfia.clone(), hints);
            } else if scfia.check_condition(scfia.new_bool_eq(funct7.clone(), scfia.new_bv_concrete(0b1, 7))) {
                let result: ActiveValue = scfia.new_bv_multiply(s1.clone(), s2.clone(), 32);
                _register_write_BV32(state, rd.clone(), result.clone(), memory, scfia.clone(), hints);
                _progress_pc_4(state, memory, scfia.clone(), hints);
            } else if scfia.check_condition(scfia.new_bool_eq(funct7.clone(), scfia.new_bv_concrete(0b100000, 7))) {
                let sum: ActiveValue = scfia.new_bv_sub(s1.clone(), s2.clone(), 32);
                _register_write_BV32(state, rd.clone(), sum.clone(), memory, scfia.clone(), hints);
                _progress_pc_4(state, memory, scfia.clone(), hints);
            } else {
                unimplemented!();
            }
        } else if scfia.check_condition(scfia.new_bool_eq(funct3.clone(), scfia.new_bv_concrete(0b1, 3))) {
            if scfia.check_condition(scfia.new_bool_eq(funct7.clone(), scfia.new_bv_concrete(0b0, 7))) {
                let shamt: ActiveValue = scfia.new_bv_slice(_register_read_BV32(state, rs2.clone(), memory, scfia.clone(), hints), 4, 0);
                let result: ActiveValue = scfia.new_bv_sll(_register_read_BV32(state, rs1.clone(), memory, scfia.clone(), hints), shamt.clone(), 32, 5);
                _register_write_BV32(state, rd.clone(), result.clone(), memory, scfia.clone(), hints);
                _progress_pc_4(state, memory, scfia.clone(), hints);
            } else {
                unimplemented!();
            }
        } else if scfia.check_condition(scfia.new_bool_eq(funct3.clone(), scfia.new_bv_concrete(0b10, 3))) {
            if scfia.check_condition(scfia.new_bool_eq(funct7.clone(), scfia.new_bv_concrete(0b0, 7))) {
                unimplemented!();
            } else {
                unimplemented!();
            }
        } else if scfia.check_condition(scfia.new_bool_eq(funct3.clone(), scfia.new_bv_concrete(0b11, 3))) {
            if scfia.check_condition(scfia.new_bool_eq(funct7.clone(), scfia.new_bv_concrete(0b0, 7))) {
                if scfia.check_condition(scfia.new_bool_unsigned_less_than(
                    _register_read_BV32(state, rs1.clone(), memory, scfia.clone(), hints),
                    _register_read_BV32(state, rs2.clone(), memory, scfia.clone(), hints),
                )) {
                    _register_write_BV32(state, rd.clone(), scfia.new_bv_concrete(0b1, 32), memory, scfia.clone(), hints);
                } else {
                    _register_write_BV32(state, rd.clone(), scfia.new_bv_concrete(0b0, 32), memory, scfia.clone(), hints);
                }
                _progress_pc_4(state, memory, scfia.clone(), hints);
            } else {
                unimplemented!();
            }
        } else if scfia.check_condition(scfia.new_bool_eq(funct3.clone(), scfia.new_bv_concrete(0b100, 3))) {
            if scfia.check_condition(scfia.new_bool_eq(funct7.clone(), scfia.new_bv_concrete(0b0, 7))) {
                unimplemented!();
            } else {
                unimplemented!();
            }
        } else if scfia.check_condition(scfia.new_bool_eq(funct3.clone(), scfia.new_bv_concrete(0b101, 3))) {
            if scfia.check_condition(scfia.new_bool_eq(funct7.clone(), scfia.new_bv_concrete(0b0, 7))) {
                let shamt: ActiveValue = scfia.new_bv_slice(_register_read_BV32(state, rs2.clone(), memory, scfia.clone(), hints), 4, 0);
                let result: ActiveValue = scfia.new_bv_srl(_register_read_BV32(state, rs1.clone(), memory, scfia.clone(), hints), shamt.clone(), 32, 5);
                _register_write_BV32(state, rd.clone(), result.clone(), memory, scfia.clone(), hints);
                _progress_pc_4(state, memory, scfia.clone(), hints);
            } else if scfia.check_condition(scfia.new_bool_eq(funct7.clone(), scfia.new_bv_concrete(0b100000, 7))) {
                unimplemented!();
            } else {
                unimplemented!();
            }
        } else if scfia.check_condition(scfia.new_bool_eq(funct3.clone(), scfia.new_bv_concrete(0b110, 3))) {
            if scfia.check_condition(scfia.new_bool_eq(funct7.clone(), scfia.new_bv_concrete(0b0, 7))) {
                let rd: ActiveValue = _extract_rd_32(instruction_32.clone(), memory, scfia.clone(), hints);
                let rs1: ActiveValue = _extract_rs1_32(instruction_32.clone(), memory, scfia.clone(), hints);
                let rs2: ActiveValue = _extract_rs2_32(instruction_32.clone(), memory, scfia.clone(), hints);
                let result: ActiveValue = scfia.new_bv_or(
                    _register_read_BV32(state, rs1.clone(), memory, scfia.clone(), hints),
                    _register_read_BV32(state, rs2.clone(), memory, scfia.clone(), hints),
                    32,
                );
                _register_write_BV32(state, rd.clone(), result.clone(), memory, scfia.clone(), hints);
                _progress_pc_4(state, memory, scfia.clone(), hints);
            } else {
                unimplemented!();
            }
        } else if scfia.check_condition(scfia.new_bool_eq(funct3.clone(), scfia.new_bv_concrete(0b111, 3))) {
            let rd: ActiveValue = _extract_rd_32(instruction_32.clone(), memory, scfia.clone(), hints);
            let rs1: ActiveValue = _extract_rs1_32(instruction_32.clone(), memory, scfia.clone(), hints);
            let rs2: ActiveValue = _extract_rs2_32(instruction_32.clone(), memory, scfia.clone(), hints);
            if scfia.check_condition(scfia.new_bool_eq(funct7.clone(), scfia.new_bv_concrete(0b0, 7))) {
                let result: ActiveValue = scfia.new_bv_add(
                    _register_read_BV32(state, rs1.clone(), memory, scfia.clone(), hints),
                    _register_read_BV32(state, rs2.clone(), memory, scfia.clone(), hints),
                    32,
                );
                _register_write_BV32(state, rd.clone(), result.clone(), memory, scfia.clone(), hints);
                _progress_pc_4(state, memory, scfia.clone(), hints);
            } else if scfia.check_condition(scfia.new_bool_eq(funct7.clone(), scfia.new_bv_concrete(0b1, 7))) {
                let result: ActiveValue = scfia.new_bv_unsigned_remainder(
                    _register_read_BV32(state, rs1.clone(), memory, scfia.clone(), hints),
                    _register_read_BV32(state, rs2.clone(), memory, scfia.clone(), hints),
                    32,
                );
                _register_write_BV32(state, rd.clone(), result.clone(), memory, scfia.clone(), hints);
                _progress_pc_4(state, memory, scfia.clone(), hints);
            } else {
                unimplemented!();
            }
        } else {
            unimplemented!();
        }
    } else if scfia.check_condition(scfia.new_bool_eq(opcode.clone(), scfia.new_bv_concrete(0b1100011, 7))) {
        let funct3: ActiveValue = scfia.new_bv_slice(instruction_32.clone(), 14, 12);
        if scfia.check_condition(scfia.new_bool_eq(funct3.clone(), scfia.new_bv_concrete(0b0, 3))) {
            let lhs: ActiveValue = _register_read_BV32(
                state,
                _extract_rs1_32(instruction_32.clone(), memory, scfia.clone(), hints),
                memory,
                scfia.clone(),
                hints,
            );
            let rhs: ActiveValue = _register_read_BV32(
                state,
                _extract_rs2_32(instruction_32.clone(), memory, scfia.clone(), hints),
                memory,
                scfia.clone(),
                hints,
            );
            if scfia.check_condition(scfia.new_bool_eq(lhs.clone(), rhs.clone())) {
                let imm_4_1: ActiveValue = scfia.new_bv_slice(instruction_32.clone(), 11, 8);
                let imm_10_5: ActiveValue = scfia.new_bv_slice(instruction_32.clone(), 30, 25);
                let imm11: ActiveValue = scfia.new_bv_slice(instruction_32.clone(), 7, 7);
                let imm12: ActiveValue = scfia.new_bv_slice(instruction_32.clone(), 31, 31);
                let imm_4_0: ActiveValue = scfia.new_bv_concat(imm_4_1.clone(), scfia.new_bv_concrete(0b0, 1), 5);
                let imm_10_0: ActiveValue = scfia.new_bv_concat(imm_10_5.clone(), imm_4_0.clone(), 11);
                let imm_11_0: ActiveValue = scfia.new_bv_concat(imm11.clone(), imm_10_0.clone(), 12);
                let imm_12_0: ActiveValue = scfia.new_bv_concat(imm12.clone(), imm_11_0.clone(), 13);
                let offset: ActiveValue = scfia.new_bv_sign_extend(imm_12_0.clone(), 13, 32);
                let address: ActiveValue = scfia.new_bv_add((*state).pc.clone(), offset.clone(), 32);
                (*state).pc = address.clone();
            } else {
                _progress_pc_4(state, memory, scfia.clone(), hints);
            }
        } else if scfia.check_condition(scfia.new_bool_eq(funct3.clone(), scfia.new_bv_concrete(0b1, 3))) {
            let lhs: ActiveValue = _register_read_BV32(
                state,
                _extract_rs1_32(instruction_32.clone(), memory, scfia.clone(), hints),
                memory,
                scfia.clone(),
                hints,
            );
            let rhs: ActiveValue = _register_read_BV32(
                state,
                _extract_rs2_32(instruction_32.clone(), memory, scfia.clone(), hints),
                memory,
                scfia.clone(),
                hints,
            );
            if scfia.check_condition(scfia.new_bool_not(scfia.new_bool_eq(lhs.clone(), rhs.clone()))) {
                let imm_4_1: ActiveValue = scfia.new_bv_slice(instruction_32.clone(), 11, 8);
                let imm_10_5: ActiveValue = scfia.new_bv_slice(instruction_32.clone(), 30, 25);
                let imm11: ActiveValue = scfia.new_bv_slice(instruction_32.clone(), 7, 7);
                let imm12: ActiveValue = scfia.new_bv_slice(instruction_32.clone(), 31, 31);
                let imm_4_0: ActiveValue = scfia.new_bv_concat(imm_4_1.clone(), scfia.new_bv_concrete(0b0, 1), 5);
                let imm_10_0: ActiveValue = scfia.new_bv_concat(imm_10_5.clone(), imm_4_0.clone(), 11);
                let imm_11_0: ActiveValue = scfia.new_bv_concat(imm11.clone(), imm_10_0.clone(), 12);
                let imm_12_0: ActiveValue = scfia.new_bv_concat(imm12.clone(), imm_11_0.clone(), 13);
                let offset: ActiveValue = scfia.new_bv_sign_extend(imm_12_0.clone(), 13, 32);
                let address: ActiveValue = scfia.new_bv_add((*state).pc.clone(), offset.clone(), 32);
                (*state).pc = address.clone();
            } else {
                _progress_pc_4(state, memory, scfia.clone(), hints);
            }
        } else if scfia.check_condition(scfia.new_bool_eq(funct3.clone(), scfia.new_bv_concrete(0b100, 3))) {
            let lhs: ActiveValue = _register_read_BV32(
                state,
                _extract_rs1_32(instruction_32.clone(), memory, scfia.clone(), hints),
                memory,
                scfia.clone(),
                hints,
            );
            let rhs: ActiveValue = _register_read_BV32(
                state,
                _extract_rs2_32(instruction_32.clone(), memory, scfia.clone(), hints),
                memory,
                scfia.clone(),
                hints,
            );
            if scfia.check_condition(scfia.new_bool_signed_less_than(lhs.clone(), rhs.clone())) {
                let imm_4_1: ActiveValue = scfia.new_bv_slice(instruction_32.clone(), 11, 8);
                let imm_10_5: ActiveValue = scfia.new_bv_slice(instruction_32.clone(), 30, 25);
                let imm11: ActiveValue = scfia.new_bv_slice(instruction_32.clone(), 7, 7);
                let imm12: ActiveValue = scfia.new_bv_slice(instruction_32.clone(), 31, 31);
                let imm_4_0: ActiveValue = scfia.new_bv_concat(imm_4_1.clone(), scfia.new_bv_concrete(0b0, 1), 5);
                let imm_10_0: ActiveValue = scfia.new_bv_concat(imm_10_5.clone(), imm_4_0.clone(), 11);
                let imm_11_0: ActiveValue = scfia.new_bv_concat(imm11.clone(), imm_10_0.clone(), 12);
                let imm_12_0: ActiveValue = scfia.new_bv_concat(imm12.clone(), imm_11_0.clone(), 13);
                let offset: ActiveValue = scfia.new_bv_sign_extend(imm_12_0.clone(), 13, 32);
                let address: ActiveValue = scfia.new_bv_add((*state).pc.clone(), offset.clone(), 32);
                (*state).pc = address.clone();
            } else {
                _progress_pc_4(state, memory, scfia.clone(), hints);
            }
        } else if scfia.check_condition(scfia.new_bool_eq(funct3.clone(), scfia.new_bv_concrete(0b101, 3))) {
            let lhs: ActiveValue = _register_read_BV32(
                state,
                _extract_rs1_32(instruction_32.clone(), memory, scfia.clone(), hints),
                memory,
                scfia.clone(),
                hints,
            );
            let rhs: ActiveValue = _register_read_BV32(
                state,
                _extract_rs2_32(instruction_32.clone(), memory, scfia.clone(), hints),
                memory,
                scfia.clone(),
                hints,
            );
            if scfia.check_condition(scfia.new_bool_signed_less_than(lhs.clone(), rhs.clone())) {
                _progress_pc_4(state, memory, scfia.clone(), hints);
            } else {
                let imm_4_1: ActiveValue = scfia.new_bv_slice(instruction_32.clone(), 11, 8);
                let imm_10_5: ActiveValue = scfia.new_bv_slice(instruction_32.clone(), 30, 25);
                let imm11: ActiveValue = scfia.new_bv_slice(instruction_32.clone(), 7, 7);
                let imm12: ActiveValue = scfia.new_bv_slice(instruction_32.clone(), 31, 31);
                let imm_4_0: ActiveValue = scfia.new_bv_concat(imm_4_1.clone(), scfia.new_bv_concrete(0b0, 1), 5);
                let imm_10_0: ActiveValue = scfia.new_bv_concat(imm_10_5.clone(), imm_4_0.clone(), 11);
                let imm_11_0: ActiveValue = scfia.new_bv_concat(imm11.clone(), imm_10_0.clone(), 12);
                let imm_12_0: ActiveValue = scfia.new_bv_concat(imm12.clone(), imm_11_0.clone(), 13);
                let offset: ActiveValue = scfia.new_bv_sign_extend(imm_12_0.clone(), 13, 32);
                let address: ActiveValue = scfia.new_bv_add((*state).pc.clone(), offset.clone(), 32);
                (*state).pc = address.clone();
            }
        } else if scfia.check_condition(scfia.new_bool_eq(funct3.clone(), scfia.new_bv_concrete(0b110, 3))) {
            let lhs: ActiveValue = _register_read_BV32(
                state,
                _extract_rs1_32(instruction_32.clone(), memory, scfia.clone(), hints),
                memory,
                scfia.clone(),
                hints,
            );
            let rhs: ActiveValue = _register_read_BV32(
                state,
                _extract_rs2_32(instruction_32.clone(), memory, scfia.clone(), hints),
                memory,
                scfia.clone(),
                hints,
            );
            if scfia.check_condition(scfia.new_bool_unsigned_less_than(lhs.clone(), rhs.clone())) {
                let imm_4_1: ActiveValue = scfia.new_bv_slice(instruction_32.clone(), 11, 8);
                let imm_10_5: ActiveValue = scfia.new_bv_slice(instruction_32.clone(), 30, 25);
                let imm11: ActiveValue = scfia.new_bv_slice(instruction_32.clone(), 7, 7);
                let imm12: ActiveValue = scfia.new_bv_slice(instruction_32.clone(), 31, 31);
                let imm_4_0: ActiveValue = scfia.new_bv_concat(imm_4_1.clone(), scfia.new_bv_concrete(0b0, 1), 5);
                let imm_10_0: ActiveValue = scfia.new_bv_concat(imm_10_5.clone(), imm_4_0.clone(), 11);
                let imm_11_0: ActiveValue = scfia.new_bv_concat(imm11.clone(), imm_10_0.clone(), 12);
                let imm_12_0: ActiveValue = scfia.new_bv_concat(imm12.clone(), imm_11_0.clone(), 13);
                let offset: ActiveValue = scfia.new_bv_sign_extend(imm_12_0.clone(), 13, 32);
                let address: ActiveValue = scfia.new_bv_add((*state).pc.clone(), offset.clone(), 32);
                (*state).pc = address.clone();
            } else {
                _progress_pc_4(state, memory, scfia.clone(), hints);
            }
        } else if scfia.check_condition(scfia.new_bool_eq(funct3.clone(), scfia.new_bv_concrete(0b111, 3))) {
            let lhs: ActiveValue = _register_read_BV32(
                state,
                _extract_rs1_32(instruction_32.clone(), memory, scfia.clone(), hints),
                memory,
                scfia.clone(),
                hints,
            );
            let rhs: ActiveValue = _register_read_BV32(
                state,
                _extract_rs2_32(instruction_32.clone(), memory, scfia.clone(), hints),
                memory,
                scfia.clone(),
                hints,
            );
            if scfia.check_condition(scfia.new_bool_unsigned_less_than(lhs.clone(), rhs.clone())) {
                _progress_pc_4(state, memory, scfia.clone(), hints);
            } else {
                let imm_4_1: ActiveValue = scfia.new_bv_slice(instruction_32.clone(), 11, 8);
                let imm_10_5: ActiveValue = scfia.new_bv_slice(instruction_32.clone(), 30, 25);
                let imm11: ActiveValue = scfia.new_bv_slice(instruction_32.clone(), 7, 7);
                let imm12: ActiveValue = scfia.new_bv_slice(instruction_32.clone(), 31, 31);
                let imm_4_0: ActiveValue = scfia.new_bv_concat(imm_4_1.clone(), scfia.new_bv_concrete(0b0, 1), 5);
                let imm_10_0: ActiveValue = scfia.new_bv_concat(imm_10_5.clone(), imm_4_0.clone(), 11);
                let imm_11_0: ActiveValue = scfia.new_bv_concat(imm11.clone(), imm_10_0.clone(), 12);
                let imm_12_0: ActiveValue = scfia.new_bv_concat(imm12.clone(), imm_11_0.clone(), 13);
                let offset: ActiveValue = scfia.new_bv_sign_extend(imm_12_0.clone(), 13, 32);
                let address: ActiveValue = scfia.new_bv_add((*state).pc.clone(), offset.clone(), 32);
                (*state).pc = address.clone();
            }
        } else {
            unimplemented!();
        }
    } else if scfia.check_condition(scfia.new_bool_eq(opcode.clone(), scfia.new_bv_concrete(0b1100111, 7))) {
        let funct3: ActiveValue = scfia.new_bv_slice(instruction_32.clone(), 14, 12);
        if scfia.check_condition(scfia.new_bool_eq(funct3.clone(), scfia.new_bv_concrete(0b0, 3))) {
            let dst: ActiveValue = _extract_rd_32(instruction_32.clone(), memory, scfia.clone(), hints);
            let rs1: ActiveValue = _extract_rs1_32(instruction_32.clone(), memory, scfia.clone(), hints);
            let s1: ActiveValue = _register_read_BV32(state, rs1.clone(), memory, scfia.clone(), hints);
            let offset: ActiveValue = scfia.new_bv_slice(instruction_32.clone(), 31, 20);
            let offset_32: ActiveValue = scfia.new_bv_sign_extend(offset.clone(), 12, 32);
            let address: ActiveValue = scfia.new_bv_add(s1.clone(), offset_32.clone(), 32);
            let return_address: ActiveValue = scfia.new_bv_add((*state).pc.clone(), scfia.new_bv_concrete(0b100, 32), 32);
            (*state).pc = address.clone();
            _register_write_BV32(state, dst.clone(), return_address.clone(), memory, scfia.clone(), hints);
        } else {
            unimplemented!();
        }
    } else if scfia.check_condition(scfia.new_bool_eq(opcode.clone(), scfia.new_bv_concrete(0b1101111, 7))) {
        let imm_20: ActiveValue = scfia.new_bv_slice(instruction_32.clone(), 31, 31);
        let imm_10_1: ActiveValue = scfia.new_bv_slice(instruction_32.clone(), 30, 21);
        let imm_11: ActiveValue = scfia.new_bv_slice(instruction_32.clone(), 20, 20);
        let imm_19_12: ActiveValue = scfia.new_bv_slice(instruction_32.clone(), 19, 12);
        let offset_10_0: ActiveValue = scfia.new_bv_concat(imm_10_1.clone(), scfia.new_bv_concrete(0b0, 1), 11);
        let offset_11_0: ActiveValue = scfia.new_bv_concat(imm_11.clone(), offset_10_0.clone(), 12);
        let offset_19_0: ActiveValue = scfia.new_bv_concat(imm_19_12.clone(), offset_11_0.clone(), 20);
        let offset_20_0: ActiveValue = scfia.new_bv_concat(imm_20.clone(), offset_19_0.clone(), 21);
        let offset_32: ActiveValue = scfia.new_bv_sign_extend(offset_20_0.clone(), 21, 32);
        let address: ActiveValue = scfia.new_bv_add((*state).pc.clone(), offset_32.clone(), 32);
        let return_address: ActiveValue = scfia.new_bv_add((*state).pc.clone(), scfia.new_bv_concrete(0b100, 32), 32);
        let dst: ActiveValue = _extract_rd_32(instruction_32.clone(), memory, scfia.clone(), hints);
        _register_write_BV32(state, dst.clone(), return_address.clone(), memory, scfia.clone(), hints);
        (*state).pc = address.clone();
    } else {
        unimplemented!();
    }
}

pub unsafe fn _extract_rd_32(op: ActiveValue, memory: &mut Memory, scfia: Scfia, hints: &mut Option<SymbolicHints>) -> ActiveValue {
    return scfia.new_bv_slice(op.clone(), 11, 7);
}

pub unsafe fn _extract_rs1_32(op: ActiveValue, memory: &mut Memory, scfia: Scfia, hints: &mut Option<SymbolicHints>) -> ActiveValue {
    return scfia.new_bv_slice(op.clone(), 19, 15);
}

pub unsafe fn _extract_rs2_32(op: ActiveValue, memory: &mut Memory, scfia: Scfia, hints: &mut Option<SymbolicHints>) -> ActiveValue {
    return scfia.new_bv_slice(op.clone(), 24, 20);
}

pub unsafe fn _progress_pc_4(state: *mut SystemState, memory: &mut Memory, scfia: Scfia, hints: &mut Option<SymbolicHints>) {
    let old_pc: ActiveValue = (*state).pc.clone();
    let new_pc: ActiveValue = scfia.new_bv_add(old_pc.clone(), scfia.new_bv_concrete(0b100, 32), 32);
    (*state).pc = new_pc.clone();
}

pub unsafe fn _register_write_BV32(
    state: *mut SystemState,
    register_id: ActiveValue,
    value: ActiveValue,
    memory: &mut Memory,
    scfia: Scfia,
    hints: &mut Option<SymbolicHints>,
) {
    if scfia.check_condition(scfia.new_bool_eq(register_id.clone(), scfia.new_bv_concrete(0b0, 5))) {
    } else if scfia.check_condition(scfia.new_bool_eq(register_id.clone(), scfia.new_bv_concrete(0b1, 5))) {
        (*state).x1 = value.clone();
    } else if scfia.check_condition(scfia.new_bool_eq(register_id.clone(), scfia.new_bv_concrete(0b10, 5))) {
        (*state).x2 = value.clone();
    } else if scfia.check_condition(scfia.new_bool_eq(register_id.clone(), scfia.new_bv_concrete(0b11, 5))) {
        (*state).x3 = value.clone();
    } else if scfia.check_condition(scfia.new_bool_eq(register_id.clone(), scfia.new_bv_concrete(0b100, 5))) {
        (*state).x4 = value.clone();
    } else if scfia.check_condition(scfia.new_bool_eq(register_id.clone(), scfia.new_bv_concrete(0b101, 5))) {
        (*state).x5 = value.clone();
    } else if scfia.check_condition(scfia.new_bool_eq(register_id.clone(), scfia.new_bv_concrete(0b110, 5))) {
        (*state).x6 = value.clone();
    } else if scfia.check_condition(scfia.new_bool_eq(register_id.clone(), scfia.new_bv_concrete(0b111, 5))) {
        (*state).x7 = value.clone();
    } else if scfia.check_condition(scfia.new_bool_eq(register_id.clone(), scfia.new_bv_concrete(0b1000, 5))) {
        (*state).x8 = value.clone();
    } else if scfia.check_condition(scfia.new_bool_eq(register_id.clone(), scfia.new_bv_concrete(0b1001, 5))) {
        (*state).x9 = value.clone();
    } else if scfia.check_condition(scfia.new_bool_eq(register_id.clone(), scfia.new_bv_concrete(0b1010, 5))) {
        (*state).x10 = value.clone();
    } else if scfia.check_condition(scfia.new_bool_eq(register_id.clone(), scfia.new_bv_concrete(0b1011, 5))) {
        (*state).x11 = value.clone();
    } else if scfia.check_condition(scfia.new_bool_eq(register_id.clone(), scfia.new_bv_concrete(0b1100, 5))) {
        (*state).x12 = value.clone();
    } else if scfia.check_condition(scfia.new_bool_eq(register_id.clone(), scfia.new_bv_concrete(0b1101, 5))) {
        (*state).x13 = value.clone();
    } else if scfia.check_condition(scfia.new_bool_eq(register_id.clone(), scfia.new_bv_concrete(0b1110, 5))) {
        (*state).x14 = value.clone();
    } else if scfia.check_condition(scfia.new_bool_eq(register_id.clone(), scfia.new_bv_concrete(0b1111, 5))) {
        (*state).x15 = value.clone();
    } else if scfia.check_condition(scfia.new_bool_eq(register_id.clone(), scfia.new_bv_concrete(0b10000, 5))) {
        (*state).x16 = value.clone();
    } else if scfia.check_condition(scfia.new_bool_eq(register_id.clone(), scfia.new_bv_concrete(0b10001, 5))) {
        (*state).x17 = value.clone();
    } else if scfia.check_condition(scfia.new_bool_eq(register_id.clone(), scfia.new_bv_concrete(0b10010, 5))) {
        (*state).x18 = value.clone();
    } else if scfia.check_condition(scfia.new_bool_eq(register_id.clone(), scfia.new_bv_concrete(0b10011, 5))) {
        (*state).x19 = value.clone();
    } else if scfia.check_condition(scfia.new_bool_eq(register_id.clone(), scfia.new_bv_concrete(0b10100, 5))) {
        (*state).x20 = value.clone();
    } else if scfia.check_condition(scfia.new_bool_eq(register_id.clone(), scfia.new_bv_concrete(0b10101, 5))) {
        (*state).x21 = value.clone();
    } else if scfia.check_condition(scfia.new_bool_eq(register_id.clone(), scfia.new_bv_concrete(0b10110, 5))) {
        (*state).x22 = value.clone();
    } else if scfia.check_condition(scfia.new_bool_eq(register_id.clone(), scfia.new_bv_concrete(0b10111, 5))) {
        (*state).x23 = value.clone();
    } else if scfia.check_condition(scfia.new_bool_eq(register_id.clone(), scfia.new_bv_concrete(0b11000, 5))) {
        (*state).x24 = value.clone();
    } else if scfia.check_condition(scfia.new_bool_eq(register_id.clone(), scfia.new_bv_concrete(0b11001, 5))) {
        (*state).x25 = value.clone();
    } else if scfia.check_condition(scfia.new_bool_eq(register_id.clone(), scfia.new_bv_concrete(0b11010, 5))) {
        (*state).x26 = value.clone();
    } else if scfia.check_condition(scfia.new_bool_eq(register_id.clone(), scfia.new_bv_concrete(0b11011, 5))) {
        (*state).x27 = value.clone();
    } else if scfia.check_condition(scfia.new_bool_eq(register_id.clone(), scfia.new_bv_concrete(0b11100, 5))) {
        (*state).x28 = value.clone();
    } else if scfia.check_condition(scfia.new_bool_eq(register_id.clone(), scfia.new_bv_concrete(0b11101, 5))) {
        (*state).x29 = value.clone();
    } else if scfia.check_condition(scfia.new_bool_eq(register_id.clone(), scfia.new_bv_concrete(0b11110, 5))) {
        (*state).x30 = value.clone();
    } else if scfia.check_condition(scfia.new_bool_eq(register_id.clone(), scfia.new_bv_concrete(0b11111, 5))) {
        (*state).x31 = value.clone();
    } else {
        unimplemented!();
    }
}

pub unsafe fn _register_read_BV32(
    state: *mut SystemState,
    register_id: ActiveValue,
    memory: &mut Memory,
    scfia: Scfia,
    hints: &mut Option<SymbolicHints>,
) -> ActiveValue {
    if scfia.check_condition(scfia.new_bool_eq(register_id.clone(), scfia.new_bv_concrete(0b0, 5))) {
        return scfia.new_bv_concrete(0b0, 32);
    } else if scfia.check_condition(scfia.new_bool_eq(register_id.clone(), scfia.new_bv_concrete(0b1, 5))) {
        return (*state).x1.clone();
    } else if scfia.check_condition(scfia.new_bool_eq(register_id.clone(), scfia.new_bv_concrete(0b10, 5))) {
        return (*state).x2.clone();
    } else if scfia.check_condition(scfia.new_bool_eq(register_id.clone(), scfia.new_bv_concrete(0b11, 5))) {
        return (*state).x3.clone();
    } else if scfia.check_condition(scfia.new_bool_eq(register_id.clone(), scfia.new_bv_concrete(0b100, 5))) {
        return (*state).x4.clone();
    } else if scfia.check_condition(scfia.new_bool_eq(register_id.clone(), scfia.new_bv_concrete(0b101, 5))) {
        return (*state).x5.clone();
    } else if scfia.check_condition(scfia.new_bool_eq(register_id.clone(), scfia.new_bv_concrete(0b110, 5))) {
        return (*state).x6.clone();
    } else if scfia.check_condition(scfia.new_bool_eq(register_id.clone(), scfia.new_bv_concrete(0b111, 5))) {
        return (*state).x7.clone();
    } else if scfia.check_condition(scfia.new_bool_eq(register_id.clone(), scfia.new_bv_concrete(0b1000, 5))) {
        return (*state).x8.clone();
    } else if scfia.check_condition(scfia.new_bool_eq(register_id.clone(), scfia.new_bv_concrete(0b1001, 5))) {
        return (*state).x9.clone();
    } else if scfia.check_condition(scfia.new_bool_eq(register_id.clone(), scfia.new_bv_concrete(0b1010, 5))) {
        return (*state).x10.clone();
    } else if scfia.check_condition(scfia.new_bool_eq(register_id.clone(), scfia.new_bv_concrete(0b1011, 5))) {
        return (*state).x11.clone();
    } else if scfia.check_condition(scfia.new_bool_eq(register_id.clone(), scfia.new_bv_concrete(0b1100, 5))) {
        return (*state).x12.clone();
    } else if scfia.check_condition(scfia.new_bool_eq(register_id.clone(), scfia.new_bv_concrete(0b1101, 5))) {
        return (*state).x13.clone();
    } else if scfia.check_condition(scfia.new_bool_eq(register_id.clone(), scfia.new_bv_concrete(0b1110, 5))) {
        return (*state).x14.clone();
    } else if scfia.check_condition(scfia.new_bool_eq(register_id.clone(), scfia.new_bv_concrete(0b1111, 5))) {
        return (*state).x15.clone();
    } else if scfia.check_condition(scfia.new_bool_eq(register_id.clone(), scfia.new_bv_concrete(0b10000, 5))) {
        return (*state).x16.clone();
    } else if scfia.check_condition(scfia.new_bool_eq(register_id.clone(), scfia.new_bv_concrete(0b10001, 5))) {
        return (*state).x17.clone();
    } else if scfia.check_condition(scfia.new_bool_eq(register_id.clone(), scfia.new_bv_concrete(0b10010, 5))) {
        return (*state).x18.clone();
    } else if scfia.check_condition(scfia.new_bool_eq(register_id.clone(), scfia.new_bv_concrete(0b10011, 5))) {
        return (*state).x19.clone();
    } else if scfia.check_condition(scfia.new_bool_eq(register_id.clone(), scfia.new_bv_concrete(0b10100, 5))) {
        return (*state).x20.clone();
    } else if scfia.check_condition(scfia.new_bool_eq(register_id.clone(), scfia.new_bv_concrete(0b10101, 5))) {
        return (*state).x21.clone();
    } else if scfia.check_condition(scfia.new_bool_eq(register_id.clone(), scfia.new_bv_concrete(0b10110, 5))) {
        return (*state).x22.clone();
    } else if scfia.check_condition(scfia.new_bool_eq(register_id.clone(), scfia.new_bv_concrete(0b10111, 5))) {
        return (*state).x23.clone();
    } else if scfia.check_condition(scfia.new_bool_eq(register_id.clone(), scfia.new_bv_concrete(0b11000, 5))) {
        return (*state).x24.clone();
    } else if scfia.check_condition(scfia.new_bool_eq(register_id.clone(), scfia.new_bv_concrete(0b11001, 5))) {
        return (*state).x25.clone();
    } else if scfia.check_condition(scfia.new_bool_eq(register_id.clone(), scfia.new_bv_concrete(0b11010, 5))) {
        return (*state).x26.clone();
    } else if scfia.check_condition(scfia.new_bool_eq(register_id.clone(), scfia.new_bv_concrete(0b11011, 5))) {
        return (*state).x27.clone();
    } else if scfia.check_condition(scfia.new_bool_eq(register_id.clone(), scfia.new_bv_concrete(0b11100, 5))) {
        return (*state).x28.clone();
    } else if scfia.check_condition(scfia.new_bool_eq(register_id.clone(), scfia.new_bv_concrete(0b11101, 5))) {
        return (*state).x29.clone();
    } else if scfia.check_condition(scfia.new_bool_eq(register_id.clone(), scfia.new_bv_concrete(0b11110, 5))) {
        return (*state).x30.clone();
    } else if scfia.check_condition(scfia.new_bool_eq(register_id.clone(), scfia.new_bv_concrete(0b11111, 5))) {
        return (*state).x31.clone();
    } else {
        unimplemented!();
    }
}

pub unsafe fn _execute_add32(
    state: *mut SystemState,
    destination_id: ActiveValue,
    source1_id: ActiveValue,
    source2_id: ActiveValue,
    memory: &mut Memory,
    scfia: Scfia,
    hints: &mut Option<SymbolicHints>,
) {
    let s1: ActiveValue = _register_read_BV32(state, source1_id.clone(), memory, scfia.clone(), hints);
    let s2: ActiveValue = _register_read_BV32(state, source2_id.clone(), memory, scfia.clone(), hints);
    let sum: ActiveValue = scfia.new_bv_add(s1.clone(), s2.clone(), 32);
    _register_write_BV32(state, destination_id.clone(), sum.clone(), memory, scfia.clone(), hints);
}
