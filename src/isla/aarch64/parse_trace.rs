use std::{collections::HashMap, sync::Arc};

use isla_lib::{
    concrete::{bitvector64::B64, BV},
    ir::{SharedState, Val},
    smt::{Event, Solver, Sym, Trace},
};
use log::{debug, info, trace};

use crate::{
    isla::{
        aarch64::isla_handle::AARCH64_ISLA_HANDLE,
        map_isla_exp_to_transformation_expression::map_isla_exp_to_transformation_expression,
    },
    symbols::scfia_symbol::{ScfiaSymbol, ScfiaSymbolId},
    system_states::{aarch64::ScfiaAarch64SystemState, ScfiaSystemState},
    transformation_functions::{
        aarch64::{
            transformation_definition::ScfiaAarch64TransformationDefinition,
            transformation_function::ScfiaAarch64TransformationFunctionSuccessorDefinition,
        },
        transformation_function_expression::ScfiaTransformationFunctionExpression,
        ScfiaTransformationDefinition,
    },
};

use super::{
    handle_read_mem::handle_read_mem, handle_read_reg::handle_read_reg, handle_smt::handle_smt,
    handle_write_mem::handle_write_mem, handle_write_reg::handle_write_reg,
};

pub fn aarch64_parse_trace(
    old_state: &ScfiaAarch64SystemState,
    scfia_symbol_to_isla_sym_mapping: &HashMap<ScfiaSymbolId, Sym>,
    mut trace: &Trace<B64>,
    shared_state: &SharedState<B64>,
    end: &Trace<B64>,
    solver: &mut Solver<B64>,
) -> ScfiaAarch64TransformationFunctionSuccessorDefinition {
    let mut successor_definition = ScfiaAarch64TransformationFunctionSuccessorDefinition::new();
    let mut isla_sym_to_transformation_definition_mapping: HashMap<
        Sym,
        Arc<ScfiaAarch64TransformationDefinition>,
    > = HashMap::new(); //TODO check if we have dups and are complete

    let mut isla_asserts = vec![];
    loop {
        for event in &trace.head {
            trace!("{:?}", event);
            match event {
                Event::Smt(def) => handle_smt(
                    def,
                    &mut isla_sym_to_transformation_definition_mapping,
                    &mut isla_asserts,
                ),
                Event::ReadReg(name, _accessor, val) => handle_read_reg(
                    name,
                    val,
                    &mut isla_sym_to_transformation_definition_mapping,
                ),
                Event::WriteReg(name, _accessor, val) => handle_write_reg(
                    name,
                    val,
                    &mut successor_definition,
                    &mut isla_sym_to_transformation_definition_mapping,
                ),
                Event::ReadMem {
                    value,
                    read_kind,
                    address,
                    bytes,
                    tag_value,
                    kind: _,
                } => handle_read_mem(
                    value,
                    address,
                    *bytes,
                    solver,
                    old_state,
                    scfia_symbol_to_isla_sym_mapping,
                    &mut isla_sym_to_transformation_definition_mapping,
                ),
                Event::WriteMem {
                    value,
                    write_kind: _,
                    address,
                    data,
                    bytes,
                    tag_value: _,
                    kind: _,
                } => handle_write_mem(
                    data,
                    address,
                    *bytes,
                    solver,
                    old_state,
                    &mut successor_definition,
                    scfia_symbol_to_isla_sym_mapping,
                    &mut isla_sym_to_transformation_definition_mapping,
                ),
                _ => {}
            }
        }
        if let Some(tail) = &*trace.tail {
            if end as *const Trace<B64> == tail as *const Trace<B64> {
                break;
            }
            trace = tail
        } else {
            break;
        }
    }

    for isla_assert in &isla_asserts {
        match map_isla_exp_to_transformation_expression::<ScfiaAarch64SystemState>(
            isla_assert,
            &mut isla_sym_to_transformation_definition_mapping,
        ) {
            Some(expression) => {
                println!("{:?}", expression);
                successor_definition.new_asserts.push(expression);
            }
            _ => {
                panic!(
                    "Could not map isla assert exp {:?} to transformation_expression",
                    isla_assert
                )
            }
        }
    }

    successor_definition
}
