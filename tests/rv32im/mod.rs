mod constants;

use std::fs;
use std::time::Instant;

use log::{debug, info, trace, warn, LevelFilter};
use scfia_lib::memory::regions::{StableMemoryRegion, VolatileMemoryRegion};
use scfia_lib::memory::Memory;
use scfia_lib::models::riscv::rv32i;
use scfia_lib::models::riscv::rv32i::RV32i;
use scfia_lib::scfia::Scfia;
use xmas_elf::program::ProgramHeader::Ph32;
use xmas_elf::{program, ElfFile};
use z3_sys::{
    AstKind, SortKind, Z3_ast, Z3_context, Z3_dec_ref, Z3_get_ast_kind, Z3_get_bv_sort_size, Z3_get_numeral_uint64, Z3_get_sort, Z3_get_sort_kind, Z3_inc_ref,
    Z3_mk_bv_sort, Z3_mk_eq, Z3_mk_not, Z3_mk_unsigned_int64, Z3_model_eval, Z3_model_get_const_interp, Z3_solver, Z3_solver_check,
    Z3_solver_check_assumptions, Z3_solver_get_model, Z3_L_FALSE, Z3_L_TRUE,
};

use crate::rv32im::constants::{
    COPY_FROM_1, COPY_FROM_2, COPY_FROM_3, EGRESS_RECEIVEQUEUE_DRIVER_POSITIONS, EGRESS_SENDQUEUE_DRIVER_POSITIONS,
    INGRESS_RECEIVEQUEUE_DESCRIPTOR_ADDRESS_HIGHER_U32, INGRESS_RECEIVEQUEUE_DESCRIPTOR_ADDRESS_LOWER_U32, INGRESS_RECEIVEQUEUE_DESCRIPTOR_LENGTH,
    INGRESS_RECEIVEQUEUE_DRIVER_POSITIONS, INGRESS_SENDQUEUE_DRIVER_POSITIONS, START_OF_MAIN_LOOP,
};

#[test]
fn test_system_state() {
    simple_logger::SimpleLogger::new().with_level(LevelFilter::Trace).env().init().unwrap();
    let binary_blob = fs::read("./tests/rv32im/data/simple_router_risc_v").unwrap();
    let elf = ElfFile::new(&binary_blob).unwrap();

    let scfia = Scfia::default();
    let mut memory = Memory::default();

    for program_header in elf.program_iter() {
        if let Ph32(ph32) = program_header {
            match program_header.get_type().unwrap() {
                program::Type::Load => {
                    trace!("{:?}", program_header);
                    let mut i = 0;
                    let stable_region = StableMemoryRegion::new(ph32.virtual_addr as u64, ph32.mem_size as u64);
                    memory.stables.push(stable_region);

                    for b in ph32.raw_data(&elf) {
                        memory.write(
                            scfia.new_bv_concrete(ph32.virtual_addr as u64 + i, 8),
                            scfia.new_bv_concrete(*b as u64, 8),
                            8,
                            scfia.clone(),
                            &mut None,
                        );
                        i += 1;
                    }
                }
                x => warn!("Ignoring section type {:?}", x),
            }
        }
    }

    // ingress nic mmio register
    memory.volatiles.push(VolatileMemoryRegion {
        start_address: 0x0a003e00,
        length: 200,
    });

    // egress nic mmio register
    memory.volatiles.push(VolatileMemoryRegion {
        start_address: 0x0a003c00,
        length: 200,
    });

    // ingress nic receivequeue driver area
    memory.volatiles.push(VolatileMemoryRegion {
        start_address: 0x46004000,
        length: 0x1000,
    });

    // ingress nic receivequeue device area
    memory.volatiles.push(VolatileMemoryRegion {
        start_address: 0x46005000,
        length: 0x3000,
    });

    // ingress nic receivequeue buffers
    memory.volatiles.push(VolatileMemoryRegion {
        start_address: 0x46008000,
        length: 0x400000,
    });

    // ingress nic sendqueue driver area
    memory.volatiles.push(VolatileMemoryRegion {
        start_address: 0x4640c000,
        length: 0x1000,
    });

    // ingress nic sendqueue device area
    memory.volatiles.push(VolatileMemoryRegion {
        start_address: 0x4640d000,
        length: 0x3000,
    });

    // ingress nic sendqueue buffers
    memory.volatiles.push(VolatileMemoryRegion {
        start_address: 0x46410000,
        length: 0x400000,
    });

    // egress nic receivequeue driver area
    memory.volatiles.push(VolatileMemoryRegion {
        start_address: 0x46814000,
        length: 0x1000,
    });

    // egress nic receivequeue device area
    memory.volatiles.push(VolatileMemoryRegion {
        start_address: 0x46815000,
        length: 0x3000,
    });

    // egress nic receivequeue buffers
    memory.volatiles.push(VolatileMemoryRegion {
        start_address: 0x46818000,
        length: 0x400000,
    });

    // egress nic sendqueue driver area
    memory.volatiles.push(VolatileMemoryRegion {
        start_address: 0x46c1c000,
        length: 0x1000,
    });

    // egress nic sendqueue device area
    memory.volatiles.push(VolatileMemoryRegion {
        start_address: 0x46c1d000,
        length: 0x3000,
    });

    // egress nic sendqueue buffers
    memory.volatiles.push(VolatileMemoryRegion {
        start_address: 0x46c20000,
        length: 0x400000,
    });

    let mut rv32i_system_state = RV32i {
        state: rv32i::SystemState {
            x0: scfia.new_bv_concrete(0b0, 32),
            x1: scfia.new_bv_concrete(0b0, 32),
            x2: scfia.new_bv_concrete(0b0, 32),
            x3: scfia.new_bv_concrete(0b0, 32),
            x4: scfia.new_bv_concrete(0b0, 32),
            x5: scfia.new_bv_concrete(0b0, 32),
            x6: scfia.new_bv_concrete(0b0, 32),
            x7: scfia.new_bv_concrete(0b0, 32),
            x8: scfia.new_bv_concrete(0b0, 32),
            x9: scfia.new_bv_concrete(0b0, 32),
            x10: scfia.new_bv_concrete(0b0, 32),
            x11: scfia.new_bv_concrete(0b0, 32),
            x12: scfia.new_bv_concrete(0b0, 32),
            x13: scfia.new_bv_concrete(0b0, 32),
            x14: scfia.new_bv_concrete(0b0, 32),
            x15: scfia.new_bv_concrete(0b0, 32),
            x16: scfia.new_bv_concrete(0b0, 32),
            x17: scfia.new_bv_concrete(0b0, 32),
            x18: scfia.new_bv_concrete(0b0, 32),
            x19: scfia.new_bv_concrete(0b0, 32),
            x20: scfia.new_bv_concrete(0b0, 32),
            x21: scfia.new_bv_concrete(0b0, 32),
            x22: scfia.new_bv_concrete(0b0, 32),
            x23: scfia.new_bv_concrete(0b0, 32),
            x24: scfia.new_bv_concrete(0b0, 32),
            x25: scfia.new_bv_concrete(0b0, 32),
            x26: scfia.new_bv_concrete(0b0, 32),
            x27: scfia.new_bv_concrete(0b0, 32),
            x28: scfia.new_bv_concrete(0b0, 32),
            x29: scfia.new_bv_concrete(0b0, 32),
            x30: scfia.new_bv_concrete(0b0, 32),
            x31: scfia.new_bv_concrete(0b0, 32),
            pc: scfia.new_bv_concrete(0x4f4, 32),
        },
        memory,
        scfia,
    };

    info!("Stepping until NIC1 receivequeue queue_pfn check");
    while rv32i_system_state.state.pc.try_borrow().unwrap().try_as_concrete_bv().unwrap() != 0x24 {
        debug!(
            "Executing 0x{:x}",
            rv32i_system_state.state.pc.try_borrow().unwrap().try_as_concrete_bv().unwrap()
        );
        rv32i_system_state.step(None);
    }

    rv32i_system_state.clone_model();

    /*
    let mut successors = rv32i_system_state.step_forking();
    let mut panicking = successors.remove(0);
    let mut continuing = successors.remove(0);

    println!("[{}s] ### Stepping panic until loop", begin.elapsed().as_millis());
    while panicking.system_state.pc.try_borrow().unwrap().as_concrete_bitvector().value != 0x508 {
        print!("({}ms) ", begin.elapsed().as_millis());
        panicking.step(None)
    }

    println!("[{}s] ### Stepping until NIC1 receivequeue queue_num_max 0 check", begin.elapsed().as_millis());
    while continuing.system_state.pc.try_borrow().unwrap().as_concrete_bitvector().value != 0x30 {
        print!("({}ms) ", begin.elapsed().as_millis());
        continuing.step(None)
    }

    let mut successors = continuing.step_forking();
    let mut panicking = successors.remove(0);
    let mut continuing = successors.remove(0);

    println!("[{}s] ### Stepping panic until loop", begin.elapsed().as_millis());
    while panicking.system_state.pc.try_borrow().unwrap().as_concrete_bitvector().value != 0x508 {
        print!("({}ms) ", begin.elapsed().as_millis());
        panicking.step(None)
    }

    println!(
        "[{}s] ### Stepping until NIC1 receivequeue queue_num_max <1024 check",
        begin.elapsed().as_millis()
    );
    while continuing.system_state.pc.try_borrow().unwrap().as_concrete_bitvector().value != 0x38 {
        print!("({}ms) ", begin.elapsed().as_millis());
        continuing.step(None)
    }

    let mut successors = continuing.step_forking();
    let mut panicking = successors.remove(0);
    let mut continuing = successors.remove(0);

    println!("[{}s] ### Stepping panic until loop", begin.elapsed().as_millis());
    while panicking.system_state.pc.try_borrow().unwrap().as_concrete_bitvector().value != 0x508 {
        print!("({}ms) ", begin.elapsed().as_millis());
        panicking.step(None)
    }

    println!("[{}s] ### Stepping until NIC1 sendqueue configure_virtqueue", begin.elapsed().as_millis());
    while continuing.system_state.pc.try_borrow().unwrap().as_concrete_bitvector().value != 0x4 {
        print!("({}ms) ", begin.elapsed().as_millis());
        if continuing.system_state.pc.try_borrow().unwrap().as_concrete_bitvector().value == 0x3dc {
            continuing.step(Some(SymbolicHints {
                hints: vec![INGRESS_RECEIVEQUEUE_DRIVER_POSITIONS.to_vec()],
            }));
        } else {
            continuing.step(None);
        }
    }

    println!("[{}s] ### Cloning state to free some z3 memory", begin.elapsed().as_millis());
    continuing = continuing.clone();

    println!("[{}s] ### Stepping until NIC1 sendqueue queue_pfn check", begin.elapsed().as_millis());
    while continuing.system_state.pc.try_borrow().unwrap().as_concrete_bitvector().value != 0x24 {
        print!("({}ms) ", begin.elapsed().as_millis());
        continuing.step(None);
    }

    let mut successors = continuing.step_forking();
    let mut panicking = successors.remove(0);
    let mut continuing = successors.remove(0);

    println!("[{}s] ### Stepping panic until loop", begin.elapsed().as_millis());
    while panicking.system_state.pc.try_borrow().unwrap().as_concrete_bitvector().value != 0x508 {
        print!("({}ms) ", begin.elapsed().as_millis());
        panicking.step(None)
    }

    println!("[{}s] ### Stepping until NIC1 sendqueue queue_num_max 0 check", begin.elapsed().as_millis());
    while continuing.system_state.pc.try_borrow().unwrap().as_concrete_bitvector().value != 0x30 {
        print!("({}ms) ", begin.elapsed().as_millis());
        continuing.step(None)
    }

    let mut successors = continuing.step_forking();
    let mut panicking = successors.remove(0);
    let mut continuing = successors.remove(0);

    println!("[{}s] ### Stepping panic until loop", begin.elapsed().as_millis());
    while panicking.system_state.pc.try_borrow().unwrap().as_concrete_bitvector().value != 0x508 {
        print!("({}ms) ", begin.elapsed().as_millis());
        panicking.step(None)
    }

    println!("[{}s] ### Stepping until NIC1 sendqueue queue_num_max <1024 check", begin.elapsed().as_millis());
    while continuing.system_state.pc.try_borrow().unwrap().as_concrete_bitvector().value != 0x38 {
        print!("({}ms) ", begin.elapsed().as_millis());
        continuing.step(None)
    }

    let mut successors = continuing.step_forking();
    let mut panicking = successors.remove(0);
    let mut continuing = successors.remove(0);

    println!("[{}s] ### Stepping panic until loop", begin.elapsed().as_millis());
    while panicking.system_state.pc.try_borrow().unwrap().as_concrete_bitvector().value != 0x508 {
        print!("({}ms) ", begin.elapsed().as_millis());
        panicking.step(None)
    }

    println!("[{}s] ### Stepping until NIC2 receivequeue queue_pfn check", begin.elapsed().as_millis());
    while continuing.system_state.pc.try_borrow().unwrap().as_concrete_bitvector().value != 0x24 {
        print!("({}ms) ", begin.elapsed().as_millis());
        if continuing.system_state.pc.try_borrow().unwrap().as_concrete_bitvector().value == 0x3dc {
            continuing.step(Some(SymbolicHints {
                hints: vec![INGRESS_SENDQUEUE_DRIVER_POSITIONS.to_vec()],
            }));
        } else {
            continuing.step(None);
        }
    }

    let mut successors = continuing.step_forking();
    let mut panicking = successors.remove(0);
    let mut continuing = successors.remove(0);

    println!("[{}s] ### Stepping panic until loop", begin.elapsed().as_millis());
    while panicking.system_state.pc.try_borrow().unwrap().as_concrete_bitvector().value != 0x508 {
        print!("({}ms) ", begin.elapsed().as_millis());
        panicking.step(None)
    }

    println!("[{}s] ### Stepping until NIC2 receivequeue queue_num_max 0 check", begin.elapsed().as_millis());
    while continuing.system_state.pc.try_borrow().unwrap().as_concrete_bitvector().value != 0x30 {
        print!("({}ms) ", begin.elapsed().as_millis());
        continuing.step(None)
    }

    let mut successors = continuing.step_forking();
    let mut panicking = successors.remove(0);
    let mut continuing = successors.remove(0);

    println!("[{}s] ### Stepping panic until loop", begin.elapsed().as_millis());
    while panicking.system_state.pc.try_borrow().unwrap().as_concrete_bitvector().value != 0x508 {
        print!("({}ms) ", begin.elapsed().as_millis());
        panicking.step(None)
    }

    println!(
        "[{}s] ### Stepping until NIC2 receivequeue queue_num_max <1024 check",
        begin.elapsed().as_millis()
    );
    while continuing.system_state.pc.try_borrow().unwrap().as_concrete_bitvector().value != 0x38 {
        print!("({}ms) ", begin.elapsed().as_millis());
        continuing.step(None)
    }

    let mut successors = continuing.step_forking();
    let mut panicking = successors.remove(0);
    let mut continuing = successors.remove(0);

    println!("[{}s] ### Stepping panic until loop", begin.elapsed().as_millis());
    while panicking.system_state.pc.try_borrow().unwrap().as_concrete_bitvector().value != 0x508 {
        print!("({}ms) ", begin.elapsed().as_millis());
        panicking.step(None)
    }

    println!("[{}s] ### Stepping until NIC2 sendqueue configure_virtqueue", begin.elapsed().as_millis());
    while continuing.system_state.pc.try_borrow().unwrap().as_concrete_bitvector().value != 0x4 {
        print!("({}ms) ", begin.elapsed().as_millis());
        if continuing.system_state.pc.try_borrow().unwrap().as_concrete_bitvector().value == 0x3dc {
            continuing.step(Some(SymbolicHints {
                hints: vec![EGRESS_RECEIVEQUEUE_DRIVER_POSITIONS.to_vec()],
            }));
        } else {
            continuing.step(None);
        }
    }

    println!("[{}s] ### Cloning state to free some z3 memory", begin.elapsed().as_millis());
    continuing = continuing.clone();

    println!("[{}s] ### Stepping until NIC2 sendqueue queue_pfn check", begin.elapsed().as_millis());
    print!("({}ms) ", begin.elapsed().as_millis());
    while continuing.system_state.pc.try_borrow().unwrap().as_concrete_bitvector().value != 0x24 {
        print!("({}ms) ", begin.elapsed().as_millis());
        continuing.step(None)
    }

    let mut successors = continuing.step_forking();
    let mut panicking = successors.remove(0);
    let mut continuing = successors.remove(0);

    println!("[{}s] ### Stepping panic until loop", begin.elapsed().as_millis());
    while panicking.system_state.pc.try_borrow().unwrap().as_concrete_bitvector().value != 0x508 {
        print!("({}ms) ", begin.elapsed().as_millis());
        panicking.step(None)
    }

    println!("[{}s] ### Stepping until NIC2 sendqueue queue_num_max 0 check", begin.elapsed().as_millis());
    while continuing.system_state.pc.try_borrow().unwrap().as_concrete_bitvector().value != 0x30 {
        print!("({}ms) ", begin.elapsed().as_millis());
        continuing.step(None)
    }

    let mut successors = continuing.step_forking();
    let mut panicking = successors.remove(0);
    let mut continuing = successors.remove(0);

    println!("[{}s] ### Stepping panic until loop", begin.elapsed().as_millis());
    while panicking.system_state.pc.try_borrow().unwrap().as_concrete_bitvector().value != 0x508 {
        print!("({}ms) ", begin.elapsed().as_millis());
        panicking.step(None)
    }

    println!("[{}s] ### Stepping until NIC2 sendqueue queue_num_max <1024 check", begin.elapsed().as_millis());
    while continuing.system_state.pc.try_borrow().unwrap().as_concrete_bitvector().value != 0x38 {
        print!("({}ms) ", begin.elapsed().as_millis());
        continuing.step(None)
    }

    let mut successors = continuing.step_forking();
    let mut panicking = successors.remove(0);
    let mut continuing = successors.remove(0);

    println!("[{}s] ### Stepping panic until loop", begin.elapsed().as_millis());
    while panicking.system_state.pc.try_borrow().unwrap().as_concrete_bitvector().value != 0x508 {
        print!("({}ms) ", begin.elapsed().as_millis());
        panicking.step(None)
    }

    println!("[{}s] ### Stepping until start of main loop", begin.elapsed().as_millis());
    while continuing.system_state.pc.try_borrow().unwrap().as_concrete_bitvector().value != START_OF_MAIN_LOOP {
        print!("({}ms) ", begin.elapsed().as_millis());
        if continuing.system_state.pc.try_borrow().unwrap().as_concrete_bitvector().value == 0x3dc {
            continuing.step(Some(SymbolicHints {
                hints: vec![EGRESS_SENDQUEUE_DRIVER_POSITIONS.to_vec()],
            }));
        } else {
            continuing.step(None);
        }
    }

    println!("[{}s] ### Stepping until ingress try_remove fork", begin.elapsed().as_millis());
    while continuing.system_state.pc.try_borrow().unwrap().as_concrete_bitvector().value != 0x428 {
        print!("({}ms) ", begin.elapsed().as_millis());
        continuing.step(None)
    }

    let mut successors = continuing.step_forking();
    let mut continuing = successors.remove(0);
    let mut returning = successors.remove(0);

    println!("[{}s] ### Stepping aborting until start of main loop", begin.elapsed().as_millis());
    while returning.system_state.pc.try_borrow().unwrap().as_concrete_bitvector().value != START_OF_MAIN_LOOP {
        print!("({}ms) ", begin.elapsed().as_millis());
        returning.step(None)
    }

    while continuing.system_state.pc.try_borrow().unwrap().as_concrete_bitvector().value != 0x460 {
        print!("({}ms) ", begin.elapsed().as_millis());
        continuing.step(None)
    }
    println!("[{}s] ### Monomorphizing a4 to 0x46005004", begin.elapsed().as_millis());
    let mut monomorphizing_candidates = vec![0x46005004];
    continuing
        .stdlib
        .monomorphize(continuing.system_state.x14.try_borrow().unwrap().get_z3_ast(), &mut monomorphizing_candidates);
    assert_eq!(monomorphizing_candidates.len(), 1);
    continuing.system_state.x14 = BitVectorConcrete::new(*monomorphizing_candidates.iter().next().unwrap(), 32, &mut continuing.stdlib, &mut None);

    println!("[{}s] ### Creating symbolic volatile memory regions", begin.elapsed().as_millis());
    let base_symbol = BitVectorSymbol::new(None, 32, Some("SymbolicVolatileMemoryRegionBase".into()), &mut continuing.stdlib, &mut None);
    {
        let base_symbol_and_expr = BVAndExpression::new(
            base_symbol.clone(),
            BitVectorConcrete::new(0xff, 32, &mut continuing.stdlib, &mut None),
            &mut continuing.stdlib,
            &mut None,
        );
        let expr = BoolEqExpression::new(
            BitVectorConcrete::new(0, 32, &mut continuing.stdlib, &mut None),
            base_symbol_and_expr.clone(),
            &mut continuing.stdlib,
            &mut None,
        );
        expr.try_borrow_mut().unwrap().assert(&mut continuing.stdlib);
    }

    {
        // constrain to < 0xFFFF0000 (to prevent wrapping around 0xFFFFFFFF)
        let expr = BoolLessThanUIntExpression::new(
            base_symbol.clone(),
            BitVectorConcrete::new(0xFFFF0000, 32, &mut continuing.stdlib, &mut None),
            &mut continuing.stdlib,
            &mut None,
        );
        expr.try_borrow_mut().unwrap().assert(&mut continuing.stdlib);
    }

    let sym_region = SymbolicVolatileMemoryRegion {
        base_symbol: base_symbol,
        length: 4096,
    };

    println!("[{}s] ### Writing symbolic pointers to descriptor table", begin.elapsed().as_millis());
    // TODO ensure is valid generalization
    for i in 0..1024 {
        let pointer_offset: u32 = i * 16;
        let ingress_receive_queue_pointer_address = 0x46000000 + pointer_offset;
        println!(
            "[{}s] ### overwriting 0x{:x}",
            begin.elapsed().as_millis(),
            ingress_receive_queue_pointer_address
        );
        continuing.memory.write_concrete(
            ingress_receive_queue_pointer_address,
            sym_region.base_symbol.clone(),
            32,
            &mut continuing.stdlib,
            &mut None,
        );

        let egress_send_queue_pointer_address = 0x46c18000 + pointer_offset;
        println!("[{}s] ### overwriting 0x{:x}", begin.elapsed().as_millis(), egress_send_queue_pointer_address);
        continuing.memory.write_concrete(
            egress_send_queue_pointer_address,
            sym_region.base_symbol.clone(),
            32,
            &mut continuing.stdlib,
            &mut None,
        );
    }
    continuing.memory.symbolic_volatiles.push(sym_region);

    println!("[{}s] ### Stepping until ethertype ipv4 check", begin.elapsed().as_millis());
    while continuing.system_state.pc.try_borrow().unwrap().as_concrete_bitvector().value != 0x73c {
        print!("({}ms) ", begin.elapsed().as_millis());
        if continuing.system_state.pc.try_borrow().unwrap().as_concrete_bitvector().value == 0x49C {
            continuing.step(Some(SymbolicHints {
                hints: vec![INGRESS_RECEIVEQUEUE_DESCRIPTOR_ADDRESS_HIGHER_U32.to_vec()],
            }));
        } else if continuing.system_state.pc.try_borrow().unwrap().as_concrete_bitvector().value == 0x4a0 {
            continuing.step(Some(SymbolicHints {
                hints: vec![INGRESS_RECEIVEQUEUE_DESCRIPTOR_ADDRESS_LOWER_U32.to_vec()],
            }));
        } else if continuing.system_state.pc.try_borrow().unwrap().as_concrete_bitvector().value == 0x4a8 {
            continuing.step(None);
            println!("a3={:?}", continuing.system_state.x13);
            let mut candidates = vec![];
            continuing
                .stdlib
                .monomorphize(continuing.system_state.x13.try_borrow().unwrap().get_z3_ast(), &mut candidates);
            println!("a3 candidates={:?}", candidates)
        } else if continuing.system_state.pc.try_borrow().unwrap().as_concrete_bitvector().value == 0x4a4 {
            continuing.step(Some(SymbolicHints {
                hints: vec![INGRESS_RECEIVEQUEUE_DESCRIPTOR_LENGTH.to_vec()],
            }));
        } else if continuing.system_state.pc.try_borrow().unwrap().as_concrete_bitvector().value == 0x4b0 {
            continuing.step(Some(SymbolicHints {
                hints: vec![INGRESS_RECEIVEQUEUE_DESCRIPTOR_ADDRESS_HIGHER_U32.to_vec()],
            }));
        } else if continuing.system_state.pc.try_borrow().unwrap().as_concrete_bitvector().value == 0x4b4 {
            continuing.step(Some(SymbolicHints {
                hints: vec![INGRESS_RECEIVEQUEUE_DESCRIPTOR_ADDRESS_LOWER_U32.to_vec()],
            }));
        } else {
            continuing.step(None);
        }
    }

    let mut successors = continuing.step_forking();
    let mut returning = successors.remove(0);
    let mut continuing = successors.remove(0);

    println!("[{}s] ### Stepping not ipv4 until start of main loop", begin.elapsed().as_millis());
    while returning.system_state.pc.try_borrow().unwrap().as_concrete_bitvector().value != START_OF_MAIN_LOOP {
        print!("({}ms) ", begin.elapsed().as_millis());
        if returning.system_state.pc.try_borrow().unwrap().as_concrete_bitvector().value == 0x3DC {
            returning.step(Some(SymbolicHints {
                hints: vec![INGRESS_RECEIVEQUEUE_DRIVER_POSITIONS.to_vec()],
            }));
        } else {
            returning.step(None);
        }
    }

    println!("[{}s] ### Stepping until egress try_remove fork", begin.elapsed().as_millis());
    while continuing.system_state.pc.try_borrow().unwrap().as_concrete_bitvector().value != 0x428 {
        print!("({}ms) ", begin.elapsed().as_millis());
        continuing.step(None);
    }

    let mut successors = continuing.step_forking();
    let mut continuing = successors.remove(0);
    let mut returning = successors.remove(0);

    println!("[{}s] ### Stepping egress empty until start of main loop", begin.elapsed().as_millis());
    while returning.system_state.pc.try_borrow().unwrap().as_concrete_bitvector().value != START_OF_MAIN_LOOP {
        print!("({}ms) ", begin.elapsed().as_millis());
        if returning.system_state.pc.try_borrow().unwrap().as_concrete_bitvector().value == 0x3DC {
            returning.step(Some(SymbolicHints {
                hints: vec![INGRESS_RECEIVEQUEUE_DRIVER_POSITIONS.to_vec()],
            }));
        } else {
            returning.step(None);
        }
    }

    println!("[{}s] ### stepping success until start of main loop", begin.elapsed().as_millis());
    while continuing.system_state.pc.try_borrow().unwrap().as_concrete_bitvector().value != START_OF_MAIN_LOOP {
        print!("({}ms) ", begin.elapsed().as_millis());
        if continuing.system_state.pc.try_borrow().unwrap().as_concrete_bitvector().value == 0x3DC {
            continuing.step(Some(SymbolicHints { hints: vec![vec![0x46c1d004]] }));
        } else if continuing.system_state.pc.try_borrow().unwrap().as_concrete_bitvector().value == 0x460 {
            continuing.step(Some(SymbolicHints { hints: vec![vec![0x46c1d004]] }));
        } else if continuing.system_state.pc.try_borrow().unwrap().as_concrete_bitvector().value == 0x150 {
            continuing.step(Some(SymbolicHints {
                hints: vec![COPY_FROM_1.to_vec()],
            }));
        } else if continuing.system_state.pc.try_borrow().unwrap().as_concrete_bitvector().value == 0x154 {
            continuing.step(Some(SymbolicHints {
                hints: vec![COPY_FROM_2.to_vec()],
            }));
        } else if continuing.system_state.pc.try_borrow().unwrap().as_concrete_bitvector().value == 0x158 {
            continuing.step(Some(SymbolicHints {
                hints: vec![COPY_FROM_3.to_vec()],
            }));
        } else if continuing.system_state.pc.try_borrow().unwrap().as_concrete_bitvector().value == 0x160 {
            continuing.step(Some(SymbolicHints {
                hints: vec![INGRESS_RECEIVEQUEUE_DESCRIPTOR_ADDRESS_HIGHER_U32.to_vec()],
            }));
        } else if continuing.system_state.pc.try_borrow().unwrap().as_concrete_bitvector().value == 0x164 {
            continuing.step(Some(SymbolicHints {
                hints: vec![INGRESS_RECEIVEQUEUE_DESCRIPTOR_ADDRESS_LOWER_U32.to_vec()],
            }));
        } else if continuing.system_state.pc.try_borrow().unwrap().as_concrete_bitvector().value == 0x168 {
            continuing.step(Some(SymbolicHints {
                hints: vec![COPY_FROM_3.to_vec()],
            }));
        } else if continuing.system_state.pc.try_borrow().unwrap().as_concrete_bitvector().value == 0x11C0 {
            continuing.step(Some(SymbolicHints { hints: vec![vec![0]] }));
        } else if continuing.system_state.pc.try_borrow().unwrap().as_concrete_bitvector().value == 0x1264 {
            continuing.step(None);
            println!("a3={:?}", &continuing.system_state.x13);
            println!("a4={:?}", &continuing.system_state.x14);
            let sub = BVSubExpression::new(
                continuing.system_state.x13.clone(),
                continuing.system_state.x14.clone(),
                &mut continuing.stdlib,
                &mut None,
            );
            let mut candidates = vec![];
            continuing.stdlib.monomorphize(sub.try_borrow().unwrap().get_z3_ast(), &mut candidates);
            println!("diff={:?}", candidates)
        } else {
            continuing.step(None);
        }
    }
    */
}
