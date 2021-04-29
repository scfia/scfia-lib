use std::{
    borrow::Borrow,
    fmt::Display,
    fs::{self, File},
    io::{BufRead, BufReader, LineWriter},
    iter::successors,
    path::PathBuf,
    sync::{Arc, Mutex},
};

use log::{debug, info, trace};
use transformation_functions::aarch64::transformation_function_memory_write::ScfiaAarch64TransformationFunctionMemoryWrite;

use crate::{asserts::scfia_assert::ScfiaAssert, memory::ScfiaMemory, symbols::{scfia_symbol::ScfiaSymbolId, scfia_symbol_expression::ScfiaSymbolExpression, scfia_symbol_manager::ScfiaSymbolManager}, transformation_functions::{
        self,
        aarch64::{
            transformation_definition::ScfiaAarch64TransformationDefinition,
            transformation_function::ScfiaAarch64TransformationFunction,
        },
    }, values::{value1::Value1, value64::Value64}};

use crate::isla::aarch64::isla_handle::AARCH64_ISLA_HANDLE;
use crate::values::value8::Value8;

use super::ScfiaSystemState;
use std::io::Write;
use std::str::FromStr;

#[derive(Clone, Debug)]
pub struct ScfiaAarch64SystemState {
    pub symbol_manager: ScfiaSymbolManager,
    pub memory: ScfiaMemory,
    pub r0: Value64,
    pub r1: Value64,
    pub r2: Value64,
    pub r3: Value64,
    pub r4: Value64,
    pub r5: Value64,
    pub r6: Value64,
    pub r7: Value64,
    pub r8: Value64,
    pub r9: Value64,
    pub r10: Value64,
    pub r11: Value64,
    pub r12: Value64,
    pub r13: Value64,
    pub r14: Value64,
    pub r15: Value64,
    pub r16: Value64,
    pub r17: Value64,
    pub r18: Value64,
    pub r19: Value64,
    pub r20: Value64,
    pub r21: Value64,
    pub r22: Value64,
    pub r23: Value64,
    pub r24: Value64,
    pub r25: Value64,
    pub r26: Value64,
    pub r27: Value64,
    pub r28: Value64,
    pub fp: Value64,
    pub lr: Value64,
    pub sp_el3: Value64,
    pub pc: u64,
    pub pstate: Pstate,
    pub sctlr_el3: u64,
}

impl ScfiaAarch64SystemState {
    pub fn new() -> Self {
        let mut symbol_manager = ScfiaSymbolManager::new();
        let r0 = symbol_manager.create_symbol_bv(64);
        let r1 = symbol_manager.create_symbol_bv(64);
        let r2 = symbol_manager.create_symbol_bv(64);
        let r3 = symbol_manager.create_symbol_bv(64);
        let r4 = symbol_manager.create_symbol_bv(64);
        let r5 = symbol_manager.create_symbol_bv(64);
        let r6 = symbol_manager.create_symbol_bv(64);
        let r7 = symbol_manager.create_symbol_bv(64);
        let r8 = symbol_manager.create_symbol_bv(64);
        let r9 = symbol_manager.create_symbol_bv(64);
        let r10 = symbol_manager.create_symbol_bv(64);
        let r11 = symbol_manager.create_symbol_bv(64);
        let r12 = symbol_manager.create_symbol_bv(64);
        let r13 = symbol_manager.create_symbol_bv(64);
        let r14 = symbol_manager.create_symbol_bv(64);
        let r15 = symbol_manager.create_symbol_bv(64);
        let r16 = symbol_manager.create_symbol_bv(64);
        let r17 = symbol_manager.create_symbol_bv(64);
        let r18 = symbol_manager.create_symbol_bv(64);
        let r19 = symbol_manager.create_symbol_bv(64);
        let r20 = symbol_manager.create_symbol_bv(64);
        let r21 = symbol_manager.create_symbol_bv(64);
        let r22 = symbol_manager.create_symbol_bv(64);
        let r23 = symbol_manager.create_symbol_bv(64);
        let r24 = symbol_manager.create_symbol_bv(64);
        let r25 = symbol_manager.create_symbol_bv(64);
        let r26 = symbol_manager.create_symbol_bv(64);
        let r27 = symbol_manager.create_symbol_bv(64);
        let r28 = symbol_manager.create_symbol_bv(64);
        let fp = symbol_manager.create_symbol_bv(64);
        let lr = symbol_manager.create_symbol_bv(64);
        let sp_el3 = symbol_manager.create_symbol_bv(64);
        let pstate_n = symbol_manager.create_symbol_bv(1);
        let pstate_z = symbol_manager.create_symbol_bv(1);
        let pstate_c = symbol_manager.create_symbol_bv(1);
        let pstate_v = symbol_manager.create_symbol_bv(1);
        let pstate_pan = false;
        let pstate_ssbs = false;
        let pstate_tco = false;
        let pstate_uao = false;
        let pstate_i = true;
        let pstate_dit = false;
        let pstate_sp = true;
        let pstate_it = 0;
        let pstate_t = false;
        let pstate_f = true;
        let pstate_d = true;
        let pstate_nrw = false;
        let pstate_btype = 0;
        let pstate_j = false;
        let pstate_e = false;
        let pstate_el = 3;
        let pstate_q = false;
        let pstate_ss = false;
        let pstate_m = 0;
        let pstate_a = true;
        let pstate_il = false;
        ScfiaAarch64SystemState {
            symbol_manager: symbol_manager,
            memory: ScfiaMemory::new(),
            r0: Value64::Symbolic(r0),
            r1: Value64::Symbolic(r1),
            r2: Value64::Symbolic(r2),
            r3: Value64::Symbolic(r3),
            r4: Value64::Symbolic(r4),
            r5: Value64::Symbolic(r5),
            r6: Value64::Symbolic(r6),
            r7: Value64::Symbolic(r7),
            r8: Value64::Symbolic(r8),
            r9: Value64::Symbolic(r9),
            r10: Value64::Symbolic(r10),
            r11: Value64::Symbolic(r11),
            r12: Value64::Symbolic(r12),
            r13: Value64::Symbolic(r13),
            r14: Value64::Symbolic(r14),
            r15: Value64::Symbolic(r15),
            r16: Value64::Symbolic(r16),
            r17: Value64::Symbolic(r17),
            r18: Value64::Symbolic(r18),
            r19: Value64::Symbolic(r19),
            r20: Value64::Symbolic(r20),
            r21: Value64::Symbolic(r21),
            r22: Value64::Symbolic(r22),
            r23: Value64::Symbolic(r23),
            r24: Value64::Symbolic(r24),
            r25: Value64::Symbolic(r25),
            r26: Value64::Symbolic(r26),
            r27: Value64::Symbolic(r27),
            r28: Value64::Symbolic(r28),
            fp: Value64::Symbolic(fp),
            lr: Value64::Symbolic(lr),
            sp_el3: Value64::Symbolic(sp_el3),
            pc: 0,
            pstate: Pstate {
                n: Value1::Symbolic(pstate_n),
                z: Value1::Symbolic(pstate_z),
                c: Value1::Symbolic(pstate_c),
                v: Value1::Symbolic(pstate_v),
                pan: pstate_pan,
                ssbs: pstate_ssbs,
                tco: pstate_tco,
                uao: pstate_uao,
                i: pstate_i,
                dit: pstate_dit,
                sp: pstate_sp,
                it: pstate_it,
                t: pstate_t,
                f: pstate_f,
                d: pstate_d,
                nrw: pstate_nrw,
                btype: pstate_btype,
                j: pstate_j,
                e: pstate_e,
                el: pstate_el,
                q: pstate_q,
                ss: pstate_ss,
                m: pstate_m,
                a: pstate_a,
                il: pstate_il,
            },
            sctlr_el3: 0,
        }
    }

    pub fn apply_transformation_function(
        &self,
        transformation_function: &ScfiaAarch64TransformationFunction,
    ) -> Vec<ScfiaAarch64SystemState> {
        trace!("apply_transformation_function at 0x{:x}", self.pc);
        let mut successors = vec![];
        for successor_definition in &transformation_function.successors {
            let mut symbol_manager = self.symbol_manager.clone();
            let mut memory = self.memory.clone();
            let mut decremented_symbols = vec![];
            let r0 = if let Some(expression) = &successor_definition.new_r0 {
                match &self.r0 {
                    Value64::Symbolic(old_r0) => {
                        decremented_symbols.push(*old_r0);
                    }
                    _ => {}
                }
                expression.to_value_64(self, &mut symbol_manager)
            } else {
                self.r0.clone()
            };
            let r1 = if let Some(expression) = &successor_definition.new_r1 {
                match &self.r1 {
                    Value64::Symbolic(old_r1) => {
                        decremented_symbols.push(*old_r1);
                    }
                    _ => {}
                }
                expression.to_value_64(self, &mut symbol_manager)
            } else {
                self.r1.clone()
            };
            let r2 = if let Some(expression) = &successor_definition.new_r2 {
                match &self.r2 {
                    Value64::Symbolic(old_r2) => {
                        decremented_symbols.push(*old_r2);
                    }
                    _ => {}
                }
                expression.to_value_64(self, &mut symbol_manager)
            } else {
                self.r2.clone()
            };
            let r3 = if let Some(expression) = &successor_definition.new_r3 {
                match &self.r3 {
                    Value64::Symbolic(old_r3) => {
                        decremented_symbols.push(*old_r3);
                    }
                    _ => {}
                }
                expression.to_value_64(self, &mut symbol_manager)
            } else {
                self.r3.clone()
            };
            let r4 = if let Some(expression) = &successor_definition.new_r4 {
                match &self.r4 {
                    Value64::Symbolic(old_r4) => {
                        decremented_symbols.push(*old_r4);
                    }
                    _ => {}
                }
                expression.to_value_64(self, &mut symbol_manager)
            } else {
                self.r4.clone()
            };
            let r5 = if let Some(expression) = &successor_definition.new_r5 {
                match &self.r5 {
                    Value64::Symbolic(old_r5) => {
                        decremented_symbols.push(*old_r5);
                    }
                    _ => {}
                }
                expression.to_value_64(self, &mut symbol_manager)
            } else {
                self.r5.clone()
            };
            let r6 = if let Some(expression) = &successor_definition.new_r6 {
                match &self.r6 {
                    Value64::Symbolic(old_r6) => {
                        decremented_symbols.push(*old_r6);
                    }
                    _ => {}
                }
                expression.to_value_64(self, &mut symbol_manager)
            } else {
                self.r6.clone()
            };
            let r7 = if let Some(expression) = &successor_definition.new_r7 {
                match &self.r7 {
                    Value64::Symbolic(old_r7) => {
                        decremented_symbols.push(*old_r7);
                    }
                    _ => {}
                }
                expression.to_value_64(self, &mut symbol_manager)
            } else {
                self.r7.clone()
            };
            let r8 = if let Some(expression) = &successor_definition.new_r8 {
                match &self.r8 {
                    Value64::Symbolic(old_r8) => {
                        decremented_symbols.push(*old_r8);
                    }
                    _ => {}
                }
                expression.to_value_64(self, &mut symbol_manager)
            } else {
                self.r8.clone()
            };
            let r9 = if let Some(expression) = &successor_definition.new_r9 {
                match &self.r9 {
                    Value64::Symbolic(old_r9) => {
                        decremented_symbols.push(*old_r9);
                    }
                    _ => {}
                }
                expression.to_value_64(self, &mut symbol_manager)
            } else {
                self.r9.clone()
            };
            let r10 = if let Some(expression) = &successor_definition.new_r10 {
                match &self.r10 {
                    Value64::Symbolic(old_r10) => {
                        decremented_symbols.push(*old_r10);
                    }
                    _ => {}
                }
                expression.to_value_64(self, &mut symbol_manager)
            } else {
                self.r10.clone()
            };
            let r11 = if let Some(expression) = &successor_definition.new_r11 {
                match &self.r11 {
                    Value64::Symbolic(old_r11) => {
                        decremented_symbols.push(*old_r11);
                    }
                    _ => {}
                }
                expression.to_value_64(self, &mut symbol_manager)
            } else {
                self.r11.clone()
            };
            let r12 = if let Some(expression) = &successor_definition.new_r12 {
                match &self.r12 {
                    Value64::Symbolic(old_r12) => {
                        decremented_symbols.push(*old_r12);
                    }
                    _ => {}
                }
                expression.to_value_64(self, &mut symbol_manager)
            } else {
                self.r12.clone()
            };
            let r13 = if let Some(expression) = &successor_definition.new_r13 {
                match &self.r13 {
                    Value64::Symbolic(old_r13) => {
                        decremented_symbols.push(*old_r13);
                    }
                    _ => {}
                }
                expression.to_value_64(self, &mut symbol_manager)
            } else {
                self.r13.clone()
            };
            let r14 = if let Some(expression) = &successor_definition.new_r14 {
                match &self.r14 {
                    Value64::Symbolic(old_r14) => {
                        decremented_symbols.push(*old_r14);
                    }
                    _ => {}
                }
                expression.to_value_64(self, &mut symbol_manager)
            } else {
                self.r14.clone()
            };
            let r15 = if let Some(expression) = &successor_definition.new_r15 {
                match &self.r15 {
                    Value64::Symbolic(old_r15) => {
                        decremented_symbols.push(*old_r15);
                    }
                    _ => {}
                }
                expression.to_value_64(self, &mut symbol_manager)
            } else {
                self.r15.clone()
            };
            let r16 = if let Some(expression) = &successor_definition.new_r16 {
                match &self.r16 {
                    Value64::Symbolic(old_r16) => {
                        decremented_symbols.push(*old_r16);
                    }
                    _ => {}
                }
                expression.to_value_64(self, &mut symbol_manager)
            } else {
                self.r16.clone()
            };
            let r17 = if let Some(expression) = &successor_definition.new_r17 {
                match &self.r17 {
                    Value64::Symbolic(old_r17) => {
                        decremented_symbols.push(*old_r17);
                    }
                    _ => {}
                }
                expression.to_value_64(self, &mut symbol_manager)
            } else {
                self.r17.clone()
            };
            let r18 = if let Some(expression) = &successor_definition.new_r18 {
                match &self.r18 {
                    Value64::Symbolic(old_r18) => {
                        decremented_symbols.push(*old_r18);
                    }
                    _ => {}
                }
                expression.to_value_64(self, &mut symbol_manager)
            } else {
                self.r18.clone()
            };
            let r19 = if let Some(expression) = &successor_definition.new_r19 {
                match &self.r19 {
                    Value64::Symbolic(old_r19) => {
                        decremented_symbols.push(*old_r19);
                    }
                    _ => {}
                }
                expression.to_value_64(self, &mut symbol_manager)
            } else {
                self.r19.clone()
            };
            let r20 = if let Some(expression) = &successor_definition.new_r20 {
                match &self.r20 {
                    Value64::Symbolic(old_r20) => {
                        decremented_symbols.push(*old_r20);
                    }
                    _ => {}
                }
                expression.to_value_64(self, &mut symbol_manager)
            } else {
                self.r20.clone()
            };
            let r21 = if let Some(expression) = &successor_definition.new_r21 {
                match &self.r21 {
                    Value64::Symbolic(old_r21) => {
                        decremented_symbols.push(*old_r21);
                    }
                    _ => {}
                }
                expression.to_value_64(self, &mut symbol_manager)
            } else {
                self.r21.clone()
            };
            let r22 = if let Some(expression) = &successor_definition.new_r22 {
                match &self.r22 {
                    Value64::Symbolic(old_r22) => {
                        decremented_symbols.push(*old_r22);
                    }
                    _ => {}
                }
                expression.to_value_64(self, &mut symbol_manager)
            } else {
                self.r22.clone()
            };
            let r23 = if let Some(expression) = &successor_definition.new_r23 {
                match &self.r23 {
                    Value64::Symbolic(old_r23) => {
                        decremented_symbols.push(*old_r23);
                    }
                    _ => {}
                }
                expression.to_value_64(self, &mut symbol_manager)
            } else {
                self.r23.clone()
            };
            let r24 = if let Some(expression) = &successor_definition.new_r24 {
                match &self.r24 {
                    Value64::Symbolic(old_r24) => {
                        decremented_symbols.push(*old_r24);
                    }
                    _ => {}
                }
                expression.to_value_64(self, &mut symbol_manager)
            } else {
                self.r24.clone()
            };
            let r25 = if let Some(expression) = &successor_definition.new_r25 {
                match &self.r25 {
                    Value64::Symbolic(old_r25) => {
                        decremented_symbols.push(*old_r25);
                    }
                    _ => {}
                }
                expression.to_value_64(self, &mut symbol_manager)
            } else {
                self.r25.clone()
            };
            let r26 = if let Some(expression) = &successor_definition.new_r26 {
                match &self.r26 {
                    Value64::Symbolic(old_r26) => {
                        decremented_symbols.push(*old_r26);
                    }
                    _ => {}
                }
                expression.to_value_64(self, &mut symbol_manager)
            } else {
                self.r26.clone()
            };
            let r27 = if let Some(expression) = &successor_definition.new_r27 {
                match &self.r27 {
                    Value64::Symbolic(old_r27) => {
                        decremented_symbols.push(*old_r27);
                    }
                    _ => {}
                }
                expression.to_value_64(self, &mut symbol_manager)
            } else {
                self.r27.clone()
            };
            let r28 = if let Some(expression) = &successor_definition.new_r28 {
                match &self.r28 {
                    Value64::Symbolic(old_r28) => {
                        decremented_symbols.push(*old_r28);
                    }
                    _ => {}
                }
                expression.to_value_64(self, &mut symbol_manager)
            } else {
                self.r28.clone()
            };

            let fp = if let Some(expression) = &successor_definition.new_fp {
                match &self.fp {
                    Value64::Symbolic(old_fp) => {
                        decremented_symbols.push(*old_fp);
                    }
                    _ => {}
                }
                expression.to_value_64(self, &mut symbol_manager)
            } else {
                self.fp.clone()
            };

            let lr = if let Some(expression) = &successor_definition.new_lr {
                match &self.lr {
                    Value64::Symbolic(old_lr) => {
                        decremented_symbols.push(*old_lr);
                    }
                    _ => {}
                }
                expression.to_value_64(self, &mut symbol_manager)
            } else {
                self.lr.clone()
            };

            let sp_el3 = if let Some(expression) = &successor_definition.new_sp_el3 {
                match &self.sp_el3 {
                    Value64::Symbolic(old_sp_el3) => {
                        decremented_symbols.push(*old_sp_el3);
                    }
                    _ => {}
                }
                expression.to_value_64(self, &mut symbol_manager)
            } else {
                self.sp_el3.clone()
            };

            let pc = if let Some(pc) = &successor_definition.new_pc {
                *pc
            } else {
                self.pc
            };

            // PSTATE fields
            let pstate_n = if let Some(expression) = &successor_definition.new_pstate_n {
                match &self.pstate.n {
                    Value1::Symbolic(old_pstate_n) => {
                        decremented_symbols.push(*old_pstate_n);
                    }
                    _ => {}
                }
                expression.to_value_1(self, &mut symbol_manager)
            } else {
                self.pstate.n.clone()
            };
            let pstate_z = if let Some(expression) = &successor_definition.new_pstate_z {
                match &self.pstate.z {
                    Value1::Symbolic(old_pstate_z) => {
                        decremented_symbols.push(*old_pstate_z);
                    }
                    _ => {}
                }
                expression.to_value_1(self, &mut symbol_manager)
            } else {
                self.pstate.z.clone()
            };
            let pstate_c = if let Some(expression) = &successor_definition.new_pstate_c {
                match &self.pstate.c {
                    Value1::Symbolic(old_pstate_c) => {
                        decremented_symbols.push(*old_pstate_c);
                    }
                    _ => {}
                }
                expression.to_value_1(self, &mut symbol_manager)
            } else {
                self.pstate.c.clone()
            };
            let pstate_v = if let Some(expression) = &successor_definition.new_pstate_v {
                match &self.pstate.v {
                    Value1::Symbolic(old_pstate_v) => {
                        decremented_symbols.push(*old_pstate_v);
                    }
                    _ => {}
                }
                expression.to_value_1(self, &mut symbol_manager)
            } else {
                self.pstate.v.clone()
            };
            let pstate_pan = if let Some(pan) = &successor_definition.new_pstate_pan {
                *pan
            } else {
                self.pstate.pan
            };
            let pstate_ssbs = if let Some(ssbs) = &successor_definition.new_pstate_ssbs {
                *ssbs
            } else {
                self.pstate.ssbs
            };
            let pstate_tco = if let Some(tco) = &successor_definition.new_pstate_tco {
                *tco
            } else {
                self.pstate.tco
            };
            let pstate_uao = if let Some(uao) = &successor_definition.new_pstate_uao {
                *uao
            } else {
                self.pstate.uao
            };
            let pstate_i = if let Some(i) = &successor_definition.new_pstate_i {
                *i
            } else {
                self.pstate.i
            };
            let pstate_dit = if let Some(dit) = &successor_definition.new_pstate_dit {
                *dit
            } else {
                self.pstate.dit
            };
            let pstate_sp = if let Some(sp) = &successor_definition.new_pstate_sp {
                *sp
            } else {
                self.pstate.sp
            };
            let pstate_it = if let Some(it) = &successor_definition.new_pstate_it {
                *it
            } else {
                self.pstate.it
            };
            let pstate_t = if let Some(t) = &successor_definition.new_pstate_t {
                *t
            } else {
                self.pstate.t
            };
            let pstate_f = if let Some(f) = &successor_definition.new_pstate_f {
                *f
            } else {
                self.pstate.f
            };
            let pstate_d = if let Some(d) = &successor_definition.new_pstate_d {
                *d
            } else {
                self.pstate.d
            };
            let pstate_nrw = if let Some(nrw) = &successor_definition.new_pstate_nrw {
                *nrw
            } else {
                self.pstate.nrw
            };
            let pstate_btype = if let Some(btype) = &successor_definition.new_pstate_btype {
                *btype
            } else {
                self.pstate.btype
            };
            let pstate_j = if let Some(j) = &successor_definition.new_pstate_j {
                *j
            } else {
                self.pstate.j
            };
            let pstate_e = if let Some(e) = &successor_definition.new_pstate_e {
                *e
            } else {
                self.pstate.e
            };
            let pstate_el = if let Some(el) = &successor_definition.new_pstate_el {
                *el
            } else {
                self.pstate.el
            };
            let pstate_q = if let Some(q) = &successor_definition.new_pstate_q {
                *q
            } else {
                self.pstate.q
            };
            let pstate_ss = if let Some(ss) = &successor_definition.new_pstate_ss {
                *ss
            } else {
                self.pstate.ss
            };
            let pstate_m = if let Some(m) = &successor_definition.new_pstate_m {
                *m
            } else {
                self.pstate.m
            };
            let pstate_a = if let Some(a) = &successor_definition.new_pstate_a {
                *a
            } else {
                self.pstate.a
            };
            let pstate_il = if let Some(il) = &successor_definition.new_pstate_il {
                *il
            } else {
                self.pstate.il
            };

            let sctlr_el3 = if let Some(sctlr_el3) = &successor_definition.new_sctlr_el3 {
                *sctlr_el3
            } else {
                self.sctlr_el3
            };

            for memory_write in &successor_definition.memory_writes {
                match memory_write {
                    ScfiaAarch64TransformationFunctionMemoryWrite::WriteMemoryConcrete(
                        address,
                        value,
                    ) => {
                        memory.write(*address, Value8::Concrete(*value), &mut decremented_symbols);
                    }
                    ScfiaAarch64TransformationFunctionMemoryWrite::WriteMemorySymbolic(
                        address,
                        expression,
                    ) => {
                        memory.write(
                            *address,
                            expression.to_value_8(self, &mut symbol_manager),
                            &mut decremented_symbols,
                        );
                    }
                }
            }

            for assert in &successor_definition.new_asserts {
                info!("new assert: {:?}", &assert);
                let mut affected_symbol_ids = vec![];
                let scfia_assert_expression = assert.to_assert_expression(
                    &self,
                    &mut symbol_manager,
                    &mut affected_symbol_ids,
                );
                assert!(affected_symbol_ids.len() > 0);
                symbol_manager.add_assert(scfia_assert_expression, affected_symbol_ids);
            }

            for symbol in &decremented_symbols {
                symbol_manager.decrement_active_symbol_refcount(symbol)
            }

            let successor = ScfiaAarch64SystemState {
                symbol_manager: symbol_manager,
                memory: memory,
                r0: r0,
                r1: r1,
                r2: r2,
                r3: r3,
                r4: r4,
                r5: r5,
                r6: r6,
                r7: r7,
                r8: r8,
                r9: r9,
                r10: r10,
                r11: r11,
                r12: r12,
                r13: r13,
                r14: r14,
                r15: r15,
                r16: r16,
                r17: r17,
                r18: r18,
                r19: r19,
                r20: r20,
                r21: r21,
                r22: r22,
                r23: r23,
                r24: r24,
                r25: r25,
                r26: r26,
                r27: r27,
                r28: r28,
                fp: fp,
                lr: lr,
                sp_el3: sp_el3,
                pc: pc,
                pstate: Pstate {
                    n: pstate_n,
                    z: pstate_z,
                    c: pstate_c,
                    v: pstate_v,
                    pan: pstate_pan,
                    ssbs: pstate_ssbs,
                    tco: pstate_tco,
                    uao: pstate_uao,
                    i: pstate_i,
                    dit: pstate_dit,
                    sp: pstate_sp,
                    it: pstate_it,
                    t: pstate_t,
                    f: pstate_f,
                    d: pstate_d,
                    nrw: pstate_nrw,
                    btype: pstate_btype,
                    j: pstate_j,
                    e: pstate_e,
                    el: pstate_el,
                    q: pstate_q,
                    ss: pstate_ss,
                    m: pstate_m,
                    a: pstate_a,
                    il: pstate_il,
                },
                sctlr_el3,
            };

            successors.push(successor);
        }
        successors
    }

    pub fn step(&mut self) -> Vec<Self> {
        info!("Stepping {}", &self);
        if self.pc == 0x40088398 {
            info!("concretizing r2");
            self.concretize_r2();
        } else if self.pc == 0x400883A8 {
            info!("simplifying r1 and r8");
            self.simplify_offset_symbol_r1(self.memory.symbolic_volatile_regions[0].base_symbol);
            self.simplify_offset_symbol_r8(self.memory.symbolic_volatile_regions[0].base_symbol);
            if let Value1::Symbolic(_) = self.pstate.z {
                self.concretize_pstate_flags();
            }
        }
        self.apply_transformation_function(&AARCH64_ISLA_HANDLE.derive_ttf(&self))
    }

    pub fn step_until(&self, exits: &[u64]) -> Vec<Self> {
        debug!("Executing until end of main loop or panic");
        let mut states = vec![self.clone()];
        let mut finished = vec![];
        let mut i = 0;
        while states.len() > 0 {
            let mut state = states.pop().unwrap();
            let mut succs = state.step();
            while succs.len() > 0 {
                let succ = succs.pop().unwrap();
                if !exits.contains(&succ.pc) {
                    states.push(succ);
                } else {
                    info!("[INFO] a branch has stopped at {:x}", succ.pc);
                    let path = PathBuf::from(format!("snapshots/{}_{:x}", i, succ.pc));
                    fs::create_dir(&path).unwrap();
                    succ.serialize(&path);
                    i += 1;
                    finished.push(succ);
                }
            }
        }
        finished
    }
    
    /// Attempt to simplify a symbol of the form `base_symbol + x`, where x is a concrete number.
    /// scfia-lib keeps retired symbols alive until all affected symbols are retired, so the user must ensure no dangling relatives are alive.
    pub fn simplify_offset_symbol_r1(&mut self, base_symbol: ScfiaSymbolId) {
        info!("simplify_offset_symbol_r1");
        let symbol = self.r1.as_symbolic();
        let offset = AARCH64_ISLA_HANDLE.determine_symbol_offset(self, base_symbol, symbol);
        self.symbol_manager.increment_active_symbol_refcount(base_symbol);
        self.symbol_manager.decrement_active_symbol_refcount(&symbol);
        self.r1 = Value64::Symbolic(self.symbol_manager.create_symbol_defined(
            ScfiaSymbolExpression::Bvadd(
                Arc::new(ScfiaSymbolExpression::Var(base_symbol)),
                Arc::new(ScfiaSymbolExpression::Bits64(offset, 64))
            )
        ));
    }

    /// Attempt to simplify a symbol of the form `base_symbol + x`, where x is a concrete number.
    /// scfia-lib keeps retired symbols alive until all affected symbols are retired, so the user must ensure no dangling relatives are alive.
    pub fn simplify_offset_symbol_r8(&mut self, base_symbol: ScfiaSymbolId) {
        info!("simplify_offset_symbol_r8");
        let symbol = self.r8.as_symbolic();
        let offset = AARCH64_ISLA_HANDLE.determine_symbol_offset(self, base_symbol, symbol);
        self.symbol_manager.increment_active_symbol_refcount(base_symbol);
        self.symbol_manager.decrement_active_symbol_refcount(&symbol);
        self.r8 = Value64::Symbolic(self.symbol_manager.create_symbol_defined(
            ScfiaSymbolExpression::Bvadd(
                Arc::new(ScfiaSymbolExpression::Var(base_symbol)),
                Arc::new(ScfiaSymbolExpression::Bits64(offset, 64))
            )
        ));
    }

    pub fn concretize_r2(&mut self) {
        let candidates = AARCH64_ISLA_HANDLE.monomorphize_u64_scfia_symbol(self, self.r2.as_symbolic());
        assert_eq!(candidates.len(), 1);
        self.symbol_manager.decrement_active_symbol_refcount(&self.r2.as_symbolic());
        self.r2 = Value64::Concrete(candidates[0]);
    }

    pub fn concretize_pstate_flags(&mut self) {
        let n_candidates = AARCH64_ISLA_HANDLE.monomorphize_u64_scfia_symbol(self, self.pstate.n.as_symbolic());
        assert_eq!(n_candidates.len(), 1);
        let z_candidates = AARCH64_ISLA_HANDLE.monomorphize_u64_scfia_symbol(self, self.pstate.z.as_symbolic());
        assert_eq!(z_candidates.len(), 1);
        let c_candidates = AARCH64_ISLA_HANDLE.monomorphize_u64_scfia_symbol(self, self.pstate.c.as_symbolic());
        assert_eq!(c_candidates.len(), 1);
        let v_candidates = AARCH64_ISLA_HANDLE.monomorphize_u64_scfia_symbol(self, self.pstate.v.as_symbolic());
        assert_eq!(v_candidates.len(), 1);
        self.symbol_manager.decrement_active_symbol_refcount(&self.pstate.n.as_symbolic());
        self.symbol_manager.decrement_active_symbol_refcount(&self.pstate.z.as_symbolic());
        self.symbol_manager.decrement_active_symbol_refcount(&self.pstate.c.as_symbolic());
        self.symbol_manager.decrement_active_symbol_refcount(&self.pstate.v.as_symbolic());
        self.pstate.n = Value1::Concrete(n_candidates[0]);
        self.pstate.z = Value1::Concrete(z_candidates[0]);
        self.pstate.c = Value1::Concrete(c_candidates[0]);
        self.pstate.v = Value1::Concrete(v_candidates[0]);
    }

    pub fn serialize(&self, folder: &PathBuf) {
        self.memory.serialize(folder);
        self.serialize_registers(folder);
    }

    fn serialize_registers(&self, folder: &PathBuf) {
        let file = File::create(folder.join("registers")).unwrap();
        let mut file = LineWriter::new(file);

        match self.r0 {
            Value64::Concrete(val) => {
                file.write(format!("R0={:x}\n", val).as_bytes()).unwrap();
            }
            Value64::Symbolic(_) => {}
        }
        match self.r1 {
            Value64::Concrete(val) => {
                file.write(format!("R1={:x}\n", val).as_bytes()).unwrap();
            }
            Value64::Symbolic(_) => {}
        }
        match self.r2 {
            Value64::Concrete(val) => {
                file.write(format!("R2={:x}\n", val).as_bytes()).unwrap();
            }
            Value64::Symbolic(_) => {}
        }
        match self.r3 {
            Value64::Concrete(val) => {
                file.write(format!("R3={:x}\n", val).as_bytes()).unwrap();
            }
            Value64::Symbolic(_) => {}
        }
        match self.r4 {
            Value64::Concrete(val) => {
                file.write(format!("R4={:x}\n", val).as_bytes()).unwrap();
            }
            Value64::Symbolic(_) => {}
        }
        match self.r5 {
            Value64::Concrete(val) => {
                file.write(format!("R5={:x}\n", val).as_bytes()).unwrap();
            }
            Value64::Symbolic(_) => {}
        }
        match self.r6 {
            Value64::Concrete(val) => {
                file.write(format!("R6={:x}\n", val).as_bytes()).unwrap();
            }
            Value64::Symbolic(_) => {}
        }
        match self.r7 {
            Value64::Concrete(val) => {
                file.write(format!("R7={:x}\n", val).as_bytes()).unwrap();
            }
            Value64::Symbolic(_) => {}
        }
        match self.r8 {
            Value64::Concrete(val) => {
                file.write(format!("R8={:x}\n", val).as_bytes()).unwrap();
            }
            Value64::Symbolic(_) => {}
        }
        match self.r9 {
            Value64::Concrete(val) => {
                file.write(format!("R9={:x}\n", val).as_bytes()).unwrap();
            }
            Value64::Symbolic(_) => {}
        }
        match self.r10 {
            Value64::Concrete(val) => {
                file.write(format!("R10={:x}\n", val).as_bytes()).unwrap();
            }
            Value64::Symbolic(_) => {}
        }
        match self.r11 {
            Value64::Concrete(val) => {
                file.write(format!("R11={:x}\n", val).as_bytes()).unwrap();
            }
            Value64::Symbolic(_) => {}
        }
        match self.r12 {
            Value64::Concrete(val) => {
                file.write(format!("R12={:x}\n", val).as_bytes()).unwrap();
            }
            Value64::Symbolic(_) => {}
        }
        match self.r13 {
            Value64::Concrete(val) => {
                file.write(format!("R13={:x}\n", val).as_bytes()).unwrap();
            }
            Value64::Symbolic(_) => {}
        }
        match self.r14 {
            Value64::Concrete(val) => {
                file.write(format!("R14={:x}\n", val).as_bytes()).unwrap();
            }
            Value64::Symbolic(_) => {}
        }
        match self.r15 {
            Value64::Concrete(val) => {
                file.write(format!("R15={:x}\n", val).as_bytes()).unwrap();
            }
            Value64::Symbolic(_) => {}
        }
        match self.r16 {
            Value64::Concrete(val) => {
                file.write(format!("R16={:x}\n", val).as_bytes()).unwrap();
            }
            Value64::Symbolic(_) => {}
        }
        match self.r17 {
            Value64::Concrete(val) => {
                file.write(format!("R17={:x}\n", val).as_bytes()).unwrap();
            }
            Value64::Symbolic(_) => {}
        }
        match self.r18 {
            Value64::Concrete(val) => {
                file.write(format!("R18={:x}\n", val).as_bytes()).unwrap();
            }
            Value64::Symbolic(_) => {}
        }
        match self.r19 {
            Value64::Concrete(val) => {
                file.write(format!("R19={:x}\n", val).as_bytes()).unwrap();
            }
            Value64::Symbolic(_) => {}
        }
        match self.r20 {
            Value64::Concrete(val) => {
                file.write(format!("R20={:x}\n", val).as_bytes()).unwrap();
            }
            Value64::Symbolic(_) => {}
        }
        match self.r21 {
            Value64::Concrete(val) => {
                file.write(format!("R21={:x}\n", val).as_bytes()).unwrap();
            }
            Value64::Symbolic(_) => {}
        }
        match self.r22 {
            Value64::Concrete(val) => {
                file.write(format!("R22={:x}\n", val).as_bytes()).unwrap();
            }
            Value64::Symbolic(_) => {}
        }
        match self.r23 {
            Value64::Concrete(val) => {
                file.write(format!("R23={:x}\n", val).as_bytes()).unwrap();
            }
            Value64::Symbolic(_) => {}
        }
        match self.r24 {
            Value64::Concrete(val) => {
                file.write(format!("R24={:x}\n", val).as_bytes()).unwrap();
            }
            Value64::Symbolic(_) => {}
        }
        match self.r25 {
            Value64::Concrete(val) => {
                file.write(format!("R25={:x}\n", val).as_bytes()).unwrap();
            }
            Value64::Symbolic(_) => {}
        }
        match self.r26 {
            Value64::Concrete(val) => {
                file.write(format!("R26={:x}\n", val).as_bytes()).unwrap();
            }
            Value64::Symbolic(_) => {}
        }
        match self.r27 {
            Value64::Concrete(val) => {
                file.write(format!("R27={:x}\n", val).as_bytes()).unwrap();
            }
            Value64::Symbolic(_) => {}
        }
        match self.r28 {
            Value64::Concrete(val) => {
                file.write(format!("R28={:x}\n", val).as_bytes()).unwrap();
            }
            Value64::Symbolic(_) => {}
        }
        match self.fp {
            Value64::Concrete(val) => {
                file.write(format!("FP={:x}\n", val).as_bytes()).unwrap();
            }
            Value64::Symbolic(_) => {}
        }
        match self.lr {
            Value64::Concrete(val) => {
                file.write(format!("LR={:x}\n", val).as_bytes()).unwrap();
            }
            Value64::Symbolic(_) => {}
        }
        match self.sp_el3 {
            Value64::Concrete(val) => {
                file.write(format!("SP_EL3={:x}\n", val).as_bytes())
                    .unwrap();
            }
            Value64::Symbolic(_) => {}
        }
        file.write(format!("PC={:x}\n", self.pc).as_bytes())
            .unwrap();

        file.write(format!("SCTLR_EL3={:x}\n", self.sctlr_el3).as_bytes())
            .unwrap();

        self.serialize_pstate(folder);
    }

    fn serialize_pstate(&self, folder: &PathBuf) {
        let file = File::create(folder.join("registers_pstate")).unwrap();
        let mut file = LineWriter::new(file);

        match self.pstate.n {
            Value1::Concrete(n) => {
                file.write(format!("N={}\n", n).as_bytes()).unwrap();
            }
            Value1::Symbolic(_) => {}
        }
        match self.pstate.z {
            Value1::Concrete(z) => {
                file.write(format!("Z={}\n", z).as_bytes()).unwrap();
            }
            Value1::Symbolic(_) => {}
        }
        match self.pstate.c {
            Value1::Concrete(c) => {
                file.write(format!("C={}\n", c).as_bytes()).unwrap();
            }
            Value1::Symbolic(_) => {}
        }
        match self.pstate.v {
            Value1::Concrete(v) => {
                file.write(format!("V={}\n", v).as_bytes()).unwrap();
            }
            Value1::Symbolic(_) => {}
        }

        file.write(format!("PAN={}\n", self.pstate.pan).as_bytes())
            .unwrap();
        file.write(format!("SSBS={}\n", self.pstate.ssbs).as_bytes())
            .unwrap();
        file.write(format!("TCO={}\n", self.pstate.tco).as_bytes())
            .unwrap();
        file.write(format!("UAO={}\n", self.pstate.uao).as_bytes())
            .unwrap();
        file.write(format!("I={}\n", self.pstate.i).as_bytes())
            .unwrap();
        file.write(format!("DIT={}\n", self.pstate.dit).as_bytes())
            .unwrap();
        file.write(format!("SP={}\n", self.pstate.sp).as_bytes())
            .unwrap();
        file.write(format!("IT={}\n", self.pstate.it).as_bytes())
            .unwrap();
        file.write(format!("T={}\n", self.pstate.t).as_bytes())
            .unwrap();
        file.write(format!("F={}\n", self.pstate.f).as_bytes())
            .unwrap();
        file.write(format!("D={}\n", self.pstate.d).as_bytes())
            .unwrap();
        file.write(format!("nRW={}\n", self.pstate.nrw).as_bytes())
            .unwrap();
        file.write(format!("BTYPE={}\n", self.pstate.btype).as_bytes())
            .unwrap();
        file.write(format!("J={}\n", self.pstate.j).as_bytes())
            .unwrap();
        file.write(format!("E={}\n", self.pstate.e).as_bytes())
            .unwrap();
        file.write(format!("EL={}\n", self.pstate.el).as_bytes())
            .unwrap();
        file.write(format!("Q={}\n", self.pstate.q).as_bytes())
            .unwrap();
        file.write(format!("SS={}\n", self.pstate.ss).as_bytes())
            .unwrap();
        file.write(format!("M={}\n", self.pstate.m).as_bytes())
            .unwrap();
        file.write(format!("A={}\n", self.pstate.a).as_bytes())
            .unwrap();
        file.write(format!("IL={}\n", self.pstate.il).as_bytes())
            .unwrap();
    }

    pub fn deserialize(folder: &PathBuf) -> Self {
        let mut aarch64_state = ScfiaAarch64SystemState::new();
        aarch64_state.deserialize_registers(folder);
        let memory = &mut aarch64_state.memory;
        memory.deserialize(folder, &mut aarch64_state.symbol_manager);
        aarch64_state
    }

    fn deserialize_registers(&mut self, folder: &PathBuf) {
        let lines = BufReader::new(File::open(folder.join("registers")).unwrap()).lines();
        // TODO complain if PC and SCTLR_EL3 are not found
        for line in lines {
            let line = line.unwrap();
            let parts = line.split('=').collect::<Vec<&str>>();
            let value = u64::from_str_radix(parts[1], 16).unwrap();
            match parts[0] {
                "R0" => self.r0 = Value64::Concrete(value),
                "R1" => self.r1 = Value64::Concrete(value),
                "R2" => self.r2 = Value64::Concrete(value),
                "R3" => self.r3 = Value64::Concrete(value),
                "R4" => self.r4 = Value64::Concrete(value),
                "R5" => self.r5 = Value64::Concrete(value),
                "R6" => self.r6 = Value64::Concrete(value),
                "R7" => self.r7 = Value64::Concrete(value),
                "R8" => self.r8 = Value64::Concrete(value),
                "R9" => self.r9 = Value64::Concrete(value),
                "R10" => self.r10 = Value64::Concrete(value),
                "R11" => self.r11 = Value64::Concrete(value),
                "R12" => self.r12 = Value64::Concrete(value),
                "R13" => self.r13 = Value64::Concrete(value),
                "R14" => self.r14 = Value64::Concrete(value),
                "R15" => self.r15 = Value64::Concrete(value),
                "R16" => self.r16 = Value64::Concrete(value),
                "R17" => self.r17 = Value64::Concrete(value),
                "R18" => self.r18 = Value64::Concrete(value),
                "R19" => self.r19 = Value64::Concrete(value),
                "R20" => self.r20 = Value64::Concrete(value),
                "R21" => self.r21 = Value64::Concrete(value),
                "R22" => self.r22 = Value64::Concrete(value),
                "R23" => self.r23 = Value64::Concrete(value),
                "R24" => self.r24 = Value64::Concrete(value),
                "R25" => self.r25 = Value64::Concrete(value),
                "R26" => self.r26 = Value64::Concrete(value),
                "R27" => self.r27 = Value64::Concrete(value),
                "R28" => self.r28 = Value64::Concrete(value),
                "FP" => self.fp = Value64::Concrete(value),
                "LR" => self.lr = Value64::Concrete(value),
                "SP_EL3" => self.sp_el3 = Value64::Concrete(value),
                "PC" => self.pc = value,
                "SCTLR_EL3" => self.sctlr_el3 = value,
                x => panic!("{:?}", x),
            }
        }

        // TODO complain if non-symbolic parts are missing
        let lines = BufReader::new(File::open(folder.join("registers_pstate")).unwrap()).lines();
        for line in lines {
            let line = line.unwrap();
            let parts = line.split('=').collect::<Vec<&str>>();
            match parts[0] {
                "N" => self.pstate.n = Value1::Concrete(u64::from_str(parts[1]).unwrap()),
                "Z" => self.pstate.z = Value1::Concrete(u64::from_str(parts[1]).unwrap()),
                "C" => self.pstate.c = Value1::Concrete(u64::from_str(parts[1]).unwrap()),
                "V" => self.pstate.v = Value1::Concrete(u64::from_str(parts[1]).unwrap()),
                "PAN" => self.pstate.pan = bool::from_str(parts[1]).unwrap(),
                "SSBS" => self.pstate.ssbs = bool::from_str(parts[1]).unwrap(),
                "TCO" => self.pstate.tco = bool::from_str(parts[1]).unwrap(),
                "UAO" => self.pstate.uao = bool::from_str(parts[1]).unwrap(),
                "I" => self.pstate.i = bool::from_str(parts[1]).unwrap(),
                "DIT" => self.pstate.dit = bool::from_str(parts[1]).unwrap(),
                "SP" => self.pstate.sp = bool::from_str(parts[1]).unwrap(),
                "IT" => self.pstate.it = u8::from_str_radix(parts[1], 16).unwrap(),
                "T" => self.pstate.t = bool::from_str(parts[1]).unwrap(),
                "F" => self.pstate.f = bool::from_str(parts[1]).unwrap(),
                "D" => self.pstate.d = bool::from_str(parts[1]).unwrap(),
                "nRW" => self.pstate.nrw = bool::from_str(parts[1]).unwrap(),
                "BTYPE" => self.pstate.btype = u8::from_str_radix(parts[1], 16).unwrap(),
                "J" => self.pstate.j = bool::from_str(parts[1]).unwrap(),
                "E" => self.pstate.e = bool::from_str(parts[1]).unwrap(),
                "EL" => self.pstate.el = u8::from_str_radix(parts[1], 16).unwrap(),
                "Q" => self.pstate.q = bool::from_str(parts[1]).unwrap(),
                "SS" => self.pstate.ss = bool::from_str(parts[1]).unwrap(),
                "M" => self.pstate.m = u8::from_str_radix(parts[1], 16).unwrap(),
                "A" => self.pstate.a = bool::from_str(parts[1]).unwrap(),
                "IL" => self.pstate.il = bool::from_str(parts[1]).unwrap(),
                x => panic!("{:?}", x),
            }
        }
    }
}

impl Display for ScfiaAarch64SystemState {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        writeln!(f, "ScfiaAarch64SystemState")?;
        writeln!(f, "    PC:  {:20x}    PSTATE: {:?}", self.pc, self.pstate,)?;
        writeln!(f, "    R0:  {:20}    R16: {}", self.r0, self.r16)?;
        writeln!(f, "    R1:  {:20}    R17: {}", self.r1, self.r17)?;
        writeln!(f, "    R2:  {:20}    R18: {}", self.r2, self.r18)?;
        writeln!(f, "    R3:  {:20}    R19: {}", self.r3, self.r19)?;
        writeln!(f, "    R4:  {:20}    R20: {}", self.r4, self.r20)?;
        writeln!(f, "    R5:  {:20}    R21: {}", self.r5, self.r21)?;
        writeln!(f, "    R6:  {:20}    R22: {}", self.r6, self.r22)?;
        writeln!(f, "    R7:  {:20}    R23: {}", self.r7, self.r23)?;
        writeln!(f, "    R8:  {:20}    R24: {}", self.r8, self.r24)?;
        writeln!(f, "    R9:  {:20}    R25: {}", self.r9, self.r25)?;
        writeln!(f, "    R10: {:20}    R26: {}", self.r10, self.r26)?;
        writeln!(f, "    R11: {:20}    R27: {}", self.r11, self.r27)?;
        writeln!(f, "    R12: {:20}    R28: {}", self.r12, self.r28)?;
        writeln!(f, "    R13: {:20}    FP:  {}", self.r13, self.fp)?;
        writeln!(f, "    R14: {:20}    LR:  {}", self.r14, self.lr)?;
        writeln!(f, "    R15: {:20}    SP:  {}", self.r15, self.sp_el3)?;
        // TODO stack? if yes, how? bytewise? wordwise?
        Ok(())
    }
}

impl ScfiaSystemState for ScfiaAarch64SystemState {
    type TransformationDefinition = ScfiaAarch64TransformationDefinition;
}

#[derive(Clone, Debug)]
pub struct Pstate {
    pub n: Value1,
    pub z: Value1,
    pub c: Value1,
    pub v: Value1,
    pub pan: bool,
    pub ssbs: bool,
    pub tco: bool,
    pub uao: bool,
    pub i: bool,
    pub dit: bool,
    pub sp: bool,
    pub it: u8,
    pub t: bool,
    pub f: bool,
    pub d: bool,
    pub nrw: bool,
    pub btype: u8,
    pub j: bool,
    pub e: bool,
    pub el: u8,
    pub q: bool,
    pub ss: bool,
    pub m: u8,
    pub a: bool,
    pub il: bool,
}
