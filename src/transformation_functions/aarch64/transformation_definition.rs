use std::sync::Arc;

use log::{debug, trace};

use crate::{symbols::{scfia_symbol::{ScfiaSymbol, ScfiaSymbolId}, scfia_symbol_expression::ScfiaSymbolExpression, scfia_symbol_manager::ScfiaSymbolManager}, system_states::{aarch64::ScfiaAarch64SystemState, ScfiaSystemState}, transformation_functions::{
        transformation_function_expression::ScfiaTransformationFunctionExpression,
        ScfiaTransformationDefinition,
    }, values::{ScfiaValue, value1::Value1, value64::Value64, value8::Value8}};
use crate::asserts::scfia_assert_expression::ScfiaAssertExpression;

#[derive(Debug)]
pub enum ScfiaAarch64TransformationDefinition {
    ByteFromMemory(u64),
    BytesFromMemoryCandidates(Vec<u64>, u32),
    BytesFromMemoryVolatile(u32),
    FromR0,
    FromR1,
    FromR2,
    FromR3,
    FromR4,
    FromR5,
    FromR6,
    FromR7,
    FromR8,
    FromR9,
    FromR10,
    FromR11,
    FromR12,
    FromR13,
    FromR14,
    FromR15,
    FromR16,
    FromR17,
    FromR18,
    FromR19,
    FromR20,
    FromR21,
    FromR22,
    FromR23,
    FromR24,
    FromR25,
    FromR26,
    FromR27,
    FromR28,
    FromR29,
    FromR30,
    FromPstateN,
    FromPstateZ,
    FromPstateC,
    FromPstateV,
    FromExpression(Arc<ScfiaTransformationFunctionExpression<ScfiaAarch64SystemState>>),
    Constant(u32),
    ConstantBool,
}

impl ScfiaTransformationDefinition<ScfiaAarch64SystemState>
    for ScfiaAarch64TransformationDefinition
{
    fn to_symbol(
        &self,
        old_state: &ScfiaAarch64SystemState,
        new_symbol_manager: &mut ScfiaSymbolManager,
    ) -> ScfiaSymbolId {
        match self {
            ScfiaAarch64TransformationDefinition::ByteFromMemory(address) => {
                // this has to increment the refcount
                for region in &old_state.memory.stable_regions {
                    if region.range.contains(address) {
                        let symbol_id = region.data.get(address).unwrap().as_symbolic();
                        new_symbol_manager.increment_active_symbol_refcount(symbol_id);
                        return symbol_id;
                    }
                }

                for region in &old_state.memory.volatile_regions {
                    if region.range.contains(address) {
                        return new_symbol_manager.create_symbol_bv(8);
                    }
                }

                panic!()
            }
            ScfiaAarch64TransformationDefinition::BytesFromMemoryCandidates(candidates, bytes) => {
                debug!("handling cfiaAarch64TransformationDefinition::BytesFromMemoryCandidates({} candidates, {} bytes)", candidates.len(), bytes);
                let symbol = new_symbol_manager.create_symbol_bv(*bytes * 8);
                for byte in 0..*bytes {
                    let read_cell = old_state.memory.read_stable(candidates[0] + byte as u64).unwrap();
                    // assert unanimity
                    for candidate in candidates {
                        assert!(read_cell == old_state.memory.read_stable(*candidate + byte as u64).unwrap())
                    }

                    match read_cell {
                        Value8::Concrete(read_byte) => {
                            debug!("building extraction {} - {} for 0x{:x}", ((byte + 1) * 8) - 1, byte * 8, read_byte);
                            new_symbol_manager.increment_active_symbol_refcount(symbol); //TODO hacky
                            new_symbol_manager.add_assert(ScfiaAssertExpression::Eq(
                                Arc::new(ScfiaAssertExpression::Extract(((byte + 1) * 8)-1, byte * 8, Arc::new(ScfiaAssertExpression::Var(symbol)))),
                                Arc::new(ScfiaAssertExpression::Bits64(read_byte as u64, 8))
                            ), vec![symbol]);
                        }
                        Value8::Symbolic(read_symbol) => {
                            new_symbol_manager.increment_active_symbol_refcount(symbol); //TODO hacky
                            new_symbol_manager.increment_active_symbol_refcount(read_symbol); //TODO hacky
                            debug!("building extraction {} - {}", ((byte + 1) * 8) - 1, byte * 8);
                            new_symbol_manager.add_assert(ScfiaAssertExpression::Eq(
                                Arc::new(ScfiaAssertExpression::Extract(((byte + 1) * 8)-1, byte * 8, Arc::new(ScfiaAssertExpression::Var(symbol)))),
                                Arc::new(ScfiaAssertExpression::Var(read_symbol))
                            ), vec![symbol, read_symbol]);
                        }
                    }
                }
                symbol
            }
            ScfiaAarch64TransformationDefinition::BytesFromMemoryVolatile(bytes) => {
                new_symbol_manager.create_symbol_bv(*bytes * 8)
            }
            ScfiaAarch64TransformationDefinition::FromR0 => {
                let symbol_id = old_state.r0.as_symbolic();
                new_symbol_manager.increment_active_symbol_refcount(symbol_id);
                symbol_id
            }
            ScfiaAarch64TransformationDefinition::FromR1 => {
                let symbol_id = old_state.r1.as_symbolic();
                new_symbol_manager.increment_active_symbol_refcount(symbol_id);
                symbol_id
            }
            ScfiaAarch64TransformationDefinition::FromR2 => {
                let symbol_id = old_state.r2.as_symbolic();
                new_symbol_manager.increment_active_symbol_refcount(symbol_id);
                symbol_id
            }
            ScfiaAarch64TransformationDefinition::FromR3 => {
                let symbol_id = old_state.r3.as_symbolic();
                new_symbol_manager.increment_active_symbol_refcount(symbol_id);
                symbol_id
            }
            ScfiaAarch64TransformationDefinition::FromR4 => {
                let symbol_id = old_state.r4.as_symbolic();
                new_symbol_manager.increment_active_symbol_refcount(symbol_id);
                symbol_id
            }
            ScfiaAarch64TransformationDefinition::FromR5 => {
                let symbol_id = old_state.r5.as_symbolic();
                new_symbol_manager.increment_active_symbol_refcount(symbol_id);
                symbol_id
            }
            ScfiaAarch64TransformationDefinition::FromR6 => {
                let symbol_id = old_state.r6.as_symbolic();
                new_symbol_manager.increment_active_symbol_refcount(symbol_id);
                symbol_id
            }
            ScfiaAarch64TransformationDefinition::FromR7 => {
                let symbol_id = old_state.r7.as_symbolic();
                new_symbol_manager.increment_active_symbol_refcount(symbol_id);
                symbol_id
            }
            ScfiaAarch64TransformationDefinition::FromR8 => {
                let symbol_id = old_state.r8.as_symbolic();
                new_symbol_manager.increment_active_symbol_refcount(symbol_id);
                symbol_id
            }
            ScfiaAarch64TransformationDefinition::FromR9 => {
                let symbol_id = old_state.r9.as_symbolic();
                new_symbol_manager.increment_active_symbol_refcount(symbol_id);
                symbol_id
            }
            ScfiaAarch64TransformationDefinition::FromR10 => {
                let symbol_id = old_state.r10.as_symbolic();
                new_symbol_manager.increment_active_symbol_refcount(symbol_id);
                symbol_id
            }
            ScfiaAarch64TransformationDefinition::FromR11 => {
                let symbol_id = old_state.r11.as_symbolic();
                new_symbol_manager.increment_active_symbol_refcount(symbol_id);
                symbol_id
            }
            ScfiaAarch64TransformationDefinition::FromR12 => {
                let symbol_id = old_state.r12.as_symbolic();
                new_symbol_manager.increment_active_symbol_refcount(symbol_id);
                symbol_id
            }
            ScfiaAarch64TransformationDefinition::FromR13 => {
                let symbol_id = old_state.r13.as_symbolic();
                new_symbol_manager.increment_active_symbol_refcount(symbol_id);
                symbol_id
            }
            ScfiaAarch64TransformationDefinition::FromR14 => {
                let symbol_id = old_state.r14.as_symbolic();
                new_symbol_manager.increment_active_symbol_refcount(symbol_id);
                symbol_id
            }
            ScfiaAarch64TransformationDefinition::FromR15 => {
                let symbol_id = old_state.r15.as_symbolic();
                new_symbol_manager.increment_active_symbol_refcount(symbol_id);
                symbol_id
            }
            ScfiaAarch64TransformationDefinition::FromR16 => {
                let symbol_id = old_state.r16.as_symbolic();
                new_symbol_manager.increment_active_symbol_refcount(symbol_id);
                symbol_id
            }
            ScfiaAarch64TransformationDefinition::FromR17 => {
                let symbol_id = old_state.r17.as_symbolic();
                new_symbol_manager.increment_active_symbol_refcount(symbol_id);
                symbol_id
            }
            ScfiaAarch64TransformationDefinition::FromR18 => {
                let symbol_id = old_state.r18.as_symbolic();
                new_symbol_manager.increment_active_symbol_refcount(symbol_id);
                symbol_id
            }
            ScfiaAarch64TransformationDefinition::FromR19 => {
                let symbol_id = old_state.r19.as_symbolic();
                new_symbol_manager.increment_active_symbol_refcount(symbol_id);
                symbol_id
            }
            ScfiaAarch64TransformationDefinition::FromR20 => {
                let symbol_id = old_state.r20.as_symbolic();
                new_symbol_manager.increment_active_symbol_refcount(symbol_id);
                symbol_id
            }
            ScfiaAarch64TransformationDefinition::FromR21 => {
                let symbol_id = old_state.r21.as_symbolic();
                new_symbol_manager.increment_active_symbol_refcount(symbol_id);
                symbol_id
            }
            ScfiaAarch64TransformationDefinition::FromR22 => {
                let symbol_id = old_state.r22.as_symbolic();
                new_symbol_manager.increment_active_symbol_refcount(symbol_id);
                symbol_id
            }
            ScfiaAarch64TransformationDefinition::FromR23 => {
                let symbol_id = old_state.r23.as_symbolic();
                new_symbol_manager.increment_active_symbol_refcount(symbol_id);
                symbol_id
            }
            ScfiaAarch64TransformationDefinition::FromR24 => {
                let symbol_id = old_state.r24.as_symbolic();
                new_symbol_manager.increment_active_symbol_refcount(symbol_id);
                symbol_id
            }
            ScfiaAarch64TransformationDefinition::FromR25 => {
                let symbol_id = old_state.r25.as_symbolic();
                new_symbol_manager.increment_active_symbol_refcount(symbol_id);
                symbol_id
            }
            ScfiaAarch64TransformationDefinition::FromR26 => {
                let symbol_id = old_state.r26.as_symbolic();
                new_symbol_manager.increment_active_symbol_refcount(symbol_id);
                symbol_id
            }
            ScfiaAarch64TransformationDefinition::FromR27 => {
                let symbol_id = old_state.r27.as_symbolic();
                new_symbol_manager.increment_active_symbol_refcount(symbol_id);
                symbol_id
            }
            ScfiaAarch64TransformationDefinition::FromR28 => {
                let symbol_id = old_state.r28.as_symbolic();
                new_symbol_manager.increment_active_symbol_refcount(symbol_id);
                symbol_id
            }
            ScfiaAarch64TransformationDefinition::FromR29 => {
                let symbol_id = old_state.fp.as_symbolic();
                new_symbol_manager.increment_active_symbol_refcount(symbol_id);
                symbol_id
            }
            ScfiaAarch64TransformationDefinition::FromR30 => {
                let symbol_id = old_state.lr.as_symbolic();
                new_symbol_manager.increment_active_symbol_refcount(symbol_id);
                symbol_id
            }
            ScfiaAarch64TransformationDefinition::FromPstateN => {
                let symbol_id = old_state.pstate.n.as_symbolic();
                new_symbol_manager.increment_active_symbol_refcount(symbol_id);
                symbol_id
            }
            ScfiaAarch64TransformationDefinition::FromPstateZ => {
                let symbol_id = old_state.pstate.z.as_symbolic();
                new_symbol_manager.increment_active_symbol_refcount(symbol_id);
                symbol_id
            }
            ScfiaAarch64TransformationDefinition::FromPstateC => {
                let symbol_id = old_state.pstate.c.as_symbolic();
                new_symbol_manager.increment_active_symbol_refcount(symbol_id);
                symbol_id
            }
            ScfiaAarch64TransformationDefinition::FromPstateV => {
                let symbol_id = old_state.pstate.v.as_symbolic();
                new_symbol_manager.increment_active_symbol_refcount(symbol_id);
                symbol_id
            }
            ScfiaAarch64TransformationDefinition::FromExpression(expression) => {
                let scfia_expression =
                    expression.to_scfia_symbol_expression(old_state, new_symbol_manager);
                new_symbol_manager.create_symbol_defined(scfia_expression)
            }
            ScfiaAarch64TransformationDefinition::Constant(_) => {
                todo!()
            }
            ScfiaAarch64TransformationDefinition::ConstantBool => {
                todo!()
            }
        }
    }
}
