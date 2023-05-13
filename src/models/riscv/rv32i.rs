#![allow(clippy::all)]
#![allow(non_snake_case)]
#![allow(unused)]
use crate::{memory::Memory, scfia::Scfia, values::active_value::ActiveValue, StepContext, SymbolicHints, ScfiaComposition};

pub struct RV32IScfiaComposition {}
impl ScfiaComposition for RV32IScfiaComposition {
    
}

pub struct RV32i {
    pub state: SystemState,
    pub memory: Memory<RV32IScfiaComposition>,
    pub scfia: Scfia<RV32IScfiaComposition>,
}

impl RV32i {
    pub fn step(&mut self, mut hints: Option<SymbolicHints>) {
        unsafe {
            let mut context = StepContext {
                memory: &mut self.memory,
                scfia: self.scfia.clone(),
                hints,
                fork_sink: None,
            };
            _step(&mut self.state, &mut context);
        }
    }

    /*
    pub fn step_forking(model: RV32i, mut hints: Option<SymbolicHints>) -> Vec<RV32i> {
        unsafe {
            let mut states = vec![model];
            let mut results = vec![];

            while let Some(mut model) = states.pop() {
                _step(&mut model.state, &mut model.memory, model.scfia.clone(), &mut hints);
                // TODO apply write cache to model
                results.push(model)
            }

            results
        }
    }
    */

    pub fn clone_model(&self) -> RV32i {
        unsafe {
            let own_scfia = self.scfia.inner.try_borrow().unwrap();
            let (cloned_scfia, cloned_active_symbols) = own_scfia.clone_symbols();
            let cloned_memory = self.memory.clone_to(cloned_scfia.clone());

            RV32i {
                memory: cloned_memory,
                state: self.state.clone_to(cloned_scfia.clone()),
                scfia: cloned_scfia,
            }
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

impl SystemState {
    fn clone_to(&self, cloned_scfia: Scfia) -> SystemState {
        SystemState {
            x0: cloned_scfia.try_get_active_value(self.x0.try_borrow().unwrap().id).unwrap(),
            x1: cloned_scfia.try_get_active_value(self.x1.try_borrow().unwrap().id).unwrap(),
            x2: cloned_scfia.try_get_active_value(self.x2.try_borrow().unwrap().id).unwrap(),
            x3: cloned_scfia.try_get_active_value(self.x3.try_borrow().unwrap().id).unwrap(),
            x4: cloned_scfia.try_get_active_value(self.x4.try_borrow().unwrap().id).unwrap(),
            x5: cloned_scfia.try_get_active_value(self.x5.try_borrow().unwrap().id).unwrap(),
            x6: cloned_scfia.try_get_active_value(self.x6.try_borrow().unwrap().id).unwrap(),
            x7: cloned_scfia.try_get_active_value(self.x7.try_borrow().unwrap().id).unwrap(),
            x8: cloned_scfia.try_get_active_value(self.x8.try_borrow().unwrap().id).unwrap(),
            x9: cloned_scfia.try_get_active_value(self.x9.try_borrow().unwrap().id).unwrap(),
            x10: cloned_scfia.try_get_active_value(self.x10.try_borrow().unwrap().id).unwrap(),
            x11: cloned_scfia.try_get_active_value(self.x11.try_borrow().unwrap().id).unwrap(),
            x12: cloned_scfia.try_get_active_value(self.x12.try_borrow().unwrap().id).unwrap(),
            x13: cloned_scfia.try_get_active_value(self.x13.try_borrow().unwrap().id).unwrap(),
            x14: cloned_scfia.try_get_active_value(self.x14.try_borrow().unwrap().id).unwrap(),
            x15: cloned_scfia.try_get_active_value(self.x15.try_borrow().unwrap().id).unwrap(),
            x16: cloned_scfia.try_get_active_value(self.x16.try_borrow().unwrap().id).unwrap(),
            x17: cloned_scfia.try_get_active_value(self.x17.try_borrow().unwrap().id).unwrap(),
            x18: cloned_scfia.try_get_active_value(self.x18.try_borrow().unwrap().id).unwrap(),
            x19: cloned_scfia.try_get_active_value(self.x19.try_borrow().unwrap().id).unwrap(),
            x20: cloned_scfia.try_get_active_value(self.x20.try_borrow().unwrap().id).unwrap(),
            x21: cloned_scfia.try_get_active_value(self.x21.try_borrow().unwrap().id).unwrap(),
            x22: cloned_scfia.try_get_active_value(self.x22.try_borrow().unwrap().id).unwrap(),
            x23: cloned_scfia.try_get_active_value(self.x23.try_borrow().unwrap().id).unwrap(),
            x24: cloned_scfia.try_get_active_value(self.x24.try_borrow().unwrap().id).unwrap(),
            x25: cloned_scfia.try_get_active_value(self.x25.try_borrow().unwrap().id).unwrap(),
            x26: cloned_scfia.try_get_active_value(self.x26.try_borrow().unwrap().id).unwrap(),
            x27: cloned_scfia.try_get_active_value(self.x27.try_borrow().unwrap().id).unwrap(),
            x28: cloned_scfia.try_get_active_value(self.x28.try_borrow().unwrap().id).unwrap(),
            x29: cloned_scfia.try_get_active_value(self.x29.try_borrow().unwrap().id).unwrap(),
            x30: cloned_scfia.try_get_active_value(self.x30.try_borrow().unwrap().id).unwrap(),
            x31: cloned_scfia.try_get_active_value(self.x31.try_borrow().unwrap().id).unwrap(),
            pc: cloned_scfia.try_get_active_value(self.pc.try_borrow().unwrap().id).unwrap(),
        }
    }
}

#[derive(Debug)]
pub struct ComplexSpecialStruct {
    pub some_flag: ActiveValue,
}

impl ComplexSpecialStruct {
    fn clone_to(&self, cloned_scfia: Scfia) -> ComplexSpecialStruct {
        ComplexSpecialStruct {
            some_flag: cloned_scfia.try_get_active_value(self.some_flag.try_borrow().unwrap().id).unwrap(),
        }
    }
}

unsafe fn _reset(state: *mut SystemState, context: *mut StepContext) {
    (*state).x0 = (*context).scfia.new_bv_concrete(0b0, 32, &mut (*context).fork_sink);
    (*state).x1 = (*context).scfia.new_bv_concrete(0b0, 32, &mut (*context).fork_sink);
    (*state).x2 = (*context).scfia.new_bv_concrete(0b0, 32, &mut (*context).fork_sink);
    (*state).x3 = (*context).scfia.new_bv_concrete(0b0, 32, &mut (*context).fork_sink);
    (*state).x4 = (*context).scfia.new_bv_concrete(0b0, 32, &mut (*context).fork_sink);
    (*state).x5 = (*context).scfia.new_bv_concrete(0b0, 32, &mut (*context).fork_sink);
    (*state).x6 = (*context).scfia.new_bv_concrete(0b0, 32, &mut (*context).fork_sink);
    (*state).x7 = (*context).scfia.new_bv_concrete(0b0, 32, &mut (*context).fork_sink);
    (*state).x8 = (*context).scfia.new_bv_concrete(0b0, 32, &mut (*context).fork_sink);
    (*state).x9 = (*context).scfia.new_bv_concrete(0b0, 32, &mut (*context).fork_sink);
    (*state).x10 = (*context).scfia.new_bv_concrete(0b0, 32, &mut (*context).fork_sink);
    (*state).x11 = (*context).scfia.new_bv_concrete(0b0, 32, &mut (*context).fork_sink);
    (*state).x12 = (*context).scfia.new_bv_concrete(0b0, 32, &mut (*context).fork_sink);
    (*state).x13 = (*context).scfia.new_bv_concrete(0b0, 32, &mut (*context).fork_sink);
    (*state).x14 = (*context).scfia.new_bv_concrete(0b0, 32, &mut (*context).fork_sink);
    (*state).x15 = (*context).scfia.new_bv_concrete(0b0, 32, &mut (*context).fork_sink);
    (*state).x16 = (*context).scfia.new_bv_concrete(0b0, 32, &mut (*context).fork_sink);
    (*state).x17 = (*context).scfia.new_bv_concrete(0b0, 32, &mut (*context).fork_sink);
    (*state).x18 = (*context).scfia.new_bv_concrete(0b0, 32, &mut (*context).fork_sink);
    (*state).x19 = (*context).scfia.new_bv_concrete(0b0, 32, &mut (*context).fork_sink);
    (*state).x20 = (*context).scfia.new_bv_concrete(0b0, 32, &mut (*context).fork_sink);
    (*state).x21 = (*context).scfia.new_bv_concrete(0b0, 32, &mut (*context).fork_sink);
    (*state).x22 = (*context).scfia.new_bv_concrete(0b0, 32, &mut (*context).fork_sink);
    (*state).x23 = (*context).scfia.new_bv_concrete(0b0, 32, &mut (*context).fork_sink);
    (*state).x24 = (*context).scfia.new_bv_concrete(0b0, 32, &mut (*context).fork_sink);
    (*state).x25 = (*context).scfia.new_bv_concrete(0b0, 32, &mut (*context).fork_sink);
    (*state).x26 = (*context).scfia.new_bv_concrete(0b0, 32, &mut (*context).fork_sink);
    (*state).x27 = (*context).scfia.new_bv_concrete(0b0, 32, &mut (*context).fork_sink);
    (*state).x28 = (*context).scfia.new_bv_concrete(0b0, 32, &mut (*context).fork_sink);
    (*state).x29 = (*context).scfia.new_bv_concrete(0b0, 32, &mut (*context).fork_sink);
    (*state).x30 = (*context).scfia.new_bv_concrete(0b0, 32, &mut (*context).fork_sink);
    (*state).x31 = (*context).scfia.new_bv_concrete(0b0, 32, &mut (*context).fork_sink);
}

unsafe fn _sum(state: *mut SystemState, context: *mut StepContext) -> ActiveValue {
    return (*context)
        .scfia
        .new_bv_add((*state).x1.clone(), (*state).x2.clone(), 32, &mut (*context).fork_sink);
}

unsafe fn _step(state: *mut SystemState, context: *mut StepContext) {
    let instruction_32: ActiveValue = (*(*context).memory).read(
        (*state).pc.clone(),
        32,
        (*context).scfia.clone(),
        &mut (*context).hints,
        &mut (*context).fork_sink,
    );
    let opcode: ActiveValue = (*context).scfia.new_bv_slice(instruction_32.clone(), 6, 0, &mut (*context).fork_sink);
    if (*context).scfia.check_condition(
        (*context).scfia.new_bool_eq(
            opcode.clone(),
            (*context).scfia.new_bv_concrete(0b11, 7, &mut (*context).fork_sink),
            &mut (*context).fork_sink,
        ),
        &mut (*context).fork_sink,
    ) {
        let funct3: ActiveValue = (*context).scfia.new_bv_slice(instruction_32.clone(), 14, 12, &mut (*context).fork_sink);
        if (*context).scfia.check_condition(
            (*context).scfia.new_bool_eq(
                funct3.clone(),
                (*context).scfia.new_bv_concrete(0b1, 3, &mut (*context).fork_sink),
                &mut (*context).fork_sink,
            ),
            &mut (*context).fork_sink,
        ) {
            let rd: ActiveValue = _extract_rd_32(instruction_32.clone(), context);
            let rs1: ActiveValue = _extract_rs1_32(instruction_32.clone(), context);
            let imm: ActiveValue = (*context).scfia.new_bv_slice(instruction_32.clone(), 31, 20, &mut (*context).fork_sink);
            let imm32: ActiveValue = (*context).scfia.new_bv_sign_extend(imm.clone(), 12, 32, &mut (*context).fork_sink);
            let base_address: ActiveValue = _register_read_BV32(state, rs1.clone(), context);
            let address: ActiveValue = (*context).scfia.new_bv_add(base_address.clone(), imm32.clone(), 32, &mut (*context).fork_sink);
            let value16: ActiveValue =
                (*(*context).memory).read(address.clone(), 16, (*context).scfia.clone(), &mut (*context).hints, &mut (*context).fork_sink);
            let value: ActiveValue = (*context).scfia.new_bv_sign_extend(value16.clone(), 16, 32, &mut (*context).fork_sink);
            _register_write_BV32(state, rd.clone(), value.clone(), context);
            _progress_pc_4(state, context);
        } else if (*context).scfia.check_condition(
            (*context).scfia.new_bool_eq(
                funct3.clone(),
                (*context).scfia.new_bv_concrete(0b10, 3, &mut (*context).fork_sink),
                &mut (*context).fork_sink,
            ),
            &mut (*context).fork_sink,
        ) {
            let rd: ActiveValue = _extract_rd_32(instruction_32.clone(), context);
            let rs1: ActiveValue = _extract_rs1_32(instruction_32.clone(), context);
            let imm: ActiveValue = (*context).scfia.new_bv_slice(instruction_32.clone(), 31, 20, &mut (*context).fork_sink);
            let imm32: ActiveValue = (*context).scfia.new_bv_sign_extend(imm.clone(), 12, 32, &mut (*context).fork_sink);
            let base_address: ActiveValue = _register_read_BV32(state, rs1.clone(), context);
            let address: ActiveValue = (*context).scfia.new_bv_add(base_address.clone(), imm32.clone(), 32, &mut (*context).fork_sink);
            let value: ActiveValue = (*(*context).memory).read(address.clone(), 32, (*context).scfia.clone(), &mut (*context).hints, &mut (*context).fork_sink);
            _register_write_BV32(state, rd.clone(), value.clone(), context);
            _progress_pc_4(state, context);
        } else if (*context).scfia.check_condition(
            (*context).scfia.new_bool_eq(
                funct3.clone(),
                (*context).scfia.new_bv_concrete(0b100, 3, &mut (*context).fork_sink),
                &mut (*context).fork_sink,
            ),
            &mut (*context).fork_sink,
        ) {
            let rd: ActiveValue = _extract_rd_32(instruction_32.clone(), context);
            let rs1: ActiveValue = _extract_rs1_32(instruction_32.clone(), context);
            let imm: ActiveValue = (*context).scfia.new_bv_slice(instruction_32.clone(), 31, 20, &mut (*context).fork_sink);
            let imm32: ActiveValue = (*context).scfia.new_bv_sign_extend(imm.clone(), 12, 32, &mut (*context).fork_sink);
            let base_address: ActiveValue = _register_read_BV32(state, rs1.clone(), context);
            let address: ActiveValue = (*context).scfia.new_bv_add(base_address.clone(), imm32.clone(), 32, &mut (*context).fork_sink);
            let value8: ActiveValue = (*(*context).memory).read(address.clone(), 8, (*context).scfia.clone(), &mut (*context).hints, &mut (*context).fork_sink);
            let value: ActiveValue = (*context).scfia.new_bv_concat(
                (*context).scfia.new_bv_concrete(0b0, 24, &mut (*context).fork_sink),
                value8.clone(),
                32,
                &mut (*context).fork_sink,
            );
            _register_write_BV32(state, rd.clone(), value.clone(), context);
            _progress_pc_4(state, context);
        } else if (*context).scfia.check_condition(
            (*context).scfia.new_bool_eq(
                funct3.clone(),
                (*context).scfia.new_bv_concrete(0b101, 3, &mut (*context).fork_sink),
                &mut (*context).fork_sink,
            ),
            &mut (*context).fork_sink,
        ) {
            let rd: ActiveValue = _extract_rd_32(instruction_32.clone(), context);
            let rs1: ActiveValue = _extract_rs1_32(instruction_32.clone(), context);
            let imm: ActiveValue = (*context).scfia.new_bv_slice(instruction_32.clone(), 31, 20, &mut (*context).fork_sink);
            let imm32: ActiveValue = (*context).scfia.new_bv_sign_extend(imm.clone(), 12, 32, &mut (*context).fork_sink);
            let base_address: ActiveValue = _register_read_BV32(state, rs1.clone(), context);
            let address: ActiveValue = (*context).scfia.new_bv_add(base_address.clone(), imm32.clone(), 32, &mut (*context).fork_sink);
            let value16: ActiveValue =
                (*(*context).memory).read(address.clone(), 16, (*context).scfia.clone(), &mut (*context).hints, &mut (*context).fork_sink);
            let value: ActiveValue = (*context).scfia.new_bv_concat(
                (*context).scfia.new_bv_concrete(0b0, 16, &mut (*context).fork_sink),
                value16.clone(),
                32,
                &mut (*context).fork_sink,
            );
            _register_write_BV32(state, rd.clone(), value.clone(), context);
            _progress_pc_4(state, context);
        } else {
            unimplemented!();
        }
    } else if (*context).scfia.check_condition(
        (*context).scfia.new_bool_eq(
            opcode.clone(),
            (*context).scfia.new_bv_concrete(0b1111, 7, &mut (*context).fork_sink),
            &mut (*context).fork_sink,
        ),
        &mut (*context).fork_sink,
    ) {
        let rd_zeroes: ActiveValue = (*context).scfia.new_bv_slice(instruction_32.clone(), 11, 7, &mut (*context).fork_sink);
        if (*context).scfia.check_condition(
            (*context).scfia.new_bool_eq(
                rd_zeroes.clone(),
                (*context).scfia.new_bv_concrete(0b0, 5, &mut (*context).fork_sink),
                &mut (*context).fork_sink,
            ),
            &mut (*context).fork_sink,
        ) {
            let funct3: ActiveValue = (*context).scfia.new_bv_slice(instruction_32.clone(), 14, 12, &mut (*context).fork_sink);
            let rs1_zeroes: ActiveValue = (*context).scfia.new_bv_slice(instruction_32.clone(), 19, 15, &mut (*context).fork_sink);
            if (*context).scfia.check_condition(
                (*context).scfia.new_bool_eq(
                    rs1_zeroes.clone(),
                    (*context).scfia.new_bv_concrete(0b0, 5, &mut (*context).fork_sink),
                    &mut (*context).fork_sink,
                ),
                &mut (*context).fork_sink,
            ) {
                if (*context).scfia.check_condition(
                    (*context).scfia.new_bool_eq(
                        funct3.clone(),
                        (*context).scfia.new_bv_concrete(0b0, 3, &mut (*context).fork_sink),
                        &mut (*context).fork_sink,
                    ),
                    &mut (*context).fork_sink,
                ) {
                    _progress_pc_4(state, context);
                } else if (*context).scfia.check_condition(
                    (*context).scfia.new_bool_eq(
                        funct3.clone(),
                        (*context).scfia.new_bv_concrete(0b1, 3, &mut (*context).fork_sink),
                        &mut (*context).fork_sink,
                    ),
                    &mut (*context).fork_sink,
                ) {
                    _progress_pc_4(state, context);
                }
            } else {
                unimplemented!();
            }
        } else {
            unimplemented!();
        }
    } else if (*context).scfia.check_condition(
        (*context).scfia.new_bool_eq(
            opcode.clone(),
            (*context).scfia.new_bv_concrete(0b10011, 7, &mut (*context).fork_sink),
            &mut (*context).fork_sink,
        ),
        &mut (*context).fork_sink,
    ) {
        let funct3: ActiveValue = (*context).scfia.new_bv_slice(instruction_32.clone(), 14, 12, &mut (*context).fork_sink);
        if (*context).scfia.check_condition(
            (*context).scfia.new_bool_eq(
                funct3.clone(),
                (*context).scfia.new_bv_concrete(0b0, 3, &mut (*context).fork_sink),
                &mut (*context).fork_sink,
            ),
            &mut (*context).fork_sink,
        ) {
            let rd: ActiveValue = _extract_rd_32(instruction_32.clone(), context);
            let rs1: ActiveValue = _extract_rs1_32(instruction_32.clone(), context);
            let offset: ActiveValue = (*context).scfia.new_bv_slice(instruction_32.clone(), 31, 20, &mut (*context).fork_sink);
            let offset_32: ActiveValue = (*context).scfia.new_bv_sign_extend(offset.clone(), 12, 32, &mut (*context).fork_sink);
            let result: ActiveValue = (*context).scfia.new_bv_add(
                offset_32.clone(),
                _register_read_BV32(state, rs1.clone(), context),
                32,
                &mut (*context).fork_sink,
            );
            _register_write_BV32(state, rd.clone(), result.clone(), context);
            _progress_pc_4(state, context);
        } else if (*context).scfia.check_condition(
            (*context).scfia.new_bool_eq(
                funct3.clone(),
                (*context).scfia.new_bv_concrete(0b1, 3, &mut (*context).fork_sink),
                &mut (*context).fork_sink,
            ),
            &mut (*context).fork_sink,
        ) {
            let funct7: ActiveValue = (*context).scfia.new_bv_slice(instruction_32.clone(), 31, 25, &mut (*context).fork_sink);
            if (*context).scfia.check_condition(
                (*context).scfia.new_bool_eq(
                    funct7.clone(),
                    (*context).scfia.new_bv_concrete(0b0, 7, &mut (*context).fork_sink),
                    &mut (*context).fork_sink,
                ),
                &mut (*context).fork_sink,
            ) {
                let rd: ActiveValue = _extract_rd_32(instruction_32.clone(), context);
                let rs1: ActiveValue = _extract_rs1_32(instruction_32.clone(), context);
                let shamt: ActiveValue = (*context).scfia.new_bv_slice(instruction_32.clone(), 24, 20, &mut (*context).fork_sink);
                let result: ActiveValue = (*context).scfia.new_bv_sll(
                    _register_read_BV32(state, rs1.clone(), context),
                    shamt.clone(),
                    32,
                    5,
                    &mut (*context).fork_sink,
                );
                _register_write_BV32(state, rd.clone(), result.clone(), context);
                _progress_pc_4(state, context);
            } else {
                unimplemented!();
            }
        } else if (*context).scfia.check_condition(
            (*context).scfia.new_bool_eq(
                funct3.clone(),
                (*context).scfia.new_bv_concrete(0b100, 3, &mut (*context).fork_sink),
                &mut (*context).fork_sink,
            ),
            &mut (*context).fork_sink,
        ) {
            let rd: ActiveValue = _extract_rd_32(instruction_32.clone(), context);
            let rs1: ActiveValue = _extract_rs1_32(instruction_32.clone(), context);
            let imm: ActiveValue = (*context).scfia.new_bv_slice(instruction_32.clone(), 31, 20, &mut (*context).fork_sink);
            let imm_32: ActiveValue = (*context).scfia.new_bv_sign_extend(imm.clone(), 12, 32, &mut (*context).fork_sink);
            let result: ActiveValue =
                (*context)
                    .scfia
                    .new_bv_xor(_register_read_BV32(state, rs1.clone(), context), imm_32.clone(), 32, &mut (*context).fork_sink);
            _register_write_BV32(state, rd.clone(), result.clone(), context);
            _progress_pc_4(state, context);
        } else if (*context).scfia.check_condition(
            (*context).scfia.new_bool_eq(
                funct3.clone(),
                (*context).scfia.new_bv_concrete(0b101, 3, &mut (*context).fork_sink),
                &mut (*context).fork_sink,
            ),
            &mut (*context).fork_sink,
        ) {
            let funct7: ActiveValue = (*context).scfia.new_bv_slice(instruction_32.clone(), 31, 25, &mut (*context).fork_sink);
            if (*context).scfia.check_condition(
                (*context).scfia.new_bool_eq(
                    funct7.clone(),
                    (*context).scfia.new_bv_concrete(0b0, 7, &mut (*context).fork_sink),
                    &mut (*context).fork_sink,
                ),
                &mut (*context).fork_sink,
            ) {
                let rd: ActiveValue = _extract_rd_32(instruction_32.clone(), context);
                let rs1: ActiveValue = _extract_rs1_32(instruction_32.clone(), context);
                let shamt: ActiveValue = (*context).scfia.new_bv_slice(instruction_32.clone(), 24, 20, &mut (*context).fork_sink);
                let result: ActiveValue = (*context).scfia.new_bv_srl(
                    _register_read_BV32(state, rs1.clone(), context),
                    shamt.clone(),
                    32,
                    5,
                    &mut (*context).fork_sink,
                );
                _register_write_BV32(state, rd.clone(), result.clone(), context);
                _progress_pc_4(state, context);
            } else {
                unimplemented!();
            }
        } else if (*context).scfia.check_condition(
            (*context).scfia.new_bool_eq(
                funct3.clone(),
                (*context).scfia.new_bv_concrete(0b110, 3, &mut (*context).fork_sink),
                &mut (*context).fork_sink,
            ),
            &mut (*context).fork_sink,
        ) {
            let rd: ActiveValue = _extract_rd_32(instruction_32.clone(), context);
            let rs1: ActiveValue = _extract_rs1_32(instruction_32.clone(), context);
            let imm: ActiveValue = (*context).scfia.new_bv_slice(instruction_32.clone(), 31, 20, &mut (*context).fork_sink);
            let imm_32: ActiveValue = (*context).scfia.new_bv_sign_extend(imm.clone(), 12, 32, &mut (*context).fork_sink);
            let result: ActiveValue =
                (*context)
                    .scfia
                    .new_bv_or(_register_read_BV32(state, rs1.clone(), context), imm_32.clone(), 32, &mut (*context).fork_sink);
            _register_write_BV32(state, rd.clone(), result.clone(), context);
            _progress_pc_4(state, context);
        } else if (*context).scfia.check_condition(
            (*context).scfia.new_bool_eq(
                funct3.clone(),
                (*context).scfia.new_bv_concrete(0b111, 3, &mut (*context).fork_sink),
                &mut (*context).fork_sink,
            ),
            &mut (*context).fork_sink,
        ) {
            let rd: ActiveValue = _extract_rd_32(instruction_32.clone(), context);
            let rs1: ActiveValue = _extract_rs1_32(instruction_32.clone(), context);
            let imm: ActiveValue = (*context).scfia.new_bv_slice(instruction_32.clone(), 31, 20, &mut (*context).fork_sink);
            let imm_32: ActiveValue = (*context).scfia.new_bv_sign_extend(imm.clone(), 12, 32, &mut (*context).fork_sink);
            let result: ActiveValue =
                (*context)
                    .scfia
                    .new_bv_add(_register_read_BV32(state, rs1.clone(), context), imm_32.clone(), 32, &mut (*context).fork_sink);
            _register_write_BV32(state, rd.clone(), result.clone(), context);
            _progress_pc_4(state, context);
        } else {
            unimplemented!();
        }
    } else if (*context).scfia.check_condition(
        (*context).scfia.new_bool_eq(
            opcode.clone(),
            (*context).scfia.new_bv_concrete(0b100011, 7, &mut (*context).fork_sink),
            &mut (*context).fork_sink,
        ),
        &mut (*context).fork_sink,
    ) {
        let funct3: ActiveValue = (*context).scfia.new_bv_slice(instruction_32.clone(), 14, 12, &mut (*context).fork_sink);
        if (*context).scfia.check_condition(
            (*context).scfia.new_bool_eq(
                funct3.clone(),
                (*context).scfia.new_bv_concrete(0b0, 3, &mut (*context).fork_sink),
                &mut (*context).fork_sink,
            ),
            &mut (*context).fork_sink,
        ) {
            let rs1: ActiveValue = _extract_rs1_32(instruction_32.clone(), context);
            let rs2: ActiveValue = _extract_rs2_32(instruction_32.clone(), context);
            let offset_11_5: ActiveValue = (*context).scfia.new_bv_slice(instruction_32.clone(), 31, 25, &mut (*context).fork_sink);
            let offset_4_0: ActiveValue = (*context).scfia.new_bv_slice(instruction_32.clone(), 11, 7, &mut (*context).fork_sink);
            let offset: ActiveValue = (*context)
                .scfia
                .new_bv_concat(offset_11_5.clone(), offset_4_0.clone(), 12, &mut (*context).fork_sink);
            let offset_32: ActiveValue = (*context).scfia.new_bv_sign_extend(offset.clone(), 12, 32, &mut (*context).fork_sink);
            let base_address: ActiveValue = _register_read_BV32(state, rs1.clone(), context);
            let address: ActiveValue = (*context)
                .scfia
                .new_bv_add(base_address.clone(), offset_32.clone(), 32, &mut (*context).fork_sink);
            let value_32: ActiveValue = _register_read_BV32(state, rs2.clone(), context);
            let value: ActiveValue = (*context).scfia.new_bv_slice(value_32.clone(), 7, 0, &mut (*context).fork_sink);
            (*(*context).memory).write(
                address.clone(),
                value.clone(),
                8,
                (*context).scfia.clone(),
                &mut (*context).hints,
                &mut (*context).fork_sink,
            );
            _progress_pc_4(state, context);
        } else if (*context).scfia.check_condition(
            (*context).scfia.new_bool_eq(
                funct3.clone(),
                (*context).scfia.new_bv_concrete(0b1, 3, &mut (*context).fork_sink),
                &mut (*context).fork_sink,
            ),
            &mut (*context).fork_sink,
        ) {
            let rs1: ActiveValue = _extract_rs1_32(instruction_32.clone(), context);
            let rs2: ActiveValue = _extract_rs2_32(instruction_32.clone(), context);
            let offset_11_5: ActiveValue = (*context).scfia.new_bv_slice(instruction_32.clone(), 31, 25, &mut (*context).fork_sink);
            let offset_4_0: ActiveValue = (*context).scfia.new_bv_slice(instruction_32.clone(), 11, 7, &mut (*context).fork_sink);
            let offset: ActiveValue = (*context)
                .scfia
                .new_bv_concat(offset_11_5.clone(), offset_4_0.clone(), 12, &mut (*context).fork_sink);
            let offset_32: ActiveValue = (*context).scfia.new_bv_sign_extend(offset.clone(), 12, 32, &mut (*context).fork_sink);
            let base_address: ActiveValue = _register_read_BV32(state, rs1.clone(), context);
            let address: ActiveValue = (*context)
                .scfia
                .new_bv_add(base_address.clone(), offset_32.clone(), 32, &mut (*context).fork_sink);
            let value_32: ActiveValue = _register_read_BV32(state, rs2.clone(), context);
            let value: ActiveValue = (*context).scfia.new_bv_slice(value_32.clone(), 15, 0, &mut (*context).fork_sink);
            (*(*context).memory).write(
                address.clone(),
                value.clone(),
                16,
                (*context).scfia.clone(),
                &mut (*context).hints,
                &mut (*context).fork_sink,
            );
            _progress_pc_4(state, context);
        } else if (*context).scfia.check_condition(
            (*context).scfia.new_bool_eq(
                funct3.clone(),
                (*context).scfia.new_bv_concrete(0b10, 3, &mut (*context).fork_sink),
                &mut (*context).fork_sink,
            ),
            &mut (*context).fork_sink,
        ) {
            let rs1: ActiveValue = _extract_rs1_32(instruction_32.clone(), context);
            let rs2: ActiveValue = _extract_rs2_32(instruction_32.clone(), context);
            let offset_11_5: ActiveValue = (*context).scfia.new_bv_slice(instruction_32.clone(), 31, 25, &mut (*context).fork_sink);
            let offset_4_0: ActiveValue = (*context).scfia.new_bv_slice(instruction_32.clone(), 11, 7, &mut (*context).fork_sink);
            let offset: ActiveValue = (*context)
                .scfia
                .new_bv_concat(offset_11_5.clone(), offset_4_0.clone(), 12, &mut (*context).fork_sink);
            let offset_32: ActiveValue = (*context).scfia.new_bv_sign_extend(offset.clone(), 12, 32, &mut (*context).fork_sink);
            let base_address: ActiveValue = _register_read_BV32(state, rs1.clone(), context);
            let address: ActiveValue = (*context)
                .scfia
                .new_bv_add(base_address.clone(), offset_32.clone(), 32, &mut (*context).fork_sink);
            let value: ActiveValue = _register_read_BV32(state, rs2.clone(), context);
            (*(*context).memory).write(
                address.clone(),
                value.clone(),
                32,
                (*context).scfia.clone(),
                &mut (*context).hints,
                &mut (*context).fork_sink,
            );
            _progress_pc_4(state, context);
        } else {
            unimplemented!();
        }
    } else if (*context).scfia.check_condition(
        (*context).scfia.new_bool_eq(
            opcode.clone(),
            (*context).scfia.new_bv_concrete(0b110111, 7, &mut (*context).fork_sink),
            &mut (*context).fork_sink,
        ),
        &mut (*context).fork_sink,
    ) {
        let rd: ActiveValue = _extract_rd_32(instruction_32.clone(), context);
        let rs: ActiveValue = _extract_rs1_32(instruction_32.clone(), context);
        let imm: ActiveValue = (*context).scfia.new_bv_slice(instruction_32.clone(), 31, 12, &mut (*context).fork_sink);
        let value: ActiveValue = (*context).scfia.new_bv_concat(
            imm.clone(),
            (*context).scfia.new_bv_concrete(0b0, 12, &mut (*context).fork_sink),
            32,
            &mut (*context).fork_sink,
        );
        _register_write_BV32(state, rd.clone(), value.clone(), context);
        _progress_pc_4(state, context);
    } else if (*context).scfia.check_condition(
        (*context).scfia.new_bool_eq(
            opcode.clone(),
            (*context).scfia.new_bv_concrete(0b10111, 7, &mut (*context).fork_sink),
            &mut (*context).fork_sink,
        ),
        &mut (*context).fork_sink,
    ) {
        let dst: ActiveValue = _extract_rd_32(instruction_32.clone(), context);
        let imm: ActiveValue = (*context).scfia.new_bv_slice(instruction_32.clone(), 31, 12, &mut (*context).fork_sink);
        let imm32: ActiveValue = (*context).scfia.new_bv_concat(
            imm.clone(),
            (*context).scfia.new_bv_concrete(0b0, 12, &mut (*context).fork_sink),
            32,
            &mut (*context).fork_sink,
        );
        let sum: ActiveValue = (*context).scfia.new_bv_add(imm32.clone(), (*state).pc.clone(), 32, &mut (*context).fork_sink);
        _register_write_BV32(state, dst.clone(), sum.clone(), context);
        _progress_pc_4(state, context);
    } else if (*context).scfia.check_condition(
        (*context).scfia.new_bool_eq(
            opcode.clone(),
            (*context).scfia.new_bv_concrete(0b110011, 7, &mut (*context).fork_sink),
            &mut (*context).fork_sink,
        ),
        &mut (*context).fork_sink,
    ) {
        let funct3: ActiveValue = (*context).scfia.new_bv_slice(instruction_32.clone(), 14, 12, &mut (*context).fork_sink);
        let funct7: ActiveValue = (*context).scfia.new_bv_slice(instruction_32.clone(), 31, 25, &mut (*context).fork_sink);
        let rd: ActiveValue = (*context).scfia.new_bv_slice(instruction_32.clone(), 11, 7, &mut (*context).fork_sink);
        let rs1: ActiveValue = (*context).scfia.new_bv_slice(instruction_32.clone(), 19, 15, &mut (*context).fork_sink);
        let rs2: ActiveValue = (*context).scfia.new_bv_slice(instruction_32.clone(), 24, 20, &mut (*context).fork_sink);
        if (*context).scfia.check_condition(
            (*context).scfia.new_bool_eq(
                funct3.clone(),
                (*context).scfia.new_bv_concrete(0b0, 3, &mut (*context).fork_sink),
                &mut (*context).fork_sink,
            ),
            &mut (*context).fork_sink,
        ) {
            let s1: ActiveValue = _register_read_BV32(state, rs1.clone(), context);
            let s2: ActiveValue = _register_read_BV32(state, rs2.clone(), context);
            if (*context).scfia.check_condition(
                (*context).scfia.new_bool_eq(
                    funct7.clone(),
                    (*context).scfia.new_bv_concrete(0b0, 7, &mut (*context).fork_sink),
                    &mut (*context).fork_sink,
                ),
                &mut (*context).fork_sink,
            ) {
                _execute_add32(state, rd.clone(), rs1.clone(), rs2.clone(), context);
                _progress_pc_4(state, context);
            } else if (*context).scfia.check_condition(
                (*context).scfia.new_bool_eq(
                    funct7.clone(),
                    (*context).scfia.new_bv_concrete(0b1, 7, &mut (*context).fork_sink),
                    &mut (*context).fork_sink,
                ),
                &mut (*context).fork_sink,
            ) {
                let result: ActiveValue = (*context).scfia.new_bv_multiply(s1.clone(), s2.clone(), 32, &mut (*context).fork_sink);
                _register_write_BV32(state, rd.clone(), result.clone(), context);
                _progress_pc_4(state, context);
            } else if (*context).scfia.check_condition(
                (*context).scfia.new_bool_eq(
                    funct7.clone(),
                    (*context).scfia.new_bv_concrete(0b100000, 7, &mut (*context).fork_sink),
                    &mut (*context).fork_sink,
                ),
                &mut (*context).fork_sink,
            ) {
                let sum: ActiveValue = (*context).scfia.new_bv_sub(s1.clone(), s2.clone(), 32, &mut (*context).fork_sink);
                _register_write_BV32(state, rd.clone(), sum.clone(), context);
                _progress_pc_4(state, context);
            } else {
                unimplemented!();
            }
        } else if (*context).scfia.check_condition(
            (*context).scfia.new_bool_eq(
                funct3.clone(),
                (*context).scfia.new_bv_concrete(0b1, 3, &mut (*context).fork_sink),
                &mut (*context).fork_sink,
            ),
            &mut (*context).fork_sink,
        ) {
            if (*context).scfia.check_condition(
                (*context).scfia.new_bool_eq(
                    funct7.clone(),
                    (*context).scfia.new_bv_concrete(0b0, 7, &mut (*context).fork_sink),
                    &mut (*context).fork_sink,
                ),
                &mut (*context).fork_sink,
            ) {
                let shamt: ActiveValue = (*context)
                    .scfia
                    .new_bv_slice(_register_read_BV32(state, rs2.clone(), context), 4, 0, &mut (*context).fork_sink);
                let result: ActiveValue = (*context).scfia.new_bv_sll(
                    _register_read_BV32(state, rs1.clone(), context),
                    shamt.clone(),
                    32,
                    5,
                    &mut (*context).fork_sink,
                );
                _register_write_BV32(state, rd.clone(), result.clone(), context);
                _progress_pc_4(state, context);
            } else {
                unimplemented!();
            }
        } else if (*context).scfia.check_condition(
            (*context).scfia.new_bool_eq(
                funct3.clone(),
                (*context).scfia.new_bv_concrete(0b10, 3, &mut (*context).fork_sink),
                &mut (*context).fork_sink,
            ),
            &mut (*context).fork_sink,
        ) {
            if (*context).scfia.check_condition(
                (*context).scfia.new_bool_eq(
                    funct7.clone(),
                    (*context).scfia.new_bv_concrete(0b0, 7, &mut (*context).fork_sink),
                    &mut (*context).fork_sink,
                ),
                &mut (*context).fork_sink,
            ) {
                unimplemented!();
            } else {
                unimplemented!();
            }
        } else if (*context).scfia.check_condition(
            (*context).scfia.new_bool_eq(
                funct3.clone(),
                (*context).scfia.new_bv_concrete(0b11, 3, &mut (*context).fork_sink),
                &mut (*context).fork_sink,
            ),
            &mut (*context).fork_sink,
        ) {
            if (*context).scfia.check_condition(
                (*context).scfia.new_bool_eq(
                    funct7.clone(),
                    (*context).scfia.new_bv_concrete(0b0, 7, &mut (*context).fork_sink),
                    &mut (*context).fork_sink,
                ),
                &mut (*context).fork_sink,
            ) {
                if (*context).scfia.check_condition(
                    (*context).scfia.new_bool_unsigned_less_than(
                        _register_read_BV32(state, rs1.clone(), context),
                        _register_read_BV32(state, rs2.clone(), context),
                        &mut (*context).fork_sink,
                    ),
                    &mut (*context).fork_sink,
                ) {
                    _register_write_BV32(state, rd.clone(), (*context).scfia.new_bv_concrete(0b1, 32, &mut (*context).fork_sink), context);
                } else {
                    _register_write_BV32(state, rd.clone(), (*context).scfia.new_bv_concrete(0b0, 32, &mut (*context).fork_sink), context);
                }
                _progress_pc_4(state, context);
            } else {
                unimplemented!();
            }
        } else if (*context).scfia.check_condition(
            (*context).scfia.new_bool_eq(
                funct3.clone(),
                (*context).scfia.new_bv_concrete(0b100, 3, &mut (*context).fork_sink),
                &mut (*context).fork_sink,
            ),
            &mut (*context).fork_sink,
        ) {
            if (*context).scfia.check_condition(
                (*context).scfia.new_bool_eq(
                    funct7.clone(),
                    (*context).scfia.new_bv_concrete(0b0, 7, &mut (*context).fork_sink),
                    &mut (*context).fork_sink,
                ),
                &mut (*context).fork_sink,
            ) {
                unimplemented!();
            } else {
                unimplemented!();
            }
        } else if (*context).scfia.check_condition(
            (*context).scfia.new_bool_eq(
                funct3.clone(),
                (*context).scfia.new_bv_concrete(0b101, 3, &mut (*context).fork_sink),
                &mut (*context).fork_sink,
            ),
            &mut (*context).fork_sink,
        ) {
            if (*context).scfia.check_condition(
                (*context).scfia.new_bool_eq(
                    funct7.clone(),
                    (*context).scfia.new_bv_concrete(0b0, 7, &mut (*context).fork_sink),
                    &mut (*context).fork_sink,
                ),
                &mut (*context).fork_sink,
            ) {
                let shamt: ActiveValue = (*context)
                    .scfia
                    .new_bv_slice(_register_read_BV32(state, rs2.clone(), context), 4, 0, &mut (*context).fork_sink);
                let result: ActiveValue = (*context).scfia.new_bv_srl(
                    _register_read_BV32(state, rs1.clone(), context),
                    shamt.clone(),
                    32,
                    5,
                    &mut (*context).fork_sink,
                );
                _register_write_BV32(state, rd.clone(), result.clone(), context);
                _progress_pc_4(state, context);
            } else if (*context).scfia.check_condition(
                (*context).scfia.new_bool_eq(
                    funct7.clone(),
                    (*context).scfia.new_bv_concrete(0b100000, 7, &mut (*context).fork_sink),
                    &mut (*context).fork_sink,
                ),
                &mut (*context).fork_sink,
            ) {
                unimplemented!();
            } else {
                unimplemented!();
            }
        } else if (*context).scfia.check_condition(
            (*context).scfia.new_bool_eq(
                funct3.clone(),
                (*context).scfia.new_bv_concrete(0b110, 3, &mut (*context).fork_sink),
                &mut (*context).fork_sink,
            ),
            &mut (*context).fork_sink,
        ) {
            if (*context).scfia.check_condition(
                (*context).scfia.new_bool_eq(
                    funct7.clone(),
                    (*context).scfia.new_bv_concrete(0b0, 7, &mut (*context).fork_sink),
                    &mut (*context).fork_sink,
                ),
                &mut (*context).fork_sink,
            ) {
                let rd: ActiveValue = _extract_rd_32(instruction_32.clone(), context);
                let rs1: ActiveValue = _extract_rs1_32(instruction_32.clone(), context);
                let rs2: ActiveValue = _extract_rs2_32(instruction_32.clone(), context);
                let result: ActiveValue = (*context).scfia.new_bv_or(
                    _register_read_BV32(state, rs1.clone(), context),
                    _register_read_BV32(state, rs2.clone(), context),
                    32,
                    &mut (*context).fork_sink,
                );
                _register_write_BV32(state, rd.clone(), result.clone(), context);
                _progress_pc_4(state, context);
            } else {
                unimplemented!();
            }
        } else if (*context).scfia.check_condition(
            (*context).scfia.new_bool_eq(
                funct3.clone(),
                (*context).scfia.new_bv_concrete(0b111, 3, &mut (*context).fork_sink),
                &mut (*context).fork_sink,
            ),
            &mut (*context).fork_sink,
        ) {
            let rd: ActiveValue = _extract_rd_32(instruction_32.clone(), context);
            let rs1: ActiveValue = _extract_rs1_32(instruction_32.clone(), context);
            let rs2: ActiveValue = _extract_rs2_32(instruction_32.clone(), context);
            if (*context).scfia.check_condition(
                (*context).scfia.new_bool_eq(
                    funct7.clone(),
                    (*context).scfia.new_bv_concrete(0b0, 7, &mut (*context).fork_sink),
                    &mut (*context).fork_sink,
                ),
                &mut (*context).fork_sink,
            ) {
                let result: ActiveValue = (*context).scfia.new_bv_add(
                    _register_read_BV32(state, rs1.clone(), context),
                    _register_read_BV32(state, rs2.clone(), context),
                    32,
                    &mut (*context).fork_sink,
                );
                _register_write_BV32(state, rd.clone(), result.clone(), context);
                _progress_pc_4(state, context);
            } else if (*context).scfia.check_condition(
                (*context).scfia.new_bool_eq(
                    funct7.clone(),
                    (*context).scfia.new_bv_concrete(0b1, 7, &mut (*context).fork_sink),
                    &mut (*context).fork_sink,
                ),
                &mut (*context).fork_sink,
            ) {
                let result: ActiveValue = (*context).scfia.new_bv_unsigned_remainder(
                    _register_read_BV32(state, rs1.clone(), context),
                    _register_read_BV32(state, rs2.clone(), context),
                    32,
                    &mut (*context).fork_sink,
                );
                _register_write_BV32(state, rd.clone(), result.clone(), context);
                _progress_pc_4(state, context);
            } else {
                unimplemented!();
            }
        } else {
            unimplemented!();
        }
    } else if (*context).scfia.check_condition(
        (*context).scfia.new_bool_eq(
            opcode.clone(),
            (*context).scfia.new_bv_concrete(0b1100011, 7, &mut (*context).fork_sink),
            &mut (*context).fork_sink,
        ),
        &mut (*context).fork_sink,
    ) {
        let funct3: ActiveValue = (*context).scfia.new_bv_slice(instruction_32.clone(), 14, 12, &mut (*context).fork_sink);
        if (*context).scfia.check_condition(
            (*context).scfia.new_bool_eq(
                funct3.clone(),
                (*context).scfia.new_bv_concrete(0b0, 3, &mut (*context).fork_sink),
                &mut (*context).fork_sink,
            ),
            &mut (*context).fork_sink,
        ) {
            let lhs: ActiveValue = _register_read_BV32(state, _extract_rs1_32(instruction_32.clone(), context), context);
            let rhs: ActiveValue = _register_read_BV32(state, _extract_rs2_32(instruction_32.clone(), context), context);
            if (*context).scfia.check_condition(
                (*context).scfia.new_bool_eq(lhs.clone(), rhs.clone(), &mut (*context).fork_sink),
                &mut (*context).fork_sink,
            ) {
                let imm_4_1: ActiveValue = (*context).scfia.new_bv_slice(instruction_32.clone(), 11, 8, &mut (*context).fork_sink);
                let imm_10_5: ActiveValue = (*context).scfia.new_bv_slice(instruction_32.clone(), 30, 25, &mut (*context).fork_sink);
                let imm11: ActiveValue = (*context).scfia.new_bv_slice(instruction_32.clone(), 7, 7, &mut (*context).fork_sink);
                let imm12: ActiveValue = (*context).scfia.new_bv_slice(instruction_32.clone(), 31, 31, &mut (*context).fork_sink);
                let imm_4_0: ActiveValue = (*context).scfia.new_bv_concat(
                    imm_4_1.clone(),
                    (*context).scfia.new_bv_concrete(0b0, 1, &mut (*context).fork_sink),
                    5,
                    &mut (*context).fork_sink,
                );
                let imm_10_0: ActiveValue = (*context).scfia.new_bv_concat(imm_10_5.clone(), imm_4_0.clone(), 11, &mut (*context).fork_sink);
                let imm_11_0: ActiveValue = (*context).scfia.new_bv_concat(imm11.clone(), imm_10_0.clone(), 12, &mut (*context).fork_sink);
                let imm_12_0: ActiveValue = (*context).scfia.new_bv_concat(imm12.clone(), imm_11_0.clone(), 13, &mut (*context).fork_sink);
                let offset: ActiveValue = (*context).scfia.new_bv_sign_extend(imm_12_0.clone(), 13, 32, &mut (*context).fork_sink);
                let address: ActiveValue = (*context).scfia.new_bv_add((*state).pc.clone(), offset.clone(), 32, &mut (*context).fork_sink);
                (*state).pc = address.clone();
            } else {
                _progress_pc_4(state, context);
            }
        } else if (*context).scfia.check_condition(
            (*context).scfia.new_bool_eq(
                funct3.clone(),
                (*context).scfia.new_bv_concrete(0b1, 3, &mut (*context).fork_sink),
                &mut (*context).fork_sink,
            ),
            &mut (*context).fork_sink,
        ) {
            let lhs: ActiveValue = _register_read_BV32(state, _extract_rs1_32(instruction_32.clone(), context), context);
            let rhs: ActiveValue = _register_read_BV32(state, _extract_rs2_32(instruction_32.clone(), context), context);
            if (*context).scfia.check_condition(
                (*context).scfia.new_bool_not(
                    (*context).scfia.new_bool_eq(lhs.clone(), rhs.clone(), &mut (*context).fork_sink),
                    &mut (*context).fork_sink,
                ),
                &mut (*context).fork_sink,
            ) {
                let imm_4_1: ActiveValue = (*context).scfia.new_bv_slice(instruction_32.clone(), 11, 8, &mut (*context).fork_sink);
                let imm_10_5: ActiveValue = (*context).scfia.new_bv_slice(instruction_32.clone(), 30, 25, &mut (*context).fork_sink);
                let imm11: ActiveValue = (*context).scfia.new_bv_slice(instruction_32.clone(), 7, 7, &mut (*context).fork_sink);
                let imm12: ActiveValue = (*context).scfia.new_bv_slice(instruction_32.clone(), 31, 31, &mut (*context).fork_sink);
                let imm_4_0: ActiveValue = (*context).scfia.new_bv_concat(
                    imm_4_1.clone(),
                    (*context).scfia.new_bv_concrete(0b0, 1, &mut (*context).fork_sink),
                    5,
                    &mut (*context).fork_sink,
                );
                let imm_10_0: ActiveValue = (*context).scfia.new_bv_concat(imm_10_5.clone(), imm_4_0.clone(), 11, &mut (*context).fork_sink);
                let imm_11_0: ActiveValue = (*context).scfia.new_bv_concat(imm11.clone(), imm_10_0.clone(), 12, &mut (*context).fork_sink);
                let imm_12_0: ActiveValue = (*context).scfia.new_bv_concat(imm12.clone(), imm_11_0.clone(), 13, &mut (*context).fork_sink);
                let offset: ActiveValue = (*context).scfia.new_bv_sign_extend(imm_12_0.clone(), 13, 32, &mut (*context).fork_sink);
                let address: ActiveValue = (*context).scfia.new_bv_add((*state).pc.clone(), offset.clone(), 32, &mut (*context).fork_sink);
                (*state).pc = address.clone();
            } else {
                _progress_pc_4(state, context);
            }
        } else if (*context).scfia.check_condition(
            (*context).scfia.new_bool_eq(
                funct3.clone(),
                (*context).scfia.new_bv_concrete(0b100, 3, &mut (*context).fork_sink),
                &mut (*context).fork_sink,
            ),
            &mut (*context).fork_sink,
        ) {
            let lhs: ActiveValue = _register_read_BV32(state, _extract_rs1_32(instruction_32.clone(), context), context);
            let rhs: ActiveValue = _register_read_BV32(state, _extract_rs2_32(instruction_32.clone(), context), context);
            if (*context).scfia.check_condition(
                (*context).scfia.new_bool_signed_less_than(lhs.clone(), rhs.clone(), &mut (*context).fork_sink),
                &mut (*context).fork_sink,
            ) {
                let imm_4_1: ActiveValue = (*context).scfia.new_bv_slice(instruction_32.clone(), 11, 8, &mut (*context).fork_sink);
                let imm_10_5: ActiveValue = (*context).scfia.new_bv_slice(instruction_32.clone(), 30, 25, &mut (*context).fork_sink);
                let imm11: ActiveValue = (*context).scfia.new_bv_slice(instruction_32.clone(), 7, 7, &mut (*context).fork_sink);
                let imm12: ActiveValue = (*context).scfia.new_bv_slice(instruction_32.clone(), 31, 31, &mut (*context).fork_sink);
                let imm_4_0: ActiveValue = (*context).scfia.new_bv_concat(
                    imm_4_1.clone(),
                    (*context).scfia.new_bv_concrete(0b0, 1, &mut (*context).fork_sink),
                    5,
                    &mut (*context).fork_sink,
                );
                let imm_10_0: ActiveValue = (*context).scfia.new_bv_concat(imm_10_5.clone(), imm_4_0.clone(), 11, &mut (*context).fork_sink);
                let imm_11_0: ActiveValue = (*context).scfia.new_bv_concat(imm11.clone(), imm_10_0.clone(), 12, &mut (*context).fork_sink);
                let imm_12_0: ActiveValue = (*context).scfia.new_bv_concat(imm12.clone(), imm_11_0.clone(), 13, &mut (*context).fork_sink);
                let offset: ActiveValue = (*context).scfia.new_bv_sign_extend(imm_12_0.clone(), 13, 32, &mut (*context).fork_sink);
                let address: ActiveValue = (*context).scfia.new_bv_add((*state).pc.clone(), offset.clone(), 32, &mut (*context).fork_sink);
                (*state).pc = address.clone();
            } else {
                _progress_pc_4(state, context);
            }
        } else if (*context).scfia.check_condition(
            (*context).scfia.new_bool_eq(
                funct3.clone(),
                (*context).scfia.new_bv_concrete(0b101, 3, &mut (*context).fork_sink),
                &mut (*context).fork_sink,
            ),
            &mut (*context).fork_sink,
        ) {
            let lhs: ActiveValue = _register_read_BV32(state, _extract_rs1_32(instruction_32.clone(), context), context);
            let rhs: ActiveValue = _register_read_BV32(state, _extract_rs2_32(instruction_32.clone(), context), context);
            if (*context).scfia.check_condition(
                (*context).scfia.new_bool_signed_less_than(lhs.clone(), rhs.clone(), &mut (*context).fork_sink),
                &mut (*context).fork_sink,
            ) {
                _progress_pc_4(state, context);
            } else {
                let imm_4_1: ActiveValue = (*context).scfia.new_bv_slice(instruction_32.clone(), 11, 8, &mut (*context).fork_sink);
                let imm_10_5: ActiveValue = (*context).scfia.new_bv_slice(instruction_32.clone(), 30, 25, &mut (*context).fork_sink);
                let imm11: ActiveValue = (*context).scfia.new_bv_slice(instruction_32.clone(), 7, 7, &mut (*context).fork_sink);
                let imm12: ActiveValue = (*context).scfia.new_bv_slice(instruction_32.clone(), 31, 31, &mut (*context).fork_sink);
                let imm_4_0: ActiveValue = (*context).scfia.new_bv_concat(
                    imm_4_1.clone(),
                    (*context).scfia.new_bv_concrete(0b0, 1, &mut (*context).fork_sink),
                    5,
                    &mut (*context).fork_sink,
                );
                let imm_10_0: ActiveValue = (*context).scfia.new_bv_concat(imm_10_5.clone(), imm_4_0.clone(), 11, &mut (*context).fork_sink);
                let imm_11_0: ActiveValue = (*context).scfia.new_bv_concat(imm11.clone(), imm_10_0.clone(), 12, &mut (*context).fork_sink);
                let imm_12_0: ActiveValue = (*context).scfia.new_bv_concat(imm12.clone(), imm_11_0.clone(), 13, &mut (*context).fork_sink);
                let offset: ActiveValue = (*context).scfia.new_bv_sign_extend(imm_12_0.clone(), 13, 32, &mut (*context).fork_sink);
                let address: ActiveValue = (*context).scfia.new_bv_add((*state).pc.clone(), offset.clone(), 32, &mut (*context).fork_sink);
                (*state).pc = address.clone();
            }
        } else if (*context).scfia.check_condition(
            (*context).scfia.new_bool_eq(
                funct3.clone(),
                (*context).scfia.new_bv_concrete(0b110, 3, &mut (*context).fork_sink),
                &mut (*context).fork_sink,
            ),
            &mut (*context).fork_sink,
        ) {
            let lhs: ActiveValue = _register_read_BV32(state, _extract_rs1_32(instruction_32.clone(), context), context);
            let rhs: ActiveValue = _register_read_BV32(state, _extract_rs2_32(instruction_32.clone(), context), context);
            if (*context).scfia.check_condition(
                (*context)
                    .scfia
                    .new_bool_unsigned_less_than(lhs.clone(), rhs.clone(), &mut (*context).fork_sink),
                &mut (*context).fork_sink,
            ) {
                let imm_4_1: ActiveValue = (*context).scfia.new_bv_slice(instruction_32.clone(), 11, 8, &mut (*context).fork_sink);
                let imm_10_5: ActiveValue = (*context).scfia.new_bv_slice(instruction_32.clone(), 30, 25, &mut (*context).fork_sink);
                let imm11: ActiveValue = (*context).scfia.new_bv_slice(instruction_32.clone(), 7, 7, &mut (*context).fork_sink);
                let imm12: ActiveValue = (*context).scfia.new_bv_slice(instruction_32.clone(), 31, 31, &mut (*context).fork_sink);
                let imm_4_0: ActiveValue = (*context).scfia.new_bv_concat(
                    imm_4_1.clone(),
                    (*context).scfia.new_bv_concrete(0b0, 1, &mut (*context).fork_sink),
                    5,
                    &mut (*context).fork_sink,
                );
                let imm_10_0: ActiveValue = (*context).scfia.new_bv_concat(imm_10_5.clone(), imm_4_0.clone(), 11, &mut (*context).fork_sink);
                let imm_11_0: ActiveValue = (*context).scfia.new_bv_concat(imm11.clone(), imm_10_0.clone(), 12, &mut (*context).fork_sink);
                let imm_12_0: ActiveValue = (*context).scfia.new_bv_concat(imm12.clone(), imm_11_0.clone(), 13, &mut (*context).fork_sink);
                let offset: ActiveValue = (*context).scfia.new_bv_sign_extend(imm_12_0.clone(), 13, 32, &mut (*context).fork_sink);
                let address: ActiveValue = (*context).scfia.new_bv_add((*state).pc.clone(), offset.clone(), 32, &mut (*context).fork_sink);
                (*state).pc = address.clone();
            } else {
                _progress_pc_4(state, context);
            }
        } else if (*context).scfia.check_condition(
            (*context).scfia.new_bool_eq(
                funct3.clone(),
                (*context).scfia.new_bv_concrete(0b111, 3, &mut (*context).fork_sink),
                &mut (*context).fork_sink,
            ),
            &mut (*context).fork_sink,
        ) {
            let lhs: ActiveValue = _register_read_BV32(state, _extract_rs1_32(instruction_32.clone(), context), context);
            let rhs: ActiveValue = _register_read_BV32(state, _extract_rs2_32(instruction_32.clone(), context), context);
            if (*context).scfia.check_condition(
                (*context)
                    .scfia
                    .new_bool_unsigned_less_than(lhs.clone(), rhs.clone(), &mut (*context).fork_sink),
                &mut (*context).fork_sink,
            ) {
                _progress_pc_4(state, context);
            } else {
                let imm_4_1: ActiveValue = (*context).scfia.new_bv_slice(instruction_32.clone(), 11, 8, &mut (*context).fork_sink);
                let imm_10_5: ActiveValue = (*context).scfia.new_bv_slice(instruction_32.clone(), 30, 25, &mut (*context).fork_sink);
                let imm11: ActiveValue = (*context).scfia.new_bv_slice(instruction_32.clone(), 7, 7, &mut (*context).fork_sink);
                let imm12: ActiveValue = (*context).scfia.new_bv_slice(instruction_32.clone(), 31, 31, &mut (*context).fork_sink);
                let imm_4_0: ActiveValue = (*context).scfia.new_bv_concat(
                    imm_4_1.clone(),
                    (*context).scfia.new_bv_concrete(0b0, 1, &mut (*context).fork_sink),
                    5,
                    &mut (*context).fork_sink,
                );
                let imm_10_0: ActiveValue = (*context).scfia.new_bv_concat(imm_10_5.clone(), imm_4_0.clone(), 11, &mut (*context).fork_sink);
                let imm_11_0: ActiveValue = (*context).scfia.new_bv_concat(imm11.clone(), imm_10_0.clone(), 12, &mut (*context).fork_sink);
                let imm_12_0: ActiveValue = (*context).scfia.new_bv_concat(imm12.clone(), imm_11_0.clone(), 13, &mut (*context).fork_sink);
                let offset: ActiveValue = (*context).scfia.new_bv_sign_extend(imm_12_0.clone(), 13, 32, &mut (*context).fork_sink);
                let address: ActiveValue = (*context).scfia.new_bv_add((*state).pc.clone(), offset.clone(), 32, &mut (*context).fork_sink);
                (*state).pc = address.clone();
            }
        } else {
            unimplemented!();
        }
    } else if (*context).scfia.check_condition(
        (*context).scfia.new_bool_eq(
            opcode.clone(),
            (*context).scfia.new_bv_concrete(0b1100111, 7, &mut (*context).fork_sink),
            &mut (*context).fork_sink,
        ),
        &mut (*context).fork_sink,
    ) {
        let funct3: ActiveValue = (*context).scfia.new_bv_slice(instruction_32.clone(), 14, 12, &mut (*context).fork_sink);
        if (*context).scfia.check_condition(
            (*context).scfia.new_bool_eq(
                funct3.clone(),
                (*context).scfia.new_bv_concrete(0b0, 3, &mut (*context).fork_sink),
                &mut (*context).fork_sink,
            ),
            &mut (*context).fork_sink,
        ) {
            let dst: ActiveValue = _extract_rd_32(instruction_32.clone(), context);
            let rs1: ActiveValue = _extract_rs1_32(instruction_32.clone(), context);
            let s1: ActiveValue = _register_read_BV32(state, rs1.clone(), context);
            let offset: ActiveValue = (*context).scfia.new_bv_slice(instruction_32.clone(), 31, 20, &mut (*context).fork_sink);
            let offset_32: ActiveValue = (*context).scfia.new_bv_sign_extend(offset.clone(), 12, 32, &mut (*context).fork_sink);
            let address: ActiveValue = (*context).scfia.new_bv_add(s1.clone(), offset_32.clone(), 32, &mut (*context).fork_sink);
            let return_address: ActiveValue = (*context).scfia.new_bv_add(
                (*state).pc.clone(),
                (*context).scfia.new_bv_concrete(0b100, 32, &mut (*context).fork_sink),
                32,
                &mut (*context).fork_sink,
            );
            (*state).pc = address.clone();
            _register_write_BV32(state, dst.clone(), return_address.clone(), context);
        } else {
            unimplemented!();
        }
    } else if (*context).scfia.check_condition(
        (*context).scfia.new_bool_eq(
            opcode.clone(),
            (*context).scfia.new_bv_concrete(0b1101111, 7, &mut (*context).fork_sink),
            &mut (*context).fork_sink,
        ),
        &mut (*context).fork_sink,
    ) {
        let imm_20: ActiveValue = (*context).scfia.new_bv_slice(instruction_32.clone(), 31, 31, &mut (*context).fork_sink);
        let imm_10_1: ActiveValue = (*context).scfia.new_bv_slice(instruction_32.clone(), 30, 21, &mut (*context).fork_sink);
        let imm_11: ActiveValue = (*context).scfia.new_bv_slice(instruction_32.clone(), 20, 20, &mut (*context).fork_sink);
        let imm_19_12: ActiveValue = (*context).scfia.new_bv_slice(instruction_32.clone(), 19, 12, &mut (*context).fork_sink);
        let offset_10_0: ActiveValue = (*context).scfia.new_bv_concat(
            imm_10_1.clone(),
            (*context).scfia.new_bv_concrete(0b0, 1, &mut (*context).fork_sink),
            11,
            &mut (*context).fork_sink,
        );
        let offset_11_0: ActiveValue = (*context)
            .scfia
            .new_bv_concat(imm_11.clone(), offset_10_0.clone(), 12, &mut (*context).fork_sink);
        let offset_19_0: ActiveValue = (*context)
            .scfia
            .new_bv_concat(imm_19_12.clone(), offset_11_0.clone(), 20, &mut (*context).fork_sink);
        let offset_20_0: ActiveValue = (*context)
            .scfia
            .new_bv_concat(imm_20.clone(), offset_19_0.clone(), 21, &mut (*context).fork_sink);
        let offset_32: ActiveValue = (*context).scfia.new_bv_sign_extend(offset_20_0.clone(), 21, 32, &mut (*context).fork_sink);
        let address: ActiveValue = (*context)
            .scfia
            .new_bv_add((*state).pc.clone(), offset_32.clone(), 32, &mut (*context).fork_sink);
        let return_address: ActiveValue = (*context).scfia.new_bv_add(
            (*state).pc.clone(),
            (*context).scfia.new_bv_concrete(0b100, 32, &mut (*context).fork_sink),
            32,
            &mut (*context).fork_sink,
        );
        let dst: ActiveValue = _extract_rd_32(instruction_32.clone(), context);
        _register_write_BV32(state, dst.clone(), return_address.clone(), context);
        (*state).pc = address.clone();
    } else {
        unimplemented!();
    }
}

unsafe fn _extract_rd_32(op: ActiveValue, context: *mut StepContext) -> ActiveValue {
    return (*context).scfia.new_bv_slice(op.clone(), 11, 7, &mut (*context).fork_sink);
}

unsafe fn _extract_rs1_32(op: ActiveValue, context: *mut StepContext) -> ActiveValue {
    return (*context).scfia.new_bv_slice(op.clone(), 19, 15, &mut (*context).fork_sink);
}

unsafe fn _extract_rs2_32(op: ActiveValue, context: *mut StepContext) -> ActiveValue {
    return (*context).scfia.new_bv_slice(op.clone(), 24, 20, &mut (*context).fork_sink);
}

unsafe fn _progress_pc_4(state: *mut SystemState, context: *mut StepContext) {
    let old_pc: ActiveValue = (*state).pc.clone();
    let new_pc: ActiveValue = (*context).scfia.new_bv_add(
        old_pc.clone(),
        (*context).scfia.new_bv_concrete(0b100, 32, &mut (*context).fork_sink),
        32,
        &mut (*context).fork_sink,
    );
    (*state).pc = new_pc.clone();
}

unsafe fn _register_write_BV32(state: *mut SystemState, register_id: ActiveValue, value: ActiveValue, context: *mut StepContext) {
    if (*context).scfia.check_condition(
        (*context).scfia.new_bool_eq(
            register_id.clone(),
            (*context).scfia.new_bv_concrete(0b0, 5, &mut (*context).fork_sink),
            &mut (*context).fork_sink,
        ),
        &mut (*context).fork_sink,
    ) {
    } else if (*context).scfia.check_condition(
        (*context).scfia.new_bool_eq(
            register_id.clone(),
            (*context).scfia.new_bv_concrete(0b1, 5, &mut (*context).fork_sink),
            &mut (*context).fork_sink,
        ),
        &mut (*context).fork_sink,
    ) {
        (*state).x1 = value.clone();
    } else if (*context).scfia.check_condition(
        (*context).scfia.new_bool_eq(
            register_id.clone(),
            (*context).scfia.new_bv_concrete(0b10, 5, &mut (*context).fork_sink),
            &mut (*context).fork_sink,
        ),
        &mut (*context).fork_sink,
    ) {
        (*state).x2 = value.clone();
    } else if (*context).scfia.check_condition(
        (*context).scfia.new_bool_eq(
            register_id.clone(),
            (*context).scfia.new_bv_concrete(0b11, 5, &mut (*context).fork_sink),
            &mut (*context).fork_sink,
        ),
        &mut (*context).fork_sink,
    ) {
        (*state).x3 = value.clone();
    } else if (*context).scfia.check_condition(
        (*context).scfia.new_bool_eq(
            register_id.clone(),
            (*context).scfia.new_bv_concrete(0b100, 5, &mut (*context).fork_sink),
            &mut (*context).fork_sink,
        ),
        &mut (*context).fork_sink,
    ) {
        (*state).x4 = value.clone();
    } else if (*context).scfia.check_condition(
        (*context).scfia.new_bool_eq(
            register_id.clone(),
            (*context).scfia.new_bv_concrete(0b101, 5, &mut (*context).fork_sink),
            &mut (*context).fork_sink,
        ),
        &mut (*context).fork_sink,
    ) {
        (*state).x5 = value.clone();
    } else if (*context).scfia.check_condition(
        (*context).scfia.new_bool_eq(
            register_id.clone(),
            (*context).scfia.new_bv_concrete(0b110, 5, &mut (*context).fork_sink),
            &mut (*context).fork_sink,
        ),
        &mut (*context).fork_sink,
    ) {
        (*state).x6 = value.clone();
    } else if (*context).scfia.check_condition(
        (*context).scfia.new_bool_eq(
            register_id.clone(),
            (*context).scfia.new_bv_concrete(0b111, 5, &mut (*context).fork_sink),
            &mut (*context).fork_sink,
        ),
        &mut (*context).fork_sink,
    ) {
        (*state).x7 = value.clone();
    } else if (*context).scfia.check_condition(
        (*context).scfia.new_bool_eq(
            register_id.clone(),
            (*context).scfia.new_bv_concrete(0b1000, 5, &mut (*context).fork_sink),
            &mut (*context).fork_sink,
        ),
        &mut (*context).fork_sink,
    ) {
        (*state).x8 = value.clone();
    } else if (*context).scfia.check_condition(
        (*context).scfia.new_bool_eq(
            register_id.clone(),
            (*context).scfia.new_bv_concrete(0b1001, 5, &mut (*context).fork_sink),
            &mut (*context).fork_sink,
        ),
        &mut (*context).fork_sink,
    ) {
        (*state).x9 = value.clone();
    } else if (*context).scfia.check_condition(
        (*context).scfia.new_bool_eq(
            register_id.clone(),
            (*context).scfia.new_bv_concrete(0b1010, 5, &mut (*context).fork_sink),
            &mut (*context).fork_sink,
        ),
        &mut (*context).fork_sink,
    ) {
        (*state).x10 = value.clone();
    } else if (*context).scfia.check_condition(
        (*context).scfia.new_bool_eq(
            register_id.clone(),
            (*context).scfia.new_bv_concrete(0b1011, 5, &mut (*context).fork_sink),
            &mut (*context).fork_sink,
        ),
        &mut (*context).fork_sink,
    ) {
        (*state).x11 = value.clone();
    } else if (*context).scfia.check_condition(
        (*context).scfia.new_bool_eq(
            register_id.clone(),
            (*context).scfia.new_bv_concrete(0b1100, 5, &mut (*context).fork_sink),
            &mut (*context).fork_sink,
        ),
        &mut (*context).fork_sink,
    ) {
        (*state).x12 = value.clone();
    } else if (*context).scfia.check_condition(
        (*context).scfia.new_bool_eq(
            register_id.clone(),
            (*context).scfia.new_bv_concrete(0b1101, 5, &mut (*context).fork_sink),
            &mut (*context).fork_sink,
        ),
        &mut (*context).fork_sink,
    ) {
        (*state).x13 = value.clone();
    } else if (*context).scfia.check_condition(
        (*context).scfia.new_bool_eq(
            register_id.clone(),
            (*context).scfia.new_bv_concrete(0b1110, 5, &mut (*context).fork_sink),
            &mut (*context).fork_sink,
        ),
        &mut (*context).fork_sink,
    ) {
        (*state).x14 = value.clone();
    } else if (*context).scfia.check_condition(
        (*context).scfia.new_bool_eq(
            register_id.clone(),
            (*context).scfia.new_bv_concrete(0b1111, 5, &mut (*context).fork_sink),
            &mut (*context).fork_sink,
        ),
        &mut (*context).fork_sink,
    ) {
        (*state).x15 = value.clone();
    } else if (*context).scfia.check_condition(
        (*context).scfia.new_bool_eq(
            register_id.clone(),
            (*context).scfia.new_bv_concrete(0b10000, 5, &mut (*context).fork_sink),
            &mut (*context).fork_sink,
        ),
        &mut (*context).fork_sink,
    ) {
        (*state).x16 = value.clone();
    } else if (*context).scfia.check_condition(
        (*context).scfia.new_bool_eq(
            register_id.clone(),
            (*context).scfia.new_bv_concrete(0b10001, 5, &mut (*context).fork_sink),
            &mut (*context).fork_sink,
        ),
        &mut (*context).fork_sink,
    ) {
        (*state).x17 = value.clone();
    } else if (*context).scfia.check_condition(
        (*context).scfia.new_bool_eq(
            register_id.clone(),
            (*context).scfia.new_bv_concrete(0b10010, 5, &mut (*context).fork_sink),
            &mut (*context).fork_sink,
        ),
        &mut (*context).fork_sink,
    ) {
        (*state).x18 = value.clone();
    } else if (*context).scfia.check_condition(
        (*context).scfia.new_bool_eq(
            register_id.clone(),
            (*context).scfia.new_bv_concrete(0b10011, 5, &mut (*context).fork_sink),
            &mut (*context).fork_sink,
        ),
        &mut (*context).fork_sink,
    ) {
        (*state).x19 = value.clone();
    } else if (*context).scfia.check_condition(
        (*context).scfia.new_bool_eq(
            register_id.clone(),
            (*context).scfia.new_bv_concrete(0b10100, 5, &mut (*context).fork_sink),
            &mut (*context).fork_sink,
        ),
        &mut (*context).fork_sink,
    ) {
        (*state).x20 = value.clone();
    } else if (*context).scfia.check_condition(
        (*context).scfia.new_bool_eq(
            register_id.clone(),
            (*context).scfia.new_bv_concrete(0b10101, 5, &mut (*context).fork_sink),
            &mut (*context).fork_sink,
        ),
        &mut (*context).fork_sink,
    ) {
        (*state).x21 = value.clone();
    } else if (*context).scfia.check_condition(
        (*context).scfia.new_bool_eq(
            register_id.clone(),
            (*context).scfia.new_bv_concrete(0b10110, 5, &mut (*context).fork_sink),
            &mut (*context).fork_sink,
        ),
        &mut (*context).fork_sink,
    ) {
        (*state).x22 = value.clone();
    } else if (*context).scfia.check_condition(
        (*context).scfia.new_bool_eq(
            register_id.clone(),
            (*context).scfia.new_bv_concrete(0b10111, 5, &mut (*context).fork_sink),
            &mut (*context).fork_sink,
        ),
        &mut (*context).fork_sink,
    ) {
        (*state).x23 = value.clone();
    } else if (*context).scfia.check_condition(
        (*context).scfia.new_bool_eq(
            register_id.clone(),
            (*context).scfia.new_bv_concrete(0b11000, 5, &mut (*context).fork_sink),
            &mut (*context).fork_sink,
        ),
        &mut (*context).fork_sink,
    ) {
        (*state).x24 = value.clone();
    } else if (*context).scfia.check_condition(
        (*context).scfia.new_bool_eq(
            register_id.clone(),
            (*context).scfia.new_bv_concrete(0b11001, 5, &mut (*context).fork_sink),
            &mut (*context).fork_sink,
        ),
        &mut (*context).fork_sink,
    ) {
        (*state).x25 = value.clone();
    } else if (*context).scfia.check_condition(
        (*context).scfia.new_bool_eq(
            register_id.clone(),
            (*context).scfia.new_bv_concrete(0b11010, 5, &mut (*context).fork_sink),
            &mut (*context).fork_sink,
        ),
        &mut (*context).fork_sink,
    ) {
        (*state).x26 = value.clone();
    } else if (*context).scfia.check_condition(
        (*context).scfia.new_bool_eq(
            register_id.clone(),
            (*context).scfia.new_bv_concrete(0b11011, 5, &mut (*context).fork_sink),
            &mut (*context).fork_sink,
        ),
        &mut (*context).fork_sink,
    ) {
        (*state).x27 = value.clone();
    } else if (*context).scfia.check_condition(
        (*context).scfia.new_bool_eq(
            register_id.clone(),
            (*context).scfia.new_bv_concrete(0b11100, 5, &mut (*context).fork_sink),
            &mut (*context).fork_sink,
        ),
        &mut (*context).fork_sink,
    ) {
        (*state).x28 = value.clone();
    } else if (*context).scfia.check_condition(
        (*context).scfia.new_bool_eq(
            register_id.clone(),
            (*context).scfia.new_bv_concrete(0b11101, 5, &mut (*context).fork_sink),
            &mut (*context).fork_sink,
        ),
        &mut (*context).fork_sink,
    ) {
        (*state).x29 = value.clone();
    } else if (*context).scfia.check_condition(
        (*context).scfia.new_bool_eq(
            register_id.clone(),
            (*context).scfia.new_bv_concrete(0b11110, 5, &mut (*context).fork_sink),
            &mut (*context).fork_sink,
        ),
        &mut (*context).fork_sink,
    ) {
        (*state).x30 = value.clone();
    } else if (*context).scfia.check_condition(
        (*context).scfia.new_bool_eq(
            register_id.clone(),
            (*context).scfia.new_bv_concrete(0b11111, 5, &mut (*context).fork_sink),
            &mut (*context).fork_sink,
        ),
        &mut (*context).fork_sink,
    ) {
        (*state).x31 = value.clone();
    } else {
        unimplemented!();
    }
}

unsafe fn _register_read_BV32(state: *mut SystemState, register_id: ActiveValue, context: *mut StepContext) -> ActiveValue {
    if (*context).scfia.check_condition(
        (*context).scfia.new_bool_eq(
            register_id.clone(),
            (*context).scfia.new_bv_concrete(0b0, 5, &mut (*context).fork_sink),
            &mut (*context).fork_sink,
        ),
        &mut (*context).fork_sink,
    ) {
        return (*context).scfia.new_bv_concrete(0b0, 32, &mut (*context).fork_sink);
    } else if (*context).scfia.check_condition(
        (*context).scfia.new_bool_eq(
            register_id.clone(),
            (*context).scfia.new_bv_concrete(0b1, 5, &mut (*context).fork_sink),
            &mut (*context).fork_sink,
        ),
        &mut (*context).fork_sink,
    ) {
        return (*state).x1.clone();
    } else if (*context).scfia.check_condition(
        (*context).scfia.new_bool_eq(
            register_id.clone(),
            (*context).scfia.new_bv_concrete(0b10, 5, &mut (*context).fork_sink),
            &mut (*context).fork_sink,
        ),
        &mut (*context).fork_sink,
    ) {
        return (*state).x2.clone();
    } else if (*context).scfia.check_condition(
        (*context).scfia.new_bool_eq(
            register_id.clone(),
            (*context).scfia.new_bv_concrete(0b11, 5, &mut (*context).fork_sink),
            &mut (*context).fork_sink,
        ),
        &mut (*context).fork_sink,
    ) {
        return (*state).x3.clone();
    } else if (*context).scfia.check_condition(
        (*context).scfia.new_bool_eq(
            register_id.clone(),
            (*context).scfia.new_bv_concrete(0b100, 5, &mut (*context).fork_sink),
            &mut (*context).fork_sink,
        ),
        &mut (*context).fork_sink,
    ) {
        return (*state).x4.clone();
    } else if (*context).scfia.check_condition(
        (*context).scfia.new_bool_eq(
            register_id.clone(),
            (*context).scfia.new_bv_concrete(0b101, 5, &mut (*context).fork_sink),
            &mut (*context).fork_sink,
        ),
        &mut (*context).fork_sink,
    ) {
        return (*state).x5.clone();
    } else if (*context).scfia.check_condition(
        (*context).scfia.new_bool_eq(
            register_id.clone(),
            (*context).scfia.new_bv_concrete(0b110, 5, &mut (*context).fork_sink),
            &mut (*context).fork_sink,
        ),
        &mut (*context).fork_sink,
    ) {
        return (*state).x6.clone();
    } else if (*context).scfia.check_condition(
        (*context).scfia.new_bool_eq(
            register_id.clone(),
            (*context).scfia.new_bv_concrete(0b111, 5, &mut (*context).fork_sink),
            &mut (*context).fork_sink,
        ),
        &mut (*context).fork_sink,
    ) {
        return (*state).x7.clone();
    } else if (*context).scfia.check_condition(
        (*context).scfia.new_bool_eq(
            register_id.clone(),
            (*context).scfia.new_bv_concrete(0b1000, 5, &mut (*context).fork_sink),
            &mut (*context).fork_sink,
        ),
        &mut (*context).fork_sink,
    ) {
        return (*state).x8.clone();
    } else if (*context).scfia.check_condition(
        (*context).scfia.new_bool_eq(
            register_id.clone(),
            (*context).scfia.new_bv_concrete(0b1001, 5, &mut (*context).fork_sink),
            &mut (*context).fork_sink,
        ),
        &mut (*context).fork_sink,
    ) {
        return (*state).x9.clone();
    } else if (*context).scfia.check_condition(
        (*context).scfia.new_bool_eq(
            register_id.clone(),
            (*context).scfia.new_bv_concrete(0b1010, 5, &mut (*context).fork_sink),
            &mut (*context).fork_sink,
        ),
        &mut (*context).fork_sink,
    ) {
        return (*state).x10.clone();
    } else if (*context).scfia.check_condition(
        (*context).scfia.new_bool_eq(
            register_id.clone(),
            (*context).scfia.new_bv_concrete(0b1011, 5, &mut (*context).fork_sink),
            &mut (*context).fork_sink,
        ),
        &mut (*context).fork_sink,
    ) {
        return (*state).x11.clone();
    } else if (*context).scfia.check_condition(
        (*context).scfia.new_bool_eq(
            register_id.clone(),
            (*context).scfia.new_bv_concrete(0b1100, 5, &mut (*context).fork_sink),
            &mut (*context).fork_sink,
        ),
        &mut (*context).fork_sink,
    ) {
        return (*state).x12.clone();
    } else if (*context).scfia.check_condition(
        (*context).scfia.new_bool_eq(
            register_id.clone(),
            (*context).scfia.new_bv_concrete(0b1101, 5, &mut (*context).fork_sink),
            &mut (*context).fork_sink,
        ),
        &mut (*context).fork_sink,
    ) {
        return (*state).x13.clone();
    } else if (*context).scfia.check_condition(
        (*context).scfia.new_bool_eq(
            register_id.clone(),
            (*context).scfia.new_bv_concrete(0b1110, 5, &mut (*context).fork_sink),
            &mut (*context).fork_sink,
        ),
        &mut (*context).fork_sink,
    ) {
        return (*state).x14.clone();
    } else if (*context).scfia.check_condition(
        (*context).scfia.new_bool_eq(
            register_id.clone(),
            (*context).scfia.new_bv_concrete(0b1111, 5, &mut (*context).fork_sink),
            &mut (*context).fork_sink,
        ),
        &mut (*context).fork_sink,
    ) {
        return (*state).x15.clone();
    } else if (*context).scfia.check_condition(
        (*context).scfia.new_bool_eq(
            register_id.clone(),
            (*context).scfia.new_bv_concrete(0b10000, 5, &mut (*context).fork_sink),
            &mut (*context).fork_sink,
        ),
        &mut (*context).fork_sink,
    ) {
        return (*state).x16.clone();
    } else if (*context).scfia.check_condition(
        (*context).scfia.new_bool_eq(
            register_id.clone(),
            (*context).scfia.new_bv_concrete(0b10001, 5, &mut (*context).fork_sink),
            &mut (*context).fork_sink,
        ),
        &mut (*context).fork_sink,
    ) {
        return (*state).x17.clone();
    } else if (*context).scfia.check_condition(
        (*context).scfia.new_bool_eq(
            register_id.clone(),
            (*context).scfia.new_bv_concrete(0b10010, 5, &mut (*context).fork_sink),
            &mut (*context).fork_sink,
        ),
        &mut (*context).fork_sink,
    ) {
        return (*state).x18.clone();
    } else if (*context).scfia.check_condition(
        (*context).scfia.new_bool_eq(
            register_id.clone(),
            (*context).scfia.new_bv_concrete(0b10011, 5, &mut (*context).fork_sink),
            &mut (*context).fork_sink,
        ),
        &mut (*context).fork_sink,
    ) {
        return (*state).x19.clone();
    } else if (*context).scfia.check_condition(
        (*context).scfia.new_bool_eq(
            register_id.clone(),
            (*context).scfia.new_bv_concrete(0b10100, 5, &mut (*context).fork_sink),
            &mut (*context).fork_sink,
        ),
        &mut (*context).fork_sink,
    ) {
        return (*state).x20.clone();
    } else if (*context).scfia.check_condition(
        (*context).scfia.new_bool_eq(
            register_id.clone(),
            (*context).scfia.new_bv_concrete(0b10101, 5, &mut (*context).fork_sink),
            &mut (*context).fork_sink,
        ),
        &mut (*context).fork_sink,
    ) {
        return (*state).x21.clone();
    } else if (*context).scfia.check_condition(
        (*context).scfia.new_bool_eq(
            register_id.clone(),
            (*context).scfia.new_bv_concrete(0b10110, 5, &mut (*context).fork_sink),
            &mut (*context).fork_sink,
        ),
        &mut (*context).fork_sink,
    ) {
        return (*state).x22.clone();
    } else if (*context).scfia.check_condition(
        (*context).scfia.new_bool_eq(
            register_id.clone(),
            (*context).scfia.new_bv_concrete(0b10111, 5, &mut (*context).fork_sink),
            &mut (*context).fork_sink,
        ),
        &mut (*context).fork_sink,
    ) {
        return (*state).x23.clone();
    } else if (*context).scfia.check_condition(
        (*context).scfia.new_bool_eq(
            register_id.clone(),
            (*context).scfia.new_bv_concrete(0b11000, 5, &mut (*context).fork_sink),
            &mut (*context).fork_sink,
        ),
        &mut (*context).fork_sink,
    ) {
        return (*state).x24.clone();
    } else if (*context).scfia.check_condition(
        (*context).scfia.new_bool_eq(
            register_id.clone(),
            (*context).scfia.new_bv_concrete(0b11001, 5, &mut (*context).fork_sink),
            &mut (*context).fork_sink,
        ),
        &mut (*context).fork_sink,
    ) {
        return (*state).x25.clone();
    } else if (*context).scfia.check_condition(
        (*context).scfia.new_bool_eq(
            register_id.clone(),
            (*context).scfia.new_bv_concrete(0b11010, 5, &mut (*context).fork_sink),
            &mut (*context).fork_sink,
        ),
        &mut (*context).fork_sink,
    ) {
        return (*state).x26.clone();
    } else if (*context).scfia.check_condition(
        (*context).scfia.new_bool_eq(
            register_id.clone(),
            (*context).scfia.new_bv_concrete(0b11011, 5, &mut (*context).fork_sink),
            &mut (*context).fork_sink,
        ),
        &mut (*context).fork_sink,
    ) {
        return (*state).x27.clone();
    } else if (*context).scfia.check_condition(
        (*context).scfia.new_bool_eq(
            register_id.clone(),
            (*context).scfia.new_bv_concrete(0b11100, 5, &mut (*context).fork_sink),
            &mut (*context).fork_sink,
        ),
        &mut (*context).fork_sink,
    ) {
        return (*state).x28.clone();
    } else if (*context).scfia.check_condition(
        (*context).scfia.new_bool_eq(
            register_id.clone(),
            (*context).scfia.new_bv_concrete(0b11101, 5, &mut (*context).fork_sink),
            &mut (*context).fork_sink,
        ),
        &mut (*context).fork_sink,
    ) {
        return (*state).x29.clone();
    } else if (*context).scfia.check_condition(
        (*context).scfia.new_bool_eq(
            register_id.clone(),
            (*context).scfia.new_bv_concrete(0b11110, 5, &mut (*context).fork_sink),
            &mut (*context).fork_sink,
        ),
        &mut (*context).fork_sink,
    ) {
        return (*state).x30.clone();
    } else if (*context).scfia.check_condition(
        (*context).scfia.new_bool_eq(
            register_id.clone(),
            (*context).scfia.new_bv_concrete(0b11111, 5, &mut (*context).fork_sink),
            &mut (*context).fork_sink,
        ),
        &mut (*context).fork_sink,
    ) {
        return (*state).x31.clone();
    } else {
        unimplemented!();
    }
}

unsafe fn _execute_add32(state: *mut SystemState, destination_id: ActiveValue, source1_id: ActiveValue, source2_id: ActiveValue, context: *mut StepContext) {
    let s1: ActiveValue = _register_read_BV32(state, source1_id.clone(), context);
    let s2: ActiveValue = _register_read_BV32(state, source2_id.clone(), context);
    let sum: ActiveValue = (*context).scfia.new_bv_add(s1.clone(), s2.clone(), 32, &mut (*context).fork_sink);
    _register_write_BV32(state, destination_id.clone(), sum.clone(), context);
}
