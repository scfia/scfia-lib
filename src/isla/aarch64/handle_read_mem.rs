use std::{collections::HashMap, sync::Arc};

use crate::{isla::aarch64::isla_handle::AARCH64_ISLA_HANDLE, symbols::scfia_symbol::ScfiaSymbolId, system_states::aarch64::ScfiaAarch64SystemState};
use isla_lib::{concrete::{bitvector64::B64, BV}, ir::{Name, Val}, smt::{Model, Solver, Sym, smtlib::{Def, Exp, Ty}}};
use log::{debug, trace};

use crate::transformation_functions::aarch64::transformation_definition::ScfiaAarch64TransformationDefinition;
use crate::isla::find_isla_sym_maximum;
use crate::isla::find_isla_sym_minimum;

pub fn handle_read_mem(
    value: &Val<B64>,
    address: &Val<B64>,
    bytes: u32,
    solver: &mut Solver<B64>,
    old_state: &ScfiaAarch64SystemState,
    scfia_symbol_to_isla_sym_mapping: &HashMap<ScfiaSymbolId, Sym>,
    isla_sym_to_transformation_definition_mapping: &mut HashMap<
        Sym,
        Arc<ScfiaAarch64TransformationDefinition>,
    >,
) {
    trace!(
        "handling Event::ReadMem(value={:?}, address={:?}, bytes={})",
        &value,
        &address,
        &bytes
    );
    match address {
        Val::Bits(bv) => {
            // We control the readmem event source, therefore bytes should always be 1
            assert_eq!(bytes, 1);
            let cell_address = bv.lower_u64();

            for region in &old_state.memory.stable_regions {
                if region.range.contains(&cell_address) {
                    match value {
                        Val::Symbolic(_) => {
                            isla_sym_to_transformation_definition_mapping.insert(
                                value.as_sym().unwrap(),
                                Arc::new(ScfiaAarch64TransformationDefinition::ByteFromMemory(
                                    bv.lower_u64(),
                                )),
                            );
                        }
                        _ => {} // concrete reads don't matter for now
                    }
                    return;
                }
            }

            for region in &old_state.memory.volatile_regions {
                if region.range.contains(&cell_address) {
                    isla_sym_to_transformation_definition_mapping.insert(
                        value.as_sym().unwrap(),
                        Arc::new(ScfiaAarch64TransformationDefinition::ByteFromMemory(
                            bv.lower_u64(),
                        )),
                    );
                    return;
                }
            }
        }
        Val::Symbolic(symbolic_address) => {
            let min = find_isla_sym_minimum(*symbolic_address, solver, 64);
            let max = find_isla_sym_maximum(*symbolic_address, solver, 64);
            debug!("handle_read_mem symbolic {} ({}-{})", bytes, min, max);

            // Check whether all models of the symbolic address point to a volatile memory region //TODO add bytes
            for volatile_region in &old_state.memory.volatile_regions {
                if volatile_region.range.start <= min
                    && max < volatile_region.range.end
                {
                    debug!("Event::ReadMem: matched a volatile region");
                    isla_sym_to_transformation_definition_mapping.insert(
                        match value {
                            Val::Symbolic(sym) => *sym,
                            _ => panic!(),
                        },
                        Arc::new(ScfiaAarch64TransformationDefinition::BytesFromMemoryVolatile(bytes)),
                    );
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
                    debug!("Event::ReadMem: Matched a symbolic volatile region");
                    isla_sym_to_transformation_definition_mapping.insert(
                        match value {
                            Val::Symbolic(sym) => *sym,
                            _ => panic!(),
                        },
                        Arc::new(ScfiaAarch64TransformationDefinition::BytesFromMemoryVolatile(
                            bytes,
                        )),
                    );
                    return;
                }
            }

            debug!("Did not match a volatile or symbolic volatile region, monomorphizing candidates");
            let address_symbol = solver.declare_const(Ty::BitVec(64));
            solver.add(Def::Assert(Exp::Eq(
                Box::new(Exp::Var(address_symbol)),
                Box::new(Exp::Var(*symbolic_address)),
            )));

            let mut address_candidates = vec![];
            while solver.check_sat().is_sat().unwrap() {
                let (address, size) = {
                    let mut model = Model::new(solver);
                    match model.get_var(address_symbol) {
                        Ok(Some(Exp::Bits64(address, size))) => (address, size),
                        x => unimplemented!("{:?}", x),
                    }
                };
                address_candidates.push(address);
                solver.add(Def::Assert(Exp::Neq(
                    Box::new(Exp::Var(address_symbol)),
                    Box::new(Exp::Bits64(address, size)),
                )));
                if address_candidates.len() > 1024 {
                    panic!()
                }
            }
            debug!("adding FromMemory for {:?}", symbolic_address);
            let symbolic_value = match value {
                Val::Symbolic(symbolic_value) => symbolic_value,
                x => panic!("{:?}", x),
            };

            if address_candidates.len() > 1 {
                isla_sym_to_transformation_definition_mapping.insert(
                    *symbolic_value,
                    Arc::new(ScfiaAarch64TransformationDefinition::BytesFromMemoryCandidates(
                        address_candidates,
                        bytes
                    )),
                );
            } else {
                isla_sym_to_transformation_definition_mapping.insert(
                    *symbolic_value,
                    Arc::new(ScfiaAarch64TransformationDefinition::ByteFromMemory(
                        address_candidates[0],
                    )),
                );
            }
        }
        x => {
            panic!("{:?}", x);
        }
    }
}
