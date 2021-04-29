use std::{collections::HashMap, sync::Arc};

use crate::{
    isla::aarch64::isla_handle::AARCH64_ISLA_HANDLE,
    transformation_functions::{
        aarch64::transformation_function::ScfiaAarch64TransformationFunctionSuccessorDefinition,
        transformation_function_expression::ScfiaTransformationFunctionExpression,
    },
};
use isla_lib::{
    concrete::{bitvector64::B64, BV},
    ir::{Name, Val},
    smt::Sym,
};
use log::{trace, warn};

use crate::transformation_functions::aarch64::transformation_definition::ScfiaAarch64TransformationDefinition;

use super::IGNORED_REGISTERS;

pub fn handle_write_reg(
    name: &Name,
    val: &Val<B64>,
    successor_definition: &mut ScfiaAarch64TransformationFunctionSuccessorDefinition,
    isla_sym_to_transformation_definition_mapping: &mut HashMap<
        Sym,
        Arc<ScfiaAarch64TransformationDefinition>,
    >,
) {
    let isla_reg_name = AARCH64_ISLA_HANDLE.shared_state.symtab.to_str(*name);
    trace!("handle_write_reg {}", isla_reg_name);
    if IGNORED_REGISTERS.contains(&isla_reg_name) {
        return;
    }

    match val {
        Val::Bits(bv) => {
            if isla_reg_name == "zR0" {
                successor_definition.new_r0 = Some(ScfiaTransformationFunctionExpression::Bits64(
                    bv.lower_u64(),
                    64,
                ))
            } else if isla_reg_name == "zR1" {
                successor_definition.new_r1 = Some(ScfiaTransformationFunctionExpression::Bits64(
                    bv.lower_u64(),
                    64,
                ))
            } else if isla_reg_name == "zR2" {
                successor_definition.new_r2 = Some(ScfiaTransformationFunctionExpression::Bits64(
                    bv.lower_u64(),
                    64,
                ))
            } else if isla_reg_name == "zR3" {
                successor_definition.new_r3 = Some(ScfiaTransformationFunctionExpression::Bits64(
                    bv.lower_u64(),
                    64,
                ))
            } else if isla_reg_name == "zR4" {
                successor_definition.new_r4 = Some(ScfiaTransformationFunctionExpression::Bits64(
                    bv.lower_u64(),
                    64,
                ))
            } else if isla_reg_name == "zR5" {
                successor_definition.new_r5 = Some(ScfiaTransformationFunctionExpression::Bits64(
                    bv.lower_u64(),
                    64,
                ))
            } else if isla_reg_name == "zR6" {
                successor_definition.new_r6 = Some(ScfiaTransformationFunctionExpression::Bits64(
                    bv.lower_u64(),
                    64,
                ))
            } else if isla_reg_name == "zR7" {
                successor_definition.new_r7 = Some(ScfiaTransformationFunctionExpression::Bits64(
                    bv.lower_u64(),
                    64,
                ))
            } else if isla_reg_name == "zR8" {
                successor_definition.new_r8 = Some(ScfiaTransformationFunctionExpression::Bits64(
                    bv.lower_u64(),
                    64,
                ))
            } else if isla_reg_name == "zR9" {
                successor_definition.new_r9 = Some(ScfiaTransformationFunctionExpression::Bits64(
                    bv.lower_u64(),
                    64,
                ))
            } else if isla_reg_name == "zR10" {
                successor_definition.new_r10 = Some(ScfiaTransformationFunctionExpression::Bits64(
                    bv.lower_u64(),
                    64,
                ))
            } else if isla_reg_name == "zR11" {
                successor_definition.new_r11 = Some(ScfiaTransformationFunctionExpression::Bits64(
                    bv.lower_u64(),
                    64,
                ))
            } else if isla_reg_name == "zR12" {
                successor_definition.new_r12 = Some(ScfiaTransformationFunctionExpression::Bits64(
                    bv.lower_u64(),
                    64,
                ))
            } else if isla_reg_name == "zR13" {
                successor_definition.new_r13 = Some(ScfiaTransformationFunctionExpression::Bits64(
                    bv.lower_u64(),
                    64,
                ))
            } else if isla_reg_name == "zR14" {
                successor_definition.new_r14 = Some(ScfiaTransformationFunctionExpression::Bits64(
                    bv.lower_u64(),
                    64,
                ))
            } else if isla_reg_name == "zR15" {
                successor_definition.new_r15 = Some(ScfiaTransformationFunctionExpression::Bits64(
                    bv.lower_u64(),
                    64,
                ))
            } else if isla_reg_name == "zR16" {
                successor_definition.new_r16 = Some(ScfiaTransformationFunctionExpression::Bits64(
                    bv.lower_u64(),
                    64,
                ))
            } else if isla_reg_name == "zR17" {
                successor_definition.new_r17 = Some(ScfiaTransformationFunctionExpression::Bits64(
                    bv.lower_u64(),
                    64,
                ))
            } else if isla_reg_name == "zR18" {
                successor_definition.new_r18 = Some(ScfiaTransformationFunctionExpression::Bits64(
                    bv.lower_u64(),
                    64,
                ))
            } else if isla_reg_name == "zR19" {
                successor_definition.new_r19 = Some(ScfiaTransformationFunctionExpression::Bits64(
                    bv.lower_u64(),
                    64,
                ))
            } else if isla_reg_name == "zR20" {
                successor_definition.new_r20 = Some(ScfiaTransformationFunctionExpression::Bits64(
                    bv.lower_u64(),
                    64,
                ))
            } else if isla_reg_name == "zR21" {
                successor_definition.new_r21 = Some(ScfiaTransformationFunctionExpression::Bits64(
                    bv.lower_u64(),
                    64,
                ))
            } else if isla_reg_name == "zR22" {
                successor_definition.new_r22 = Some(ScfiaTransformationFunctionExpression::Bits64(
                    bv.lower_u64(),
                    64,
                ))
            } else if isla_reg_name == "zR23" {
                successor_definition.new_r23 = Some(ScfiaTransformationFunctionExpression::Bits64(
                    bv.lower_u64(),
                    64,
                ))
            } else if isla_reg_name == "zR23" {
                successor_definition.new_r23 = Some(ScfiaTransformationFunctionExpression::Bits64(
                    bv.lower_u64(),
                    64,
                ))
            } else if isla_reg_name == "zR24" {
                successor_definition.new_r24 = Some(ScfiaTransformationFunctionExpression::Bits64(
                    bv.lower_u64(),
                    64,
                ))
            } else if isla_reg_name == "zR25" {
                successor_definition.new_r25 = Some(ScfiaTransformationFunctionExpression::Bits64(
                    bv.lower_u64(),
                    64,
                ))
            } else if isla_reg_name == "zR26" {
                successor_definition.new_r26 = Some(ScfiaTransformationFunctionExpression::Bits64(
                    bv.lower_u64(),
                    64,
                ))
            } else if isla_reg_name == "zR27" {
                successor_definition.new_r27 = Some(ScfiaTransformationFunctionExpression::Bits64(
                    bv.lower_u64(),
                    64,
                ))
            } else if isla_reg_name == "zR28" {
                successor_definition.new_r28 = Some(ScfiaTransformationFunctionExpression::Bits64(
                    bv.lower_u64(),
                    64,
                ))
            } else if isla_reg_name == "zR29" {
                successor_definition.new_fp = Some(ScfiaTransformationFunctionExpression::Bits64(
                    bv.lower_u64(),
                    64,
                ))
            } else if isla_reg_name == "zR30" {
                successor_definition.new_lr = Some(ScfiaTransformationFunctionExpression::Bits64(
                    bv.lower_u64(),
                    64,
                ))
            } else if isla_reg_name == "zSP_EL3" {
                successor_definition.new_sp_el3 = Some(
                    ScfiaTransformationFunctionExpression::Bits64(bv.lower_u64(), 64),
                )
            } else if isla_reg_name == "z_PC" {
                successor_definition.new_pc = Some(bv.lower_u64())
            } else {
                panic!("Unsupported WriteReg {:?} = {:?}", isla_reg_name, bv)
            }
        }
        Val::Symbolic(sym) => {
            if isla_reg_name == "zR0" {
                successor_definition.new_r0 = Some(ScfiaTransformationFunctionExpression::Var(
                    isla_sym_to_transformation_definition_mapping
                        .get(sym)
                        .unwrap()
                        .clone(),
                ))
            } else if isla_reg_name == "zR1" {
                successor_definition.new_r1 = Some(ScfiaTransformationFunctionExpression::Var(
                    isla_sym_to_transformation_definition_mapping
                        .get(sym)
                        .unwrap()
                        .clone(),
                ))
            } else if isla_reg_name == "zR2" {
                successor_definition.new_r2 = Some(ScfiaTransformationFunctionExpression::Var(
                    isla_sym_to_transformation_definition_mapping
                        .get(sym)
                        .unwrap()
                        .clone(),
                ))
            } else if isla_reg_name == "zR3" {
                successor_definition.new_r3 = Some(ScfiaTransformationFunctionExpression::Var(
                    isla_sym_to_transformation_definition_mapping
                        .get(sym)
                        .unwrap()
                        .clone(),
                ))
            } else if isla_reg_name == "zR4" {
                successor_definition.new_r4 = Some(ScfiaTransformationFunctionExpression::Var(
                    isla_sym_to_transformation_definition_mapping
                        .get(sym)
                        .unwrap()
                        .clone(),
                ))
            } else if isla_reg_name == "zR5" {
                successor_definition.new_r5 = Some(ScfiaTransformationFunctionExpression::Var(
                    isla_sym_to_transformation_definition_mapping
                        .get(sym)
                        .unwrap()
                        .clone(),
                ))
            } else if isla_reg_name == "zR6" {
                successor_definition.new_r6 = Some(ScfiaTransformationFunctionExpression::Var(
                    isla_sym_to_transformation_definition_mapping
                        .get(sym)
                        .unwrap()
                        .clone(),
                ))
            } else if isla_reg_name == "zR7" {
                successor_definition.new_r7 = Some(ScfiaTransformationFunctionExpression::Var(
                    isla_sym_to_transformation_definition_mapping
                        .get(sym)
                        .unwrap()
                        .clone(),
                ))
            } else if isla_reg_name == "zR8" {
                successor_definition.new_r8 = Some(ScfiaTransformationFunctionExpression::Var(
                    isla_sym_to_transformation_definition_mapping
                        .get(sym)
                        .unwrap()
                        .clone(),
                ))
            } else if isla_reg_name == "zR9" {
                successor_definition.new_r9 = Some(ScfiaTransformationFunctionExpression::Var(
                    isla_sym_to_transformation_definition_mapping
                        .get(sym)
                        .unwrap()
                        .clone(),
                ))
            } else if isla_reg_name == "zR10" {
                successor_definition.new_r10 = Some(ScfiaTransformationFunctionExpression::Var(
                    isla_sym_to_transformation_definition_mapping
                        .get(sym)
                        .unwrap()
                        .clone(),
                ))
            } else if isla_reg_name == "zR11" {
                successor_definition.new_r11 = Some(ScfiaTransformationFunctionExpression::Var(
                    isla_sym_to_transformation_definition_mapping
                        .get(sym)
                        .unwrap()
                        .clone(),
                ))
            } else if isla_reg_name == "zR12" {
                successor_definition.new_r12 = Some(ScfiaTransformationFunctionExpression::Var(
                    isla_sym_to_transformation_definition_mapping
                        .get(sym)
                        .unwrap()
                        .clone(),
                ))
            } else if isla_reg_name == "zR13" {
                successor_definition.new_r13 = Some(ScfiaTransformationFunctionExpression::Var(
                    isla_sym_to_transformation_definition_mapping
                        .get(sym)
                        .unwrap()
                        .clone(),
                ))
            } else if isla_reg_name == "zR14" {
                successor_definition.new_r14 = Some(ScfiaTransformationFunctionExpression::Var(
                    isla_sym_to_transformation_definition_mapping
                        .get(sym)
                        .unwrap()
                        .clone(),
                ))
            } else if isla_reg_name == "zR15" {
                successor_definition.new_r15 = Some(ScfiaTransformationFunctionExpression::Var(
                    isla_sym_to_transformation_definition_mapping
                        .get(sym)
                        .unwrap()
                        .clone(),
                ))
            } else if isla_reg_name == "zR16" {
                successor_definition.new_r16 = Some(ScfiaTransformationFunctionExpression::Var(
                    isla_sym_to_transformation_definition_mapping
                        .get(sym)
                        .unwrap()
                        .clone(),
                ))
            } else if isla_reg_name == "zR17" {
                successor_definition.new_r17 = Some(ScfiaTransformationFunctionExpression::Var(
                    isla_sym_to_transformation_definition_mapping
                        .get(sym)
                        .unwrap()
                        .clone(),
                ))
            } else if isla_reg_name == "zR18" {
                successor_definition.new_r18 = Some(ScfiaTransformationFunctionExpression::Var(
                    isla_sym_to_transformation_definition_mapping
                        .get(sym)
                        .unwrap()
                        .clone(),
                ))
            } else if isla_reg_name == "zR19" {
                successor_definition.new_r19 = Some(ScfiaTransformationFunctionExpression::Var(
                    isla_sym_to_transformation_definition_mapping
                        .get(sym)
                        .unwrap()
                        .clone(),
                ))
            } else if isla_reg_name == "zR20" {
                successor_definition.new_r20 = Some(ScfiaTransformationFunctionExpression::Var(
                    isla_sym_to_transformation_definition_mapping
                        .get(sym)
                        .unwrap()
                        .clone(),
                ))
            } else if isla_reg_name == "zR21" {
                successor_definition.new_r21 = Some(ScfiaTransformationFunctionExpression::Var(
                    isla_sym_to_transformation_definition_mapping
                        .get(sym)
                        .unwrap()
                        .clone(),
                ))
            } else if isla_reg_name == "zR22" {
                successor_definition.new_r22 = Some(ScfiaTransformationFunctionExpression::Var(
                    isla_sym_to_transformation_definition_mapping
                        .get(sym)
                        .unwrap()
                        .clone(),
                ))
            } else if isla_reg_name == "zR23" {
                successor_definition.new_r23 = Some(ScfiaTransformationFunctionExpression::Var(
                    isla_sym_to_transformation_definition_mapping
                        .get(sym)
                        .unwrap()
                        .clone(),
                ))
            } else if isla_reg_name == "zR24" {
                successor_definition.new_r24 = Some(ScfiaTransformationFunctionExpression::Var(
                    isla_sym_to_transformation_definition_mapping
                        .get(sym)
                        .unwrap()
                        .clone(),
                ))
            } else if isla_reg_name == "zR25" {
                successor_definition.new_r25 = Some(ScfiaTransformationFunctionExpression::Var(
                    isla_sym_to_transformation_definition_mapping
                        .get(sym)
                        .unwrap()
                        .clone(),
                ))
            } else if isla_reg_name == "zR26" {
                successor_definition.new_r26 = Some(ScfiaTransformationFunctionExpression::Var(
                    isla_sym_to_transformation_definition_mapping
                        .get(sym)
                        .unwrap()
                        .clone(),
                ))
            } else if isla_reg_name == "zR27" {
                successor_definition.new_r27 = Some(ScfiaTransformationFunctionExpression::Var(
                    isla_sym_to_transformation_definition_mapping
                        .get(sym)
                        .unwrap()
                        .clone(),
                ))
            } else if isla_reg_name == "zR28" {
                successor_definition.new_r28 = Some(ScfiaTransformationFunctionExpression::Var(
                    isla_sym_to_transformation_definition_mapping
                        .get(sym)
                        .unwrap()
                        .clone(),
                ))
            } else if isla_reg_name == "zR29" {
                successor_definition.new_fp = Some(ScfiaTransformationFunctionExpression::Var(
                    isla_sym_to_transformation_definition_mapping
                        .get(sym)
                        .unwrap()
                        .clone(),
                ))
            } else if isla_reg_name == "zR30" {
                successor_definition.new_lr = Some(ScfiaTransformationFunctionExpression::Var(
                    isla_sym_to_transformation_definition_mapping
                        .get(sym)
                        .unwrap()
                        .clone(),
                ))
            } else {
                panic!("Unsupported WriteReg {:?} = {:?}", isla_reg_name, sym)
            }
        }
        Val::Struct(isla_struct) => {
            if isla_reg_name == "zPSTATE" {
                warn!("handlewritereg pstate {:?}", isla_struct);
                match isla_struct.get(&AARCH64_ISLA_HANDLE.shared_state.symtab.lookup("zN")) {
                    Some(Val::Symbolic(sym)) => {
                        successor_definition.new_pstate_n =
                            Some(ScfiaTransformationFunctionExpression::Var(
                                isla_sym_to_transformation_definition_mapping
                                    .get(sym)
                                    .unwrap()
                                    .clone(),
                            ))
                    }
                    Some(Val::Bits(b)) => {
                        successor_definition.new_pstate_n = Some(
                            ScfiaTransformationFunctionExpression::Bits64(b.lower_u64(), 1),
                        );
                    }
                    x => panic!("{:?}", x),
                }
                match isla_struct.get(&AARCH64_ISLA_HANDLE.shared_state.symtab.lookup("zZ")) {
                    Some(Val::Symbolic(sym)) => {
                        successor_definition.new_pstate_z =
                            Some(ScfiaTransformationFunctionExpression::Var(
                                isla_sym_to_transformation_definition_mapping
                                    .get(sym)
                                    .unwrap()
                                    .clone(),
                            ));
                    }
                    Some(Val::Bits(b)) => {
                        trace!("#### new z = {:?}", b);
                        successor_definition.new_pstate_z = Some(
                            ScfiaTransformationFunctionExpression::Bits64(b.lower_u64(), 1),
                        );
                    }
                    x => panic!("{:?}", x),
                }
                match isla_struct.get(&AARCH64_ISLA_HANDLE.shared_state.symtab.lookup("zC")) {
                    Some(Val::Symbolic(sym)) => {
                        successor_definition.new_pstate_c =
                            Some(ScfiaTransformationFunctionExpression::Var(
                                isla_sym_to_transformation_definition_mapping
                                    .get(sym)
                                    .unwrap()
                                    .clone(),
                            ))
                    }
                    Some(Val::Bits(b)) => {
                        successor_definition.new_pstate_c = Some(
                            ScfiaTransformationFunctionExpression::Bits64(b.lower_u64(), 1),
                        );
                    }
                    x => panic!("{:?}", x),
                }
                match isla_struct.get(&AARCH64_ISLA_HANDLE.shared_state.symtab.lookup("zV")) {
                    Some(Val::Symbolic(sym)) => {
                        successor_definition.new_pstate_v =
                            Some(ScfiaTransformationFunctionExpression::Var(
                                isla_sym_to_transformation_definition_mapping
                                    .get(sym)
                                    .unwrap()
                                    .clone(),
                            ))
                    }
                    Some(Val::Bits(b)) => {
                        successor_definition.new_pstate_v = Some(
                            ScfiaTransformationFunctionExpression::Bits64(b.lower_u64(), 1),
                        );
                    }
                    x => panic!("{:?}", x),
                }
            } else {
                panic!(
                    "Unsupported WriteReg {:?} = {:?}",
                    isla_reg_name, isla_struct
                )
            }
        }
        x => {
            panic!("Unsupported WriteReg {:?} = {:?}", isla_reg_name, x)
        }
    }
}
