#![allow(clippy::all)]
#![allow(non_snake_case)]
#![allow(non_camel_case_types)]
#![allow(unused)]
use log::debug;
use std::{borrow::BorrowMut, fmt::Debug, collections::BTreeMap, rc::Rc};

use crate::{memory::Memory, scfia::Scfia, values::{active_value::ActiveValue, retired_value::RetiredValue}, GenericForkSink, ScfiaComposition, StepContext, SymbolicHints};

pub struct ARMv7M {
    pub state: SystemState,
    pub memory: Memory<ARMv7MScfiaComposition>,
    pub scfia: Rc<Scfia<ARMv7MScfiaComposition>>,
}

#[derive(Debug)]
pub struct ARMv7MForkSink {
    base_state: ARMv7M,
    new_values_history: Vec<ActiveValue<ARMv7MScfiaComposition>>,
    forks: Vec<ARMv7M>,
}

#[derive(Debug, Clone)]
pub struct ARMv7MScfiaComposition {}


impl GenericForkSink<ARMv7MScfiaComposition> for ARMv7MForkSink {
    fn fork(&mut self, fork_symbol: ActiveValue<ARMv7MScfiaComposition>) {
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

    fn push_value(&mut self, value: ActiveValue<ARMv7MScfiaComposition>) {
        self.new_values_history.push(value)
    }
}

impl ScfiaComposition for ARMv7MScfiaComposition {
    type Model = ARMv7M;
    type ForkSink = ARMv7MForkSink;
}

impl ARMv7M {
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

    pub fn step_forking(self, mut hints: Option<SymbolicHints>) -> Vec<ARMv7M> {
        unsafe {
            let mut states: Vec<ARMv7M> = vec![self];
            let mut results = vec![];

            while let Some(mut state) = states.pop() {
                let mut context = StepContext {
                    memory: &mut state.memory,
                    scfia: &state.scfia,
                    hints: hints.clone(),
                    fork_sink: Some(ARMv7MForkSink {
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

    pub fn clone_model(&self) -> (ARMv7M, BTreeMap<u64, ActiveValue<ARMv7MScfiaComposition>>, BTreeMap<u64, RetiredValue<ARMv7MScfiaComposition>>) {
        unsafe {
            let cloned_scfia = Scfia::new(Some(self.scfia.next_symbol_id.get()));
            let mut cloned_actives = BTreeMap::new();
            let mut cloned_retireds = BTreeMap::new();
            debug!("cloning scfia {:?} to {:?}",
                Rc::as_ptr(&self.scfia.selff.get().unwrap().upgrade().unwrap()),
                Rc::as_ptr(&cloned_scfia.selff.get().unwrap().upgrade().unwrap()));
            (ARMv7M {
                state: self.state.clone_to_stdlib(&cloned_scfia, &mut cloned_actives, &mut cloned_retireds),
                memory: self.memory.clone_to_stdlib(&cloned_scfia, &mut cloned_actives, &mut cloned_retireds),
                scfia: cloned_scfia,
            }, cloned_actives, cloned_retireds)
        }
    }
}

impl Debug for ARMv7M {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("ARMv7M").field("state", &self.state).finish()
    }
}

#[derive(Debug)]
pub struct ApplicationProgramStatusRegister {
    pub N: ActiveValue<ARMv7MScfiaComposition>,
    pub Z: ActiveValue<ARMv7MScfiaComposition>,
    pub C: ActiveValue<ARMv7MScfiaComposition>,
    pub V: ActiveValue<ARMv7MScfiaComposition>,
    pub Q: ActiveValue<ARMv7MScfiaComposition>,
    pub GE: ActiveValue<ARMv7MScfiaComposition>
}

impl ApplicationProgramStatusRegister {
    fn clone_to_stdlib(&self, cloned_scfia: &Scfia<ARMv7MScfiaComposition>, cloned_actives: &mut BTreeMap<u64, ActiveValue<ARMv7MScfiaComposition>>, cloned_inactives: &mut BTreeMap<u64, RetiredValue<ARMv7MScfiaComposition>>) -> ApplicationProgramStatusRegister {
        ApplicationProgramStatusRegister {
            N: self.N.clone_to_stdlib(cloned_scfia, cloned_actives, cloned_inactives),
            Z: self.Z.clone_to_stdlib(cloned_scfia, cloned_actives, cloned_inactives),
            C: self.C.clone_to_stdlib(cloned_scfia, cloned_actives, cloned_inactives),
            V: self.V.clone_to_stdlib(cloned_scfia, cloned_actives, cloned_inactives),
            Q: self.Q.clone_to_stdlib(cloned_scfia, cloned_actives, cloned_inactives),
            GE: self.GE.clone_to_stdlib(cloned_scfia, cloned_actives, cloned_inactives)
        }
    }
}

#[derive(Debug)]
pub struct ExecutionProgramStatusRegister {
    pub ICI_IT: ActiveValue<ARMv7MScfiaComposition>,
    pub T: ActiveValue<ARMv7MScfiaComposition>,
    pub ICI_IT2: ActiveValue<ARMv7MScfiaComposition>
}

impl ExecutionProgramStatusRegister {
    fn clone_to_stdlib(&self, cloned_scfia: &Scfia<ARMv7MScfiaComposition>, cloned_actives: &mut BTreeMap<u64, ActiveValue<ARMv7MScfiaComposition>>, cloned_inactives: &mut BTreeMap<u64, RetiredValue<ARMv7MScfiaComposition>>) -> ExecutionProgramStatusRegister {
        ExecutionProgramStatusRegister {
            ICI_IT: self.ICI_IT.clone_to_stdlib(cloned_scfia, cloned_actives, cloned_inactives),
            T: self.T.clone_to_stdlib(cloned_scfia, cloned_actives, cloned_inactives),
            ICI_IT2: self.ICI_IT2.clone_to_stdlib(cloned_scfia, cloned_actives, cloned_inactives)
        }
    }
}

#[derive(Debug)]
pub struct SystemState {
    pub R0: ActiveValue<ARMv7MScfiaComposition>,
    pub R1: ActiveValue<ARMv7MScfiaComposition>,
    pub R2: ActiveValue<ARMv7MScfiaComposition>,
    pub R3: ActiveValue<ARMv7MScfiaComposition>,
    pub R4: ActiveValue<ARMv7MScfiaComposition>,
    pub R5: ActiveValue<ARMv7MScfiaComposition>,
    pub R6: ActiveValue<ARMv7MScfiaComposition>,
    pub R7: ActiveValue<ARMv7MScfiaComposition>,
    pub R8: ActiveValue<ARMv7MScfiaComposition>,
    pub R9: ActiveValue<ARMv7MScfiaComposition>,
    pub R10: ActiveValue<ARMv7MScfiaComposition>,
    pub R11: ActiveValue<ARMv7MScfiaComposition>,
    pub R12: ActiveValue<ARMv7MScfiaComposition>,
    pub SP: ActiveValue<ARMv7MScfiaComposition>,
    pub LR: ActiveValue<ARMv7MScfiaComposition>,
    pub PC: ActiveValue<ARMv7MScfiaComposition>,
    pub APSR: ApplicationProgramStatusRegister,
    pub EPSR: ExecutionProgramStatusRegister
}

impl SystemState {
    fn clone_to_stdlib(&self, cloned_scfia: &Scfia<ARMv7MScfiaComposition>, cloned_actives: &mut BTreeMap<u64, ActiveValue<ARMv7MScfiaComposition>>, cloned_inactives: &mut BTreeMap<u64, RetiredValue<ARMv7MScfiaComposition>>) -> SystemState {
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
            APSR: self.APSR.clone_to_stdlib(cloned_scfia, cloned_actives, cloned_inactives),
            EPSR: self.EPSR.clone_to_stdlib(cloned_scfia, cloned_actives, cloned_inactives)
        }
    }
}

#[derive(Debug, Eq, PartialEq)]
pub enum SRType {
    None,
    LSL,
    LSR,
    ASR,
    ROR,
    RRX
}

unsafe fn _step(mut state: *mut SystemState, context: *mut StepContext<ARMv7MScfiaComposition>) {
    let mut address_low: ActiveValue<ARMv7MScfiaComposition> = (*context).scfia.new_bv_slice(&(*state).PC.clone(), 1, 0, None, &mut (*context).fork_sink, None);
    //if (EQUALS_BV2(address_low, BV2(0b00))) {
    // PC is 4 byte aligned
    let mut instruction32: ActiveValue<ARMv7MScfiaComposition> = (*(*context).memory).read(&(*state).PC.clone(), 32, (*context).scfia.clone(), &mut (*context).hints, &mut (*context).fork_sink);
    _progress_pc_4(state, context);
    let mut b5: ActiveValue<ARMv7MScfiaComposition> = (*context).scfia.new_bv_slice(&instruction32.clone(), 15, 11, None, &mut (*context).fork_sink, None);
    if (*context).scfia.check_condition(&(*context).scfia.new_bool_eq(&b5.clone(), &(*context).scfia.new_bv_concrete(0b11101, 5), None, false, &mut (*context).fork_sink, None), &mut (*context).fork_sink) {
        _thumb32(instruction32.clone(), state, context);
    } else if (*context).scfia.check_condition(&(*context).scfia.new_bool_eq(&b5.clone(), &(*context).scfia.new_bv_concrete(0b11110, 5), None, false, &mut (*context).fork_sink, None), &mut (*context).fork_sink) {
        _thumb32(instruction32.clone(), state, context);
    } else if (*context).scfia.check_condition(&(*context).scfia.new_bool_eq(&b5.clone(), &(*context).scfia.new_bv_concrete(0b11111, 5), None, false, &mut (*context).fork_sink, None), &mut (*context).fork_sink) {
        _thumb32(instruction32.clone(), state, context);
    } else {
        // PC points to a thumb16 instruction
        let mut old_pc: ActiveValue<ARMv7MScfiaComposition> = (*state).PC.clone();
        _thumb16((*context).scfia.new_bv_slice(&instruction32.clone(), 15, 0, None, &mut (*context).fork_sink, None), state, context);
        if (*context).scfia.check_condition(&(*context).scfia.new_bool_eq(&old_pc.clone(), &(*state).PC.clone(), None, false, &mut (*context).fork_sink, None), &mut (*context).fork_sink) {
            let mut b5: ActiveValue<ARMv7MScfiaComposition> = (*context).scfia.new_bv_slice(&instruction32.clone(), 31, 27, None, &mut (*context).fork_sink, None);
            if (*context).scfia.check_condition(&(*context).scfia.new_bool_eq(&b5.clone(), &(*context).scfia.new_bv_concrete(0b11101, 5), None, false, &mut (*context).fork_sink, None), &mut (*context).fork_sink) {
                instruction32 = (*context).scfia.new_bv_concat(&(*(*context).memory).read(&(*state).PC.clone(), 16, (*context).scfia.clone(), &mut (*context).hints, &mut (*context).fork_sink), &(*context).scfia.new_bv_slice(&instruction32.clone(), 31, 16, None, &mut (*context).fork_sink, None), 32, None, &mut (*context).fork_sink, None);
                _progress_pc_2(state, context);
                _thumb32(instruction32.clone(), state, context);
            } else if (*context).scfia.check_condition(&(*context).scfia.new_bool_eq(&b5.clone(), &(*context).scfia.new_bv_concrete(0b11110, 5), None, false, &mut (*context).fork_sink, None), &mut (*context).fork_sink) {
                instruction32 = (*context).scfia.new_bv_concat(&(*(*context).memory).read(&(*state).PC.clone(), 16, (*context).scfia.clone(), &mut (*context).hints, &mut (*context).fork_sink), &(*context).scfia.new_bv_slice(&instruction32.clone(), 31, 16, None, &mut (*context).fork_sink, None), 32, None, &mut (*context).fork_sink, None);
                _progress_pc_2(state, context);
                _thumb32(instruction32.clone(), state, context);
            } else if (*context).scfia.check_condition(&(*context).scfia.new_bool_eq(&b5.clone(), &(*context).scfia.new_bv_concrete(0b11111, 5), None, false, &mut (*context).fork_sink, None), &mut (*context).fork_sink) {
                instruction32 = (*context).scfia.new_bv_concat(&(*(*context).memory).read(&(*state).PC.clone(), 16, (*context).scfia.clone(), &mut (*context).hints, &mut (*context).fork_sink), &(*context).scfia.new_bv_slice(&instruction32.clone(), 31, 16, None, &mut (*context).fork_sink, None), 32, None, &mut (*context).fork_sink, None);
                _progress_pc_2(state, context);
                _thumb32(instruction32.clone(), state, context);
            } else {
                _thumb16((*context).scfia.new_bv_slice(&instruction32.clone(), 31, 16, None, &mut (*context).fork_sink, None), state, context);
                //TODO do I need to check the first 5 bits here?
            }
        }
    }
    //}
    //elif (EQUALS_BV2(address_low, BV2(0b01))) {
    //    UNIMPLEMENTED();
    //} elif (EQUALS_BV2(address_low, BV2(0b10))) {
    //    // PC is 2 byte aligned
    //    instruction: BV16 = PHYSICAL_MEMORY_32_READ_BV16(state.PC);
    //    progress_pc_2(state);
    //    b5: BV5 = SLICE_BV16_15_11(instruction);
    //    if (EQUALS_BV5(b5, BV5(0b11101))) {
    //        instruction32: BV32 = CONCAT_BV16_BV16(PHYSICAL_MEMORY_32_READ_BV16(state.PC), instruction);
    //        progress_pc_2(state);
    //        thumb32(instruction32, state);
    //    } elif (EQUALS_BV5(b5, BV5(0b11110))) {
    //        instruction32: BV32 = CONCAT_BV16_BV16(PHYSICAL_MEMORY_32_READ_BV16(state.PC), instruction);
    //        progress_pc_2(state);
    //        thumb32(instruction32, state);
    //    } elif (EQUALS_BV5(b5, BV5(0b11111))) {
    //        instruction32: BV32 = CONCAT_BV16_BV16(PHYSICAL_MEMORY_32_READ_BV16(state.PC), instruction);
    //        progress_pc_2(state);
    //        thumb32(instruction32, state);
    //    } else {
    //        // PC points to a thumb16 instruction
    //        old_pc: BV32 = state.PC;
    //        thumb16(instruction, state);
    //    }
    //} elif (EQUALS_BV2(address_low, BV2(0b11))) {
    //    UNIMPLEMENTED();
    //}
}

unsafe fn _thumb32(mut instruction: ActiveValue<ARMv7MScfiaComposition>, mut state: *mut SystemState, context: *mut StepContext<ARMv7MScfiaComposition>) {
    let mut instruction2: ActiveValue<ARMv7MScfiaComposition> = (*context).scfia.new_bv_slice(&instruction.clone(), 31, 16, None, &mut (*context).fork_sink, None);
    let mut instruction1: ActiveValue<ARMv7MScfiaComposition> = (*context).scfia.new_bv_slice(&instruction.clone(), 15, 0, None, &mut (*context).fork_sink, None);
    let mut op1: ActiveValue<ARMv7MScfiaComposition> = (*context).scfia.new_bv_slice(&instruction1.clone(), 12, 11, None, &mut (*context).fork_sink, None);
    let mut op2: ActiveValue<ARMv7MScfiaComposition> = (*context).scfia.new_bv_slice(&instruction1.clone(), 10, 4, None, &mut (*context).fork_sink, None);
    let mut op: ActiveValue<ARMv7MScfiaComposition> = (*context).scfia.new_bv_slice(&instruction2.clone(), 15, 15, None, &mut (*context).fork_sink, None);
    if (*context).scfia.check_condition(&(*context).scfia.new_bool_eq(&op1.clone(), &(*context).scfia.new_bv_concrete(0b01, 2), None, false, &mut (*context).fork_sink, None), &mut (*context).fork_sink) {
        if (*context).scfia.check_condition(&(*context).scfia.new_bool_eq(&_matches_BV7(op2.clone(), (*context).scfia.new_bv_concrete(0b0000000, 7), (*context).scfia.new_bv_concrete(0b1100100, 7), context), &(*context).scfia.new_bv_concrete(1, 1), None, false, &mut (*context).fork_sink, None), &mut (*context).fork_sink) {
            _thumb32_load_multiple_and_store_multiple(instruction1.clone(), instruction2.clone(), state, context);
        } else {
            //TODO do remaining foo
            unimplemented!();
        }
    } else if (*context).scfia.check_condition(&(*context).scfia.new_bool_eq(&op1.clone(), &(*context).scfia.new_bv_concrete(0b10, 2), None, false, &mut (*context).fork_sink, None), &mut (*context).fork_sink) {
        if (*context).scfia.check_condition(&(*context).scfia.new_bool_eq(&op.clone(), &(*context).scfia.new_bv_concrete(0, 1), None, false, &mut (*context).fork_sink, None), &mut (*context).fork_sink) {
            let mut pbi: ActiveValue<ARMv7MScfiaComposition> = (*context).scfia.new_bv_slice(&op2.clone(), 5, 5, None, &mut (*context).fork_sink, None);
            if (*context).scfia.check_condition(&(*context).scfia.new_bool_eq(&pbi.clone(), &(*context).scfia.new_bv_concrete(0, 1), None, false, &mut (*context).fork_sink, None), &mut (*context).fork_sink) {
                // Data processing (modified immediate)
                _thumb32_data_processing_modified_immediate(instruction1.clone(), instruction2.clone(), state, context);
            } else {
                // Data processing (plain binary immediate)
                _thumb32_data_processing_plain_binary_immediate(instruction1.clone(), instruction2.clone(), state, context);
            }
        } else {
            // Branches and miscellaneous control
            _thumb32_branches_and_misc_control(instruction1.clone(), instruction2.clone(), state, context);
        }
    } else if (*context).scfia.check_condition(&(*context).scfia.new_bool_eq(&op1.clone(), &(*context).scfia.new_bv_concrete(0b11, 2), None, false, &mut (*context).fork_sink, None), &mut (*context).fork_sink) {
        if (*context).scfia.check_condition(&(*context).scfia.new_bool_eq(&_matches_BV7(op2.clone(), (*context).scfia.new_bv_concrete(0b0000000, 7), (*context).scfia.new_bv_concrete(0b1110001, 7), context), &(*context).scfia.new_bv_concrete(1, 1), None, false, &mut (*context).fork_sink, None), &mut (*context).fork_sink) {
            // Store single data item
            _thumb32_store_single_data_item(instruction1.clone(), instruction2.clone(), state, context);
        } else if (*context).scfia.check_condition(&(*context).scfia.new_bool_eq(&_matches_BV7(op2.clone(), (*context).scfia.new_bv_concrete(0b0000001, 7), (*context).scfia.new_bv_concrete(0b1100110, 7), context), &(*context).scfia.new_bv_concrete(1, 1), None, false, &mut (*context).fork_sink, None), &mut (*context).fork_sink) {
            // Load byte, memory hints
            unimplemented!();
        } else if (*context).scfia.check_condition(&(*context).scfia.new_bool_eq(&_matches_BV7(op2.clone(), (*context).scfia.new_bv_concrete(0b0000011, 7), (*context).scfia.new_bv_concrete(0b1100100, 7), context), &(*context).scfia.new_bv_concrete(1, 1), None, false, &mut (*context).fork_sink, None), &mut (*context).fork_sink) {
            // Load halfword, memory hints
            unimplemented!();
        } else if (*context).scfia.check_condition(&(*context).scfia.new_bool_eq(&_matches_BV7(op2.clone(), (*context).scfia.new_bv_concrete(0b0000101, 7), (*context).scfia.new_bv_concrete(0b1100010, 7), context), &(*context).scfia.new_bv_concrete(1, 1), None, false, &mut (*context).fork_sink, None), &mut (*context).fork_sink) {
            // Load word
            _thumb32_load_word(instruction1.clone(), instruction2.clone(), state, context);
        } else if (*context).scfia.check_condition(&(*context).scfia.new_bool_eq(&_matches_BV7(op2.clone(), (*context).scfia.new_bv_concrete(0b0000111, 7), (*context).scfia.new_bv_concrete(0b1100000, 7), context), &(*context).scfia.new_bv_concrete(1, 1), None, false, &mut (*context).fork_sink, None), &mut (*context).fork_sink) {
            // Undefinfed
            unimplemented!();
        } else if (*context).scfia.check_condition(&(*context).scfia.new_bool_eq(&_matches_BV7(op2.clone(), (*context).scfia.new_bv_concrete(0b0100000, 7), (*context).scfia.new_bv_concrete(0b1010000, 7), context), &(*context).scfia.new_bv_concrete(1, 1), None, false, &mut (*context).fork_sink, None), &mut (*context).fork_sink) {
            // Data processing (register)
            unimplemented!();
        } else if (*context).scfia.check_condition(&(*context).scfia.new_bool_eq(&_matches_BV7(op2.clone(), (*context).scfia.new_bv_concrete(0b0110000, 7), (*context).scfia.new_bv_concrete(0b1001000, 7), context), &(*context).scfia.new_bv_concrete(1, 1), None, false, &mut (*context).fork_sink, None), &mut (*context).fork_sink) {
            // Multiply, multiply accumulate, and absolute difference
            unimplemented!();
        } else if (*context).scfia.check_condition(&(*context).scfia.new_bool_eq(&_matches_BV7(op2.clone(), (*context).scfia.new_bv_concrete(0b0111000, 7), (*context).scfia.new_bv_concrete(0b1000000, 7), context), &(*context).scfia.new_bv_concrete(1, 1), None, false, &mut (*context).fork_sink, None), &mut (*context).fork_sink) {
            // Long multiply, long multiply accumulate, and divide
            _thumb32_long_multiply_long_multiply_accumluate_and_devide(instruction1.clone(), instruction2.clone(), state, context);
        } else {
            // TODO do the remaining bitmasks
            unimplemented!();
        }
    } else {
        unimplemented!();
    }
}

unsafe fn _thumb32_long_multiply_long_multiply_accumluate_and_devide(mut instruction1: ActiveValue<ARMv7MScfiaComposition>, mut instruction2: ActiveValue<ARMv7MScfiaComposition>, mut state: *mut SystemState, context: *mut StepContext<ARMv7MScfiaComposition>) {
    let mut op2: ActiveValue<ARMv7MScfiaComposition> = (*context).scfia.new_bv_slice(&instruction2.clone(), 7, 4, None, &mut (*context).fork_sink, None);
    let mut op1: ActiveValue<ARMv7MScfiaComposition> = (*context).scfia.new_bv_slice(&instruction1.clone(), 6, 4, None, &mut (*context).fork_sink, None);
    if (*context).scfia.check_condition(&(*context).scfia.new_bool_eq(&op1.clone(), &(*context).scfia.new_bv_concrete(0b000, 3), None, false, &mut (*context).fork_sink, None), &mut (*context).fork_sink) {
        unimplemented!();
    } else if (*context).scfia.check_condition(&(*context).scfia.new_bool_eq(&op1.clone(), &(*context).scfia.new_bv_concrete(0b001, 3), None, false, &mut (*context).fork_sink, None), &mut (*context).fork_sink) {
        unimplemented!();
    } else if (*context).scfia.check_condition(&(*context).scfia.new_bool_eq(&op1.clone(), &(*context).scfia.new_bv_concrete(0b010, 3), None, false, &mut (*context).fork_sink, None), &mut (*context).fork_sink) {
        unimplemented!();
    } else if (*context).scfia.check_condition(&(*context).scfia.new_bool_eq(&op1.clone(), &(*context).scfia.new_bv_concrete(0b011, 3), None, false, &mut (*context).fork_sink, None), &mut (*context).fork_sink) {
        // UDIV Encoding T1
        let mut rm: ActiveValue<ARMv7MScfiaComposition> = (*context).scfia.new_bv_slice(&instruction2.clone(), 3, 0, None, &mut (*context).fork_sink, None);
        let mut rd: ActiveValue<ARMv7MScfiaComposition> = (*context).scfia.new_bv_slice(&instruction2.clone(), 11, 8, None, &mut (*context).fork_sink, None);
        let mut rn: ActiveValue<ARMv7MScfiaComposition> = (*context).scfia.new_bv_slice(&instruction2.clone(), 3, 0, None, &mut (*context).fork_sink, None);
        // TODO unpredictable
        let mut m: ActiveValue<ARMv7MScfiaComposition> = _register_read_BV32_wide(rm.clone(), state, context);
        let mut result: ActiveValue<ARMv7MScfiaComposition> = (*context).scfia.new_bv_concrete(0, 32);
        if (*context).scfia.check_condition(&(*context).scfia.new_bool_eq(&m.clone(), &(*context).scfia.new_bv_concrete(0, 32), None, false, &mut (*context).fork_sink, None), &mut (*context).fork_sink) {
            //TODO if InterZeroDivideTrappingEnabled
            unimplemented!();
        } else {
            result = (*context).scfia.new_bv_udiv(&_register_read_BV32_wide(rn.clone(), state, context), &m.clone(), 32, None, &mut (*context).fork_sink, None);
        }
        _register_write_BV32_wide(rd.clone(), result.clone(), state, context);
    } else if (*context).scfia.check_condition(&(*context).scfia.new_bool_eq(&op1.clone(), &(*context).scfia.new_bv_concrete(0b100, 3), None, false, &mut (*context).fork_sink, None), &mut (*context).fork_sink) {
        unimplemented!();
    } else if (*context).scfia.check_condition(&(*context).scfia.new_bool_eq(&op1.clone(), &(*context).scfia.new_bv_concrete(0b101, 3), None, false, &mut (*context).fork_sink, None), &mut (*context).fork_sink) {
        unimplemented!();
    } else if (*context).scfia.check_condition(&(*context).scfia.new_bool_eq(&op1.clone(), &(*context).scfia.new_bv_concrete(0b110, 3), None, false, &mut (*context).fork_sink, None), &mut (*context).fork_sink) {
        unimplemented!();
    } else {
        //TODO undefined
        unimplemented!();
    }
}

unsafe fn _thumb32_load_multiple_and_store_multiple(mut instruction1: ActiveValue<ARMv7MScfiaComposition>, mut instruction2: ActiveValue<ARMv7MScfiaComposition>, mut state: *mut SystemState, context: *mut StepContext<ARMv7MScfiaComposition>) {
    let mut rn: ActiveValue<ARMv7MScfiaComposition> = (*context).scfia.new_bv_slice(&instruction1.clone(), 3, 0, None, &mut (*context).fork_sink, None);
    let mut l: ActiveValue<ARMv7MScfiaComposition> = (*context).scfia.new_bv_slice(&instruction1.clone(), 4, 4, None, &mut (*context).fork_sink, None);
    let mut w: ActiveValue<ARMv7MScfiaComposition> = (*context).scfia.new_bv_slice(&instruction1.clone(), 5, 5, None, &mut (*context).fork_sink, None);
    let mut op: ActiveValue<ARMv7MScfiaComposition> = (*context).scfia.new_bv_slice(&instruction1.clone(), 8, 7, None, &mut (*context).fork_sink, None);
    let mut wrn: ActiveValue<ARMv7MScfiaComposition> = (*context).scfia.new_bv_concat(&w.clone(), &rn.clone(), 5, None, &mut (*context).fork_sink, None);
    if (*context).scfia.check_condition(&(*context).scfia.new_bool_eq(&op.clone(), &(*context).scfia.new_bv_concrete(0b01, 2), None, false, &mut (*context).fork_sink, None), &mut (*context).fork_sink) {
        unimplemented!();
    } else if (*context).scfia.check_condition(&(*context).scfia.new_bool_eq(&op.clone(), &(*context).scfia.new_bv_concrete(0b10, 2), None, false, &mut (*context).fork_sink, None), &mut (*context).fork_sink) {
        if (*context).scfia.check_condition(&(*context).scfia.new_bool_eq(&l.clone(), &(*context).scfia.new_bv_concrete(0b0, 1), None, false, &mut (*context).fork_sink, None), &mut (*context).fork_sink) {
            // PUSH Encoding T2
            let mut register_list: ActiveValue<ARMv7MScfiaComposition> = (*context).scfia.new_bv_slice(&instruction2.clone(), 12, 0, None, &mut (*context).fork_sink, None);
            let mut m: ActiveValue<ARMv7MScfiaComposition> = (*context).scfia.new_bv_slice(&instruction2.clone(), 14, 14, None, &mut (*context).fork_sink, None);
            let mut registers14: ActiveValue<ARMv7MScfiaComposition> = (*context).scfia.new_bv_concat(&(*context).scfia.new_bv_concrete(0, 1), &register_list.clone(), 14, None, &mut (*context).fork_sink, None);
            let mut registers15: ActiveValue<ARMv7MScfiaComposition> = (*context).scfia.new_bv_concat(&m.clone(), &registers14.clone(), 15, None, &mut (*context).fork_sink, None);
            let mut registers: ActiveValue<ARMv7MScfiaComposition> = (*context).scfia.new_bv_concat(&(*context).scfia.new_bv_concrete(0, 1), &registers15.clone(), 16, None, &mut (*context).fork_sink, None);
            //TODO if bitcount < 2 then unpredictable
            _push(registers.clone(), state, context);
        } else {
            // LDMDB, LDMEA
            unimplemented!();
        }
    } else {
        //TODO undefined
        unimplemented!();
    }
}

unsafe fn _thumb32_data_processing_modified_immediate(mut instruction1: ActiveValue<ARMv7MScfiaComposition>, mut instruction2: ActiveValue<ARMv7MScfiaComposition>, mut state: *mut SystemState, context: *mut StepContext<ARMv7MScfiaComposition>) {
    //TODO Other encodings in this space are UNDEFINED
    let mut op: ActiveValue<ARMv7MScfiaComposition> = (*context).scfia.new_bv_slice(&instruction1.clone(), 8, 4, None, &mut (*context).fork_sink, None);
    let mut rn: ActiveValue<ARMv7MScfiaComposition> = (*context).scfia.new_bv_slice(&instruction1.clone(), 3, 0, None, &mut (*context).fork_sink, None);
    let mut rd: ActiveValue<ARMv7MScfiaComposition> = (*context).scfia.new_bv_slice(&instruction2.clone(), 11, 8, None, &mut (*context).fork_sink, None);
    if (*context).scfia.check_condition(&(*context).scfia.new_bool_eq(&_matches_BV5(op.clone(), (*context).scfia.new_bv_concrete(0b00000, 5), (*context).scfia.new_bv_concrete(0b11110, 5), context), &(*context).scfia.new_bv_concrete(1, 1), None, false, &mut (*context).fork_sink, None), &mut (*context).fork_sink) {
        if (*context).scfia.check_condition(&(*context).scfia.new_bool_not(&(*context).scfia.new_bool_eq(&rd.clone(), &(*context).scfia.new_bv_concrete(0b111, 4), None, false, &mut (*context).fork_sink, None), None, false, &mut (*context).fork_sink, None), &mut (*context).fork_sink) {
            // AND (immediate) Encoding T1
            let mut imm8: ActiveValue<ARMv7MScfiaComposition> = (*context).scfia.new_bv_slice(&instruction2.clone(), 7, 0, None, &mut (*context).fork_sink, None);
            let mut rd: ActiveValue<ARMv7MScfiaComposition> = (*context).scfia.new_bv_slice(&instruction2.clone(), 11, 8, None, &mut (*context).fork_sink, None);
            let mut imm3: ActiveValue<ARMv7MScfiaComposition> = (*context).scfia.new_bv_slice(&instruction2.clone(), 14, 12, None, &mut (*context).fork_sink, None);
            let mut rn: ActiveValue<ARMv7MScfiaComposition> = (*context).scfia.new_bv_slice(&instruction1.clone(), 3, 0, None, &mut (*context).fork_sink, None);
            let mut i: ActiveValue<ARMv7MScfiaComposition> = (*context).scfia.new_bv_slice(&instruction1.clone(), 10, 10, None, &mut (*context).fork_sink, None);
            let mut setflags: ActiveValue<ARMv7MScfiaComposition> = (*context).scfia.new_bv_slice(&instruction1.clone(), 4, 4, None, &mut (*context).fork_sink, None);
            let mut imm11: ActiveValue<ARMv7MScfiaComposition> = (*context).scfia.new_bv_concat(&imm3.clone(), &imm8.clone(), 11, None, &mut (*context).fork_sink, None);
            let mut imm12: ActiveValue<ARMv7MScfiaComposition> = (*context).scfia.new_bv_concat(&i.clone(), &imm11.clone(), 12, None, &mut (*context).fork_sink, None);
            let (mut imm32, mut carry) = _thumb_expand_imm_c(imm12.clone(), (*state).APSR.C.clone(), context);
            //TODO unpredictable voodoo
            let mut result: ActiveValue<ARMv7MScfiaComposition> = (*context).scfia.new_bv_and(&_register_read_BV32_wide(rn.clone(), state, context), &imm32.clone(), 32, None, &mut (*context).fork_sink, None);
            _register_write_BV32_wide(rd.clone(), result.clone(), state, context);
            if (*context).scfia.check_condition(&(*context).scfia.new_bool_eq(&setflags.clone(), &(*context).scfia.new_bv_concrete(1, 1), None, false, &mut (*context).fork_sink, None), &mut (*context).fork_sink) {
                (*state).APSR.N = (*context).scfia.new_bv_slice(&result.clone(), 31, 31, None, &mut (*context).fork_sink, None);
                (*state).APSR.Z = _is_zero_bit_BV32(result.clone(), context);
                (*state).APSR.C = carry.clone();
            }
        } else {
            //TST
            unimplemented!();
        }
    } else if (*context).scfia.check_condition(&(*context).scfia.new_bool_eq(&_matches_BV5(op.clone(), (*context).scfia.new_bv_concrete(0b00010, 5), (*context).scfia.new_bv_concrete(0b11100, 5), context), &(*context).scfia.new_bv_concrete(1, 1), None, false, &mut (*context).fork_sink, None), &mut (*context).fork_sink) {
        // BIC (immediate) Encoding T1
        let mut setflags: ActiveValue<ARMv7MScfiaComposition> = (*context).scfia.new_bv_slice(&instruction1.clone(), 4, 4, None, &mut (*context).fork_sink, None);
        let mut imm8: ActiveValue<ARMv7MScfiaComposition> = (*context).scfia.new_bv_slice(&instruction2.clone(), 7, 0, None, &mut (*context).fork_sink, None);
        let mut imm3: ActiveValue<ARMv7MScfiaComposition> = (*context).scfia.new_bv_slice(&instruction2.clone(), 14, 12, None, &mut (*context).fork_sink, None);
        let mut i: ActiveValue<ARMv7MScfiaComposition> = (*context).scfia.new_bv_slice(&instruction1.clone(), 10, 10, None, &mut (*context).fork_sink, None);
        let mut imm11: ActiveValue<ARMv7MScfiaComposition> = (*context).scfia.new_bv_concat(&imm3.clone(), &imm8.clone(), 11, None, &mut (*context).fork_sink, None);
        let mut imm12: ActiveValue<ARMv7MScfiaComposition> = (*context).scfia.new_bv_concat(&i.clone(), &imm11.clone(), 12, None, &mut (*context).fork_sink, None);
        let (mut imm32, mut carry) = _thumb_expand_imm_c(imm12.clone(), (*state).APSR.C.clone(), context);
        // TODO UNPREDICTABLE for some sources and dests

        let mut result: ActiveValue<ARMv7MScfiaComposition> = (*context).scfia.new_bv_and(&_register_read_BV32_wide(rn.clone(), state, context), &(*context).scfia.new_bv_not(&imm32.clone(), 32, None, &mut (*context).fork_sink, None), 32, None, &mut (*context).fork_sink, None);
        if (*context).scfia.check_condition(&(*context).scfia.new_bool_eq(&setflags.clone(), &(*context).scfia.new_bv_concrete(1, 1), None, false, &mut (*context).fork_sink, None), &mut (*context).fork_sink) {
            (*state).APSR.N = (*context).scfia.new_bv_slice(&result.clone(), 31, 31, None, &mut (*context).fork_sink, None);
            (*state).APSR.Z = _is_zero_bit_BV32(result.clone(), context);
            (*state).APSR.C = carry.clone();
        }
    } else if (*context).scfia.check_condition(&(*context).scfia.new_bool_eq(&_matches_BV5(op.clone(), (*context).scfia.new_bv_concrete(0b00100, 5), (*context).scfia.new_bv_concrete(0b11010, 5), context), &(*context).scfia.new_bv_concrete(1, 1), None, false, &mut (*context).fork_sink, None), &mut (*context).fork_sink) {
        if (*context).scfia.check_condition(&(*context).scfia.new_bool_not(&(*context).scfia.new_bool_eq(&rn.clone(), &(*context).scfia.new_bv_concrete(0b111, 4), None, false, &mut (*context).fork_sink, None), None, false, &mut (*context).fork_sink, None), &mut (*context).fork_sink) {
            // ORR (immediate) Encoding T1
            let mut n: ActiveValue<ARMv7MScfiaComposition> = _register_read_BV32_wide(rn.clone(), state, context);
            let mut s: ActiveValue<ARMv7MScfiaComposition> = (*context).scfia.new_bv_slice(&instruction1.clone(), 4, 4, None, &mut (*context).fork_sink, None);
            let mut i: ActiveValue<ARMv7MScfiaComposition> = (*context).scfia.new_bv_slice(&instruction1.clone(), 10, 10, None, &mut (*context).fork_sink, None);
            let mut imm3: ActiveValue<ARMv7MScfiaComposition> = (*context).scfia.new_bv_slice(&instruction2.clone(), 14, 12, None, &mut (*context).fork_sink, None);
            let mut imm8: ActiveValue<ARMv7MScfiaComposition> = (*context).scfia.new_bv_slice(&instruction2.clone(), 7, 0, None, &mut (*context).fork_sink, None);
            let mut imm11: ActiveValue<ARMv7MScfiaComposition> = (*context).scfia.new_bv_concat(&imm3.clone(), &imm8.clone(), 11, None, &mut (*context).fork_sink, None);
            let mut imm12: ActiveValue<ARMv7MScfiaComposition> = (*context).scfia.new_bv_concat(&i.clone(), &imm11.clone(), 12, None, &mut (*context).fork_sink, None);
            let (mut imm32, mut carry) = _thumb_expand_imm_c(imm12.clone(), (*state).APSR.C.clone(), context);
            let mut result: ActiveValue<ARMv7MScfiaComposition> = (*context).scfia.new_bv_or(&n.clone(), &imm32.clone(), 32, None, &mut (*context).fork_sink, None);
            _register_write_BV32_wide(rd.clone(), result.clone(), state, context);
            if (*context).scfia.check_condition(&(*context).scfia.new_bool_eq(&s.clone(), &(*context).scfia.new_bv_concrete(1, 1), None, false, &mut (*context).fork_sink, None), &mut (*context).fork_sink) {
                (*state).APSR.N = (*context).scfia.new_bv_slice(&result.clone(), 31, 31, None, &mut (*context).fork_sink, None);
                (*state).APSR.Z = _is_zero_bit_BV32(result.clone(), context);
                (*state).APSR.C = carry.clone();
            }
        } else {
            // MOV
            unimplemented!();
        }
    } else if (*context).scfia.check_condition(&(*context).scfia.new_bool_eq(&_matches_BV5(op.clone(), (*context).scfia.new_bv_concrete(0b00110, 5), (*context).scfia.new_bv_concrete(0b11000, 5), context), &(*context).scfia.new_bv_concrete(1, 1), None, false, &mut (*context).fork_sink, None), &mut (*context).fork_sink) {
        unimplemented!();
    } else if (*context).scfia.check_condition(&(*context).scfia.new_bool_eq(&_matches_BV5(op.clone(), (*context).scfia.new_bv_concrete(0b01000, 5), (*context).scfia.new_bv_concrete(0b10110, 5), context), &(*context).scfia.new_bv_concrete(1, 1), None, false, &mut (*context).fork_sink, None), &mut (*context).fork_sink) {
        unimplemented!();
    } else if (*context).scfia.check_condition(&(*context).scfia.new_bool_eq(&_matches_BV5(op.clone(), (*context).scfia.new_bv_concrete(0b10000, 5), (*context).scfia.new_bv_concrete(0b01110, 5), context), &(*context).scfia.new_bv_concrete(1, 1), None, false, &mut (*context).fork_sink, None), &mut (*context).fork_sink) {
        unimplemented!();
    } else if (*context).scfia.check_condition(&(*context).scfia.new_bool_eq(&_matches_BV5(op.clone(), (*context).scfia.new_bv_concrete(0b10100, 5), (*context).scfia.new_bv_concrete(0b01010, 5), context), &(*context).scfia.new_bv_concrete(1, 1), None, false, &mut (*context).fork_sink, None), &mut (*context).fork_sink) {
        // ADC (immediate)
        unimplemented!();
    } else if (*context).scfia.check_condition(&(*context).scfia.new_bool_eq(&_matches_BV5(op.clone(), (*context).scfia.new_bv_concrete(0b10110, 5), (*context).scfia.new_bv_concrete(0b01000, 5), context), &(*context).scfia.new_bv_concrete(1, 1), None, false, &mut (*context).fork_sink, None), &mut (*context).fork_sink) {
        // SBC (immediate)
        unimplemented!();
    } else if (*context).scfia.check_condition(&(*context).scfia.new_bool_eq(&_matches_BV5(op.clone(), (*context).scfia.new_bv_concrete(0b11010, 5), (*context).scfia.new_bv_concrete(0b00100, 5), context), &(*context).scfia.new_bv_concrete(1, 1), None, false, &mut (*context).fork_sink, None), &mut (*context).fork_sink) {
        // SUB/CMP
        if (*context).scfia.check_condition(&(*context).scfia.new_bool_not(&(*context).scfia.new_bool_eq(&rd.clone(), &(*context).scfia.new_bv_concrete(0b1111, 4), None, false, &mut (*context).fork_sink, None), None, false, &mut (*context).fork_sink, None), &mut (*context).fork_sink) {
            // SUB
            unimplemented!();
        } else {
            // CMP (immediate) Encoding T2
            let mut imm8: ActiveValue<ARMv7MScfiaComposition> = (*context).scfia.new_bv_slice(&instruction2.clone(), 7, 0, None, &mut (*context).fork_sink, None);
            let mut imm3: ActiveValue<ARMv7MScfiaComposition> = (*context).scfia.new_bv_slice(&instruction2.clone(), 14, 12, None, &mut (*context).fork_sink, None);
            let mut rn: ActiveValue<ARMv7MScfiaComposition> = (*context).scfia.new_bv_slice(&instruction1.clone(), 3, 0, None, &mut (*context).fork_sink, None);
            let mut i: ActiveValue<ARMv7MScfiaComposition> = (*context).scfia.new_bv_slice(&instruction1.clone(), 10, 10, None, &mut (*context).fork_sink, None);
            let mut imm11: ActiveValue<ARMv7MScfiaComposition> = (*context).scfia.new_bv_concat(&imm3.clone(), &imm8.clone(), 11, None, &mut (*context).fork_sink, None);
            let mut imm12: ActiveValue<ARMv7MScfiaComposition> = (*context).scfia.new_bv_concat(&i.clone(), &imm11.clone(), 12, None, &mut (*context).fork_sink, None);
            let mut imm32: ActiveValue<ARMv7MScfiaComposition> = _thumb_expand_imm(imm12.clone(), state, context);
            let (mut result, mut carry, mut overflow) = _add_with_carry_BV32(_register_read_BV32_wide(rn.clone(), state, context), (*context).scfia.new_bv_not(&imm32.clone(), 32, None, &mut (*context).fork_sink, None), (*context).scfia.new_bv_concrete(1, 1), context);
            (*state).APSR.N = (*context).scfia.new_bv_slice(&result.clone(), 31, 31, None, &mut (*context).fork_sink, None);
            (*state).APSR.Z = _is_zero_bit_BV32(result.clone(), context);
            (*state).APSR.C = carry.clone();
            (*state).APSR.V = overflow.clone();
        }
    } else if (*context).scfia.check_condition(&(*context).scfia.new_bool_eq(&_matches_BV5(op.clone(), (*context).scfia.new_bv_concrete(0b11100, 5), (*context).scfia.new_bv_concrete(0b00010, 5), context), &(*context).scfia.new_bv_concrete(1, 1), None, false, &mut (*context).fork_sink, None), &mut (*context).fork_sink) {
        // RSB (immediate) Encoding T2
        let mut imm8: ActiveValue<ARMv7MScfiaComposition> = (*context).scfia.new_bv_slice(&instruction2.clone(), 7, 0, None, &mut (*context).fork_sink, None);
        let mut rd: ActiveValue<ARMv7MScfiaComposition> = (*context).scfia.new_bv_slice(&instruction2.clone(), 11, 8, None, &mut (*context).fork_sink, None);
        let mut imm3: ActiveValue<ARMv7MScfiaComposition> = (*context).scfia.new_bv_slice(&instruction2.clone(), 14, 12, None, &mut (*context).fork_sink, None);
        let mut rn: ActiveValue<ARMv7MScfiaComposition> = (*context).scfia.new_bv_slice(&instruction1.clone(), 3, 0, None, &mut (*context).fork_sink, None);
        let mut setflags: ActiveValue<ARMv7MScfiaComposition> = (*context).scfia.new_bv_slice(&instruction1.clone(), 4, 4, None, &mut (*context).fork_sink, None);
        let mut i: ActiveValue<ARMv7MScfiaComposition> = (*context).scfia.new_bv_slice(&instruction1.clone(), 10, 10, None, &mut (*context).fork_sink, None);
        let mut imm11: ActiveValue<ARMv7MScfiaComposition> = (*context).scfia.new_bv_concat(&imm3.clone(), &imm8.clone(), 11, None, &mut (*context).fork_sink, None);
        let mut imm12: ActiveValue<ARMv7MScfiaComposition> = (*context).scfia.new_bv_concat(&i.clone(), &imm11.clone(), 12, None, &mut (*context).fork_sink, None);
        let mut imm32: ActiveValue<ARMv7MScfiaComposition> = _thumb_expand_imm(imm12.clone(), state, context);
        //TODO if d/n in 13/15 unpredictable
        let (mut result, mut carry, mut overflow) = _add_with_carry_BV32((*context).scfia.new_bv_not(&_register_read_BV32_wide(rn.clone(), state, context), 32, None, &mut (*context).fork_sink, None), imm32.clone(), (*context).scfia.new_bv_concrete(1, 1), context);
        _register_write_BV32_wide(rd.clone(), result.clone(), state, context);
        if (*context).scfia.check_condition(&(*context).scfia.new_bool_eq(&setflags.clone(), &(*context).scfia.new_bv_concrete(1, 1), None, false, &mut (*context).fork_sink, None), &mut (*context).fork_sink) {
            (*state).APSR.N = (*context).scfia.new_bv_slice(&result.clone(), 31, 31, None, &mut (*context).fork_sink, None);
            (*state).APSR.Z = _is_zero_bit_BV32(result.clone(), context);
            (*state).APSR.C = carry.clone();
            (*state).APSR.V = overflow.clone();
        }
    } else {
        //TODO undefined
        unimplemented!();
    }
}

unsafe fn _thumb32_data_processing_plain_binary_immediate(mut instruction1: ActiveValue<ARMv7MScfiaComposition>, mut instruction2: ActiveValue<ARMv7MScfiaComposition>, mut state: *mut SystemState, context: *mut StepContext<ARMv7MScfiaComposition>) {
    let mut rn: ActiveValue<ARMv7MScfiaComposition> = (*context).scfia.new_bv_slice(&instruction1.clone(), 3, 0, None, &mut (*context).fork_sink, None);
    let mut op: ActiveValue<ARMv7MScfiaComposition> = (*context).scfia.new_bv_slice(&instruction1.clone(), 8, 4, None, &mut (*context).fork_sink, None);
    if (*context).scfia.check_condition(&(*context).scfia.new_bool_eq(&op.clone(), &(*context).scfia.new_bv_concrete(0, 5), None, false, &mut (*context).fork_sink, None), &mut (*context).fork_sink) {
        unimplemented!();
    } else if (*context).scfia.check_condition(&(*context).scfia.new_bool_eq(&op.clone(), &(*context).scfia.new_bv_concrete(0b00100, 5), None, false, &mut (*context).fork_sink, None), &mut (*context).fork_sink) {
        unimplemented!();
    } else if (*context).scfia.check_condition(&(*context).scfia.new_bool_eq(&op.clone(), &(*context).scfia.new_bv_concrete(0b01010, 5), None, false, &mut (*context).fork_sink, None), &mut (*context).fork_sink) {
        unimplemented!();
    } else if (*context).scfia.check_condition(&(*context).scfia.new_bool_eq(&op.clone(), &(*context).scfia.new_bv_concrete(0b01100, 5), None, false, &mut (*context).fork_sink, None), &mut (*context).fork_sink) {
        unimplemented!();
    } else if (*context).scfia.check_condition(&(*context).scfia.new_bool_eq(&op.clone(), &(*context).scfia.new_bv_concrete(0b10000, 5), None, false, &mut (*context).fork_sink, None), &mut (*context).fork_sink) {
        unimplemented!();
    } else if (*context).scfia.check_condition(&(*context).scfia.new_bool_eq(&op.clone(), &(*context).scfia.new_bv_concrete(0b10010, 5), None, false, &mut (*context).fork_sink, None), &mut (*context).fork_sink) {
        unimplemented!();
    } else if (*context).scfia.check_condition(&(*context).scfia.new_bool_eq(&op.clone(), &(*context).scfia.new_bv_concrete(0b10100, 5), None, false, &mut (*context).fork_sink, None), &mut (*context).fork_sink) {
        unimplemented!();
    } else if (*context).scfia.check_condition(&(*context).scfia.new_bool_eq(&op.clone(), &(*context).scfia.new_bv_concrete(0b10110, 5), None, false, &mut (*context).fork_sink, None), &mut (*context).fork_sink) {
        unimplemented!();
    } else if (*context).scfia.check_condition(&(*context).scfia.new_bool_eq(&op.clone(), &(*context).scfia.new_bv_concrete(0b11000, 5), None, false, &mut (*context).fork_sink, None), &mut (*context).fork_sink) {
        unimplemented!();
    } else if (*context).scfia.check_condition(&(*context).scfia.new_bool_eq(&op.clone(), &(*context).scfia.new_bv_concrete(0b11010, 5), None, false, &mut (*context).fork_sink, None), &mut (*context).fork_sink) {
        unimplemented!();
    } else if (*context).scfia.check_condition(&(*context).scfia.new_bool_eq(&op.clone(), &(*context).scfia.new_bv_concrete(0b11100, 5), None, false, &mut (*context).fork_sink, None), &mut (*context).fork_sink) {
        // UBFX Encoding T1
        let mut widthm1: ActiveValue<ARMv7MScfiaComposition> = (*context).scfia.new_bv_slice(&instruction2.clone(), 4, 0, None, &mut (*context).fork_sink, None);
        let mut imm2: ActiveValue<ARMv7MScfiaComposition> = (*context).scfia.new_bv_slice(&instruction2.clone(), 7, 6, None, &mut (*context).fork_sink, None);
        let mut rd: ActiveValue<ARMv7MScfiaComposition> = (*context).scfia.new_bv_slice(&instruction2.clone(), 11, 8, None, &mut (*context).fork_sink, None);
        let mut imm3: ActiveValue<ARMv7MScfiaComposition> = (*context).scfia.new_bv_slice(&instruction2.clone(), 14, 12, None, &mut (*context).fork_sink, None);
        let mut lsbit: ActiveValue<ARMv7MScfiaComposition> = (*context).scfia.new_bv_concat(&imm3.clone(), &imm2.clone(), 5, None, &mut (*context).fork_sink, None);
        let mut lsbit_extended: ActiveValue<ARMv7MScfiaComposition> = (*context).scfia.new_bv_concat(&(*context).scfia.new_bv_concrete(0, 27), &lsbit.clone(), 32, None, &mut (*context).fork_sink, None);
        let mut msbit: ActiveValue<ARMv7MScfiaComposition> = (*context).scfia.new_bv_add(&lsbit.clone(), &widthm1.clone(), 5, None, &mut (*context).fork_sink, None);
        let mut msbit_extended: ActiveValue<ARMv7MScfiaComposition> = (*context).scfia.new_bv_concat(&(*context).scfia.new_bv_concrete(0, 27), &msbit.clone(), 32, None, &mut (*context).fork_sink, None);
        let mut n: ActiveValue<ARMv7MScfiaComposition> = _register_read_BV32_wide(rn.clone(), state, context);
        if (*context).scfia.check_condition(&(*context).scfia.new_bool_unsigned_less_than(&msbit.clone(), &(*context).scfia.new_bv_concrete(32, 5), None, false, &mut (*context).fork_sink, None), &mut (*context).fork_sink) {
            let mut l_shift: ActiveValue<ARMv7MScfiaComposition> = (*context).scfia.new_bv_sub(&(*context).scfia.new_bv_concrete(31, 32), &msbit_extended.clone(), 32, None, &mut (*context).fork_sink, None);
            let mut l_shifted: ActiveValue<ARMv7MScfiaComposition> = (*context).scfia.new_bv_sll(&n.clone(), &l_shift.clone(), 32, None, &mut (*context).fork_sink, None);
            let mut r_shift: ActiveValue<ARMv7MScfiaComposition> = (*context).scfia.new_bv_add(&l_shift.clone(), &lsbit_extended.clone(), 32, None, &mut (*context).fork_sink, None);
            let mut result: ActiveValue<ARMv7MScfiaComposition> = (*context).scfia.new_bv_sll(&l_shifted.clone(), &r_shift.clone(), 32, None, &mut (*context).fork_sink, None);
            _register_write_BV32_wide(rd.clone(), result.clone(), state, context);
        } else {
            //TODO unpredictable
            unimplemented!();
        }
    } else {
        //TODO remaining ops
        //TODO undefined
        unimplemented!();
    }
}

unsafe fn _thumb32_branches_and_misc_control(mut instruction1: ActiveValue<ARMv7MScfiaComposition>, mut instruction2: ActiveValue<ARMv7MScfiaComposition>, mut state: *mut SystemState, context: *mut StepContext<ARMv7MScfiaComposition>) {
    let mut op: ActiveValue<ARMv7MScfiaComposition> = (*context).scfia.new_bv_slice(&instruction1.clone(), 10, 4, None, &mut (*context).fork_sink, None);
    let mut op1: ActiveValue<ARMv7MScfiaComposition> = (*context).scfia.new_bv_slice(&instruction2.clone(), 14, 12, None, &mut (*context).fork_sink, None);
    if (*context).scfia.check_condition(&(*context).scfia.new_bool_eq(&_matches_BV3(op1.clone(), (*context).scfia.new_bv_concrete(0b000, 3), (*context).scfia.new_bv_concrete(0b101, 3), context), &(*context).scfia.new_bv_concrete(1, 1), None, false, &mut (*context).fork_sink, None), &mut (*context).fork_sink) {
        unimplemented!();
    } else if (*context).scfia.check_condition(&(*context).scfia.new_bool_eq(&op1.clone(), &(*context).scfia.new_bv_concrete(0b010, 3), None, false, &mut (*context).fork_sink, None), &mut (*context).fork_sink) {
        unimplemented!();
    } else if (*context).scfia.check_condition(&(*context).scfia.new_bool_eq(&_matches_BV3(op1.clone(), (*context).scfia.new_bv_concrete(0b001, 3), (*context).scfia.new_bv_concrete(0b100, 3), context), &(*context).scfia.new_bv_concrete(1, 1), None, false, &mut (*context).fork_sink, None), &mut (*context).fork_sink) {
        unimplemented!();
    } else if (*context).scfia.check_condition(&(*context).scfia.new_bool_eq(&_matches_BV3(op1.clone(), (*context).scfia.new_bv_concrete(0b101, 3), (*context).scfia.new_bv_concrete(0b000, 3), context), &(*context).scfia.new_bv_concrete(1, 1), None, false, &mut (*context).fork_sink, None), &mut (*context).fork_sink) {
        // BL Encoding T1
        let mut s: ActiveValue<ARMv7MScfiaComposition> = (*context).scfia.new_bv_slice(&instruction1.clone(), 10, 10, None, &mut (*context).fork_sink, None);
        let mut j1: ActiveValue<ARMv7MScfiaComposition> = (*context).scfia.new_bv_slice(&instruction2.clone(), 13, 13, None, &mut (*context).fork_sink, None);
        let mut j2: ActiveValue<ARMv7MScfiaComposition> = (*context).scfia.new_bv_slice(&instruction2.clone(), 11, 11, None, &mut (*context).fork_sink, None);
        let mut imm10: ActiveValue<ARMv7MScfiaComposition> = (*context).scfia.new_bv_slice(&instruction1.clone(), 9, 0, None, &mut (*context).fork_sink, None);
        let mut imm11: ActiveValue<ARMv7MScfiaComposition> = (*context).scfia.new_bv_slice(&instruction2.clone(), 10, 0, None, &mut (*context).fork_sink, None);
        let mut i1: ActiveValue<ARMv7MScfiaComposition> = (*context).scfia.new_bv_not(&(*context).scfia.new_bv_xor(&j1.clone(), &s.clone(), 1, None, &mut (*context).fork_sink, None), 1, None, &mut (*context).fork_sink, None);
        let mut i2: ActiveValue<ARMv7MScfiaComposition> = (*context).scfia.new_bv_not(&(*context).scfia.new_bv_xor(&j2.clone(), &s.clone(), 1, None, &mut (*context).fork_sink, None), 1, None, &mut (*context).fork_sink, None);
        let mut imm1: ActiveValue<ARMv7MScfiaComposition> = (*context).scfia.new_bv_concat(&imm11.clone(), &(*context).scfia.new_bv_concrete(0, 1), 12, None, &mut (*context).fork_sink, None);
        let mut imm2: ActiveValue<ARMv7MScfiaComposition> = (*context).scfia.new_bv_concat(&imm10.clone(), &imm1.clone(), 22, None, &mut (*context).fork_sink, None);
        let mut imm3: ActiveValue<ARMv7MScfiaComposition> = (*context).scfia.new_bv_concat(&i2.clone(), &imm2.clone(), 23, None, &mut (*context).fork_sink, None);
        let mut imm4: ActiveValue<ARMv7MScfiaComposition> = (*context).scfia.new_bv_concat(&i1.clone(), &imm3.clone(), 24, None, &mut (*context).fork_sink, None);
        let mut imm5: ActiveValue<ARMv7MScfiaComposition> = (*context).scfia.new_bv_concat(&s.clone(), &imm4.clone(), 25, None, &mut (*context).fork_sink, None);
        let mut imm: ActiveValue<ARMv7MScfiaComposition> = (*context).scfia.new_bv_sign_extend(&imm5.clone(), 25, 32, None, &mut (*context).fork_sink, None);
        let mut lr: ActiveValue<ARMv7MScfiaComposition> = (*context).scfia.new_bv_concat(&(*context).scfia.new_bv_slice(&(*state).PC.clone(), 31, 1, None, &mut (*context).fork_sink, None), &(*context).scfia.new_bv_concrete(1, 1), 32, None, &mut (*context).fork_sink, None);
        _register_write_BV32_wide((*context).scfia.new_bv_concrete(14, 4), lr.clone(), state, context);
        _branch_write_pc((*context).scfia.new_bv_add(&(*state).PC.clone(), &imm.clone(), 32, None, &mut (*context).fork_sink, None), state, context);
    } else {
        unimplemented!();
    }
}

unsafe fn _thumb32_store_single_data_item(mut instruction1: ActiveValue<ARMv7MScfiaComposition>, mut instruction2: ActiveValue<ARMv7MScfiaComposition>, mut state: *mut SystemState, context: *mut StepContext<ARMv7MScfiaComposition>) {
    let mut op1: ActiveValue<ARMv7MScfiaComposition> = (*context).scfia.new_bv_slice(&instruction1.clone(), 7, 5, None, &mut (*context).fork_sink, None);
    let mut op2: ActiveValue<ARMv7MScfiaComposition> = (*context).scfia.new_bv_slice(&instruction2.clone(), 11, 11, None, &mut (*context).fork_sink, None);
    if (*context).scfia.check_condition(&(*context).scfia.new_bool_eq(&op1.clone(), &(*context).scfia.new_bv_concrete(0b000, 3), None, false, &mut (*context).fork_sink, None), &mut (*context).fork_sink) {
        unimplemented!();
    } else if (*context).scfia.check_condition(&(*context).scfia.new_bool_eq(&op1.clone(), &(*context).scfia.new_bv_concrete(0b001, 3), None, false, &mut (*context).fork_sink, None), &mut (*context).fork_sink) {
        unimplemented!();
    } else if (*context).scfia.check_condition(&(*context).scfia.new_bool_eq(&op1.clone(), &(*context).scfia.new_bv_concrete(0b010, 3), None, false, &mut (*context).fork_sink, None), &mut (*context).fork_sink) {
        if (*context).scfia.check_condition(&(*context).scfia.new_bool_eq(&op2.clone(), &(*context).scfia.new_bv_concrete(1, 1), None, false, &mut (*context).fork_sink, None), &mut (*context).fork_sink) {
            // STR (immediate) Encoding T4
            //TODO see strt, push?
            //TODO undefined n 1111?
            let mut rt: ActiveValue<ARMv7MScfiaComposition> = (*context).scfia.new_bv_slice(&instruction2.clone(), 15, 12, None, &mut (*context).fork_sink, None);
            let mut rn: ActiveValue<ARMv7MScfiaComposition> = (*context).scfia.new_bv_slice(&instruction1.clone(), 3, 0, None, &mut (*context).fork_sink, None);
            let mut n: ActiveValue<ARMv7MScfiaComposition> = _register_read_BV32_wide(rn.clone(), state, context);
            let mut imm8: ActiveValue<ARMv7MScfiaComposition> = (*context).scfia.new_bv_slice(&instruction2.clone(), 7, 0, None, &mut (*context).fork_sink, None);
            let mut imm32: ActiveValue<ARMv7MScfiaComposition> = (*context).scfia.new_bv_concat(&(*context).scfia.new_bv_concrete(0, 24), &imm8.clone(), 32, None, &mut (*context).fork_sink, None);
            let mut index: ActiveValue<ARMv7MScfiaComposition> = (*context).scfia.new_bv_slice(&instruction2.clone(), 10, 10, None, &mut (*context).fork_sink, None);
            let mut add: ActiveValue<ARMv7MScfiaComposition> = (*context).scfia.new_bv_slice(&instruction2.clone(), 9, 9, None, &mut (*context).fork_sink, None);
            let mut wback: ActiveValue<ARMv7MScfiaComposition> = (*context).scfia.new_bv_slice(&instruction2.clone(), 8, 8, None, &mut (*context).fork_sink, None);
            let mut offset_addr: ActiveValue<ARMv7MScfiaComposition> = (*context).scfia.new_bv_concrete(0, 32);
            if (*context).scfia.check_condition(&(*context).scfia.new_bool_eq(&add.clone(), &(*context).scfia.new_bv_concrete(1, 1), None, false, &mut (*context).fork_sink, None), &mut (*context).fork_sink) {
                offset_addr = (*context).scfia.new_bv_add(&n.clone(), &imm32.clone(), 32, None, &mut (*context).fork_sink, None);
            } else {
                offset_addr = (*context).scfia.new_bv_sub(&n.clone(), &imm32.clone(), 32, None, &mut (*context).fork_sink, None);
            }
            let mut address: ActiveValue<ARMv7MScfiaComposition> = (*context).scfia.new_bv_concrete(0, 32);
            if (*context).scfia.check_condition(&(*context).scfia.new_bool_eq(&index.clone(), &(*context).scfia.new_bv_concrete(1, 1), None, false, &mut (*context).fork_sink, None), &mut (*context).fork_sink) {
                address = offset_addr.clone();
            } else {
                address = n.clone();
            }
            (*(*context).memory).write(&address.clone(), &_register_read_BV32_wide(rt.clone(), state, context), 32, (*context).scfia.clone(), &mut (*context).hints, &mut (*context).fork_sink);
            if (*context).scfia.check_condition(&(*context).scfia.new_bool_eq(&wback.clone(), &(*context).scfia.new_bv_concrete(1, 1), None, false, &mut (*context).fork_sink, None), &mut (*context).fork_sink) {
                _register_write_BV32_wide(rn.clone(), offset_addr.clone(), state, context);
            }
        } else {
            // STR (register)
            unimplemented!();
        }
    } else if (*context).scfia.check_condition(&(*context).scfia.new_bool_eq(&op1.clone(), &(*context).scfia.new_bv_concrete(0b100, 3), None, false, &mut (*context).fork_sink, None), &mut (*context).fork_sink) {
        // STRB (immediate) Encoding T2
        let mut imm12: ActiveValue<ARMv7MScfiaComposition> = (*context).scfia.new_bv_slice(&instruction2.clone(), 11, 0, None, &mut (*context).fork_sink, None);
        let mut rt: ActiveValue<ARMv7MScfiaComposition> = (*context).scfia.new_bv_slice(&instruction2.clone(), 15, 12, None, &mut (*context).fork_sink, None);
        let mut rn: ActiveValue<ARMv7MScfiaComposition> = (*context).scfia.new_bv_slice(&instruction1.clone(), 3, 0, None, &mut (*context).fork_sink, None);
        let mut imm32: ActiveValue<ARMv7MScfiaComposition> = (*context).scfia.new_bv_concat(&(*context).scfia.new_bv_concrete(0, 20), &imm12.clone(), 32, None, &mut (*context).fork_sink, None);
        // index=true, add=true, wback=false
        //TODO if t in 13,15 unpredictable
        let mut address: ActiveValue<ARMv7MScfiaComposition> = (*context).scfia.new_bv_add(&_register_read_BV32_wide(rn.clone(), state, context), &imm32.clone(), 32, None, &mut (*context).fork_sink, None);
        (*(*context).memory).write(&address.clone(), &(*context).scfia.new_bv_slice(&_register_read_BV32_wide(rt.clone(), state, context), 7, 0, None, &mut (*context).fork_sink, None), 8, (*context).scfia.clone(), &mut (*context).hints, &mut (*context).fork_sink);
    } else if (*context).scfia.check_condition(&(*context).scfia.new_bool_eq(&op1.clone(), &(*context).scfia.new_bv_concrete(0b101, 3), None, false, &mut (*context).fork_sink, None), &mut (*context).fork_sink) {
        unimplemented!();
    } else if (*context).scfia.check_condition(&(*context).scfia.new_bool_eq(&op1.clone(), &(*context).scfia.new_bv_concrete(0b110, 3), None, false, &mut (*context).fork_sink, None), &mut (*context).fork_sink) {
        unimplemented!();
    } else {
        // Other encodings in this space are UNDEFINED
        unimplemented!();
    }
}

unsafe fn _thumb32_load_word(mut instruction1: ActiveValue<ARMv7MScfiaComposition>, mut instruction2: ActiveValue<ARMv7MScfiaComposition>, mut state: *mut SystemState, context: *mut StepContext<ARMv7MScfiaComposition>) {
    let mut op1: ActiveValue<ARMv7MScfiaComposition> = (*context).scfia.new_bv_slice(&instruction1.clone(), 8, 7, None, &mut (*context).fork_sink, None);
    let mut rn: ActiveValue<ARMv7MScfiaComposition> = (*context).scfia.new_bv_slice(&instruction1.clone(), 3, 0, None, &mut (*context).fork_sink, None);
    let mut op2: ActiveValue<ARMv7MScfiaComposition> = (*context).scfia.new_bv_slice(&instruction2.clone(), 11, 6, None, &mut (*context).fork_sink, None);
    if (*context).scfia.check_condition(&(*context).scfia.new_bool_not(&(*context).scfia.new_bool_eq(&rn.clone(), &(*context).scfia.new_bv_concrete(0b1111, 4), None, false, &mut (*context).fork_sink, None), None, false, &mut (*context).fork_sink, None), &mut (*context).fork_sink) {
        if (*context).scfia.check_condition(&(*context).scfia.new_bool_eq(&op1.clone(), &(*context).scfia.new_bv_concrete(0b01, 2), None, false, &mut (*context).fork_sink, None), &mut (*context).fork_sink) {
            // LDR (immediate)
            unimplemented!();
        } else {
            if (*context).scfia.check_condition(&(*context).scfia.new_bool_eq(&_matches_BV6(op2.clone(), (*context).scfia.new_bv_concrete(0b100100, 6), (*context).scfia.new_bv_concrete(0b000000, 6), context), &(*context).scfia.new_bv_concrete(1, 1), None, false, &mut (*context).fork_sink, None), &mut (*context).fork_sink) {
                // LDR (immediate)
                unimplemented!();
            } else if (*context).scfia.check_condition(&(*context).scfia.new_bool_eq(&_matches_BV6(op2.clone(), (*context).scfia.new_bv_concrete(0b110000, 6), (*context).scfia.new_bv_concrete(0b001100, 6), context), &(*context).scfia.new_bv_concrete(1, 1), None, false, &mut (*context).fork_sink, None), &mut (*context).fork_sink) {
                // LDR (immediate)
                unimplemented!();
            } else if (*context).scfia.check_condition(&(*context).scfia.new_bool_eq(&_matches_BV6(op2.clone(), (*context).scfia.new_bv_concrete(0b111000, 6), (*context).scfia.new_bv_concrete(0b000100, 6), context), &(*context).scfia.new_bv_concrete(1, 1), None, false, &mut (*context).fork_sink, None), &mut (*context).fork_sink) {
                // LDRT
                unimplemented!();
            } else if (*context).scfia.check_condition(&(*context).scfia.new_bool_eq(&op2.clone(), &(*context).scfia.new_bv_concrete(0b000000, 6), None, false, &mut (*context).fork_sink, None), &mut (*context).fork_sink) {
                // LDR (register) Encoding T2
                let mut rm: ActiveValue<ARMv7MScfiaComposition> = (*context).scfia.new_bv_slice(&instruction2.clone(), 3, 0, None, &mut (*context).fork_sink, None);
                let mut imm2: ActiveValue<ARMv7MScfiaComposition> = (*context).scfia.new_bv_slice(&instruction2.clone(), 5, 4, None, &mut (*context).fork_sink, None);
                let mut rt: ActiveValue<ARMv7MScfiaComposition> = (*context).scfia.new_bv_slice(&instruction2.clone(), 15, 12, None, &mut (*context).fork_sink, None);
                let mut rn: ActiveValue<ARMv7MScfiaComposition> = (*context).scfia.new_bv_slice(&instruction1.clone(), 3, 0, None, &mut (*context).fork_sink, None);
                //TODO if m in {13, 15} then unpredictable
                //TODO if t == 15 && init unpredictable
                let (mut offset, mut _carry) = _shift_c(_register_read_BV32_wide(rm.clone(), state, context), SRType::LSL, (*context).scfia.new_bv_concat(&(*context).scfia.new_bv_concrete(0, 3), &imm2.clone(), 5, None, &mut (*context).fork_sink, None), (*state).APSR.C.clone(), context);
                let mut offset_addr: ActiveValue<ARMv7MScfiaComposition> = (*context).scfia.new_bv_add(&_register_read_BV32_wide(rn.clone(), state, context), &offset.clone(), 32, None, &mut (*context).fork_sink, None);
                let mut data: ActiveValue<ARMv7MScfiaComposition> = (*(*context).memory).read(&offset_addr.clone(), 32, (*context).scfia.clone(), &mut (*context).hints, &mut (*context).fork_sink);
                if (*context).scfia.check_condition(&(*context).scfia.new_bool_eq(&rt.clone(), &(*context).scfia.new_bv_concrete(15, 4), None, false, &mut (*context).fork_sink, None), &mut (*context).fork_sink) {
                    //TODO pc magic or unpredictable
                    unimplemented!();
                } else {
                    _register_write_BV32_wide(rt.clone(), data.clone(), state, context);
                }
            } else {
                // undefined
                unimplemented!();
            }
        }
    } else {
        unimplemented!();
    }
}

unsafe fn _thumb32_load_store_dual_or_exclusive_table_branch(mut instruction1: ActiveValue<ARMv7MScfiaComposition>, mut instruction2: ActiveValue<ARMv7MScfiaComposition>, mut state: *mut SystemState, context: *mut StepContext<ARMv7MScfiaComposition>) {
    let mut op1: ActiveValue<ARMv7MScfiaComposition> = (*context).scfia.new_bv_slice(&instruction1.clone(), 8, 7, None, &mut (*context).fork_sink, None);
    let mut op2: ActiveValue<ARMv7MScfiaComposition> = (*context).scfia.new_bv_slice(&instruction1.clone(), 5, 4, None, &mut (*context).fork_sink, None);
    let mut rn: ActiveValue<ARMv7MScfiaComposition> = (*context).scfia.new_bv_slice(&instruction1.clone(), 3, 0, None, &mut (*context).fork_sink, None);
    let mut op3: ActiveValue<ARMv7MScfiaComposition> = (*context).scfia.new_bv_slice(&instruction2.clone(), 7, 4, None, &mut (*context).fork_sink, None);
    unimplemented!();
}

unsafe fn _thumb16(mut instruction: ActiveValue<ARMv7MScfiaComposition>, mut state: *mut SystemState, context: *mut StepContext<ARMv7MScfiaComposition>) {
    let mut opcode: ActiveValue<ARMv7MScfiaComposition> = (*context).scfia.new_bv_slice(&instruction.clone(), 15, 10, None, &mut (*context).fork_sink, None);
    if (*context).scfia.check_condition(&(*context).scfia.new_bool_unsigned_less_than(&opcode.clone(), &(*context).scfia.new_bv_concrete(0b010000, 6), None, false, &mut (*context).fork_sink, None), &mut (*context).fork_sink) {
        _thumb16_basic(instruction.clone(), state, context);
    } else if (*context).scfia.check_condition(&(*context).scfia.new_bool_eq(&opcode.clone(), &(*context).scfia.new_bv_concrete(0b010000, 6), None, false, &mut (*context).fork_sink, None), &mut (*context).fork_sink) {
        // Data processing
        _thumb16_data_processing(instruction.clone(), state, context);
    } else if (*context).scfia.check_condition(&(*context).scfia.new_bool_eq(&opcode.clone(), &(*context).scfia.new_bv_concrete(0b010001, 6), None, false, &mut (*context).fork_sink, None), &mut (*context).fork_sink) {
        // Special data instructions and branch and exchange
        _thumb16_special_data_instructions_and_branch_and_merge(instruction.clone(), state, context);
    } else if (*context).scfia.check_condition(&(*context).scfia.new_bool_unsigned_less_than(&opcode.clone(), &(*context).scfia.new_bv_concrete(0b010100, 6), None, false, &mut (*context).fork_sink, None), &mut (*context).fork_sink) {
        // Load from literal pool
        _thumb16_load_from_literal_pool(instruction.clone(), state, context);
    } else if (*context).scfia.check_condition(&(*context).scfia.new_bool_unsigned_less_than(&opcode.clone(), &(*context).scfia.new_bv_concrete(0b101000, 6), None, false, &mut (*context).fork_sink, None), &mut (*context).fork_sink) {
        // Load/store single data item
        _thumb16_load_store_single_data_item(instruction.clone(), state, context);
    } else if (*context).scfia.check_condition(&(*context).scfia.new_bool_unsigned_less_than(&opcode.clone(), &(*context).scfia.new_bv_concrete(0b101010, 6), None, false, &mut (*context).fork_sink, None), &mut (*context).fork_sink) {
        // Generate PC-relative address
        unimplemented!();
    } else if (*context).scfia.check_condition(&(*context).scfia.new_bool_unsigned_less_than(&opcode.clone(), &(*context).scfia.new_bv_concrete(0b101100, 6), None, false, &mut (*context).fork_sink, None), &mut (*context).fork_sink) {
        // Generate SP-relative address
        unimplemented!();
    } else if (*context).scfia.check_condition(&(*context).scfia.new_bool_unsigned_less_than(&opcode.clone(), &(*context).scfia.new_bv_concrete(0b110000, 6), None, false, &mut (*context).fork_sink, None), &mut (*context).fork_sink) {
        // Misc 16-bit instructions
        _thumb16_misc_16_bit_instruction(instruction.clone(), state, context);
    } else if (*context).scfia.check_condition(&(*context).scfia.new_bool_unsigned_less_than(&opcode.clone(), &(*context).scfia.new_bv_concrete(0b110010, 6), None, false, &mut (*context).fork_sink, None), &mut (*context).fork_sink) {
        // Store multiple registers
        unimplemented!();
    } else if (*context).scfia.check_condition(&(*context).scfia.new_bool_unsigned_less_than(&opcode.clone(), &(*context).scfia.new_bv_concrete(0b110100, 6), None, false, &mut (*context).fork_sink, None), &mut (*context).fork_sink) {
        // Load multiple registers
        unimplemented!();
    } else if (*context).scfia.check_condition(&(*context).scfia.new_bool_unsigned_less_than(&opcode.clone(), &(*context).scfia.new_bv_concrete(0b111000, 6), None, false, &mut (*context).fork_sink, None), &mut (*context).fork_sink) {
        // Conditional branch and supervisor call
        _thumb16_conditional_branch_and_svc(instruction.clone(), state, context);
    } else if (*context).scfia.check_condition(&(*context).scfia.new_bool_unsigned_less_than(&opcode.clone(), &(*context).scfia.new_bv_concrete(0b111010, 6), None, false, &mut (*context).fork_sink, None), &mut (*context).fork_sink) {
        // Unconditional branch
        _thumb16_unconditional_branch(instruction.clone(), state, context);
    } else {
        unimplemented!();
    }
}

unsafe fn _thumb16_basic(mut instruction: ActiveValue<ARMv7MScfiaComposition>, mut state: *mut SystemState, context: *mut StepContext<ARMv7MScfiaComposition>) {
    let mut opcode: ActiveValue<ARMv7MScfiaComposition> = (*context).scfia.new_bv_slice(&instruction.clone(), 13, 9, None, &mut (*context).fork_sink, None);
    if (*context).scfia.check_condition(&(*context).scfia.new_bool_unsigned_less_than(&opcode.clone(), &(*context).scfia.new_bv_concrete(0b00100, 5), None, false, &mut (*context).fork_sink, None), &mut (*context).fork_sink) {
        // LSL (immediate) Encoding T1
        let mut rd: ActiveValue<ARMv7MScfiaComposition> = (*context).scfia.new_bv_slice(&instruction.clone(), 2, 0, None, &mut (*context).fork_sink, None);
        let mut rm: ActiveValue<ARMv7MScfiaComposition> = (*context).scfia.new_bv_slice(&instruction.clone(), 5, 3, None, &mut (*context).fork_sink, None);
        let mut imm5: ActiveValue<ARMv7MScfiaComposition> = (*context).scfia.new_bv_slice(&instruction.clone(), 10, 6, None, &mut (*context).fork_sink, None);
        let mut setflags: ActiveValue<ARMv7MScfiaComposition> = (*context).scfia.new_bv_concrete(1, 1);
        //TODO !initblock
        let (mut _srtype, mut shift_n) = _decode_imm_shift((*context).scfia.new_bv_concrete(0b00, 2), imm5.clone(), context);
        let (mut result, mut carry) = _shift_c(_register_read_BV32(rm.clone(), state, context), SRType::LSL, shift_n.clone(), (*state).APSR.C.clone(), context);
        if (*context).scfia.check_condition(&(*context).scfia.new_bool_eq(&setflags.clone(), &(*context).scfia.new_bv_concrete(1, 1), None, false, &mut (*context).fork_sink, None), &mut (*context).fork_sink) {
            (*state).APSR.N = (*context).scfia.new_bv_slice(&result.clone(), 31, 31, None, &mut (*context).fork_sink, None);
            (*state).APSR.Z = _is_zero_bit_BV32(result.clone(), context);
            (*state).APSR.C = carry.clone();
        }
    } else if (*context).scfia.check_condition(&(*context).scfia.new_bool_unsigned_less_than(&opcode.clone(), &(*context).scfia.new_bv_concrete(0b01000, 5), None, false, &mut (*context).fork_sink, None), &mut (*context).fork_sink) {
        // LSR (immediate) Encoding T1
        let mut rd: ActiveValue<ARMv7MScfiaComposition> = (*context).scfia.new_bv_slice(&instruction.clone(), 2, 0, None, &mut (*context).fork_sink, None);
        let mut rm: ActiveValue<ARMv7MScfiaComposition> = (*context).scfia.new_bv_slice(&instruction.clone(), 5, 3, None, &mut (*context).fork_sink, None);
        let mut imm5: ActiveValue<ARMv7MScfiaComposition> = (*context).scfia.new_bv_slice(&instruction.clone(), 10, 6, None, &mut (*context).fork_sink, None);
        let mut setflags: ActiveValue<ARMv7MScfiaComposition> = (*context).scfia.new_bv_concrete(1, 1);
        //TODO !initblock
        let (mut srtype, mut shift_n) = _decode_imm_shift((*context).scfia.new_bv_concrete(0b01, 2), imm5.clone(), context);
        let (mut result, mut carry) = _shift_c(_register_read_BV32(rm.clone(), state, context), SRType::LSR, shift_n.clone(), (*state).APSR.C.clone(), context);
        _register_write_BV32(rd.clone(), result.clone(), state, context);
        if (*context).scfia.check_condition(&(*context).scfia.new_bool_eq(&setflags.clone(), &(*context).scfia.new_bv_concrete(1, 1), None, false, &mut (*context).fork_sink, None), &mut (*context).fork_sink) {
            (*state).APSR.N = (*context).scfia.new_bv_slice(&result.clone(), 31, 31, None, &mut (*context).fork_sink, None);
            (*state).APSR.Z = _is_zero_bit_BV32(result.clone(), context);
            (*state).APSR.C = carry.clone();
        }
    } else if (*context).scfia.check_condition(&(*context).scfia.new_bool_unsigned_less_than(&opcode.clone(), &(*context).scfia.new_bv_concrete(0b01100, 5), None, false, &mut (*context).fork_sink, None), &mut (*context).fork_sink) {
        // ASR (immediate) Encoding T1
        let mut rd: ActiveValue<ARMv7MScfiaComposition> = (*context).scfia.new_bv_slice(&instruction.clone(), 2, 0, None, &mut (*context).fork_sink, None);
        let mut rm: ActiveValue<ARMv7MScfiaComposition> = (*context).scfia.new_bv_slice(&instruction.clone(), 5, 3, None, &mut (*context).fork_sink, None);
        let mut imm5: ActiveValue<ARMv7MScfiaComposition> = (*context).scfia.new_bv_slice(&instruction.clone(), 10, 6, None, &mut (*context).fork_sink, None);
        let (mut sr_type, mut shift_n) = _decode_imm_shift((*context).scfia.new_bv_concrete(0b10, 2), imm5.clone(), context);
        let mut setflags: ActiveValue<ARMv7MScfiaComposition> = (*context).scfia.new_bv_concrete(1, 1);
        //TODO setflags = !initblock
        let (mut result, mut carry) = _shift_c(_register_read_BV32(rm.clone(), state, context), SRType::ASR, shift_n.clone(), (*state).APSR.C.clone(), context);
        if (*context).scfia.check_condition(&(*context).scfia.new_bool_eq(&setflags.clone(), &(*context).scfia.new_bv_concrete(1, 1), None, false, &mut (*context).fork_sink, None), &mut (*context).fork_sink) {
            (*state).APSR.N = (*context).scfia.new_bv_slice(&result.clone(), 31, 31, None, &mut (*context).fork_sink, None);
            (*state).APSR.Z = _is_zero_bit_BV32(result.clone(), context);
            (*state).APSR.C = carry.clone();
        }
    } else if (*context).scfia.check_condition(&(*context).scfia.new_bool_eq(&opcode.clone(), &(*context).scfia.new_bv_concrete(0b01100, 5), None, false, &mut (*context).fork_sink, None), &mut (*context).fork_sink) {
        // ADD (register)
        //TODO use addwithcarry instead?
        let mut rd: ActiveValue<ARMv7MScfiaComposition> = (*context).scfia.new_bv_slice(&instruction.clone(), 2, 0, None, &mut (*context).fork_sink, None);
        let mut rn: ActiveValue<ARMv7MScfiaComposition> = (*context).scfia.new_bv_slice(&instruction.clone(), 5, 3, None, &mut (*context).fork_sink, None);
        let mut rm: ActiveValue<ARMv7MScfiaComposition> = (*context).scfia.new_bv_slice(&instruction.clone(), 8, 6, None, &mut (*context).fork_sink, None);
        let mut setflags: ActiveValue<ARMv7MScfiaComposition> = (*context).scfia.new_bv_concrete(1, 1);
        //TODO !InITBlock
        let mut result: ActiveValue<ARMv7MScfiaComposition> = (*context).scfia.new_bv_add(&_register_read_BV32(rn.clone(), state, context), &_register_read_BV32(rm.clone(), state, context), 32, None, &mut (*context).fork_sink, None);
        let mut carry: ActiveValue<ARMv7MScfiaComposition> = (*context).scfia.new_bv_concrete(0, 1);
        //TODO
        let mut overflow: ActiveValue<ARMv7MScfiaComposition> = (*context).scfia.new_bv_concrete(0, 1);
        //TODO
        _register_write_BV32(rd.clone(), result.clone(), state, context);
        if (*context).scfia.check_condition(&(*context).scfia.new_bool_eq(&setflags.clone(), &(*context).scfia.new_bv_concrete(1, 1), None, false, &mut (*context).fork_sink, None), &mut (*context).fork_sink) {
            (*state).APSR.N = (*context).scfia.new_bv_slice(&result.clone(), 31, 31, None, &mut (*context).fork_sink, None);
            (*state).APSR.Z = _is_zero_bit_BV32(result.clone(), context);
            (*state).APSR.C = carry.clone();
            (*state).APSR.V = overflow.clone();
        }
    } else if (*context).scfia.check_condition(&(*context).scfia.new_bool_eq(&opcode.clone(), &(*context).scfia.new_bv_concrete(0b01101, 5), None, false, &mut (*context).fork_sink, None), &mut (*context).fork_sink) {
        // SUB Encoding T?
        let mut rd: ActiveValue<ARMv7MScfiaComposition> = (*context).scfia.new_bv_slice(&instruction.clone(), 2, 0, None, &mut (*context).fork_sink, None);
        let mut rn: ActiveValue<ARMv7MScfiaComposition> = (*context).scfia.new_bv_slice(&instruction.clone(), 5, 3, None, &mut (*context).fork_sink, None);
        let mut rm: ActiveValue<ARMv7MScfiaComposition> = (*context).scfia.new_bv_slice(&instruction.clone(), 8, 6, None, &mut (*context).fork_sink, None);
        let mut setflags: ActiveValue<ARMv7MScfiaComposition> = (*context).scfia.new_bv_concrete(1, 1);
        //TODO !InITBlock
        let (mut result, mut carry, mut overflow) = _add_with_carry_BV32(_register_read_BV32(rn.clone(), state, context), (*context).scfia.new_bv_not(&_register_read_BV32(rm.clone(), state, context), 32, None, &mut (*context).fork_sink, None), (*context).scfia.new_bv_concrete(1, 1), context);
        _register_write_BV32(rd.clone(), result.clone(), state, context);
        if (*context).scfia.check_condition(&(*context).scfia.new_bool_eq(&setflags.clone(), &(*context).scfia.new_bv_concrete(1, 1), None, false, &mut (*context).fork_sink, None), &mut (*context).fork_sink) {
            (*state).APSR.N = (*context).scfia.new_bv_slice(&result.clone(), 31, 31, None, &mut (*context).fork_sink, None);
            (*state).APSR.Z = _is_zero_bit_BV32(result.clone(), context);
            (*state).APSR.C = carry.clone();
            (*state).APSR.V = overflow.clone();
        }
    } else if (*context).scfia.check_condition(&(*context).scfia.new_bool_eq(&opcode.clone(), &(*context).scfia.new_bv_concrete(0b01110, 5), None, false, &mut (*context).fork_sink, None), &mut (*context).fork_sink) {
        // Add 3-bit immediate Encoding T1
        let mut rd: ActiveValue<ARMv7MScfiaComposition> = (*context).scfia.new_bv_slice(&instruction.clone(), 2, 0, None, &mut (*context).fork_sink, None);
        let mut rn: ActiveValue<ARMv7MScfiaComposition> = (*context).scfia.new_bv_slice(&instruction.clone(), 5, 3, None, &mut (*context).fork_sink, None);
        let mut imm3: ActiveValue<ARMv7MScfiaComposition> = (*context).scfia.new_bv_slice(&instruction.clone(), 8, 6, None, &mut (*context).fork_sink, None);
        let mut setflags: ActiveValue<ARMv7MScfiaComposition> = (*context).scfia.new_bv_concrete(1, 1);
        //TODO !initblock
        let mut imm32: ActiveValue<ARMv7MScfiaComposition> = (*context).scfia.new_bv_concat(&(*context).scfia.new_bv_concrete(0, 29), &imm3.clone(), 32, None, &mut (*context).fork_sink, None);
        let (mut result, mut carry, mut overflow) = _add_with_carry_BV32(_register_read_BV32(rn.clone(), state, context), imm32.clone(), (*context).scfia.new_bv_concrete(0, 1), context);
        _register_write_BV32(rd.clone(), result.clone(), state, context);
        if (*context).scfia.check_condition(&(*context).scfia.new_bool_eq(&setflags.clone(), &(*context).scfia.new_bv_concrete(1, 1), None, false, &mut (*context).fork_sink, None), &mut (*context).fork_sink) {
            (*state).APSR.N = (*context).scfia.new_bv_slice(&result.clone(), 31, 31, None, &mut (*context).fork_sink, None);
            (*state).APSR.Z = _is_zero_bit_BV32(result.clone(), context);
            (*state).APSR.C = carry.clone();
            (*state).APSR.V = overflow.clone();
        }
    } else if (*context).scfia.check_condition(&(*context).scfia.new_bool_eq(&opcode.clone(), &(*context).scfia.new_bv_concrete(0b01111, 5), None, false, &mut (*context).fork_sink, None), &mut (*context).fork_sink) {
        // Subtract 3-bit immediate
        unimplemented!();
    } else if (*context).scfia.check_condition(&(*context).scfia.new_bool_unsigned_less_than(&opcode.clone(), &(*context).scfia.new_bv_concrete(0b10100, 5), None, false, &mut (*context).fork_sink, None), &mut (*context).fork_sink) {
        // MOV T1
        //TODO if ConditionPassed()?
        let mut rd: ActiveValue<ARMv7MScfiaComposition> = (*context).scfia.new_bv_slice(&instruction.clone(), 10, 8, None, &mut (*context).fork_sink, None);
        let mut imm8: ActiveValue<ARMv7MScfiaComposition> = (*context).scfia.new_bv_slice(&instruction.clone(), 7, 0, None, &mut (*context).fork_sink, None);
        let mut imm9: ActiveValue<ARMv7MScfiaComposition> = (*context).scfia.new_bv_concat(&imm8.clone(), &(*context).scfia.new_bv_concrete(0, 1), 9, None, &mut (*context).fork_sink, None);
        let mut imm32: ActiveValue<ARMv7MScfiaComposition> = (*context).scfia.new_bv_concat(&(*context).scfia.new_bv_concrete(0, 23), &imm9.clone(), 32, None, &mut (*context).fork_sink, None);
        let mut setflags: ActiveValue<ARMv7MScfiaComposition> = (*context).scfia.new_bv_concrete(1, 1);
        let mut carry: ActiveValue<ARMv7MScfiaComposition> = (*state).APSR.C.clone();
        _mov_immediate(rd.clone(), setflags.clone(), imm32.clone(), carry.clone(), state, context);
    } else if (*context).scfia.check_condition(&(*context).scfia.new_bool_unsigned_less_than(&opcode.clone(), &(*context).scfia.new_bv_concrete(0b11000, 5), None, false, &mut (*context).fork_sink, None), &mut (*context).fork_sink) {
        // CMP (immediate) Encoding T1
        let mut imm8: ActiveValue<ARMv7MScfiaComposition> = (*context).scfia.new_bv_slice(&instruction.clone(), 7, 0, None, &mut (*context).fork_sink, None);
        let mut rn: ActiveValue<ARMv7MScfiaComposition> = (*context).scfia.new_bv_slice(&instruction.clone(), 10, 8, None, &mut (*context).fork_sink, None);
        let mut imm32: ActiveValue<ARMv7MScfiaComposition> = (*context).scfia.new_bv_concat(&(*context).scfia.new_bv_concrete(0, 24), &imm8.clone(), 32, None, &mut (*context).fork_sink, None);
        //TODO if n == 15 unpredictable
        let (mut result, mut carry, mut overflow) = _add_with_carry_BV32(_register_read_BV32(rn.clone(), state, context), (*context).scfia.new_bv_not(&imm32.clone(), 32, None, &mut (*context).fork_sink, None), (*context).scfia.new_bv_concrete(1, 1), context);
        (*state).APSR.N = (*context).scfia.new_bv_slice(&result.clone(), 31, 31, None, &mut (*context).fork_sink, None);
        (*state).APSR.Z = _is_zero_bit_BV32(result.clone(), context);
        (*state).APSR.C = carry.clone();
        (*state).APSR.V = overflow.clone();
    } else if (*context).scfia.check_condition(&(*context).scfia.new_bool_unsigned_less_than(&opcode.clone(), &(*context).scfia.new_bv_concrete(0b11100, 5), None, false, &mut (*context).fork_sink, None), &mut (*context).fork_sink) {
        // ADD (8-bit immediate) Encoding T2
        let mut rdn: ActiveValue<ARMv7MScfiaComposition> = (*context).scfia.new_bv_slice(&instruction.clone(), 10, 8, None, &mut (*context).fork_sink, None);
        let mut setflags: ActiveValue<ARMv7MScfiaComposition> = (*context).scfia.new_bv_concrete(1, 1);
        //TODO !InITBlock
        let mut imm8: ActiveValue<ARMv7MScfiaComposition> = (*context).scfia.new_bv_slice(&instruction.clone(), 7, 0, None, &mut (*context).fork_sink, None);
        let mut imm32: ActiveValue<ARMv7MScfiaComposition> = (*context).scfia.new_bv_concat(&(*context).scfia.new_bv_concrete(0, 24), &imm8.clone(), 32, None, &mut (*context).fork_sink, None);
        let (mut result, mut carry, mut overflow) = _add_with_carry_BV32(_register_read_BV32(rdn.clone(), state, context), imm32.clone(), (*context).scfia.new_bv_concrete(0, 1), context);
        _register_write_BV32(rdn.clone(), result.clone(), state, context);
        if (*context).scfia.check_condition(&(*context).scfia.new_bool_eq(&setflags.clone(), &(*context).scfia.new_bv_concrete(1, 1), None, false, &mut (*context).fork_sink, None), &mut (*context).fork_sink) {
            (*state).APSR.N = (*context).scfia.new_bv_slice(&result.clone(), 31, 31, None, &mut (*context).fork_sink, None);
            (*state).APSR.Z = _is_zero_bit_BV32(result.clone(), context);
            (*state).APSR.C = carry.clone();
            (*state).APSR.V = overflow.clone();
        }
    } else {
        // SUB (8-bit immediate) Encoding T2
        let mut imm8: ActiveValue<ARMv7MScfiaComposition> = (*context).scfia.new_bv_slice(&instruction.clone(), 7, 0, None, &mut (*context).fork_sink, None);
        let mut rdn: ActiveValue<ARMv7MScfiaComposition> = (*context).scfia.new_bv_slice(&instruction.clone(), 10, 8, None, &mut (*context).fork_sink, None);
        let mut setflags: ActiveValue<ARMv7MScfiaComposition> = (*context).scfia.new_bv_concrete(1, 1);
        //TODO !initblock
        let mut imm32: ActiveValue<ARMv7MScfiaComposition> = (*context).scfia.new_bv_concat(&(*context).scfia.new_bv_concrete(0, 24), &imm8.clone(), 32, None, &mut (*context).fork_sink, None);
        let (mut result, mut carry, mut overflow) = _add_with_carry_BV32(_register_read_BV32(rdn.clone(), state, context), (*context).scfia.new_bv_not(&imm32.clone(), 32, None, &mut (*context).fork_sink, None), (*context).scfia.new_bv_concrete(1, 1), context);
        _register_write_BV32(rdn.clone(), result.clone(), state, context);
        if (*context).scfia.check_condition(&(*context).scfia.new_bool_eq(&setflags.clone(), &(*context).scfia.new_bv_concrete(1, 1), None, false, &mut (*context).fork_sink, None), &mut (*context).fork_sink) {
            (*state).APSR.N = (*context).scfia.new_bv_slice(&result.clone(), 31, 31, None, &mut (*context).fork_sink, None);
            (*state).APSR.Z = _is_zero_bit_BV32(result.clone(), context);
            (*state).APSR.C = carry.clone();
            (*state).APSR.V = overflow.clone();
        }
    }
}

unsafe fn _thumb16_data_processing(mut instruction: ActiveValue<ARMv7MScfiaComposition>, mut state: *mut SystemState, context: *mut StepContext<ARMv7MScfiaComposition>) {
    let mut opcode: ActiveValue<ARMv7MScfiaComposition> = (*context).scfia.new_bv_slice(&instruction.clone(), 9, 6, None, &mut (*context).fork_sink, None);
    if (*context).scfia.check_condition(&(*context).scfia.new_bool_eq(&opcode.clone(), &(*context).scfia.new_bv_concrete(0b000, 4), None, false, &mut (*context).fork_sink, None), &mut (*context).fork_sink) {
        // AND (register) Encoding T1
        let mut rdn: ActiveValue<ARMv7MScfiaComposition> = (*context).scfia.new_bv_slice(&instruction.clone(), 2, 0, None, &mut (*context).fork_sink, None);
        let mut rm: ActiveValue<ARMv7MScfiaComposition> = (*context).scfia.new_bv_slice(&instruction.clone(), 5, 3, None, &mut (*context).fork_sink, None);
        let mut setflags: ActiveValue<ARMv7MScfiaComposition> = (*context).scfia.new_bv_concrete(1, 1);
        //TODO !InITBlock
        let mut result: ActiveValue<ARMv7MScfiaComposition> = (*context).scfia.new_bv_and(&_register_read_BV32(rdn.clone(), state, context), &_register_read_BV32(rm.clone(), state, context), 32, None, &mut (*context).fork_sink, None);
        _register_write_BV32(rdn.clone(), result.clone(), state, context);
        if (*context).scfia.check_condition(&(*context).scfia.new_bool_eq(&setflags.clone(), &(*context).scfia.new_bv_concrete(1, 1), None, false, &mut (*context).fork_sink, None), &mut (*context).fork_sink) {
            (*state).APSR.N = (*context).scfia.new_bv_slice(&result.clone(), 31, 31, None, &mut (*context).fork_sink, None);
            (*state).APSR.Z = _is_zero_bit_BV32(result.clone(), context);
        }
    } else if (*context).scfia.check_condition(&(*context).scfia.new_bool_eq(&opcode.clone(), &(*context).scfia.new_bv_concrete(0b0000, 4), None, false, &mut (*context).fork_sink, None), &mut (*context).fork_sink) {
        // EOR
        unimplemented!();
    } else if (*context).scfia.check_condition(&(*context).scfia.new_bool_eq(&opcode.clone(), &(*context).scfia.new_bv_concrete(0b0001, 4), None, false, &mut (*context).fork_sink, None), &mut (*context).fork_sink) {
        // LSL
        unimplemented!();
    } else if (*context).scfia.check_condition(&(*context).scfia.new_bool_eq(&opcode.clone(), &(*context).scfia.new_bv_concrete(0b0010, 4), None, false, &mut (*context).fork_sink, None), &mut (*context).fork_sink) {
        // LSR
        unimplemented!();
    } else if (*context).scfia.check_condition(&(*context).scfia.new_bool_eq(&opcode.clone(), &(*context).scfia.new_bv_concrete(0b0011, 4), None, false, &mut (*context).fork_sink, None), &mut (*context).fork_sink) {
        // ASR
        unimplemented!();
    } else if (*context).scfia.check_condition(&(*context).scfia.new_bool_eq(&opcode.clone(), &(*context).scfia.new_bv_concrete(0b0101, 4), None, false, &mut (*context).fork_sink, None), &mut (*context).fork_sink) {
        // ADC
        unimplemented!();
    } else if (*context).scfia.check_condition(&(*context).scfia.new_bool_eq(&opcode.clone(), &(*context).scfia.new_bv_concrete(0b0110, 4), None, false, &mut (*context).fork_sink, None), &mut (*context).fork_sink) {
        // SBC
        unimplemented!();
    } else if (*context).scfia.check_condition(&(*context).scfia.new_bool_eq(&opcode.clone(), &(*context).scfia.new_bv_concrete(0b0111, 4), None, false, &mut (*context).fork_sink, None), &mut (*context).fork_sink) {
        // ROR
        unimplemented!();
    } else if (*context).scfia.check_condition(&(*context).scfia.new_bool_eq(&opcode.clone(), &(*context).scfia.new_bv_concrete(0b1000, 4), None, false, &mut (*context).fork_sink, None), &mut (*context).fork_sink) {
        // TST Encoding T1
        unimplemented!();
    } else if (*context).scfia.check_condition(&(*context).scfia.new_bool_eq(&opcode.clone(), &(*context).scfia.new_bv_concrete(0b1001, 4), None, false, &mut (*context).fork_sink, None), &mut (*context).fork_sink) {
        // RSB
        unimplemented!();
    } else if (*context).scfia.check_condition(&(*context).scfia.new_bool_eq(&opcode.clone(), &(*context).scfia.new_bv_concrete(0b1010, 4), None, false, &mut (*context).fork_sink, None), &mut (*context).fork_sink) {
        let mut rn: ActiveValue<ARMv7MScfiaComposition> = (*context).scfia.new_bv_slice(&instruction.clone(), 2, 0, None, &mut (*context).fork_sink, None);
        let mut rm: ActiveValue<ARMv7MScfiaComposition> = (*context).scfia.new_bv_slice(&instruction.clone(), 5, 3, None, &mut (*context).fork_sink, None);
        let (mut result, mut carry, mut overflow) = _add_with_carry_BV32(_register_read_BV32(rn.clone(), state, context), (*context).scfia.new_bv_not(&_register_read_BV32(rm.clone(), state, context), 32, None, &mut (*context).fork_sink, None), (*context).scfia.new_bv_concrete(1, 1), context);
        (*state).APSR.N = (*context).scfia.new_bv_slice(&result.clone(), 31, 31, None, &mut (*context).fork_sink, None);
        (*state).APSR.Z = _is_zero_bit_BV32(result.clone(), context);
        (*state).APSR.C = carry.clone();
        (*state).APSR.V = overflow.clone();
    } else if (*context).scfia.check_condition(&(*context).scfia.new_bool_eq(&opcode.clone(), &(*context).scfia.new_bv_concrete(0b1011, 4), None, false, &mut (*context).fork_sink, None), &mut (*context).fork_sink) {
        // CMN
        unimplemented!();
    } else if (*context).scfia.check_condition(&(*context).scfia.new_bool_eq(&opcode.clone(), &(*context).scfia.new_bv_concrete(0b1100, 4), None, false, &mut (*context).fork_sink, None), &mut (*context).fork_sink) {
        // ORR (register) Encoding T1
        let mut rdn: ActiveValue<ARMv7MScfiaComposition> = (*context).scfia.new_bv_slice(&instruction.clone(), 2, 0, None, &mut (*context).fork_sink, None);
        let mut rm: ActiveValue<ARMv7MScfiaComposition> = (*context).scfia.new_bv_slice(&instruction.clone(), 5, 3, None, &mut (*context).fork_sink, None);
        let mut setflags: ActiveValue<ARMv7MScfiaComposition> = (*context).scfia.new_bv_concrete(1, 1);
        //TODO !initblock
        let mut result: ActiveValue<ARMv7MScfiaComposition> = (*context).scfia.new_bv_or(&_register_read_BV32(rdn.clone(), state, context), &_register_read_BV32(rm.clone(), state, context), 32, None, &mut (*context).fork_sink, None);
        _register_write_BV32(rdn.clone(), result.clone(), state, context);
        if (*context).scfia.check_condition(&(*context).scfia.new_bool_eq(&setflags.clone(), &(*context).scfia.new_bv_concrete(1, 1), None, false, &mut (*context).fork_sink, None), &mut (*context).fork_sink) {
            (*state).APSR.N = (*context).scfia.new_bv_slice(&result.clone(), 31, 31, None, &mut (*context).fork_sink, None);
            (*state).APSR.Z = _is_zero_bit_BV32(result.clone(), context);
        }
    } else if (*context).scfia.check_condition(&(*context).scfia.new_bool_eq(&opcode.clone(), &(*context).scfia.new_bv_concrete(0b1101, 4), None, false, &mut (*context).fork_sink, None), &mut (*context).fork_sink) {
        // MUL
        unimplemented!();
    } else if (*context).scfia.check_condition(&(*context).scfia.new_bool_eq(&opcode.clone(), &(*context).scfia.new_bv_concrete(0b1110, 4), None, false, &mut (*context).fork_sink, None), &mut (*context).fork_sink) {
        // BIC
        unimplemented!();
    } else if (*context).scfia.check_condition(&(*context).scfia.new_bool_eq(&opcode.clone(), &(*context).scfia.new_bv_concrete(0b1111, 4), None, false, &mut (*context).fork_sink, None), &mut (*context).fork_sink) {
        // MVN
        unimplemented!();
    }
}

unsafe fn _thumb16_special_data_instructions_and_branch_and_merge(mut instruction: ActiveValue<ARMv7MScfiaComposition>, mut state: *mut SystemState, context: *mut StepContext<ARMv7MScfiaComposition>) {
    let mut opcode: ActiveValue<ARMv7MScfiaComposition> = (*context).scfia.new_bv_slice(&instruction.clone(), 9, 6, None, &mut (*context).fork_sink, None);
    if (*context).scfia.check_condition(&(*context).scfia.new_bool_eq(&_matches_BV4(opcode.clone(), (*context).scfia.new_bv_concrete(0b0000, 4), (*context).scfia.new_bv_concrete(0b1100, 4), context), &(*context).scfia.new_bv_concrete(1, 1), None, false, &mut (*context).fork_sink, None), &mut (*context).fork_sink) {
        // ADD (register)
        unimplemented!();
    } else if (*context).scfia.check_condition(&(*context).scfia.new_bool_eq(&opcode.clone(), &(*context).scfia.new_bv_concrete(0b0100, 4), None, false, &mut (*context).fork_sink, None), &mut (*context).fork_sink) {
        // unpredictable
        unimplemented!();
    } else if (*context).scfia.check_condition(&(*context).scfia.new_bool_unsigned_less_than(&opcode.clone(), &(*context).scfia.new_bv_concrete(0b1000, 4), None, false, &mut (*context).fork_sink, None), &mut (*context).fork_sink) {
        // CMP (register)
        unimplemented!();
    } else if (*context).scfia.check_condition(&(*context).scfia.new_bool_eq(&_matches_BV4(opcode.clone(), (*context).scfia.new_bv_concrete(0b1000, 4), (*context).scfia.new_bv_concrete(0b0100, 4), context), &(*context).scfia.new_bv_concrete(1, 1), None, false, &mut (*context).fork_sink, None), &mut (*context).fork_sink) {
        // MOV (register) Encoding T1
        let mut rd: ActiveValue<ARMv7MScfiaComposition> = (*context).scfia.new_bv_slice(&instruction.clone(), 2, 0, None, &mut (*context).fork_sink, None);
        let mut rm: ActiveValue<ARMv7MScfiaComposition> = (*context).scfia.new_bv_slice(&instruction.clone(), 6, 3, None, &mut (*context).fork_sink, None);
        let mut D: ActiveValue<ARMv7MScfiaComposition> = (*context).scfia.new_bv_slice(&instruction.clone(), 7, 7, None, &mut (*context).fork_sink, None);
        let mut d: ActiveValue<ARMv7MScfiaComposition> = (*context).scfia.new_bv_concat(&D.clone(), &rd.clone(), 4, None, &mut (*context).fork_sink, None);
        let mut setflags: ActiveValue<ARMv7MScfiaComposition> = (*context).scfia.new_bv_concrete(0, 1);
        //TODO d == 15 itblock
        let mut result: ActiveValue<ARMv7MScfiaComposition> = _register_read_BV32_wide(rm.clone(), state, context);
        if (*context).scfia.check_condition(&(*context).scfia.new_bool_eq(&d.clone(), &(*context).scfia.new_bv_concrete(15, 4), None, false, &mut (*context).fork_sink, None), &mut (*context).fork_sink) {
            unimplemented!();
        } else {
            _register_write_BV32(rd.clone(), result.clone(), state, context);
            if (*context).scfia.check_condition(&(*context).scfia.new_bool_eq(&setflags.clone(), &(*context).scfia.new_bv_concrete(1, 1), None, false, &mut (*context).fork_sink, None), &mut (*context).fork_sink) {
                (*state).APSR.N = (*context).scfia.new_bv_slice(&result.clone(), 31, 31, None, &mut (*context).fork_sink, None);
                (*state).APSR.Z = _is_zero_bit_BV32(result.clone(), context);
            }
        }
    } else if (*context).scfia.check_condition(&(*context).scfia.new_bool_eq(&_matches_BV4(opcode.clone(), (*context).scfia.new_bv_concrete(0b1100, 4), (*context).scfia.new_bv_concrete(0b0010, 4), context), &(*context).scfia.new_bv_concrete(1, 1), None, false, &mut (*context).fork_sink, None), &mut (*context).fork_sink) {
        // BX Encoding T1
        let mut rm: ActiveValue<ARMv7MScfiaComposition> = (*context).scfia.new_bv_slice(&instruction.clone(), 6, 3, None, &mut (*context).fork_sink, None);
        // TODO if initblock && !lastiniitblock UNPREDICTABLE
        let mut m: ActiveValue<ARMv7MScfiaComposition> = _register_read_BV32_wide(rm.clone(), state, context);
        _bx_write_pc(m.clone(), state, context);
    } else {
        // BLX (register) Encoding T1
        let mut rm: ActiveValue<ARMv7MScfiaComposition> = (*context).scfia.new_bv_slice(&instruction.clone(), 6, 3, None, &mut (*context).fork_sink, None);
        if (*context).scfia.check_condition(&(*context).scfia.new_bool_eq(&rm.clone(), &(*context).scfia.new_bv_concrete(15, 4), None, false, &mut (*context).fork_sink, None), &mut (*context).fork_sink) {
            unimplemented!();
        }
        // TODO initblock unpredictable
        let mut target: ActiveValue<ARMv7MScfiaComposition> = _register_read_BV32_wide(rm.clone(), state, context);
        let mut next_instr_addr: ActiveValue<ARMv7MScfiaComposition> = (*context).scfia.new_bv_sub(&(*state).PC.clone(), &(*context).scfia.new_bv_concrete(2, 32), 32, None, &mut (*context).fork_sink, None);
        (*state).LR = (*context).scfia.new_bv_or(&next_instr_addr.clone(), &(*context).scfia.new_bv_concrete(1, 32), 32, None, &mut (*context).fork_sink, None);
        _blx_write_pc(target.clone(), state, context);
    }
}

unsafe fn _thumb16_load_from_literal_pool(mut instruction: ActiveValue<ARMv7MScfiaComposition>, mut state: *mut SystemState, context: *mut StepContext<ARMv7MScfiaComposition>) {
    // LDR (immediate) Encoding T2
    let mut bit_11: ActiveValue<ARMv7MScfiaComposition> = (*context).scfia.new_bv_slice(&instruction.clone(), 11, 11, None, &mut (*context).fork_sink, None);
    if (*context).scfia.check_condition(&(*context).scfia.new_bool_eq(&bit_11.clone(), &(*context).scfia.new_bv_concrete(1, 1), None, false, &mut (*context).fork_sink, None), &mut (*context).fork_sink) {
        let mut rt: ActiveValue<ARMv7MScfiaComposition> = (*context).scfia.new_bv_slice(&instruction.clone(), 10, 8, None, &mut (*context).fork_sink, None);
        let mut imm32: ActiveValue<ARMv7MScfiaComposition> = (*context).scfia.new_bv_concat(&(*context).scfia.new_bv_concrete(0, 22), &(*context).scfia.new_bv_concat(&(*context).scfia.new_bv_slice(&instruction.clone(), 7, 0, None, &mut (*context).fork_sink, None), &(*context).scfia.new_bv_concrete(0b00, 2), 10, None, &mut (*context).fork_sink, None), 32, None, &mut (*context).fork_sink, None);
        let mut add: ActiveValue<ARMv7MScfiaComposition> = (*context).scfia.new_bv_concrete(1, 1);
        _load_register(rt.clone(), imm32.clone(), add.clone(), state, context);
    } else {
        unimplemented!();
    }
}

unsafe fn _thumb16_load_store_single_data_item(mut instruction: ActiveValue<ARMv7MScfiaComposition>, mut state: *mut SystemState, context: *mut StepContext<ARMv7MScfiaComposition>) {
    let mut op_a: ActiveValue<ARMv7MScfiaComposition> = (*context).scfia.new_bv_slice(&instruction.clone(), 15, 12, None, &mut (*context).fork_sink, None);
    let mut op_b: ActiveValue<ARMv7MScfiaComposition> = (*context).scfia.new_bv_slice(&instruction.clone(), 11, 9, None, &mut (*context).fork_sink, None);
    if (*context).scfia.check_condition(&(*context).scfia.new_bool_eq(&op_a.clone(), &(*context).scfia.new_bv_concrete(0b0101, 4), None, false, &mut (*context).fork_sink, None), &mut (*context).fork_sink) {
        if (*context).scfia.check_condition(&(*context).scfia.new_bool_eq(&op_b.clone(), &(*context).scfia.new_bv_concrete(0b000, 3), None, false, &mut (*context).fork_sink, None), &mut (*context).fork_sink) {
            let mut rt: ActiveValue<ARMv7MScfiaComposition> = (*context).scfia.new_bv_slice(&instruction.clone(), 2, 0, None, &mut (*context).fork_sink, None);
            let mut rn: ActiveValue<ARMv7MScfiaComposition> = (*context).scfia.new_bv_slice(&instruction.clone(), 5, 3, None, &mut (*context).fork_sink, None);
            let mut rm: ActiveValue<ARMv7MScfiaComposition> = (*context).scfia.new_bv_slice(&instruction.clone(), 8, 6, None, &mut (*context).fork_sink, None);
            let mut offset: ActiveValue<ARMv7MScfiaComposition> = _register_read_BV32(rm.clone(), state, context);
            let mut base_address: ActiveValue<ARMv7MScfiaComposition> = _register_read_BV32(rn.clone(), state, context);
            let mut address: ActiveValue<ARMv7MScfiaComposition> = (*context).scfia.new_bv_add(&base_address.clone(), &offset.clone(), 32, None, &mut (*context).fork_sink, None);
            (*(*context).memory).write(&address.clone(), &_register_read_BV32(rt.clone(), state, context), 32, (*context).scfia.clone(), &mut (*context).hints, &mut (*context).fork_sink);
        } else if (*context).scfia.check_condition(&(*context).scfia.new_bool_eq(&op_b.clone(), &(*context).scfia.new_bv_concrete(0b001, 3), None, false, &mut (*context).fork_sink, None), &mut (*context).fork_sink) {
            unimplemented!();
        } else if (*context).scfia.check_condition(&(*context).scfia.new_bool_eq(&op_b.clone(), &(*context).scfia.new_bv_concrete(0b010, 3), None, false, &mut (*context).fork_sink, None), &mut (*context).fork_sink) {
            unimplemented!();
        } else if (*context).scfia.check_condition(&(*context).scfia.new_bool_eq(&op_b.clone(), &(*context).scfia.new_bv_concrete(0b011, 3), None, false, &mut (*context).fork_sink, None), &mut (*context).fork_sink) {
            unimplemented!();
        } else if (*context).scfia.check_condition(&(*context).scfia.new_bool_eq(&op_b.clone(), &(*context).scfia.new_bv_concrete(0b100, 3), None, false, &mut (*context).fork_sink, None), &mut (*context).fork_sink) {
            let mut rt: ActiveValue<ARMv7MScfiaComposition> = (*context).scfia.new_bv_slice(&instruction.clone(), 2, 0, None, &mut (*context).fork_sink, None);
            let mut rn: ActiveValue<ARMv7MScfiaComposition> = (*context).scfia.new_bv_slice(&instruction.clone(), 5, 3, None, &mut (*context).fork_sink, None);
            let mut rm: ActiveValue<ARMv7MScfiaComposition> = (*context).scfia.new_bv_slice(&instruction.clone(), 8, 6, None, &mut (*context).fork_sink, None);
            let mut offset: ActiveValue<ARMv7MScfiaComposition> = _register_read_BV32(rm.clone(), state, context);
            let mut address: ActiveValue<ARMv7MScfiaComposition> = (*context).scfia.new_bv_add(&offset.clone(), &_register_read_BV32(rn.clone(), state, context), 32, None, &mut (*context).fork_sink, None);
            let mut value: ActiveValue<ARMv7MScfiaComposition> = (*(*context).memory).read(&address.clone(), 32, (*context).scfia.clone(), &mut (*context).hints, &mut (*context).fork_sink);
            _register_write_BV32(rt.clone(), value.clone(), state, context);
            //TODO unpredictable reg15
        } else if (*context).scfia.check_condition(&(*context).scfia.new_bool_eq(&op_b.clone(), &(*context).scfia.new_bv_concrete(0b101, 3), None, false, &mut (*context).fork_sink, None), &mut (*context).fork_sink) {
            unimplemented!();
        } else if (*context).scfia.check_condition(&(*context).scfia.new_bool_eq(&op_b.clone(), &(*context).scfia.new_bv_concrete(0b110, 3), None, false, &mut (*context).fork_sink, None), &mut (*context).fork_sink) {
            unimplemented!();
        } else if (*context).scfia.check_condition(&(*context).scfia.new_bool_eq(&op_b.clone(), &(*context).scfia.new_bv_concrete(0b111, 3), None, false, &mut (*context).fork_sink, None), &mut (*context).fork_sink) {
            unimplemented!();
        }
    } else if (*context).scfia.check_condition(&(*context).scfia.new_bool_eq(&op_a.clone(), &(*context).scfia.new_bv_concrete(0b0110, 4), None, false, &mut (*context).fork_sink, None), &mut (*context).fork_sink) {
        if (*context).scfia.check_condition(&(*context).scfia.new_bool_eq(&(*context).scfia.new_bv_slice(&op_b.clone(), 2, 2, None, &mut (*context).fork_sink, None), &(*context).scfia.new_bv_concrete(0, 1), None, false, &mut (*context).fork_sink, None), &mut (*context).fork_sink) {
            let mut imm5: ActiveValue<ARMv7MScfiaComposition> = (*context).scfia.new_bv_slice(&instruction.clone(), 10, 6, None, &mut (*context).fork_sink, None);
            let mut rn: ActiveValue<ARMv7MScfiaComposition> = (*context).scfia.new_bv_slice(&instruction.clone(), 5, 3, None, &mut (*context).fork_sink, None);
            let mut rt: ActiveValue<ARMv7MScfiaComposition> = (*context).scfia.new_bv_slice(&instruction.clone(), 2, 0, None, &mut (*context).fork_sink, None);
            let mut imm7: ActiveValue<ARMv7MScfiaComposition> = (*context).scfia.new_bv_concat(&imm5.clone(), &(*context).scfia.new_bv_concrete(0b00, 2), 7, None, &mut (*context).fork_sink, None);
            let mut imm32: ActiveValue<ARMv7MScfiaComposition> = (*context).scfia.new_bv_concat(&(*context).scfia.new_bv_concrete(0, 25), &imm7.clone(), 32, None, &mut (*context).fork_sink, None);
            let mut address: ActiveValue<ARMv7MScfiaComposition> = (*context).scfia.new_bv_add(&_register_read_BV32(rn.clone(), state, context), &imm32.clone(), 32, None, &mut (*context).fork_sink, None);
            (*(*context).memory).write(&address.clone(), &_register_read_BV32(rt.clone(), state, context), 32, (*context).scfia.clone(), &mut (*context).hints, &mut (*context).fork_sink);
        } else {
            let mut rt: ActiveValue<ARMv7MScfiaComposition> = (*context).scfia.new_bv_slice(&instruction.clone(), 2, 0, None, &mut (*context).fork_sink, None);
            let mut rn: ActiveValue<ARMv7MScfiaComposition> = (*context).scfia.new_bv_slice(&instruction.clone(), 5, 3, None, &mut (*context).fork_sink, None);
            let mut imm5: ActiveValue<ARMv7MScfiaComposition> = (*context).scfia.new_bv_slice(&instruction.clone(), 10, 6, None, &mut (*context).fork_sink, None);
            let mut imm7: ActiveValue<ARMv7MScfiaComposition> = (*context).scfia.new_bv_concat(&imm5.clone(), &(*context).scfia.new_bv_concrete(0b00, 2), 7, None, &mut (*context).fork_sink, None);
            let mut imm32: ActiveValue<ARMv7MScfiaComposition> = (*context).scfia.new_bv_concat(&(*context).scfia.new_bv_concrete(0, 25), &imm7.clone(), 32, None, &mut (*context).fork_sink, None);
            let mut address: ActiveValue<ARMv7MScfiaComposition> = (*context).scfia.new_bv_add(&_register_read_BV32(rn.clone(), state, context), &imm32.clone(), 32, None, &mut (*context).fork_sink, None);
            let mut data: ActiveValue<ARMv7MScfiaComposition> = (*(*context).memory).read(&address.clone(), 32, (*context).scfia.clone(), &mut (*context).hints, &mut (*context).fork_sink);
            _register_write_BV32(rt.clone(), data.clone(), state, context);
        }
    } else if (*context).scfia.check_condition(&(*context).scfia.new_bool_eq(&op_a.clone(), &(*context).scfia.new_bv_concrete(0b0111, 4), None, false, &mut (*context).fork_sink, None), &mut (*context).fork_sink) {
        unimplemented!();
    } else if (*context).scfia.check_condition(&(*context).scfia.new_bool_eq(&op_a.clone(), &(*context).scfia.new_bv_concrete(0b1000, 4), None, false, &mut (*context).fork_sink, None), &mut (*context).fork_sink) {
        unimplemented!();
    } else if (*context).scfia.check_condition(&(*context).scfia.new_bool_eq(&op_a.clone(), &(*context).scfia.new_bv_concrete(0b1001, 4), None, false, &mut (*context).fork_sink, None), &mut (*context).fork_sink) {
        unimplemented!();
    } else {
        unimplemented!();
    }
}

unsafe fn _thumb16_misc_16_bit_instruction(mut instruction: ActiveValue<ARMv7MScfiaComposition>, mut state: *mut SystemState, context: *mut StepContext<ARMv7MScfiaComposition>) {
    let mut opcode: ActiveValue<ARMv7MScfiaComposition> = (*context).scfia.new_bv_slice(&instruction.clone(), 11, 5, None, &mut (*context).fork_sink, None);
    if (*context).scfia.check_condition(&(*context).scfia.new_bool_eq(&opcode.clone(), &(*context).scfia.new_bv_concrete(0b0110011, 7), None, false, &mut (*context).fork_sink, None), &mut (*context).fork_sink) {
        // CPS
        unimplemented!();
    } else if (*context).scfia.check_condition(&(*context).scfia.new_bool_eq(&_matches_BV7(opcode.clone(), (*context).scfia.new_bv_concrete(0b0000000, 7), (*context).scfia.new_bv_concrete(0b1111100, 7), context), &(*context).scfia.new_bv_concrete(1, 1), None, false, &mut (*context).fork_sink, None), &mut (*context).fork_sink) {
        // ADD (SP plus immediate);
        unimplemented!();
    } else if (*context).scfia.check_condition(&(*context).scfia.new_bool_eq(&_matches_BV7(opcode.clone(), (*context).scfia.new_bv_concrete(0b0000100, 7), (*context).scfia.new_bv_concrete(0b1111000, 7), context), &(*context).scfia.new_bv_concrete(1, 1), None, false, &mut (*context).fork_sink, None), &mut (*context).fork_sink) {
        // SUB (SP minus immediate) Encoding T1
        let mut imm7: ActiveValue<ARMv7MScfiaComposition> = (*context).scfia.new_bv_slice(&instruction.clone(), 6, 0, None, &mut (*context).fork_sink, None);
        let mut imm9: ActiveValue<ARMv7MScfiaComposition> = (*context).scfia.new_bv_concat(&imm7.clone(), &(*context).scfia.new_bv_concrete(0, 2), 9, None, &mut (*context).fork_sink, None);
        let mut imm32: ActiveValue<ARMv7MScfiaComposition> = (*context).scfia.new_bv_concat(&(*context).scfia.new_bv_concrete(0, 23), &imm9.clone(), 32, None, &mut (*context).fork_sink, None);
        (*state).SP = (*context).scfia.new_bv_sub(&(*state).SP.clone(), &imm32.clone(), 32, None, &mut (*context).fork_sink, None);
    } else if (*context).scfia.check_condition(&(*context).scfia.new_bool_eq(&_matches_BV7(opcode.clone(), (*context).scfia.new_bv_concrete(0b0001000, 7), (*context).scfia.new_bv_concrete(0b1110000, 7), context), &(*context).scfia.new_bv_concrete(1, 1), None, false, &mut (*context).fork_sink, None), &mut (*context).fork_sink) {
        // Compare and Branch on Zero
        // CBNZ, CBZ Encoding T1
        let mut rn: ActiveValue<ARMv7MScfiaComposition> = (*context).scfia.new_bv_slice(&instruction.clone(), 2, 0, None, &mut (*context).fork_sink, None);
        let mut imm5: ActiveValue<ARMv7MScfiaComposition> = (*context).scfia.new_bv_slice(&instruction.clone(), 7, 3, None, &mut (*context).fork_sink, None);
        let mut i: ActiveValue<ARMv7MScfiaComposition> = (*context).scfia.new_bv_slice(&instruction.clone(), 9, 9, None, &mut (*context).fork_sink, None);
        let mut op: ActiveValue<ARMv7MScfiaComposition> = (*context).scfia.new_bv_slice(&instruction.clone(), 11, 11, None, &mut (*context).fork_sink, None);
        let mut imm6: ActiveValue<ARMv7MScfiaComposition> = (*context).scfia.new_bv_concat(&imm5.clone(), &(*context).scfia.new_bv_concrete(0, 1), 6, None, &mut (*context).fork_sink, None);
        let mut imm7: ActiveValue<ARMv7MScfiaComposition> = (*context).scfia.new_bv_concat(&i.clone(), &imm6.clone(), 7, None, &mut (*context).fork_sink, None);
        let mut imm32: ActiveValue<ARMv7MScfiaComposition> = (*context).scfia.new_bv_concat(&(*context).scfia.new_bv_concrete(0, 25), &imm7.clone(), 32, None, &mut (*context).fork_sink, None);
        if (*context).scfia.check_condition(&(*context).scfia.new_bool_eq(&op.clone(), &(*context).scfia.new_bv_concrete(0, 1), None, false, &mut (*context).fork_sink, None), &mut (*context).fork_sink) {
            // nonzero = false
            if (*context).scfia.check_condition(&(*context).scfia.new_bool_eq(&_register_read_BV32(rn.clone(), state, context), &(*context).scfia.new_bv_concrete(0, 32), None, false, &mut (*context).fork_sink, None), &mut (*context).fork_sink) {
                _branch_write_pc((*context).scfia.new_bv_add(&(*state).PC.clone(), &imm32.clone(), 32, None, &mut (*context).fork_sink, None), state, context);
            }
        } else {
            // nonzero = true
            unimplemented!();
        }
    } else if (*context).scfia.check_condition(&(*context).scfia.new_bool_eq(&_matches_BV7(opcode.clone(), (*context).scfia.new_bv_concrete(0b0010000, 7), (*context).scfia.new_bv_concrete(0b1101110, 7), context), &(*context).scfia.new_bv_concrete(1, 1), None, false, &mut (*context).fork_sink, None), &mut (*context).fork_sink) {
        // SXTH
        unimplemented!();
    } else if (*context).scfia.check_condition(&(*context).scfia.new_bool_eq(&_matches_BV7(opcode.clone(), (*context).scfia.new_bv_concrete(0b0010010, 7), (*context).scfia.new_bv_concrete(0b1101100, 7), context), &(*context).scfia.new_bv_concrete(1, 1), None, false, &mut (*context).fork_sink, None), &mut (*context).fork_sink) {
        // SXTB
        unimplemented!();
    } else if (*context).scfia.check_condition(&(*context).scfia.new_bool_eq(&_matches_BV7(opcode.clone(), (*context).scfia.new_bv_concrete(0b0010110, 7), (*context).scfia.new_bv_concrete(0b1101000, 7), context), &(*context).scfia.new_bv_concrete(1, 1), None, false, &mut (*context).fork_sink, None), &mut (*context).fork_sink) {
        // UXTH
        unimplemented!();
    } else if (*context).scfia.check_condition(&(*context).scfia.new_bool_eq(&_matches_BV7(opcode.clone(), (*context).scfia.new_bv_concrete(0b0011000, 7), (*context).scfia.new_bv_concrete(0b1100000, 7), context), &(*context).scfia.new_bv_concrete(1, 1), None, false, &mut (*context).fork_sink, None), &mut (*context).fork_sink) {
        // Compare and Branch on Zero
        // CBNZ, CBZ
        unimplemented!();
    } else if (*context).scfia.check_condition(&(*context).scfia.new_bool_eq(&_matches_BV7(opcode.clone(), (*context).scfia.new_bv_concrete(0b0100000, 7), (*context).scfia.new_bv_concrete(0b1010000, 7), context), &(*context).scfia.new_bv_concrete(1, 1), None, false, &mut (*context).fork_sink, None), &mut (*context).fork_sink) {
        // PUSH Encoding T1
        let mut register_list: ActiveValue<ARMv7MScfiaComposition> = (*context).scfia.new_bv_slice(&instruction.clone(), 7, 0, None, &mut (*context).fork_sink, None);
        let mut m: ActiveValue<ARMv7MScfiaComposition> = (*context).scfia.new_bv_slice(&instruction.clone(), 8, 8, None, &mut (*context).fork_sink, None);
        let mut registers_14: ActiveValue<ARMv7MScfiaComposition> = (*context).scfia.new_bv_concat(&(*context).scfia.new_bv_concrete(0, 6), &register_list.clone(), 14, None, &mut (*context).fork_sink, None);
        let mut registers_15: ActiveValue<ARMv7MScfiaComposition> = (*context).scfia.new_bv_concat(&m.clone(), &registers_14.clone(), 15, None, &mut (*context).fork_sink, None);
        let mut registers: ActiveValue<ARMv7MScfiaComposition> = (*context).scfia.new_bv_concat(&(*context).scfia.new_bv_concrete(0, 1), &registers_15.clone(), 16, None, &mut (*context).fork_sink, None);
        //TODO if BitCount(registers) < 1 then UNPREDICTABLE;
        _push(registers.clone(), state, context);
    } else if (*context).scfia.check_condition(&(*context).scfia.new_bool_eq(&_matches_BV7(opcode.clone(), (*context).scfia.new_bv_concrete(0b1001000, 7), (*context).scfia.new_bv_concrete(0b0110000, 7), context), &(*context).scfia.new_bv_concrete(1, 1), None, false, &mut (*context).fork_sink, None), &mut (*context).fork_sink) {
        // Compare and Branch on Nonzero
        // CBNZ, CBZ
        unimplemented!();
    } else if (*context).scfia.check_condition(&(*context).scfia.new_bool_eq(&_matches_BV7(opcode.clone(), (*context).scfia.new_bv_concrete(0b1010000, 7), (*context).scfia.new_bv_concrete(0b0101110, 7), context), &(*context).scfia.new_bv_concrete(1, 1), None, false, &mut (*context).fork_sink, None), &mut (*context).fork_sink) {
        // REV
        unimplemented!();
    } else if (*context).scfia.check_condition(&(*context).scfia.new_bool_eq(&_matches_BV7(opcode.clone(), (*context).scfia.new_bv_concrete(0b1010010, 7), (*context).scfia.new_bv_concrete(0b0101100, 7), context), &(*context).scfia.new_bv_concrete(1, 1), None, false, &mut (*context).fork_sink, None), &mut (*context).fork_sink) {
        // REV16
        unimplemented!();
    } else if (*context).scfia.check_condition(&(*context).scfia.new_bool_eq(&_matches_BV7(opcode.clone(), (*context).scfia.new_bv_concrete(0b1010110, 7), (*context).scfia.new_bv_concrete(0b0101000, 7), context), &(*context).scfia.new_bv_concrete(1, 1), None, false, &mut (*context).fork_sink, None), &mut (*context).fork_sink) {
        // REVSH
        unimplemented!();
    } else if (*context).scfia.check_condition(&(*context).scfia.new_bool_eq(&_matches_BV7(opcode.clone(), (*context).scfia.new_bv_concrete(0b1011000, 7), (*context).scfia.new_bv_concrete(0b0100000, 7), context), &(*context).scfia.new_bv_concrete(1, 1), None, false, &mut (*context).fork_sink, None), &mut (*context).fork_sink) {
        // Compare and Branch on Nonzero
        // CBNZ, CBZ
    } else if (*context).scfia.check_condition(&(*context).scfia.new_bool_eq(&_matches_BV7(opcode.clone(), (*context).scfia.new_bv_concrete(0b1100000, 7), (*context).scfia.new_bv_concrete(0b0010000, 7), context), &(*context).scfia.new_bv_concrete(1, 1), None, false, &mut (*context).fork_sink, None), &mut (*context).fork_sink) {
        // POP Encoding T1
        let mut register_list: ActiveValue<ARMv7MScfiaComposition> = (*context).scfia.new_bv_slice(&instruction.clone(), 7, 0, None, &mut (*context).fork_sink, None);
        let mut p: ActiveValue<ARMv7MScfiaComposition> = (*context).scfia.new_bv_slice(&instruction.clone(), 8, 8, None, &mut (*context).fork_sink, None);
        let mut register_list15: ActiveValue<ARMv7MScfiaComposition> = (*context).scfia.new_bv_concat(&(*context).scfia.new_bv_concrete(0, 7), &register_list.clone(), 15, None, &mut (*context).fork_sink, None);
        let mut registers: ActiveValue<ARMv7MScfiaComposition> = (*context).scfia.new_bv_concat(&p.clone(), &register_list15.clone(), 16, None, &mut (*context).fork_sink, None);
        let mut p: ActiveValue<ARMv7MScfiaComposition> = (*context).scfia.new_bv_slice(&instruction.clone(), 8, 8, None, &mut (*context).fork_sink, None);
        if (*context).scfia.check_condition(&(*context).scfia.new_bool_eq(&register_list.clone(), &(*context).scfia.new_bv_concrete(0, 8), None, false, &mut (*context).fork_sink, None), &mut (*context).fork_sink) {
            // UNPREDICTABLE
            unimplemented!();
        }
        //TODO if registers<15> && initblock unpredictable
        let mut address: ActiveValue<ARMv7MScfiaComposition> = (*state).SP.clone();
        let mut bit_count: ActiveValue<ARMv7MScfiaComposition> = (*context).scfia.new_bv_concat(&(*context).scfia.new_bv_concrete(0, 16), &_bit_count_BV16(registers.clone(), context), 32, None, &mut (*context).fork_sink, None);
        (*state).SP = (*context).scfia.new_bv_add(&(*state).SP.clone(), &(*context).scfia.new_bv_multiply(&(*context).scfia.new_bv_concrete(4, 32), &bit_count.clone(), 32, None, &mut (*context).fork_sink, None), 32, None, &mut (*context).fork_sink, None);
        for i in 0u64..=14 {
            if (*context).scfia.check_condition(&(*context).scfia.new_bool_eq(&(*context).scfia.new_bv_concrete(1, 1), &(*context).scfia.new_bv_slice(&registers.clone(), i as u32, (i as u32+1-1), None, &mut (*context).fork_sink, None), None, false, &mut (*context).fork_sink, None), &mut (*context).fork_sink) {
                _register_write_BV32_wide((*context).scfia.new_bv_concrete(i, 4), (*(*context).memory).read(&address.clone(), 32, (*context).scfia.clone(), &mut (*context).hints, &mut (*context).fork_sink), state, context);
                address = (*context).scfia.new_bv_add(&address.clone(), &(*context).scfia.new_bv_concrete(4, 32), 32, None, &mut (*context).fork_sink, None);
            }
        }
        if (*context).scfia.check_condition(&(*context).scfia.new_bool_eq(&(*context).scfia.new_bv_slice(&registers.clone(), 15, 15, None, &mut (*context).fork_sink, None), &(*context).scfia.new_bv_concrete(1, 1), None, false, &mut (*context).fork_sink, None), &mut (*context).fork_sink) {
            _load_write_pc((*(*context).memory).read(&address.clone(), 32, (*context).scfia.clone(), &mut (*context).hints, &mut (*context).fork_sink), state, context);
        }
    } else if (*context).scfia.check_condition(&(*context).scfia.new_bool_eq(&_matches_BV7(opcode.clone(), (*context).scfia.new_bv_concrete(0b1110000, 7), (*context).scfia.new_bv_concrete(0b0001000, 7), context), &(*context).scfia.new_bv_concrete(1, 1), None, false, &mut (*context).fork_sink, None), &mut (*context).fork_sink) {
        // BKPT
        unimplemented!();
    } else if (*context).scfia.check_condition(&(*context).scfia.new_bool_eq(&_matches_BV7(opcode.clone(), (*context).scfia.new_bv_concrete(0b1111000, 7), (*context).scfia.new_bv_concrete(0b0000000, 7), context), &(*context).scfia.new_bv_concrete(1, 1), None, false, &mut (*context).fork_sink, None), &mut (*context).fork_sink) {
        // If-Then, and hints
        _thumb16_if_then_and_hints(instruction.clone(), state, context);
    } else {
        //TODO do remaining foo
        //TODO Other encodings in this space are UNDEFINED.
        unimplemented!();
    }
}

unsafe fn _push(mut registers: ActiveValue<ARMv7MScfiaComposition>, mut state: *mut SystemState, context: *mut StepContext<ARMv7MScfiaComposition>) {
    let mut SP: ActiveValue<ARMv7MScfiaComposition> = (*state).SP.clone();
    let mut bit_count: ActiveValue<ARMv7MScfiaComposition> = (*context).scfia.new_bv_concat(&(*context).scfia.new_bv_concrete(0, 16), &_bit_count_BV16(registers.clone(), context), 32, None, &mut (*context).fork_sink, None);
    let mut address: ActiveValue<ARMv7MScfiaComposition> = (*context).scfia.new_bv_sub(&SP.clone(), &(*context).scfia.new_bv_multiply(&(*context).scfia.new_bv_concrete(4, 32), &bit_count.clone(), 32, None, &mut (*context).fork_sink, None), 32, None, &mut (*context).fork_sink, None);
    for i in 0u64..=14 {
        if (*context).scfia.check_condition(&(*context).scfia.new_bool_eq(&(*context).scfia.new_bv_concrete(1, 1), &(*context).scfia.new_bv_slice(&registers.clone(), i as u32, (i as u32+1-1), None, &mut (*context).fork_sink, None), None, false, &mut (*context).fork_sink, None), &mut (*context).fork_sink) {
            (*(*context).memory).write(&address.clone(), &_register_read_BV32_wide((*context).scfia.new_bv_concrete(i, 4), state, context), 32, (*context).scfia.clone(), &mut (*context).hints, &mut (*context).fork_sink);
            address = (*context).scfia.new_bv_add(&address.clone(), &(*context).scfia.new_bv_concrete(4, 32), 32, None, &mut (*context).fork_sink, None);
        }
    }
    (*state).SP = (*context).scfia.new_bv_sub(&SP.clone(), &(*context).scfia.new_bv_multiply(&(*context).scfia.new_bv_concrete(4, 32), &bit_count.clone(), 32, None, &mut (*context).fork_sink, None), 32, None, &mut (*context).fork_sink, None);
}

unsafe fn _load_write_pc(mut address: ActiveValue<ARMv7MScfiaComposition>, mut state: *mut SystemState, context: *mut StepContext<ARMv7MScfiaComposition>) {
    _bx_write_pc(address.clone(), state, context);
}

unsafe fn _thumb16_if_then_and_hints(mut instruction: ActiveValue<ARMv7MScfiaComposition>, mut state: *mut SystemState, context: *mut StepContext<ARMv7MScfiaComposition>) {
    let mut opB: ActiveValue<ARMv7MScfiaComposition> = (*context).scfia.new_bv_slice(&instruction.clone(), 3, 0, None, &mut (*context).fork_sink, None);
    let mut opA: ActiveValue<ARMv7MScfiaComposition> = (*context).scfia.new_bv_slice(&instruction.clone(), 7, 4, None, &mut (*context).fork_sink, None);
    if (*context).scfia.check_condition(&(*context).scfia.new_bool_not(&(*context).scfia.new_bool_eq(&opB.clone(), &(*context).scfia.new_bv_concrete(0, 4), None, false, &mut (*context).fork_sink, None), None, false, &mut (*context).fork_sink, None), &mut (*context).fork_sink) {
        // IT
        unimplemented!();
    } else {
        if (*context).scfia.check_condition(&(*context).scfia.new_bool_eq(&opA.clone(), &(*context).scfia.new_bv_concrete(0b0000, 4), None, false, &mut (*context).fork_sink, None), &mut (*context).fork_sink) {
            // NOP
        } else if (*context).scfia.check_condition(&(*context).scfia.new_bool_eq(&opA.clone(), &(*context).scfia.new_bv_concrete(0b0001, 4), None, false, &mut (*context).fork_sink, None), &mut (*context).fork_sink) {
            // YIELD
            unimplemented!();
        } else if (*context).scfia.check_condition(&(*context).scfia.new_bool_eq(&opA.clone(), &(*context).scfia.new_bv_concrete(0b0010, 4), None, false, &mut (*context).fork_sink, None), &mut (*context).fork_sink) {
            // WFE
            unimplemented!();
        } else if (*context).scfia.check_condition(&(*context).scfia.new_bool_eq(&opA.clone(), &(*context).scfia.new_bv_concrete(0b0011, 4), None, false, &mut (*context).fork_sink, None), &mut (*context).fork_sink) {
            // WFI
            unimplemented!();
        } else if (*context).scfia.check_condition(&(*context).scfia.new_bool_eq(&opA.clone(), &(*context).scfia.new_bv_concrete(0b0100, 4), None, false, &mut (*context).fork_sink, None), &mut (*context).fork_sink) {
            // SEV
            unimplemented!();
        }
    }
}

unsafe fn _thumb16_conditional_branch_and_svc(mut instruction: ActiveValue<ARMv7MScfiaComposition>, mut state: *mut SystemState, context: *mut StepContext<ARMv7MScfiaComposition>) {
    let mut opcode: ActiveValue<ARMv7MScfiaComposition> = (*context).scfia.new_bv_slice(&instruction.clone(), 11, 8, None, &mut (*context).fork_sink, None);
    if (*context).scfia.check_condition(&(*context).scfia.new_bool_eq(&opcode.clone(), &(*context).scfia.new_bv_concrete(0b1110, 4), None, false, &mut (*context).fork_sink, None), &mut (*context).fork_sink) {
        // UDF
        unimplemented!();
    } else if (*context).scfia.check_condition(&(*context).scfia.new_bool_eq(&opcode.clone(), &(*context).scfia.new_bv_concrete(0b1111, 4), None, false, &mut (*context).fork_sink, None), &mut (*context).fork_sink) {
        // SVC
        unimplemented!();
    } else {
        // B Encoding T1
        let mut cond: ActiveValue<ARMv7MScfiaComposition> = (*context).scfia.new_bv_slice(&instruction.clone(), 11, 8, None, &mut (*context).fork_sink, None);
        if (*context).scfia.check_condition(&(*context).scfia.new_bool_eq(&cond.clone(), &(*context).scfia.new_bv_concrete(0b1110, 4), None, false, &mut (*context).fork_sink, None), &mut (*context).fork_sink) {
            unimplemented!();
        } else if (*context).scfia.check_condition(&(*context).scfia.new_bool_eq(&cond.clone(), &(*context).scfia.new_bv_concrete(0b1111, 4), None, false, &mut (*context).fork_sink, None), &mut (*context).fork_sink) {
            unimplemented!();
        }
        let mut imm8: ActiveValue<ARMv7MScfiaComposition> = (*context).scfia.new_bv_slice(&instruction.clone(), 7, 0, None, &mut (*context).fork_sink, None);
        let mut imm9: ActiveValue<ARMv7MScfiaComposition> = (*context).scfia.new_bv_concat(&imm8.clone(), &(*context).scfia.new_bv_concrete(0, 1), 9, None, &mut (*context).fork_sink, None);
        let mut imm32: ActiveValue<ARMv7MScfiaComposition> = (*context).scfia.new_bv_sign_extend(&imm9.clone(), 9, 32, None, &mut (*context).fork_sink, None);
        //TODO if InITBlock then UNPREDICTABLE
        if (*context).scfia.check_condition(&(*context).scfia.new_bool_eq(&_condition_passed(cond.clone(), state, context), &(*context).scfia.new_bv_concrete(1, 1), None, false, &mut (*context).fork_sink, None), &mut (*context).fork_sink) {
            _branch_write_pc((*context).scfia.new_bv_add(&(*state).PC.clone(), &imm32.clone(), 32, None, &mut (*context).fork_sink, None), state, context);
        }
    }
}

unsafe fn _thumb16_unconditional_branch(mut instruction: ActiveValue<ARMv7MScfiaComposition>, mut state: *mut SystemState, context: *mut StepContext<ARMv7MScfiaComposition>) {
    // Encoding T2
    let mut imm11: ActiveValue<ARMv7MScfiaComposition> = (*context).scfia.new_bv_slice(&instruction.clone(), 10, 0, None, &mut (*context).fork_sink, None);
    let mut imm32: ActiveValue<ARMv7MScfiaComposition> = (*context).scfia.new_bv_sign_extend(&(*context).scfia.new_bv_concat(&imm11.clone(), &(*context).scfia.new_bv_concrete(0, 1), 12, None, &mut (*context).fork_sink, None), 12, 32, None, &mut (*context).fork_sink, None);
    let mut value: ActiveValue<ARMv7MScfiaComposition> = (*context).scfia.new_bv_add(&(*state).PC.clone(), &imm32.clone(), 32, None, &mut (*context).fork_sink, None);
    _branch_write_pc(value.clone(), state, context);
}

unsafe fn _mov_immediate(mut rd: ActiveValue<ARMv7MScfiaComposition>, mut setflags: ActiveValue<ARMv7MScfiaComposition>, mut imm32: ActiveValue<ARMv7MScfiaComposition>, mut carry: ActiveValue<ARMv7MScfiaComposition>, mut state: *mut SystemState, context: *mut StepContext<ARMv7MScfiaComposition>) {
    _register_write_BV32(rd.clone(), imm32.clone(), state, context);
    if (*context).scfia.check_condition(&(*context).scfia.new_bool_eq(&(*context).scfia.new_bv_concrete(0b1, 1), &setflags.clone(), None, false, &mut (*context).fork_sink, None), &mut (*context).fork_sink) {
        (*state).APSR.N = (*context).scfia.new_bv_slice(&imm32.clone(), 31, 31, None, &mut (*context).fork_sink, None);
        (*state).APSR.Z = _is_zero_bit_BV32(imm32.clone(), context);
        (*state).APSR.C = carry.clone();
    }
}

unsafe fn _register_write_BV32(mut register_id: ActiveValue<ARMv7MScfiaComposition>, mut value: ActiveValue<ARMv7MScfiaComposition>, mut state: *mut SystemState, context: *mut StepContext<ARMv7MScfiaComposition>) {
    _register_write_BV32_wide((*context).scfia.new_bv_concat(&(*context).scfia.new_bv_concrete(0, 1), &register_id.clone(), 4, None, &mut (*context).fork_sink, None), value.clone(), state, context);
}

unsafe fn _register_write_BV32_wide(mut register_id: ActiveValue<ARMv7MScfiaComposition>, mut value: ActiveValue<ARMv7MScfiaComposition>, mut state: *mut SystemState, context: *mut StepContext<ARMv7MScfiaComposition>) {
    if (*context).scfia.check_condition(&(*context).scfia.new_bool_eq(&register_id.clone(), &(*context).scfia.new_bv_concrete(0, 4), None, false, &mut (*context).fork_sink, None), &mut (*context).fork_sink) {
        (*state).R0 = value.clone();
    } else if (*context).scfia.check_condition(&(*context).scfia.new_bool_eq(&register_id.clone(), &(*context).scfia.new_bv_concrete(1, 4), None, false, &mut (*context).fork_sink, None), &mut (*context).fork_sink) {
        (*state).R1 = value.clone();
    } else if (*context).scfia.check_condition(&(*context).scfia.new_bool_eq(&register_id.clone(), &(*context).scfia.new_bv_concrete(2, 4), None, false, &mut (*context).fork_sink, None), &mut (*context).fork_sink) {
        (*state).R2 = value.clone();
    } else if (*context).scfia.check_condition(&(*context).scfia.new_bool_eq(&register_id.clone(), &(*context).scfia.new_bv_concrete(3, 4), None, false, &mut (*context).fork_sink, None), &mut (*context).fork_sink) {
        (*state).R3 = value.clone();
    } else if (*context).scfia.check_condition(&(*context).scfia.new_bool_eq(&register_id.clone(), &(*context).scfia.new_bv_concrete(4, 4), None, false, &mut (*context).fork_sink, None), &mut (*context).fork_sink) {
        (*state).R4 = value.clone();
    } else if (*context).scfia.check_condition(&(*context).scfia.new_bool_eq(&register_id.clone(), &(*context).scfia.new_bv_concrete(5, 4), None, false, &mut (*context).fork_sink, None), &mut (*context).fork_sink) {
        (*state).R5 = value.clone();
    } else if (*context).scfia.check_condition(&(*context).scfia.new_bool_eq(&register_id.clone(), &(*context).scfia.new_bv_concrete(6, 4), None, false, &mut (*context).fork_sink, None), &mut (*context).fork_sink) {
        (*state).R6 = value.clone();
    } else if (*context).scfia.check_condition(&(*context).scfia.new_bool_eq(&register_id.clone(), &(*context).scfia.new_bv_concrete(7, 4), None, false, &mut (*context).fork_sink, None), &mut (*context).fork_sink) {
        (*state).R7 = value.clone();
    } else if (*context).scfia.check_condition(&(*context).scfia.new_bool_eq(&register_id.clone(), &(*context).scfia.new_bv_concrete(8, 4), None, false, &mut (*context).fork_sink, None), &mut (*context).fork_sink) {
        (*state).R8 = value.clone();
    } else if (*context).scfia.check_condition(&(*context).scfia.new_bool_eq(&register_id.clone(), &(*context).scfia.new_bv_concrete(9, 4), None, false, &mut (*context).fork_sink, None), &mut (*context).fork_sink) {
        (*state).R9 = value.clone();
    } else if (*context).scfia.check_condition(&(*context).scfia.new_bool_eq(&register_id.clone(), &(*context).scfia.new_bv_concrete(10, 4), None, false, &mut (*context).fork_sink, None), &mut (*context).fork_sink) {
        (*state).R10 = value.clone();
    } else if (*context).scfia.check_condition(&(*context).scfia.new_bool_eq(&register_id.clone(), &(*context).scfia.new_bv_concrete(11, 4), None, false, &mut (*context).fork_sink, None), &mut (*context).fork_sink) {
        (*state).R11 = value.clone();
    } else if (*context).scfia.check_condition(&(*context).scfia.new_bool_eq(&register_id.clone(), &(*context).scfia.new_bv_concrete(12, 4), None, false, &mut (*context).fork_sink, None), &mut (*context).fork_sink) {
        (*state).R12 = value.clone();
    } else if (*context).scfia.check_condition(&(*context).scfia.new_bool_eq(&register_id.clone(), &(*context).scfia.new_bv_concrete(13, 4), None, false, &mut (*context).fork_sink, None), &mut (*context).fork_sink) {
        (*state).SP = value.clone();
    } else if (*context).scfia.check_condition(&(*context).scfia.new_bool_eq(&register_id.clone(), &(*context).scfia.new_bv_concrete(14, 4), None, false, &mut (*context).fork_sink, None), &mut (*context).fork_sink) {
        (*state).LR = value.clone();
    } else {
        (*state).PC = value.clone();
    }
}

unsafe fn _register_read_BV32(mut register_id: ActiveValue<ARMv7MScfiaComposition>, mut state: *mut SystemState, context: *mut StepContext<ARMv7MScfiaComposition>) -> ActiveValue<ARMv7MScfiaComposition> {
    return _register_read_BV32_wide((*context).scfia.new_bv_concat(&(*context).scfia.new_bv_concrete(0, 1), &register_id.clone(), 4, None, &mut (*context).fork_sink, None), state, context);
}

unsafe fn _register_read_BV32_wide(mut register_id: ActiveValue<ARMv7MScfiaComposition>, mut state: *mut SystemState, context: *mut StepContext<ARMv7MScfiaComposition>) -> ActiveValue<ARMv7MScfiaComposition> {
    if (*context).scfia.check_condition(&(*context).scfia.new_bool_eq(&register_id.clone(), &(*context).scfia.new_bv_concrete(0, 4), None, false, &mut (*context).fork_sink, None), &mut (*context).fork_sink) {
        return (*state).R0.clone();
    } else if (*context).scfia.check_condition(&(*context).scfia.new_bool_eq(&register_id.clone(), &(*context).scfia.new_bv_concrete(1, 4), None, false, &mut (*context).fork_sink, None), &mut (*context).fork_sink) {
        return (*state).R1.clone();
    } else if (*context).scfia.check_condition(&(*context).scfia.new_bool_eq(&register_id.clone(), &(*context).scfia.new_bv_concrete(2, 4), None, false, &mut (*context).fork_sink, None), &mut (*context).fork_sink) {
        return (*state).R2.clone();
    } else if (*context).scfia.check_condition(&(*context).scfia.new_bool_eq(&register_id.clone(), &(*context).scfia.new_bv_concrete(3, 4), None, false, &mut (*context).fork_sink, None), &mut (*context).fork_sink) {
        return (*state).R3.clone();
    } else if (*context).scfia.check_condition(&(*context).scfia.new_bool_eq(&register_id.clone(), &(*context).scfia.new_bv_concrete(4, 4), None, false, &mut (*context).fork_sink, None), &mut (*context).fork_sink) {
        return (*state).R4.clone();
    } else if (*context).scfia.check_condition(&(*context).scfia.new_bool_eq(&register_id.clone(), &(*context).scfia.new_bv_concrete(5, 4), None, false, &mut (*context).fork_sink, None), &mut (*context).fork_sink) {
        return (*state).R5.clone();
    } else if (*context).scfia.check_condition(&(*context).scfia.new_bool_eq(&register_id.clone(), &(*context).scfia.new_bv_concrete(6, 4), None, false, &mut (*context).fork_sink, None), &mut (*context).fork_sink) {
        return (*state).R6.clone();
    } else if (*context).scfia.check_condition(&(*context).scfia.new_bool_eq(&register_id.clone(), &(*context).scfia.new_bv_concrete(7, 4), None, false, &mut (*context).fork_sink, None), &mut (*context).fork_sink) {
        return (*state).R7.clone();
    } else if (*context).scfia.check_condition(&(*context).scfia.new_bool_eq(&register_id.clone(), &(*context).scfia.new_bv_concrete(8, 4), None, false, &mut (*context).fork_sink, None), &mut (*context).fork_sink) {
        return (*state).R8.clone();
    } else if (*context).scfia.check_condition(&(*context).scfia.new_bool_eq(&register_id.clone(), &(*context).scfia.new_bv_concrete(9, 4), None, false, &mut (*context).fork_sink, None), &mut (*context).fork_sink) {
        return (*state).R9.clone();
    } else if (*context).scfia.check_condition(&(*context).scfia.new_bool_eq(&register_id.clone(), &(*context).scfia.new_bv_concrete(10, 4), None, false, &mut (*context).fork_sink, None), &mut (*context).fork_sink) {
        return (*state).R10.clone();
    } else if (*context).scfia.check_condition(&(*context).scfia.new_bool_eq(&register_id.clone(), &(*context).scfia.new_bv_concrete(11, 4), None, false, &mut (*context).fork_sink, None), &mut (*context).fork_sink) {
        return (*state).R11.clone();
    } else if (*context).scfia.check_condition(&(*context).scfia.new_bool_eq(&register_id.clone(), &(*context).scfia.new_bv_concrete(12, 4), None, false, &mut (*context).fork_sink, None), &mut (*context).fork_sink) {
        return (*state).R12.clone();
    } else if (*context).scfia.check_condition(&(*context).scfia.new_bool_eq(&register_id.clone(), &(*context).scfia.new_bv_concrete(13, 4), None, false, &mut (*context).fork_sink, None), &mut (*context).fork_sink) {
        return (*state).SP.clone();
    } else if (*context).scfia.check_condition(&(*context).scfia.new_bool_eq(&register_id.clone(), &(*context).scfia.new_bv_concrete(14, 4), None, false, &mut (*context).fork_sink, None), &mut (*context).fork_sink) {
        return (*state).LR.clone();
    } else {
        return (*state).PC.clone();
    }
}

unsafe fn _progress_pc_2(mut state: *mut SystemState, context: *mut StepContext<ARMv7MScfiaComposition>) {
    let mut old_pc: ActiveValue<ARMv7MScfiaComposition> = (*state).PC.clone();
    let mut new_pc: ActiveValue<ARMv7MScfiaComposition> = (*context).scfia.new_bv_add(&old_pc.clone(), &(*context).scfia.new_bv_concrete(0x02, 32), 32, None, &mut (*context).fork_sink, None);
    (*state).PC = new_pc.clone();
}

unsafe fn _progress_pc_4(mut state: *mut SystemState, context: *mut StepContext<ARMv7MScfiaComposition>) {
    let mut old_pc: ActiveValue<ARMv7MScfiaComposition> = (*state).PC.clone();
    let mut new_pc: ActiveValue<ARMv7MScfiaComposition> = (*context).scfia.new_bv_add(&old_pc.clone(), &(*context).scfia.new_bv_concrete(0x04, 32), 32, None, &mut (*context).fork_sink, None);
    (*state).PC = new_pc.clone();
}

unsafe fn _add_with_carry_BV32(mut x: ActiveValue<ARMv7MScfiaComposition>, mut y: ActiveValue<ARMv7MScfiaComposition>, mut carry_in: ActiveValue<ARMv7MScfiaComposition>, context: *mut StepContext<ARMv7MScfiaComposition>) -> (ActiveValue<ARMv7MScfiaComposition>, ActiveValue<ARMv7MScfiaComposition>, ActiveValue<ARMv7MScfiaComposition>) {
    let mut unsigned_sum: ActiveValue<ARMv7MScfiaComposition> = (*context).scfia.new_bv_add(&(*context).scfia.new_bv_add(&(*context).scfia.new_bv_concat(&(*context).scfia.new_bv_concrete(0, 2), &x.clone(), 34, None, &mut (*context).fork_sink, None), &(*context).scfia.new_bv_concat(&(*context).scfia.new_bv_concrete(0, 2), &y.clone(), 34, None, &mut (*context).fork_sink, None), 34, None, &mut (*context).fork_sink, None), &(*context).scfia.new_bv_concat(&(*context).scfia.new_bv_concrete(0, 33), &carry_in.clone(), 34, None, &mut (*context).fork_sink, None), 34, None, &mut (*context).fork_sink, None);
    let mut signed_sum: ActiveValue<ARMv7MScfiaComposition> = (*context).scfia.new_bv_add(&(*context).scfia.new_bv_add(&(*context).scfia.new_bv_sign_extend(&x.clone(), 32, 34, None, &mut (*context).fork_sink, None), &(*context).scfia.new_bv_sign_extend(&y.clone(), 32, 34, None, &mut (*context).fork_sink, None), 34, None, &mut (*context).fork_sink, None), &(*context).scfia.new_bv_concat(&(*context).scfia.new_bv_concrete(0, 33), &carry_in.clone(), 34, None, &mut (*context).fork_sink, None), 34, None, &mut (*context).fork_sink, None);
    let mut result: ActiveValue<ARMv7MScfiaComposition> = (*context).scfia.new_bv_slice(&unsigned_sum.clone(), 31, 0, None, &mut (*context).fork_sink, None);
    let mut carry_out: ActiveValue<ARMv7MScfiaComposition> = _is_not_eq_BV34((*context).scfia.new_bv_concat(&(*context).scfia.new_bv_concrete(0, 2), &result.clone(), 34, None, &mut (*context).fork_sink, None), unsigned_sum.clone(), context);
    let mut overflow: ActiveValue<ARMv7MScfiaComposition> = _is_not_eq_BV34((*context).scfia.new_bv_sign_extend(&result.clone(), 32, 34, None, &mut (*context).fork_sink, None), unsigned_sum.clone(), context);
    return (result.clone(), carry_out.clone(), overflow.clone());
}

unsafe fn _thumb_expand_imm(mut imm12: ActiveValue<ARMv7MScfiaComposition>, mut state: *mut SystemState, context: *mut StepContext<ARMv7MScfiaComposition>) -> ActiveValue<ARMv7MScfiaComposition> {
    // "APSR.C argument to following function call does not affect the imm32 result."
    let (mut imm32, mut _carry_out) = _thumb_expand_imm_c(imm12.clone(), (*state).APSR.C.clone(), context);
    return imm32.clone();
}

unsafe fn _thumb_expand_imm_c(mut imm12: ActiveValue<ARMv7MScfiaComposition>, mut carry_in: ActiveValue<ARMv7MScfiaComposition>, context: *mut StepContext<ARMv7MScfiaComposition>) -> (ActiveValue<ARMv7MScfiaComposition>, ActiveValue<ARMv7MScfiaComposition>) {
    let mut imm32: ActiveValue<ARMv7MScfiaComposition> = (*context).scfia.new_bv_concrete(0, 32);
    let mut carry_out: ActiveValue<ARMv7MScfiaComposition> = (*context).scfia.new_bv_concrete(0, 1);
    if (*context).scfia.check_condition(&(*context).scfia.new_bool_eq(&(*context).scfia.new_bv_slice(&imm12.clone(), 11, 10, None, &mut (*context).fork_sink, None), &(*context).scfia.new_bv_concrete(0b00, 2), None, false, &mut (*context).fork_sink, None), &mut (*context).fork_sink) {
        let mut mode: ActiveValue<ARMv7MScfiaComposition> = (*context).scfia.new_bv_slice(&imm12.clone(), 9, 8, None, &mut (*context).fork_sink, None);
        if (*context).scfia.check_condition(&(*context).scfia.new_bool_eq(&mode.clone(), &(*context).scfia.new_bv_concrete(0b00, 2), None, false, &mut (*context).fork_sink, None), &mut (*context).fork_sink) {
            imm32 = (*context).scfia.new_bv_concat(&(*context).scfia.new_bv_concrete(0, 24), &(*context).scfia.new_bv_slice(&imm12.clone(), 7, 0, None, &mut (*context).fork_sink, None), 32, None, &mut (*context).fork_sink, None);
        } else if (*context).scfia.check_condition(&(*context).scfia.new_bool_eq(&mode.clone(), &(*context).scfia.new_bv_concrete(0b01, 2), None, false, &mut (*context).fork_sink, None), &mut (*context).fork_sink) {
            unimplemented!();
        } else if (*context).scfia.check_condition(&(*context).scfia.new_bool_eq(&mode.clone(), &(*context).scfia.new_bv_concrete(0b10, 2), None, false, &mut (*context).fork_sink, None), &mut (*context).fork_sink) {
            unimplemented!();
        } else {
            let mut imm8: ActiveValue<ARMv7MScfiaComposition> = (*context).scfia.new_bv_slice(&imm12.clone(), 7, 0, None, &mut (*context).fork_sink, None);
            if (*context).scfia.check_condition(&(*context).scfia.new_bool_eq(&imm8.clone(), &(*context).scfia.new_bv_concrete(0, 8), None, false, &mut (*context).fork_sink, None), &mut (*context).fork_sink) {
                // TODO unpredictable
                unimplemented!();
            } else {
                let mut imm16: ActiveValue<ARMv7MScfiaComposition> = (*context).scfia.new_bv_concat(&imm8.clone(), &imm8.clone(), 16, None, &mut (*context).fork_sink, None);
                let mut imm24: ActiveValue<ARMv7MScfiaComposition> = (*context).scfia.new_bv_concat(&imm16.clone(), &imm8.clone(), 24, None, &mut (*context).fork_sink, None);
                imm32 = (*context).scfia.new_bv_concat(&imm24.clone(), &imm8.clone(), 32, None, &mut (*context).fork_sink, None);
            }
        }
        carry_out = carry_in.clone();
    } else {
        let mut imm7: ActiveValue<ARMv7MScfiaComposition> = (*context).scfia.new_bv_slice(&imm12.clone(), 6, 0, None, &mut (*context).fork_sink, None);
        let mut imm8: ActiveValue<ARMv7MScfiaComposition> = (*context).scfia.new_bv_concat(&(*context).scfia.new_bv_concrete(1, 1), &imm7.clone(), 8, None, &mut (*context).fork_sink, None);
        let mut unrotated_value: ActiveValue<ARMv7MScfiaComposition> = (*context).scfia.new_bv_concat(&(*context).scfia.new_bv_concrete(0, 24), &imm8.clone(), 32, None, &mut (*context).fork_sink, None);
        let mut shift5: ActiveValue<ARMv7MScfiaComposition> = (*context).scfia.new_bv_slice(&imm12.clone(), 11, 7, None, &mut (*context).fork_sink, None);
        let mut result_carry: ActiveValue<ARMv7MScfiaComposition> = _ror_C_BV32(unrotated_value.clone(), (*context).scfia.new_bv_concat(&(*context).scfia.new_bv_concrete(0, 27), &shift5.clone(), 32, None, &mut (*context).fork_sink, None), context);
        imm32 = (*context).scfia.new_bv_slice(&result_carry.clone(), 32, 1, None, &mut (*context).fork_sink, None);
        carry_out = (*context).scfia.new_bv_slice(&result_carry.clone(), 0, 0, None, &mut (*context).fork_sink, None);
    }
    return (imm32.clone(), carry_out.clone());
}

unsafe fn _ror_C_BV32(mut x: ActiveValue<ARMv7MScfiaComposition>, mut shift: ActiveValue<ARMv7MScfiaComposition>, context: *mut StepContext<ARMv7MScfiaComposition>) -> ActiveValue<ARMv7MScfiaComposition> {
    if (*context).scfia.check_condition(&(*context).scfia.new_bool_eq(&shift.clone(), &(*context).scfia.new_bv_concrete(0, 32), None, false, &mut (*context).fork_sink, None), &mut (*context).fork_sink) {
        unimplemented!();
        //TODO assert
    }
    let mut m: ActiveValue<ARMv7MScfiaComposition> = (*context).scfia.new_bv_unsigned_remainder(&shift.clone(), &(*context).scfia.new_bv_concrete(32, 32), 32, None, &mut (*context).fork_sink, None);
    let mut i1: ActiveValue<ARMv7MScfiaComposition> = (*context).scfia.new_bv_sll(&x.clone(), &m.clone(), 32, None, &mut (*context).fork_sink, None);
    let mut i2: ActiveValue<ARMv7MScfiaComposition> = (*context).scfia.new_bv_sll(&x.clone(), &(*context).scfia.new_bv_sub(&(*context).scfia.new_bv_concrete(31, 32), &m.clone(), 32, None, &mut (*context).fork_sink, None), 32, None, &mut (*context).fork_sink, None);
    let mut result: ActiveValue<ARMv7MScfiaComposition> = (*context).scfia.new_bv_or(&i1.clone(), &i2.clone(), 32, None, &mut (*context).fork_sink, None);
    let mut carry_out: ActiveValue<ARMv7MScfiaComposition> = (*context).scfia.new_bv_slice(&result.clone(), 31, 31, None, &mut (*context).fork_sink, None);
    return (*context).scfia.new_bv_concat(&result.clone(), &carry_out.clone(), 33, None, &mut (*context).fork_sink, None);
}

unsafe fn _is_not_eq_BV34(mut x: ActiveValue<ARMv7MScfiaComposition>, mut y: ActiveValue<ARMv7MScfiaComposition>, context: *mut StepContext<ARMv7MScfiaComposition>) -> ActiveValue<ARMv7MScfiaComposition> {
    if (*context).scfia.check_condition(&(*context).scfia.new_bool_eq(&x.clone(), &y.clone(), None, false, &mut (*context).fork_sink, None), &mut (*context).fork_sink) {
        return (*context).scfia.new_bv_concrete(0, 1);
    } else {
        return (*context).scfia.new_bv_concrete(1, 1);
    }
}

unsafe fn _is_zero_bit_BV32(mut value: ActiveValue<ARMv7MScfiaComposition>, context: *mut StepContext<ARMv7MScfiaComposition>) -> ActiveValue<ARMv7MScfiaComposition> {
    //if (EQUALS_BV32(value, BV32(0))) {
    //    return BV1(1);
    //} else {
    //    return BV1(0);
    //}
    let mut t: ActiveValue<ARMv7MScfiaComposition> = (*context).scfia.new_bv_concrete(0, 1);
    for i in 0u64..=31 {
        t = (*context).scfia.new_bv_or(&t.clone(), &(*context).scfia.new_bv_slice(&value.clone(), i as u32, (i as u32+1-1), None, &mut (*context).fork_sink, None), 1, None, &mut (*context).fork_sink, None);
    }
    return (*context).scfia.new_bv_not(&t.clone(), 1, None, &mut (*context).fork_sink, None);
}

unsafe fn _condition_passed(mut condition: ActiveValue<ARMv7MScfiaComposition>, mut state: *mut SystemState, context: *mut StepContext<ARMv7MScfiaComposition>) -> ActiveValue<ARMv7MScfiaComposition> {
    let mut cond: ActiveValue<ARMv7MScfiaComposition> = (*context).scfia.new_bv_slice(&condition.clone(), 3, 1, None, &mut (*context).fork_sink, None);
    let mut result: ActiveValue<ARMv7MScfiaComposition> = (*context).scfia.new_bv_concrete(0, 1);
    if (*context).scfia.check_condition(&(*context).scfia.new_bool_eq(&cond.clone(), &(*context).scfia.new_bv_concrete(0b000, 3), None, false, &mut (*context).fork_sink, None), &mut (*context).fork_sink) {
        result = (*state).APSR.Z.clone();
    } else if (*context).scfia.check_condition(&(*context).scfia.new_bool_eq(&cond.clone(), &(*context).scfia.new_bv_concrete(0b001, 3), None, false, &mut (*context).fork_sink, None), &mut (*context).fork_sink) {
        result = (*state).APSR.C.clone();
    } else if (*context).scfia.check_condition(&(*context).scfia.new_bool_eq(&cond.clone(), &(*context).scfia.new_bv_concrete(0b010, 3), None, false, &mut (*context).fork_sink, None), &mut (*context).fork_sink) {
        result = (*state).APSR.N.clone();
    } else if (*context).scfia.check_condition(&(*context).scfia.new_bool_eq(&cond.clone(), &(*context).scfia.new_bv_concrete(0b011, 3), None, false, &mut (*context).fork_sink, None), &mut (*context).fork_sink) {
        result = (*state).APSR.V.clone();
    } else if (*context).scfia.check_condition(&(*context).scfia.new_bool_eq(&cond.clone(), &(*context).scfia.new_bv_concrete(0b100, 3), None, false, &mut (*context).fork_sink, None), &mut (*context).fork_sink) {
        unimplemented!();
    } else if (*context).scfia.check_condition(&(*context).scfia.new_bool_eq(&cond.clone(), &(*context).scfia.new_bv_concrete(0b101, 3), None, false, &mut (*context).fork_sink, None), &mut (*context).fork_sink) {
        unimplemented!();
    } else if (*context).scfia.check_condition(&(*context).scfia.new_bool_eq(&cond.clone(), &(*context).scfia.new_bv_concrete(0b110, 3), None, false, &mut (*context).fork_sink, None), &mut (*context).fork_sink) {
        unimplemented!();
    } else if (*context).scfia.check_condition(&(*context).scfia.new_bool_eq(&cond.clone(), &(*context).scfia.new_bv_concrete(0b111, 3), None, false, &mut (*context).fork_sink, None), &mut (*context).fork_sink) {
        unimplemented!();
    }
    let mut wat: ActiveValue<ARMv7MScfiaComposition> = (*context).scfia.new_bv_slice(&condition.clone(), 0, 0, None, &mut (*context).fork_sink, None);
    //TODO isn't this more complex than it should be?
    if (*context).scfia.check_condition(&(*context).scfia.new_bool_eq(&wat.clone(), &(*context).scfia.new_bv_concrete(1, 1), None, false, &mut (*context).fork_sink, None), &mut (*context).fork_sink) {
        if (*context).scfia.check_condition(&(*context).scfia.new_bool_not(&(*context).scfia.new_bool_eq(&condition.clone(), &(*context).scfia.new_bv_concrete(0b1111, 4), None, false, &mut (*context).fork_sink, None), None, false, &mut (*context).fork_sink, None), &mut (*context).fork_sink) {
            result = (*context).scfia.new_bv_not(&result.clone(), 1, None, &mut (*context).fork_sink, None);
        }
    }
    return result.clone();
}

unsafe fn _load_register(mut t: ActiveValue<ARMv7MScfiaComposition>, mut imm32: ActiveValue<ARMv7MScfiaComposition>, mut add: ActiveValue<ARMv7MScfiaComposition>, mut state: *mut SystemState, context: *mut StepContext<ARMv7MScfiaComposition>) {
    let mut base: ActiveValue<ARMv7MScfiaComposition> = (*context).scfia.new_bv_and(&(*state).PC.clone(), &(*context).scfia.new_bv_not(&(*context).scfia.new_bv_concrete(0b11, 32), 32, None, &mut (*context).fork_sink, None), 32, None, &mut (*context).fork_sink, None);
    if (*context).scfia.check_condition(&(*context).scfia.new_bool_eq(&add.clone(), &(*context).scfia.new_bv_concrete(1, 1), None, false, &mut (*context).fork_sink, None), &mut (*context).fork_sink) {
        let mut address: ActiveValue<ARMv7MScfiaComposition> = (*context).scfia.new_bv_add(&base.clone(), &imm32.clone(), 32, None, &mut (*context).fork_sink, None);
        //TODO t==15 unpredictable foo
        _register_write_BV32(t.clone(), (*(*context).memory).read(&address.clone(), 32, (*context).scfia.clone(), &mut (*context).hints, &mut (*context).fork_sink), state, context);
    } else {
        let mut address: ActiveValue<ARMv7MScfiaComposition> = (*context).scfia.new_bv_sub(&base.clone(), &imm32.clone(), 32, None, &mut (*context).fork_sink, None);
        _register_write_BV32(t.clone(), (*(*context).memory).read(&address.clone(), 32, (*context).scfia.clone(), &mut (*context).hints, &mut (*context).fork_sink), state, context);
    }
}

unsafe fn _branch_write_pc(mut address: ActiveValue<ARMv7MScfiaComposition>, mut state: *mut SystemState, context: *mut StepContext<ARMv7MScfiaComposition>) {
    let mut address: ActiveValue<ARMv7MScfiaComposition> = (*context).scfia.new_bv_concat(&(*context).scfia.new_bv_slice(&address.clone(), 31, 1, None, &mut (*context).fork_sink, None), &(*context).scfia.new_bv_concrete(0, 1), 32, None, &mut (*context).fork_sink, None);
    _branch_to(address.clone(), state, context);
}

unsafe fn _bx_write_pc(mut address: ActiveValue<ARMv7MScfiaComposition>, mut state: *mut SystemState, context: *mut StepContext<ARMv7MScfiaComposition>) {
    // TODO currentmode address ExceptionReturn
    // TODO usage fault is taken on the next instruction?
    (*state).EPSR.T = (*context).scfia.new_bv_slice(&address.clone(), 0, 0, None, &mut (*context).fork_sink, None);
    _branch_to((*context).scfia.new_bv_concat(&(*context).scfia.new_bv_slice(&address.clone(), 31, 1, None, &mut (*context).fork_sink, None), &(*context).scfia.new_bv_concrete(0, 1), 32, None, &mut (*context).fork_sink, None), state, context);
}

unsafe fn _blx_write_pc(mut address: ActiveValue<ARMv7MScfiaComposition>, mut state: *mut SystemState, context: *mut StepContext<ARMv7MScfiaComposition>) {
    (*state).EPSR.T = (*context).scfia.new_bv_slice(&address.clone(), 0, 0, None, &mut (*context).fork_sink, None);
    _branch_to((*context).scfia.new_bv_concat(&(*context).scfia.new_bv_slice(&address.clone(), 31, 1, None, &mut (*context).fork_sink, None), &(*context).scfia.new_bv_concrete(0, 1), 32, None, &mut (*context).fork_sink, None), state, context);
}

unsafe fn _branch_to(mut address: ActiveValue<ARMv7MScfiaComposition>, mut state: *mut SystemState, context: *mut StepContext<ARMv7MScfiaComposition>) {
    (*state).PC = address.clone();
}

unsafe fn _shift_c(mut value: ActiveValue<ARMv7MScfiaComposition>, mut srtype: SRType, mut amount: ActiveValue<ARMv7MScfiaComposition>, mut carry_in: ActiveValue<ARMv7MScfiaComposition>, context: *mut StepContext<ARMv7MScfiaComposition>) -> (ActiveValue<ARMv7MScfiaComposition>, ActiveValue<ARMv7MScfiaComposition>) {
    //TODO assert rrx amount
    let mut result: ActiveValue<ARMv7MScfiaComposition> = (*context).scfia.new_bv_concrete(0, 32);
    let mut carry_out: ActiveValue<ARMv7MScfiaComposition> = (*context).scfia.new_bv_concrete(0, 1);
    if (*context).scfia.check_condition(&(*context).scfia.new_bool_eq(&amount.clone(), &(*context).scfia.new_bv_concrete(0, 5), None, false, &mut (*context).fork_sink, None), &mut (*context).fork_sink) {
        result = value.clone();
        carry_out = carry_in.clone();
    } else {
        if (*context).scfia.check_condition(&(*context).scfia.new_bool_concrete(srtype == SRType::LSL, None, &mut (*context).fork_sink), &mut (*context).fork_sink) {
            let (mut lsl_result, mut lsl_carry) = _lsl_c(value.clone(), amount.clone(), context);
            result = lsl_result.clone();
            carry_out = lsl_carry.clone();
        } else if (*context).scfia.check_condition(&(*context).scfia.new_bool_concrete(srtype == SRType::LSR, None, &mut (*context).fork_sink), &mut (*context).fork_sink) {
            let (mut lsr_result, mut lsr_carry) = _lsr_c(value.clone(), amount.clone(), context);
            result = lsr_result.clone();
            carry_out = lsr_carry.clone();
        } else if (*context).scfia.check_condition(&(*context).scfia.new_bool_concrete(srtype == SRType::ASR, None, &mut (*context).fork_sink), &mut (*context).fork_sink) {
            let (mut asr_result, mut asr_carry) = _asr_c(value.clone(), amount.clone(), context);
            result = asr_result.clone();
            carry_out = asr_carry.clone();
        } else if (*context).scfia.check_condition(&(*context).scfia.new_bool_concrete(srtype == SRType::ROR, None, &mut (*context).fork_sink), &mut (*context).fork_sink) {
            unimplemented!();
        } else if (*context).scfia.check_condition(&(*context).scfia.new_bool_concrete(srtype == SRType::RRX, None, &mut (*context).fork_sink), &mut (*context).fork_sink) {
            unimplemented!();
        }
    }
    return (result.clone(), carry_out.clone());
}

unsafe fn _lsl_c(mut bits: ActiveValue<ARMv7MScfiaComposition>, mut shift: ActiveValue<ARMv7MScfiaComposition>, context: *mut StepContext<ARMv7MScfiaComposition>) -> (ActiveValue<ARMv7MScfiaComposition>, ActiveValue<ARMv7MScfiaComposition>) {
    //TODO assert >0
    let mut result: ActiveValue<ARMv7MScfiaComposition> = (*context).scfia.new_bv_srl(&bits.clone(), &(*context).scfia.new_bv_concat(&(*context).scfia.new_bv_concrete(0, 27), &shift.clone(), 32, None, &mut (*context).fork_sink, None), 32, None, &mut (*context).fork_sink, None);
    let mut carry: ActiveValue<ARMv7MScfiaComposition> = (*context).scfia.new_bv_slice(&(*context).scfia.new_bv_srl(&bits.clone(), &(*context).scfia.new_bv_concat(&(*context).scfia.new_bv_concrete(0, 27), &(*context).scfia.new_bv_sub(&shift.clone(), &(*context).scfia.new_bv_concrete(0x01, 5), 5, None, &mut (*context).fork_sink, None), 32, None, &mut (*context).fork_sink, None), 32, None, &mut (*context).fork_sink, None), 31, 31, None, &mut (*context).fork_sink, None);
    return (result.clone(), carry.clone());
}

unsafe fn _lsr_c(mut bits: ActiveValue<ARMv7MScfiaComposition>, mut shift: ActiveValue<ARMv7MScfiaComposition>, context: *mut StepContext<ARMv7MScfiaComposition>) -> (ActiveValue<ARMv7MScfiaComposition>, ActiveValue<ARMv7MScfiaComposition>) {
    //TODO assert >0
    let mut result: ActiveValue<ARMv7MScfiaComposition> = (*context).scfia.new_bv_srl(&bits.clone(), &(*context).scfia.new_bv_concat(&(*context).scfia.new_bv_concrete(0, 27), &shift.clone(), 32, None, &mut (*context).fork_sink, None), 32, None, &mut (*context).fork_sink, None);
    let mut carry: ActiveValue<ARMv7MScfiaComposition> = (*context).scfia.new_bv_slice(&(*context).scfia.new_bv_sll(&bits.clone(), &(*context).scfia.new_bv_concat(&(*context).scfia.new_bv_concrete(0, 27), &(*context).scfia.new_bv_sub(&shift.clone(), &(*context).scfia.new_bv_concrete(0x01, 5), 5, None, &mut (*context).fork_sink, None), 32, None, &mut (*context).fork_sink, None), 32, None, &mut (*context).fork_sink, None), 0, 0, None, &mut (*context).fork_sink, None);
    return (result.clone(), carry.clone());
}

unsafe fn _asr_c(mut bits: ActiveValue<ARMv7MScfiaComposition>, mut shift: ActiveValue<ARMv7MScfiaComposition>, context: *mut StepContext<ARMv7MScfiaComposition>) -> (ActiveValue<ARMv7MScfiaComposition>, ActiveValue<ARMv7MScfiaComposition>) {
    //TODO convert to int, assert shift>0?
    return (*context).scfia.new_bv_asr(&bits.clone(), &(*context).scfia.new_bv_sign_extend(&shift.clone(), 32, 5, None, &mut (*context).fork_sink, None), 32, None, &mut (*context).fork_sink, None);
}

unsafe fn _decode_imm_shift(mut shift_type: ActiveValue<ARMv7MScfiaComposition>, mut imm5: ActiveValue<ARMv7MScfiaComposition>, context: *mut StepContext<ARMv7MScfiaComposition>) -> (SRType, ActiveValue<ARMv7MScfiaComposition>) {
    let mut shift_t: SRType = SRType::None;
    let mut imm: ActiveValue<ARMv7MScfiaComposition> = (*context).scfia.new_bv_concrete(0, 5);
    if (*context).scfia.check_condition(&(*context).scfia.new_bool_eq(&shift_type.clone(), &(*context).scfia.new_bv_concrete(0b00, 2), None, false, &mut (*context).fork_sink, None), &mut (*context).fork_sink) {
        shift_t = SRType::LSL;
        imm = imm5.clone();
    } else if (*context).scfia.check_condition(&(*context).scfia.new_bool_eq(&shift_type.clone(), &(*context).scfia.new_bv_concrete(0b01, 2), None, false, &mut (*context).fork_sink, None), &mut (*context).fork_sink) {
        shift_t = SRType::LSR;
        if (*context).scfia.check_condition(&(*context).scfia.new_bool_eq(&imm5.clone(), &(*context).scfia.new_bv_concrete(0b00000, 5), None, false, &mut (*context).fork_sink, None), &mut (*context).fork_sink) {
            imm = (*context).scfia.new_bv_concrete(32, 5);
        } else {
            imm = imm5.clone();
        }
    } else if (*context).scfia.check_condition(&(*context).scfia.new_bool_eq(&shift_type.clone(), &(*context).scfia.new_bv_concrete(0b10, 2), None, false, &mut (*context).fork_sink, None), &mut (*context).fork_sink) {
        shift_t = SRType::ASR;
        if (*context).scfia.check_condition(&(*context).scfia.new_bool_eq(&imm5.clone(), &(*context).scfia.new_bv_concrete(0b00000, 5), None, false, &mut (*context).fork_sink, None), &mut (*context).fork_sink) {
            imm = (*context).scfia.new_bv_concrete(32, 5);
        } else {
            imm = imm5.clone();
        }
    } else {
        unimplemented!();
    }
    return (shift_t, imm.clone());
}

unsafe fn _matches_BV3(mut input: ActiveValue<ARMv7MScfiaComposition>, mut required_ones: ActiveValue<ARMv7MScfiaComposition>, mut required_zeroes: ActiveValue<ARMv7MScfiaComposition>, context: *mut StepContext<ARMv7MScfiaComposition>) -> ActiveValue<ARMv7MScfiaComposition> {
    if (*context).scfia.check_condition(&(*context).scfia.new_bool_eq(&(*context).scfia.new_bv_and(&input.clone(), &required_ones.clone(), 3, None, &mut (*context).fork_sink, None), &required_ones.clone(), None, false, &mut (*context).fork_sink, None), &mut (*context).fork_sink) {
        // all required ones are set!
        if (*context).scfia.check_condition(&(*context).scfia.new_bool_eq(&(*context).scfia.new_bv_and(&input.clone(), &required_zeroes.clone(), 3, None, &mut (*context).fork_sink, None), &(*context).scfia.new_bv_concrete(0, 3), None, false, &mut (*context).fork_sink, None), &mut (*context).fork_sink) {
            // all required zeroes are not set!
            return (*context).scfia.new_bv_concrete(1, 1);
        }
    }
    return (*context).scfia.new_bv_concrete(0, 1);
}

unsafe fn _matches_BV4(mut input: ActiveValue<ARMv7MScfiaComposition>, mut required_ones: ActiveValue<ARMv7MScfiaComposition>, mut required_zeroes: ActiveValue<ARMv7MScfiaComposition>, context: *mut StepContext<ARMv7MScfiaComposition>) -> ActiveValue<ARMv7MScfiaComposition> {
    if (*context).scfia.check_condition(&(*context).scfia.new_bool_eq(&(*context).scfia.new_bv_and(&input.clone(), &required_ones.clone(), 4, None, &mut (*context).fork_sink, None), &required_ones.clone(), None, false, &mut (*context).fork_sink, None), &mut (*context).fork_sink) {
        // all required ones are set!
        if (*context).scfia.check_condition(&(*context).scfia.new_bool_eq(&(*context).scfia.new_bv_and(&input.clone(), &required_zeroes.clone(), 4, None, &mut (*context).fork_sink, None), &(*context).scfia.new_bv_concrete(0, 4), None, false, &mut (*context).fork_sink, None), &mut (*context).fork_sink) {
            // all required zeroes are not set!
            return (*context).scfia.new_bv_concrete(1, 1);
        }
    }
    return (*context).scfia.new_bv_concrete(0, 1);
}

unsafe fn _matches_BV5(mut input: ActiveValue<ARMv7MScfiaComposition>, mut required_ones: ActiveValue<ARMv7MScfiaComposition>, mut required_zeroes: ActiveValue<ARMv7MScfiaComposition>, context: *mut StepContext<ARMv7MScfiaComposition>) -> ActiveValue<ARMv7MScfiaComposition> {
    if (*context).scfia.check_condition(&(*context).scfia.new_bool_eq(&(*context).scfia.new_bv_and(&input.clone(), &required_ones.clone(), 5, None, &mut (*context).fork_sink, None), &required_ones.clone(), None, false, &mut (*context).fork_sink, None), &mut (*context).fork_sink) {
        // all required ones are set!
        if (*context).scfia.check_condition(&(*context).scfia.new_bool_eq(&(*context).scfia.new_bv_and(&input.clone(), &required_zeroes.clone(), 5, None, &mut (*context).fork_sink, None), &(*context).scfia.new_bv_concrete(0, 5), None, false, &mut (*context).fork_sink, None), &mut (*context).fork_sink) {
            // all required zeroes are not set!
            return (*context).scfia.new_bv_concrete(1, 1);
        }
    }
    return (*context).scfia.new_bv_concrete(0, 1);
}

unsafe fn _matches_BV6(mut input: ActiveValue<ARMv7MScfiaComposition>, mut required_ones: ActiveValue<ARMv7MScfiaComposition>, mut required_zeroes: ActiveValue<ARMv7MScfiaComposition>, context: *mut StepContext<ARMv7MScfiaComposition>) -> ActiveValue<ARMv7MScfiaComposition> {
    if (*context).scfia.check_condition(&(*context).scfia.new_bool_eq(&(*context).scfia.new_bv_and(&input.clone(), &required_ones.clone(), 6, None, &mut (*context).fork_sink, None), &required_ones.clone(), None, false, &mut (*context).fork_sink, None), &mut (*context).fork_sink) {
        // all required ones are set!
        if (*context).scfia.check_condition(&(*context).scfia.new_bool_eq(&(*context).scfia.new_bv_and(&input.clone(), &required_zeroes.clone(), 6, None, &mut (*context).fork_sink, None), &(*context).scfia.new_bv_concrete(0, 6), None, false, &mut (*context).fork_sink, None), &mut (*context).fork_sink) {
            // all required zeroes are not set!
            return (*context).scfia.new_bv_concrete(1, 1);
        }
    }
    return (*context).scfia.new_bv_concrete(0, 1);
}

unsafe fn _matches_BV7(mut input: ActiveValue<ARMv7MScfiaComposition>, mut required_ones: ActiveValue<ARMv7MScfiaComposition>, mut required_zeroes: ActiveValue<ARMv7MScfiaComposition>, context: *mut StepContext<ARMv7MScfiaComposition>) -> ActiveValue<ARMv7MScfiaComposition> {
    if (*context).scfia.check_condition(&(*context).scfia.new_bool_eq(&(*context).scfia.new_bv_and(&input.clone(), &required_ones.clone(), 7, None, &mut (*context).fork_sink, None), &required_ones.clone(), None, false, &mut (*context).fork_sink, None), &mut (*context).fork_sink) {
        // all required ones are set!
        if (*context).scfia.check_condition(&(*context).scfia.new_bool_eq(&(*context).scfia.new_bv_and(&input.clone(), &required_zeroes.clone(), 7, None, &mut (*context).fork_sink, None), &(*context).scfia.new_bv_concrete(0, 7), None, false, &mut (*context).fork_sink, None), &mut (*context).fork_sink) {
            // all required zeroes are not set!
            return (*context).scfia.new_bv_concrete(1, 1);
        }
    }
    return (*context).scfia.new_bv_concrete(0, 1);
}

unsafe fn _bit_count_BV16(mut input: ActiveValue<ARMv7MScfiaComposition>, context: *mut StepContext<ARMv7MScfiaComposition>) -> ActiveValue<ARMv7MScfiaComposition> {
    let mut count: ActiveValue<ARMv7MScfiaComposition> = (*context).scfia.new_bv_concrete(0, 16);
    for i in 0u64..=15 {
        if (*context).scfia.check_condition(&(*context).scfia.new_bool_eq(&(*context).scfia.new_bv_concrete(1, 1), &(*context).scfia.new_bv_slice(&input.clone(), i as u32, (i as u32+1-1), None, &mut (*context).fork_sink, None), None, false, &mut (*context).fork_sink, None), &mut (*context).fork_sink) {
            count = (*context).scfia.new_bv_add(&count.clone(), &(*context).scfia.new_bv_concrete(1, 16), 16, None, &mut (*context).fork_sink, None);
        }
    }
    return count.clone();
}

