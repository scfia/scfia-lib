#![allow(clippy::all)]
#![allow(non_snake_case)]
#![allow(unused)]
use log::debug;
use std::{borrow::BorrowMut, fmt::Debug, collections::BTreeMap};

use crate::{memory::Memory, scfia::ScfiaOld, values::{active_value::ActiveValue, retired_value::RetiredValue}, GenericForkSink, ScfiaComposition, StepContext, SymbolicHints};

pub struct RV32i {
    pub state: SystemState,
    pub memory: Memory<RV32iScfiaComposition>,
    pub scfia: ScfiaOld<RV32iScfiaComposition>,
}

#[derive(Debug)]
pub struct RV32iForkSink {
    base_state: RV32i,
    new_values_history: Vec<ActiveValue<RV32iScfiaComposition>>,
    forks: Vec<RV32i>,
}

#[derive(Debug, Clone)]
pub struct RV32iScfiaComposition {}


impl GenericForkSink<RV32iScfiaComposition> for RV32iForkSink {
    fn fork(&mut self, fork_symbol: ActiveValue<RV32iScfiaComposition>) {
        // Clone the base state.
        debug!("fork cloning base state");
        let (clone, mut cloned_actives, mut cloned_retired) = self.base_state.clone_model();

        // Clone all values that were created after self.base_state was created.
        // new_values keeps the values active until we have cloned fork_symbol.
        debug!("fork cloning new values");
        let mut new_values = vec![];
        for new_value in &self.new_values_history {
            new_values.push(
                new_value
                    .try_borrow()
                    .unwrap()
                    .clone_to_stdlib(&mut clone.scfia.inner.try_borrow_mut().unwrap(), clone.scfia.clone(), &mut cloned_actives, &mut cloned_retired),
            );
        }
        debug!("fork asserting fork symbol");
        let fork_symbol_id = fork_symbol.try_borrow().unwrap().id;
        clone.scfia.inner.try_borrow_mut().unwrap().next_symbol_id = fork_symbol_id + 1;
        let cloned_fork_symbol = cloned_actives.get(&fork_symbol_id).unwrap();// fork_symbol.try_borrow().unwrap().clone_to_stdlib(&mut clone.scfia.inner.try_borrow_mut().unwrap(), clone.scfia.clone(), &mut cloned_actives, &mut cloned_retired);
        cloned_fork_symbol.try_borrow_mut().unwrap().assert();
        self.forks.push(clone);
    }

    fn push_value(&mut self, value: ActiveValue<RV32iScfiaComposition>) {
        self.new_values_history.push(value)
    }
}

impl ScfiaComposition for RV32iScfiaComposition {
    type Model = RV32i;
    type ForkSink = RV32iForkSink;
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

    pub fn step_forking(self, mut hints: Option<SymbolicHints>) -> Vec<RV32i> {
        unsafe {
            let mut states: Vec<RV32i> = vec![self];
            let mut results = vec![];

            while let Some(mut state) = states.pop() {
                let mut context = StepContext {
                    memory: &mut state.memory,
                    scfia: state.scfia.clone(),
                    hints: hints.clone(),
                    fork_sink: Some(RV32iForkSink {
                        base_state: state.clone_model().0,
                        new_values_history: vec![],
                        forks: vec![],
                    }),
                };
                debug!("forking step start");
                _step(&mut state.state, &mut context);
                debug!("forking step done");
                states.append(&mut context.fork_sink.unwrap().forks);
                results.push(state)
            }

            results
        }
    }

    pub fn clone_model(&self) -> (RV32i, BTreeMap<u64, ActiveValue<RV32iScfiaComposition>>, BTreeMap<u64, RetiredValue<RV32iScfiaComposition>>) {
        unsafe {
            let own_scfia = self.scfia.inner.try_borrow().unwrap();
            let cloned_scfia_rc: ScfiaOld<RV32iScfiaComposition> = ScfiaOld::new(Some(own_scfia.next_symbol_id));
            let mut cloned_actives = BTreeMap::new();
            let mut cloned_retireds = BTreeMap::new();
            debug!("cloning scfia {:?} to {:?}", self.scfia.inner.as_ptr(), cloned_scfia_rc.inner.as_ptr());
            (RV32i {
                state: self.state.clone_to_stdlib(cloned_scfia_rc.clone(), &mut cloned_actives, &mut cloned_retireds),
                memory: self.memory.clone_to_stdlib(cloned_scfia_rc.clone(), &mut cloned_actives, &mut cloned_retireds),
                scfia: cloned_scfia_rc,
            }, cloned_actives, cloned_retireds)
        }
    }
}

impl Debug for RV32i {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("RV32i").field("state", &self.state).finish()
    }
}

#[derive(Debug)]
pub struct SystemState {
    pub x0: ActiveValue<RV32iScfiaComposition>,
    pub x1: ActiveValue<RV32iScfiaComposition>,
    pub x2: ActiveValue<RV32iScfiaComposition>,
    pub x3: ActiveValue<RV32iScfiaComposition>,
    pub x4: ActiveValue<RV32iScfiaComposition>,
    pub x5: ActiveValue<RV32iScfiaComposition>,
    pub x6: ActiveValue<RV32iScfiaComposition>,
    pub x7: ActiveValue<RV32iScfiaComposition>,
    pub x8: ActiveValue<RV32iScfiaComposition>,
    pub x9: ActiveValue<RV32iScfiaComposition>,
    pub x10: ActiveValue<RV32iScfiaComposition>,
    pub x11: ActiveValue<RV32iScfiaComposition>,
    pub x12: ActiveValue<RV32iScfiaComposition>,
    pub x13: ActiveValue<RV32iScfiaComposition>,
    pub x14: ActiveValue<RV32iScfiaComposition>,
    pub x15: ActiveValue<RV32iScfiaComposition>,
    pub x16: ActiveValue<RV32iScfiaComposition>,
    pub x17: ActiveValue<RV32iScfiaComposition>,
    pub x18: ActiveValue<RV32iScfiaComposition>,
    pub x19: ActiveValue<RV32iScfiaComposition>,
    pub x20: ActiveValue<RV32iScfiaComposition>,
    pub x21: ActiveValue<RV32iScfiaComposition>,
    pub x22: ActiveValue<RV32iScfiaComposition>,
    pub x23: ActiveValue<RV32iScfiaComposition>,
    pub x24: ActiveValue<RV32iScfiaComposition>,
    pub x25: ActiveValue<RV32iScfiaComposition>,
    pub x26: ActiveValue<RV32iScfiaComposition>,
    pub x27: ActiveValue<RV32iScfiaComposition>,
    pub x28: ActiveValue<RV32iScfiaComposition>,
    pub x29: ActiveValue<RV32iScfiaComposition>,
    pub x30: ActiveValue<RV32iScfiaComposition>,
    pub x31: ActiveValue<RV32iScfiaComposition>,
    pub pc: ActiveValue<RV32iScfiaComposition>,
}

impl SystemState {
    fn clone_to_stdlib(&self, cloned_scfia_rc: ScfiaOld<RV32iScfiaComposition>, cloned_actives: &mut BTreeMap<u64, ActiveValue<RV32iScfiaComposition>>, cloned_retired: &mut BTreeMap<u64, RetiredValue<RV32iScfiaComposition>>) -> SystemState {
        let mut cloned_scfia = cloned_scfia_rc.inner.try_borrow_mut().unwrap();
        SystemState {
            x0: self.x0.try_borrow().unwrap().clone_to_stdlib(&mut cloned_scfia, cloned_scfia_rc.clone(), cloned_actives, cloned_retired),
            x1: self.x1.try_borrow().unwrap().clone_to_stdlib(&mut cloned_scfia, cloned_scfia_rc.clone(), cloned_actives, cloned_retired),
            x2: self.x2.try_borrow().unwrap().clone_to_stdlib(&mut cloned_scfia, cloned_scfia_rc.clone(), cloned_actives, cloned_retired),
            x3: self.x3.try_borrow().unwrap().clone_to_stdlib(&mut cloned_scfia, cloned_scfia_rc.clone(), cloned_actives, cloned_retired),
            x4: self.x4.try_borrow().unwrap().clone_to_stdlib(&mut cloned_scfia, cloned_scfia_rc.clone(), cloned_actives, cloned_retired),
            x5: self.x5.try_borrow().unwrap().clone_to_stdlib(&mut cloned_scfia, cloned_scfia_rc.clone(), cloned_actives, cloned_retired),
            x6: self.x6.try_borrow().unwrap().clone_to_stdlib(&mut cloned_scfia, cloned_scfia_rc.clone(), cloned_actives, cloned_retired),
            x7: self.x7.try_borrow().unwrap().clone_to_stdlib(&mut cloned_scfia, cloned_scfia_rc.clone(), cloned_actives, cloned_retired),
            x8: self.x8.try_borrow().unwrap().clone_to_stdlib(&mut cloned_scfia, cloned_scfia_rc.clone(), cloned_actives, cloned_retired),
            x9: self.x9.try_borrow().unwrap().clone_to_stdlib(&mut cloned_scfia, cloned_scfia_rc.clone(), cloned_actives, cloned_retired),
            x10: self.x10.try_borrow().unwrap().clone_to_stdlib(&mut cloned_scfia, cloned_scfia_rc.clone(), cloned_actives, cloned_retired),
            x11: self.x11.try_borrow().unwrap().clone_to_stdlib(&mut cloned_scfia, cloned_scfia_rc.clone(), cloned_actives, cloned_retired),
            x12: self.x12.try_borrow().unwrap().clone_to_stdlib(&mut cloned_scfia, cloned_scfia_rc.clone(), cloned_actives, cloned_retired),
            x13: self.x13.try_borrow().unwrap().clone_to_stdlib(&mut cloned_scfia, cloned_scfia_rc.clone(), cloned_actives, cloned_retired),
            x14: self.x14.try_borrow().unwrap().clone_to_stdlib(&mut cloned_scfia, cloned_scfia_rc.clone(), cloned_actives, cloned_retired),
            x15: self.x15.try_borrow().unwrap().clone_to_stdlib(&mut cloned_scfia, cloned_scfia_rc.clone(), cloned_actives, cloned_retired),
            x16: self.x16.try_borrow().unwrap().clone_to_stdlib(&mut cloned_scfia, cloned_scfia_rc.clone(), cloned_actives, cloned_retired),
            x17: self.x17.try_borrow().unwrap().clone_to_stdlib(&mut cloned_scfia, cloned_scfia_rc.clone(), cloned_actives, cloned_retired),
            x18: self.x18.try_borrow().unwrap().clone_to_stdlib(&mut cloned_scfia, cloned_scfia_rc.clone(), cloned_actives, cloned_retired),
            x19: self.x19.try_borrow().unwrap().clone_to_stdlib(&mut cloned_scfia, cloned_scfia_rc.clone(), cloned_actives, cloned_retired),
            x20: self.x20.try_borrow().unwrap().clone_to_stdlib(&mut cloned_scfia, cloned_scfia_rc.clone(), cloned_actives, cloned_retired),
            x21: self.x21.try_borrow().unwrap().clone_to_stdlib(&mut cloned_scfia, cloned_scfia_rc.clone(), cloned_actives, cloned_retired),
            x22: self.x22.try_borrow().unwrap().clone_to_stdlib(&mut cloned_scfia, cloned_scfia_rc.clone(), cloned_actives, cloned_retired),
            x23: self.x23.try_borrow().unwrap().clone_to_stdlib(&mut cloned_scfia, cloned_scfia_rc.clone(), cloned_actives, cloned_retired),
            x24: self.x24.try_borrow().unwrap().clone_to_stdlib(&mut cloned_scfia, cloned_scfia_rc.clone(), cloned_actives, cloned_retired),
            x25: self.x25.try_borrow().unwrap().clone_to_stdlib(&mut cloned_scfia, cloned_scfia_rc.clone(), cloned_actives, cloned_retired),
            x26: self.x26.try_borrow().unwrap().clone_to_stdlib(&mut cloned_scfia, cloned_scfia_rc.clone(), cloned_actives, cloned_retired),
            x27: self.x27.try_borrow().unwrap().clone_to_stdlib(&mut cloned_scfia, cloned_scfia_rc.clone(), cloned_actives, cloned_retired),
            x28: self.x28.try_borrow().unwrap().clone_to_stdlib(&mut cloned_scfia, cloned_scfia_rc.clone(), cloned_actives, cloned_retired),
            x29: self.x29.try_borrow().unwrap().clone_to_stdlib(&mut cloned_scfia, cloned_scfia_rc.clone(), cloned_actives, cloned_retired),
            x30: self.x30.try_borrow().unwrap().clone_to_stdlib(&mut cloned_scfia, cloned_scfia_rc.clone(), cloned_actives, cloned_retired),
            x31: self.x31.try_borrow().unwrap().clone_to_stdlib(&mut cloned_scfia, cloned_scfia_rc.clone(), cloned_actives, cloned_retired),
            pc: self.pc.try_borrow().unwrap().clone_to_stdlib(&mut cloned_scfia, cloned_scfia_rc.clone(), cloned_actives, cloned_retired),
        }
    }
}

#[derive(Debug)]
pub struct ComplexSpecialStruct {
    pub some_flag: ActiveValue<RV32iScfiaComposition>,
}

unsafe fn _reset(state: *mut SystemState, context: *mut StepContext<RV32iScfiaComposition>) {
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

unsafe fn _sum(state: *mut SystemState, context: *mut StepContext<RV32iScfiaComposition>) -> ActiveValue<RV32iScfiaComposition> {
    return (*context)
        .scfia
        .new_bv_add((*state).x1.clone(), (*state).x2.clone(), 32, &mut (*context).fork_sink);
}

unsafe fn _step(state: *mut SystemState, context: *mut StepContext<RV32iScfiaComposition>) {
    let instruction_32: ActiveValue<RV32iScfiaComposition> = (*(*context).memory).read(
        (*state).pc.clone(),
        32,
        (*context).scfia.clone(),
        &mut (*context).hints,
        &mut (*context).fork_sink,
    );
    let opcode: ActiveValue<RV32iScfiaComposition> = (*context).scfia.new_bv_slice(instruction_32.clone(), 6, 0, &mut (*context).fork_sink);
    if (*context).scfia.check_condition(
        (*context).scfia.new_bool_eq(
            opcode.clone(),
            (*context).scfia.new_bv_concrete(0b11, 7, &mut (*context).fork_sink),
            &mut (*context).fork_sink,
        ),
        &mut (*context).fork_sink,
    ) {
        let funct3: ActiveValue<RV32iScfiaComposition> = (*context).scfia.new_bv_slice(instruction_32.clone(), 14, 12, &mut (*context).fork_sink);
        if (*context).scfia.check_condition(
            (*context).scfia.new_bool_eq(
                funct3.clone(),
                (*context).scfia.new_bv_concrete(0b1, 3, &mut (*context).fork_sink),
                &mut (*context).fork_sink,
            ),
            &mut (*context).fork_sink,
        ) {
            let rd: ActiveValue<RV32iScfiaComposition> = _extract_rd_32(instruction_32.clone(), context);
            let rs1: ActiveValue<RV32iScfiaComposition> = _extract_rs1_32(instruction_32.clone(), context);
            let imm: ActiveValue<RV32iScfiaComposition> = (*context).scfia.new_bv_slice(instruction_32.clone(), 31, 20, &mut (*context).fork_sink);
            let imm32: ActiveValue<RV32iScfiaComposition> = (*context).scfia.new_bv_sign_extend(imm.clone(), 12, 32, &mut (*context).fork_sink);
            let base_address: ActiveValue<RV32iScfiaComposition> = _register_read_BV32(state, rs1.clone(), context);
            let address: ActiveValue<RV32iScfiaComposition> = (*context).scfia.new_bv_add(base_address.clone(), imm32.clone(), 32, &mut (*context).fork_sink);
            let value16: ActiveValue<RV32iScfiaComposition> =
                (*(*context).memory).read(address.clone(), 16, (*context).scfia.clone(), &mut (*context).hints, &mut (*context).fork_sink);
            let value: ActiveValue<RV32iScfiaComposition> = (*context).scfia.new_bv_sign_extend(value16.clone(), 16, 32, &mut (*context).fork_sink);
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
            let rd: ActiveValue<RV32iScfiaComposition> = _extract_rd_32(instruction_32.clone(), context);
            let rs1: ActiveValue<RV32iScfiaComposition> = _extract_rs1_32(instruction_32.clone(), context);
            let imm: ActiveValue<RV32iScfiaComposition> = (*context).scfia.new_bv_slice(instruction_32.clone(), 31, 20, &mut (*context).fork_sink);
            let imm32: ActiveValue<RV32iScfiaComposition> = (*context).scfia.new_bv_sign_extend(imm.clone(), 12, 32, &mut (*context).fork_sink);
            let base_address: ActiveValue<RV32iScfiaComposition> = _register_read_BV32(state, rs1.clone(), context);
            let address: ActiveValue<RV32iScfiaComposition> = (*context).scfia.new_bv_add(base_address.clone(), imm32.clone(), 32, &mut (*context).fork_sink);
            let value: ActiveValue<RV32iScfiaComposition> =
                (*(*context).memory).read(address.clone(), 32, (*context).scfia.clone(), &mut (*context).hints, &mut (*context).fork_sink);
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
            let rd: ActiveValue<RV32iScfiaComposition> = _extract_rd_32(instruction_32.clone(), context);
            let rs1: ActiveValue<RV32iScfiaComposition> = _extract_rs1_32(instruction_32.clone(), context);
            let imm: ActiveValue<RV32iScfiaComposition> = (*context).scfia.new_bv_slice(instruction_32.clone(), 31, 20, &mut (*context).fork_sink);
            let imm32: ActiveValue<RV32iScfiaComposition> = (*context).scfia.new_bv_sign_extend(imm.clone(), 12, 32, &mut (*context).fork_sink);
            let base_address: ActiveValue<RV32iScfiaComposition> = _register_read_BV32(state, rs1.clone(), context);
            let address: ActiveValue<RV32iScfiaComposition> = (*context).scfia.new_bv_add(base_address.clone(), imm32.clone(), 32, &mut (*context).fork_sink);
            let value8: ActiveValue<RV32iScfiaComposition> =
                (*(*context).memory).read(address.clone(), 8, (*context).scfia.clone(), &mut (*context).hints, &mut (*context).fork_sink);
            let value: ActiveValue<RV32iScfiaComposition> = (*context).scfia.new_bv_concat(
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
            let rd: ActiveValue<RV32iScfiaComposition> = _extract_rd_32(instruction_32.clone(), context);
            let rs1: ActiveValue<RV32iScfiaComposition> = _extract_rs1_32(instruction_32.clone(), context);
            let imm: ActiveValue<RV32iScfiaComposition> = (*context).scfia.new_bv_slice(instruction_32.clone(), 31, 20, &mut (*context).fork_sink);
            let imm32: ActiveValue<RV32iScfiaComposition> = (*context).scfia.new_bv_sign_extend(imm.clone(), 12, 32, &mut (*context).fork_sink);
            let base_address: ActiveValue<RV32iScfiaComposition> = _register_read_BV32(state, rs1.clone(), context);
            let address: ActiveValue<RV32iScfiaComposition> = (*context).scfia.new_bv_add(base_address.clone(), imm32.clone(), 32, &mut (*context).fork_sink);
            let value16: ActiveValue<RV32iScfiaComposition> =
                (*(*context).memory).read(address.clone(), 16, (*context).scfia.clone(), &mut (*context).hints, &mut (*context).fork_sink);
            let value: ActiveValue<RV32iScfiaComposition> = (*context).scfia.new_bv_concat(
                (*context).scfia.new_bv_concrete(0b0, 16, &mut (*context).fork_sink),
                value16.clone(),
                32,
                &mut (*context).fork_sink,
            );
            _register_write_BV32(state, rd.clone(), value.clone(), context);
            _progress_pc_4(state, context);
        } else {
            unimplemented!("unsupported func3 {:x?}", funct3);
        }
    } else if (*context).scfia.check_condition(
        (*context).scfia.new_bool_eq(
            opcode.clone(),
            (*context).scfia.new_bv_concrete(0b1111, 7, &mut (*context).fork_sink),
            &mut (*context).fork_sink,
        ),
        &mut (*context).fork_sink,
    ) {
        let rd_zeroes: ActiveValue<RV32iScfiaComposition> = (*context).scfia.new_bv_slice(instruction_32.clone(), 11, 7, &mut (*context).fork_sink);
        if (*context).scfia.check_condition(
            (*context).scfia.new_bool_eq(
                rd_zeroes.clone(),
                (*context).scfia.new_bv_concrete(0b0, 5, &mut (*context).fork_sink),
                &mut (*context).fork_sink,
            ),
            &mut (*context).fork_sink,
        ) {
            let funct3: ActiveValue<RV32iScfiaComposition> = (*context).scfia.new_bv_slice(instruction_32.clone(), 14, 12, &mut (*context).fork_sink);
            let rs1_zeroes: ActiveValue<RV32iScfiaComposition> = (*context).scfia.new_bv_slice(instruction_32.clone(), 19, 15, &mut (*context).fork_sink);
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
        let funct3: ActiveValue<RV32iScfiaComposition> = (*context).scfia.new_bv_slice(instruction_32.clone(), 14, 12, &mut (*context).fork_sink);
        if (*context).scfia.check_condition(
            (*context).scfia.new_bool_eq(
                funct3.clone(),
                (*context).scfia.new_bv_concrete(0b0, 3, &mut (*context).fork_sink),
                &mut (*context).fork_sink,
            ),
            &mut (*context).fork_sink,
        ) {
            let rd: ActiveValue<RV32iScfiaComposition> = _extract_rd_32(instruction_32.clone(), context);
            let rs1: ActiveValue<RV32iScfiaComposition> = _extract_rs1_32(instruction_32.clone(), context);
            let offset: ActiveValue<RV32iScfiaComposition> = (*context).scfia.new_bv_slice(instruction_32.clone(), 31, 20, &mut (*context).fork_sink);
            let offset_32: ActiveValue<RV32iScfiaComposition> = (*context).scfia.new_bv_sign_extend(offset.clone(), 12, 32, &mut (*context).fork_sink);
            let result: ActiveValue<RV32iScfiaComposition> = (*context).scfia.new_bv_add(
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
            let funct7: ActiveValue<RV32iScfiaComposition> = (*context).scfia.new_bv_slice(instruction_32.clone(), 31, 25, &mut (*context).fork_sink);
            if (*context).scfia.check_condition(
                (*context).scfia.new_bool_eq(
                    funct7.clone(),
                    (*context).scfia.new_bv_concrete(0b0, 7, &mut (*context).fork_sink),
                    &mut (*context).fork_sink,
                ),
                &mut (*context).fork_sink,
            ) {
                let rd: ActiveValue<RV32iScfiaComposition> = _extract_rd_32(instruction_32.clone(), context);
                let rs1: ActiveValue<RV32iScfiaComposition> = _extract_rs1_32(instruction_32.clone(), context);
                let shamt: ActiveValue<RV32iScfiaComposition> = (*context).scfia.new_bv_slice(instruction_32.clone(), 24, 20, &mut (*context).fork_sink);
                let result: ActiveValue<RV32iScfiaComposition> = (*context).scfia.new_bv_sll(
                    _register_read_BV32(state, rs1.clone(), context),
                    (*context).scfia.new_bv_concat(
                        (*context).scfia.new_bv_concrete(0, 32-5, &mut (*context).fork_sink),
                        shamt.clone(), 32, &mut (*context).fork_sink),
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
                (*context).scfia.new_bv_concrete(0b100, 3, &mut (*context).fork_sink),
                &mut (*context).fork_sink,
            ),
            &mut (*context).fork_sink,
        ) {
            let rd: ActiveValue<RV32iScfiaComposition> = _extract_rd_32(instruction_32.clone(), context);
            let rs1: ActiveValue<RV32iScfiaComposition> = _extract_rs1_32(instruction_32.clone(), context);
            let imm: ActiveValue<RV32iScfiaComposition> = (*context).scfia.new_bv_slice(instruction_32.clone(), 31, 20, &mut (*context).fork_sink);
            let imm_32: ActiveValue<RV32iScfiaComposition> = (*context).scfia.new_bv_sign_extend(imm.clone(), 12, 32, &mut (*context).fork_sink);
            let result: ActiveValue<RV32iScfiaComposition> =
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
            let funct7: ActiveValue<RV32iScfiaComposition> = (*context).scfia.new_bv_slice(instruction_32.clone(), 31, 25, &mut (*context).fork_sink);
            if (*context).scfia.check_condition(
                (*context).scfia.new_bool_eq(
                    funct7.clone(),
                    (*context).scfia.new_bv_concrete(0b0, 7, &mut (*context).fork_sink),
                    &mut (*context).fork_sink,
                ),
                &mut (*context).fork_sink,
            ) {
                let rd: ActiveValue<RV32iScfiaComposition> = _extract_rd_32(instruction_32.clone(), context);
                let rs1: ActiveValue<RV32iScfiaComposition> = _extract_rs1_32(instruction_32.clone(), context);
                let shamt: ActiveValue<RV32iScfiaComposition> = (*context).scfia.new_bv_slice(instruction_32.clone(), 24, 20, &mut (*context).fork_sink);
                let result: ActiveValue<RV32iScfiaComposition> = (*context).scfia.new_bv_srl(
                    _register_read_BV32(state, rs1.clone(), context),
                    (*context).scfia.new_bv_concat((*context).scfia.new_bv_concrete(0, 32-5, &mut (*context).fork_sink), shamt.clone(), 32, &mut (*context).fork_sink),
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
            let rd: ActiveValue<RV32iScfiaComposition> = _extract_rd_32(instruction_32.clone(), context);
            let rs1: ActiveValue<RV32iScfiaComposition> = _extract_rs1_32(instruction_32.clone(), context);
            let imm: ActiveValue<RV32iScfiaComposition> = (*context).scfia.new_bv_slice(instruction_32.clone(), 31, 20, &mut (*context).fork_sink);
            let imm_32: ActiveValue<RV32iScfiaComposition> = (*context).scfia.new_bv_sign_extend(imm.clone(), 12, 32, &mut (*context).fork_sink);
            let result: ActiveValue<RV32iScfiaComposition> =
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
            let rd: ActiveValue<RV32iScfiaComposition> = _extract_rd_32(instruction_32.clone(), context);
            let rs1: ActiveValue<RV32iScfiaComposition> = _extract_rs1_32(instruction_32.clone(), context);
            let imm: ActiveValue<RV32iScfiaComposition> = (*context).scfia.new_bv_slice(instruction_32.clone(), 31, 20, &mut (*context).fork_sink);
            let imm_32: ActiveValue<RV32iScfiaComposition> = (*context).scfia.new_bv_sign_extend(imm.clone(), 12, 32, &mut (*context).fork_sink);
            let result: ActiveValue<RV32iScfiaComposition> =
                (*context)
                    .scfia
                    .new_bv_and(_register_read_BV32(state, rs1.clone(), context), imm_32.clone(), 32, &mut (*context).fork_sink);
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
        let funct3: ActiveValue<RV32iScfiaComposition> = (*context).scfia.new_bv_slice(instruction_32.clone(), 14, 12, &mut (*context).fork_sink);
        if (*context).scfia.check_condition(
            (*context).scfia.new_bool_eq(
                funct3.clone(),
                (*context).scfia.new_bv_concrete(0b0, 3, &mut (*context).fork_sink),
                &mut (*context).fork_sink,
            ),
            &mut (*context).fork_sink,
        ) {
            let rs1: ActiveValue<RV32iScfiaComposition> = _extract_rs1_32(instruction_32.clone(), context);
            let rs2: ActiveValue<RV32iScfiaComposition> = _extract_rs2_32(instruction_32.clone(), context);
            let offset_11_5: ActiveValue<RV32iScfiaComposition> = (*context).scfia.new_bv_slice(instruction_32.clone(), 31, 25, &mut (*context).fork_sink);
            let offset_4_0: ActiveValue<RV32iScfiaComposition> = (*context).scfia.new_bv_slice(instruction_32.clone(), 11, 7, &mut (*context).fork_sink);
            let offset: ActiveValue<RV32iScfiaComposition> =
                (*context)
                    .scfia
                    .new_bv_concat(offset_11_5.clone(), offset_4_0.clone(), 12, &mut (*context).fork_sink);
            let offset_32: ActiveValue<RV32iScfiaComposition> = (*context).scfia.new_bv_sign_extend(offset.clone(), 12, 32, &mut (*context).fork_sink);
            let base_address: ActiveValue<RV32iScfiaComposition> = _register_read_BV32(state, rs1.clone(), context);
            let address: ActiveValue<RV32iScfiaComposition> =
                (*context)
                    .scfia
                    .new_bv_add(base_address.clone(), offset_32.clone(), 32, &mut (*context).fork_sink);
            let value_32: ActiveValue<RV32iScfiaComposition> = _register_read_BV32(state, rs2.clone(), context);
            let value: ActiveValue<RV32iScfiaComposition> = (*context).scfia.new_bv_slice(value_32.clone(), 7, 0, &mut (*context).fork_sink);
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
            let rs1: ActiveValue<RV32iScfiaComposition> = _extract_rs1_32(instruction_32.clone(), context);
            let rs2: ActiveValue<RV32iScfiaComposition> = _extract_rs2_32(instruction_32.clone(), context);
            let offset_11_5: ActiveValue<RV32iScfiaComposition> = (*context).scfia.new_bv_slice(instruction_32.clone(), 31, 25, &mut (*context).fork_sink);
            let offset_4_0: ActiveValue<RV32iScfiaComposition> = (*context).scfia.new_bv_slice(instruction_32.clone(), 11, 7, &mut (*context).fork_sink);
            let offset: ActiveValue<RV32iScfiaComposition> =
                (*context)
                    .scfia
                    .new_bv_concat(offset_11_5.clone(), offset_4_0.clone(), 12, &mut (*context).fork_sink);
            let offset_32: ActiveValue<RV32iScfiaComposition> = (*context).scfia.new_bv_sign_extend(offset.clone(), 12, 32, &mut (*context).fork_sink);
            let base_address: ActiveValue<RV32iScfiaComposition> = _register_read_BV32(state, rs1.clone(), context);
            let address: ActiveValue<RV32iScfiaComposition> =
                (*context)
                    .scfia
                    .new_bv_add(base_address.clone(), offset_32.clone(), 32, &mut (*context).fork_sink);
            let value_32: ActiveValue<RV32iScfiaComposition> = _register_read_BV32(state, rs2.clone(), context);
            let value: ActiveValue<RV32iScfiaComposition> = (*context).scfia.new_bv_slice(value_32.clone(), 15, 0, &mut (*context).fork_sink);
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
            let rs1: ActiveValue<RV32iScfiaComposition> = _extract_rs1_32(instruction_32.clone(), context);
            let rs2: ActiveValue<RV32iScfiaComposition> = _extract_rs2_32(instruction_32.clone(), context);
            let offset_11_5: ActiveValue<RV32iScfiaComposition> = (*context).scfia.new_bv_slice(instruction_32.clone(), 31, 25, &mut (*context).fork_sink);
            let offset_4_0: ActiveValue<RV32iScfiaComposition> = (*context).scfia.new_bv_slice(instruction_32.clone(), 11, 7, &mut (*context).fork_sink);
            let offset: ActiveValue<RV32iScfiaComposition> =
                (*context)
                    .scfia
                    .new_bv_concat(offset_11_5.clone(), offset_4_0.clone(), 12, &mut (*context).fork_sink);
            let offset_32: ActiveValue<RV32iScfiaComposition> = (*context).scfia.new_bv_sign_extend(offset.clone(), 12, 32, &mut (*context).fork_sink);
            let base_address: ActiveValue<RV32iScfiaComposition> = _register_read_BV32(state, rs1.clone(), context);
            let address: ActiveValue<RV32iScfiaComposition> =
                (*context)
                    .scfia
                    .new_bv_add(base_address.clone(), offset_32.clone(), 32, &mut (*context).fork_sink);
            let value: ActiveValue<RV32iScfiaComposition> = _register_read_BV32(state, rs2.clone(), context);
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
        let rd: ActiveValue<RV32iScfiaComposition> = _extract_rd_32(instruction_32.clone(), context);
        let rs: ActiveValue<RV32iScfiaComposition> = _extract_rs1_32(instruction_32.clone(), context);
        let imm: ActiveValue<RV32iScfiaComposition> = (*context).scfia.new_bv_slice(instruction_32.clone(), 31, 12, &mut (*context).fork_sink);
        let value: ActiveValue<RV32iScfiaComposition> = (*context).scfia.new_bv_concat(
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
        let dst: ActiveValue<RV32iScfiaComposition> = _extract_rd_32(instruction_32.clone(), context);
        let imm: ActiveValue<RV32iScfiaComposition> = (*context).scfia.new_bv_slice(instruction_32.clone(), 31, 12, &mut (*context).fork_sink);
        let imm32: ActiveValue<RV32iScfiaComposition> = (*context).scfia.new_bv_concat(
            imm.clone(),
            (*context).scfia.new_bv_concrete(0b0, 12, &mut (*context).fork_sink),
            32,
            &mut (*context).fork_sink,
        );
        let sum: ActiveValue<RV32iScfiaComposition> = (*context).scfia.new_bv_add(imm32.clone(), (*state).pc.clone(), 32, &mut (*context).fork_sink);
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
        let funct3: ActiveValue<RV32iScfiaComposition> = (*context).scfia.new_bv_slice(instruction_32.clone(), 14, 12, &mut (*context).fork_sink);
        let funct7: ActiveValue<RV32iScfiaComposition> = (*context).scfia.new_bv_slice(instruction_32.clone(), 31, 25, &mut (*context).fork_sink);
        let rd: ActiveValue<RV32iScfiaComposition> = (*context).scfia.new_bv_slice(instruction_32.clone(), 11, 7, &mut (*context).fork_sink);
        let rs1: ActiveValue<RV32iScfiaComposition> = (*context).scfia.new_bv_slice(instruction_32.clone(), 19, 15, &mut (*context).fork_sink);
        let rs2: ActiveValue<RV32iScfiaComposition> = (*context).scfia.new_bv_slice(instruction_32.clone(), 24, 20, &mut (*context).fork_sink);
        if (*context).scfia.check_condition(
            (*context).scfia.new_bool_eq(
                funct3.clone(),
                (*context).scfia.new_bv_concrete(0b0, 3, &mut (*context).fork_sink),
                &mut (*context).fork_sink,
            ),
            &mut (*context).fork_sink,
        ) {
            let s1: ActiveValue<RV32iScfiaComposition> = _register_read_BV32(state, rs1.clone(), context);
            let s2: ActiveValue<RV32iScfiaComposition> = _register_read_BV32(state, rs2.clone(), context);
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
                let result: ActiveValue<RV32iScfiaComposition> = (*context).scfia.new_bv_multiply(s1.clone(), s2.clone(), 32, &mut (*context).fork_sink);
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
                let sum: ActiveValue<RV32iScfiaComposition> = (*context).scfia.new_bv_sub(s1.clone(), s2.clone(), 32, &mut (*context).fork_sink);
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
                let shamt: ActiveValue<RV32iScfiaComposition> =
                    (*context)
                        .scfia
                        .new_bv_slice(_register_read_BV32(state, rs2.clone(), context), 4, 0, &mut (*context).fork_sink);
                let result: ActiveValue<RV32iScfiaComposition> = (*context).scfia.new_bv_sll(
                    _register_read_BV32(state, rs1.clone(), context),
                    (*context).scfia.new_bv_concat((*context).scfia.new_bv_concrete(0, 32-5, &mut (*context).fork_sink), shamt.clone(), 32, &mut (*context).fork_sink),
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
                let shamt: ActiveValue<RV32iScfiaComposition> =
                    (*context)
                        .scfia
                        .new_bv_slice(_register_read_BV32(state, rs2.clone(), context), 4, 0, &mut (*context).fork_sink);
                let result: ActiveValue<RV32iScfiaComposition> = (*context).scfia.new_bv_srl(
                    _register_read_BV32(state, rs1.clone(), context),
                    (*context).scfia.new_bv_concat((*context).scfia.new_bv_concrete(0, 32-5, &mut (*context).fork_sink), shamt.clone(), 32, &mut (*context).fork_sink),
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
                let rd: ActiveValue<RV32iScfiaComposition> = _extract_rd_32(instruction_32.clone(), context);
                let rs1: ActiveValue<RV32iScfiaComposition> = _extract_rs1_32(instruction_32.clone(), context);
                let rs2: ActiveValue<RV32iScfiaComposition> = _extract_rs2_32(instruction_32.clone(), context);
                let result: ActiveValue<RV32iScfiaComposition> = (*context).scfia.new_bv_or(
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
            let rd: ActiveValue<RV32iScfiaComposition> = _extract_rd_32(instruction_32.clone(), context);
            let rs1: ActiveValue<RV32iScfiaComposition> = _extract_rs1_32(instruction_32.clone(), context);
            let rs2: ActiveValue<RV32iScfiaComposition> = _extract_rs2_32(instruction_32.clone(), context);
            if (*context).scfia.check_condition(
                (*context).scfia.new_bool_eq(
                    funct7.clone(),
                    (*context).scfia.new_bv_concrete(0b0, 7, &mut (*context).fork_sink),
                    &mut (*context).fork_sink,
                ),
                &mut (*context).fork_sink,
            ) {
                let result: ActiveValue<RV32iScfiaComposition> = (*context).scfia.new_bv_and(
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
                let result: ActiveValue<RV32iScfiaComposition> = (*context).scfia.new_bv_unsigned_remainder(
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
        let funct3: ActiveValue<RV32iScfiaComposition> = (*context).scfia.new_bv_slice(instruction_32.clone(), 14, 12, &mut (*context).fork_sink);
        if (*context).scfia.check_condition(
            (*context).scfia.new_bool_eq(
                funct3.clone(),
                (*context).scfia.new_bv_concrete(0b0, 3, &mut (*context).fork_sink),
                &mut (*context).fork_sink,
            ),
            &mut (*context).fork_sink,
        ) {
            let lhs: ActiveValue<RV32iScfiaComposition> = _register_read_BV32(state, _extract_rs1_32(instruction_32.clone(), context), context);
            let rhs: ActiveValue<RV32iScfiaComposition> = _register_read_BV32(state, _extract_rs2_32(instruction_32.clone(), context), context);
            if (*context).scfia.check_condition(
                (*context).scfia.new_bool_eq(lhs.clone(), rhs.clone(), &mut (*context).fork_sink),
                &mut (*context).fork_sink,
            ) {
                let imm_4_1: ActiveValue<RV32iScfiaComposition> = (*context).scfia.new_bv_slice(instruction_32.clone(), 11, 8, &mut (*context).fork_sink);
                let imm_10_5: ActiveValue<RV32iScfiaComposition> = (*context).scfia.new_bv_slice(instruction_32.clone(), 30, 25, &mut (*context).fork_sink);
                let imm11: ActiveValue<RV32iScfiaComposition> = (*context).scfia.new_bv_slice(instruction_32.clone(), 7, 7, &mut (*context).fork_sink);
                let imm12: ActiveValue<RV32iScfiaComposition> = (*context).scfia.new_bv_slice(instruction_32.clone(), 31, 31, &mut (*context).fork_sink);
                let imm_4_0: ActiveValue<RV32iScfiaComposition> = (*context).scfia.new_bv_concat(
                    imm_4_1.clone(),
                    (*context).scfia.new_bv_concrete(0b0, 1, &mut (*context).fork_sink),
                    5,
                    &mut (*context).fork_sink,
                );
                let imm_10_0: ActiveValue<RV32iScfiaComposition> =
                    (*context).scfia.new_bv_concat(imm_10_5.clone(), imm_4_0.clone(), 11, &mut (*context).fork_sink);
                let imm_11_0: ActiveValue<RV32iScfiaComposition> =
                    (*context).scfia.new_bv_concat(imm11.clone(), imm_10_0.clone(), 12, &mut (*context).fork_sink);
                let imm_12_0: ActiveValue<RV32iScfiaComposition> =
                    (*context).scfia.new_bv_concat(imm12.clone(), imm_11_0.clone(), 13, &mut (*context).fork_sink);
                let offset: ActiveValue<RV32iScfiaComposition> = (*context).scfia.new_bv_sign_extend(imm_12_0.clone(), 13, 32, &mut (*context).fork_sink);
                let address: ActiveValue<RV32iScfiaComposition> =
                    (*context).scfia.new_bv_add((*state).pc.clone(), offset.clone(), 32, &mut (*context).fork_sink);
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
            let lhs: ActiveValue<RV32iScfiaComposition> = _register_read_BV32(state, _extract_rs1_32(instruction_32.clone(), context), context);
            let rhs: ActiveValue<RV32iScfiaComposition> = _register_read_BV32(state, _extract_rs2_32(instruction_32.clone(), context), context);
            if (*context).scfia.check_condition(
                (*context).scfia.new_bool_not(
                    (*context).scfia.new_bool_eq(lhs.clone(), rhs.clone(), &mut (*context).fork_sink),
                    &mut (*context).fork_sink,
                ),
                &mut (*context).fork_sink,
            ) {
                let imm_4_1: ActiveValue<RV32iScfiaComposition> = (*context).scfia.new_bv_slice(instruction_32.clone(), 11, 8, &mut (*context).fork_sink);
                let imm_10_5: ActiveValue<RV32iScfiaComposition> = (*context).scfia.new_bv_slice(instruction_32.clone(), 30, 25, &mut (*context).fork_sink);
                let imm11: ActiveValue<RV32iScfiaComposition> = (*context).scfia.new_bv_slice(instruction_32.clone(), 7, 7, &mut (*context).fork_sink);
                let imm12: ActiveValue<RV32iScfiaComposition> = (*context).scfia.new_bv_slice(instruction_32.clone(), 31, 31, &mut (*context).fork_sink);
                let imm_4_0: ActiveValue<RV32iScfiaComposition> = (*context).scfia.new_bv_concat(
                    imm_4_1.clone(),
                    (*context).scfia.new_bv_concrete(0b0, 1, &mut (*context).fork_sink),
                    5,
                    &mut (*context).fork_sink,
                );
                let imm_10_0: ActiveValue<RV32iScfiaComposition> =
                    (*context).scfia.new_bv_concat(imm_10_5.clone(), imm_4_0.clone(), 11, &mut (*context).fork_sink);
                let imm_11_0: ActiveValue<RV32iScfiaComposition> =
                    (*context).scfia.new_bv_concat(imm11.clone(), imm_10_0.clone(), 12, &mut (*context).fork_sink);
                let imm_12_0: ActiveValue<RV32iScfiaComposition> =
                    (*context).scfia.new_bv_concat(imm12.clone(), imm_11_0.clone(), 13, &mut (*context).fork_sink);
                let offset: ActiveValue<RV32iScfiaComposition> = (*context).scfia.new_bv_sign_extend(imm_12_0.clone(), 13, 32, &mut (*context).fork_sink);
                let address: ActiveValue<RV32iScfiaComposition> =
                    (*context).scfia.new_bv_add((*state).pc.clone(), offset.clone(), 32, &mut (*context).fork_sink);
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
            let lhs: ActiveValue<RV32iScfiaComposition> = _register_read_BV32(state, _extract_rs1_32(instruction_32.clone(), context), context);
            let rhs: ActiveValue<RV32iScfiaComposition> = _register_read_BV32(state, _extract_rs2_32(instruction_32.clone(), context), context);
            if (*context).scfia.check_condition(
                (*context).scfia.new_bool_signed_less_than(lhs.clone(), rhs.clone(), &mut (*context).fork_sink),
                &mut (*context).fork_sink,
            ) {
                let imm_4_1: ActiveValue<RV32iScfiaComposition> = (*context).scfia.new_bv_slice(instruction_32.clone(), 11, 8, &mut (*context).fork_sink);
                let imm_10_5: ActiveValue<RV32iScfiaComposition> = (*context).scfia.new_bv_slice(instruction_32.clone(), 30, 25, &mut (*context).fork_sink);
                let imm11: ActiveValue<RV32iScfiaComposition> = (*context).scfia.new_bv_slice(instruction_32.clone(), 7, 7, &mut (*context).fork_sink);
                let imm12: ActiveValue<RV32iScfiaComposition> = (*context).scfia.new_bv_slice(instruction_32.clone(), 31, 31, &mut (*context).fork_sink);
                let imm_4_0: ActiveValue<RV32iScfiaComposition> = (*context).scfia.new_bv_concat(
                    imm_4_1.clone(),
                    (*context).scfia.new_bv_concrete(0b0, 1, &mut (*context).fork_sink),
                    5,
                    &mut (*context).fork_sink,
                );
                let imm_10_0: ActiveValue<RV32iScfiaComposition> =
                    (*context).scfia.new_bv_concat(imm_10_5.clone(), imm_4_0.clone(), 11, &mut (*context).fork_sink);
                let imm_11_0: ActiveValue<RV32iScfiaComposition> =
                    (*context).scfia.new_bv_concat(imm11.clone(), imm_10_0.clone(), 12, &mut (*context).fork_sink);
                let imm_12_0: ActiveValue<RV32iScfiaComposition> =
                    (*context).scfia.new_bv_concat(imm12.clone(), imm_11_0.clone(), 13, &mut (*context).fork_sink);
                let offset: ActiveValue<RV32iScfiaComposition> = (*context).scfia.new_bv_sign_extend(imm_12_0.clone(), 13, 32, &mut (*context).fork_sink);
                let address: ActiveValue<RV32iScfiaComposition> =
                    (*context).scfia.new_bv_add((*state).pc.clone(), offset.clone(), 32, &mut (*context).fork_sink);
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
            let lhs: ActiveValue<RV32iScfiaComposition> = _register_read_BV32(state, _extract_rs1_32(instruction_32.clone(), context), context);
            let rhs: ActiveValue<RV32iScfiaComposition> = _register_read_BV32(state, _extract_rs2_32(instruction_32.clone(), context), context);
            if (*context).scfia.check_condition(
                (*context).scfia.new_bool_signed_less_than(lhs.clone(), rhs.clone(), &mut (*context).fork_sink),
                &mut (*context).fork_sink,
            ) {
                _progress_pc_4(state, context);
            } else {
                let imm_4_1: ActiveValue<RV32iScfiaComposition> = (*context).scfia.new_bv_slice(instruction_32.clone(), 11, 8, &mut (*context).fork_sink);
                let imm_10_5: ActiveValue<RV32iScfiaComposition> = (*context).scfia.new_bv_slice(instruction_32.clone(), 30, 25, &mut (*context).fork_sink);
                let imm11: ActiveValue<RV32iScfiaComposition> = (*context).scfia.new_bv_slice(instruction_32.clone(), 7, 7, &mut (*context).fork_sink);
                let imm12: ActiveValue<RV32iScfiaComposition> = (*context).scfia.new_bv_slice(instruction_32.clone(), 31, 31, &mut (*context).fork_sink);
                let imm_4_0: ActiveValue<RV32iScfiaComposition> = (*context).scfia.new_bv_concat(
                    imm_4_1.clone(),
                    (*context).scfia.new_bv_concrete(0b0, 1, &mut (*context).fork_sink),
                    5,
                    &mut (*context).fork_sink,
                );
                let imm_10_0: ActiveValue<RV32iScfiaComposition> =
                    (*context).scfia.new_bv_concat(imm_10_5.clone(), imm_4_0.clone(), 11, &mut (*context).fork_sink);
                let imm_11_0: ActiveValue<RV32iScfiaComposition> =
                    (*context).scfia.new_bv_concat(imm11.clone(), imm_10_0.clone(), 12, &mut (*context).fork_sink);
                let imm_12_0: ActiveValue<RV32iScfiaComposition> =
                    (*context).scfia.new_bv_concat(imm12.clone(), imm_11_0.clone(), 13, &mut (*context).fork_sink);
                let offset: ActiveValue<RV32iScfiaComposition> = (*context).scfia.new_bv_sign_extend(imm_12_0.clone(), 13, 32, &mut (*context).fork_sink);
                let address: ActiveValue<RV32iScfiaComposition> =
                    (*context).scfia.new_bv_add((*state).pc.clone(), offset.clone(), 32, &mut (*context).fork_sink);
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
            // BLTU
            let lhs: ActiveValue<RV32iScfiaComposition> = _register_read_BV32(state, _extract_rs1_32(instruction_32.clone(), context), context);
            let rhs: ActiveValue<RV32iScfiaComposition> = _register_read_BV32(state, _extract_rs2_32(instruction_32.clone(), context), context);
            if (*context).scfia.check_condition(
                (*context)
                    .scfia
                    .new_bool_unsigned_less_than(lhs.clone(), rhs.clone(), &mut (*context).fork_sink),
                &mut (*context).fork_sink,
            ) {
                let imm_4_1: ActiveValue<RV32iScfiaComposition> = (*context).scfia.new_bv_slice(instruction_32.clone(), 11, 8, &mut (*context).fork_sink);
                let imm_10_5: ActiveValue<RV32iScfiaComposition> = (*context).scfia.new_bv_slice(instruction_32.clone(), 30, 25, &mut (*context).fork_sink);
                let imm11: ActiveValue<RV32iScfiaComposition> = (*context).scfia.new_bv_slice(instruction_32.clone(), 7, 7, &mut (*context).fork_sink);
                let imm12: ActiveValue<RV32iScfiaComposition> = (*context).scfia.new_bv_slice(instruction_32.clone(), 31, 31, &mut (*context).fork_sink);
                let imm_4_0: ActiveValue<RV32iScfiaComposition> = (*context).scfia.new_bv_concat(
                    imm_4_1.clone(),
                    (*context).scfia.new_bv_concrete(0b0, 1, &mut (*context).fork_sink),
                    5,
                    &mut (*context).fork_sink,
                );
                let imm_10_0: ActiveValue<RV32iScfiaComposition> =
                    (*context).scfia.new_bv_concat(imm_10_5.clone(), imm_4_0.clone(), 11, &mut (*context).fork_sink);
                let imm_11_0: ActiveValue<RV32iScfiaComposition> =
                    (*context).scfia.new_bv_concat(imm11.clone(), imm_10_0.clone(), 12, &mut (*context).fork_sink);
                let imm_12_0: ActiveValue<RV32iScfiaComposition> =
                    (*context).scfia.new_bv_concat(imm12.clone(), imm_11_0.clone(), 13, &mut (*context).fork_sink);
                let offset: ActiveValue<RV32iScfiaComposition> = (*context).scfia.new_bv_sign_extend(imm_12_0.clone(), 13, 32, &mut (*context).fork_sink);
                let address: ActiveValue<RV32iScfiaComposition> =
                    (*context).scfia.new_bv_add((*state).pc.clone(), offset.clone(), 32, &mut (*context).fork_sink);
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
            let lhs: ActiveValue<RV32iScfiaComposition> = _register_read_BV32(state, _extract_rs1_32(instruction_32.clone(), context), context);
            let rhs: ActiveValue<RV32iScfiaComposition> = _register_read_BV32(state, _extract_rs2_32(instruction_32.clone(), context), context);
            if (*context).scfia.check_condition(
                (*context)
                    .scfia
                    .new_bool_unsigned_less_than(lhs.clone(), rhs.clone(), &mut (*context).fork_sink),
                &mut (*context).fork_sink,
            ) {
                _progress_pc_4(state, context);
            } else {
                let imm_4_1: ActiveValue<RV32iScfiaComposition> = (*context).scfia.new_bv_slice(instruction_32.clone(), 11, 8, &mut (*context).fork_sink);
                let imm_10_5: ActiveValue<RV32iScfiaComposition> = (*context).scfia.new_bv_slice(instruction_32.clone(), 30, 25, &mut (*context).fork_sink);
                let imm11: ActiveValue<RV32iScfiaComposition> = (*context).scfia.new_bv_slice(instruction_32.clone(), 7, 7, &mut (*context).fork_sink);
                let imm12: ActiveValue<RV32iScfiaComposition> = (*context).scfia.new_bv_slice(instruction_32.clone(), 31, 31, &mut (*context).fork_sink);
                let imm_4_0: ActiveValue<RV32iScfiaComposition> = (*context).scfia.new_bv_concat(
                    imm_4_1.clone(),
                    (*context).scfia.new_bv_concrete(0b0, 1, &mut (*context).fork_sink),
                    5,
                    &mut (*context).fork_sink,
                );
                let imm_10_0: ActiveValue<RV32iScfiaComposition> =
                    (*context).scfia.new_bv_concat(imm_10_5.clone(), imm_4_0.clone(), 11, &mut (*context).fork_sink);
                let imm_11_0: ActiveValue<RV32iScfiaComposition> =
                    (*context).scfia.new_bv_concat(imm11.clone(), imm_10_0.clone(), 12, &mut (*context).fork_sink);
                let imm_12_0: ActiveValue<RV32iScfiaComposition> =
                    (*context).scfia.new_bv_concat(imm12.clone(), imm_11_0.clone(), 13, &mut (*context).fork_sink);
                let offset: ActiveValue<RV32iScfiaComposition> = (*context).scfia.new_bv_sign_extend(imm_12_0.clone(), 13, 32, &mut (*context).fork_sink);
                let address: ActiveValue<RV32iScfiaComposition> =
                    (*context).scfia.new_bv_add((*state).pc.clone(), offset.clone(), 32, &mut (*context).fork_sink);
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
        let funct3: ActiveValue<RV32iScfiaComposition> = (*context).scfia.new_bv_slice(instruction_32.clone(), 14, 12, &mut (*context).fork_sink);
        if (*context).scfia.check_condition(
            (*context).scfia.new_bool_eq(
                funct3.clone(),
                (*context).scfia.new_bv_concrete(0b0, 3, &mut (*context).fork_sink),
                &mut (*context).fork_sink,
            ),
            &mut (*context).fork_sink,
        ) {
            let dst: ActiveValue<RV32iScfiaComposition> = _extract_rd_32(instruction_32.clone(), context);
            let rs1: ActiveValue<RV32iScfiaComposition> = _extract_rs1_32(instruction_32.clone(), context);
            let s1: ActiveValue<RV32iScfiaComposition> = _register_read_BV32(state, rs1.clone(), context);
            let offset: ActiveValue<RV32iScfiaComposition> = (*context).scfia.new_bv_slice(instruction_32.clone(), 31, 20, &mut (*context).fork_sink);
            let offset_32: ActiveValue<RV32iScfiaComposition> = (*context).scfia.new_bv_sign_extend(offset.clone(), 12, 32, &mut (*context).fork_sink);
            let address: ActiveValue<RV32iScfiaComposition> = (*context).scfia.new_bv_add(s1.clone(), offset_32.clone(), 32, &mut (*context).fork_sink);
            let return_address: ActiveValue<RV32iScfiaComposition> = (*context).scfia.new_bv_add(
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
        let imm_20: ActiveValue<RV32iScfiaComposition> = (*context).scfia.new_bv_slice(instruction_32.clone(), 31, 31, &mut (*context).fork_sink);
        let imm_10_1: ActiveValue<RV32iScfiaComposition> = (*context).scfia.new_bv_slice(instruction_32.clone(), 30, 21, &mut (*context).fork_sink);
        let imm_11: ActiveValue<RV32iScfiaComposition> = (*context).scfia.new_bv_slice(instruction_32.clone(), 20, 20, &mut (*context).fork_sink);
        let imm_19_12: ActiveValue<RV32iScfiaComposition> = (*context).scfia.new_bv_slice(instruction_32.clone(), 19, 12, &mut (*context).fork_sink);
        let offset_10_0: ActiveValue<RV32iScfiaComposition> = (*context).scfia.new_bv_concat(
            imm_10_1.clone(),
            (*context).scfia.new_bv_concrete(0b0, 1, &mut (*context).fork_sink),
            11,
            &mut (*context).fork_sink,
        );
        let offset_11_0: ActiveValue<RV32iScfiaComposition> =
            (*context)
                .scfia
                .new_bv_concat(imm_11.clone(), offset_10_0.clone(), 12, &mut (*context).fork_sink);
        let offset_19_0: ActiveValue<RV32iScfiaComposition> =
            (*context)
                .scfia
                .new_bv_concat(imm_19_12.clone(), offset_11_0.clone(), 20, &mut (*context).fork_sink);
        let offset_20_0: ActiveValue<RV32iScfiaComposition> =
            (*context)
                .scfia
                .new_bv_concat(imm_20.clone(), offset_19_0.clone(), 21, &mut (*context).fork_sink);
        let offset_32: ActiveValue<RV32iScfiaComposition> = (*context).scfia.new_bv_sign_extend(offset_20_0.clone(), 21, 32, &mut (*context).fork_sink);
        let address: ActiveValue<RV32iScfiaComposition> = (*context)
            .scfia
            .new_bv_add((*state).pc.clone(), offset_32.clone(), 32, &mut (*context).fork_sink);
        let return_address: ActiveValue<RV32iScfiaComposition> = (*context).scfia.new_bv_add(
            (*state).pc.clone(),
            (*context).scfia.new_bv_concrete(0b100, 32, &mut (*context).fork_sink),
            32,
            &mut (*context).fork_sink,
        );
        let dst: ActiveValue<RV32iScfiaComposition> = _extract_rd_32(instruction_32.clone(), context);
        _register_write_BV32(state, dst.clone(), return_address.clone(), context);
        (*state).pc = address.clone();
    } else {
        unimplemented!();
    }
}

unsafe fn _extract_rd_32(op: ActiveValue<RV32iScfiaComposition>, context: *mut StepContext<RV32iScfiaComposition>) -> ActiveValue<RV32iScfiaComposition> {
    return (*context).scfia.new_bv_slice(op.clone(), 11, 7, &mut (*context).fork_sink);
}

unsafe fn _extract_rs1_32(op: ActiveValue<RV32iScfiaComposition>, context: *mut StepContext<RV32iScfiaComposition>) -> ActiveValue<RV32iScfiaComposition> {
    return (*context).scfia.new_bv_slice(op.clone(), 19, 15, &mut (*context).fork_sink);
}

unsafe fn _extract_rs2_32(op: ActiveValue<RV32iScfiaComposition>, context: *mut StepContext<RV32iScfiaComposition>) -> ActiveValue<RV32iScfiaComposition> {
    return (*context).scfia.new_bv_slice(op.clone(), 24, 20, &mut (*context).fork_sink);
}

unsafe fn _progress_pc_4(state: *mut SystemState, context: *mut StepContext<RV32iScfiaComposition>) {
    let old_pc: ActiveValue<RV32iScfiaComposition> = (*state).pc.clone();
    let new_pc: ActiveValue<RV32iScfiaComposition> = (*context).scfia.new_bv_add(
        old_pc.clone(),
        (*context).scfia.new_bv_concrete(0b100, 32, &mut (*context).fork_sink),
        32,
        &mut (*context).fork_sink,
    );
    (*state).pc = new_pc.clone();
}

unsafe fn _register_write_BV32(
    state: *mut SystemState,
    register_id: ActiveValue<RV32iScfiaComposition>,
    value: ActiveValue<RV32iScfiaComposition>,
    context: *mut StepContext<RV32iScfiaComposition>,
) {
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

unsafe fn _register_read_BV32(
    state: *mut SystemState,
    register_id: ActiveValue<RV32iScfiaComposition>,
    context: *mut StepContext<RV32iScfiaComposition>,
) -> ActiveValue<RV32iScfiaComposition> {
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

unsafe fn _execute_add32(
    state: *mut SystemState,
    destination_id: ActiveValue<RV32iScfiaComposition>,
    source1_id: ActiveValue<RV32iScfiaComposition>,
    source2_id: ActiveValue<RV32iScfiaComposition>,
    context: *mut StepContext<RV32iScfiaComposition>,
) {
    let s1: ActiveValue<RV32iScfiaComposition> = _register_read_BV32(state, source1_id.clone(), context);
    let s2: ActiveValue<RV32iScfiaComposition> = _register_read_BV32(state, source2_id.clone(), context);
    let sum: ActiveValue<RV32iScfiaComposition> = (*context).scfia.new_bv_add(s1.clone(), s2.clone(), 32, &mut (*context).fork_sink);
    _register_write_BV32(state, destination_id.clone(), sum.clone(), context);
}
