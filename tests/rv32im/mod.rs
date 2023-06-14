mod constants;

use std::collections::HashMap;
use std::hash::Hash;
use std::{fs, thread};
use std::time::Instant;

use log::{debug, info, trace, warn, LevelFilter};
use scfia_lib::memory::regions::{StableMemoryRegion, SymbolicVolatileMemoryRegion, VolatileMemoryRegion};
use scfia_lib::memory::Memory;
use scfia_lib::models::riscv::rv32i;
use scfia_lib::models::riscv::rv32i::RV32i;
use scfia_lib::scfia::Scfia;
use scfia_lib::values::active_value::ActiveValueImpl;
use scfia_lib::SymbolicHints;
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

pub struct StepContext<'a> {
    hints: &'a [(u64, &'a [u64])],
}

impl<'a> StepContext<'a> {
    fn new(hints: &'a [(u64, &'a [u64])]) -> Self {
        StepContext {
            hints,
        }
    }
}

fn step_until(rv32i_system_state: &mut RV32i, address: u64, begin: &Instant) {
    while rv32i_system_state.state.pc.to_u64() != address {
        assert!(rv32i_system_state.state.pc.to_u64() != 0x508);
        debug!("({}ms) Executing {:#x}", begin.elapsed().as_millis(), rv32i_system_state.state.pc.to_u64());
        rv32i_system_state.step(None);
    }
}

fn step_until_hinted(rv32i_system_state: &mut RV32i, address: u64, begin: &Instant, context: &StepContext) {
    let mut pc = rv32i_system_state.state.pc.to_u64();
    while pc != address {
        debug!("({}ms) Executing {:#x}", begin.elapsed().as_millis(), pc);
        let mut found_hint = None;
        for (hint_address, hint) in context.hints {
            if address == address {
                found_hint = Some(hint);
                break;
            }
        }
        if let Some(hints) = found_hint {
            rv32i_system_state.step(Some(SymbolicHints {
                hints: vec![hints.to_vec()],
            }));
        } else {
            rv32i_system_state.step(None);
        }
        pc = rv32i_system_state.state.pc.to_u64();
    }
}

#[test]
fn test_system_state() {
    let builder = thread::Builder::new().stack_size(4 * 1024 * 1024 * 1024);
    let handler = builder
        .spawn(test_system_state_inner)
        .unwrap();
    handler.join().unwrap();
}

fn dump_regs(state: &RV32i) {
    println!("x1  = {:x?}", state.state.x1);
    println!("x2  = {:x?}", state.state.x2);
    println!("x3  = {:x?}", state.state.x3);
    println!("x4  = {:x?}", state.state.x4);
    println!("x5  = {:x?}", state.state.x5);
    println!("x6  = {:x?}", state.state.x6);
    println!("x7  = {:x?}", state.state.x7);
    println!("x8  = {:x?}", state.state.x8);
    println!("x9  = {:x?}", state.state.x9);
    println!("x10 = {:x?}", state.state.x10);
    println!("x11 = {:x?}", state.state.x11);
    println!("x12 = {:x?}", state.state.x12);
    println!("x13 = {:x?}", state.state.x13);
    println!("x14 = {:x?}", state.state.x14);
    println!("x15 = {:x?}", state.state.x15);
    println!("x16 = {:x?}", state.state.x16);
    println!("x17 = {:x?}", state.state.x17);
    println!("x18 = {:x?}", state.state.x18);
    println!("x19 = {:x?}", state.state.x19);
    println!("x20 = {:x?}", state.state.x20);
    println!("x21 = {:x?}", state.state.x21);
    println!("x22 = {:x?}", state.state.x22);
    println!("x23 = {:x?}", state.state.x23);
    println!("x24 = {:x?}", state.state.x24);
    println!("x25 = {:x?}", state.state.x25);
    println!("x26 = {:x?}", state.state.x26);
    println!("x27 = {:x?}", state.state.x27);
    println!("x28 = {:x?}", state.state.x28);
    println!("x29 = {:x?}", state.state.x29);
    println!("x30 = {:x?}", state.state.x30);
    println!("x31 = {:x?}", state.state.x31);
}

fn test_system_state_inner() {
    simple_logger::SimpleLogger::new().with_level(LevelFilter::Debug).env().init().unwrap();
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
                            scfia.new_bv_concrete(ph32.virtual_addr as u64 + i, 8, &mut None),
                            scfia.new_bv_concrete(*b as u64, 8, &mut None),
                            8,
                            scfia.clone(),
                            &mut None,
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
            x0: scfia.new_bv_concrete(0b0, 32, &mut None),
            x1: scfia.new_bv_concrete(0b0, 32, &mut None),
            x2: scfia.new_bv_concrete(0b0, 32, &mut None),
            x3: scfia.new_bv_concrete(0b0, 32, &mut None),
            x4: scfia.new_bv_concrete(0b0, 32, &mut None),
            x5: scfia.new_bv_concrete(0b0, 32, &mut None),
            x6: scfia.new_bv_concrete(0b0, 32, &mut None),
            x7: scfia.new_bv_concrete(0b0, 32, &mut None),
            x8: scfia.new_bv_concrete(0b0, 32, &mut None),
            x9: scfia.new_bv_concrete(0b0, 32, &mut None),
            x10: scfia.new_bv_concrete(0b0, 32, &mut None),
            x11: scfia.new_bv_concrete(0b0, 32, &mut None),
            x12: scfia.new_bv_concrete(0b0, 32, &mut None),
            x13: scfia.new_bv_concrete(0b0, 32, &mut None),
            x14: scfia.new_bv_concrete(0b0, 32, &mut None),
            x15: scfia.new_bv_concrete(0b0, 32, &mut None),
            x16: scfia.new_bv_concrete(0b0, 32, &mut None),
            x17: scfia.new_bv_concrete(0b0, 32, &mut None),
            x18: scfia.new_bv_concrete(0b0, 32, &mut None),
            x19: scfia.new_bv_concrete(0b0, 32, &mut None),
            x20: scfia.new_bv_concrete(0b0, 32, &mut None),
            x21: scfia.new_bv_concrete(0b0, 32, &mut None),
            x22: scfia.new_bv_concrete(0b0, 32, &mut None),
            x23: scfia.new_bv_concrete(0b0, 32, &mut None),
            x24: scfia.new_bv_concrete(0b0, 32, &mut None),
            x25: scfia.new_bv_concrete(0b0, 32, &mut None),
            x26: scfia.new_bv_concrete(0b0, 32, &mut None),
            x27: scfia.new_bv_concrete(0b0, 32, &mut None),
            x28: scfia.new_bv_concrete(0b0, 32, &mut None),
            x29: scfia.new_bv_concrete(0b0, 32, &mut None),
            x30: scfia.new_bv_concrete(0b0, 32, &mut None),
            x31: scfia.new_bv_concrete(0b0, 32, &mut None),
            pc: scfia.new_bv_concrete(0x4f4, 32, &mut None),
        },
        memory,
        scfia,
    };

    let begin = Instant::now();
    info!("Stepping until NIC1 receivequeue queue_pfn check");
    step_until(&mut rv32i_system_state, 0x24, &begin);

    let mut successors = rv32i_system_state.step_forking(None);
    let mut panicking = successors.remove(0);
    let mut continuing = successors.remove(0);

    info!("({}ms) Stepping panic until loop", begin.elapsed().as_millis());
    step_until(&mut panicking, 0x508, &begin);

    info!("({}ms) Stepping until NIC1 receivequeue queue_num_max 0 check", begin.elapsed().as_millis());
    step_until(&mut continuing, 0x30, &begin);

    let mut successors = continuing.step_forking(None);
    let mut panicking = successors.remove(0);
    let mut continuing = successors.remove(0);

    info!("({}ms) Stepping panic until loop", begin.elapsed().as_millis());
    step_until(&mut panicking, 0x508, &begin);

    info!("({}ms) Stepping until NIC1 receivequeue queue_num_max <1024 check", begin.elapsed().as_millis());
    step_until(&mut continuing, 0x38, &begin);

    let mut successors = continuing.step_forking(None);
    let mut panicking = successors.remove(0);
    let mut continuing = successors.remove(0);

    info!("({}ms) Stepping panic until loop", begin.elapsed().as_millis());
    step_until(&mut panicking, 0x508, &begin);

    info!("({}ms) Stepping until NIC1 sendqueue configure_virtqueue", begin.elapsed().as_millis());
    step_until_hinted(&mut continuing, 0x04, &begin, &StepContext { hints: &[(0x3dc, &INGRESS_RECEIVEQUEUE_DRIVER_POSITIONS)] });

    info!("({}ms) Cloning state to free some z3 memory", begin.elapsed().as_millis());
    continuing = continuing.clone_model();

    info!("({}ms) Stepping until NIC1 sendqueue queue_pfn check", begin.elapsed().as_millis());
    step_until(&mut continuing, 0x24, &begin);

    let mut successors = continuing.step_forking(None);
    let mut panicking = successors.remove(0);
    let mut continuing = successors.remove(0);

    info!("({}ms) Stepping panic until loop", begin.elapsed().as_millis());
    step_until(&mut panicking, 0x508, &begin);

    info!("({}ms) Stepping until NIC1 sendqueue queue_num_max 0 check", begin.elapsed().as_millis());
    step_until(&mut continuing, 0x30, &begin);

    let mut successors = continuing.step_forking(None);
    let mut panicking = successors.remove(0);
    let mut continuing = successors.remove(0);

    info!("({}ms) Stepping panic until loop", begin.elapsed().as_millis());
    step_until(&mut panicking, 0x508, &begin);

    info!("({}ms) Stepping until NIC1 sendqueue queue_num_max <1024 check", begin.elapsed().as_millis());
    step_until(&mut continuing, 0x38, &begin);

    let mut successors = continuing.step_forking(None);
    let mut panicking = successors.remove(0);
    let mut continuing = successors.remove(0);

    info!("({}ms) Stepping panic until loop", begin.elapsed().as_millis());
    step_until(&mut panicking, 0x508, &begin);

    info!("({}ms) Stepping until NIC2 receivequeue queue_pfn check", begin.elapsed().as_millis());
    step_until_hinted(&mut continuing, 0x24, &begin, &StepContext { hints: &[(0x3dc, &INGRESS_SENDQUEUE_DRIVER_POSITIONS)] });

    let mut successors = continuing.step_forking(None);
    let mut panicking = successors.remove(0);
    let mut continuing = successors.remove(0);

    info!("({}ms) Stepping panic until loop", begin.elapsed().as_millis());
    step_until(&mut panicking, 0x508, &begin);

    info!("({}ms) Stepping until NIC2 receivequeue queue_num_max 0 check", begin.elapsed().as_millis());
    step_until(&mut continuing, 0x30, &begin);

    let mut successors = continuing.step_forking(None);
    let mut panicking = successors.remove(0);
    let mut continuing = successors.remove(0);

    info!("({}ms) Stepping panic until loop", begin.elapsed().as_millis());
    step_until(&mut panicking, 0x508, &begin);

    info!("({}ms) Stepping until NIC2 receivequeue queue_num_max <1024 check", begin.elapsed().as_millis());
    step_until(&mut continuing, 0x38, &begin);

    let mut successors = continuing.step_forking(None);
    let mut panicking = successors.remove(0);
    let mut continuing = successors.remove(0);

    info!("({}ms) Stepping panic until loop", begin.elapsed().as_millis());
    step_until(&mut panicking, 0x508, &begin);

    info!("({}ms) Stepping until NIC2 sendqueue configure_virtqueue", begin.elapsed().as_millis());
    step_until_hinted(&mut continuing, 0x04, &begin, &StepContext { hints: &[(0x3dc, &EGRESS_RECEIVEQUEUE_DRIVER_POSITIONS)] });

    info!("({}ms) Cloning state to free some z3 memory", begin.elapsed().as_millis());
    continuing = continuing.clone_model();

    info!("({}ms) Stepping until NIC2 sendqueue queue_pfn check", begin.elapsed().as_millis());
    step_until(&mut continuing, 0x24, &begin);

    let mut successors = continuing.step_forking(None);
    let mut panicking = successors.remove(0);
    let mut continuing = successors.remove(0);

    info!("({}ms) Stepping panic until loop", begin.elapsed().as_millis());
    step_until(&mut panicking, 0x508, &begin);

    info!("({}ms) Stepping until NIC2 sendqueue queue_num_max 0 check", begin.elapsed().as_millis());
    step_until(&mut continuing, 0x30, &begin);

    let mut successors = continuing.step_forking(None);
    let mut panicking = successors.remove(0);
    let mut continuing = successors.remove(0);

    info!("({}ms) Stepping panic until loop", begin.elapsed().as_millis());
    step_until(&mut panicking, 0x508, &begin);

    info!("({}ms) Stepping until NIC2 sendqueue queue_num_max <1024 check", begin.elapsed().as_millis());
    step_until(&mut continuing, 0x38, &begin);

    let mut successors = continuing.step_forking(None);
    let mut panicking = successors.remove(0);
    let mut continuing = successors.remove(0);

    info!("({}ms) Stepping panic until loop", begin.elapsed().as_millis());
    step_until(&mut panicking, 0x508, &begin);

    info!("({}ms) Stepping until start of main loop", begin.elapsed().as_millis());
    step_until_hinted(&mut continuing, START_OF_MAIN_LOOP, &begin, &StepContext { hints: &[(0x3dc, &EGRESS_SENDQUEUE_DRIVER_POSITIONS)] });

    info!("({}ms) Stepping until ingress try_remove fork", begin.elapsed().as_millis());
    step_until(&mut continuing, 0x428, &begin);

    let mut successors = continuing.step_forking(None);
    let mut continuing = successors.remove(0);
    let mut returning = successors.remove(0);

    info!("({}ms) Stepping aborting until start of main loop", begin.elapsed().as_millis());
    step_until(&mut returning, START_OF_MAIN_LOOP, &begin);

    step_until(&mut continuing, 0x460, &begin);
    info!("({}ms) Monomorphizing a4 to 0x46005004", begin.elapsed().as_millis());
    let mut monomorphizing_candidates = vec![0x46005004];
    continuing.scfia.monomorphize_active(&continuing.state.x14.try_borrow().unwrap(), &mut monomorphizing_candidates);
    assert_eq!(monomorphizing_candidates.len(), 1);
    continuing.state.x14 = continuing.scfia.new_bv_concrete(monomorphizing_candidates[0], 32, &mut None);
    /*
    info!("({}ms) Creating symbolic volatile memory regions", begin.elapsed().as_millis());
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

    info!("({}ms) Writing symbolic pointers to descriptor table", begin.elapsed().as_millis());
    // TODO ensure is valid generalization
    for i in 0..1024 {
        let pointer_offset: u32 = i * 16;
        let ingress_receive_queue_pointer_address = 0x46000000 + pointer_offset;
        info!(
            "({}ms) overwriting 0x{:x}",
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
        info!("({}ms) overwriting 0x{:x}", begin.elapsed().as_millis(), egress_send_queue_pointer_address);
        continuing.memory.write_concrete(
            egress_send_queue_pointer_address,
            sym_region.base_symbol.clone(),
            32,
            &mut continuing.stdlib,
            &mut None,
        );
    }
    continuing.memory.symbolic_volatiles.push(sym_region);

    info!("({}ms) Stepping until ethertype ipv4 check", begin.elapsed().as_millis());
    while continuing.state.pc.to_u64() != 0x73c {
        print!("({}ms) ", begin.elapsed().as_millis());
        if continuing.state.pc.to_u64() == 0x49C {
            continuing.step(Some(SymbolicHints {
                hints: vec![INGRESS_RECEIVEQUEUE_DESCRIPTOR_ADDRESS_HIGHER_U32.to_vec()],
            }));
        } else if continuing.state.pc.to_u64() == 0x4a0 {
            continuing.step(Some(SymbolicHints {
                hints: vec![INGRESS_RECEIVEQUEUE_DESCRIPTOR_ADDRESS_LOWER_U32.to_vec()],
            }));
        } else if continuing.state.pc.to_u64() == 0x4a8 {
            continuing.step(None);
            info!("a3={:?}", continuing.system_state.x13);
            let mut candidates = vec![];
            continuing
                .stdlib
                .monomorphize(continuing.system_state.x13.try_borrow().unwrap().get_z3_ast(), &mut candidates);
            info!("a3 candidates={:?}", candidates)
        } else if continuing.state.pc.to_u64() == 0x4a4 {
            continuing.step(Some(SymbolicHints {
                hints: vec![INGRESS_RECEIVEQUEUE_DESCRIPTOR_LENGTH.to_vec()],
            }));
        } else if continuing.state.pc.to_u64() == 0x4b0 {
            continuing.step(Some(SymbolicHints {
                hints: vec![INGRESS_RECEIVEQUEUE_DESCRIPTOR_ADDRESS_HIGHER_U32.to_vec()],
            }));
        } else if continuing.state.pc.to_u64() == 0x4b4 {
            continuing.step(Some(SymbolicHints {
                hints: vec![INGRESS_RECEIVEQUEUE_DESCRIPTOR_ADDRESS_LOWER_U32.to_vec()],
            }));
        } else {
            continuing.step(None);
        }
    }

    let mut successors = continuing.step_forking(None);
    let mut returning = successors.remove(0);
    let mut continuing = successors.remove(0);

    info!("({}ms) Stepping not ipv4 until start of main loop", begin.elapsed().as_millis());
    while returning.state.pc.to_u64() != START_OF_MAIN_LOOP {
        print!("({}ms) ", begin.elapsed().as_millis());
        if returning.state.pc.to_u64() == 0x3DC {
            returning.step(Some(SymbolicHints {
                hints: vec![INGRESS_RECEIVEQUEUE_DRIVER_POSITIONS.to_vec()],
            }));
        } else {
            returning.step(None);
        }
    }

    info!("({}ms) Stepping until egress try_remove fork", begin.elapsed().as_millis());
    while continuing.state.pc.to_u64() != 0x428 {
        print!("({}ms) ", begin.elapsed().as_millis());
        continuing.step(None);
    }

    let mut successors = continuing.step_forking(None);
    let mut continuing = successors.remove(0);
    let mut returning = successors.remove(0);

    info!("({}ms) Stepping egress empty until start of main loop", begin.elapsed().as_millis());
    while returning.state.pc.to_u64() != START_OF_MAIN_LOOP {
        print!("({}ms) ", begin.elapsed().as_millis());
        if returning.state.pc.to_u64() == 0x3DC {
            returning.step(Some(SymbolicHints {
                hints: vec![INGRESS_RECEIVEQUEUE_DRIVER_POSITIONS.to_vec()],
            }));
        } else {
            returning.step(None);
        }
    }

    info!("({}ms) stepping success until start of main loop", begin.elapsed().as_millis());
    while continuing.state.pc.to_u64() != START_OF_MAIN_LOOP {
        print!("({}ms) ", begin.elapsed().as_millis());
        if continuing.state.pc.to_u64() == 0x3DC {
            continuing.step(Some(SymbolicHints { hints: vec![vec![0x46c1d004]] }));
        } else if continuing.state.pc.to_u64() == 0x460 {
            continuing.step(Some(SymbolicHints { hints: vec![vec![0x46c1d004]] }));
        } else if continuing.state.pc.to_u64() == 0x150 {
            continuing.step(Some(SymbolicHints {
                hints: vec![COPY_FROM_1.to_vec()],
            }));
        } else if continuing.state.pc.to_u64() == 0x154 {
            continuing.step(Some(SymbolicHints {
                hints: vec![COPY_FROM_2.to_vec()],
            }));
        } else if continuing.state.pc.to_u64() == 0x158 {
            continuing.step(Some(SymbolicHints {
                hints: vec![COPY_FROM_3.to_vec()],
            }));
        } else if continuing.state.pc.to_u64() == 0x160 {
            continuing.step(Some(SymbolicHints {
                hints: vec![INGRESS_RECEIVEQUEUE_DESCRIPTOR_ADDRESS_HIGHER_U32.to_vec()],
            }));
        } else if continuing.state.pc.to_u64() == 0x164 {
            continuing.step(Some(SymbolicHints {
                hints: vec![INGRESS_RECEIVEQUEUE_DESCRIPTOR_ADDRESS_LOWER_U32.to_vec()],
            }));
        } else if continuing.state.pc.to_u64() == 0x168 {
            continuing.step(Some(SymbolicHints {
                hints: vec![COPY_FROM_3.to_vec()],
            }));
        } else if continuing.state.pc.to_u64() == 0x11C0 {
            continuing.step(Some(SymbolicHints { hints: vec![vec![0]] }));
        } else if continuing.state.pc.to_u64() == 0x1264 {
            continuing.step(None);
            info!("a3={:?}", &continuing.system_state.x13);
            info!("a4={:?}", &continuing.system_state.x14);
            let sub = BVSubExpression::new(
                continuing.system_state.x13.clone(),
                continuing.system_state.x14.clone(),
                &mut continuing.stdlib,
                &mut None,
            );
            let mut candidates = vec![];
            continuing.stdlib.monomorphize(sub.try_borrow().unwrap().get_z3_ast(), &mut candidates);
            info!("diff={:?}", candidates)
        } else {
            continuing.step(None);
        }
    }
    */
}
