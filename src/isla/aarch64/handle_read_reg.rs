use std::{collections::HashMap, sync::Arc};

use crate::isla::aarch64::isla_handle::AARCH64_ISLA_HANDLE;
use isla_lib::{
    concrete::bitvector64::B64,
    ir::{Name, Val},
    smt::Sym,
};

use crate::transformation_functions::aarch64::transformation_definition::ScfiaAarch64TransformationDefinition;

use super::IGNORED_REGISTERS;

/// Handles an ReadReg event from an isla trace.
/// Certain operations cause multiple reads, and the must yield the same symbol!
pub fn handle_read_reg(
    name: &Name,
    val: &Val<B64>,
    isla_sym_to_transformation_definition_mapping: &mut HashMap<
        Sym,
        Arc<ScfiaAarch64TransformationDefinition>,
    >,
) {
    let isla_reg_name = AARCH64_ISLA_HANDLE.shared_state.symtab.to_str(*name);
    if IGNORED_REGISTERS.contains(&isla_reg_name) {
        return;
    }

    match val {
        Val::Symbolic(sym) => {
            if !isla_sym_to_transformation_definition_mapping.contains_key(sym) {
                if isla_reg_name == "zR0" {
                    isla_sym_to_transformation_definition_mapping
                        .insert(*sym, Arc::new(ScfiaAarch64TransformationDefinition::FromR0));
                } else if isla_reg_name == "zR1" {
                    isla_sym_to_transformation_definition_mapping
                        .insert(*sym, Arc::new(ScfiaAarch64TransformationDefinition::FromR1));
                } else if isla_reg_name == "zR2" {
                    isla_sym_to_transformation_definition_mapping
                        .insert(*sym, Arc::new(ScfiaAarch64TransformationDefinition::FromR2));
                } else if isla_reg_name == "zR3" {
                    isla_sym_to_transformation_definition_mapping
                        .insert(*sym, Arc::new(ScfiaAarch64TransformationDefinition::FromR3));
                } else if isla_reg_name == "zR4" {
                    isla_sym_to_transformation_definition_mapping
                        .insert(*sym, Arc::new(ScfiaAarch64TransformationDefinition::FromR4));
                } else if isla_reg_name == "zR5" {
                    isla_sym_to_transformation_definition_mapping
                        .insert(*sym, Arc::new(ScfiaAarch64TransformationDefinition::FromR5));
                } else if isla_reg_name == "zR6" {
                    isla_sym_to_transformation_definition_mapping
                        .insert(*sym, Arc::new(ScfiaAarch64TransformationDefinition::FromR6));
                } else if isla_reg_name == "zR7" {
                    isla_sym_to_transformation_definition_mapping
                        .insert(*sym, Arc::new(ScfiaAarch64TransformationDefinition::FromR7));
                } else if isla_reg_name == "zR8" {
                    isla_sym_to_transformation_definition_mapping
                        .insert(*sym, Arc::new(ScfiaAarch64TransformationDefinition::FromR8));
                } else if isla_reg_name == "zR9" {
                    isla_sym_to_transformation_definition_mapping
                        .insert(*sym, Arc::new(ScfiaAarch64TransformationDefinition::FromR9));
                } else if isla_reg_name == "zR10" {
                    isla_sym_to_transformation_definition_mapping.insert(
                        *sym,
                        Arc::new(ScfiaAarch64TransformationDefinition::FromR10),
                    );
                } else if isla_reg_name == "zR11" {
                    isla_sym_to_transformation_definition_mapping.insert(
                        *sym,
                        Arc::new(ScfiaAarch64TransformationDefinition::FromR11),
                    );
                } else if isla_reg_name == "zR12" {
                    isla_sym_to_transformation_definition_mapping.insert(
                        *sym,
                        Arc::new(ScfiaAarch64TransformationDefinition::FromR12),
                    );
                } else if isla_reg_name == "zR13" {
                    isla_sym_to_transformation_definition_mapping.insert(
                        *sym,
                        Arc::new(ScfiaAarch64TransformationDefinition::FromR13),
                    );
                } else if isla_reg_name == "zR14" {
                    isla_sym_to_transformation_definition_mapping.insert(
                        *sym,
                        Arc::new(ScfiaAarch64TransformationDefinition::FromR14),
                    );
                } else if isla_reg_name == "zR15" {
                    isla_sym_to_transformation_definition_mapping.insert(
                        *sym,
                        Arc::new(ScfiaAarch64TransformationDefinition::FromR15),
                    );
                } else if isla_reg_name == "zR16" {
                    isla_sym_to_transformation_definition_mapping.insert(
                        *sym,
                        Arc::new(ScfiaAarch64TransformationDefinition::FromR16),
                    );
                } else if isla_reg_name == "zR17" {
                    isla_sym_to_transformation_definition_mapping.insert(
                        *sym,
                        Arc::new(ScfiaAarch64TransformationDefinition::FromR17),
                    );
                } else if isla_reg_name == "zR18" {
                    isla_sym_to_transformation_definition_mapping.insert(
                        *sym,
                        Arc::new(ScfiaAarch64TransformationDefinition::FromR18),
                    );
                } else if isla_reg_name == "zR19" {
                    isla_sym_to_transformation_definition_mapping.insert(
                        *sym,
                        Arc::new(ScfiaAarch64TransformationDefinition::FromR19),
                    );
                } else if isla_reg_name == "zR20" {
                    isla_sym_to_transformation_definition_mapping.insert(
                        *sym,
                        Arc::new(ScfiaAarch64TransformationDefinition::FromR20),
                    );
                } else if isla_reg_name == "zR21" {
                    isla_sym_to_transformation_definition_mapping.insert(
                        *sym,
                        Arc::new(ScfiaAarch64TransformationDefinition::FromR21),
                    );
                } else if isla_reg_name == "zR22" {
                    isla_sym_to_transformation_definition_mapping.insert(
                        *sym,
                        Arc::new(ScfiaAarch64TransformationDefinition::FromR22),
                    );
                } else if isla_reg_name == "zR23" {
                    isla_sym_to_transformation_definition_mapping.insert(
                        *sym,
                        Arc::new(ScfiaAarch64TransformationDefinition::FromR23),
                    );
                } else if isla_reg_name == "zR24" {
                    isla_sym_to_transformation_definition_mapping.insert(
                        *sym,
                        Arc::new(ScfiaAarch64TransformationDefinition::FromR24),
                    );
                } else if isla_reg_name == "zR25" {
                    isla_sym_to_transformation_definition_mapping.insert(
                        *sym,
                        Arc::new(ScfiaAarch64TransformationDefinition::FromR25),
                    );
                } else if isla_reg_name == "zR26" {
                    isla_sym_to_transformation_definition_mapping.insert(
                        *sym,
                        Arc::new(ScfiaAarch64TransformationDefinition::FromR26),
                    );
                } else if isla_reg_name == "zR27" {
                    isla_sym_to_transformation_definition_mapping.insert(
                        *sym,
                        Arc::new(ScfiaAarch64TransformationDefinition::FromR27),
                    );
                } else if isla_reg_name == "zR28" {
                    isla_sym_to_transformation_definition_mapping.insert(
                        *sym,
                        Arc::new(ScfiaAarch64TransformationDefinition::FromR28),
                    );
                } else if isla_reg_name == "zR29" {
                    isla_sym_to_transformation_definition_mapping.insert(
                        *sym,
                        Arc::new(ScfiaAarch64TransformationDefinition::FromR29),
                    );
                } else if isla_reg_name == "zR30" {
                    isla_sym_to_transformation_definition_mapping.insert(
                        *sym,
                        Arc::new(ScfiaAarch64TransformationDefinition::FromR30),
                    );
                } else {
                    panic!("Unsupported ReadReg {:?}", isla_reg_name)
                }
            }
        }
        Val::Bits(_) => {}
        Val::Struct(isla_struct) => {
            if isla_reg_name == "zPSTATE" {
                match isla_struct.get(&AARCH64_ISLA_HANDLE.shared_state.symtab.lookup("zN")) {
                    Some(Val::Symbolic(sym)) => {
                        if !isla_sym_to_transformation_definition_mapping.contains_key(sym) {
                            isla_sym_to_transformation_definition_mapping.insert(
                                *sym,
                                Arc::new(ScfiaAarch64TransformationDefinition::FromPstateN),
                            );
                        }
                    }
                    Some(Val::Bits(_)) => {}
                    x => panic!("{:?}", x),
                }
                match isla_struct.get(&AARCH64_ISLA_HANDLE.shared_state.symtab.lookup("zZ")) {
                    Some(Val::Symbolic(sym)) => {
                        if !isla_sym_to_transformation_definition_mapping.contains_key(sym) {
                            isla_sym_to_transformation_definition_mapping.insert(
                                *sym,
                                Arc::new(ScfiaAarch64TransformationDefinition::FromPstateZ),
                            );
                        }
                    }
                    Some(Val::Bits(_)) => {}
                    x => panic!("{:?}", x),
                }
                match isla_struct.get(&AARCH64_ISLA_HANDLE.shared_state.symtab.lookup("zC")) {
                    Some(Val::Symbolic(sym)) => {
                        if !isla_sym_to_transformation_definition_mapping.contains_key(sym) {
                            isla_sym_to_transformation_definition_mapping.insert(
                                *sym,
                                Arc::new(ScfiaAarch64TransformationDefinition::FromPstateC),
                            );
                        }
                    }
                    Some(Val::Bits(_)) => {}
                    x => panic!("{:?}", x),
                }
                match isla_struct.get(&AARCH64_ISLA_HANDLE.shared_state.symtab.lookup("zV")) {
                    Some(Val::Symbolic(sym)) => {
                        if !isla_sym_to_transformation_definition_mapping.contains_key(sym) {
                            isla_sym_to_transformation_definition_mapping.insert(
                                *sym,
                                Arc::new(ScfiaAarch64TransformationDefinition::FromPstateV),
                            );
                        }
                    }
                    Some(Val::Bits(_)) => {}
                    x => panic!("{:?}", x),
                }
            } else {
                panic!("Unsupported ReadReg {:?}", isla_reg_name)
            }
        }
        x => panic!("Unsupported ReadReg {:?} ({:?})", isla_reg_name, x),
    }
}
