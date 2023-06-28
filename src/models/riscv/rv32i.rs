#![allow(clippy::all)]
#![allow(non_snake_case)]
#![allow(unused)]
use log::debug;
use std::{borrow::BorrowMut, fmt::Debug, collections::BTreeMap, rc::Rc};

use crate::{memory::Memory, scfia::Scfia, values::{active_value::ActiveValue, retired_value::RetiredValue}, GenericForkSink, ScfiaComposition, StepContext, SymbolicHints};

pub struct RV32i {
    pub state: SystemState,
    pub memory: Memory<RV32iScfiaComposition>,
    pub scfia: Rc<Scfia<RV32iScfiaComposition>>,
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
                new_value.clone_to_stdlib(&clone.scfia, &mut cloned_actives, &mut cloned_retired),
            );
        }
        debug!("fork asserting fork symbol");
        let fork_symbol_id = fork_symbol.get_z3_value().try_borrow().unwrap().id;
        clone.scfia.next_symbol_id.set(fork_symbol_id + 1);
        let cloned_fork_symbol = cloned_actives.get(&fork_symbol_id).unwrap();
        cloned_fork_symbol.assert(&clone.scfia);
        debug!("fork pushing cloned state");
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
                scfia: &self.scfia,
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
                debug!("forking step building step context");
                let mut context = StepContext {
                    memory: &mut state.memory,
                    scfia: &state.scfia,
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
            let cloned_scfia = Scfia::new(Some(self.scfia.next_symbol_id.get()));
            let mut cloned_actives = BTreeMap::new();
            let mut cloned_retireds = BTreeMap::new();
            debug!("cloning scfia {:?} to {:?}",
                Rc::as_ptr(&self.scfia.selff.get().unwrap().upgrade().unwrap()),
                Rc::as_ptr(&cloned_scfia.selff.get().unwrap().upgrade().unwrap()));
            (RV32i {
                state: self.state.clone_to_stdlib(&cloned_scfia, &mut cloned_actives, &mut cloned_retireds),
                memory: self.memory.clone_to_stdlib(&cloned_scfia, &mut cloned_actives, &mut cloned_retireds),
                scfia: cloned_scfia,
            }, cloned_actives, cloned_retireds)
        }
    }

    pub fn debug(&self) {
        debug!("Register depths:");
        debug!("x0:\tdepth={}", self.state.x0.get_depth());
        debug!("x1:\tdepth={}", self.state.x1.get_depth());
        debug!("x2:\tdepth={}", self.state.x2.get_depth());
        debug!("x3:\tdepth={}", self.state.x3.get_depth());
        debug!("x4:\tdepth={}", self.state.x4.get_depth());
        debug!("x5:\tdepth={}", self.state.x5.get_depth());
        debug!("x6:\tdepth={}", self.state.x6.get_depth());
        debug!("x7:\tdepth={}", self.state.x7.get_depth());
        debug!("x8:\tdepth={}", self.state.x8.get_depth());
        debug!("x9:\tdepth={}", self.state.x9.get_depth());
        debug!("x10:\tdepth={}", self.state.x10.get_depth());
        debug!("x11:\tdepth={}", self.state.x11.get_depth());
        debug!("x12:\tdepth={}", self.state.x12.get_depth());
        debug!("x13:\tdepth={}", self.state.x13.get_depth());
        debug!("x14:\tdepth={}", self.state.x14.get_depth());
        debug!("x15:\tdepth={}", self.state.x15.get_depth());
        debug!("x16:\tdepth={}", self.state.x16.get_depth());
        debug!("x17:\tdepth={}", self.state.x17.get_depth());
        debug!("x18:\tdepth={}", self.state.x18.get_depth());
        debug!("x19:\tdepth={}", self.state.x19.get_depth());
        debug!("x20:\tdepth={}", self.state.x20.get_depth());
        debug!("x21:\tdepth={}", self.state.x21.get_depth());
        debug!("x22:\tdepth={}", self.state.x22.get_depth());
        debug!("x23:\tdepth={}", self.state.x23.get_depth());
        debug!("x24:\tdepth={}", self.state.x24.get_depth());
        debug!("x25:\tdepth={}", self.state.x25.get_depth());
        debug!("x26:\tdepth={}", self.state.x26.get_depth());
        debug!("x27:\tdepth={}", self.state.x27.get_depth());
        debug!("x28:\tdepth={}", self.state.x28.get_depth());
        debug!("x29:\tdepth={}", self.state.x29.get_depth());
        debug!("x30:\tdepth={}", self.state.x30.get_depth());
        debug!("x31:\tdepth={}", self.state.x31.get_depth());
        debug!("pc:\tdepth={}", self.state.pc.get_depth());
        debug!("Stable memory cells: {}", self.memory.stables.iter().map(|e|e.memory.len()).sum::<usize>());
        let (worst_cell, depth) = self.memory.get_highest_depth().unwrap();
        debug!("Worst memory cell: {:x} depth={}", worst_cell, depth);
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
    fn clone_to_stdlib(&self, cloned_scfia: &Scfia<RV32iScfiaComposition>, cloned_actives: &mut BTreeMap<u64, ActiveValue<RV32iScfiaComposition>>, cloned_inactives: &mut BTreeMap<u64, RetiredValue<RV32iScfiaComposition>>) -> SystemState {
        SystemState {
            x0: self.x0.clone_to_stdlib(cloned_scfia, cloned_actives, cloned_inactives),
            x1: self.x1.clone_to_stdlib(cloned_scfia, cloned_actives, cloned_inactives),
            x2: self.x2.clone_to_stdlib(cloned_scfia, cloned_actives, cloned_inactives),
            x3: self.x3.clone_to_stdlib(cloned_scfia, cloned_actives, cloned_inactives),
            x4: self.x4.clone_to_stdlib(cloned_scfia, cloned_actives, cloned_inactives),
            x5: self.x5.clone_to_stdlib(cloned_scfia, cloned_actives, cloned_inactives),
            x6: self.x6.clone_to_stdlib(cloned_scfia, cloned_actives, cloned_inactives),
            x7: self.x7.clone_to_stdlib(cloned_scfia, cloned_actives, cloned_inactives),
            x8: self.x8.clone_to_stdlib(cloned_scfia, cloned_actives, cloned_inactives),
            x9: self.x9.clone_to_stdlib(cloned_scfia, cloned_actives, cloned_inactives),
            x10: self.x10.clone_to_stdlib(cloned_scfia, cloned_actives, cloned_inactives),
            x11: self.x11.clone_to_stdlib(cloned_scfia, cloned_actives, cloned_inactives),
            x12: self.x12.clone_to_stdlib(cloned_scfia, cloned_actives, cloned_inactives),
            x13: self.x13.clone_to_stdlib(cloned_scfia, cloned_actives, cloned_inactives),
            x14: self.x14.clone_to_stdlib(cloned_scfia, cloned_actives, cloned_inactives),
            x15: self.x15.clone_to_stdlib(cloned_scfia, cloned_actives, cloned_inactives),
            x16: self.x16.clone_to_stdlib(cloned_scfia, cloned_actives, cloned_inactives),
            x17: self.x17.clone_to_stdlib(cloned_scfia, cloned_actives, cloned_inactives),
            x18: self.x18.clone_to_stdlib(cloned_scfia, cloned_actives, cloned_inactives),
            x19: self.x19.clone_to_stdlib(cloned_scfia, cloned_actives, cloned_inactives),
            x20: self.x20.clone_to_stdlib(cloned_scfia, cloned_actives, cloned_inactives),
            x21: self.x21.clone_to_stdlib(cloned_scfia, cloned_actives, cloned_inactives),
            x22: self.x22.clone_to_stdlib(cloned_scfia, cloned_actives, cloned_inactives),
            x23: self.x23.clone_to_stdlib(cloned_scfia, cloned_actives, cloned_inactives),
            x24: self.x24.clone_to_stdlib(cloned_scfia, cloned_actives, cloned_inactives),
            x25: self.x25.clone_to_stdlib(cloned_scfia, cloned_actives, cloned_inactives),
            x26: self.x26.clone_to_stdlib(cloned_scfia, cloned_actives, cloned_inactives),
            x27: self.x27.clone_to_stdlib(cloned_scfia, cloned_actives, cloned_inactives),
            x28: self.x28.clone_to_stdlib(cloned_scfia, cloned_actives, cloned_inactives),
            x29: self.x29.clone_to_stdlib(cloned_scfia, cloned_actives, cloned_inactives),
            x30: self.x30.clone_to_stdlib(cloned_scfia, cloned_actives, cloned_inactives),
            x31: self.x31.clone_to_stdlib(cloned_scfia, cloned_actives, cloned_inactives),
            pc: self.pc.clone_to_stdlib(cloned_scfia, cloned_actives, cloned_inactives),
        }
    }
}

#[derive(Debug)]
pub struct ComplexSpecialStruct {
    pub some_flag: ActiveValue<RV32iScfiaComposition>,
}

impl ComplexSpecialStruct {
    fn clone_to_stdlib(&self, cloned_scfia: &Scfia<RV32iScfiaComposition>, cloned_actives: &mut BTreeMap<u64, ActiveValue<RV32iScfiaComposition>>, cloned_inactives: &mut BTreeMap<u64, RetiredValue<RV32iScfiaComposition>>) -> ComplexSpecialStruct {
        ComplexSpecialStruct {
            some_flag: self.some_flag.clone_to_stdlib(cloned_scfia, cloned_actives, cloned_inactives),
        }
    }
}

unsafe fn _reset(state: *mut SystemState, context: *mut StepContext<RV32iScfiaComposition>) {
    (*state).x0 = (*context).scfia.new_bv_concrete(0b0, 32);
    (*state).x1 = (*context).scfia.new_bv_concrete(0b0, 32);
    (*state).x2 = (*context).scfia.new_bv_concrete(0b0, 32);
    (*state).x3 = (*context).scfia.new_bv_concrete(0b0, 32);
    (*state).x4 = (*context).scfia.new_bv_concrete(0b0, 32);
    (*state).x5 = (*context).scfia.new_bv_concrete(0b0, 32);
    (*state).x6 = (*context).scfia.new_bv_concrete(0b0, 32);
    (*state).x7 = (*context).scfia.new_bv_concrete(0b0, 32);
    (*state).x8 = (*context).scfia.new_bv_concrete(0b0, 32);
    (*state).x9 = (*context).scfia.new_bv_concrete(0b0, 32);
    (*state).x10 = (*context).scfia.new_bv_concrete(0b0, 32);
    (*state).x11 = (*context).scfia.new_bv_concrete(0b0, 32);
    (*state).x12 = (*context).scfia.new_bv_concrete(0b0, 32);
    (*state).x13 = (*context).scfia.new_bv_concrete(0b0, 32);
    (*state).x14 = (*context).scfia.new_bv_concrete(0b0, 32);
    (*state).x15 = (*context).scfia.new_bv_concrete(0b0, 32);
    (*state).x16 = (*context).scfia.new_bv_concrete(0b0, 32);
    (*state).x17 = (*context).scfia.new_bv_concrete(0b0, 32);
    (*state).x18 = (*context).scfia.new_bv_concrete(0b0, 32);
    (*state).x19 = (*context).scfia.new_bv_concrete(0b0, 32);
    (*state).x20 = (*context).scfia.new_bv_concrete(0b0, 32);
    (*state).x21 = (*context).scfia.new_bv_concrete(0b0, 32);
    (*state).x22 = (*context).scfia.new_bv_concrete(0b0, 32);
    (*state).x23 = (*context).scfia.new_bv_concrete(0b0, 32);
    (*state).x24 = (*context).scfia.new_bv_concrete(0b0, 32);
    (*state).x25 = (*context).scfia.new_bv_concrete(0b0, 32);
    (*state).x26 = (*context).scfia.new_bv_concrete(0b0, 32);
    (*state).x27 = (*context).scfia.new_bv_concrete(0b0, 32);
    (*state).x28 = (*context).scfia.new_bv_concrete(0b0, 32);
    (*state).x29 = (*context).scfia.new_bv_concrete(0b0, 32);
    (*state).x30 = (*context).scfia.new_bv_concrete(0b0, 32);
    (*state).x31 = (*context).scfia.new_bv_concrete(0b0, 32);
}

unsafe fn _sum(state: *mut SystemState, context: *mut StepContext<RV32iScfiaComposition>) -> ActiveValue<RV32iScfiaComposition> {
    return (*context).scfia.new_bv_add(&(*state).x1.clone(), &(*state).x2.clone(), 32, None, &mut (*context).fork_sink, None);
}

unsafe fn _step(state: *mut SystemState, context: *mut StepContext<RV32iScfiaComposition>) {
    let instruction_32: ActiveValue<RV32iScfiaComposition> = (*(*context).memory).read(&(*state).pc.clone(), 32, (*context).scfia.clone(), &mut (*context).hints, &mut (*context).fork_sink);
    let opcode: ActiveValue<RV32iScfiaComposition> = (*context).scfia.new_bv_slice(&instruction_32.clone(), 6, 0, None, &mut (*context).fork_sink, None);
    if (*context).scfia.check_condition(&(*context).scfia.new_bool_eq(&opcode.clone(), &(*context).scfia.new_bv_concrete(0b11, 7), None, false, &mut (*context).fork_sink, None), &mut (*context).fork_sink) {
        let funct3: ActiveValue<RV32iScfiaComposition> = (*context).scfia.new_bv_slice(&instruction_32.clone(), 14, 12, None, &mut (*context).fork_sink, None);
        if (*context).scfia.check_condition(&(*context).scfia.new_bool_eq(&funct3.clone(), &(*context).scfia.new_bv_concrete(0b1, 3), None, false, &mut (*context).fork_sink, None), &mut (*context).fork_sink) {
            let rd: ActiveValue<RV32iScfiaComposition> = _extract_rd_32(instruction_32.clone(), context);
            let rs1: ActiveValue<RV32iScfiaComposition> = _extract_rs1_32(instruction_32.clone(), context);
            let imm: ActiveValue<RV32iScfiaComposition> = (*context).scfia.new_bv_slice(&instruction_32.clone(), 31, 20, None, &mut (*context).fork_sink, None);
            let imm32: ActiveValue<RV32iScfiaComposition> = (*context).scfia.new_bv_sign_extend(&imm.clone(), 12, 32, None, &mut (*context).fork_sink, None);
            let base_address: ActiveValue<RV32iScfiaComposition> = _register_read_BV32(state, rs1.clone(), context);
            let address: ActiveValue<RV32iScfiaComposition> = (*context).scfia.new_bv_add(&base_address.clone(), &imm32.clone(), 32, None, &mut (*context).fork_sink, None);
            let value16: ActiveValue<RV32iScfiaComposition> = (*(*context).memory).read(&address.clone(), 16, (*context).scfia.clone(), &mut (*context).hints, &mut (*context).fork_sink);
            let value: ActiveValue<RV32iScfiaComposition> = (*context).scfia.new_bv_sign_extend(&value16.clone(), 16, 32, None, &mut (*context).fork_sink, None);
            _register_write_BV32(state, rd.clone(), value.clone(), context);
            _progress_pc_4(state, context);
        }
        else if (*context).scfia.check_condition(&(*context).scfia.new_bool_eq(&funct3.clone(), &(*context).scfia.new_bv_concrete(0b10, 3), None, false, &mut (*context).fork_sink, None), &mut (*context).fork_sink) {
            let rd: ActiveValue<RV32iScfiaComposition> = _extract_rd_32(instruction_32.clone(), context);
            let rs1: ActiveValue<RV32iScfiaComposition> = _extract_rs1_32(instruction_32.clone(), context);
            let imm: ActiveValue<RV32iScfiaComposition> = (*context).scfia.new_bv_slice(&instruction_32.clone(), 31, 20, None, &mut (*context).fork_sink, None);
            let imm32: ActiveValue<RV32iScfiaComposition> = (*context).scfia.new_bv_sign_extend(&imm.clone(), 12, 32, None, &mut (*context).fork_sink, None);
            let base_address: ActiveValue<RV32iScfiaComposition> = _register_read_BV32(state, rs1.clone(), context);
            let address: ActiveValue<RV32iScfiaComposition> = (*context).scfia.new_bv_add(&base_address.clone(), &imm32.clone(), 32, None, &mut (*context).fork_sink, None);
            let value: ActiveValue<RV32iScfiaComposition> = (*(*context).memory).read(&address.clone(), 32, (*context).scfia.clone(), &mut (*context).hints, &mut (*context).fork_sink);
            _register_write_BV32(state, rd.clone(), value.clone(), context);
            _progress_pc_4(state, context);
        }
        else if (*context).scfia.check_condition(&(*context).scfia.new_bool_eq(&funct3.clone(), &(*context).scfia.new_bv_concrete(0b100, 3), None, false, &mut (*context).fork_sink, None), &mut (*context).fork_sink) {
            let rd: ActiveValue<RV32iScfiaComposition> = _extract_rd_32(instruction_32.clone(), context);
            let rs1: ActiveValue<RV32iScfiaComposition> = _extract_rs1_32(instruction_32.clone(), context);
            let imm: ActiveValue<RV32iScfiaComposition> = (*context).scfia.new_bv_slice(&instruction_32.clone(), 31, 20, None, &mut (*context).fork_sink, None);
            let imm32: ActiveValue<RV32iScfiaComposition> = (*context).scfia.new_bv_sign_extend(&imm.clone(), 12, 32, None, &mut (*context).fork_sink, None);
            let base_address: ActiveValue<RV32iScfiaComposition> = _register_read_BV32(state, rs1.clone(), context);
            let address: ActiveValue<RV32iScfiaComposition> = (*context).scfia.new_bv_add(&base_address.clone(), &imm32.clone(), 32, None, &mut (*context).fork_sink, None);
            let value8: ActiveValue<RV32iScfiaComposition> = (*(*context).memory).read(&address.clone(), 8, (*context).scfia.clone(), &mut (*context).hints, &mut (*context).fork_sink);
            let value: ActiveValue<RV32iScfiaComposition> = (*context).scfia.new_bv_concat(&(*context).scfia.new_bv_concrete(0b0, 24), &value8.clone(), 32, None, &mut (*context).fork_sink, None);
            _register_write_BV32(state, rd.clone(), value.clone(), context);
            _progress_pc_4(state, context);
        }
        else if (*context).scfia.check_condition(&(*context).scfia.new_bool_eq(&funct3.clone(), &(*context).scfia.new_bv_concrete(0b101, 3), None, false, &mut (*context).fork_sink, None), &mut (*context).fork_sink) {
            let rd: ActiveValue<RV32iScfiaComposition> = _extract_rd_32(instruction_32.clone(), context);
            let rs1: ActiveValue<RV32iScfiaComposition> = _extract_rs1_32(instruction_32.clone(), context);
            let imm: ActiveValue<RV32iScfiaComposition> = (*context).scfia.new_bv_slice(&instruction_32.clone(), 31, 20, None, &mut (*context).fork_sink, None);
            let imm32: ActiveValue<RV32iScfiaComposition> = (*context).scfia.new_bv_sign_extend(&imm.clone(), 12, 32, None, &mut (*context).fork_sink, None);
            let base_address: ActiveValue<RV32iScfiaComposition> = _register_read_BV32(state, rs1.clone(), context);
            let address: ActiveValue<RV32iScfiaComposition> = (*context).scfia.new_bv_add(&base_address.clone(), &imm32.clone(), 32, None, &mut (*context).fork_sink, None);
            let value16: ActiveValue<RV32iScfiaComposition> = (*(*context).memory).read(&address.clone(), 16, (*context).scfia.clone(), &mut (*context).hints, &mut (*context).fork_sink);
            let value: ActiveValue<RV32iScfiaComposition> = (*context).scfia.new_bv_concat(&(*context).scfia.new_bv_concrete(0b0, 16), &value16.clone(), 32, None, &mut (*context).fork_sink, None);
            _register_write_BV32(state, rd.clone(), value.clone(), context);
            _progress_pc_4(state, context);
        }
        else {
            unimplemented!();
        }
    }
    else if (*context).scfia.check_condition(&(*context).scfia.new_bool_eq(&opcode.clone(), &(*context).scfia.new_bv_concrete(0b1111, 7), None, false, &mut (*context).fork_sink, None), &mut (*context).fork_sink) {
        let rd_zeroes: ActiveValue<RV32iScfiaComposition> = (*context).scfia.new_bv_slice(&instruction_32.clone(), 11, 7, None, &mut (*context).fork_sink, None);
        if (*context).scfia.check_condition(&(*context).scfia.new_bool_eq(&rd_zeroes.clone(), &(*context).scfia.new_bv_concrete(0b0, 5), None, false, &mut (*context).fork_sink, None), &mut (*context).fork_sink) {
            let funct3: ActiveValue<RV32iScfiaComposition> = (*context).scfia.new_bv_slice(&instruction_32.clone(), 14, 12, None, &mut (*context).fork_sink, None);
            let rs1_zeroes: ActiveValue<RV32iScfiaComposition> = (*context).scfia.new_bv_slice(&instruction_32.clone(), 19, 15, None, &mut (*context).fork_sink, None);
            if (*context).scfia.check_condition(&(*context).scfia.new_bool_eq(&rs1_zeroes.clone(), &(*context).scfia.new_bv_concrete(0b0, 5), None, false, &mut (*context).fork_sink, None), &mut (*context).fork_sink) {
                if (*context).scfia.check_condition(&(*context).scfia.new_bool_eq(&funct3.clone(), &(*context).scfia.new_bv_concrete(0b0, 3), None, false, &mut (*context).fork_sink, None), &mut (*context).fork_sink) {
                    _progress_pc_4(state, context);
                }
                else if (*context).scfia.check_condition(&(*context).scfia.new_bool_eq(&funct3.clone(), &(*context).scfia.new_bv_concrete(0b1, 3), None, false, &mut (*context).fork_sink, None), &mut (*context).fork_sink) {
                    _progress_pc_4(state, context);
                }
            }
            else {
                unimplemented!();
            }
        }
        else {
            unimplemented!();
        }
    }
    else if (*context).scfia.check_condition(&(*context).scfia.new_bool_eq(&opcode.clone(), &(*context).scfia.new_bv_concrete(0b10011, 7), None, false, &mut (*context).fork_sink, None), &mut (*context).fork_sink) {
        let funct3: ActiveValue<RV32iScfiaComposition> = (*context).scfia.new_bv_slice(&instruction_32.clone(), 14, 12, None, &mut (*context).fork_sink, None);
        if (*context).scfia.check_condition(&(*context).scfia.new_bool_eq(&funct3.clone(), &(*context).scfia.new_bv_concrete(0b0, 3), None, false, &mut (*context).fork_sink, None), &mut (*context).fork_sink) {
            let rd: ActiveValue<RV32iScfiaComposition> = _extract_rd_32(instruction_32.clone(), context);
            let rs1: ActiveValue<RV32iScfiaComposition> = _extract_rs1_32(instruction_32.clone(), context);
            let offset: ActiveValue<RV32iScfiaComposition> = (*context).scfia.new_bv_slice(&instruction_32.clone(), 31, 20, None, &mut (*context).fork_sink, None);
            let offset_32: ActiveValue<RV32iScfiaComposition> = (*context).scfia.new_bv_sign_extend(&offset.clone(), 12, 32, None, &mut (*context).fork_sink, None);
            let result: ActiveValue<RV32iScfiaComposition> = (*context).scfia.new_bv_add(&offset_32.clone(), &_register_read_BV32(state, rs1.clone(), context), 32, None, &mut (*context).fork_sink, None);
            _register_write_BV32(state, rd.clone(), result.clone(), context);
            _progress_pc_4(state, context);
        }
        else if (*context).scfia.check_condition(&(*context).scfia.new_bool_eq(&funct3.clone(), &(*context).scfia.new_bv_concrete(0b1, 3), None, false, &mut (*context).fork_sink, None), &mut (*context).fork_sink) {
            let funct7: ActiveValue<RV32iScfiaComposition> = (*context).scfia.new_bv_slice(&instruction_32.clone(), 31, 25, None, &mut (*context).fork_sink, None);
            if (*context).scfia.check_condition(&(*context).scfia.new_bool_eq(&funct7.clone(), &(*context).scfia.new_bv_concrete(0b0, 7), None, false, &mut (*context).fork_sink, None), &mut (*context).fork_sink) {
                let rd: ActiveValue<RV32iScfiaComposition> = _extract_rd_32(instruction_32.clone(), context);
                let rs1: ActiveValue<RV32iScfiaComposition> = _extract_rs1_32(instruction_32.clone(), context);
                let shamt: ActiveValue<RV32iScfiaComposition> = (*context).scfia.new_bv_slice(&instruction_32.clone(), 24, 20, None, &mut (*context).fork_sink, None);
                let result: ActiveValue<RV32iScfiaComposition> = (*context).scfia.new_bv_sll(&_register_read_BV32(state, rs1.clone(), context), &(*context).scfia.new_bv_concat(&(*context).scfia.new_bv_concrete(0, 27), &shamt.clone(), 32, None, &mut (*context).fork_sink, None), 32, None, &mut (*context).fork_sink, None);
                _register_write_BV32(state, rd.clone(), result.clone(), context);
                _progress_pc_4(state, context);
            }
            else {
                unimplemented!();
            }
        }
        else if (*context).scfia.check_condition(&(*context).scfia.new_bool_eq(&funct3.clone(), &(*context).scfia.new_bv_concrete(0b100, 3), None, false, &mut (*context).fork_sink, None), &mut (*context).fork_sink) {
            let rd: ActiveValue<RV32iScfiaComposition> = _extract_rd_32(instruction_32.clone(), context);
            let rs1: ActiveValue<RV32iScfiaComposition> = _extract_rs1_32(instruction_32.clone(), context);
            let imm: ActiveValue<RV32iScfiaComposition> = (*context).scfia.new_bv_slice(&instruction_32.clone(), 31, 20, None, &mut (*context).fork_sink, None);
            let imm_32: ActiveValue<RV32iScfiaComposition> = (*context).scfia.new_bv_sign_extend(&imm.clone(), 12, 32, None, &mut (*context).fork_sink, None);
            let result: ActiveValue<RV32iScfiaComposition> = (*context).scfia.new_bv_xor(&_register_read_BV32(state, rs1.clone(), context), &imm_32.clone(), 32, None, &mut (*context).fork_sink, None);
            _register_write_BV32(state, rd.clone(), result.clone(), context);
            _progress_pc_4(state, context);
        }
        else if (*context).scfia.check_condition(&(*context).scfia.new_bool_eq(&funct3.clone(), &(*context).scfia.new_bv_concrete(0b101, 3), None, false, &mut (*context).fork_sink, None), &mut (*context).fork_sink) {
            let funct7: ActiveValue<RV32iScfiaComposition> = (*context).scfia.new_bv_slice(&instruction_32.clone(), 31, 25, None, &mut (*context).fork_sink, None);
            if (*context).scfia.check_condition(&(*context).scfia.new_bool_eq(&funct7.clone(), &(*context).scfia.new_bv_concrete(0b0, 7), None, false, &mut (*context).fork_sink, None), &mut (*context).fork_sink) {
                let rd: ActiveValue<RV32iScfiaComposition> = _extract_rd_32(instruction_32.clone(), context);
                let rs1: ActiveValue<RV32iScfiaComposition> = _extract_rs1_32(instruction_32.clone(), context);
                let shamt: ActiveValue<RV32iScfiaComposition> = (*context).scfia.new_bv_slice(&instruction_32.clone(), 24, 20, None, &mut (*context).fork_sink, None);
                let result: ActiveValue<RV32iScfiaComposition> = (*context).scfia.new_bv_srl(&_register_read_BV32(state, rs1.clone(), context), &(*context).scfia.new_bv_concat(&(*context).scfia.new_bv_concrete(0, 27), &shamt.clone(), 32, None, &mut (*context).fork_sink, None), 32, None, &mut (*context).fork_sink, None);
                _register_write_BV32(state, rd.clone(), result.clone(), context);
                _progress_pc_4(state, context);
            }
            else {
                unimplemented!();
            }
        }
        else if (*context).scfia.check_condition(&(*context).scfia.new_bool_eq(&funct3.clone(), &(*context).scfia.new_bv_concrete(0b110, 3), None, false, &mut (*context).fork_sink, None), &mut (*context).fork_sink) {
            let rd: ActiveValue<RV32iScfiaComposition> = _extract_rd_32(instruction_32.clone(), context);
            let rs1: ActiveValue<RV32iScfiaComposition> = _extract_rs1_32(instruction_32.clone(), context);
            let imm: ActiveValue<RV32iScfiaComposition> = (*context).scfia.new_bv_slice(&instruction_32.clone(), 31, 20, None, &mut (*context).fork_sink, None);
            let imm_32: ActiveValue<RV32iScfiaComposition> = (*context).scfia.new_bv_sign_extend(&imm.clone(), 12, 32, None, &mut (*context).fork_sink, None);
            let result: ActiveValue<RV32iScfiaComposition> = (*context).scfia.new_bv_or(&_register_read_BV32(state, rs1.clone(), context), &imm_32.clone(), 32, None, &mut (*context).fork_sink, None);
            _register_write_BV32(state, rd.clone(), result.clone(), context);
            _progress_pc_4(state, context);
        }
        else if (*context).scfia.check_condition(&(*context).scfia.new_bool_eq(&funct3.clone(), &(*context).scfia.new_bv_concrete(0b111, 3), None, false, &mut (*context).fork_sink, None), &mut (*context).fork_sink) {
            let rd: ActiveValue<RV32iScfiaComposition> = _extract_rd_32(instruction_32.clone(), context);
            let rs1: ActiveValue<RV32iScfiaComposition> = _extract_rs1_32(instruction_32.clone(), context);
            let imm: ActiveValue<RV32iScfiaComposition> = (*context).scfia.new_bv_slice(&instruction_32.clone(), 31, 20, None, &mut (*context).fork_sink, None);
            let imm_32: ActiveValue<RV32iScfiaComposition> = (*context).scfia.new_bv_sign_extend(&imm.clone(), 12, 32, None, &mut (*context).fork_sink, None);
            let result: ActiveValue<RV32iScfiaComposition> = (*context).scfia.new_bv_and(&_register_read_BV32(state, rs1.clone(), context), &imm_32.clone(), 32, None, &mut (*context).fork_sink, None);
            _register_write_BV32(state, rd.clone(), result.clone(), context);
            _progress_pc_4(state, context);
        }
        else {
            unimplemented!();
        }
    }
    else if (*context).scfia.check_condition(&(*context).scfia.new_bool_eq(&opcode.clone(), &(*context).scfia.new_bv_concrete(0b100011, 7), None, false, &mut (*context).fork_sink, None), &mut (*context).fork_sink) {
        let funct3: ActiveValue<RV32iScfiaComposition> = (*context).scfia.new_bv_slice(&instruction_32.clone(), 14, 12, None, &mut (*context).fork_sink, None);
        if (*context).scfia.check_condition(&(*context).scfia.new_bool_eq(&funct3.clone(), &(*context).scfia.new_bv_concrete(0b0, 3), None, false, &mut (*context).fork_sink, None), &mut (*context).fork_sink) {
            let rs1: ActiveValue<RV32iScfiaComposition> = _extract_rs1_32(instruction_32.clone(), context);
            let rs2: ActiveValue<RV32iScfiaComposition> = _extract_rs2_32(instruction_32.clone(), context);
            let offset_11_5: ActiveValue<RV32iScfiaComposition> = (*context).scfia.new_bv_slice(&instruction_32.clone(), 31, 25, None, &mut (*context).fork_sink, None);
            let offset_4_0: ActiveValue<RV32iScfiaComposition> = (*context).scfia.new_bv_slice(&instruction_32.clone(), 11, 7, None, &mut (*context).fork_sink, None);
            let offset: ActiveValue<RV32iScfiaComposition> = (*context).scfia.new_bv_concat(&offset_11_5.clone(), &offset_4_0.clone(), 12, None, &mut (*context).fork_sink, None);
            let offset_32: ActiveValue<RV32iScfiaComposition> = (*context).scfia.new_bv_sign_extend(&offset.clone(), 12, 32, None, &mut (*context).fork_sink, None);
            let base_address: ActiveValue<RV32iScfiaComposition> = _register_read_BV32(state, rs1.clone(), context);
            let address: ActiveValue<RV32iScfiaComposition> = (*context).scfia.new_bv_add(&base_address.clone(), &offset_32.clone(), 32, None, &mut (*context).fork_sink, None);
            let value_32: ActiveValue<RV32iScfiaComposition> = _register_read_BV32(state, rs2.clone(), context);
            let value: ActiveValue<RV32iScfiaComposition> = (*context).scfia.new_bv_slice(&value_32.clone(), 7, 0, None, &mut (*context).fork_sink, None);
            (*(*context).memory).write(&address.clone(), &value.clone(), 8, (*context).scfia.clone(), &mut (*context).hints, &mut (*context).fork_sink);
            _progress_pc_4(state, context);
        }
        else if (*context).scfia.check_condition(&(*context).scfia.new_bool_eq(&funct3.clone(), &(*context).scfia.new_bv_concrete(0b1, 3), None, false, &mut (*context).fork_sink, None), &mut (*context).fork_sink) {
            let rs1: ActiveValue<RV32iScfiaComposition> = _extract_rs1_32(instruction_32.clone(), context);
            let rs2: ActiveValue<RV32iScfiaComposition> = _extract_rs2_32(instruction_32.clone(), context);
            let offset_11_5: ActiveValue<RV32iScfiaComposition> = (*context).scfia.new_bv_slice(&instruction_32.clone(), 31, 25, None, &mut (*context).fork_sink, None);
            let offset_4_0: ActiveValue<RV32iScfiaComposition> = (*context).scfia.new_bv_slice(&instruction_32.clone(), 11, 7, None, &mut (*context).fork_sink, None);
            let offset: ActiveValue<RV32iScfiaComposition> = (*context).scfia.new_bv_concat(&offset_11_5.clone(), &offset_4_0.clone(), 12, None, &mut (*context).fork_sink, None);
            let offset_32: ActiveValue<RV32iScfiaComposition> = (*context).scfia.new_bv_sign_extend(&offset.clone(), 12, 32, None, &mut (*context).fork_sink, None);
            let base_address: ActiveValue<RV32iScfiaComposition> = _register_read_BV32(state, rs1.clone(), context);
            let address: ActiveValue<RV32iScfiaComposition> = (*context).scfia.new_bv_add(&base_address.clone(), &offset_32.clone(), 32, None, &mut (*context).fork_sink, None);
            let value_32: ActiveValue<RV32iScfiaComposition> = _register_read_BV32(state, rs2.clone(), context);
            let value: ActiveValue<RV32iScfiaComposition> = (*context).scfia.new_bv_slice(&value_32.clone(), 15, 0, None, &mut (*context).fork_sink, None);
            (*(*context).memory).write(&address.clone(), &value.clone(), 16, (*context).scfia.clone(), &mut (*context).hints, &mut (*context).fork_sink);
            _progress_pc_4(state, context);
        }
        else if (*context).scfia.check_condition(&(*context).scfia.new_bool_eq(&funct3.clone(), &(*context).scfia.new_bv_concrete(0b10, 3), None, false, &mut (*context).fork_sink, None), &mut (*context).fork_sink) {
            let rs1: ActiveValue<RV32iScfiaComposition> = _extract_rs1_32(instruction_32.clone(), context);
            let rs2: ActiveValue<RV32iScfiaComposition> = _extract_rs2_32(instruction_32.clone(), context);
            let offset_11_5: ActiveValue<RV32iScfiaComposition> = (*context).scfia.new_bv_slice(&instruction_32.clone(), 31, 25, None, &mut (*context).fork_sink, None);
            let offset_4_0: ActiveValue<RV32iScfiaComposition> = (*context).scfia.new_bv_slice(&instruction_32.clone(), 11, 7, None, &mut (*context).fork_sink, None);
            let offset: ActiveValue<RV32iScfiaComposition> = (*context).scfia.new_bv_concat(&offset_11_5.clone(), &offset_4_0.clone(), 12, None, &mut (*context).fork_sink, None);
            let offset_32: ActiveValue<RV32iScfiaComposition> = (*context).scfia.new_bv_sign_extend(&offset.clone(), 12, 32, None, &mut (*context).fork_sink, None);
            let base_address: ActiveValue<RV32iScfiaComposition> = _register_read_BV32(state, rs1.clone(), context);
            let address: ActiveValue<RV32iScfiaComposition> = (*context).scfia.new_bv_add(&base_address.clone(), &offset_32.clone(), 32, None, &mut (*context).fork_sink, None);
            let value: ActiveValue<RV32iScfiaComposition> = _register_read_BV32(state, rs2.clone(), context);
            (*(*context).memory).write(&address.clone(), &value.clone(), 32, (*context).scfia.clone(), &mut (*context).hints, &mut (*context).fork_sink);
            _progress_pc_4(state, context);
        }
        else {
            unimplemented!();
        }
    }
    else if (*context).scfia.check_condition(&(*context).scfia.new_bool_eq(&opcode.clone(), &(*context).scfia.new_bv_concrete(0b110111, 7), None, false, &mut (*context).fork_sink, None), &mut (*context).fork_sink) {
        let rd: ActiveValue<RV32iScfiaComposition> = _extract_rd_32(instruction_32.clone(), context);
        let rs: ActiveValue<RV32iScfiaComposition> = _extract_rs1_32(instruction_32.clone(), context);
        let imm: ActiveValue<RV32iScfiaComposition> = (*context).scfia.new_bv_slice(&instruction_32.clone(), 31, 12, None, &mut (*context).fork_sink, None);
        let value: ActiveValue<RV32iScfiaComposition> = (*context).scfia.new_bv_concat(&imm.clone(), &(*context).scfia.new_bv_concrete(0b0, 12), 32, None, &mut (*context).fork_sink, None);
        _register_write_BV32(state, rd.clone(), value.clone(), context);
        _progress_pc_4(state, context);
    }
    else if (*context).scfia.check_condition(&(*context).scfia.new_bool_eq(&opcode.clone(), &(*context).scfia.new_bv_concrete(0b10111, 7), None, false, &mut (*context).fork_sink, None), &mut (*context).fork_sink) {
        let dst: ActiveValue<RV32iScfiaComposition> = _extract_rd_32(instruction_32.clone(), context);
        let imm: ActiveValue<RV32iScfiaComposition> = (*context).scfia.new_bv_slice(&instruction_32.clone(), 31, 12, None, &mut (*context).fork_sink, None);
        let imm32: ActiveValue<RV32iScfiaComposition> = (*context).scfia.new_bv_concat(&imm.clone(), &(*context).scfia.new_bv_concrete(0b0, 12), 32, None, &mut (*context).fork_sink, None);
        let sum: ActiveValue<RV32iScfiaComposition> = (*context).scfia.new_bv_add(&imm32.clone(), &(*state).pc.clone(), 32, None, &mut (*context).fork_sink, None);
        _register_write_BV32(state, dst.clone(), sum.clone(), context);
        _progress_pc_4(state, context);
    }
    else if (*context).scfia.check_condition(&(*context).scfia.new_bool_eq(&opcode.clone(), &(*context).scfia.new_bv_concrete(0b110011, 7), None, false, &mut (*context).fork_sink, None), &mut (*context).fork_sink) {
        let funct3: ActiveValue<RV32iScfiaComposition> = (*context).scfia.new_bv_slice(&instruction_32.clone(), 14, 12, None, &mut (*context).fork_sink, None);
        let funct7: ActiveValue<RV32iScfiaComposition> = (*context).scfia.new_bv_slice(&instruction_32.clone(), 31, 25, None, &mut (*context).fork_sink, None);
        let rd: ActiveValue<RV32iScfiaComposition> = (*context).scfia.new_bv_slice(&instruction_32.clone(), 11, 7, None, &mut (*context).fork_sink, None);
        let rs1: ActiveValue<RV32iScfiaComposition> = (*context).scfia.new_bv_slice(&instruction_32.clone(), 19, 15, None, &mut (*context).fork_sink, None);
        let rs2: ActiveValue<RV32iScfiaComposition> = (*context).scfia.new_bv_slice(&instruction_32.clone(), 24, 20, None, &mut (*context).fork_sink, None);
        if (*context).scfia.check_condition(&(*context).scfia.new_bool_eq(&funct3.clone(), &(*context).scfia.new_bv_concrete(0b0, 3), None, false, &mut (*context).fork_sink, None), &mut (*context).fork_sink) {
            let s1: ActiveValue<RV32iScfiaComposition> = _register_read_BV32(state, rs1.clone(), context);
            let s2: ActiveValue<RV32iScfiaComposition> = _register_read_BV32(state, rs2.clone(), context);
            if (*context).scfia.check_condition(&(*context).scfia.new_bool_eq(&funct7.clone(), &(*context).scfia.new_bv_concrete(0b0, 7), None, false, &mut (*context).fork_sink, None), &mut (*context).fork_sink) {
                _execute_add32(state, rd.clone(), rs1.clone(), rs2.clone(), context);
                _progress_pc_4(state, context);
            }
            else if (*context).scfia.check_condition(&(*context).scfia.new_bool_eq(&funct7.clone(), &(*context).scfia.new_bv_concrete(0b1, 7), None, false, &mut (*context).fork_sink, None), &mut (*context).fork_sink) {
                let result: ActiveValue<RV32iScfiaComposition> = (*context).scfia.new_bv_multiply(&s1.clone(), &s2.clone(), 32, None, &mut (*context).fork_sink, None);
                _register_write_BV32(state, rd.clone(), result.clone(), context);
                _progress_pc_4(state, context);
            }
            else if (*context).scfia.check_condition(&(*context).scfia.new_bool_eq(&funct7.clone(), &(*context).scfia.new_bv_concrete(0b100000, 7), None, false, &mut (*context).fork_sink, None), &mut (*context).fork_sink) {
                let sum: ActiveValue<RV32iScfiaComposition> = (*context).scfia.new_bv_sub(&s1.clone(), &s2.clone(), 32, None, &mut (*context).fork_sink, None);
                _register_write_BV32(state, rd.clone(), sum.clone(), context);
                _progress_pc_4(state, context);
            }
            else {
                unimplemented!();
            }
        }
        else if (*context).scfia.check_condition(&(*context).scfia.new_bool_eq(&funct3.clone(), &(*context).scfia.new_bv_concrete(0b1, 3), None, false, &mut (*context).fork_sink, None), &mut (*context).fork_sink) {
            if (*context).scfia.check_condition(&(*context).scfia.new_bool_eq(&funct7.clone(), &(*context).scfia.new_bv_concrete(0b0, 7), None, false, &mut (*context).fork_sink, None), &mut (*context).fork_sink) {
                let shamt: ActiveValue<RV32iScfiaComposition> = (*context).scfia.new_bv_slice(&_register_read_BV32(state, rs2.clone(), context), 4, 0, None, &mut (*context).fork_sink, None);
                let result: ActiveValue<RV32iScfiaComposition> = (*context).scfia.new_bv_sll(&_register_read_BV32(state, rs1.clone(), context), &(*context).scfia.new_bv_concat(&(*context).scfia.new_bv_concrete(0, 27), &shamt.clone(), 32, None, &mut (*context).fork_sink, None), 32, None, &mut (*context).fork_sink, None);
                _register_write_BV32(state, rd.clone(), result.clone(), context);
                _progress_pc_4(state, context);
            }
            else {
                unimplemented!();
            }
        }
        else if (*context).scfia.check_condition(&(*context).scfia.new_bool_eq(&funct3.clone(), &(*context).scfia.new_bv_concrete(0b10, 3), None, false, &mut (*context).fork_sink, None), &mut (*context).fork_sink) {
            if (*context).scfia.check_condition(&(*context).scfia.new_bool_eq(&funct7.clone(), &(*context).scfia.new_bv_concrete(0b0, 7), None, false, &mut (*context).fork_sink, None), &mut (*context).fork_sink) {
                unimplemented!();
            }
            else {
                unimplemented!();
            }
        }
        else if (*context).scfia.check_condition(&(*context).scfia.new_bool_eq(&funct3.clone(), &(*context).scfia.new_bv_concrete(0b11, 3), None, false, &mut (*context).fork_sink, None), &mut (*context).fork_sink) {
            if (*context).scfia.check_condition(&(*context).scfia.new_bool_eq(&funct7.clone(), &(*context).scfia.new_bv_concrete(0b0, 7), None, false, &mut (*context).fork_sink, None), &mut (*context).fork_sink) {
                if (*context).scfia.check_condition(&(*context).scfia.new_bool_unsigned_less_than(&_register_read_BV32(state, rs1.clone(), context), &_register_read_BV32(state, rs2.clone(), context), None, false, &mut (*context).fork_sink, None), &mut (*context).fork_sink) {
                    _register_write_BV32(state, rd.clone(), (*context).scfia.new_bv_concrete(0b1, 32), context);
                }
                else {
                    _register_write_BV32(state, rd.clone(), (*context).scfia.new_bv_concrete(0b0, 32), context);
                }
                _progress_pc_4(state, context);
            }
            else {
                unimplemented!();
            }
        }
        else if (*context).scfia.check_condition(&(*context).scfia.new_bool_eq(&funct3.clone(), &(*context).scfia.new_bv_concrete(0b100, 3), None, false, &mut (*context).fork_sink, None), &mut (*context).fork_sink) {
            if (*context).scfia.check_condition(&(*context).scfia.new_bool_eq(&funct7.clone(), &(*context).scfia.new_bv_concrete(0b0, 7), None, false, &mut (*context).fork_sink, None), &mut (*context).fork_sink) {
                unimplemented!();
            }
            else {
                unimplemented!();
            }
        }
        else if (*context).scfia.check_condition(&(*context).scfia.new_bool_eq(&funct3.clone(), &(*context).scfia.new_bv_concrete(0b101, 3), None, false, &mut (*context).fork_sink, None), &mut (*context).fork_sink) {
            if (*context).scfia.check_condition(&(*context).scfia.new_bool_eq(&funct7.clone(), &(*context).scfia.new_bv_concrete(0b0, 7), None, false, &mut (*context).fork_sink, None), &mut (*context).fork_sink) {
                let shamt: ActiveValue<RV32iScfiaComposition> = (*context).scfia.new_bv_slice(&_register_read_BV32(state, rs2.clone(), context), 4, 0, None, &mut (*context).fork_sink, None);
                let result: ActiveValue<RV32iScfiaComposition> = (*context).scfia.new_bv_srl(&_register_read_BV32(state, rs1.clone(), context), &(*context).scfia.new_bv_concat(&(*context).scfia.new_bv_concrete(0, 27), &shamt.clone(), 32, None, &mut (*context).fork_sink, None), 32, None, &mut (*context).fork_sink, None);
                _register_write_BV32(state, rd.clone(), result.clone(), context);
                _progress_pc_4(state, context);
            }
            else if (*context).scfia.check_condition(&(*context).scfia.new_bool_eq(&funct7.clone(), &(*context).scfia.new_bv_concrete(0b100000, 7), None, false, &mut (*context).fork_sink, None), &mut (*context).fork_sink) {
                unimplemented!();
            }
            else {
                unimplemented!();
            }
        }
        else if (*context).scfia.check_condition(&(*context).scfia.new_bool_eq(&funct3.clone(), &(*context).scfia.new_bv_concrete(0b110, 3), None, false, &mut (*context).fork_sink, None), &mut (*context).fork_sink) {
            if (*context).scfia.check_condition(&(*context).scfia.new_bool_eq(&funct7.clone(), &(*context).scfia.new_bv_concrete(0b0, 7), None, false, &mut (*context).fork_sink, None), &mut (*context).fork_sink) {
                let rd: ActiveValue<RV32iScfiaComposition> = _extract_rd_32(instruction_32.clone(), context);
                let rs1: ActiveValue<RV32iScfiaComposition> = _extract_rs1_32(instruction_32.clone(), context);
                let rs2: ActiveValue<RV32iScfiaComposition> = _extract_rs2_32(instruction_32.clone(), context);
                let result: ActiveValue<RV32iScfiaComposition> = (*context).scfia.new_bv_or(&_register_read_BV32(state, rs1.clone(), context), &_register_read_BV32(state, rs2.clone(), context), 32, None, &mut (*context).fork_sink, None);
                _register_write_BV32(state, rd.clone(), result.clone(), context);
                _progress_pc_4(state, context);
            }
            else {
                unimplemented!();
            }
        }
        else if (*context).scfia.check_condition(&(*context).scfia.new_bool_eq(&funct3.clone(), &(*context).scfia.new_bv_concrete(0b111, 3), None, false, &mut (*context).fork_sink, None), &mut (*context).fork_sink) {
            let rd: ActiveValue<RV32iScfiaComposition> = _extract_rd_32(instruction_32.clone(), context);
            let rs1: ActiveValue<RV32iScfiaComposition> = _extract_rs1_32(instruction_32.clone(), context);
            let rs2: ActiveValue<RV32iScfiaComposition> = _extract_rs2_32(instruction_32.clone(), context);
            if (*context).scfia.check_condition(&(*context).scfia.new_bool_eq(&funct7.clone(), &(*context).scfia.new_bv_concrete(0b0, 7), None, false, &mut (*context).fork_sink, None), &mut (*context).fork_sink) {
                let result: ActiveValue<RV32iScfiaComposition> = (*context).scfia.new_bv_and(&_register_read_BV32(state, rs1.clone(), context), &_register_read_BV32(state, rs2.clone(), context), 32, None, &mut (*context).fork_sink, None);
                _register_write_BV32(state, rd.clone(), result.clone(), context);
                _progress_pc_4(state, context);
            }
            else if (*context).scfia.check_condition(&(*context).scfia.new_bool_eq(&funct7.clone(), &(*context).scfia.new_bv_concrete(0b1, 7), None, false, &mut (*context).fork_sink, None), &mut (*context).fork_sink) {
                let result: ActiveValue<RV32iScfiaComposition> = (*context).scfia.new_bv_unsigned_remainder(&_register_read_BV32(state, rs1.clone(), context), &_register_read_BV32(state, rs2.clone(), context), 32, None, &mut (*context).fork_sink, None);
                _register_write_BV32(state, rd.clone(), result.clone(), context);
                _progress_pc_4(state, context);
            }
            else {
                unimplemented!();
            }
        }
        else {
            unimplemented!();
        }
    }
    else if (*context).scfia.check_condition(&(*context).scfia.new_bool_eq(&opcode.clone(), &(*context).scfia.new_bv_concrete(0b1100011, 7), None, false, &mut (*context).fork_sink, None), &mut (*context).fork_sink) {
        let funct3: ActiveValue<RV32iScfiaComposition> = (*context).scfia.new_bv_slice(&instruction_32.clone(), 14, 12, None, &mut (*context).fork_sink, None);
        if (*context).scfia.check_condition(&(*context).scfia.new_bool_eq(&funct3.clone(), &(*context).scfia.new_bv_concrete(0b0, 3), None, false, &mut (*context).fork_sink, None), &mut (*context).fork_sink) {
            let lhs: ActiveValue<RV32iScfiaComposition> = _register_read_BV32(state, _extract_rs1_32(instruction_32.clone(), context), context);
            let rhs: ActiveValue<RV32iScfiaComposition> = _register_read_BV32(state, _extract_rs2_32(instruction_32.clone(), context), context);
            if (*context).scfia.check_condition(&(*context).scfia.new_bool_eq(&lhs.clone(), &rhs.clone(), None, false, &mut (*context).fork_sink, None), &mut (*context).fork_sink) {
                let imm_4_1: ActiveValue<RV32iScfiaComposition> = (*context).scfia.new_bv_slice(&instruction_32.clone(), 11, 8, None, &mut (*context).fork_sink, None);
                let imm_10_5: ActiveValue<RV32iScfiaComposition> = (*context).scfia.new_bv_slice(&instruction_32.clone(), 30, 25, None, &mut (*context).fork_sink, None);
                let imm11: ActiveValue<RV32iScfiaComposition> = (*context).scfia.new_bv_slice(&instruction_32.clone(), 7, 7, None, &mut (*context).fork_sink, None);
                let imm12: ActiveValue<RV32iScfiaComposition> = (*context).scfia.new_bv_slice(&instruction_32.clone(), 31, 31, None, &mut (*context).fork_sink, None);
                let imm_4_0: ActiveValue<RV32iScfiaComposition> = (*context).scfia.new_bv_concat(&imm_4_1.clone(), &(*context).scfia.new_bv_concrete(0b0, 1), 5, None, &mut (*context).fork_sink, None);
                let imm_10_0: ActiveValue<RV32iScfiaComposition> = (*context).scfia.new_bv_concat(&imm_10_5.clone(), &imm_4_0.clone(), 11, None, &mut (*context).fork_sink, None);
                let imm_11_0: ActiveValue<RV32iScfiaComposition> = (*context).scfia.new_bv_concat(&imm11.clone(), &imm_10_0.clone(), 12, None, &mut (*context).fork_sink, None);
                let imm_12_0: ActiveValue<RV32iScfiaComposition> = (*context).scfia.new_bv_concat(&imm12.clone(), &imm_11_0.clone(), 13, None, &mut (*context).fork_sink, None);
                let offset: ActiveValue<RV32iScfiaComposition> = (*context).scfia.new_bv_sign_extend(&imm_12_0.clone(), 13, 32, None, &mut (*context).fork_sink, None);
                let address: ActiveValue<RV32iScfiaComposition> = (*context).scfia.new_bv_add(&(*state).pc.clone(), &offset.clone(), 32, None, &mut (*context).fork_sink, None);
                (*state).pc = address.clone();
            }
            else {
                _progress_pc_4(state, context);
            }
        }
        else if (*context).scfia.check_condition(&(*context).scfia.new_bool_eq(&funct3.clone(), &(*context).scfia.new_bv_concrete(0b1, 3), None, false, &mut (*context).fork_sink, None), &mut (*context).fork_sink) {
            let lhs: ActiveValue<RV32iScfiaComposition> = _register_read_BV32(state, _extract_rs1_32(instruction_32.clone(), context), context);
            let rhs: ActiveValue<RV32iScfiaComposition> = _register_read_BV32(state, _extract_rs2_32(instruction_32.clone(), context), context);
            if (*context).scfia.check_condition(&(*context).scfia.new_bool_not(&(*context).scfia.new_bool_eq(&lhs.clone(), &rhs.clone(), None, false, &mut (*context).fork_sink, None), None, false, &mut (*context).fork_sink, None), &mut (*context).fork_sink) {
                let imm_4_1: ActiveValue<RV32iScfiaComposition> = (*context).scfia.new_bv_slice(&instruction_32.clone(), 11, 8, None, &mut (*context).fork_sink, None);
                let imm_10_5: ActiveValue<RV32iScfiaComposition> = (*context).scfia.new_bv_slice(&instruction_32.clone(), 30, 25, None, &mut (*context).fork_sink, None);
                let imm11: ActiveValue<RV32iScfiaComposition> = (*context).scfia.new_bv_slice(&instruction_32.clone(), 7, 7, None, &mut (*context).fork_sink, None);
                let imm12: ActiveValue<RV32iScfiaComposition> = (*context).scfia.new_bv_slice(&instruction_32.clone(), 31, 31, None, &mut (*context).fork_sink, None);
                let imm_4_0: ActiveValue<RV32iScfiaComposition> = (*context).scfia.new_bv_concat(&imm_4_1.clone(), &(*context).scfia.new_bv_concrete(0b0, 1), 5, None, &mut (*context).fork_sink, None);
                let imm_10_0: ActiveValue<RV32iScfiaComposition> = (*context).scfia.new_bv_concat(&imm_10_5.clone(), &imm_4_0.clone(), 11, None, &mut (*context).fork_sink, None);
                let imm_11_0: ActiveValue<RV32iScfiaComposition> = (*context).scfia.new_bv_concat(&imm11.clone(), &imm_10_0.clone(), 12, None, &mut (*context).fork_sink, None);
                let imm_12_0: ActiveValue<RV32iScfiaComposition> = (*context).scfia.new_bv_concat(&imm12.clone(), &imm_11_0.clone(), 13, None, &mut (*context).fork_sink, None);
                let offset: ActiveValue<RV32iScfiaComposition> = (*context).scfia.new_bv_sign_extend(&imm_12_0.clone(), 13, 32, None, &mut (*context).fork_sink, None);
                let address: ActiveValue<RV32iScfiaComposition> = (*context).scfia.new_bv_add(&(*state).pc.clone(), &offset.clone(), 32, None, &mut (*context).fork_sink, None);
                (*state).pc = address.clone();
            }
            else {
                _progress_pc_4(state, context);
            }
        }
        else if (*context).scfia.check_condition(&(*context).scfia.new_bool_eq(&funct3.clone(), &(*context).scfia.new_bv_concrete(0b100, 3), None, false, &mut (*context).fork_sink, None), &mut (*context).fork_sink) {
            let lhs: ActiveValue<RV32iScfiaComposition> = _register_read_BV32(state, _extract_rs1_32(instruction_32.clone(), context), context);
            let rhs: ActiveValue<RV32iScfiaComposition> = _register_read_BV32(state, _extract_rs2_32(instruction_32.clone(), context), context);
            if (*context).scfia.check_condition(&(*context).scfia.new_bool_signed_less_than(&lhs.clone(), &rhs.clone(), None, false, &mut (*context).fork_sink, None), &mut (*context).fork_sink) {
                let imm_4_1: ActiveValue<RV32iScfiaComposition> = (*context).scfia.new_bv_slice(&instruction_32.clone(), 11, 8, None, &mut (*context).fork_sink, None);
                let imm_10_5: ActiveValue<RV32iScfiaComposition> = (*context).scfia.new_bv_slice(&instruction_32.clone(), 30, 25, None, &mut (*context).fork_sink, None);
                let imm11: ActiveValue<RV32iScfiaComposition> = (*context).scfia.new_bv_slice(&instruction_32.clone(), 7, 7, None, &mut (*context).fork_sink, None);
                let imm12: ActiveValue<RV32iScfiaComposition> = (*context).scfia.new_bv_slice(&instruction_32.clone(), 31, 31, None, &mut (*context).fork_sink, None);
                let imm_4_0: ActiveValue<RV32iScfiaComposition> = (*context).scfia.new_bv_concat(&imm_4_1.clone(), &(*context).scfia.new_bv_concrete(0b0, 1), 5, None, &mut (*context).fork_sink, None);
                let imm_10_0: ActiveValue<RV32iScfiaComposition> = (*context).scfia.new_bv_concat(&imm_10_5.clone(), &imm_4_0.clone(), 11, None, &mut (*context).fork_sink, None);
                let imm_11_0: ActiveValue<RV32iScfiaComposition> = (*context).scfia.new_bv_concat(&imm11.clone(), &imm_10_0.clone(), 12, None, &mut (*context).fork_sink, None);
                let imm_12_0: ActiveValue<RV32iScfiaComposition> = (*context).scfia.new_bv_concat(&imm12.clone(), &imm_11_0.clone(), 13, None, &mut (*context).fork_sink, None);
                let offset: ActiveValue<RV32iScfiaComposition> = (*context).scfia.new_bv_sign_extend(&imm_12_0.clone(), 13, 32, None, &mut (*context).fork_sink, None);
                let address: ActiveValue<RV32iScfiaComposition> = (*context).scfia.new_bv_add(&(*state).pc.clone(), &offset.clone(), 32, None, &mut (*context).fork_sink, None);
                (*state).pc = address.clone();
            }
            else {
                _progress_pc_4(state, context);
            }
        }
        else if (*context).scfia.check_condition(&(*context).scfia.new_bool_eq(&funct3.clone(), &(*context).scfia.new_bv_concrete(0b101, 3), None, false, &mut (*context).fork_sink, None), &mut (*context).fork_sink) {
            let lhs: ActiveValue<RV32iScfiaComposition> = _register_read_BV32(state, _extract_rs1_32(instruction_32.clone(), context), context);
            let rhs: ActiveValue<RV32iScfiaComposition> = _register_read_BV32(state, _extract_rs2_32(instruction_32.clone(), context), context);
            if (*context).scfia.check_condition(&(*context).scfia.new_bool_signed_less_than(&lhs.clone(), &rhs.clone(), None, false, &mut (*context).fork_sink, None), &mut (*context).fork_sink) {
                _progress_pc_4(state, context);
            }
            else {
                let imm_4_1: ActiveValue<RV32iScfiaComposition> = (*context).scfia.new_bv_slice(&instruction_32.clone(), 11, 8, None, &mut (*context).fork_sink, None);
                let imm_10_5: ActiveValue<RV32iScfiaComposition> = (*context).scfia.new_bv_slice(&instruction_32.clone(), 30, 25, None, &mut (*context).fork_sink, None);
                let imm11: ActiveValue<RV32iScfiaComposition> = (*context).scfia.new_bv_slice(&instruction_32.clone(), 7, 7, None, &mut (*context).fork_sink, None);
                let imm12: ActiveValue<RV32iScfiaComposition> = (*context).scfia.new_bv_slice(&instruction_32.clone(), 31, 31, None, &mut (*context).fork_sink, None);
                let imm_4_0: ActiveValue<RV32iScfiaComposition> = (*context).scfia.new_bv_concat(&imm_4_1.clone(), &(*context).scfia.new_bv_concrete(0b0, 1), 5, None, &mut (*context).fork_sink, None);
                let imm_10_0: ActiveValue<RV32iScfiaComposition> = (*context).scfia.new_bv_concat(&imm_10_5.clone(), &imm_4_0.clone(), 11, None, &mut (*context).fork_sink, None);
                let imm_11_0: ActiveValue<RV32iScfiaComposition> = (*context).scfia.new_bv_concat(&imm11.clone(), &imm_10_0.clone(), 12, None, &mut (*context).fork_sink, None);
                let imm_12_0: ActiveValue<RV32iScfiaComposition> = (*context).scfia.new_bv_concat(&imm12.clone(), &imm_11_0.clone(), 13, None, &mut (*context).fork_sink, None);
                let offset: ActiveValue<RV32iScfiaComposition> = (*context).scfia.new_bv_sign_extend(&imm_12_0.clone(), 13, 32, None, &mut (*context).fork_sink, None);
                let address: ActiveValue<RV32iScfiaComposition> = (*context).scfia.new_bv_add(&(*state).pc.clone(), &offset.clone(), 32, None, &mut (*context).fork_sink, None);
                (*state).pc = address.clone();
            }
        }
        else if (*context).scfia.check_condition(&(*context).scfia.new_bool_eq(&funct3.clone(), &(*context).scfia.new_bv_concrete(0b110, 3), None, false, &mut (*context).fork_sink, None), &mut (*context).fork_sink) {
            let lhs: ActiveValue<RV32iScfiaComposition> = _register_read_BV32(state, _extract_rs1_32(instruction_32.clone(), context), context);
            let rhs: ActiveValue<RV32iScfiaComposition> = _register_read_BV32(state, _extract_rs2_32(instruction_32.clone(), context), context);
            if (*context).scfia.check_condition(&(*context).scfia.new_bool_unsigned_less_than(&lhs.clone(), &rhs.clone(), None, false, &mut (*context).fork_sink, None), &mut (*context).fork_sink) {
                let imm_4_1: ActiveValue<RV32iScfiaComposition> = (*context).scfia.new_bv_slice(&instruction_32.clone(), 11, 8, None, &mut (*context).fork_sink, None);
                let imm_10_5: ActiveValue<RV32iScfiaComposition> = (*context).scfia.new_bv_slice(&instruction_32.clone(), 30, 25, None, &mut (*context).fork_sink, None);
                let imm11: ActiveValue<RV32iScfiaComposition> = (*context).scfia.new_bv_slice(&instruction_32.clone(), 7, 7, None, &mut (*context).fork_sink, None);
                let imm12: ActiveValue<RV32iScfiaComposition> = (*context).scfia.new_bv_slice(&instruction_32.clone(), 31, 31, None, &mut (*context).fork_sink, None);
                let imm_4_0: ActiveValue<RV32iScfiaComposition> = (*context).scfia.new_bv_concat(&imm_4_1.clone(), &(*context).scfia.new_bv_concrete(0b0, 1), 5, None, &mut (*context).fork_sink, None);
                let imm_10_0: ActiveValue<RV32iScfiaComposition> = (*context).scfia.new_bv_concat(&imm_10_5.clone(), &imm_4_0.clone(), 11, None, &mut (*context).fork_sink, None);
                let imm_11_0: ActiveValue<RV32iScfiaComposition> = (*context).scfia.new_bv_concat(&imm11.clone(), &imm_10_0.clone(), 12, None, &mut (*context).fork_sink, None);
                let imm_12_0: ActiveValue<RV32iScfiaComposition> = (*context).scfia.new_bv_concat(&imm12.clone(), &imm_11_0.clone(), 13, None, &mut (*context).fork_sink, None);
                let offset: ActiveValue<RV32iScfiaComposition> = (*context).scfia.new_bv_sign_extend(&imm_12_0.clone(), 13, 32, None, &mut (*context).fork_sink, None);
                let address: ActiveValue<RV32iScfiaComposition> = (*context).scfia.new_bv_add(&(*state).pc.clone(), &offset.clone(), 32, None, &mut (*context).fork_sink, None);
                (*state).pc = address.clone();
            }
            else {
                _progress_pc_4(state, context);
            }
        }
        else if (*context).scfia.check_condition(&(*context).scfia.new_bool_eq(&funct3.clone(), &(*context).scfia.new_bv_concrete(0b111, 3), None, false, &mut (*context).fork_sink, None), &mut (*context).fork_sink) {
            let lhs: ActiveValue<RV32iScfiaComposition> = _register_read_BV32(state, _extract_rs1_32(instruction_32.clone(), context), context);
            let rhs: ActiveValue<RV32iScfiaComposition> = _register_read_BV32(state, _extract_rs2_32(instruction_32.clone(), context), context);
            if (*context).scfia.check_condition(&(*context).scfia.new_bool_unsigned_less_than(&lhs.clone(), &rhs.clone(), None, false, &mut (*context).fork_sink, None), &mut (*context).fork_sink) {
                _progress_pc_4(state, context);
            }
            else {
                let imm_4_1: ActiveValue<RV32iScfiaComposition> = (*context).scfia.new_bv_slice(&instruction_32.clone(), 11, 8, None, &mut (*context).fork_sink, None);
                let imm_10_5: ActiveValue<RV32iScfiaComposition> = (*context).scfia.new_bv_slice(&instruction_32.clone(), 30, 25, None, &mut (*context).fork_sink, None);
                let imm11: ActiveValue<RV32iScfiaComposition> = (*context).scfia.new_bv_slice(&instruction_32.clone(), 7, 7, None, &mut (*context).fork_sink, None);
                let imm12: ActiveValue<RV32iScfiaComposition> = (*context).scfia.new_bv_slice(&instruction_32.clone(), 31, 31, None, &mut (*context).fork_sink, None);
                let imm_4_0: ActiveValue<RV32iScfiaComposition> = (*context).scfia.new_bv_concat(&imm_4_1.clone(), &(*context).scfia.new_bv_concrete(0b0, 1), 5, None, &mut (*context).fork_sink, None);
                let imm_10_0: ActiveValue<RV32iScfiaComposition> = (*context).scfia.new_bv_concat(&imm_10_5.clone(), &imm_4_0.clone(), 11, None, &mut (*context).fork_sink, None);
                let imm_11_0: ActiveValue<RV32iScfiaComposition> = (*context).scfia.new_bv_concat(&imm11.clone(), &imm_10_0.clone(), 12, None, &mut (*context).fork_sink, None);
                let imm_12_0: ActiveValue<RV32iScfiaComposition> = (*context).scfia.new_bv_concat(&imm12.clone(), &imm_11_0.clone(), 13, None, &mut (*context).fork_sink, None);
                let offset: ActiveValue<RV32iScfiaComposition> = (*context).scfia.new_bv_sign_extend(&imm_12_0.clone(), 13, 32, None, &mut (*context).fork_sink, None);
                let address: ActiveValue<RV32iScfiaComposition> = (*context).scfia.new_bv_add(&(*state).pc.clone(), &offset.clone(), 32, None, &mut (*context).fork_sink, None);
                (*state).pc = address.clone();
            }
        }
        else {
            unimplemented!();
        }
    }
    else if (*context).scfia.check_condition(&(*context).scfia.new_bool_eq(&opcode.clone(), &(*context).scfia.new_bv_concrete(0b1100111, 7), None, false, &mut (*context).fork_sink, None), &mut (*context).fork_sink) {
        let funct3: ActiveValue<RV32iScfiaComposition> = (*context).scfia.new_bv_slice(&instruction_32.clone(), 14, 12, None, &mut (*context).fork_sink, None);
        if (*context).scfia.check_condition(&(*context).scfia.new_bool_eq(&funct3.clone(), &(*context).scfia.new_bv_concrete(0b0, 3), None, false, &mut (*context).fork_sink, None), &mut (*context).fork_sink) {
            let dst: ActiveValue<RV32iScfiaComposition> = _extract_rd_32(instruction_32.clone(), context);
            let rs1: ActiveValue<RV32iScfiaComposition> = _extract_rs1_32(instruction_32.clone(), context);
            let s1: ActiveValue<RV32iScfiaComposition> = _register_read_BV32(state, rs1.clone(), context);
            let offset: ActiveValue<RV32iScfiaComposition> = (*context).scfia.new_bv_slice(&instruction_32.clone(), 31, 20, None, &mut (*context).fork_sink, None);
            let offset_32: ActiveValue<RV32iScfiaComposition> = (*context).scfia.new_bv_sign_extend(&offset.clone(), 12, 32, None, &mut (*context).fork_sink, None);
            let address: ActiveValue<RV32iScfiaComposition> = (*context).scfia.new_bv_add(&s1.clone(), &offset_32.clone(), 32, None, &mut (*context).fork_sink, None);
            let return_address: ActiveValue<RV32iScfiaComposition> = (*context).scfia.new_bv_add(&(*state).pc.clone(), &(*context).scfia.new_bv_concrete(0b100, 32), 32, None, &mut (*context).fork_sink, None);
            (*state).pc = address.clone();
            _register_write_BV32(state, dst.clone(), return_address.clone(), context);
        }
        else {
            unimplemented!();
        }
    }
    else if (*context).scfia.check_condition(&(*context).scfia.new_bool_eq(&opcode.clone(), &(*context).scfia.new_bv_concrete(0b1101111, 7), None, false, &mut (*context).fork_sink, None), &mut (*context).fork_sink) {
        let imm_20: ActiveValue<RV32iScfiaComposition> = (*context).scfia.new_bv_slice(&instruction_32.clone(), 31, 31, None, &mut (*context).fork_sink, None);
        let imm_10_1: ActiveValue<RV32iScfiaComposition> = (*context).scfia.new_bv_slice(&instruction_32.clone(), 30, 21, None, &mut (*context).fork_sink, None);
        let imm_11: ActiveValue<RV32iScfiaComposition> = (*context).scfia.new_bv_slice(&instruction_32.clone(), 20, 20, None, &mut (*context).fork_sink, None);
        let imm_19_12: ActiveValue<RV32iScfiaComposition> = (*context).scfia.new_bv_slice(&instruction_32.clone(), 19, 12, None, &mut (*context).fork_sink, None);
        let offset_10_0: ActiveValue<RV32iScfiaComposition> = (*context).scfia.new_bv_concat(&imm_10_1.clone(), &(*context).scfia.new_bv_concrete(0b0, 1), 11, None, &mut (*context).fork_sink, None);
        let offset_11_0: ActiveValue<RV32iScfiaComposition> = (*context).scfia.new_bv_concat(&imm_11.clone(), &offset_10_0.clone(), 12, None, &mut (*context).fork_sink, None);
        let offset_19_0: ActiveValue<RV32iScfiaComposition> = (*context).scfia.new_bv_concat(&imm_19_12.clone(), &offset_11_0.clone(), 20, None, &mut (*context).fork_sink, None);
        let offset_20_0: ActiveValue<RV32iScfiaComposition> = (*context).scfia.new_bv_concat(&imm_20.clone(), &offset_19_0.clone(), 21, None, &mut (*context).fork_sink, None);
        let offset_32: ActiveValue<RV32iScfiaComposition> = (*context).scfia.new_bv_sign_extend(&offset_20_0.clone(), 21, 32, None, &mut (*context).fork_sink, None);
        let address: ActiveValue<RV32iScfiaComposition> = (*context).scfia.new_bv_add(&(*state).pc.clone(), &offset_32.clone(), 32, None, &mut (*context).fork_sink, None);
        let return_address: ActiveValue<RV32iScfiaComposition> = (*context).scfia.new_bv_add(&(*state).pc.clone(), &(*context).scfia.new_bv_concrete(0b100, 32), 32, None, &mut (*context).fork_sink, None);
        let dst: ActiveValue<RV32iScfiaComposition> = _extract_rd_32(instruction_32.clone(), context);
        _register_write_BV32(state, dst.clone(), return_address.clone(), context);
        (*state).pc = address.clone();
    }
    else {
        unimplemented!();
    }
}

unsafe fn _extract_rd_32(op: ActiveValue<RV32iScfiaComposition>, context: *mut StepContext<RV32iScfiaComposition>) -> ActiveValue<RV32iScfiaComposition> {
    return (*context).scfia.new_bv_slice(&op.clone(), 11, 7, None, &mut (*context).fork_sink, None);
}

unsafe fn _extract_rs1_32(op: ActiveValue<RV32iScfiaComposition>, context: *mut StepContext<RV32iScfiaComposition>) -> ActiveValue<RV32iScfiaComposition> {
    return (*context).scfia.new_bv_slice(&op.clone(), 19, 15, None, &mut (*context).fork_sink, None);
}

unsafe fn _extract_rs2_32(op: ActiveValue<RV32iScfiaComposition>, context: *mut StepContext<RV32iScfiaComposition>) -> ActiveValue<RV32iScfiaComposition> {
    return (*context).scfia.new_bv_slice(&op.clone(), 24, 20, None, &mut (*context).fork_sink, None);
}

unsafe fn _progress_pc_4(state: *mut SystemState, context: *mut StepContext<RV32iScfiaComposition>) {
    let old_pc: ActiveValue<RV32iScfiaComposition> = (*state).pc.clone();
    let new_pc: ActiveValue<RV32iScfiaComposition> = (*context).scfia.new_bv_add(&old_pc.clone(), &(*context).scfia.new_bv_concrete(0b100, 32), 32, None, &mut (*context).fork_sink, None);
    (*state).pc = new_pc.clone();
}

unsafe fn _register_write_BV32(state: *mut SystemState, register_id: ActiveValue<RV32iScfiaComposition>, value: ActiveValue<RV32iScfiaComposition>, context: *mut StepContext<RV32iScfiaComposition>) {
    if (*context).scfia.check_condition(&(*context).scfia.new_bool_eq(&register_id.clone(), &(*context).scfia.new_bv_concrete(0b0, 5), None, false, &mut (*context).fork_sink, None), &mut (*context).fork_sink) {
    }
    else if (*context).scfia.check_condition(&(*context).scfia.new_bool_eq(&register_id.clone(), &(*context).scfia.new_bv_concrete(0b1, 5), None, false, &mut (*context).fork_sink, None), &mut (*context).fork_sink) {
        (*state).x1 = value.clone();
    }
    else if (*context).scfia.check_condition(&(*context).scfia.new_bool_eq(&register_id.clone(), &(*context).scfia.new_bv_concrete(0b10, 5), None, false, &mut (*context).fork_sink, None), &mut (*context).fork_sink) {
        (*state).x2 = value.clone();
    }
    else if (*context).scfia.check_condition(&(*context).scfia.new_bool_eq(&register_id.clone(), &(*context).scfia.new_bv_concrete(0b11, 5), None, false, &mut (*context).fork_sink, None), &mut (*context).fork_sink) {
        (*state).x3 = value.clone();
    }
    else if (*context).scfia.check_condition(&(*context).scfia.new_bool_eq(&register_id.clone(), &(*context).scfia.new_bv_concrete(0b100, 5), None, false, &mut (*context).fork_sink, None), &mut (*context).fork_sink) {
        (*state).x4 = value.clone();
    }
    else if (*context).scfia.check_condition(&(*context).scfia.new_bool_eq(&register_id.clone(), &(*context).scfia.new_bv_concrete(0b101, 5), None, false, &mut (*context).fork_sink, None), &mut (*context).fork_sink) {
        (*state).x5 = value.clone();
    }
    else if (*context).scfia.check_condition(&(*context).scfia.new_bool_eq(&register_id.clone(), &(*context).scfia.new_bv_concrete(0b110, 5), None, false, &mut (*context).fork_sink, None), &mut (*context).fork_sink) {
        (*state).x6 = value.clone();
    }
    else if (*context).scfia.check_condition(&(*context).scfia.new_bool_eq(&register_id.clone(), &(*context).scfia.new_bv_concrete(0b111, 5), None, false, &mut (*context).fork_sink, None), &mut (*context).fork_sink) {
        (*state).x7 = value.clone();
    }
    else if (*context).scfia.check_condition(&(*context).scfia.new_bool_eq(&register_id.clone(), &(*context).scfia.new_bv_concrete(0b1000, 5), None, false, &mut (*context).fork_sink, None), &mut (*context).fork_sink) {
        (*state).x8 = value.clone();
    }
    else if (*context).scfia.check_condition(&(*context).scfia.new_bool_eq(&register_id.clone(), &(*context).scfia.new_bv_concrete(0b1001, 5), None, false, &mut (*context).fork_sink, None), &mut (*context).fork_sink) {
        (*state).x9 = value.clone();
    }
    else if (*context).scfia.check_condition(&(*context).scfia.new_bool_eq(&register_id.clone(), &(*context).scfia.new_bv_concrete(0b1010, 5), None, false, &mut (*context).fork_sink, None), &mut (*context).fork_sink) {
        (*state).x10 = value.clone();
    }
    else if (*context).scfia.check_condition(&(*context).scfia.new_bool_eq(&register_id.clone(), &(*context).scfia.new_bv_concrete(0b1011, 5), None, false, &mut (*context).fork_sink, None), &mut (*context).fork_sink) {
        (*state).x11 = value.clone();
    }
    else if (*context).scfia.check_condition(&(*context).scfia.new_bool_eq(&register_id.clone(), &(*context).scfia.new_bv_concrete(0b1100, 5), None, false, &mut (*context).fork_sink, None), &mut (*context).fork_sink) {
        (*state).x12 = value.clone();
    }
    else if (*context).scfia.check_condition(&(*context).scfia.new_bool_eq(&register_id.clone(), &(*context).scfia.new_bv_concrete(0b1101, 5), None, false, &mut (*context).fork_sink, None), &mut (*context).fork_sink) {
        (*state).x13 = value.clone();
    }
    else if (*context).scfia.check_condition(&(*context).scfia.new_bool_eq(&register_id.clone(), &(*context).scfia.new_bv_concrete(0b1110, 5), None, false, &mut (*context).fork_sink, None), &mut (*context).fork_sink) {
        (*state).x14 = value.clone();
    }
    else if (*context).scfia.check_condition(&(*context).scfia.new_bool_eq(&register_id.clone(), &(*context).scfia.new_bv_concrete(0b1111, 5), None, false, &mut (*context).fork_sink, None), &mut (*context).fork_sink) {
        (*state).x15 = value.clone();
    }
    else if (*context).scfia.check_condition(&(*context).scfia.new_bool_eq(&register_id.clone(), &(*context).scfia.new_bv_concrete(0b10000, 5), None, false, &mut (*context).fork_sink, None), &mut (*context).fork_sink) {
        (*state).x16 = value.clone();
    }
    else if (*context).scfia.check_condition(&(*context).scfia.new_bool_eq(&register_id.clone(), &(*context).scfia.new_bv_concrete(0b10001, 5), None, false, &mut (*context).fork_sink, None), &mut (*context).fork_sink) {
        (*state).x17 = value.clone();
    }
    else if (*context).scfia.check_condition(&(*context).scfia.new_bool_eq(&register_id.clone(), &(*context).scfia.new_bv_concrete(0b10010, 5), None, false, &mut (*context).fork_sink, None), &mut (*context).fork_sink) {
        (*state).x18 = value.clone();
    }
    else if (*context).scfia.check_condition(&(*context).scfia.new_bool_eq(&register_id.clone(), &(*context).scfia.new_bv_concrete(0b10011, 5), None, false, &mut (*context).fork_sink, None), &mut (*context).fork_sink) {
        (*state).x19 = value.clone();
    }
    else if (*context).scfia.check_condition(&(*context).scfia.new_bool_eq(&register_id.clone(), &(*context).scfia.new_bv_concrete(0b10100, 5), None, false, &mut (*context).fork_sink, None), &mut (*context).fork_sink) {
        (*state).x20 = value.clone();
    }
    else if (*context).scfia.check_condition(&(*context).scfia.new_bool_eq(&register_id.clone(), &(*context).scfia.new_bv_concrete(0b10101, 5), None, false, &mut (*context).fork_sink, None), &mut (*context).fork_sink) {
        (*state).x21 = value.clone();
    }
    else if (*context).scfia.check_condition(&(*context).scfia.new_bool_eq(&register_id.clone(), &(*context).scfia.new_bv_concrete(0b10110, 5), None, false, &mut (*context).fork_sink, None), &mut (*context).fork_sink) {
        (*state).x22 = value.clone();
    }
    else if (*context).scfia.check_condition(&(*context).scfia.new_bool_eq(&register_id.clone(), &(*context).scfia.new_bv_concrete(0b10111, 5), None, false, &mut (*context).fork_sink, None), &mut (*context).fork_sink) {
        (*state).x23 = value.clone();
    }
    else if (*context).scfia.check_condition(&(*context).scfia.new_bool_eq(&register_id.clone(), &(*context).scfia.new_bv_concrete(0b11000, 5), None, false, &mut (*context).fork_sink, None), &mut (*context).fork_sink) {
        (*state).x24 = value.clone();
    }
    else if (*context).scfia.check_condition(&(*context).scfia.new_bool_eq(&register_id.clone(), &(*context).scfia.new_bv_concrete(0b11001, 5), None, false, &mut (*context).fork_sink, None), &mut (*context).fork_sink) {
        (*state).x25 = value.clone();
    }
    else if (*context).scfia.check_condition(&(*context).scfia.new_bool_eq(&register_id.clone(), &(*context).scfia.new_bv_concrete(0b11010, 5), None, false, &mut (*context).fork_sink, None), &mut (*context).fork_sink) {
        (*state).x26 = value.clone();
    }
    else if (*context).scfia.check_condition(&(*context).scfia.new_bool_eq(&register_id.clone(), &(*context).scfia.new_bv_concrete(0b11011, 5), None, false, &mut (*context).fork_sink, None), &mut (*context).fork_sink) {
        (*state).x27 = value.clone();
    }
    else if (*context).scfia.check_condition(&(*context).scfia.new_bool_eq(&register_id.clone(), &(*context).scfia.new_bv_concrete(0b11100, 5), None, false, &mut (*context).fork_sink, None), &mut (*context).fork_sink) {
        (*state).x28 = value.clone();
    }
    else if (*context).scfia.check_condition(&(*context).scfia.new_bool_eq(&register_id.clone(), &(*context).scfia.new_bv_concrete(0b11101, 5), None, false, &mut (*context).fork_sink, None), &mut (*context).fork_sink) {
        (*state).x29 = value.clone();
    }
    else if (*context).scfia.check_condition(&(*context).scfia.new_bool_eq(&register_id.clone(), &(*context).scfia.new_bv_concrete(0b11110, 5), None, false, &mut (*context).fork_sink, None), &mut (*context).fork_sink) {
        (*state).x30 = value.clone();
    }
    else if (*context).scfia.check_condition(&(*context).scfia.new_bool_eq(&register_id.clone(), &(*context).scfia.new_bv_concrete(0b11111, 5), None, false, &mut (*context).fork_sink, None), &mut (*context).fork_sink) {
        (*state).x31 = value.clone();
    }
    else {
        unimplemented!();
    }
}

unsafe fn _register_read_BV32(state: *mut SystemState, register_id: ActiveValue<RV32iScfiaComposition>, context: *mut StepContext<RV32iScfiaComposition>) -> ActiveValue<RV32iScfiaComposition> {
    if (*context).scfia.check_condition(&(*context).scfia.new_bool_eq(&register_id.clone(), &(*context).scfia.new_bv_concrete(0b0, 5), None, false, &mut (*context).fork_sink, None), &mut (*context).fork_sink) {
        return (*context).scfia.new_bv_concrete(0b0, 32);
    }
    else if (*context).scfia.check_condition(&(*context).scfia.new_bool_eq(&register_id.clone(), &(*context).scfia.new_bv_concrete(0b1, 5), None, false, &mut (*context).fork_sink, None), &mut (*context).fork_sink) {
        return (*state).x1.clone();
    }
    else if (*context).scfia.check_condition(&(*context).scfia.new_bool_eq(&register_id.clone(), &(*context).scfia.new_bv_concrete(0b10, 5), None, false, &mut (*context).fork_sink, None), &mut (*context).fork_sink) {
        return (*state).x2.clone();
    }
    else if (*context).scfia.check_condition(&(*context).scfia.new_bool_eq(&register_id.clone(), &(*context).scfia.new_bv_concrete(0b11, 5), None, false, &mut (*context).fork_sink, None), &mut (*context).fork_sink) {
        return (*state).x3.clone();
    }
    else if (*context).scfia.check_condition(&(*context).scfia.new_bool_eq(&register_id.clone(), &(*context).scfia.new_bv_concrete(0b100, 5), None, false, &mut (*context).fork_sink, None), &mut (*context).fork_sink) {
        return (*state).x4.clone();
    }
    else if (*context).scfia.check_condition(&(*context).scfia.new_bool_eq(&register_id.clone(), &(*context).scfia.new_bv_concrete(0b101, 5), None, false, &mut (*context).fork_sink, None), &mut (*context).fork_sink) {
        return (*state).x5.clone();
    }
    else if (*context).scfia.check_condition(&(*context).scfia.new_bool_eq(&register_id.clone(), &(*context).scfia.new_bv_concrete(0b110, 5), None, false, &mut (*context).fork_sink, None), &mut (*context).fork_sink) {
        return (*state).x6.clone();
    }
    else if (*context).scfia.check_condition(&(*context).scfia.new_bool_eq(&register_id.clone(), &(*context).scfia.new_bv_concrete(0b111, 5), None, false, &mut (*context).fork_sink, None), &mut (*context).fork_sink) {
        return (*state).x7.clone();
    }
    else if (*context).scfia.check_condition(&(*context).scfia.new_bool_eq(&register_id.clone(), &(*context).scfia.new_bv_concrete(0b1000, 5), None, false, &mut (*context).fork_sink, None), &mut (*context).fork_sink) {
        return (*state).x8.clone();
    }
    else if (*context).scfia.check_condition(&(*context).scfia.new_bool_eq(&register_id.clone(), &(*context).scfia.new_bv_concrete(0b1001, 5), None, false, &mut (*context).fork_sink, None), &mut (*context).fork_sink) {
        return (*state).x9.clone();
    }
    else if (*context).scfia.check_condition(&(*context).scfia.new_bool_eq(&register_id.clone(), &(*context).scfia.new_bv_concrete(0b1010, 5), None, false, &mut (*context).fork_sink, None), &mut (*context).fork_sink) {
        return (*state).x10.clone();
    }
    else if (*context).scfia.check_condition(&(*context).scfia.new_bool_eq(&register_id.clone(), &(*context).scfia.new_bv_concrete(0b1011, 5), None, false, &mut (*context).fork_sink, None), &mut (*context).fork_sink) {
        return (*state).x11.clone();
    }
    else if (*context).scfia.check_condition(&(*context).scfia.new_bool_eq(&register_id.clone(), &(*context).scfia.new_bv_concrete(0b1100, 5), None, false, &mut (*context).fork_sink, None), &mut (*context).fork_sink) {
        return (*state).x12.clone();
    }
    else if (*context).scfia.check_condition(&(*context).scfia.new_bool_eq(&register_id.clone(), &(*context).scfia.new_bv_concrete(0b1101, 5), None, false, &mut (*context).fork_sink, None), &mut (*context).fork_sink) {
        return (*state).x13.clone();
    }
    else if (*context).scfia.check_condition(&(*context).scfia.new_bool_eq(&register_id.clone(), &(*context).scfia.new_bv_concrete(0b1110, 5), None, false, &mut (*context).fork_sink, None), &mut (*context).fork_sink) {
        return (*state).x14.clone();
    }
    else if (*context).scfia.check_condition(&(*context).scfia.new_bool_eq(&register_id.clone(), &(*context).scfia.new_bv_concrete(0b1111, 5), None, false, &mut (*context).fork_sink, None), &mut (*context).fork_sink) {
        return (*state).x15.clone();
    }
    else if (*context).scfia.check_condition(&(*context).scfia.new_bool_eq(&register_id.clone(), &(*context).scfia.new_bv_concrete(0b10000, 5), None, false, &mut (*context).fork_sink, None), &mut (*context).fork_sink) {
        return (*state).x16.clone();
    }
    else if (*context).scfia.check_condition(&(*context).scfia.new_bool_eq(&register_id.clone(), &(*context).scfia.new_bv_concrete(0b10001, 5), None, false, &mut (*context).fork_sink, None), &mut (*context).fork_sink) {
        return (*state).x17.clone();
    }
    else if (*context).scfia.check_condition(&(*context).scfia.new_bool_eq(&register_id.clone(), &(*context).scfia.new_bv_concrete(0b10010, 5), None, false, &mut (*context).fork_sink, None), &mut (*context).fork_sink) {
        return (*state).x18.clone();
    }
    else if (*context).scfia.check_condition(&(*context).scfia.new_bool_eq(&register_id.clone(), &(*context).scfia.new_bv_concrete(0b10011, 5), None, false, &mut (*context).fork_sink, None), &mut (*context).fork_sink) {
        return (*state).x19.clone();
    }
    else if (*context).scfia.check_condition(&(*context).scfia.new_bool_eq(&register_id.clone(), &(*context).scfia.new_bv_concrete(0b10100, 5), None, false, &mut (*context).fork_sink, None), &mut (*context).fork_sink) {
        return (*state).x20.clone();
    }
    else if (*context).scfia.check_condition(&(*context).scfia.new_bool_eq(&register_id.clone(), &(*context).scfia.new_bv_concrete(0b10101, 5), None, false, &mut (*context).fork_sink, None), &mut (*context).fork_sink) {
        return (*state).x21.clone();
    }
    else if (*context).scfia.check_condition(&(*context).scfia.new_bool_eq(&register_id.clone(), &(*context).scfia.new_bv_concrete(0b10110, 5), None, false, &mut (*context).fork_sink, None), &mut (*context).fork_sink) {
        return (*state).x22.clone();
    }
    else if (*context).scfia.check_condition(&(*context).scfia.new_bool_eq(&register_id.clone(), &(*context).scfia.new_bv_concrete(0b10111, 5), None, false, &mut (*context).fork_sink, None), &mut (*context).fork_sink) {
        return (*state).x23.clone();
    }
    else if (*context).scfia.check_condition(&(*context).scfia.new_bool_eq(&register_id.clone(), &(*context).scfia.new_bv_concrete(0b11000, 5), None, false, &mut (*context).fork_sink, None), &mut (*context).fork_sink) {
        return (*state).x24.clone();
    }
    else if (*context).scfia.check_condition(&(*context).scfia.new_bool_eq(&register_id.clone(), &(*context).scfia.new_bv_concrete(0b11001, 5), None, false, &mut (*context).fork_sink, None), &mut (*context).fork_sink) {
        return (*state).x25.clone();
    }
    else if (*context).scfia.check_condition(&(*context).scfia.new_bool_eq(&register_id.clone(), &(*context).scfia.new_bv_concrete(0b11010, 5), None, false, &mut (*context).fork_sink, None), &mut (*context).fork_sink) {
        return (*state).x26.clone();
    }
    else if (*context).scfia.check_condition(&(*context).scfia.new_bool_eq(&register_id.clone(), &(*context).scfia.new_bv_concrete(0b11011, 5), None, false, &mut (*context).fork_sink, None), &mut (*context).fork_sink) {
        return (*state).x27.clone();
    }
    else if (*context).scfia.check_condition(&(*context).scfia.new_bool_eq(&register_id.clone(), &(*context).scfia.new_bv_concrete(0b11100, 5), None, false, &mut (*context).fork_sink, None), &mut (*context).fork_sink) {
        return (*state).x28.clone();
    }
    else if (*context).scfia.check_condition(&(*context).scfia.new_bool_eq(&register_id.clone(), &(*context).scfia.new_bv_concrete(0b11101, 5), None, false, &mut (*context).fork_sink, None), &mut (*context).fork_sink) {
        return (*state).x29.clone();
    }
    else if (*context).scfia.check_condition(&(*context).scfia.new_bool_eq(&register_id.clone(), &(*context).scfia.new_bv_concrete(0b11110, 5), None, false, &mut (*context).fork_sink, None), &mut (*context).fork_sink) {
        return (*state).x30.clone();
    }
    else if (*context).scfia.check_condition(&(*context).scfia.new_bool_eq(&register_id.clone(), &(*context).scfia.new_bv_concrete(0b11111, 5), None, false, &mut (*context).fork_sink, None), &mut (*context).fork_sink) {
        return (*state).x31.clone();
    }
    else {
        unimplemented!();
    }
}

unsafe fn _execute_add32(state: *mut SystemState, destination_id: ActiveValue<RV32iScfiaComposition>, source1_id: ActiveValue<RV32iScfiaComposition>, source2_id: ActiveValue<RV32iScfiaComposition>, context: *mut StepContext<RV32iScfiaComposition>) {
    let s1: ActiveValue<RV32iScfiaComposition> = _register_read_BV32(state, source1_id.clone(), context);
    let s2: ActiveValue<RV32iScfiaComposition> = _register_read_BV32(state, source2_id.clone(), context);
    let sum: ActiveValue<RV32iScfiaComposition> = (*context).scfia.new_bv_add(&s1.clone(), &s2.clone(), 32, None, &mut (*context).fork_sink, None);
    _register_write_BV32(state, destination_id.clone(), sum.clone(), context);
}
