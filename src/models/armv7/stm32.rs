#![allow(clippy::all)]
#![allow(non_snake_case)]
#![allow(unused)]
use log::debug;
use std::{borrow::BorrowMut, fmt::Debug, collections::BTreeMap, rc::Rc};

use crate::{memory::Memory, scfia::Scfia, values::{active_value::ActiveValue, retired_value::RetiredValue}, GenericForkSink, ScfiaComposition, StepContext, SymbolicHints};

pub struct STM32 {
    pub state: SystemState,
    pub memory: Memory<STM32ScfiaComposition>,
    pub scfia: Rc<Scfia<STM32ScfiaComposition>>,
}

#[derive(Debug)]
pub struct STM32ForkSink {
    base_state: STM32,
    new_values_history: Vec<ActiveValue<STM32ScfiaComposition>>,
    forks: Vec<STM32>,
}

#[derive(Debug, Clone)]
pub struct STM32ScfiaComposition {}


impl GenericForkSink<STM32ScfiaComposition> for STM32ForkSink {
    fn fork(&mut self, fork_symbol: ActiveValue<STM32ScfiaComposition>) {
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
        self.forks.push(clone);
    }

    fn push_value(&mut self, value: ActiveValue<STM32ScfiaComposition>) {
        self.new_values_history.push(value)
    }
}

impl ScfiaComposition for STM32ScfiaComposition {
    type Model = STM32;
    type ForkSink = STM32ForkSink;
}

impl STM32 {
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

    pub fn step_forking(self, mut hints: Option<SymbolicHints>) -> Vec<STM32> {
        unsafe {
            let mut states: Vec<STM32> = vec![self];
            let mut results = vec![];

            while let Some(mut state) = states.pop() {
                let mut context = StepContext {
                    memory: &mut state.memory,
                    scfia: &state.scfia,
                    hints: hints.clone(),
                    fork_sink: Some(STM32ForkSink {
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

    pub fn clone_model(&self) -> (STM32, BTreeMap<u64, ActiveValue<STM32ScfiaComposition>>, BTreeMap<u64, RetiredValue<STM32ScfiaComposition>>) {
        unsafe {
            let cloned_scfia = Scfia::new(Some(self.scfia.next_symbol_id.get()));
            let mut cloned_actives = BTreeMap::new();
            let mut cloned_retireds = BTreeMap::new();
            debug!("cloning scfia {:?} to {:?}",
                Rc::as_ptr(&self.scfia.selff.get().unwrap().upgrade().unwrap()),
                Rc::as_ptr(&cloned_scfia.selff.get().unwrap().upgrade().unwrap()));
            (STM32 {
                state: self.state.clone_to_stdlib(&cloned_scfia, &mut cloned_actives, &mut cloned_retireds),
                memory: self.memory.clone_to_stdlib(&cloned_scfia, &mut cloned_actives, &mut cloned_retireds),
                scfia: cloned_scfia,
            }, cloned_actives, cloned_retireds)
        }
    }

    pub fn debug(&self) {
        debug!("Register depths:");
        debug!("R0:\tdepth={}", self.state.R0.get_depth());
        debug!("R1:\tdepth={}", self.state.R1.get_depth());
        debug!("R2:\tdepth={}", self.state.R2.get_depth());
        debug!("R3:\tdepth={}", self.state.R3.get_depth());
        debug!("R4:\tdepth={}", self.state.R4.get_depth());
        debug!("R5:\tdepth={}", self.state.R5.get_depth());
        debug!("R6:\tdepth={}", self.state.R6.get_depth());
        debug!("R7:\tdepth={}", self.state.R7.get_depth());
        debug!("R8:\tdepth={}", self.state.R8.get_depth());
        debug!("R9:\tdepth={}", self.state.R9.get_depth());
        debug!("R10:\tdepth={}", self.state.R10.get_depth());
        debug!("R11:\tdepth={}", self.state.R11.get_depth());
        debug!("R12:\tdepth={}", self.state.R12.get_depth());
        debug!("SP:\tdepth={}", self.state.SP.get_depth());
        debug!("LR:\tdepth={}", self.state.LR.get_depth());
        debug!("PC:\tdepth={}", self.state.PC.get_depth());
        // debug!("apsr:\tdepth={}", self.state.apsr.get_depth());
        debug!("Stable memory cells: {}", self.memory.stables.iter().map(|e|e.memory.len()).sum::<usize>());
        let (worst_cell, depth) = self.memory.get_highest_depth().unwrap();
        debug!("Worst memory cell: {:x} depth={}", worst_cell, depth);
    }

}

impl Debug for STM32 {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("STM32").field("state", &self.state).finish()
    }
}


#[derive(Debug)]
pub struct ApplicationProgramStatusRegister {
    pub N: ActiveValue<STM32ScfiaComposition>,
    pub Z: ActiveValue<STM32ScfiaComposition>,
    pub C: ActiveValue<STM32ScfiaComposition>,
    pub V: ActiveValue<STM32ScfiaComposition>,
    pub Q: ActiveValue<STM32ScfiaComposition>,
    pub GE: ActiveValue<STM32ScfiaComposition>,
}

impl ApplicationProgramStatusRegister {
    fn clone_to_stdlib(&self, cloned_scfia: &Scfia<STM32ScfiaComposition>, cloned_actives: &mut BTreeMap<u64, ActiveValue<STM32ScfiaComposition>>, cloned_inactives: &mut BTreeMap<u64, RetiredValue<STM32ScfiaComposition>>) -> ApplicationProgramStatusRegister {
        ApplicationProgramStatusRegister {
            N: self.N.clone_to_stdlib(cloned_scfia, cloned_actives, cloned_inactives),
            Z: self.Z.clone_to_stdlib(cloned_scfia, cloned_actives, cloned_inactives),
            C: self.C.clone_to_stdlib(cloned_scfia, cloned_actives, cloned_inactives),
            V: self.V.clone_to_stdlib(cloned_scfia, cloned_actives, cloned_inactives),
            Q: self.Q.clone_to_stdlib(cloned_scfia, cloned_actives, cloned_inactives),
            GE: self.GE.clone_to_stdlib(cloned_scfia, cloned_actives, cloned_inactives),
        }
    }
}

#[derive(Debug)]
pub struct SystemState {
    pub R0: ActiveValue<STM32ScfiaComposition>,
    pub R1: ActiveValue<STM32ScfiaComposition>,
    pub R2: ActiveValue<STM32ScfiaComposition>,
    pub R3: ActiveValue<STM32ScfiaComposition>,
    pub R4: ActiveValue<STM32ScfiaComposition>,
    pub R5: ActiveValue<STM32ScfiaComposition>,
    pub R6: ActiveValue<STM32ScfiaComposition>,
    pub R7: ActiveValue<STM32ScfiaComposition>,
    pub R8: ActiveValue<STM32ScfiaComposition>,
    pub R9: ActiveValue<STM32ScfiaComposition>,
    pub R10: ActiveValue<STM32ScfiaComposition>,
    pub R11: ActiveValue<STM32ScfiaComposition>,
    pub R12: ActiveValue<STM32ScfiaComposition>,
    pub SP: ActiveValue<STM32ScfiaComposition>,
    pub LR: ActiveValue<STM32ScfiaComposition>,
    pub PC: ActiveValue<STM32ScfiaComposition>,
    pub apsr: ApplicationProgramStatusRegister,
}

impl SystemState {
    fn clone_to_stdlib(&self, cloned_scfia: &Scfia<STM32ScfiaComposition>, cloned_actives: &mut BTreeMap<u64, ActiveValue<STM32ScfiaComposition>>, cloned_inactives: &mut BTreeMap<u64, RetiredValue<STM32ScfiaComposition>>) -> SystemState {
        SystemState {
            R0: self.R0.clone_to_stdlib(cloned_scfia, cloned_actives, cloned_inactives),
            R1: self.R1.clone_to_stdlib(cloned_scfia, cloned_actives, cloned_inactives),
            R2: self.R2.clone_to_stdlib(cloned_scfia, cloned_actives, cloned_inactives),
            R3: self.R3.clone_to_stdlib(cloned_scfia, cloned_actives, cloned_inactives),
            R4: self.R4.clone_to_stdlib(cloned_scfia, cloned_actives, cloned_inactives),
            R5: self.R5.clone_to_stdlib(cloned_scfia, cloned_actives, cloned_inactives),
            R6: self.R6.clone_to_stdlib(cloned_scfia, cloned_actives, cloned_inactives),
            R7: self.R7.clone_to_stdlib(cloned_scfia, cloned_actives, cloned_inactives),
            R8: self.R8.clone_to_stdlib(cloned_scfia, cloned_actives, cloned_inactives),
            R9: self.R9.clone_to_stdlib(cloned_scfia, cloned_actives, cloned_inactives),
            R10: self.R10.clone_to_stdlib(cloned_scfia, cloned_actives, cloned_inactives),
            R11: self.R11.clone_to_stdlib(cloned_scfia, cloned_actives, cloned_inactives),
            R12: self.R12.clone_to_stdlib(cloned_scfia, cloned_actives, cloned_inactives),
            SP: self.SP.clone_to_stdlib(cloned_scfia, cloned_actives, cloned_inactives),
            LR: self.LR.clone_to_stdlib(cloned_scfia, cloned_actives, cloned_inactives),
            PC: self.PC.clone_to_stdlib(cloned_scfia, cloned_actives, cloned_inactives),
            apsr: self.apsr.clone_to_stdlib(cloned_scfia, cloned_actives, cloned_inactives),
        }
    }
}

unsafe fn _step(state: *mut SystemState, context: *mut StepContext<STM32ScfiaComposition>) {
    let instruction_16: ActiveValue<STM32ScfiaComposition> = (*(*context).memory).read(&(*state).PC.clone(), 16, (*context).scfia.clone(), &mut (*context).hints, &mut (*context).fork_sink);
    let b5: ActiveValue<STM32ScfiaComposition> = (*context).scfia.new_bv_slice(&instruction_16.clone(), 15, 11, None, &mut (*context).fork_sink, None);
    if (*context).scfia.check_condition(&(*context).scfia.new_bool_eq(&b5.clone(), &(*context).scfia.new_bv_concrete(0b11101, 5), None, false, &mut (*context).fork_sink, None), &mut (*context).fork_sink) {
        unimplemented!();
    }
    else if (*context).scfia.check_condition(&(*context).scfia.new_bool_eq(&b5.clone(), &(*context).scfia.new_bv_concrete(0b11110, 5), None, false, &mut (*context).fork_sink, None), &mut (*context).fork_sink) {
        unimplemented!();
    }
    else if (*context).scfia.check_condition(&(*context).scfia.new_bool_eq(&b5.clone(), &(*context).scfia.new_bv_concrete(0b11111, 5), None, false, &mut (*context).fork_sink, None), &mut (*context).fork_sink) {
        unimplemented!();
    }
    else {
        _thumb16(instruction_16.clone(), state, context);
    }
}

unsafe fn _thumb16(instruction: ActiveValue<STM32ScfiaComposition>, state: *mut SystemState, context: *mut StepContext<STM32ScfiaComposition>) {
    let opcode: ActiveValue<STM32ScfiaComposition> = (*context).scfia.new_bv_slice(&instruction.clone(), 15, 10, None, &mut (*context).fork_sink, None);
    if (*context).scfia.check_condition(&(*context).scfia.new_bool_unsigned_less_than(&opcode.clone(), &(*context).scfia.new_bv_concrete(0b10000, 6), None, false, &mut (*context).fork_sink, None), &mut (*context).fork_sink) {
        _thumb16_basic(instruction.clone(), state, context);
    }
    else if (*context).scfia.check_condition(&(*context).scfia.new_bool_eq(&opcode.clone(), &(*context).scfia.new_bv_concrete(0b10000, 6), None, false, &mut (*context).fork_sink, None), &mut (*context).fork_sink) {
        _thumb16_data_processing(instruction.clone(), state, context);
    }
    else if (*context).scfia.check_condition(&(*context).scfia.new_bool_eq(&opcode.clone(), &(*context).scfia.new_bv_concrete(0b10001, 6), None, false, &mut (*context).fork_sink, None), &mut (*context).fork_sink) {
        unimplemented!();
    }
    else if (*context).scfia.check_condition(&(*context).scfia.new_bool_unsigned_less_than(&opcode.clone(), &(*context).scfia.new_bv_concrete(0b10100, 6), None, false, &mut (*context).fork_sink, None), &mut (*context).fork_sink) {
        _thumb16_load_from_literal_pool(instruction.clone(), state, context);
    }
    else if (*context).scfia.check_condition(&(*context).scfia.new_bool_unsigned_less_than(&opcode.clone(), &(*context).scfia.new_bv_concrete(0b101000, 6), None, false, &mut (*context).fork_sink, None), &mut (*context).fork_sink) {
        _thumb16_load_store_single_data_item(instruction.clone(), state, context);
    }
    else if (*context).scfia.check_condition(&(*context).scfia.new_bool_unsigned_less_than(&opcode.clone(), &(*context).scfia.new_bv_concrete(0b101010, 6), None, false, &mut (*context).fork_sink, None), &mut (*context).fork_sink) {
        unimplemented!();
    }
    else if (*context).scfia.check_condition(&(*context).scfia.new_bool_unsigned_less_than(&opcode.clone(), &(*context).scfia.new_bv_concrete(0b101100, 6), None, false, &mut (*context).fork_sink, None), &mut (*context).fork_sink) {
        unimplemented!();
    }
    else if (*context).scfia.check_condition(&(*context).scfia.new_bool_unsigned_less_than(&opcode.clone(), &(*context).scfia.new_bv_concrete(0b110000, 6), None, false, &mut (*context).fork_sink, None), &mut (*context).fork_sink) {
        unimplemented!();
    }
    else if (*context).scfia.check_condition(&(*context).scfia.new_bool_unsigned_less_than(&opcode.clone(), &(*context).scfia.new_bv_concrete(0b110010, 6), None, false, &mut (*context).fork_sink, None), &mut (*context).fork_sink) {
        unimplemented!();
    }
    else if (*context).scfia.check_condition(&(*context).scfia.new_bool_unsigned_less_than(&opcode.clone(), &(*context).scfia.new_bv_concrete(0b110100, 6), None, false, &mut (*context).fork_sink, None), &mut (*context).fork_sink) {
        unimplemented!();
    }
    else if (*context).scfia.check_condition(&(*context).scfia.new_bool_unsigned_less_than(&opcode.clone(), &(*context).scfia.new_bv_concrete(0b111000, 6), None, false, &mut (*context).fork_sink, None), &mut (*context).fork_sink) {
        unimplemented!();
    }
    else if (*context).scfia.check_condition(&(*context).scfia.new_bool_unsigned_less_than(&opcode.clone(), &(*context).scfia.new_bv_concrete(0b111010, 6), None, false, &mut (*context).fork_sink, None), &mut (*context).fork_sink) {
        _thumb16_unconditional_branch(instruction.clone(), state, context);
    }
    else {
        unimplemented!();
    }
    _progress_pc_2(state, context);
}

unsafe fn _thumb16_basic(instruction: ActiveValue<STM32ScfiaComposition>, state: *mut SystemState, context: *mut StepContext<STM32ScfiaComposition>) {
    let opcode: ActiveValue<STM32ScfiaComposition> = (*context).scfia.new_bv_slice(&instruction.clone(), 13, 9, None, &mut (*context).fork_sink, None);
    let opcode_pt1: ActiveValue<STM32ScfiaComposition> = (*context).scfia.new_bv_slice(&instruction.clone(), 13, 11, None, &mut (*context).fork_sink, None);
    if (*context).scfia.check_condition(&(*context).scfia.new_bool_eq(&opcode_pt1.clone(), &(*context).scfia.new_bv_concrete(0b0, 3), None, false, &mut (*context).fork_sink, None), &mut (*context).fork_sink) {
        unimplemented!();
    }
    else if (*context).scfia.check_condition(&(*context).scfia.new_bool_eq(&opcode_pt1.clone(), &(*context).scfia.new_bv_concrete(0b1, 3), None, false, &mut (*context).fork_sink, None), &mut (*context).fork_sink) {
        unimplemented!();
    }
    else if (*context).scfia.check_condition(&(*context).scfia.new_bool_eq(&opcode_pt1.clone(), &(*context).scfia.new_bv_concrete(0b10, 3), None, false, &mut (*context).fork_sink, None), &mut (*context).fork_sink) {
        unimplemented!();
    }
    else if (*context).scfia.check_condition(&(*context).scfia.new_bool_eq(&opcode.clone(), &(*context).scfia.new_bv_concrete(0b1100, 5), None, false, &mut (*context).fork_sink, None), &mut (*context).fork_sink) {
        let rd: ActiveValue<STM32ScfiaComposition> = (*context).scfia.new_bv_slice(&instruction.clone(), 2, 0, None, &mut (*context).fork_sink, None);
        let rn: ActiveValue<STM32ScfiaComposition> = (*context).scfia.new_bv_slice(&instruction.clone(), 5, 3, None, &mut (*context).fork_sink, None);
        let rm: ActiveValue<STM32ScfiaComposition> = (*context).scfia.new_bv_slice(&instruction.clone(), 8, 6, None, &mut (*context).fork_sink, None);
        let setflags: ActiveValue<STM32ScfiaComposition> = (*context).scfia.new_bv_concrete(0b1, 1);
        let result: ActiveValue<STM32ScfiaComposition> = (*context).scfia.new_bv_add(&_register_read_BV32(rn.clone(), state, context), &_register_read_BV32(rm.clone(), state, context), 32, None, &mut (*context).fork_sink, None);
        let carry: ActiveValue<STM32ScfiaComposition> = (*context).scfia.new_bv_concrete(0b0, 1);
        let overflow: ActiveValue<STM32ScfiaComposition> = (*context).scfia.new_bv_concrete(0b0, 1);
        _register_write_BV32(rd.clone(), result.clone(), state, context);
        if (*context).scfia.check_condition(&(*context).scfia.new_bool_eq(&setflags.clone(), &(*context).scfia.new_bv_concrete(0b1, 1), None, false, &mut (*context).fork_sink, None), &mut (*context).fork_sink) {
            (*state).apsr.N = (*context).scfia.new_bv_slice(&result.clone(), 31, 31, None, &mut (*context).fork_sink, None);
            (*state).apsr.Z = _is_zero_bit_BV32(result.clone(), context);
            (*state).apsr.C = carry.clone();
            (*state).apsr.V = overflow.clone();
        }
    }
    else if (*context).scfia.check_condition(&(*context).scfia.new_bool_eq(&opcode.clone(), &(*context).scfia.new_bv_concrete(0b1101, 5), None, false, &mut (*context).fork_sink, None), &mut (*context).fork_sink) {
        unimplemented!();
    }
    else if (*context).scfia.check_condition(&(*context).scfia.new_bool_eq(&opcode.clone(), &(*context).scfia.new_bv_concrete(0b1110, 5), None, false, &mut (*context).fork_sink, None), &mut (*context).fork_sink) {
        unimplemented!();
    }
    else if (*context).scfia.check_condition(&(*context).scfia.new_bool_eq(&opcode.clone(), &(*context).scfia.new_bv_concrete(0b1111, 5), None, false, &mut (*context).fork_sink, None), &mut (*context).fork_sink) {
        unimplemented!();
    }
    else if (*context).scfia.check_condition(&(*context).scfia.new_bool_eq(&opcode_pt1.clone(), &(*context).scfia.new_bv_concrete(0b100, 3), None, false, &mut (*context).fork_sink, None), &mut (*context).fork_sink) {
        let rd: ActiveValue<STM32ScfiaComposition> = (*context).scfia.new_bv_slice(&instruction.clone(), 10, 8, None, &mut (*context).fork_sink, None);
        let imm8: ActiveValue<STM32ScfiaComposition> = (*context).scfia.new_bv_slice(&instruction.clone(), 7, 0, None, &mut (*context).fork_sink, None);
        let imm9: ActiveValue<STM32ScfiaComposition> = (*context).scfia.new_bv_concat(&imm8.clone(), &(*context).scfia.new_bv_concrete(0b0, 1), 9, None, &mut (*context).fork_sink, None);
        let imm32: ActiveValue<STM32ScfiaComposition> = (*context).scfia.new_bv_concat(&(*context).scfia.new_bv_concrete(0b0, 23), &imm9.clone(), 32, None, &mut (*context).fork_sink, None);
        let setflags: ActiveValue<STM32ScfiaComposition> = (*context).scfia.new_bv_concrete(0b1, 1);
        let carry: ActiveValue<STM32ScfiaComposition> = (*state).apsr.C.clone();
        _mov_immediate(rd.clone(), setflags.clone(), imm32.clone(), carry.clone(), state, context);
    }
    else {
        unimplemented!();
    }
}

unsafe fn _thumb16_data_processing(instruction: ActiveValue<STM32ScfiaComposition>, state: *mut SystemState, context: *mut StepContext<STM32ScfiaComposition>) {
    let opcode: ActiveValue<STM32ScfiaComposition> = (*context).scfia.new_bv_slice(&instruction.clone(), 9, 6, None, &mut (*context).fork_sink, None);
    if (*context).scfia.check_condition(&(*context).scfia.new_bool_eq(&opcode.clone(), &(*context).scfia.new_bv_concrete(0b0, 4), None, false, &mut (*context).fork_sink, None), &mut (*context).fork_sink) {
        unimplemented!();
    }
    else if (*context).scfia.check_condition(&(*context).scfia.new_bool_eq(&opcode.clone(), &(*context).scfia.new_bv_concrete(0b0, 4), None, false, &mut (*context).fork_sink, None), &mut (*context).fork_sink) {
        unimplemented!();
    }
    else if (*context).scfia.check_condition(&(*context).scfia.new_bool_eq(&opcode.clone(), &(*context).scfia.new_bv_concrete(0b1, 4), None, false, &mut (*context).fork_sink, None), &mut (*context).fork_sink) {
        unimplemented!();
    }
    else if (*context).scfia.check_condition(&(*context).scfia.new_bool_eq(&opcode.clone(), &(*context).scfia.new_bv_concrete(0b10, 4), None, false, &mut (*context).fork_sink, None), &mut (*context).fork_sink) {
        unimplemented!();
    }
    else if (*context).scfia.check_condition(&(*context).scfia.new_bool_eq(&opcode.clone(), &(*context).scfia.new_bv_concrete(0b11, 4), None, false, &mut (*context).fork_sink, None), &mut (*context).fork_sink) {
        unimplemented!();
    }
    else if (*context).scfia.check_condition(&(*context).scfia.new_bool_eq(&opcode.clone(), &(*context).scfia.new_bv_concrete(0b101, 4), None, false, &mut (*context).fork_sink, None), &mut (*context).fork_sink) {
        unimplemented!();
    }
    else if (*context).scfia.check_condition(&(*context).scfia.new_bool_eq(&opcode.clone(), &(*context).scfia.new_bv_concrete(0b110, 4), None, false, &mut (*context).fork_sink, None), &mut (*context).fork_sink) {
        unimplemented!();
    }
    else if (*context).scfia.check_condition(&(*context).scfia.new_bool_eq(&opcode.clone(), &(*context).scfia.new_bv_concrete(0b111, 4), None, false, &mut (*context).fork_sink, None), &mut (*context).fork_sink) {
        unimplemented!();
    }
    else if (*context).scfia.check_condition(&(*context).scfia.new_bool_eq(&opcode.clone(), &(*context).scfia.new_bv_concrete(0b1000, 4), None, false, &mut (*context).fork_sink, None), &mut (*context).fork_sink) {
        unimplemented!();
    }
    else if (*context).scfia.check_condition(&(*context).scfia.new_bool_eq(&opcode.clone(), &(*context).scfia.new_bv_concrete(0b1001, 4), None, false, &mut (*context).fork_sink, None), &mut (*context).fork_sink) {
        unimplemented!();
    }
    else if (*context).scfia.check_condition(&(*context).scfia.new_bool_eq(&opcode.clone(), &(*context).scfia.new_bv_concrete(0b1010, 4), None, false, &mut (*context).fork_sink, None), &mut (*context).fork_sink) {
        let rn: ActiveValue<STM32ScfiaComposition> = (*context).scfia.new_bv_slice(&instruction.clone(), 2, 0, None, &mut (*context).fork_sink, None);
        let rm: ActiveValue<STM32ScfiaComposition> = (*context).scfia.new_bv_slice(&instruction.clone(), 5, 3, None, &mut (*context).fork_sink, None);
        let not: ActiveValue<STM32ScfiaComposition> = (*context).scfia.new_bv_concrete(0b0, 32);
        let result: ActiveValue<STM32ScfiaComposition> = (*context).scfia.new_bv_add(&_register_read_BV32(rn.clone(), state, context), &not.clone(), 32, None, &mut (*context).fork_sink, None);
        let carry: ActiveValue<STM32ScfiaComposition> = (*context).scfia.new_bv_concrete(0b0, 1);
        let overflow: ActiveValue<STM32ScfiaComposition> = (*context).scfia.new_bv_concrete(0b0, 1);
        (*state).apsr.N = (*context).scfia.new_bv_slice(&result.clone(), 31, 31, None, &mut (*context).fork_sink, None);
        (*state).apsr.Z = _is_zero_bit_BV32(result.clone(), context);
        (*state).apsr.C = carry.clone();
        (*state).apsr.V = overflow.clone();
        unimplemented!();
    }
    else if (*context).scfia.check_condition(&(*context).scfia.new_bool_eq(&opcode.clone(), &(*context).scfia.new_bv_concrete(0b1011, 4), None, false, &mut (*context).fork_sink, None), &mut (*context).fork_sink) {
        unimplemented!();
    }
    else if (*context).scfia.check_condition(&(*context).scfia.new_bool_eq(&opcode.clone(), &(*context).scfia.new_bv_concrete(0b1100, 4), None, false, &mut (*context).fork_sink, None), &mut (*context).fork_sink) {
        unimplemented!();
    }
    else if (*context).scfia.check_condition(&(*context).scfia.new_bool_eq(&opcode.clone(), &(*context).scfia.new_bv_concrete(0b1101, 4), None, false, &mut (*context).fork_sink, None), &mut (*context).fork_sink) {
        unimplemented!();
    }
    else if (*context).scfia.check_condition(&(*context).scfia.new_bool_eq(&opcode.clone(), &(*context).scfia.new_bv_concrete(0b1110, 4), None, false, &mut (*context).fork_sink, None), &mut (*context).fork_sink) {
        unimplemented!();
    }
    else if (*context).scfia.check_condition(&(*context).scfia.new_bool_eq(&opcode.clone(), &(*context).scfia.new_bv_concrete(0b1111, 4), None, false, &mut (*context).fork_sink, None), &mut (*context).fork_sink) {
        unimplemented!();
    }
}

unsafe fn _thumb16_load_from_literal_pool(instruction: ActiveValue<STM32ScfiaComposition>, state: *mut SystemState, context: *mut StepContext<STM32ScfiaComposition>) {
    let bit_11: ActiveValue<STM32ScfiaComposition> = (*context).scfia.new_bv_slice(&instruction.clone(), 11, 11, None, &mut (*context).fork_sink, None);
    if (*context).scfia.check_condition(&(*context).scfia.new_bool_eq(&bit_11.clone(), &(*context).scfia.new_bv_concrete(0b1, 1), None, false, &mut (*context).fork_sink, None), &mut (*context).fork_sink) {
        let rt: ActiveValue<STM32ScfiaComposition> = (*context).scfia.new_bv_slice(&instruction.clone(), 10, 8, None, &mut (*context).fork_sink, None);
        let imm32: ActiveValue<STM32ScfiaComposition> = (*context).scfia.new_bv_concat(&(*context).scfia.new_bv_concrete(0b0, 22), &(*context).scfia.new_bv_concat(&(*context).scfia.new_bv_slice(&instruction.clone(), 7, 0, None, &mut (*context).fork_sink, None), &(*context).scfia.new_bv_concrete(0b0, 2), 10, None, &mut (*context).fork_sink, None), 32, None, &mut (*context).fork_sink, None);
        let add: ActiveValue<STM32ScfiaComposition> = (*context).scfia.new_bv_concrete(0b1, 1);
        _load_register(rt.clone(), imm32.clone(), add.clone(), state, context);
    }
    else {
        unimplemented!();
    }
}

unsafe fn _thumb16_load_store_single_data_item(instruction: ActiveValue<STM32ScfiaComposition>, state: *mut SystemState, context: *mut StepContext<STM32ScfiaComposition>) {
    let op_a: ActiveValue<STM32ScfiaComposition> = (*context).scfia.new_bv_slice(&instruction.clone(), 15, 12, None, &mut (*context).fork_sink, None);
    let op_b: ActiveValue<STM32ScfiaComposition> = (*context).scfia.new_bv_slice(&instruction.clone(), 11, 9, None, &mut (*context).fork_sink, None);
    if (*context).scfia.check_condition(&(*context).scfia.new_bool_eq(&op_a.clone(), &(*context).scfia.new_bv_concrete(0b101, 4), None, false, &mut (*context).fork_sink, None), &mut (*context).fork_sink) {
        unimplemented!();
    }
    else if (*context).scfia.check_condition(&(*context).scfia.new_bool_eq(&op_a.clone(), &(*context).scfia.new_bv_concrete(0b110, 4), None, false, &mut (*context).fork_sink, None), &mut (*context).fork_sink) {
        unimplemented!();
    }
    else if (*context).scfia.check_condition(&(*context).scfia.new_bool_eq(&op_a.clone(), &(*context).scfia.new_bv_concrete(0b111, 4), None, false, &mut (*context).fork_sink, None), &mut (*context).fork_sink) {
        unimplemented!();
    }
    else if (*context).scfia.check_condition(&(*context).scfia.new_bool_eq(&op_a.clone(), &(*context).scfia.new_bv_concrete(0b1000, 4), None, false, &mut (*context).fork_sink, None), &mut (*context).fork_sink) {
        unimplemented!();
    }
    else if (*context).scfia.check_condition(&(*context).scfia.new_bool_eq(&op_a.clone(), &(*context).scfia.new_bv_concrete(0b1001, 4), None, false, &mut (*context).fork_sink, None), &mut (*context).fork_sink) {
        unimplemented!();
    }
    else {
        unimplemented!();
    }
}

unsafe fn _thumb16_unconditional_branch(instruction: ActiveValue<STM32ScfiaComposition>, state: *mut SystemState, context: *mut StepContext<STM32ScfiaComposition>) {
    let imm11: ActiveValue<STM32ScfiaComposition> = (*context).scfia.new_bv_slice(&instruction.clone(), 10, 0, None, &mut (*context).fork_sink, None);
    let imm32: ActiveValue<STM32ScfiaComposition> = (*context).scfia.new_bv_sign_extend(&(*context).scfia.new_bv_concat(&imm11.clone(), &(*context).scfia.new_bv_concrete(0b0, 1), 12, None, &mut (*context).fork_sink, None), 12, 32, None, &mut (*context).fork_sink, None);
    let value: ActiveValue<STM32ScfiaComposition> = (*context).scfia.new_bv_add(&(*state).PC.clone(), &imm32.clone(), 32, None, &mut (*context).fork_sink, None);
    let value_why_do_I_do_this: ActiveValue<STM32ScfiaComposition> = (*context).scfia.new_bv_add(&value.clone(), &(*context).scfia.new_bv_concrete(0b10, 32), 32, None, &mut (*context).fork_sink, None);
    _branch_write_pc(value_why_do_I_do_this.clone(), state, context);
}

unsafe fn _mov_immediate(rd: ActiveValue<STM32ScfiaComposition>, setflags: ActiveValue<STM32ScfiaComposition>, imm32: ActiveValue<STM32ScfiaComposition>, carry: ActiveValue<STM32ScfiaComposition>, state: *mut SystemState, context: *mut StepContext<STM32ScfiaComposition>) {
    _register_write_BV32(rd.clone(), imm32.clone(), state, context);
    if (*context).scfia.check_condition(&(*context).scfia.new_bool_eq(&(*context).scfia.new_bv_concrete(0b1, 1), &setflags.clone(), None, false, &mut (*context).fork_sink, None), &mut (*context).fork_sink) {
        (*state).apsr.N = (*context).scfia.new_bv_slice(&imm32.clone(), 31, 31, None, &mut (*context).fork_sink, None);
        (*state).apsr.Z = _is_zero_bit_BV32(imm32.clone(), context);
        (*state).apsr.C = carry.clone();
    }
}

unsafe fn _register_write_BV32(register_id: ActiveValue<STM32ScfiaComposition>, value: ActiveValue<STM32ScfiaComposition>, state: *mut SystemState, context: *mut StepContext<STM32ScfiaComposition>) {
    if (*context).scfia.check_condition(&(*context).scfia.new_bool_eq(&register_id.clone(), &(*context).scfia.new_bv_concrete(0b0, 3), None, false, &mut (*context).fork_sink, None), &mut (*context).fork_sink) {
        (*state).R0 = value.clone();
    }
    else if (*context).scfia.check_condition(&(*context).scfia.new_bool_eq(&register_id.clone(), &(*context).scfia.new_bv_concrete(0b1, 3), None, false, &mut (*context).fork_sink, None), &mut (*context).fork_sink) {
        (*state).R1 = value.clone();
    }
    else if (*context).scfia.check_condition(&(*context).scfia.new_bool_eq(&register_id.clone(), &(*context).scfia.new_bv_concrete(0b10, 3), None, false, &mut (*context).fork_sink, None), &mut (*context).fork_sink) {
        (*state).R2 = value.clone();
    }
    else if (*context).scfia.check_condition(&(*context).scfia.new_bool_eq(&register_id.clone(), &(*context).scfia.new_bv_concrete(0b11, 3), None, false, &mut (*context).fork_sink, None), &mut (*context).fork_sink) {
        (*state).R3 = value.clone();
    }
    else if (*context).scfia.check_condition(&(*context).scfia.new_bool_eq(&register_id.clone(), &(*context).scfia.new_bv_concrete(0b100, 3), None, false, &mut (*context).fork_sink, None), &mut (*context).fork_sink) {
        (*state).R4 = value.clone();
    }
    else if (*context).scfia.check_condition(&(*context).scfia.new_bool_eq(&register_id.clone(), &(*context).scfia.new_bv_concrete(0b101, 3), None, false, &mut (*context).fork_sink, None), &mut (*context).fork_sink) {
        (*state).R5 = value.clone();
    }
    else if (*context).scfia.check_condition(&(*context).scfia.new_bool_eq(&register_id.clone(), &(*context).scfia.new_bv_concrete(0b110, 3), None, false, &mut (*context).fork_sink, None), &mut (*context).fork_sink) {
        (*state).R6 = value.clone();
    }
    else if (*context).scfia.check_condition(&(*context).scfia.new_bool_eq(&register_id.clone(), &(*context).scfia.new_bv_concrete(0b111, 3), None, false, &mut (*context).fork_sink, None), &mut (*context).fork_sink) {
        (*state).R7 = value.clone();
    }
}

unsafe fn _register_read_BV32(register_id: ActiveValue<STM32ScfiaComposition>, state: *mut SystemState, context: *mut StepContext<STM32ScfiaComposition>) -> ActiveValue<STM32ScfiaComposition> {
    if (*context).scfia.check_condition(&(*context).scfia.new_bool_eq(&register_id.clone(), &(*context).scfia.new_bv_concrete(0b0, 3), None, false, &mut (*context).fork_sink, None), &mut (*context).fork_sink) {
        return (*context).scfia.new_bv_concrete(0b0, 32);
    }
    else if (*context).scfia.check_condition(&(*context).scfia.new_bool_eq(&register_id.clone(), &(*context).scfia.new_bv_concrete(0b1, 3), None, false, &mut (*context).fork_sink, None), &mut (*context).fork_sink) {
        return (*state).R1.clone();
    }
    else if (*context).scfia.check_condition(&(*context).scfia.new_bool_eq(&register_id.clone(), &(*context).scfia.new_bv_concrete(0b10, 3), None, false, &mut (*context).fork_sink, None), &mut (*context).fork_sink) {
        return (*state).R2.clone();
    }
    else if (*context).scfia.check_condition(&(*context).scfia.new_bool_eq(&register_id.clone(), &(*context).scfia.new_bv_concrete(0b11, 3), None, false, &mut (*context).fork_sink, None), &mut (*context).fork_sink) {
        return (*state).R3.clone();
    }
    else if (*context).scfia.check_condition(&(*context).scfia.new_bool_eq(&register_id.clone(), &(*context).scfia.new_bv_concrete(0b100, 3), None, false, &mut (*context).fork_sink, None), &mut (*context).fork_sink) {
        return (*state).R4.clone();
    }
    else if (*context).scfia.check_condition(&(*context).scfia.new_bool_eq(&register_id.clone(), &(*context).scfia.new_bv_concrete(0b101, 3), None, false, &mut (*context).fork_sink, None), &mut (*context).fork_sink) {
        return (*state).R5.clone();
    }
    else if (*context).scfia.check_condition(&(*context).scfia.new_bool_eq(&register_id.clone(), &(*context).scfia.new_bv_concrete(0b110, 3), None, false, &mut (*context).fork_sink, None), &mut (*context).fork_sink) {
        return (*state).R6.clone();
    }
    else if (*context).scfia.check_condition(&(*context).scfia.new_bool_eq(&register_id.clone(), &(*context).scfia.new_bv_concrete(0b111, 3), None, false, &mut (*context).fork_sink, None), &mut (*context).fork_sink) {
        return (*state).R7.clone();
    }
    else {
        unimplemented!();
    }
}

unsafe fn _progress_pc_2(state: *mut SystemState, context: *mut StepContext<STM32ScfiaComposition>) {
    let old_pc: ActiveValue<STM32ScfiaComposition> = (*state).PC.clone();
    let new_pc: ActiveValue<STM32ScfiaComposition> = (*context).scfia.new_bv_add(&old_pc.clone(), &(*context).scfia.new_bv_concrete(0b10, 32), 32, None, &mut (*context).fork_sink, None);
    (*state).PC = new_pc.clone();
}

unsafe fn _progress_pc_4(state: *mut SystemState, context: *mut StepContext<STM32ScfiaComposition>) {
    let old_pc: ActiveValue<STM32ScfiaComposition> = (*state).PC.clone();
    let new_pc: ActiveValue<STM32ScfiaComposition> = (*context).scfia.new_bv_add(&old_pc.clone(), &(*context).scfia.new_bv_concrete(0b100, 32), 32, None, &mut (*context).fork_sink, None);
    (*state).PC = new_pc.clone();
}

unsafe fn _add_with_carry_BV32(x: ActiveValue<STM32ScfiaComposition>, y: ActiveValue<STM32ScfiaComposition>, carry_in: ActiveValue<STM32ScfiaComposition>, context: *mut StepContext<STM32ScfiaComposition>) -> ActiveValue<STM32ScfiaComposition> {
    let unsigned_sum: ActiveValue<STM32ScfiaComposition> = (*context).scfia.new_bv_add(&(*context).scfia.new_bv_add(&(*context).scfia.new_bv_concat(&(*context).scfia.new_bv_concrete(0b0, 2), &x.clone(), 34, None, &mut (*context).fork_sink, None), &(*context).scfia.new_bv_concat(&(*context).scfia.new_bv_concrete(0b0, 2), &y.clone(), 34, None, &mut (*context).fork_sink, None), 34, None, &mut (*context).fork_sink, None), &(*context).scfia.new_bv_concat(&(*context).scfia.new_bv_concrete(0b0, 33), &carry_in.clone(), 34, None, &mut (*context).fork_sink, None), 34, None, &mut (*context).fork_sink, None);
    let signed_sum: ActiveValue<STM32ScfiaComposition> = (*context).scfia.new_bv_add(&(*context).scfia.new_bv_add(&(*context).scfia.new_bv_sign_extend(&x.clone(), 32, 34, None, &mut (*context).fork_sink, None), &(*context).scfia.new_bv_sign_extend(&y.clone(), 32, 34, None, &mut (*context).fork_sink, None), 34, None, &mut (*context).fork_sink, None), &(*context).scfia.new_bv_concat(&(*context).scfia.new_bv_concrete(0b0, 33), &carry_in.clone(), 34, None, &mut (*context).fork_sink, None), 34, None, &mut (*context).fork_sink, None);
    let result: ActiveValue<STM32ScfiaComposition> = (*context).scfia.new_bv_slice(&unsigned_sum.clone(), 31, 0, None, &mut (*context).fork_sink, None);
    let carry_out: ActiveValue<STM32ScfiaComposition> = _is_not_eq_BV34((*context).scfia.new_bv_concat(&(*context).scfia.new_bv_concrete(0b0, 2), &result.clone(), 34, None, &mut (*context).fork_sink, None), unsigned_sum.clone(), context);
    let overflow: ActiveValue<STM32ScfiaComposition> = _is_not_eq_BV34((*context).scfia.new_bv_sign_extend(&result.clone(), 32, 34, None, &mut (*context).fork_sink, None), unsigned_sum.clone(), context);
    return (*context).scfia.new_bv_concat(&result.clone(), &(*context).scfia.new_bv_concat(&carry_out.clone(), &overflow.clone(), 2, None, &mut (*context).fork_sink, None), 34, None, &mut (*context).fork_sink, None);
}

unsafe fn _is_not_eq_BV34(x: ActiveValue<STM32ScfiaComposition>, y: ActiveValue<STM32ScfiaComposition>, context: *mut StepContext<STM32ScfiaComposition>) -> ActiveValue<STM32ScfiaComposition> {
    if (*context).scfia.check_condition(&(*context).scfia.new_bool_eq(&x.clone(), &y.clone(), None, false, &mut (*context).fork_sink, None), &mut (*context).fork_sink) {
        return (*context).scfia.new_bv_concrete(0b0, 1);
    }
    else {
        return (*context).scfia.new_bv_concrete(0b1, 1);
    }
}

unsafe fn _is_zero_bit_BV32(value: ActiveValue<STM32ScfiaComposition>, context: *mut StepContext<STM32ScfiaComposition>) -> ActiveValue<STM32ScfiaComposition> {
    if (*context).scfia.check_condition(&(*context).scfia.new_bool_eq(&value.clone(), &(*context).scfia.new_bv_concrete(0b0, 32), None, false, &mut (*context).fork_sink, None), &mut (*context).fork_sink) {
        return (*context).scfia.new_bv_concrete(0b1, 1);
    }
    else {
        return (*context).scfia.new_bv_concrete(0b0, 1);
    }
}

unsafe fn _load_register(t: ActiveValue<STM32ScfiaComposition>, imm32: ActiveValue<STM32ScfiaComposition>, add: ActiveValue<STM32ScfiaComposition>, state: *mut SystemState, context: *mut StepContext<STM32ScfiaComposition>) {
    let base: ActiveValue<STM32ScfiaComposition> = (*context).scfia.new_bv_or(&(*state).PC.clone(), &(*context).scfia.new_bv_concrete(0b11, 32), 32, None, &mut (*context).fork_sink, None);
    if (*context).scfia.check_condition(&(*context).scfia.new_bool_eq(&add.clone(), &(*context).scfia.new_bv_concrete(0b1, 1), None, false, &mut (*context).fork_sink, None), &mut (*context).fork_sink) {
        let address: ActiveValue<STM32ScfiaComposition> = (*context).scfia.new_bv_add(&base.clone(), &imm32.clone(), 32, None, &mut (*context).fork_sink, None);
        _register_write_BV32(t.clone(), (*(*context).memory).read(&address.clone(), 32, (*context).scfia.clone(), &mut (*context).hints, &mut (*context).fork_sink), state, context);
    }
    else {
        let address: ActiveValue<STM32ScfiaComposition> = (*context).scfia.new_bv_sub(&base.clone(), &imm32.clone(), 32, None, &mut (*context).fork_sink, None);
        _register_write_BV32(t.clone(), (*(*context).memory).read(&address.clone(), 32, (*context).scfia.clone(), &mut (*context).hints, &mut (*context).fork_sink), state, context);
    }
}

unsafe fn _branch_write_pc(address: ActiveValue<STM32ScfiaComposition>, state: *mut SystemState, context: *mut StepContext<STM32ScfiaComposition>) {
    let address: ActiveValue<STM32ScfiaComposition> = (*context).scfia.new_bv_concat(&(*context).scfia.new_bv_slice(&address.clone(), 31, 1, None, &mut (*context).fork_sink, None), &(*context).scfia.new_bv_concrete(0b0, 1), 32, None, &mut (*context).fork_sink, None);
    _branch_to(address.clone(), state, context);
}

unsafe fn _branch_to(address: ActiveValue<STM32ScfiaComposition>, state: *mut SystemState, context: *mut StepContext<STM32ScfiaComposition>) {
    (*state).PC = address.clone();
}
