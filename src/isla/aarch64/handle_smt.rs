use std::{collections::HashMap, sync::Arc};

use isla_lib::{
    concrete::bitvector64::B64,
    smt::{
        smtlib::{Def, Exp, Ty},
        Solver, Sym,
    },
};
use log::warn;

use crate::{
    isla::map_isla_exp_to_transformation_expression::map_isla_exp_to_transformation_expression,
    system_states::aarch64::ScfiaAarch64SystemState,
    transformation_functions::aarch64::transformation_definition::ScfiaAarch64TransformationDefinition,
};

pub fn handle_smt(
    def: &Def,
    isla_sym_to_symbol_definition_mapping: &mut HashMap<
        Sym,
        Arc<ScfiaAarch64TransformationDefinition>,
    >,
    isla_asserts: &mut Vec<Exp>,
) {
    match def {
        Def::DeclareConst(sym, ty) => match ty {
            Ty::BitVec(width) => {
                isla_sym_to_symbol_definition_mapping.insert(
                    *sym,
                    Arc::new(ScfiaAarch64TransformationDefinition::Constant(*width)),
                );
            }
            Ty::Bool => {
                isla_sym_to_symbol_definition_mapping.insert(
                    *sym,
                    Arc::new(ScfiaAarch64TransformationDefinition::ConstantBool),
                );
            }
            _x => {
                //TODO println!("[WARNING] ignoring symbol {:?} ({:?})", sym, x)
            }
        },
        Def::DefineConst(sym, exp) => {
            match map_isla_exp_to_transformation_expression(
                exp,
                isla_sym_to_symbol_definition_mapping,
            ) {
                Some(expression) => {
                    isla_sym_to_symbol_definition_mapping.insert(
                        *sym,
                        Arc::new(ScfiaAarch64TransformationDefinition::FromExpression(
                            Arc::new(expression),
                        )),
                    );
                }
                _ => {}
            }
        }
        Def::Assert(exp) => {
            isla_asserts.push(exp.clone());
        }
        _x => {
            //TODO println!("[WARNING] ignoring def {:?}", x)
        }
    }
}
