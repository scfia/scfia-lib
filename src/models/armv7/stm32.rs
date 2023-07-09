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
        // debug!("EPSR:\tdepth={}", self.state.EPSR.get_depth());
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
pub struct ExecutionProgramStatusRegister {
    pub ICI_IT: ActiveValue<STM32ScfiaComposition>,
    pub T: ActiveValue<STM32ScfiaComposition>,
    pub ICI_IT2: ActiveValue<STM32ScfiaComposition>,
}

impl ExecutionProgramStatusRegister {
    fn clone_to_stdlib(&self, cloned_scfia: &Scfia<STM32ScfiaComposition>, cloned_actives: &mut BTreeMap<u64, ActiveValue<STM32ScfiaComposition>>, cloned_inactives: &mut BTreeMap<u64, RetiredValue<STM32ScfiaComposition>>) -> ExecutionProgramStatusRegister {
        ExecutionProgramStatusRegister {
            ICI_IT: self.ICI_IT.clone_to_stdlib(cloned_scfia, cloned_actives, cloned_inactives),
            T: self.T.clone_to_stdlib(cloned_scfia, cloned_actives, cloned_inactives),
            ICI_IT2: self.ICI_IT2.clone_to_stdlib(cloned_scfia, cloned_actives, cloned_inactives),
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
    pub EPSR: ExecutionProgramStatusRegister,
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
            EPSR: self.EPSR.clone_to_stdlib(cloned_scfia, cloned_actives, cloned_inactives),
        }
    }
}

unsafe fn _step(state: *mut SystemState, context: *mut StepContext<STM32ScfiaComposition>) {
    let mut address_low: ActiveValue<STM32ScfiaComposition> = (*context).scfia.new_bv_slice(&(*state).PC.clone(), 1, 0, None, &mut (*context).fork_sink, None);
    if (*context).scfia.check_condition(&(*context).scfia.new_bool_eq(&address_low.clone(), &(*context).scfia.new_bv_concrete(0b0, 2), None, false, &mut (*context).fork_sink, None), &mut (*context).fork_sink) {
        let mut instruction32: ActiveValue<STM32ScfiaComposition> = (*(*context).memory).read(&(*state).PC.clone(), 32, (*context).scfia.clone(), &mut (*context).hints, &mut (*context).fork_sink);
        _progress_pc_4(state, context);
        let mut b5: ActiveValue<STM32ScfiaComposition> = (*context).scfia.new_bv_slice(&instruction32.clone(), 15, 11, None, &mut (*context).fork_sink, None);
        if (*context).scfia.check_condition(&(*context).scfia.new_bool_eq(&b5.clone(), &(*context).scfia.new_bv_concrete(0b11101, 5), None, false, &mut (*context).fork_sink, None), &mut (*context).fork_sink) {
            _thumb32(instruction32.clone(), state, context);
        }
        else if (*context).scfia.check_condition(&(*context).scfia.new_bool_eq(&b5.clone(), &(*context).scfia.new_bv_concrete(0b11110, 5), None, false, &mut (*context).fork_sink, None), &mut (*context).fork_sink) {
            _thumb32(instruction32.clone(), state, context);
        }
        else if (*context).scfia.check_condition(&(*context).scfia.new_bool_eq(&b5.clone(), &(*context).scfia.new_bv_concrete(0b11111, 5), None, false, &mut (*context).fork_sink, None), &mut (*context).fork_sink) {
            _thumb32(instruction32.clone(), state, context);
        }
        else {
            let mut old_pc: ActiveValue<STM32ScfiaComposition> = (*state).PC.clone();
            _thumb16((*context).scfia.new_bv_slice(&instruction32.clone(), 15, 0, None, &mut (*context).fork_sink, None), state, context);
            if (*context).scfia.check_condition(&(*context).scfia.new_bool_eq(&old_pc.clone(), &(*state).PC.clone(), None, false, &mut (*context).fork_sink, None), &mut (*context).fork_sink) {
                let mut b5: ActiveValue<STM32ScfiaComposition> = (*context).scfia.new_bv_slice(&instruction32.clone(), 31, 27, None, &mut (*context).fork_sink, None);
                if (*context).scfia.check_condition(&(*context).scfia.new_bool_eq(&b5.clone(), &(*context).scfia.new_bv_concrete(0b11101, 5), None, false, &mut (*context).fork_sink, None), &mut (*context).fork_sink) {
                    instruction32 = (*context).scfia.new_bv_concat(&(*(*context).memory).read(&(*state).PC.clone(), 16, (*context).scfia.clone(), &mut (*context).hints, &mut (*context).fork_sink), &(*context).scfia.new_bv_slice(&instruction32.clone(), 31, 16, None, &mut (*context).fork_sink, None), 32, None, &mut (*context).fork_sink, None);
                    _progress_pc_2(state, context);
                    _thumb32(instruction32.clone(), state, context);
                }
                else if (*context).scfia.check_condition(&(*context).scfia.new_bool_eq(&b5.clone(), &(*context).scfia.new_bv_concrete(0b11110, 5), None, false, &mut (*context).fork_sink, None), &mut (*context).fork_sink) {
                    instruction32 = (*context).scfia.new_bv_concat(&(*(*context).memory).read(&(*state).PC.clone(), 16, (*context).scfia.clone(), &mut (*context).hints, &mut (*context).fork_sink), &(*context).scfia.new_bv_slice(&instruction32.clone(), 31, 16, None, &mut (*context).fork_sink, None), 32, None, &mut (*context).fork_sink, None);
                    _progress_pc_2(state, context);
                    _thumb32(instruction32.clone(), state, context);
                }
                else if (*context).scfia.check_condition(&(*context).scfia.new_bool_eq(&b5.clone(), &(*context).scfia.new_bv_concrete(0b11111, 5), None, false, &mut (*context).fork_sink, None), &mut (*context).fork_sink) {
                    instruction32 = (*context).scfia.new_bv_concat(&(*(*context).memory).read(&(*state).PC.clone(), 16, (*context).scfia.clone(), &mut (*context).hints, &mut (*context).fork_sink), &(*context).scfia.new_bv_slice(&instruction32.clone(), 31, 16, None, &mut (*context).fork_sink, None), 32, None, &mut (*context).fork_sink, None);
                    _progress_pc_2(state, context);
                    _thumb32(instruction32.clone(), state, context);
                }
                else {
                    _thumb16((*context).scfia.new_bv_slice(&instruction32.clone(), 31, 16, None, &mut (*context).fork_sink, None), state, context);
                }
            }
        }
    }
    else if (*context).scfia.check_condition(&(*context).scfia.new_bool_eq(&address_low.clone(), &(*context).scfia.new_bv_concrete(0b1, 2), None, false, &mut (*context).fork_sink, None), &mut (*context).fork_sink) {
        unimplemented!();
    }
    else if (*context).scfia.check_condition(&(*context).scfia.new_bool_eq(&address_low.clone(), &(*context).scfia.new_bv_concrete(0b10, 2), None, false, &mut (*context).fork_sink, None), &mut (*context).fork_sink) {
        let mut instruction: ActiveValue<STM32ScfiaComposition> = (*(*context).memory).read(&(*state).PC.clone(), 16, (*context).scfia.clone(), &mut (*context).hints, &mut (*context).fork_sink);
        _progress_pc_2(state, context);
        let mut b5: ActiveValue<STM32ScfiaComposition> = (*context).scfia.new_bv_slice(&instruction.clone(), 15, 11, None, &mut (*context).fork_sink, None);
        if (*context).scfia.check_condition(&(*context).scfia.new_bool_eq(&b5.clone(), &(*context).scfia.new_bv_concrete(0b11101, 5), None, false, &mut (*context).fork_sink, None), &mut (*context).fork_sink) {
            let mut instruction32: ActiveValue<STM32ScfiaComposition> = (*context).scfia.new_bv_concat(&(*(*context).memory).read(&(*state).PC.clone(), 16, (*context).scfia.clone(), &mut (*context).hints, &mut (*context).fork_sink), &instruction.clone(), 32, None, &mut (*context).fork_sink, None);
            _progress_pc_2(state, context);
            _thumb32(instruction32.clone(), state, context);
        }
        else if (*context).scfia.check_condition(&(*context).scfia.new_bool_eq(&b5.clone(), &(*context).scfia.new_bv_concrete(0b11110, 5), None, false, &mut (*context).fork_sink, None), &mut (*context).fork_sink) {
            let mut instruction32: ActiveValue<STM32ScfiaComposition> = (*context).scfia.new_bv_concat(&(*(*context).memory).read(&(*state).PC.clone(), 16, (*context).scfia.clone(), &mut (*context).hints, &mut (*context).fork_sink), &instruction.clone(), 32, None, &mut (*context).fork_sink, None);
            _progress_pc_2(state, context);
            _thumb32(instruction32.clone(), state, context);
        }
        else if (*context).scfia.check_condition(&(*context).scfia.new_bool_eq(&b5.clone(), &(*context).scfia.new_bv_concrete(0b11111, 5), None, false, &mut (*context).fork_sink, None), &mut (*context).fork_sink) {
            let mut instruction32: ActiveValue<STM32ScfiaComposition> = (*context).scfia.new_bv_concat(&(*(*context).memory).read(&(*state).PC.clone(), 16, (*context).scfia.clone(), &mut (*context).hints, &mut (*context).fork_sink), &instruction.clone(), 32, None, &mut (*context).fork_sink, None);
            _progress_pc_2(state, context);
            _thumb32(instruction32.clone(), state, context);
        }
        else {
            let mut old_pc: ActiveValue<STM32ScfiaComposition> = (*state).PC.clone();
            _thumb16(instruction.clone(), state, context);
        }
    }
    else if (*context).scfia.check_condition(&(*context).scfia.new_bool_eq(&address_low.clone(), &(*context).scfia.new_bv_concrete(0b11, 2), None, false, &mut (*context).fork_sink, None), &mut (*context).fork_sink) {
        unimplemented!();
    }
}

unsafe fn _thumb32(instruction: ActiveValue<STM32ScfiaComposition>, state: *mut SystemState, context: *mut StepContext<STM32ScfiaComposition>) {
    let mut instruction2: ActiveValue<STM32ScfiaComposition> = (*context).scfia.new_bv_slice(&instruction.clone(), 31, 16, None, &mut (*context).fork_sink, None);
    let mut instruction1: ActiveValue<STM32ScfiaComposition> = (*context).scfia.new_bv_slice(&instruction.clone(), 15, 0, None, &mut (*context).fork_sink, None);
    let mut op1: ActiveValue<STM32ScfiaComposition> = (*context).scfia.new_bv_slice(&instruction1.clone(), 12, 11, None, &mut (*context).fork_sink, None);
    let mut op2: ActiveValue<STM32ScfiaComposition> = (*context).scfia.new_bv_slice(&instruction1.clone(), 10, 4, None, &mut (*context).fork_sink, None);
    let mut op: ActiveValue<STM32ScfiaComposition> = (*context).scfia.new_bv_slice(&instruction2.clone(), 15, 15, None, &mut (*context).fork_sink, None);
    if (*context).scfia.check_condition(&(*context).scfia.new_bool_eq(&op1.clone(), &(*context).scfia.new_bv_concrete(0b1, 2), None, false, &mut (*context).fork_sink, None), &mut (*context).fork_sink) {
        unimplemented!();
    }
    else if (*context).scfia.check_condition(&(*context).scfia.new_bool_eq(&op1.clone(), &(*context).scfia.new_bv_concrete(0b10, 2), None, false, &mut (*context).fork_sink, None), &mut (*context).fork_sink) {
        if (*context).scfia.check_condition(&(*context).scfia.new_bool_eq(&op.clone(), &(*context).scfia.new_bv_concrete(0b0, 1), None, false, &mut (*context).fork_sink, None), &mut (*context).fork_sink) {
            let mut pbi: ActiveValue<STM32ScfiaComposition> = (*context).scfia.new_bv_slice(&op2.clone(), 5, 5, None, &mut (*context).fork_sink, None);
            if (*context).scfia.check_condition(&(*context).scfia.new_bool_eq(&pbi.clone(), &(*context).scfia.new_bv_concrete(0b0, 1), None, false, &mut (*context).fork_sink, None), &mut (*context).fork_sink) {
                _thumb32_data_processing_modified_immediate(instruction1.clone(), instruction2.clone(), state, context);
            }
            else {
                unimplemented!();
            }
        }
        else {
            _thumb32_branches_and_misc_control(instruction1.clone(), instruction2.clone(), state, context);
        }
    }
    else if (*context).scfia.check_condition(&(*context).scfia.new_bool_eq(&op1.clone(), &(*context).scfia.new_bv_concrete(0b11, 2), None, false, &mut (*context).fork_sink, None), &mut (*context).fork_sink) {
        if (*context).scfia.check_condition(&(*context).scfia.new_bool_eq(&_matches_BV7(op2.clone(), (*context).scfia.new_bv_concrete(0b0, 7), (*context).scfia.new_bv_concrete(0b1110001, 7), context), &(*context).scfia.new_bv_concrete(0b1, 1), None, false, &mut (*context).fork_sink, None), &mut (*context).fork_sink) {
            _thumb32_store_single_data_item(instruction1.clone(), instruction2.clone(), state, context);
        }
        else if (*context).scfia.check_condition(&(*context).scfia.new_bool_eq(&_matches_BV7(op2.clone(), (*context).scfia.new_bv_concrete(0b1, 7), (*context).scfia.new_bv_concrete(0b1100110, 7), context), &(*context).scfia.new_bv_concrete(0b1, 1), None, false, &mut (*context).fork_sink, None), &mut (*context).fork_sink) {
            unimplemented!();
        }
        else if (*context).scfia.check_condition(&(*context).scfia.new_bool_eq(&_matches_BV7(op2.clone(), (*context).scfia.new_bv_concrete(0b11, 7), (*context).scfia.new_bv_concrete(0b1100100, 7), context), &(*context).scfia.new_bv_concrete(0b1, 1), None, false, &mut (*context).fork_sink, None), &mut (*context).fork_sink) {
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

unsafe fn _thumb32_data_processing_modified_immediate(instruction1: ActiveValue<STM32ScfiaComposition>, instruction2: ActiveValue<STM32ScfiaComposition>, state: *mut SystemState, context: *mut StepContext<STM32ScfiaComposition>) {
    let mut op: ActiveValue<STM32ScfiaComposition> = (*context).scfia.new_bv_slice(&instruction1.clone(), 8, 4, None, &mut (*context).fork_sink, None);
    let mut rn: ActiveValue<STM32ScfiaComposition> = (*context).scfia.new_bv_slice(&instruction1.clone(), 3, 0, None, &mut (*context).fork_sink, None);
    let mut rd: ActiveValue<STM32ScfiaComposition> = (*context).scfia.new_bv_slice(&instruction2.clone(), 11, 8, None, &mut (*context).fork_sink, None);
    if (*context).scfia.check_condition(&(*context).scfia.new_bool_eq(&_matches_BV5(op.clone(), (*context).scfia.new_bv_concrete(0b0, 5), (*context).scfia.new_bv_concrete(0b11110, 5), context), &(*context).scfia.new_bv_concrete(0b1, 1), None, false, &mut (*context).fork_sink, None), &mut (*context).fork_sink) {
        if (*context).scfia.check_condition(&(*context).scfia.new_bool_not(&(*context).scfia.new_bool_eq(&rd.clone(), &(*context).scfia.new_bv_concrete(0b111, 4), None, false, &mut (*context).fork_sink, None), None, false, &mut (*context).fork_sink, None), &mut (*context).fork_sink) {
            unimplemented!();
        }
        else {
            unimplemented!();
        }
    }
    else if (*context).scfia.check_condition(&(*context).scfia.new_bool_eq(&_matches_BV5(op.clone(), (*context).scfia.new_bv_concrete(0b10, 5), (*context).scfia.new_bv_concrete(0b11100, 5), context), &(*context).scfia.new_bv_concrete(0b1, 1), None, false, &mut (*context).fork_sink, None), &mut (*context).fork_sink) {
        let mut setflags: ActiveValue<STM32ScfiaComposition> = (*context).scfia.new_bv_slice(&instruction1.clone(), 4, 4, None, &mut (*context).fork_sink, None);
        let mut imm8: ActiveValue<STM32ScfiaComposition> = (*context).scfia.new_bv_slice(&instruction2.clone(), 7, 0, None, &mut (*context).fork_sink, None);
        let mut imm3: ActiveValue<STM32ScfiaComposition> = (*context).scfia.new_bv_slice(&instruction2.clone(), 14, 12, None, &mut (*context).fork_sink, None);
        let mut i: ActiveValue<STM32ScfiaComposition> = (*context).scfia.new_bv_slice(&instruction1.clone(), 10, 10, None, &mut (*context).fork_sink, None);
        let mut imm11: ActiveValue<STM32ScfiaComposition> = (*context).scfia.new_bv_concat(&imm3.clone(), &imm8.clone(), 11, None, &mut (*context).fork_sink, None);
        let mut imm12: ActiveValue<STM32ScfiaComposition> = (*context).scfia.new_bv_concat(&i.clone(), &imm11.clone(), 12, None, &mut (*context).fork_sink, None);
        let mut imm32_carry: ActiveValue<STM32ScfiaComposition> = _thumb_expand_imm_c(imm12.clone(), (*state).apsr.C.clone(), context);
        let mut imm32: ActiveValue<STM32ScfiaComposition> = (*context).scfia.new_bv_slice(&imm32_carry.clone(), 32, 1, None, &mut (*context).fork_sink, None);
        let mut carry: ActiveValue<STM32ScfiaComposition> = (*context).scfia.new_bv_slice(&imm32_carry.clone(), 0, 0, None, &mut (*context).fork_sink, None);
        let mut result: ActiveValue<STM32ScfiaComposition> = (*context).scfia.new_bv_and(&_register_read_BV32_wide(rn.clone(), state, context), &(*context).scfia.new_bv_not(&imm32.clone(), 32, None, &mut (*context).fork_sink, None), 32, None, &mut (*context).fork_sink, None);
        if (*context).scfia.check_condition(&(*context).scfia.new_bool_eq(&setflags.clone(), &(*context).scfia.new_bv_concrete(0b1, 1), None, false, &mut (*context).fork_sink, None), &mut (*context).fork_sink) {
            (*state).apsr.N = (*context).scfia.new_bv_slice(&result.clone(), 31, 31, None, &mut (*context).fork_sink, None);
            (*state).apsr.Z = _is_zero_bit_BV32(result.clone(), context);
            (*state).apsr.C = carry.clone();
        }
    }
    else if (*context).scfia.check_condition(&(*context).scfia.new_bool_eq(&_matches_BV5(op.clone(), (*context).scfia.new_bv_concrete(0b100, 5), (*context).scfia.new_bv_concrete(0b11010, 5), context), &(*context).scfia.new_bv_concrete(0b1, 1), None, false, &mut (*context).fork_sink, None), &mut (*context).fork_sink) {
        if (*context).scfia.check_condition(&(*context).scfia.new_bool_not(&(*context).scfia.new_bool_eq(&rn.clone(), &(*context).scfia.new_bv_concrete(0b111, 4), None, false, &mut (*context).fork_sink, None), None, false, &mut (*context).fork_sink, None), &mut (*context).fork_sink) {
            let mut d: ActiveValue<STM32ScfiaComposition> = _register_read_BV32_wide(rd.clone(), state, context);
            let mut n: ActiveValue<STM32ScfiaComposition> = _register_read_BV32_wide(rn.clone(), state, context);
            let mut s: ActiveValue<STM32ScfiaComposition> = (*context).scfia.new_bv_slice(&instruction1.clone(), 4, 4, None, &mut (*context).fork_sink, None);
            let mut i: ActiveValue<STM32ScfiaComposition> = (*context).scfia.new_bv_slice(&instruction1.clone(), 10, 10, None, &mut (*context).fork_sink, None);
            let mut imm3: ActiveValue<STM32ScfiaComposition> = (*context).scfia.new_bv_slice(&instruction2.clone(), 14, 12, None, &mut (*context).fork_sink, None);
            let mut imm8: ActiveValue<STM32ScfiaComposition> = (*context).scfia.new_bv_slice(&instruction2.clone(), 7, 0, None, &mut (*context).fork_sink, None);
            let mut imm11: ActiveValue<STM32ScfiaComposition> = (*context).scfia.new_bv_concat(&imm3.clone(), &imm8.clone(), 11, None, &mut (*context).fork_sink, None);
            let mut imm12: ActiveValue<STM32ScfiaComposition> = (*context).scfia.new_bv_concat(&i.clone(), &imm11.clone(), 12, None, &mut (*context).fork_sink, None);
            let mut imm32_carry: ActiveValue<STM32ScfiaComposition> = _thumb_expand_imm_c(imm12.clone(), (*state).apsr.C.clone(), context);
            let mut result: ActiveValue<STM32ScfiaComposition> = (*context).scfia.new_bv_slice(&imm32_carry.clone(), 32, 1, None, &mut (*context).fork_sink, None);
            let mut carry: ActiveValue<STM32ScfiaComposition> = (*context).scfia.new_bv_slice(&imm32_carry.clone(), 0, 0, None, &mut (*context).fork_sink, None);
            _register_write_BV32_wide(rd.clone(), result.clone(), state, context);
            if (*context).scfia.check_condition(&(*context).scfia.new_bool_eq(&s.clone(), &(*context).scfia.new_bv_concrete(0b1, 1), None, false, &mut (*context).fork_sink, None), &mut (*context).fork_sink) {
                (*state).apsr.N = (*context).scfia.new_bv_slice(&result.clone(), 31, 31, None, &mut (*context).fork_sink, None);
                (*state).apsr.Z = _is_zero_bit_BV32(result.clone(), context);
                (*state).apsr.C = carry.clone();
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

unsafe fn _thumb32_branches_and_misc_control(instruction1: ActiveValue<STM32ScfiaComposition>, instruction2: ActiveValue<STM32ScfiaComposition>, state: *mut SystemState, context: *mut StepContext<STM32ScfiaComposition>) {
    let mut op: ActiveValue<STM32ScfiaComposition> = (*context).scfia.new_bv_slice(&instruction1.clone(), 10, 4, None, &mut (*context).fork_sink, None);
    let mut op1: ActiveValue<STM32ScfiaComposition> = (*context).scfia.new_bv_slice(&instruction2.clone(), 14, 12, None, &mut (*context).fork_sink, None);
    if (*context).scfia.check_condition(&(*context).scfia.new_bool_eq(&_matches_BV3(op1.clone(), (*context).scfia.new_bv_concrete(0b0, 3), (*context).scfia.new_bv_concrete(0b101, 3), context), &(*context).scfia.new_bv_concrete(0b1, 1), None, false, &mut (*context).fork_sink, None), &mut (*context).fork_sink) {
        unimplemented!();
    }
    else if (*context).scfia.check_condition(&(*context).scfia.new_bool_eq(&op1.clone(), &(*context).scfia.new_bv_concrete(0b10, 3), None, false, &mut (*context).fork_sink, None), &mut (*context).fork_sink) {
        unimplemented!();
    }
    else if (*context).scfia.check_condition(&(*context).scfia.new_bool_eq(&_matches_BV3(op1.clone(), (*context).scfia.new_bv_concrete(0b1, 3), (*context).scfia.new_bv_concrete(0b100, 3), context), &(*context).scfia.new_bv_concrete(0b1, 1), None, false, &mut (*context).fork_sink, None), &mut (*context).fork_sink) {
        unimplemented!();
    }
    else if (*context).scfia.check_condition(&(*context).scfia.new_bool_eq(&_matches_BV3(op1.clone(), (*context).scfia.new_bv_concrete(0b101, 3), (*context).scfia.new_bv_concrete(0b0, 3), context), &(*context).scfia.new_bv_concrete(0b1, 1), None, false, &mut (*context).fork_sink, None), &mut (*context).fork_sink) {
        let mut s: ActiveValue<STM32ScfiaComposition> = (*context).scfia.new_bv_slice(&instruction1.clone(), 10, 10, None, &mut (*context).fork_sink, None);
        let mut j1: ActiveValue<STM32ScfiaComposition> = (*context).scfia.new_bv_slice(&instruction2.clone(), 13, 13, None, &mut (*context).fork_sink, None);
        let mut j2: ActiveValue<STM32ScfiaComposition> = (*context).scfia.new_bv_slice(&instruction2.clone(), 11, 11, None, &mut (*context).fork_sink, None);
        let mut imm10: ActiveValue<STM32ScfiaComposition> = (*context).scfia.new_bv_slice(&instruction1.clone(), 9, 0, None, &mut (*context).fork_sink, None);
        let mut imm11: ActiveValue<STM32ScfiaComposition> = (*context).scfia.new_bv_slice(&instruction2.clone(), 10, 0, None, &mut (*context).fork_sink, None);
        let mut i1: ActiveValue<STM32ScfiaComposition> = (*context).scfia.new_bv_not(&(*context).scfia.new_bv_xor(&j1.clone(), &s.clone(), 1, None, &mut (*context).fork_sink, None), 1, None, &mut (*context).fork_sink, None);
        let mut i2: ActiveValue<STM32ScfiaComposition> = (*context).scfia.new_bv_not(&(*context).scfia.new_bv_xor(&j2.clone(), &s.clone(), 1, None, &mut (*context).fork_sink, None), 1, None, &mut (*context).fork_sink, None);
        let mut imm1: ActiveValue<STM32ScfiaComposition> = (*context).scfia.new_bv_concat(&imm11.clone(), &(*context).scfia.new_bv_concrete(0b0, 1), 12, None, &mut (*context).fork_sink, None);
        let mut imm2: ActiveValue<STM32ScfiaComposition> = (*context).scfia.new_bv_concat(&imm10.clone(), &imm1.clone(), 22, None, &mut (*context).fork_sink, None);
        let mut imm3: ActiveValue<STM32ScfiaComposition> = (*context).scfia.new_bv_concat(&i2.clone(), &imm2.clone(), 23, None, &mut (*context).fork_sink, None);
        let mut imm4: ActiveValue<STM32ScfiaComposition> = (*context).scfia.new_bv_concat(&i1.clone(), &imm3.clone(), 24, None, &mut (*context).fork_sink, None);
        let mut imm5: ActiveValue<STM32ScfiaComposition> = (*context).scfia.new_bv_concat(&s.clone(), &imm4.clone(), 25, None, &mut (*context).fork_sink, None);
        let mut imm: ActiveValue<STM32ScfiaComposition> = (*context).scfia.new_bv_sign_extend(&imm5.clone(), 25, 32, None, &mut (*context).fork_sink, None);
        let mut lr: ActiveValue<STM32ScfiaComposition> = (*context).scfia.new_bv_concat(&(*context).scfia.new_bv_slice(&(*state).PC.clone(), 31, 1, None, &mut (*context).fork_sink, None), &(*context).scfia.new_bv_concrete(0b1, 1), 32, None, &mut (*context).fork_sink, None);
        _register_write_BV32_wide((*context).scfia.new_bv_concrete(0b1110, 4), lr.clone(), state, context);
        _branch_write_pc((*context).scfia.new_bv_add(&(*state).PC.clone(), &imm.clone(), 32, None, &mut (*context).fork_sink, None), state, context);
    }
    else {
        unimplemented!();
    }
}

unsafe fn _thumb32_store_single_data_item(instruction1: ActiveValue<STM32ScfiaComposition>, instruction2: ActiveValue<STM32ScfiaComposition>, state: *mut SystemState, context: *mut StepContext<STM32ScfiaComposition>) {
    let mut op1: ActiveValue<STM32ScfiaComposition> = (*context).scfia.new_bv_slice(&instruction1.clone(), 7, 5, None, &mut (*context).fork_sink, None);
    let mut op2: ActiveValue<STM32ScfiaComposition> = (*context).scfia.new_bv_slice(&instruction2.clone(), 11, 11, None, &mut (*context).fork_sink, None);
    if (*context).scfia.check_condition(&(*context).scfia.new_bool_eq(&op1.clone(), &(*context).scfia.new_bv_concrete(0b0, 3), None, false, &mut (*context).fork_sink, None), &mut (*context).fork_sink) {
        unimplemented!();
    }
    else if (*context).scfia.check_condition(&(*context).scfia.new_bool_eq(&op1.clone(), &(*context).scfia.new_bv_concrete(0b1, 3), None, false, &mut (*context).fork_sink, None), &mut (*context).fork_sink) {
        unimplemented!();
    }
    else if (*context).scfia.check_condition(&(*context).scfia.new_bool_eq(&op1.clone(), &(*context).scfia.new_bv_concrete(0b10, 3), None, false, &mut (*context).fork_sink, None), &mut (*context).fork_sink) {
        if (*context).scfia.check_condition(&(*context).scfia.new_bool_eq(&op2.clone(), &(*context).scfia.new_bv_concrete(0b1, 1), None, false, &mut (*context).fork_sink, None), &mut (*context).fork_sink) {
            let mut rt: ActiveValue<STM32ScfiaComposition> = (*context).scfia.new_bv_slice(&instruction2.clone(), 15, 12, None, &mut (*context).fork_sink, None);
            let mut rn: ActiveValue<STM32ScfiaComposition> = (*context).scfia.new_bv_slice(&instruction1.clone(), 3, 0, None, &mut (*context).fork_sink, None);
            let mut n: ActiveValue<STM32ScfiaComposition> = _register_read_BV32_wide(rn.clone(), state, context);
            let mut imm8: ActiveValue<STM32ScfiaComposition> = (*context).scfia.new_bv_slice(&instruction2.clone(), 7, 0, None, &mut (*context).fork_sink, None);
            let mut imm32: ActiveValue<STM32ScfiaComposition> = (*context).scfia.new_bv_concat(&(*context).scfia.new_bv_concrete(0b0, 24), &imm8.clone(), 32, None, &mut (*context).fork_sink, None);
            let mut index: ActiveValue<STM32ScfiaComposition> = (*context).scfia.new_bv_slice(&instruction2.clone(), 10, 10, None, &mut (*context).fork_sink, None);
            let mut add: ActiveValue<STM32ScfiaComposition> = (*context).scfia.new_bv_slice(&instruction2.clone(), 9, 9, None, &mut (*context).fork_sink, None);
            let mut wback: ActiveValue<STM32ScfiaComposition> = (*context).scfia.new_bv_slice(&instruction2.clone(), 8, 8, None, &mut (*context).fork_sink, None);
            let mut offset_addr: ActiveValue<STM32ScfiaComposition> = (*context).scfia.new_bv_concrete(0b0, 32);
            if (*context).scfia.check_condition(&(*context).scfia.new_bool_eq(&add.clone(), &(*context).scfia.new_bv_concrete(0b1, 1), None, false, &mut (*context).fork_sink, None), &mut (*context).fork_sink) {
                offset_addr = (*context).scfia.new_bv_add(&n.clone(), &imm32.clone(), 32, None, &mut (*context).fork_sink, None);
            }
            else {
                offset_addr = (*context).scfia.new_bv_sub(&n.clone(), &imm32.clone(), 32, None, &mut (*context).fork_sink, None);
            }
            let mut address: ActiveValue<STM32ScfiaComposition> = (*context).scfia.new_bv_concrete(0b0, 32);
            if (*context).scfia.check_condition(&(*context).scfia.new_bool_eq(&index.clone(), &(*context).scfia.new_bv_concrete(0b1, 1), None, false, &mut (*context).fork_sink, None), &mut (*context).fork_sink) {
                address = offset_addr.clone();
            }
            else {
                address = n.clone();
            }
            (*(*context).memory).write(&address.clone(), &_register_read_BV32_wide(rt.clone(), state, context), 32, (*context).scfia.clone(), &mut (*context).hints, &mut (*context).fork_sink);
            if (*context).scfia.check_condition(&(*context).scfia.new_bool_eq(&wback.clone(), &(*context).scfia.new_bv_concrete(0b1, 1), None, false, &mut (*context).fork_sink, None), &mut (*context).fork_sink) {
                _register_write_BV32_wide(rn.clone(), offset_addr.clone(), state, context);
            }
        }
        else {
            unimplemented!();
        }
    }
    else if (*context).scfia.check_condition(&(*context).scfia.new_bool_eq(&op1.clone(), &(*context).scfia.new_bv_concrete(0b100, 3), None, false, &mut (*context).fork_sink, None), &mut (*context).fork_sink) {
        unimplemented!();
    }
    else if (*context).scfia.check_condition(&(*context).scfia.new_bool_eq(&op1.clone(), &(*context).scfia.new_bv_concrete(0b101, 3), None, false, &mut (*context).fork_sink, None), &mut (*context).fork_sink) {
        unimplemented!();
    }
    else if (*context).scfia.check_condition(&(*context).scfia.new_bool_eq(&op1.clone(), &(*context).scfia.new_bv_concrete(0b110, 3), None, false, &mut (*context).fork_sink, None), &mut (*context).fork_sink) {
        unimplemented!();
    }
    else {
        unimplemented!();
    }
}

unsafe fn _thumb32_load_store_dual_or_exclusive_table_branch(instruction1: ActiveValue<STM32ScfiaComposition>, instruction2: ActiveValue<STM32ScfiaComposition>, state: *mut SystemState, context: *mut StepContext<STM32ScfiaComposition>) {
    let mut op1: ActiveValue<STM32ScfiaComposition> = (*context).scfia.new_bv_slice(&instruction1.clone(), 8, 7, None, &mut (*context).fork_sink, None);
    let mut op2: ActiveValue<STM32ScfiaComposition> = (*context).scfia.new_bv_slice(&instruction1.clone(), 5, 4, None, &mut (*context).fork_sink, None);
    let mut rn: ActiveValue<STM32ScfiaComposition> = (*context).scfia.new_bv_slice(&instruction1.clone(), 3, 0, None, &mut (*context).fork_sink, None);
    let mut op3: ActiveValue<STM32ScfiaComposition> = (*context).scfia.new_bv_slice(&instruction2.clone(), 7, 4, None, &mut (*context).fork_sink, None);
    unimplemented!();
}

unsafe fn _thumb16(instruction: ActiveValue<STM32ScfiaComposition>, state: *mut SystemState, context: *mut StepContext<STM32ScfiaComposition>) {
    let mut opcode: ActiveValue<STM32ScfiaComposition> = (*context).scfia.new_bv_slice(&instruction.clone(), 15, 10, None, &mut (*context).fork_sink, None);
    if (*context).scfia.check_condition(&(*context).scfia.new_bool_unsigned_less_than(&opcode.clone(), &(*context).scfia.new_bv_concrete(0b10000, 6), None, false, &mut (*context).fork_sink, None), &mut (*context).fork_sink) {
        _thumb16_basic(instruction.clone(), state, context);
    }
    else if (*context).scfia.check_condition(&(*context).scfia.new_bool_eq(&opcode.clone(), &(*context).scfia.new_bv_concrete(0b10000, 6), None, false, &mut (*context).fork_sink, None), &mut (*context).fork_sink) {
        _thumb16_data_processing(instruction.clone(), state, context);
    }
    else if (*context).scfia.check_condition(&(*context).scfia.new_bool_eq(&opcode.clone(), &(*context).scfia.new_bv_concrete(0b10001, 6), None, false, &mut (*context).fork_sink, None), &mut (*context).fork_sink) {
        _thumb16_special_data_instructions_and_branch_and_merge(instruction.clone(), state, context);
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
        _thumb16_misc_16_bit_instruction(instruction.clone(), state, context);
    }
    else if (*context).scfia.check_condition(&(*context).scfia.new_bool_unsigned_less_than(&opcode.clone(), &(*context).scfia.new_bv_concrete(0b110010, 6), None, false, &mut (*context).fork_sink, None), &mut (*context).fork_sink) {
        unimplemented!();
    }
    else if (*context).scfia.check_condition(&(*context).scfia.new_bool_unsigned_less_than(&opcode.clone(), &(*context).scfia.new_bv_concrete(0b110100, 6), None, false, &mut (*context).fork_sink, None), &mut (*context).fork_sink) {
        unimplemented!();
    }
    else if (*context).scfia.check_condition(&(*context).scfia.new_bool_unsigned_less_than(&opcode.clone(), &(*context).scfia.new_bv_concrete(0b111000, 6), None, false, &mut (*context).fork_sink, None), &mut (*context).fork_sink) {
        _thumb16_conditional_branch_and_svc(instruction.clone(), state, context);
    }
    else if (*context).scfia.check_condition(&(*context).scfia.new_bool_unsigned_less_than(&opcode.clone(), &(*context).scfia.new_bv_concrete(0b111010, 6), None, false, &mut (*context).fork_sink, None), &mut (*context).fork_sink) {
        _thumb16_unconditional_branch(instruction.clone(), state, context);
    }
    else {
        unimplemented!();
    }
}

unsafe fn _thumb16_basic(instruction: ActiveValue<STM32ScfiaComposition>, state: *mut SystemState, context: *mut StepContext<STM32ScfiaComposition>) {
    let mut opcode: ActiveValue<STM32ScfiaComposition> = (*context).scfia.new_bv_slice(&instruction.clone(), 13, 9, None, &mut (*context).fork_sink, None);
    if (*context).scfia.check_condition(&(*context).scfia.new_bool_unsigned_less_than(&opcode.clone(), &(*context).scfia.new_bv_concrete(0b100, 5), None, false, &mut (*context).fork_sink, None), &mut (*context).fork_sink) {
        unimplemented!();
    }
    else if (*context).scfia.check_condition(&(*context).scfia.new_bool_unsigned_less_than(&opcode.clone(), &(*context).scfia.new_bv_concrete(0b1000, 5), None, false, &mut (*context).fork_sink, None), &mut (*context).fork_sink) {
        unimplemented!();
    }
    else if (*context).scfia.check_condition(&(*context).scfia.new_bool_unsigned_less_than(&opcode.clone(), &(*context).scfia.new_bv_concrete(0b1100, 5), None, false, &mut (*context).fork_sink, None), &mut (*context).fork_sink) {
        unimplemented!();
    }
    else if (*context).scfia.check_condition(&(*context).scfia.new_bool_eq(&opcode.clone(), &(*context).scfia.new_bv_concrete(0b1100, 5), None, false, &mut (*context).fork_sink, None), &mut (*context).fork_sink) {
        let mut rd: ActiveValue<STM32ScfiaComposition> = (*context).scfia.new_bv_slice(&instruction.clone(), 2, 0, None, &mut (*context).fork_sink, None);
        let mut rn: ActiveValue<STM32ScfiaComposition> = (*context).scfia.new_bv_slice(&instruction.clone(), 5, 3, None, &mut (*context).fork_sink, None);
        let mut rm: ActiveValue<STM32ScfiaComposition> = (*context).scfia.new_bv_slice(&instruction.clone(), 8, 6, None, &mut (*context).fork_sink, None);
        let mut setflags: ActiveValue<STM32ScfiaComposition> = (*context).scfia.new_bv_concrete(0b1, 1);
        let mut result: ActiveValue<STM32ScfiaComposition> = (*context).scfia.new_bv_add(&_register_read_BV32(rn.clone(), state, context), &_register_read_BV32(rm.clone(), state, context), 32, None, &mut (*context).fork_sink, None);
        let mut carry: ActiveValue<STM32ScfiaComposition> = (*context).scfia.new_bv_concrete(0b0, 1);
        let mut overflow: ActiveValue<STM32ScfiaComposition> = (*context).scfia.new_bv_concrete(0b0, 1);
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
    else if (*context).scfia.check_condition(&(*context).scfia.new_bool_unsigned_less_than(&opcode.clone(), &(*context).scfia.new_bv_concrete(0b10100, 5), None, false, &mut (*context).fork_sink, None), &mut (*context).fork_sink) {
        let mut rd: ActiveValue<STM32ScfiaComposition> = (*context).scfia.new_bv_slice(&instruction.clone(), 10, 8, None, &mut (*context).fork_sink, None);
        let mut imm8: ActiveValue<STM32ScfiaComposition> = (*context).scfia.new_bv_slice(&instruction.clone(), 7, 0, None, &mut (*context).fork_sink, None);
        let mut imm9: ActiveValue<STM32ScfiaComposition> = (*context).scfia.new_bv_concat(&imm8.clone(), &(*context).scfia.new_bv_concrete(0b0, 1), 9, None, &mut (*context).fork_sink, None);
        let mut imm32: ActiveValue<STM32ScfiaComposition> = (*context).scfia.new_bv_concat(&(*context).scfia.new_bv_concrete(0b0, 23), &imm9.clone(), 32, None, &mut (*context).fork_sink, None);
        let mut setflags: ActiveValue<STM32ScfiaComposition> = (*context).scfia.new_bv_concrete(0b1, 1);
        let mut carry: ActiveValue<STM32ScfiaComposition> = (*state).apsr.C.clone();
        _mov_immediate(rd.clone(), setflags.clone(), imm32.clone(), carry.clone(), state, context);
    }
    else if (*context).scfia.check_condition(&(*context).scfia.new_bool_unsigned_less_than(&opcode.clone(), &(*context).scfia.new_bv_concrete(0b11000, 5), None, false, &mut (*context).fork_sink, None), &mut (*context).fork_sink) {
        unimplemented!();
    }
    else if (*context).scfia.check_condition(&(*context).scfia.new_bool_unsigned_less_than(&opcode.clone(), &(*context).scfia.new_bv_concrete(0b11100, 5), None, false, &mut (*context).fork_sink, None), &mut (*context).fork_sink) {
        let mut rdn: ActiveValue<STM32ScfiaComposition> = (*context).scfia.new_bv_slice(&instruction.clone(), 10, 8, None, &mut (*context).fork_sink, None);
        let mut setflags: ActiveValue<STM32ScfiaComposition> = (*context).scfia.new_bv_concrete(0b1, 1);
        let mut imm8: ActiveValue<STM32ScfiaComposition> = (*context).scfia.new_bv_slice(&instruction.clone(), 7, 0, None, &mut (*context).fork_sink, None);
        let mut imm32: ActiveValue<STM32ScfiaComposition> = (*context).scfia.new_bv_concat(&(*context).scfia.new_bv_concrete(0b0, 24), &imm8.clone(), 32, None, &mut (*context).fork_sink, None);
        let mut add_result: ActiveValue<STM32ScfiaComposition> = _add_with_carry_BV32(_register_read_BV32(rdn.clone(), state, context), imm32.clone(), (*context).scfia.new_bv_concrete(0b0, 1), context);
        let mut result: ActiveValue<STM32ScfiaComposition> = (*context).scfia.new_bv_slice(&add_result.clone(), 33, 2, None, &mut (*context).fork_sink, None);
        let mut carry: ActiveValue<STM32ScfiaComposition> = (*context).scfia.new_bv_slice(&add_result.clone(), 1, 1, None, &mut (*context).fork_sink, None);
        let mut overflow: ActiveValue<STM32ScfiaComposition> = (*context).scfia.new_bv_slice(&add_result.clone(), 0, 0, None, &mut (*context).fork_sink, None);
        _register_write_BV32(rdn.clone(), result.clone(), state, context);
        if (*context).scfia.check_condition(&(*context).scfia.new_bool_eq(&setflags.clone(), &(*context).scfia.new_bv_concrete(0b1, 1), None, false, &mut (*context).fork_sink, None), &mut (*context).fork_sink) {
            (*state).apsr.N = (*context).scfia.new_bv_slice(&result.clone(), 31, 31, None, &mut (*context).fork_sink, None);
            (*state).apsr.Z = _is_zero_bit_BV32(result.clone(), context);
            (*state).apsr.C = carry.clone();
            (*state).apsr.V = overflow.clone();
        }
    }
    else {
        unimplemented!();
    }
}

unsafe fn _thumb16_data_processing(instruction: ActiveValue<STM32ScfiaComposition>, state: *mut SystemState, context: *mut StepContext<STM32ScfiaComposition>) {
    let mut opcode: ActiveValue<STM32ScfiaComposition> = (*context).scfia.new_bv_slice(&instruction.clone(), 9, 6, None, &mut (*context).fork_sink, None);
    if (*context).scfia.check_condition(&(*context).scfia.new_bool_eq(&opcode.clone(), &(*context).scfia.new_bv_concrete(0b0, 4), None, false, &mut (*context).fork_sink, None), &mut (*context).fork_sink) {
        let mut rdn: ActiveValue<STM32ScfiaComposition> = (*context).scfia.new_bv_slice(&instruction.clone(), 2, 0, None, &mut (*context).fork_sink, None);
        let mut rm: ActiveValue<STM32ScfiaComposition> = (*context).scfia.new_bv_slice(&instruction.clone(), 5, 3, None, &mut (*context).fork_sink, None);
        let mut setflags: ActiveValue<STM32ScfiaComposition> = (*context).scfia.new_bv_concrete(0b1, 1);
        let mut result: ActiveValue<STM32ScfiaComposition> = (*context).scfia.new_bv_and(&_register_read_BV32(rdn.clone(), state, context), &_register_read_BV32(rm.clone(), state, context), 32, None, &mut (*context).fork_sink, None);
        _register_write_BV32(rdn.clone(), result.clone(), state, context);
        if (*context).scfia.check_condition(&(*context).scfia.new_bool_eq(&setflags.clone(), &(*context).scfia.new_bv_concrete(0b1, 1), None, false, &mut (*context).fork_sink, None), &mut (*context).fork_sink) {
            (*state).apsr.N = (*context).scfia.new_bv_slice(&result.clone(), 31, 31, None, &mut (*context).fork_sink, None);
            (*state).apsr.Z = _is_zero_bit_BV32(result.clone(), context);
        }
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
        let mut rn: ActiveValue<STM32ScfiaComposition> = (*context).scfia.new_bv_slice(&instruction.clone(), 2, 0, None, &mut (*context).fork_sink, None);
        let mut rm: ActiveValue<STM32ScfiaComposition> = (*context).scfia.new_bv_slice(&instruction.clone(), 5, 3, None, &mut (*context).fork_sink, None);
        let mut add_result: ActiveValue<STM32ScfiaComposition> = _add_with_carry_BV32(_register_read_BV32(rn.clone(), state, context), (*context).scfia.new_bv_not(&_register_read_BV32(rm.clone(), state, context), 32, None, &mut (*context).fork_sink, None), (*context).scfia.new_bv_concrete(0b1, 1), context);
        let mut result: ActiveValue<STM32ScfiaComposition> = (*context).scfia.new_bv_slice(&add_result.clone(), 33, 2, None, &mut (*context).fork_sink, None);
        let mut carry: ActiveValue<STM32ScfiaComposition> = (*context).scfia.new_bv_slice(&add_result.clone(), 1, 1, None, &mut (*context).fork_sink, None);
        let mut overflow: ActiveValue<STM32ScfiaComposition> = (*context).scfia.new_bv_slice(&add_result.clone(), 0, 0, None, &mut (*context).fork_sink, None);
        (*state).apsr.N = (*context).scfia.new_bv_slice(&result.clone(), 31, 31, None, &mut (*context).fork_sink, None);
        (*state).apsr.Z = _is_zero_bit_BV32(result.clone(), context);
        (*state).apsr.C = carry.clone();
        (*state).apsr.V = overflow.clone();
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

unsafe fn _thumb16_special_data_instructions_and_branch_and_merge(instruction: ActiveValue<STM32ScfiaComposition>, state: *mut SystemState, context: *mut StepContext<STM32ScfiaComposition>) {
    let mut opcode: ActiveValue<STM32ScfiaComposition> = (*context).scfia.new_bv_slice(&instruction.clone(), 9, 6, None, &mut (*context).fork_sink, None);
    if (*context).scfia.check_condition(&(*context).scfia.new_bool_eq(&_matches_BV4(opcode.clone(), (*context).scfia.new_bv_concrete(0b0, 4), (*context).scfia.new_bv_concrete(0b1100, 4), context), &(*context).scfia.new_bv_concrete(0b1, 1), None, false, &mut (*context).fork_sink, None), &mut (*context).fork_sink) {
        unimplemented!();
    }
    else if (*context).scfia.check_condition(&(*context).scfia.new_bool_eq(&opcode.clone(), &(*context).scfia.new_bv_concrete(0b100, 4), None, false, &mut (*context).fork_sink, None), &mut (*context).fork_sink) {
        unimplemented!();
    }
    else if (*context).scfia.check_condition(&(*context).scfia.new_bool_unsigned_less_than(&opcode.clone(), &(*context).scfia.new_bv_concrete(0b1000, 4), None, false, &mut (*context).fork_sink, None), &mut (*context).fork_sink) {
        unimplemented!();
    }
    else if (*context).scfia.check_condition(&(*context).scfia.new_bool_eq(&_matches_BV4(opcode.clone(), (*context).scfia.new_bv_concrete(0b1000, 4), (*context).scfia.new_bv_concrete(0b100, 4), context), &(*context).scfia.new_bv_concrete(0b1, 1), None, false, &mut (*context).fork_sink, None), &mut (*context).fork_sink) {
        unimplemented!();
    }
    else if (*context).scfia.check_condition(&(*context).scfia.new_bool_eq(&_matches_BV4(opcode.clone(), (*context).scfia.new_bv_concrete(0b1100, 4), (*context).scfia.new_bv_concrete(0b10, 4), context), &(*context).scfia.new_bv_concrete(0b1, 1), None, false, &mut (*context).fork_sink, None), &mut (*context).fork_sink) {
        let mut rm: ActiveValue<STM32ScfiaComposition> = (*context).scfia.new_bv_slice(&instruction.clone(), 6, 3, None, &mut (*context).fork_sink, None);
        let mut m: ActiveValue<STM32ScfiaComposition> = _register_read_BV32_wide(rm.clone(), state, context);
        _bx_write_pc(m.clone(), state, context);
    }
    else {
        unimplemented!();
    }
}

unsafe fn _thumb16_load_from_literal_pool(instruction: ActiveValue<STM32ScfiaComposition>, state: *mut SystemState, context: *mut StepContext<STM32ScfiaComposition>) {
    let mut bit_11: ActiveValue<STM32ScfiaComposition> = (*context).scfia.new_bv_slice(&instruction.clone(), 11, 11, None, &mut (*context).fork_sink, None);
    if (*context).scfia.check_condition(&(*context).scfia.new_bool_eq(&bit_11.clone(), &(*context).scfia.new_bv_concrete(0b1, 1), None, false, &mut (*context).fork_sink, None), &mut (*context).fork_sink) {
        let mut rt: ActiveValue<STM32ScfiaComposition> = (*context).scfia.new_bv_slice(&instruction.clone(), 10, 8, None, &mut (*context).fork_sink, None);
        let mut imm32: ActiveValue<STM32ScfiaComposition> = (*context).scfia.new_bv_concat(&(*context).scfia.new_bv_concrete(0b0, 22), &(*context).scfia.new_bv_concat(&(*context).scfia.new_bv_slice(&instruction.clone(), 7, 0, None, &mut (*context).fork_sink, None), &(*context).scfia.new_bv_concrete(0b0, 2), 10, None, &mut (*context).fork_sink, None), 32, None, &mut (*context).fork_sink, None);
        let mut add: ActiveValue<STM32ScfiaComposition> = (*context).scfia.new_bv_concrete(0b1, 1);
        _load_register(rt.clone(), imm32.clone(), add.clone(), state, context);
    }
    else {
        unimplemented!();
    }
}

unsafe fn _thumb16_load_store_single_data_item(instruction: ActiveValue<STM32ScfiaComposition>, state: *mut SystemState, context: *mut StepContext<STM32ScfiaComposition>) {
    let mut op_a: ActiveValue<STM32ScfiaComposition> = (*context).scfia.new_bv_slice(&instruction.clone(), 15, 12, None, &mut (*context).fork_sink, None);
    let mut op_b: ActiveValue<STM32ScfiaComposition> = (*context).scfia.new_bv_slice(&instruction.clone(), 11, 9, None, &mut (*context).fork_sink, None);
    if (*context).scfia.check_condition(&(*context).scfia.new_bool_eq(&op_a.clone(), &(*context).scfia.new_bv_concrete(0b101, 4), None, false, &mut (*context).fork_sink, None), &mut (*context).fork_sink) {
        if (*context).scfia.check_condition(&(*context).scfia.new_bool_eq(&op_b.clone(), &(*context).scfia.new_bv_concrete(0b0, 3), None, false, &mut (*context).fork_sink, None), &mut (*context).fork_sink) {
            let mut rt: ActiveValue<STM32ScfiaComposition> = (*context).scfia.new_bv_slice(&instruction.clone(), 2, 0, None, &mut (*context).fork_sink, None);
            let mut rn: ActiveValue<STM32ScfiaComposition> = (*context).scfia.new_bv_slice(&instruction.clone(), 5, 3, None, &mut (*context).fork_sink, None);
            let mut rm: ActiveValue<STM32ScfiaComposition> = (*context).scfia.new_bv_slice(&instruction.clone(), 8, 6, None, &mut (*context).fork_sink, None);
            let mut offset: ActiveValue<STM32ScfiaComposition> = _register_read_BV32(rm.clone(), state, context);
            let mut base_address: ActiveValue<STM32ScfiaComposition> = _register_read_BV32(rn.clone(), state, context);
            let mut address: ActiveValue<STM32ScfiaComposition> = (*context).scfia.new_bv_add(&base_address.clone(), &offset.clone(), 32, None, &mut (*context).fork_sink, None);
            (*(*context).memory).write(&address.clone(), &_register_read_BV32(rt.clone(), state, context), 32, (*context).scfia.clone(), &mut (*context).hints, &mut (*context).fork_sink);
        }
        else if (*context).scfia.check_condition(&(*context).scfia.new_bool_eq(&op_b.clone(), &(*context).scfia.new_bv_concrete(0b1, 3), None, false, &mut (*context).fork_sink, None), &mut (*context).fork_sink) {
            unimplemented!();
        }
        else if (*context).scfia.check_condition(&(*context).scfia.new_bool_eq(&op_b.clone(), &(*context).scfia.new_bv_concrete(0b10, 3), None, false, &mut (*context).fork_sink, None), &mut (*context).fork_sink) {
            unimplemented!();
        }
        else if (*context).scfia.check_condition(&(*context).scfia.new_bool_eq(&op_b.clone(), &(*context).scfia.new_bv_concrete(0b11, 3), None, false, &mut (*context).fork_sink, None), &mut (*context).fork_sink) {
            unimplemented!();
        }
        else if (*context).scfia.check_condition(&(*context).scfia.new_bool_eq(&op_b.clone(), &(*context).scfia.new_bv_concrete(0b100, 3), None, false, &mut (*context).fork_sink, None), &mut (*context).fork_sink) {
            let mut rt: ActiveValue<STM32ScfiaComposition> = (*context).scfia.new_bv_slice(&instruction.clone(), 2, 0, None, &mut (*context).fork_sink, None);
            let mut rn: ActiveValue<STM32ScfiaComposition> = (*context).scfia.new_bv_slice(&instruction.clone(), 5, 3, None, &mut (*context).fork_sink, None);
            let mut rm: ActiveValue<STM32ScfiaComposition> = (*context).scfia.new_bv_slice(&instruction.clone(), 8, 6, None, &mut (*context).fork_sink, None);
            let mut offset: ActiveValue<STM32ScfiaComposition> = _register_read_BV32(rm.clone(), state, context);
            let mut address: ActiveValue<STM32ScfiaComposition> = (*context).scfia.new_bv_add(&offset.clone(), &_register_read_BV32(rn.clone(), state, context), 32, None, &mut (*context).fork_sink, None);
            let mut value: ActiveValue<STM32ScfiaComposition> = (*(*context).memory).read(&address.clone(), 32, (*context).scfia.clone(), &mut (*context).hints, &mut (*context).fork_sink);
            _register_write_BV32(rt.clone(), value.clone(), state, context);
        }
        else if (*context).scfia.check_condition(&(*context).scfia.new_bool_eq(&op_b.clone(), &(*context).scfia.new_bv_concrete(0b101, 3), None, false, &mut (*context).fork_sink, None), &mut (*context).fork_sink) {
            unimplemented!();
        }
        else if (*context).scfia.check_condition(&(*context).scfia.new_bool_eq(&op_b.clone(), &(*context).scfia.new_bv_concrete(0b110, 3), None, false, &mut (*context).fork_sink, None), &mut (*context).fork_sink) {
            unimplemented!();
        }
        else if (*context).scfia.check_condition(&(*context).scfia.new_bool_eq(&op_b.clone(), &(*context).scfia.new_bv_concrete(0b111, 3), None, false, &mut (*context).fork_sink, None), &mut (*context).fork_sink) {
            unimplemented!();
        }
    }
    else if (*context).scfia.check_condition(&(*context).scfia.new_bool_eq(&op_a.clone(), &(*context).scfia.new_bv_concrete(0b110, 4), None, false, &mut (*context).fork_sink, None), &mut (*context).fork_sink) {
        if (*context).scfia.check_condition(&(*context).scfia.new_bool_eq(&(*context).scfia.new_bv_slice(&op_b.clone(), 2, 2, None, &mut (*context).fork_sink, None), &(*context).scfia.new_bv_concrete(0b0, 1), None, false, &mut (*context).fork_sink, None), &mut (*context).fork_sink) {
            let mut imm5: ActiveValue<STM32ScfiaComposition> = (*context).scfia.new_bv_slice(&instruction.clone(), 10, 6, None, &mut (*context).fork_sink, None);
            let mut rn: ActiveValue<STM32ScfiaComposition> = (*context).scfia.new_bv_slice(&instruction.clone(), 5, 3, None, &mut (*context).fork_sink, None);
            let mut rt: ActiveValue<STM32ScfiaComposition> = (*context).scfia.new_bv_slice(&instruction.clone(), 2, 0, None, &mut (*context).fork_sink, None);
            let mut imm7: ActiveValue<STM32ScfiaComposition> = (*context).scfia.new_bv_concat(&imm5.clone(), &(*context).scfia.new_bv_concrete(0b0, 2), 7, None, &mut (*context).fork_sink, None);
            let mut imm32: ActiveValue<STM32ScfiaComposition> = (*context).scfia.new_bv_concat(&(*context).scfia.new_bv_concrete(0b0, 25), &imm7.clone(), 32, None, &mut (*context).fork_sink, None);
            let mut address: ActiveValue<STM32ScfiaComposition> = (*context).scfia.new_bv_add(&_register_read_BV32(rn.clone(), state, context), &imm32.clone(), 32, None, &mut (*context).fork_sink, None);
            (*(*context).memory).write(&address.clone(), &_register_read_BV32(rt.clone(), state, context), 32, (*context).scfia.clone(), &mut (*context).hints, &mut (*context).fork_sink);
        }
        else {
            let mut rt: ActiveValue<STM32ScfiaComposition> = (*context).scfia.new_bv_slice(&instruction.clone(), 2, 0, None, &mut (*context).fork_sink, None);
            let mut rn: ActiveValue<STM32ScfiaComposition> = (*context).scfia.new_bv_slice(&instruction.clone(), 5, 3, None, &mut (*context).fork_sink, None);
            let mut imm5: ActiveValue<STM32ScfiaComposition> = (*context).scfia.new_bv_slice(&instruction.clone(), 10, 6, None, &mut (*context).fork_sink, None);
            let mut imm7: ActiveValue<STM32ScfiaComposition> = (*context).scfia.new_bv_concat(&imm5.clone(), &(*context).scfia.new_bv_concrete(0b0, 2), 7, None, &mut (*context).fork_sink, None);
            let mut imm32: ActiveValue<STM32ScfiaComposition> = (*context).scfia.new_bv_concat(&(*context).scfia.new_bv_concrete(0b0, 25), &imm7.clone(), 32, None, &mut (*context).fork_sink, None);
            let mut address: ActiveValue<STM32ScfiaComposition> = (*context).scfia.new_bv_add(&_register_read_BV32(rn.clone(), state, context), &imm32.clone(), 32, None, &mut (*context).fork_sink, None);
            let mut data: ActiveValue<STM32ScfiaComposition> = (*(*context).memory).read(&address.clone(), 32, (*context).scfia.clone(), &mut (*context).hints, &mut (*context).fork_sink);
            _register_write_BV32(rt.clone(), data.clone(), state, context);
        }
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

unsafe fn _thumb16_misc_16_bit_instruction(instruction: ActiveValue<STM32ScfiaComposition>, state: *mut SystemState, context: *mut StepContext<STM32ScfiaComposition>) {
    let mut opcode: ActiveValue<STM32ScfiaComposition> = (*context).scfia.new_bv_slice(&instruction.clone(), 11, 5, None, &mut (*context).fork_sink, None);
    if (*context).scfia.check_condition(&(*context).scfia.new_bool_eq(&opcode.clone(), &(*context).scfia.new_bv_concrete(0b110011, 7), None, false, &mut (*context).fork_sink, None), &mut (*context).fork_sink) {
        unimplemented!();
    }
    else if (*context).scfia.check_condition(&(*context).scfia.new_bool_eq(&_matches_BV7(opcode.clone(), (*context).scfia.new_bv_concrete(0b0, 7), (*context).scfia.new_bv_concrete(0b1111100, 7), context), &(*context).scfia.new_bv_concrete(0b1, 1), None, false, &mut (*context).fork_sink, None), &mut (*context).fork_sink) {
        unimplemented!();
    }
    else if (*context).scfia.check_condition(&(*context).scfia.new_bool_eq(&_matches_BV7(opcode.clone(), (*context).scfia.new_bv_concrete(0b100, 7), (*context).scfia.new_bv_concrete(0b1111000, 7), context), &(*context).scfia.new_bv_concrete(0b1, 1), None, false, &mut (*context).fork_sink, None), &mut (*context).fork_sink) {
        unimplemented!();
    }
    else if (*context).scfia.check_condition(&(*context).scfia.new_bool_eq(&_matches_BV7(opcode.clone(), (*context).scfia.new_bv_concrete(0b1000, 7), (*context).scfia.new_bv_concrete(0b1110000, 7), context), &(*context).scfia.new_bv_concrete(0b1, 1), None, false, &mut (*context).fork_sink, None), &mut (*context).fork_sink) {
        unimplemented!();
    }
    else if (*context).scfia.check_condition(&(*context).scfia.new_bool_eq(&_matches_BV7(opcode.clone(), (*context).scfia.new_bv_concrete(0b10000, 7), (*context).scfia.new_bv_concrete(0b1101110, 7), context), &(*context).scfia.new_bv_concrete(0b1, 1), None, false, &mut (*context).fork_sink, None), &mut (*context).fork_sink) {
        unimplemented!();
    }
    else if (*context).scfia.check_condition(&(*context).scfia.new_bool_eq(&_matches_BV7(opcode.clone(), (*context).scfia.new_bv_concrete(0b10010, 7), (*context).scfia.new_bv_concrete(0b1101100, 7), context), &(*context).scfia.new_bv_concrete(0b1, 1), None, false, &mut (*context).fork_sink, None), &mut (*context).fork_sink) {
        unimplemented!();
    }
    else if (*context).scfia.check_condition(&(*context).scfia.new_bool_eq(&_matches_BV7(opcode.clone(), (*context).scfia.new_bv_concrete(0b10110, 7), (*context).scfia.new_bv_concrete(0b1101000, 7), context), &(*context).scfia.new_bv_concrete(0b1, 1), None, false, &mut (*context).fork_sink, None), &mut (*context).fork_sink) {
        unimplemented!();
    }
    else if (*context).scfia.check_condition(&(*context).scfia.new_bool_eq(&_matches_BV7(opcode.clone(), (*context).scfia.new_bv_concrete(0b11000, 7), (*context).scfia.new_bv_concrete(0b1100000, 7), context), &(*context).scfia.new_bv_concrete(0b1, 1), None, false, &mut (*context).fork_sink, None), &mut (*context).fork_sink) {
        unimplemented!();
    }
    else if (*context).scfia.check_condition(&(*context).scfia.new_bool_eq(&_matches_BV7(opcode.clone(), (*context).scfia.new_bv_concrete(0b100000, 7), (*context).scfia.new_bv_concrete(0b1010000, 7), context), &(*context).scfia.new_bv_concrete(0b1, 1), None, false, &mut (*context).fork_sink, None), &mut (*context).fork_sink) {
        let mut register_list: ActiveValue<STM32ScfiaComposition> = (*context).scfia.new_bv_slice(&instruction.clone(), 7, 0, None, &mut (*context).fork_sink, None);
        let mut m: ActiveValue<STM32ScfiaComposition> = (*context).scfia.new_bv_slice(&instruction.clone(), 8, 8, None, &mut (*context).fork_sink, None);
        let mut registers_14: ActiveValue<STM32ScfiaComposition> = (*context).scfia.new_bv_concat(&(*context).scfia.new_bv_concrete(0b0, 6), &register_list.clone(), 14, None, &mut (*context).fork_sink, None);
        let mut registers_15: ActiveValue<STM32ScfiaComposition> = (*context).scfia.new_bv_concat(&m.clone(), &registers_14.clone(), 15, None, &mut (*context).fork_sink, None);
        let mut registers: ActiveValue<STM32ScfiaComposition> = (*context).scfia.new_bv_concat(&(*context).scfia.new_bv_concrete(0b0, 1), &registers_15.clone(), 16, None, &mut (*context).fork_sink, None);
        unimplemented!();
    }
    else {
    }
    unimplemented!();
}

unsafe fn _thumb16_conditional_branch_and_svc(instruction: ActiveValue<STM32ScfiaComposition>, state: *mut SystemState, context: *mut StepContext<STM32ScfiaComposition>) {
    let mut opcode: ActiveValue<STM32ScfiaComposition> = (*context).scfia.new_bv_slice(&instruction.clone(), 11, 8, None, &mut (*context).fork_sink, None);
    if (*context).scfia.check_condition(&(*context).scfia.new_bool_eq(&opcode.clone(), &(*context).scfia.new_bv_concrete(0b1110, 4), None, false, &mut (*context).fork_sink, None), &mut (*context).fork_sink) {
        unimplemented!();
    }
    else if (*context).scfia.check_condition(&(*context).scfia.new_bool_eq(&opcode.clone(), &(*context).scfia.new_bv_concrete(0b1111, 4), None, false, &mut (*context).fork_sink, None), &mut (*context).fork_sink) {
        unimplemented!();
    }
    else {
        let mut cond: ActiveValue<STM32ScfiaComposition> = (*context).scfia.new_bv_slice(&instruction.clone(), 11, 8, None, &mut (*context).fork_sink, None);
        if (*context).scfia.check_condition(&(*context).scfia.new_bool_eq(&cond.clone(), &(*context).scfia.new_bv_concrete(0b1110, 4), None, false, &mut (*context).fork_sink, None), &mut (*context).fork_sink) {
            unimplemented!();
        }
        else if (*context).scfia.check_condition(&(*context).scfia.new_bool_eq(&cond.clone(), &(*context).scfia.new_bv_concrete(0b1111, 4), None, false, &mut (*context).fork_sink, None), &mut (*context).fork_sink) {
            unimplemented!();
        }
        let mut imm8: ActiveValue<STM32ScfiaComposition> = (*context).scfia.new_bv_slice(&instruction.clone(), 7, 0, None, &mut (*context).fork_sink, None);
        let mut imm9: ActiveValue<STM32ScfiaComposition> = (*context).scfia.new_bv_concat(&imm8.clone(), &(*context).scfia.new_bv_concrete(0b0, 1), 9, None, &mut (*context).fork_sink, None);
        let mut imm32: ActiveValue<STM32ScfiaComposition> = (*context).scfia.new_bv_sign_extend(&imm9.clone(), 9, 32, None, &mut (*context).fork_sink, None);
        if (*context).scfia.check_condition(&(*context).scfia.new_bool_eq(&_condition_passed(cond.clone(), state, context), &(*context).scfia.new_bv_concrete(0b1, 1), None, false, &mut (*context).fork_sink, None), &mut (*context).fork_sink) {
            _branch_write_pc((*context).scfia.new_bv_add(&(*state).PC.clone(), &imm32.clone(), 32, None, &mut (*context).fork_sink, None), state, context);
        }
    }
}

unsafe fn _thumb16_unconditional_branch(instruction: ActiveValue<STM32ScfiaComposition>, state: *mut SystemState, context: *mut StepContext<STM32ScfiaComposition>) {
    let mut imm11: ActiveValue<STM32ScfiaComposition> = (*context).scfia.new_bv_slice(&instruction.clone(), 10, 0, None, &mut (*context).fork_sink, None);
    let mut imm32: ActiveValue<STM32ScfiaComposition> = (*context).scfia.new_bv_sign_extend(&(*context).scfia.new_bv_concat(&imm11.clone(), &(*context).scfia.new_bv_concrete(0b0, 1), 12, None, &mut (*context).fork_sink, None), 12, 32, None, &mut (*context).fork_sink, None);
    let mut value: ActiveValue<STM32ScfiaComposition> = (*context).scfia.new_bv_add(&(*state).PC.clone(), &imm32.clone(), 32, None, &mut (*context).fork_sink, None);
    _branch_write_pc(value.clone(), state, context);
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
    _register_write_BV32_wide((*context).scfia.new_bv_concat(&(*context).scfia.new_bv_concrete(0b0, 1), &register_id.clone(), 4, None, &mut (*context).fork_sink, None), value.clone(), state, context);
}

unsafe fn _register_write_BV32_wide(register_id: ActiveValue<STM32ScfiaComposition>, value: ActiveValue<STM32ScfiaComposition>, state: *mut SystemState, context: *mut StepContext<STM32ScfiaComposition>) {
    if (*context).scfia.check_condition(&(*context).scfia.new_bool_eq(&register_id.clone(), &(*context).scfia.new_bv_concrete(0b0, 4), None, false, &mut (*context).fork_sink, None), &mut (*context).fork_sink) {
        (*state).R0 = value.clone();
    }
    else if (*context).scfia.check_condition(&(*context).scfia.new_bool_eq(&register_id.clone(), &(*context).scfia.new_bv_concrete(0b1, 4), None, false, &mut (*context).fork_sink, None), &mut (*context).fork_sink) {
        (*state).R1 = value.clone();
    }
    else if (*context).scfia.check_condition(&(*context).scfia.new_bool_eq(&register_id.clone(), &(*context).scfia.new_bv_concrete(0b10, 4), None, false, &mut (*context).fork_sink, None), &mut (*context).fork_sink) {
        (*state).R2 = value.clone();
    }
    else if (*context).scfia.check_condition(&(*context).scfia.new_bool_eq(&register_id.clone(), &(*context).scfia.new_bv_concrete(0b11, 4), None, false, &mut (*context).fork_sink, None), &mut (*context).fork_sink) {
        (*state).R3 = value.clone();
    }
    else if (*context).scfia.check_condition(&(*context).scfia.new_bool_eq(&register_id.clone(), &(*context).scfia.new_bv_concrete(0b100, 4), None, false, &mut (*context).fork_sink, None), &mut (*context).fork_sink) {
        (*state).R4 = value.clone();
    }
    else if (*context).scfia.check_condition(&(*context).scfia.new_bool_eq(&register_id.clone(), &(*context).scfia.new_bv_concrete(0b101, 4), None, false, &mut (*context).fork_sink, None), &mut (*context).fork_sink) {
        (*state).R5 = value.clone();
    }
    else if (*context).scfia.check_condition(&(*context).scfia.new_bool_eq(&register_id.clone(), &(*context).scfia.new_bv_concrete(0b110, 4), None, false, &mut (*context).fork_sink, None), &mut (*context).fork_sink) {
        (*state).R6 = value.clone();
    }
    else if (*context).scfia.check_condition(&(*context).scfia.new_bool_eq(&register_id.clone(), &(*context).scfia.new_bv_concrete(0b111, 4), None, false, &mut (*context).fork_sink, None), &mut (*context).fork_sink) {
        (*state).R7 = value.clone();
    }
    else if (*context).scfia.check_condition(&(*context).scfia.new_bool_eq(&register_id.clone(), &(*context).scfia.new_bv_concrete(0b1000, 4), None, false, &mut (*context).fork_sink, None), &mut (*context).fork_sink) {
        (*state).R8 = value.clone();
    }
    else if (*context).scfia.check_condition(&(*context).scfia.new_bool_eq(&register_id.clone(), &(*context).scfia.new_bv_concrete(0b1001, 4), None, false, &mut (*context).fork_sink, None), &mut (*context).fork_sink) {
        (*state).R9 = value.clone();
    }
    else if (*context).scfia.check_condition(&(*context).scfia.new_bool_eq(&register_id.clone(), &(*context).scfia.new_bv_concrete(0b1010, 4), None, false, &mut (*context).fork_sink, None), &mut (*context).fork_sink) {
        (*state).R10 = value.clone();
    }
    else if (*context).scfia.check_condition(&(*context).scfia.new_bool_eq(&register_id.clone(), &(*context).scfia.new_bv_concrete(0b1011, 4), None, false, &mut (*context).fork_sink, None), &mut (*context).fork_sink) {
        (*state).R11 = value.clone();
    }
    else if (*context).scfia.check_condition(&(*context).scfia.new_bool_eq(&register_id.clone(), &(*context).scfia.new_bv_concrete(0b1100, 4), None, false, &mut (*context).fork_sink, None), &mut (*context).fork_sink) {
        (*state).R12 = value.clone();
    }
    else if (*context).scfia.check_condition(&(*context).scfia.new_bool_eq(&register_id.clone(), &(*context).scfia.new_bv_concrete(0b1101, 4), None, false, &mut (*context).fork_sink, None), &mut (*context).fork_sink) {
        (*state).SP = value.clone();
    }
    else if (*context).scfia.check_condition(&(*context).scfia.new_bool_eq(&register_id.clone(), &(*context).scfia.new_bv_concrete(0b1110, 4), None, false, &mut (*context).fork_sink, None), &mut (*context).fork_sink) {
        (*state).LR = value.clone();
    }
    else {
        (*state).PC = value.clone();
    }
}

unsafe fn _register_read_BV32(register_id: ActiveValue<STM32ScfiaComposition>, state: *mut SystemState, context: *mut StepContext<STM32ScfiaComposition>) -> ActiveValue<STM32ScfiaComposition> {
    return _register_read_BV32_wide((*context).scfia.new_bv_concat(&(*context).scfia.new_bv_concrete(0b0, 1), &register_id.clone(), 4, None, &mut (*context).fork_sink, None), state, context);
}

unsafe fn _register_read_BV32_wide(register_id: ActiveValue<STM32ScfiaComposition>, state: *mut SystemState, context: *mut StepContext<STM32ScfiaComposition>) -> ActiveValue<STM32ScfiaComposition> {
    if (*context).scfia.check_condition(&(*context).scfia.new_bool_eq(&register_id.clone(), &(*context).scfia.new_bv_concrete(0b0, 4), None, false, &mut (*context).fork_sink, None), &mut (*context).fork_sink) {
        return (*state).R0.clone();
    }
    else if (*context).scfia.check_condition(&(*context).scfia.new_bool_eq(&register_id.clone(), &(*context).scfia.new_bv_concrete(0b1, 4), None, false, &mut (*context).fork_sink, None), &mut (*context).fork_sink) {
        return (*state).R1.clone();
    }
    else if (*context).scfia.check_condition(&(*context).scfia.new_bool_eq(&register_id.clone(), &(*context).scfia.new_bv_concrete(0b10, 4), None, false, &mut (*context).fork_sink, None), &mut (*context).fork_sink) {
        return (*state).R2.clone();
    }
    else if (*context).scfia.check_condition(&(*context).scfia.new_bool_eq(&register_id.clone(), &(*context).scfia.new_bv_concrete(0b11, 4), None, false, &mut (*context).fork_sink, None), &mut (*context).fork_sink) {
        return (*state).R3.clone();
    }
    else if (*context).scfia.check_condition(&(*context).scfia.new_bool_eq(&register_id.clone(), &(*context).scfia.new_bv_concrete(0b100, 4), None, false, &mut (*context).fork_sink, None), &mut (*context).fork_sink) {
        return (*state).R4.clone();
    }
    else if (*context).scfia.check_condition(&(*context).scfia.new_bool_eq(&register_id.clone(), &(*context).scfia.new_bv_concrete(0b101, 4), None, false, &mut (*context).fork_sink, None), &mut (*context).fork_sink) {
        return (*state).R5.clone();
    }
    else if (*context).scfia.check_condition(&(*context).scfia.new_bool_eq(&register_id.clone(), &(*context).scfia.new_bv_concrete(0b110, 4), None, false, &mut (*context).fork_sink, None), &mut (*context).fork_sink) {
        return (*state).R6.clone();
    }
    else if (*context).scfia.check_condition(&(*context).scfia.new_bool_eq(&register_id.clone(), &(*context).scfia.new_bv_concrete(0b111, 4), None, false, &mut (*context).fork_sink, None), &mut (*context).fork_sink) {
        return (*state).R7.clone();
    }
    else if (*context).scfia.check_condition(&(*context).scfia.new_bool_eq(&register_id.clone(), &(*context).scfia.new_bv_concrete(0b1000, 4), None, false, &mut (*context).fork_sink, None), &mut (*context).fork_sink) {
        return (*state).R8.clone();
    }
    else if (*context).scfia.check_condition(&(*context).scfia.new_bool_eq(&register_id.clone(), &(*context).scfia.new_bv_concrete(0b1001, 4), None, false, &mut (*context).fork_sink, None), &mut (*context).fork_sink) {
        return (*state).R9.clone();
    }
    else if (*context).scfia.check_condition(&(*context).scfia.new_bool_eq(&register_id.clone(), &(*context).scfia.new_bv_concrete(0b1010, 4), None, false, &mut (*context).fork_sink, None), &mut (*context).fork_sink) {
        return (*state).R10.clone();
    }
    else if (*context).scfia.check_condition(&(*context).scfia.new_bool_eq(&register_id.clone(), &(*context).scfia.new_bv_concrete(0b1011, 4), None, false, &mut (*context).fork_sink, None), &mut (*context).fork_sink) {
        return (*state).R11.clone();
    }
    else if (*context).scfia.check_condition(&(*context).scfia.new_bool_eq(&register_id.clone(), &(*context).scfia.new_bv_concrete(0b1100, 4), None, false, &mut (*context).fork_sink, None), &mut (*context).fork_sink) {
        return (*state).R12.clone();
    }
    else if (*context).scfia.check_condition(&(*context).scfia.new_bool_eq(&register_id.clone(), &(*context).scfia.new_bv_concrete(0b1101, 4), None, false, &mut (*context).fork_sink, None), &mut (*context).fork_sink) {
        return (*state).SP.clone();
    }
    else if (*context).scfia.check_condition(&(*context).scfia.new_bool_eq(&register_id.clone(), &(*context).scfia.new_bv_concrete(0b1110, 4), None, false, &mut (*context).fork_sink, None), &mut (*context).fork_sink) {
        return (*state).LR.clone();
    }
    else {
        return (*state).PC.clone();
    }
}

unsafe fn _progress_pc_2(state: *mut SystemState, context: *mut StepContext<STM32ScfiaComposition>) {
    let mut old_pc: ActiveValue<STM32ScfiaComposition> = (*state).PC.clone();
    let mut new_pc: ActiveValue<STM32ScfiaComposition> = (*context).scfia.new_bv_add(&old_pc.clone(), &(*context).scfia.new_bv_concrete(0b10, 32), 32, None, &mut (*context).fork_sink, None);
    (*state).PC = new_pc.clone();
}

unsafe fn _progress_pc_4(state: *mut SystemState, context: *mut StepContext<STM32ScfiaComposition>) {
    let mut old_pc: ActiveValue<STM32ScfiaComposition> = (*state).PC.clone();
    let mut new_pc: ActiveValue<STM32ScfiaComposition> = (*context).scfia.new_bv_add(&old_pc.clone(), &(*context).scfia.new_bv_concrete(0b100, 32), 32, None, &mut (*context).fork_sink, None);
    (*state).PC = new_pc.clone();
}

unsafe fn _add_with_carry_BV32(x: ActiveValue<STM32ScfiaComposition>, y: ActiveValue<STM32ScfiaComposition>, carry_in: ActiveValue<STM32ScfiaComposition>, context: *mut StepContext<STM32ScfiaComposition>) -> ActiveValue<STM32ScfiaComposition> {
    let mut unsigned_sum: ActiveValue<STM32ScfiaComposition> = (*context).scfia.new_bv_add(&(*context).scfia.new_bv_add(&(*context).scfia.new_bv_concat(&(*context).scfia.new_bv_concrete(0b0, 2), &x.clone(), 34, None, &mut (*context).fork_sink, None), &(*context).scfia.new_bv_concat(&(*context).scfia.new_bv_concrete(0b0, 2), &y.clone(), 34, None, &mut (*context).fork_sink, None), 34, None, &mut (*context).fork_sink, None), &(*context).scfia.new_bv_concat(&(*context).scfia.new_bv_concrete(0b0, 33), &carry_in.clone(), 34, None, &mut (*context).fork_sink, None), 34, None, &mut (*context).fork_sink, None);
    let mut signed_sum: ActiveValue<STM32ScfiaComposition> = (*context).scfia.new_bv_add(&(*context).scfia.new_bv_add(&(*context).scfia.new_bv_sign_extend(&x.clone(), 32, 34, None, &mut (*context).fork_sink, None), &(*context).scfia.new_bv_sign_extend(&y.clone(), 32, 34, None, &mut (*context).fork_sink, None), 34, None, &mut (*context).fork_sink, None), &(*context).scfia.new_bv_concat(&(*context).scfia.new_bv_concrete(0b0, 33), &carry_in.clone(), 34, None, &mut (*context).fork_sink, None), 34, None, &mut (*context).fork_sink, None);
    let mut result: ActiveValue<STM32ScfiaComposition> = (*context).scfia.new_bv_slice(&unsigned_sum.clone(), 31, 0, None, &mut (*context).fork_sink, None);
    let mut carry_out: ActiveValue<STM32ScfiaComposition> = _is_not_eq_BV34((*context).scfia.new_bv_concat(&(*context).scfia.new_bv_concrete(0b0, 2), &result.clone(), 34, None, &mut (*context).fork_sink, None), unsigned_sum.clone(), context);
    let mut overflow: ActiveValue<STM32ScfiaComposition> = _is_not_eq_BV34((*context).scfia.new_bv_sign_extend(&result.clone(), 32, 34, None, &mut (*context).fork_sink, None), unsigned_sum.clone(), context);
    return (*context).scfia.new_bv_concat(&result.clone(), &(*context).scfia.new_bv_concat(&carry_out.clone(), &overflow.clone(), 2, None, &mut (*context).fork_sink, None), 34, None, &mut (*context).fork_sink, None);
}

unsafe fn _thumb_expand_imm_c(imm12: ActiveValue<STM32ScfiaComposition>, carry_in: ActiveValue<STM32ScfiaComposition>, context: *mut StepContext<STM32ScfiaComposition>) -> ActiveValue<STM32ScfiaComposition> {
    let mut imm32: ActiveValue<STM32ScfiaComposition> = (*context).scfia.new_bv_concrete(0b0, 32);
    let mut carry_out: ActiveValue<STM32ScfiaComposition> = (*context).scfia.new_bv_concrete(0b0, 1);
    if (*context).scfia.check_condition(&(*context).scfia.new_bool_eq(&(*context).scfia.new_bv_slice(&imm12.clone(), 11, 10, None, &mut (*context).fork_sink, None), &(*context).scfia.new_bv_concrete(0b0, 2), None, false, &mut (*context).fork_sink, None), &mut (*context).fork_sink) {
        let mut mode: ActiveValue<STM32ScfiaComposition> = (*context).scfia.new_bv_slice(&imm12.clone(), 9, 8, None, &mut (*context).fork_sink, None);
        if (*context).scfia.check_condition(&(*context).scfia.new_bool_eq(&mode.clone(), &(*context).scfia.new_bv_concrete(0b0, 2), None, false, &mut (*context).fork_sink, None), &mut (*context).fork_sink) {
            imm32 = (*context).scfia.new_bv_concat(&(*context).scfia.new_bv_concrete(0b0, 24), &(*context).scfia.new_bv_slice(&imm12.clone(), 7, 0, None, &mut (*context).fork_sink, None), 32, None, &mut (*context).fork_sink, None);
        }
        else if (*context).scfia.check_condition(&(*context).scfia.new_bool_eq(&mode.clone(), &(*context).scfia.new_bv_concrete(0b0, 2), None, false, &mut (*context).fork_sink, None), &mut (*context).fork_sink) {
            unimplemented!();
        }
        else if (*context).scfia.check_condition(&(*context).scfia.new_bool_eq(&mode.clone(), &(*context).scfia.new_bv_concrete(0b0, 2), None, false, &mut (*context).fork_sink, None), &mut (*context).fork_sink) {
            unimplemented!();
        }
        else {
            unimplemented!();
        }
        carry_out = carry_in.clone();
    }
    else {
        let mut imm7: ActiveValue<STM32ScfiaComposition> = (*context).scfia.new_bv_slice(&imm12.clone(), 6, 0, None, &mut (*context).fork_sink, None);
        let mut imm8: ActiveValue<STM32ScfiaComposition> = (*context).scfia.new_bv_concat(&(*context).scfia.new_bv_concrete(0b1, 1), &imm7.clone(), 8, None, &mut (*context).fork_sink, None);
        let mut unrotated_value: ActiveValue<STM32ScfiaComposition> = (*context).scfia.new_bv_concat(&(*context).scfia.new_bv_concrete(0b0, 24), &imm8.clone(), 32, None, &mut (*context).fork_sink, None);
        let mut shift5: ActiveValue<STM32ScfiaComposition> = (*context).scfia.new_bv_slice(&imm12.clone(), 11, 7, None, &mut (*context).fork_sink, None);
        let mut result_carry: ActiveValue<STM32ScfiaComposition> = _ror_C_BV32(unrotated_value.clone(), (*context).scfia.new_bv_concat(&(*context).scfia.new_bv_concrete(0b0, 27), &shift5.clone(), 32, None, &mut (*context).fork_sink, None), context);
        imm32 = (*context).scfia.new_bv_slice(&result_carry.clone(), 32, 1, None, &mut (*context).fork_sink, None);
        carry_out = (*context).scfia.new_bv_slice(&result_carry.clone(), 0, 0, None, &mut (*context).fork_sink, None);
    }
    return (*context).scfia.new_bv_concat(&imm32.clone(), &carry_out.clone(), 33, None, &mut (*context).fork_sink, None);
}

unsafe fn _ror_C_BV32(x: ActiveValue<STM32ScfiaComposition>, shift: ActiveValue<STM32ScfiaComposition>, context: *mut StepContext<STM32ScfiaComposition>) -> ActiveValue<STM32ScfiaComposition> {
    if (*context).scfia.check_condition(&(*context).scfia.new_bool_eq(&shift.clone(), &(*context).scfia.new_bv_concrete(0b0, 32), None, false, &mut (*context).fork_sink, None), &mut (*context).fork_sink) {
        unimplemented!();
    }
    let mut m: ActiveValue<STM32ScfiaComposition> = (*context).scfia.new_bv_unsigned_remainder(&shift.clone(), &(*context).scfia.new_bv_concrete(0b100000, 32), 32, None, &mut (*context).fork_sink, None);
    let mut i1: ActiveValue<STM32ScfiaComposition> = (*context).scfia.new_bv_srl(&x.clone(), &m.clone(), 32, None, &mut (*context).fork_sink, None);
    let mut i2: ActiveValue<STM32ScfiaComposition> = (*context).scfia.new_bv_sll(&x.clone(), &(*context).scfia.new_bv_sub(&(*context).scfia.new_bv_concrete(0b11111, 32), &m.clone(), 32, None, &mut (*context).fork_sink, None), 32, None, &mut (*context).fork_sink, None);
    let mut result: ActiveValue<STM32ScfiaComposition> = (*context).scfia.new_bv_or(&i1.clone(), &i2.clone(), 32, None, &mut (*context).fork_sink, None);
    let mut carry_out: ActiveValue<STM32ScfiaComposition> = (*context).scfia.new_bv_slice(&result.clone(), 31, 31, None, &mut (*context).fork_sink, None);
    return (*context).scfia.new_bv_concat(&result.clone(), &carry_out.clone(), 33, None, &mut (*context).fork_sink, None);
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

unsafe fn _condition_passed(condition: ActiveValue<STM32ScfiaComposition>, state: *mut SystemState, context: *mut StepContext<STM32ScfiaComposition>) -> ActiveValue<STM32ScfiaComposition> {
    let mut cond: ActiveValue<STM32ScfiaComposition> = (*context).scfia.new_bv_slice(&condition.clone(), 3, 1, None, &mut (*context).fork_sink, None);
    let mut result: ActiveValue<STM32ScfiaComposition> = (*context).scfia.new_bv_concrete(0b0, 1);
    if (*context).scfia.check_condition(&(*context).scfia.new_bool_eq(&cond.clone(), &(*context).scfia.new_bv_concrete(0b0, 3), None, false, &mut (*context).fork_sink, None), &mut (*context).fork_sink) {
        result = (*state).apsr.Z.clone();
    }
    else if (*context).scfia.check_condition(&(*context).scfia.new_bool_eq(&cond.clone(), &(*context).scfia.new_bv_concrete(0b1, 3), None, false, &mut (*context).fork_sink, None), &mut (*context).fork_sink) {
        result = (*state).apsr.C.clone();
    }
    else if (*context).scfia.check_condition(&(*context).scfia.new_bool_eq(&cond.clone(), &(*context).scfia.new_bv_concrete(0b10, 3), None, false, &mut (*context).fork_sink, None), &mut (*context).fork_sink) {
        result = (*state).apsr.N.clone();
    }
    else if (*context).scfia.check_condition(&(*context).scfia.new_bool_eq(&cond.clone(), &(*context).scfia.new_bv_concrete(0b11, 3), None, false, &mut (*context).fork_sink, None), &mut (*context).fork_sink) {
        result = (*state).apsr.V.clone();
    }
    else if (*context).scfia.check_condition(&(*context).scfia.new_bool_eq(&cond.clone(), &(*context).scfia.new_bv_concrete(0b100, 3), None, false, &mut (*context).fork_sink, None), &mut (*context).fork_sink) {
        unimplemented!();
    }
    else if (*context).scfia.check_condition(&(*context).scfia.new_bool_eq(&cond.clone(), &(*context).scfia.new_bv_concrete(0b101, 3), None, false, &mut (*context).fork_sink, None), &mut (*context).fork_sink) {
        unimplemented!();
    }
    else if (*context).scfia.check_condition(&(*context).scfia.new_bool_eq(&cond.clone(), &(*context).scfia.new_bv_concrete(0b110, 3), None, false, &mut (*context).fork_sink, None), &mut (*context).fork_sink) {
        unimplemented!();
    }
    else if (*context).scfia.check_condition(&(*context).scfia.new_bool_eq(&cond.clone(), &(*context).scfia.new_bv_concrete(0b111, 3), None, false, &mut (*context).fork_sink, None), &mut (*context).fork_sink) {
        unimplemented!();
    }
    let mut wat: ActiveValue<STM32ScfiaComposition> = (*context).scfia.new_bv_slice(&condition.clone(), 0, 0, None, &mut (*context).fork_sink, None);
    if (*context).scfia.check_condition(&(*context).scfia.new_bool_eq(&wat.clone(), &(*context).scfia.new_bv_concrete(0b1, 1), None, false, &mut (*context).fork_sink, None), &mut (*context).fork_sink) {
        if (*context).scfia.check_condition(&(*context).scfia.new_bool_not(&(*context).scfia.new_bool_eq(&condition.clone(), &(*context).scfia.new_bv_concrete(0b1111, 4), None, false, &mut (*context).fork_sink, None), None, false, &mut (*context).fork_sink, None), &mut (*context).fork_sink) {
            result = (*context).scfia.new_bv_not(&result.clone(), 1, None, &mut (*context).fork_sink, None);
        }
    }
    return result.clone();
}

unsafe fn _load_register(t: ActiveValue<STM32ScfiaComposition>, imm32: ActiveValue<STM32ScfiaComposition>, add: ActiveValue<STM32ScfiaComposition>, state: *mut SystemState, context: *mut StepContext<STM32ScfiaComposition>) {
    let mut base: ActiveValue<STM32ScfiaComposition> = (*context).scfia.new_bv_and(&(*state).PC.clone(), &(*context).scfia.new_bv_not(&(*context).scfia.new_bv_concrete(0b11, 32), 32, None, &mut (*context).fork_sink, None), 32, None, &mut (*context).fork_sink, None);
    if (*context).scfia.check_condition(&(*context).scfia.new_bool_eq(&add.clone(), &(*context).scfia.new_bv_concrete(0b1, 1), None, false, &mut (*context).fork_sink, None), &mut (*context).fork_sink) {
        let mut address: ActiveValue<STM32ScfiaComposition> = (*context).scfia.new_bv_add(&base.clone(), &imm32.clone(), 32, None, &mut (*context).fork_sink, None);
        _register_write_BV32(t.clone(), (*(*context).memory).read(&address.clone(), 32, (*context).scfia.clone(), &mut (*context).hints, &mut (*context).fork_sink), state, context);
    }
    else {
        let mut address: ActiveValue<STM32ScfiaComposition> = (*context).scfia.new_bv_sub(&base.clone(), &imm32.clone(), 32, None, &mut (*context).fork_sink, None);
        _register_write_BV32(t.clone(), (*(*context).memory).read(&address.clone(), 32, (*context).scfia.clone(), &mut (*context).hints, &mut (*context).fork_sink), state, context);
    }
}

unsafe fn _branch_write_pc(address: ActiveValue<STM32ScfiaComposition>, state: *mut SystemState, context: *mut StepContext<STM32ScfiaComposition>) {
    let mut address: ActiveValue<STM32ScfiaComposition> = (*context).scfia.new_bv_concat(&(*context).scfia.new_bv_slice(&address.clone(), 31, 1, None, &mut (*context).fork_sink, None), &(*context).scfia.new_bv_concrete(0b0, 1), 32, None, &mut (*context).fork_sink, None);
    _branch_to(address.clone(), state, context);
}

unsafe fn _bx_write_pc(address: ActiveValue<STM32ScfiaComposition>, state: *mut SystemState, context: *mut StepContext<STM32ScfiaComposition>) {
    (*state).EPSR.T = (*context).scfia.new_bv_slice(&address.clone(), 0, 0, None, &mut (*context).fork_sink, None);
    _branch_to((*context).scfia.new_bv_concat(&(*context).scfia.new_bv_slice(&address.clone(), 31, 1, None, &mut (*context).fork_sink, None), &(*context).scfia.new_bv_concrete(0b0, 1), 32, None, &mut (*context).fork_sink, None), state, context);
}

unsafe fn _branch_to(address: ActiveValue<STM32ScfiaComposition>, state: *mut SystemState, context: *mut StepContext<STM32ScfiaComposition>) {
    (*state).PC = address.clone();
}

unsafe fn _matches_BV3(input: ActiveValue<STM32ScfiaComposition>, required_ones: ActiveValue<STM32ScfiaComposition>, required_zeroes: ActiveValue<STM32ScfiaComposition>, context: *mut StepContext<STM32ScfiaComposition>) -> ActiveValue<STM32ScfiaComposition> {
    if (*context).scfia.check_condition(&(*context).scfia.new_bool_eq(&(*context).scfia.new_bv_and(&input.clone(), &required_ones.clone(), 3, None, &mut (*context).fork_sink, None), &required_ones.clone(), None, false, &mut (*context).fork_sink, None), &mut (*context).fork_sink) {
        if (*context).scfia.check_condition(&(*context).scfia.new_bool_eq(&(*context).scfia.new_bv_and(&input.clone(), &required_zeroes.clone(), 3, None, &mut (*context).fork_sink, None), &(*context).scfia.new_bv_concrete(0b0, 3), None, false, &mut (*context).fork_sink, None), &mut (*context).fork_sink) {
            return (*context).scfia.new_bv_concrete(0b1, 1);
        }
    }
    return (*context).scfia.new_bv_concrete(0b0, 1);
}

unsafe fn _matches_BV4(input: ActiveValue<STM32ScfiaComposition>, required_ones: ActiveValue<STM32ScfiaComposition>, required_zeroes: ActiveValue<STM32ScfiaComposition>, context: *mut StepContext<STM32ScfiaComposition>) -> ActiveValue<STM32ScfiaComposition> {
    if (*context).scfia.check_condition(&(*context).scfia.new_bool_eq(&(*context).scfia.new_bv_and(&input.clone(), &required_ones.clone(), 4, None, &mut (*context).fork_sink, None), &required_ones.clone(), None, false, &mut (*context).fork_sink, None), &mut (*context).fork_sink) {
        if (*context).scfia.check_condition(&(*context).scfia.new_bool_eq(&(*context).scfia.new_bv_and(&input.clone(), &required_zeroes.clone(), 4, None, &mut (*context).fork_sink, None), &(*context).scfia.new_bv_concrete(0b0, 4), None, false, &mut (*context).fork_sink, None), &mut (*context).fork_sink) {
            return (*context).scfia.new_bv_concrete(0b1, 1);
        }
    }
    return (*context).scfia.new_bv_concrete(0b0, 1);
}

unsafe fn _matches_BV5(input: ActiveValue<STM32ScfiaComposition>, required_ones: ActiveValue<STM32ScfiaComposition>, required_zeroes: ActiveValue<STM32ScfiaComposition>, context: *mut StepContext<STM32ScfiaComposition>) -> ActiveValue<STM32ScfiaComposition> {
    if (*context).scfia.check_condition(&(*context).scfia.new_bool_eq(&(*context).scfia.new_bv_and(&input.clone(), &required_ones.clone(), 5, None, &mut (*context).fork_sink, None), &required_ones.clone(), None, false, &mut (*context).fork_sink, None), &mut (*context).fork_sink) {
        if (*context).scfia.check_condition(&(*context).scfia.new_bool_eq(&(*context).scfia.new_bv_and(&input.clone(), &required_zeroes.clone(), 5, None, &mut (*context).fork_sink, None), &(*context).scfia.new_bv_concrete(0b0, 5), None, false, &mut (*context).fork_sink, None), &mut (*context).fork_sink) {
            return (*context).scfia.new_bv_concrete(0b1, 1);
        }
    }
    return (*context).scfia.new_bv_concrete(0b0, 1);
}

unsafe fn _matches_BV7(input: ActiveValue<STM32ScfiaComposition>, required_ones: ActiveValue<STM32ScfiaComposition>, required_zeroes: ActiveValue<STM32ScfiaComposition>, context: *mut StepContext<STM32ScfiaComposition>) -> ActiveValue<STM32ScfiaComposition> {
    if (*context).scfia.check_condition(&(*context).scfia.new_bool_eq(&(*context).scfia.new_bv_and(&input.clone(), &required_ones.clone(), 7, None, &mut (*context).fork_sink, None), &required_ones.clone(), None, false, &mut (*context).fork_sink, None), &mut (*context).fork_sink) {
        if (*context).scfia.check_condition(&(*context).scfia.new_bool_eq(&(*context).scfia.new_bv_and(&input.clone(), &required_zeroes.clone(), 7, None, &mut (*context).fork_sink, None), &(*context).scfia.new_bv_concrete(0b0, 7), None, false, &mut (*context).fork_sink, None), &mut (*context).fork_sink) {
            return (*context).scfia.new_bv_concrete(0b1, 1);
        }
    }
    return (*context).scfia.new_bv_concrete(0b0, 1);
}
