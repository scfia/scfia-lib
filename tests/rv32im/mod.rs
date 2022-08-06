use scfia_lib::memory::symbolic_volatile_memory_region::SymbolicVolatileMemoryRegion32;
use std::collections::{BTreeSet, HashSet};
use std::{fs, ptr};
use std::time::Instant;

use scfia_lib::expressions::bool_eq_expression::BoolEqExpression;
use scfia_lib::expressions::bv_and_expression::BVAndExpression;
use scfia_lib::expressions::bv_sub_expression::BVSubExpression;
use scfia_lib::memory::stable_memory_region32::StableMemoryRegion32;
use scfia_lib::memory::MemoryRegion32;
use scfia_lib::memory::volatile_memory_region::VolatileMemoryRegion32;
use scfia_lib::models::riscv::rv32i::RV32iSystemState;
use scfia_lib::values::ActiveValue;
use scfia_lib::values::bit_vector_symbol::BitVectorSymbol;
use scfia_lib::{
    memory::memory32::Memory32, models::riscv::rv32i,
    values::bit_vector_concrete::BitVectorConcrete, ScfiaStdlib,
};
use xmas_elf::program::ProgramHeader::Ph32;
use xmas_elf::{program, ElfFile};
use z3_sys::{Z3_solver_check_assumptions, Z3_solver_check, Z3_solver_get_model, Z3_model_eval, Z3_get_numeral_uint64, Z3_ast, Z3_L_TRUE, Z3_get_bv_sort_size, Z3_get_sort, Z3_get_ast_kind, AstKind, Z3_model_get_const_interp, Z3_get_sort_kind, SortKind, Z3_mk_not, Z3_mk_eq, Z3_mk_unsigned_int64, Z3_mk_bv_sort, Z3_L_FALSE, Z3_inc_ref, Z3_context, Z3_solver, Z3_dec_ref};

fn create_ss() -> RV32iSystemState {
    let binary_blob = fs::read("./tests/rv32i/data/simple_router_risc_v").unwrap();
    let elf = ElfFile::new(&binary_blob).unwrap();

    let mut stdlib = ScfiaStdlib::new("0".to_string());
    let mut memory = Memory32::new();

    for program_header in elf.program_iter() {
        if let Ph32(ph32) = program_header {
            match program_header.get_type().unwrap() {
                program::Type::Load => {
                    let mut i = 0;
                    let mut stable_region =
                        StableMemoryRegion32::new(ph32.virtual_addr as u32, ph32.mem_size as u32);

                    for b in ph32.raw_data(&elf) {
                        stable_region.write(
                            ph32.virtual_addr as u32 + i,
                            BitVectorConcrete::new(
                                *b as u64,
                                8,
                                &mut stdlib,
                                &mut None,
                            ),
                            8,
                            &mut stdlib,
                            &mut None,
                        );
                        i += 1;
                    }

                    memory.stable_memory_regions.push(stable_region);
                }
                x => println!("[WARNING] ignoring section type {:?}", x),
            }
        }
    }

    memory.volatile_memory_regions.push(VolatileMemoryRegion32 {
        start_address: 0x0a003e00,
        length: 200,
    });

    memory.volatile_memory_regions.push(VolatileMemoryRegion32 {
        start_address: 0x0a003c00,
        length: 200,
    });

    let mut rv32i_system_state = RV32iSystemState {
        system_state: rv32i::SystemState {
            x0: BitVectorConcrete::new(0b0, 32, &mut stdlib, &mut None),
            x1: BitVectorConcrete::new(0b0, 32, &mut stdlib, &mut None),
            x2: BitVectorConcrete::new(0b0, 32, &mut stdlib, &mut None),
            x3: BitVectorConcrete::new(0b0, 32, &mut stdlib, &mut None),
            x4: BitVectorConcrete::new(0b0, 32, &mut stdlib, &mut None),
            x5: BitVectorConcrete::new(0b0, 32, &mut stdlib, &mut None),
            x6: BitVectorConcrete::new(0b0, 32, &mut stdlib, &mut None),
            x7: BitVectorConcrete::new(0b0, 32, &mut stdlib, &mut None),
            x8: BitVectorConcrete::new(0b0, 32, &mut stdlib, &mut None),
            x9: BitVectorConcrete::new(0b0, 32, &mut stdlib, &mut None),
            x10: BitVectorConcrete::new(0b0, 32, &mut stdlib, &mut None),
            x11: BitVectorConcrete::new(0b0, 32, &mut stdlib, &mut None),
            x12: BitVectorConcrete::new(0b0, 32, &mut stdlib, &mut None),
            x13: BitVectorConcrete::new(0b0, 32, &mut stdlib, &mut None),
            x14: BitVectorConcrete::new(0b0, 32, &mut stdlib, &mut None),
            x15: BitVectorConcrete::new(0b0, 32, &mut stdlib, &mut None),
            x16: BitVectorConcrete::new(0b0, 32, &mut stdlib, &mut None),
            x17: BitVectorConcrete::new(0b0, 32, &mut stdlib, &mut None),
            x18: BitVectorConcrete::new(0b0, 32, &mut stdlib, &mut None),
            x19: BitVectorConcrete::new(0b0, 32, &mut stdlib, &mut None),
            x20: BitVectorConcrete::new(0b0, 32, &mut stdlib, &mut None),
            x21: BitVectorConcrete::new(0b0, 32, &mut stdlib, &mut None),
            x22: BitVectorConcrete::new(0b0, 32, &mut stdlib, &mut None),
            x23: BitVectorConcrete::new(0b0, 32, &mut stdlib, &mut None),
            x24: BitVectorConcrete::new(0b0, 32, &mut stdlib, &mut None),
            x25: BitVectorConcrete::new(0b0, 32, &mut stdlib, &mut None),
            x26: BitVectorConcrete::new(0b0, 32, &mut stdlib, &mut None),
            x27: BitVectorConcrete::new(0b0, 32, &mut stdlib, &mut None),
            x28: BitVectorConcrete::new(0b0, 32, &mut stdlib, &mut None),
            x29: BitVectorConcrete::new(0b0, 32, &mut stdlib, &mut None),
            x30: BitVectorConcrete::new(0b0, 32, &mut stdlib, &mut None),
            x31: BitVectorConcrete::new(0b0, 32, &mut stdlib, &mut None),
            pc: BitVectorConcrete::new(0x00000004, 32, &mut stdlib, &mut None),
        },
        memory,
        stdlib,
    };

    rv32i_system_state
}

#[test]
fn test_system_state() {
    let binary_blob = fs::read("./tests/rv32im/data/simple_router_risc_v").unwrap();
    let elf = ElfFile::new(&binary_blob).unwrap();

    let mut stdlib = ScfiaStdlib::new("0".to_string());
    let mut memory = Memory32::new();

    for program_header in elf.program_iter() {
        if let Ph32(ph32) = program_header {
            match program_header.get_type().unwrap() {
                program::Type::Load => {
                    println!("{:?}", program_header);
                    let mut i = 0;
                    let mut stable_region =
                        StableMemoryRegion32::new(ph32.virtual_addr as u32, ph32.mem_size as u32);

                    for b in ph32.raw_data(&elf) {
                        stable_region.write(
                            ph32.virtual_addr as u32 + i,
                            BitVectorConcrete::new(
                                *b as u64,
                                8,
                                &mut stdlib,
                                &mut None,
                            ),
                            8,
                            &mut stdlib,
                            &mut None,
                        );
                        i += 1;
                    }

                    memory.stable_memory_regions.push(stable_region);
                }
                x => println!("[WARNING] ignoring section type {:?}", x),
            }
        }
    }

    // ingress nic mmio register
    memory.volatile_memory_regions.push(VolatileMemoryRegion32 {
        start_address: 0x0a003e00,
        length: 200,
    });

    // egress nic mmio register
    memory.volatile_memory_regions.push(VolatileMemoryRegion32 {
        start_address: 0x0a003c00,
        length: 200,
    });

    // ingress nic receivequeue driver area
    memory.volatile_memory_regions.push(VolatileMemoryRegion32 {
        start_address: 0x46004000,
        length: 0x1000,
    });

    // ingress nic receivequeue device area
    memory.volatile_memory_regions.push(VolatileMemoryRegion32 {
        start_address: 0x46005000,
        length: 0x3000,
    });

    // ingress nic receivequeue buffers
    memory.volatile_memory_regions.push(VolatileMemoryRegion32 {
        start_address: 0x46008000,
        length: 0x400000,
    });

    // ingress nic sendqueue driver area
    memory.volatile_memory_regions.push(VolatileMemoryRegion32 {
        start_address: 0x4640c000,
        length: 0x1000,
    });

    // ingress nic sendqueue device area
    memory.volatile_memory_regions.push(VolatileMemoryRegion32 {
        start_address: 0x4640d000,
        length: 0x3000,
    });

    // ingress nic sendqueue buffers
    memory.volatile_memory_regions.push(VolatileMemoryRegion32 {
        start_address: 0x46410000,
        length: 0x400000,
    });

    // egress nic receivequeue driver area
    memory.volatile_memory_regions.push(VolatileMemoryRegion32 {
        start_address: 0x46814000,
        length: 0x1000,
    });

    // egress nic receivequeue device area
    memory.volatile_memory_regions.push(VolatileMemoryRegion32 {
        start_address: 0x46815000,
        length: 0x3000,
    });

    // egress nic receivequeue buffers
    memory.volatile_memory_regions.push(VolatileMemoryRegion32 {
        start_address: 0x46818000,
        length: 0x400000,
    });

    // egress nic sendqueue driver area
    memory.volatile_memory_regions.push(VolatileMemoryRegion32 {
        start_address: 0x46c1c000,
        length: 0x1000,
    });

    // egress nic sendqueue device area
    memory.volatile_memory_regions.push(VolatileMemoryRegion32 {
        start_address: 0x46c1d000,
        length: 0x3000,
    });

    // egress nic sendqueue buffers
    memory.volatile_memory_regions.push(VolatileMemoryRegion32 {
        start_address: 0x46c20000,
        length: 0x400000,
    });

    let mut rv32i_system_state = RV32iSystemState {
        system_state: rv32i::SystemState {
            x0: BitVectorConcrete::new(0b0, 32, &mut stdlib, &mut None),
            x1: BitVectorConcrete::new(0b0, 32, &mut stdlib, &mut None),
            x2: BitVectorConcrete::new(0b0, 32, &mut stdlib, &mut None),
            x3: BitVectorConcrete::new(0b0, 32, &mut stdlib, &mut None),
            x4: BitVectorConcrete::new(0b0, 32, &mut stdlib, &mut None),
            x5: BitVectorConcrete::new(0b0, 32, &mut stdlib, &mut None),
            x6: BitVectorConcrete::new(0b0, 32, &mut stdlib, &mut None),
            x7: BitVectorConcrete::new(0b0, 32, &mut stdlib, &mut None),
            x8: BitVectorConcrete::new(0b0, 32, &mut stdlib, &mut None),
            x9: BitVectorConcrete::new(0b0, 32, &mut stdlib, &mut None),
            x10: BitVectorConcrete::new(0b0, 32, &mut stdlib, &mut None),
            x11: BitVectorConcrete::new(0b0, 32, &mut stdlib, &mut None),
            x12: BitVectorConcrete::new(0b0, 32, &mut stdlib, &mut None),
            x13: BitVectorConcrete::new(0b0, 32, &mut stdlib, &mut None),
            x14: BitVectorConcrete::new(0b0, 32, &mut stdlib, &mut None),
            x15: BitVectorConcrete::new(0b0, 32, &mut stdlib, &mut None),
            x16: BitVectorConcrete::new(0b0, 32, &mut stdlib, &mut None),
            x17: BitVectorConcrete::new(0b0, 32, &mut stdlib, &mut None),
            x18: BitVectorConcrete::new(0b0, 32, &mut stdlib, &mut None),
            x19: BitVectorConcrete::new(0b0, 32, &mut stdlib, &mut None),
            x20: BitVectorConcrete::new(0b0, 32, &mut stdlib, &mut None),
            x21: BitVectorConcrete::new(0b0, 32, &mut stdlib, &mut None),
            x22: BitVectorConcrete::new(0b0, 32, &mut stdlib, &mut None),
            x23: BitVectorConcrete::new(0b0, 32, &mut stdlib, &mut None),
            x24: BitVectorConcrete::new(0b0, 32, &mut stdlib, &mut None),
            x25: BitVectorConcrete::new(0b0, 32, &mut stdlib, &mut None),
            x26: BitVectorConcrete::new(0b0, 32, &mut stdlib, &mut None),
            x27: BitVectorConcrete::new(0b0, 32, &mut stdlib, &mut None),
            x28: BitVectorConcrete::new(0b0, 32, &mut stdlib, &mut None),
            x29: BitVectorConcrete::new(0b0, 32, &mut stdlib, &mut None),
            x30: BitVectorConcrete::new(0b0, 32, &mut stdlib, &mut None),
            x31: BitVectorConcrete::new(0b0, 32, &mut stdlib, &mut None),
            pc: BitVectorConcrete::new(0x4f4, 32, &mut stdlib, &mut None),
        },
        memory,
        stdlib,
    };

    let begin = Instant::now();
    println!("Stepping until NIC1 receivequeue queue_pfn check");
    while rv32i_system_state.system_state.pc.try_borrow().unwrap().as_concrete_bitvector().value != 0x24 {
        rv32i_system_state.step();
    }
    
    let mut successors = rv32i_system_state.step_forking();
    let mut panicking = successors.remove(0);
    let mut continuing = successors.remove(0);

    println!("[{}s] Stepping panic until loop", begin.elapsed().as_secs());
    while panicking.system_state.pc.try_borrow().unwrap().as_concrete_bitvector().value != 0x508 {
        panicking.step()
    }

    println!("[{}s] Stepping until NIC1 receivequeue queue_num_max 0 check", begin.elapsed().as_secs());
    while continuing.system_state.pc.try_borrow().unwrap().as_concrete_bitvector().value != 0x30 {
        continuing.step()
    }

    let mut successors = continuing.step_forking();
    let mut panicking = successors.remove(0);
    let mut continuing = successors.remove(0);

    println!("[{}s] Stepping panic until loop", begin.elapsed().as_secs());
    while panicking.system_state.pc.try_borrow().unwrap().as_concrete_bitvector().value != 0x508 {
        panicking.step()
    }

    println!("[{}s] Stepping until NIC1 receivequeue queue_num_max <1024 check", begin.elapsed().as_secs());
    while continuing.system_state.pc.try_borrow().unwrap().as_concrete_bitvector().value != 0x38 {
        continuing.step()
    }

    let mut successors = continuing.step_forking();
    let mut panicking = successors.remove(0);
    let mut continuing = successors.remove(0);

    println!("[{}s] Stepping panic until loop", begin.elapsed().as_secs());
    while panicking.system_state.pc.try_borrow().unwrap().as_concrete_bitvector().value != 0x508 {
        panicking.step()
    }

    println!("[{}s] Stepping until NIC1 sendqueue configure_virtqueue", begin.elapsed().as_secs());
    while continuing.system_state.pc.try_borrow().unwrap().as_concrete_bitvector().value != 0x4 {
        continuing.step();
    }

    println!("[{}s] Cloning state to free some z3 memory", begin.elapsed().as_secs());
    continuing = continuing.clone();

    println!("[{}s] Stepping until NIC1 sendqueue queue_pfn check", begin.elapsed().as_secs());
    while continuing.system_state.pc.try_borrow().unwrap().as_concrete_bitvector().value != 0x24 {
        continuing.step();
    }

    let mut successors = continuing.step_forking();
    let mut panicking = successors.remove(0);
    let mut continuing = successors.remove(0);

    println!("[{}s] Stepping panic until loop", begin.elapsed().as_secs());
    while panicking.system_state.pc.try_borrow().unwrap().as_concrete_bitvector().value != 0x508 {
        panicking.step()
    }

    println!("[{}s] Stepping until NIC1 sendqueue queue_num_max 0 check", begin.elapsed().as_secs());
    while continuing.system_state.pc.try_borrow().unwrap().as_concrete_bitvector().value != 0x30 {
        continuing.step()
    }

    let mut successors = continuing.step_forking();
    let mut panicking = successors.remove(0);
    let mut continuing = successors.remove(0);

    println!("[{}s] Stepping panic until loop", begin.elapsed().as_secs());
    while panicking.system_state.pc.try_borrow().unwrap().as_concrete_bitvector().value != 0x508 {
        panicking.step()
    }

    println!("[{}s] Stepping until NIC1 sendqueue queue_num_max <1024 check", begin.elapsed().as_secs());
    while continuing.system_state.pc.try_borrow().unwrap().as_concrete_bitvector().value != 0x38 {
        continuing.step()
    }

    let mut successors = continuing.step_forking();
    let mut panicking = successors.remove(0);
    let mut continuing = successors.remove(0);

    println!("[{}s] Stepping panic until loop", begin.elapsed().as_secs());
    while panicking.system_state.pc.try_borrow().unwrap().as_concrete_bitvector().value != 0x508 {
        panicking.step()
    }

    println!("Stepping until NIC2 receivequeue queue_pfn check");
    while continuing.system_state.pc.try_borrow().unwrap().as_concrete_bitvector().value != 0x24 {
        continuing.step();
    }
    
    let mut successors = continuing.step_forking();
    let mut panicking = successors.remove(0);
    let mut continuing = successors.remove(0);

    println!("[{}s] Stepping panic until loop", begin.elapsed().as_secs());
    while panicking.system_state.pc.try_borrow().unwrap().as_concrete_bitvector().value != 0x508 {
        panicking.step()
    }

    println!("[{}s] Stepping until NIC2 receivequeue queue_num_max 0 check", begin.elapsed().as_secs());
    while continuing.system_state.pc.try_borrow().unwrap().as_concrete_bitvector().value != 0x30 {
        continuing.step()
    }

    let mut successors = continuing.step_forking();
    let mut panicking = successors.remove(0);
    let mut continuing = successors.remove(0);

    println!("[{}s] Stepping panic until loop", begin.elapsed().as_secs());
    while panicking.system_state.pc.try_borrow().unwrap().as_concrete_bitvector().value != 0x508 {
        panicking.step()
    }

    println!("[{}s] Stepping until NIC2 receivequeue queue_num_max <1024 check", begin.elapsed().as_secs());
    while continuing.system_state.pc.try_borrow().unwrap().as_concrete_bitvector().value != 0x38 {
        continuing.step()
    }

    let mut successors = continuing.step_forking();
    let mut panicking = successors.remove(0);
    let mut continuing = successors.remove(0);

    println!("[{}s] Stepping panic until loop", begin.elapsed().as_secs());
    while panicking.system_state.pc.try_borrow().unwrap().as_concrete_bitvector().value != 0x508 {
        panicking.step()
    }

    println!("[{}s] Stepping until NIC2 sendqueue configure_virtqueue", begin.elapsed().as_secs());
    while continuing.system_state.pc.try_borrow().unwrap().as_concrete_bitvector().value != 0x4 {
        continuing.step();
    }

    println!("[{}s] Cloning state to free some z3 memory", begin.elapsed().as_secs());
    continuing = continuing.clone();

    println!("[{}s] Stepping until NIC2 sendqueue queue_pfn check", begin.elapsed().as_secs());
    while continuing.system_state.pc.try_borrow().unwrap().as_concrete_bitvector().value != 0x24 {
        continuing.step();
    }

    let mut successors = continuing.step_forking();
    let mut panicking = successors.remove(0);
    let mut continuing = successors.remove(0);

    println!("[{}s] Stepping panic until loop", begin.elapsed().as_secs());
    while panicking.system_state.pc.try_borrow().unwrap().as_concrete_bitvector().value != 0x508 {
        panicking.step()
    }

    println!("[{}s] Stepping until NIC2 sendqueue queue_num_max 0 check", begin.elapsed().as_secs());
    while continuing.system_state.pc.try_borrow().unwrap().as_concrete_bitvector().value != 0x30 {
        continuing.step()
    }

    let mut successors = continuing.step_forking();
    let mut panicking = successors.remove(0);
    let mut continuing = successors.remove(0);

    println!("[{}s] Stepping panic until loop", begin.elapsed().as_secs());
    while panicking.system_state.pc.try_borrow().unwrap().as_concrete_bitvector().value != 0x508 {
        panicking.step()
    }

    println!("[{}s] Stepping until NIC2 sendqueue queue_num_max <1024 check", begin.elapsed().as_secs());
    while continuing.system_state.pc.try_borrow().unwrap().as_concrete_bitvector().value != 0x38 {
        continuing.step()
    }

    let mut successors = continuing.step_forking();
    let mut panicking = successors.remove(0);
    let mut continuing = successors.remove(0);

    println!("[{}s] Stepping panic until loop", begin.elapsed().as_secs());
    while panicking.system_state.pc.try_borrow().unwrap().as_concrete_bitvector().value != 0x508 {
        panicking.step()
    }

    println!("[{}s] Stepping until start of main loop", begin.elapsed().as_secs());
    while continuing.system_state.pc.try_borrow().unwrap().as_concrete_bitvector().value != 0x6E8 {
        continuing.step()
    }

    println!("[{}s] Stepping until ingress try_take fork", begin.elapsed().as_secs());
    while continuing.system_state.pc.try_borrow().unwrap().as_concrete_bitvector().value != 0x428 {
        continuing.step()
    }

    let mut successors = continuing.step_forking();
    let mut continuing = successors.remove(0);
    let mut returning = successors.remove(0);

    println!("[{}s] Stepping aborting until start of main loop", begin.elapsed().as_secs());
    while returning.system_state.pc.try_borrow().unwrap().as_concrete_bitvector().value != 0x6E8 {
        returning.step()
    }

    while continuing.system_state.pc.try_borrow().unwrap().as_concrete_bitvector().value != 0x460 {
        continuing.step()
    }
    println!("[{}s] Monomorphizing a4 to 0x46005004", begin.elapsed().as_secs());
    let mut monomorphizing_candidates = BTreeSet::new();
    monomorphizing_candidates.insert(0x46005004);
    continuing.stdlib.monomorphize(continuing.system_state.x14.try_borrow().unwrap().get_z3_ast(), &mut monomorphizing_candidates);
    assert_eq!(monomorphizing_candidates.len(), 1);
    continuing.system_state.x14 = BitVectorConcrete::new(*monomorphizing_candidates.iter().next().unwrap(), 32, &mut continuing.stdlib, &mut None);

    println!("[{}s] Creating symbolic volatile memory regions", begin.elapsed().as_secs());
    let sym_region = SymbolicVolatileMemoryRegion32 {
        base_symbol: BitVectorSymbol::new(None, 32, Some("SymbolicVolatileMemoryRegion32".into()), &mut continuing.stdlib, &mut None),
        length: 4096,
    };

    println!("[{}s] Writing symbolic pointers to descriptor table", begin.elapsed().as_secs());
    for i in 0..1024 {
        let pointer_offset: u32 = i * 3;
        let ingress_receive_queue_pointer_address = 0x46000000 + pointer_offset;
        continuing.memory.write_concrete(
            ingress_receive_queue_pointer_address,
            sym_region.base_symbol.clone(),
            32,
            &mut continuing.stdlib,
            &mut None);
    }
    continuing.memory.symbolic_volatile_memory_regions.push(sym_region);

    println!("[{}s] HERE BE DRAGONS", begin.elapsed().as_secs());
    loop {
        continuing.step()
    }
}

fn step_all_until(mut states: Vec<RV32iSystemState>, branch_points: Vec<u64>, until: u64) -> Vec<RV32iSystemState> {
    let mut done = vec![];
    while let Some(mut state) = states.pop() {
        if state.system_state.pc.try_borrow().unwrap().as_concrete_bitvector().value != until {
            if branch_points.contains(&state.system_state.pc.try_borrow().unwrap().as_concrete_bitvector().value) {
                let mut forks = state.step_forking();
                states.append(&mut forks);
            } else {
                state.step();
                states.push(state);
            }
        } else {
            done.push(state);
        }
    }
    
    done
}
