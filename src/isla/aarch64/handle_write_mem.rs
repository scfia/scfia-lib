use std::{collections::HashMap, sync::Arc};

use crate::{isla::find_isla_sym_maximum, symbols::scfia_symbol::ScfiaSymbolId};
use crate::isla::find_isla_sym_minimum;
use crate::transformation_functions::aarch64::transformation_function_memory_write::ScfiaAarch64TransformationFunctionMemoryWrite;
use crate::{
    isla::aarch64::isla_handle::AARCH64_ISLA_HANDLE,
    system_states::aarch64::ScfiaAarch64SystemState,
    transformation_functions::{
        aarch64::transformation_function::ScfiaAarch64TransformationFunctionSuccessorDefinition,
        transformation_function_expression::ScfiaTransformationFunctionExpression,
    },
};
use isla_lib::{concrete::{bitvector64::B64, BV}, ir::{Name, Val}, smt::{Solver, Sym, smtlib::Exp}};
use log::{debug, trace, warn};

use crate::transformation_functions::aarch64::transformation_definition::ScfiaAarch64TransformationDefinition;

use super::IGNORED_REGISTERS;

pub fn handle_write_mem(
    data: &Val<B64>,
    address: &Val<B64>,
    bytes: u32,
    solver: &mut Solver<B64>,
    old_state: &ScfiaAarch64SystemState,
    successor_definition: &mut ScfiaAarch64TransformationFunctionSuccessorDefinition,
    scfia_symbol_to_isla_sym_mapping: &HashMap<ScfiaSymbolId, Sym>,
    isla_sym_to_transformation_definition_mapping: &mut HashMap<
        Sym,
        Arc<ScfiaAarch64TransformationDefinition>,
    >,
) {
    trace!(
        "handling Event::WriteMem(data={:?}, address={:?}, bytes={})",
        &data,
        &address,
        &bytes
    );
    match address {
        Val::Bits(address) => match data {
            Val::Bits(bv) => {
                successor_definition.memory_writes.push(
                    ScfiaAarch64TransformationFunctionMemoryWrite::WriteMemoryConcrete(
                        address.lower_u64(),
                        bv.lower_u8(),
                    ),
                );
            }
            Val::Symbolic(sym) => {
                successor_definition.memory_writes.push(
                    ScfiaAarch64TransformationFunctionMemoryWrite::WriteMemorySymbolic(
                        address.lower_u64(),
                        ScfiaTransformationFunctionExpression::Var(
                            isla_sym_to_transformation_definition_mapping
                                .get(sym)
                                .unwrap()
                                .clone(),
                        ),
                    ),
                );
            }
            x => panic!("{:?}", x),
        },
        Val::Symbolic(symbolic_address) => {
            let min = find_isla_sym_minimum(*symbolic_address, solver, 64);
            let max = find_isla_sym_maximum(*symbolic_address, solver, 64);
            debug!("handle_write_mem symbolic ({}-{})", min, max);

            // Check volatile memory regions
            for region in &old_state.memory.volatile_regions {
                if region.range.contains(&min) && region.range.contains(&max) {
                    debug!("write to symbolic region {:?}", region);
                    return;
                }
            }

            // Check whether all models of the symbolic address point to a symbolic volatile memory region (and nowhere else!)
            // TODO!!! check whether all bytes fit into the regions! Those readmem events are NOT controlled by us.
            let mut volatile_ranges_check_expression = Box::new(Exp::Bool(false));
            debug!(
                "Event::ReadMem: Checking {} symbolic volatile region(s)",
                old_state.memory.symbolic_volatile_regions.len()
            );
            if old_state.memory.symbolic_volatile_regions.len() > 0 {
                for symbolic_volatile_region in &old_state.memory.symbolic_volatile_regions {
                    let region_sym = *scfia_symbol_to_isla_sym_mapping
                        .get(&symbolic_volatile_region.base_symbol)
                        .unwrap();

                    let check_expression = Box::new(Exp::Or(
                        Box::new(Exp::Bvuge(
                            Box::new(Exp::Var(*symbolic_address)),
                            Box::new(Exp::Bvadd(
                                Box::new(Exp::Var(region_sym)),
                                Box::new(Exp::Bits64(
                                    symbolic_volatile_region.length as u64,
                                    64,
                                )),
                            )),
                        )),
                        Box::new(Exp::Bvult(
                            Box::new(Exp::Var(*symbolic_address)),
                            Box::new(Exp::Var(region_sym)),
                        )),
                    ));
                    //TODO!!! more testcases for this

                    volatile_ranges_check_expression = Box::new(Exp::Or(
                        volatile_ranges_check_expression,
                        check_expression,
                    ));
                }

                if !solver
                    .check_sat_with(&volatile_ranges_check_expression)
                    .is_sat()
                    .unwrap()
                {
                    return;
                }
            }

            panic!()
        }
        x => panic!("{:?}", x),
    }
}
