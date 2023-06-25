mod constants;

use std::ffi::CStr;
use std::rc::Rc;
use std::time::Instant;
use std::{fs, thread};

use log::{debug, info, trace, warn, LevelFilter};
use scfia_lib::memory::regions::{StableMemoryRegion, SymbolicVolatileMemoryRegion, VolatileMemoryRegion};
use scfia_lib::memory::Memory;
use scfia_lib::models::riscv::rv32i::{self, RV32i, RV32iScfiaComposition};
use scfia_lib::scfia::Scfia;
use scfia_lib::values::active_value::ActiveValueImpl;
use scfia_lib::SymbolicHints;
use xmas_elf::program::ProgramHeader::Ph32;
use xmas_elf::{program, ElfFile};
use z3_sys::Z3_ast_to_string;

use crate::rv32im::constants::{
    COPY_FROM_3, EGRESS_RECEIVEQUEUE_DESCRIPTOR_ADDRESS_HIGHER_U32, EGRESS_RECEIVEQUEUE_DESCRIPTOR_ADDRESS_LOWER_U32, EGRESS_RECEIVEQUEUE_DRIVER_POSITIONS,
    EGRESS_SENDQUEUE_DRIVER_POSITIONS, INGRESS_RECEIVEQUEUE_DESCRIPTOR_ADDRESS_HIGHER_U32, INGRESS_RECEIVEQUEUE_DESCRIPTOR_ADDRESS_LOWER_U32,
    INGRESS_RECEIVEQUEUE_DESCRIPTOR_LENGTH, INGRESS_RECEIVEQUEUE_DRIVER_POSITIONS, INGRESS_SENDQUEUE_DRIVER_POSITIONS, START_OF_MAIN_LOOP,
};

pub struct StepContext<'a> {
    hints: &'a [(u64, &'a [u64])],
}

fn step_until(rv32i_system_state: &mut RV32i, address: u64, begin: &Instant) {
    while rv32i_system_state.state.pc.to_u64() != address {
        assert!(rv32i_system_state.state.pc.to_u64() != 0x508);
        debug!(
            "({}ms) Executing {:#x} ({} asts)",
            begin.elapsed().as_millis(),
            rv32i_system_state.state.pc.to_u64(),
            rv32i_system_state.scfia.z3.ast_refs.get()
        );
        rv32i_system_state.step(None);
    }
}

fn step_until_hinted(rv32i_system_state: &mut RV32i, address: u64, begin: &Instant, context: &StepContext) {
    let mut pc = rv32i_system_state.state.pc.to_u64();
    while pc != address {
        debug!(
            "({}ms) Executing {:#x} ({} asts)",
            begin.elapsed().as_millis(),
            pc,
            rv32i_system_state.scfia.z3.ast_refs.get()
        );
        if pc == 0x72c {
            unsafe {
                let ptr = Z3_ast_to_string(
                    rv32i_system_state.scfia.z3.context,
                    rv32i_system_state.state.x10.try_borrow().unwrap().z3_ast.ast,
                );
                let str = CStr::from_ptr(ptr);
                info!("Z3_ast_to_string={}", str.to_str().unwrap());
                rv32i_system_state.debug();
            }
        }

        let mut found_hint = None;
        for (hint_address, hint) in context.hints {
            if *hint_address == pc {
                found_hint = Some(hint);
                break;
            }
        }
        if let Some(hints) = found_hint {
            rv32i_system_state.step(Some(SymbolicHints { hints: vec![hints.to_vec()] }));
        } else {
            rv32i_system_state.step(None);
        }
        pc = rv32i_system_state.state.pc.to_u64();
    }
}

#[test]
fn test_rv32i_system_state() {
    let builder = thread::Builder::new().stack_size(4 * 1024 * 1024 * 1024);
    let handler = builder.spawn(test_system_state_inner).unwrap();
    handler.join().unwrap();
}

fn _dump_regs(state: &RV32i) {
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
    println!("x13 = {:#x?}", state.state.x13);
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

    let scfia: Rc<Scfia<RV32iScfiaComposition>> = Scfia::new(None);
    let mut memory = Memory::default();

    for program_header in elf.program_iter() {
        if let Ph32(ph32) = program_header {
            match program_header.get_type().unwrap() {
                program::Type::Load => {
                    trace!("{:?}", program_header);
                    let stable_region = StableMemoryRegion::new(ph32.virtual_addr as u64, ph32.mem_size as u64);
                    memory.stables.push(stable_region);

                    for (i, b) in ph32.raw_data(&elf).iter().enumerate() {
                        memory.write(
                            &scfia.new_bv_concrete(ph32.virtual_addr as u64 + i as u64, 8, None, &mut None, None),
                            &scfia.new_bv_concrete(*b as u64, 8, None, &mut None, None),
                            8,
                            &scfia,
                            &mut None,
                            &mut None,
                        );
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
            x0: scfia.new_bv_concrete(0b0, 32, None, &mut None, None),
            x1: scfia.new_bv_concrete(0b0, 32, None, &mut None, None),
            x2: scfia.new_bv_concrete(0b0, 32, None, &mut None, None),
            x3: scfia.new_bv_concrete(0b0, 32, None, &mut None, None),
            x4: scfia.new_bv_concrete(0b0, 32, None, &mut None, None),
            x5: scfia.new_bv_concrete(0b0, 32, None, &mut None, None),
            x6: scfia.new_bv_concrete(0b0, 32, None, &mut None, None),
            x7: scfia.new_bv_concrete(0b0, 32, None, &mut None, None),
            x8: scfia.new_bv_concrete(0b0, 32, None, &mut None, None),
            x9: scfia.new_bv_concrete(0b0, 32, None, &mut None, None),
            x10: scfia.new_bv_concrete(0b0, 32, None, &mut None, None),
            x11: scfia.new_bv_concrete(0b0, 32, None, &mut None, None),
            x12: scfia.new_bv_concrete(0b0, 32, None, &mut None, None),
            x13: scfia.new_bv_concrete(0b0, 32, None, &mut None, None),
            x14: scfia.new_bv_concrete(0b0, 32, None, &mut None, None),
            x15: scfia.new_bv_concrete(0b0, 32, None, &mut None, None),
            x16: scfia.new_bv_concrete(0b0, 32, None, &mut None, None),
            x17: scfia.new_bv_concrete(0b0, 32, None, &mut None, None),
            x18: scfia.new_bv_concrete(0b0, 32, None, &mut None, None),
            x19: scfia.new_bv_concrete(0b0, 32, None, &mut None, None),
            x20: scfia.new_bv_concrete(0b0, 32, None, &mut None, None),
            x21: scfia.new_bv_concrete(0b0, 32, None, &mut None, None),
            x22: scfia.new_bv_concrete(0b0, 32, None, &mut None, None),
            x23: scfia.new_bv_concrete(0b0, 32, None, &mut None, None),
            x24: scfia.new_bv_concrete(0b0, 32, None, &mut None, None),
            x25: scfia.new_bv_concrete(0b0, 32, None, &mut None, None),
            x26: scfia.new_bv_concrete(0b0, 32, None, &mut None, None),
            x27: scfia.new_bv_concrete(0b0, 32, None, &mut None, None),
            x28: scfia.new_bv_concrete(0b0, 32, None, &mut None, None),
            x29: scfia.new_bv_concrete(0b0, 32, None, &mut None, None),
            x30: scfia.new_bv_concrete(0b0, 32, None, &mut None, None),
            x31: scfia.new_bv_concrete(0b0, 32, None, &mut None, None),
            pc: scfia.new_bv_concrete(0x4f4, 32, None, &mut None, None),
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
    step_until_hinted(
        &mut continuing,
        0x04,
        &begin,
        &StepContext {
            hints: &[(0x3dc, &INGRESS_RECEIVEQUEUE_DRIVER_POSITIONS)],
        },
    );

    info!("({}ms) Cloning state to free some z3 memory", begin.elapsed().as_millis());
    continuing = continuing.clone_model().0;

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
    step_until_hinted(
        &mut continuing,
        0x24,
        &begin,
        &StepContext {
            hints: &[(0x3dc, &INGRESS_SENDQUEUE_DRIVER_POSITIONS)],
        },
    );

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
    step_until_hinted(
        &mut continuing,
        0x04,
        &begin,
        &StepContext {
            hints: &[(0x3dc, &EGRESS_RECEIVEQUEUE_DRIVER_POSITIONS)],
        },
    );

    info!("({}ms) Cloning state to free some z3 memory", begin.elapsed().as_millis());
    continuing = continuing.clone_model().0;

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
    step_until_hinted(
        &mut continuing,
        START_OF_MAIN_LOOP,
        &begin,
        &StepContext {
            hints: &[(0x3dc, &EGRESS_SENDQUEUE_DRIVER_POSITIONS)],
        },
    );

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
    continuing.scfia.monomorphize_active(&continuing.state.x14, &mut monomorphizing_candidates);
    assert_eq!(monomorphizing_candidates.len(), 1);
    continuing.state.x14 = continuing.scfia.new_bv_concrete(monomorphizing_candidates[0], 32, None, &mut None, None);

    info!("({}ms) Creating symbolic volatile memory regions", begin.elapsed().as_millis());
    let base_symbol = continuing.scfia.new_bv_constrained(32, 0xff, 0xffff0000);
    let sym_region = SymbolicVolatileMemoryRegion { base_symbol, length: 4096 };

    info!("({}ms) Writing symbolic pointers to descriptor table", begin.elapsed().as_millis());
    // TODO ensure is valid generalization
    for i in 0..1024 {
        let pointer_offset: u32 = i * 16;
        let ingress_receive_queue_pointer_address = 0x46000000 + pointer_offset;
        debug!("({}ms) overwriting 0x{:x}", begin.elapsed().as_millis(), ingress_receive_queue_pointer_address);
        continuing.memory.write_concrete(
            ingress_receive_queue_pointer_address as u64,
            &sym_region.base_symbol,
            32,
            &continuing.scfia,
            &mut None,
        );

        let egress_send_queue_pointer_address = 0x46c18000 + pointer_offset;
        debug!("({}ms) overwriting 0x{:x}", begin.elapsed().as_millis(), egress_send_queue_pointer_address);
        continuing.memory.write_concrete(
            egress_send_queue_pointer_address as u64,
            &sym_region.base_symbol,
            32,
            &continuing.scfia,
            &mut None,
        );
    }
    continuing.memory.symbolic_volatiles.push(sym_region);

    info!("({}ms) Stepping until ethertype ipv4 check", begin.elapsed().as_millis());
    step_until_hinted(
        &mut continuing,
        0x73c,
        &begin,
        &StepContext {
            hints: &[
                (0x49C, &INGRESS_RECEIVEQUEUE_DESCRIPTOR_ADDRESS_HIGHER_U32),
                (0x4a0, &INGRESS_RECEIVEQUEUE_DESCRIPTOR_ADDRESS_LOWER_U32),
                (0x4a4, &INGRESS_RECEIVEQUEUE_DESCRIPTOR_LENGTH),
                (0x4b0, &INGRESS_RECEIVEQUEUE_DESCRIPTOR_ADDRESS_HIGHER_U32),
                (0x4b4, &INGRESS_RECEIVEQUEUE_DESCRIPTOR_ADDRESS_LOWER_U32),
            ],
        },
    );

    let mut successors = continuing.step_forking(None);
    let mut returning = successors.remove(0);
    let mut continuing = successors.remove(0);

    info!("({}ms) Stepping not ipv4 until start of main loop", begin.elapsed().as_millis());
    step_until_hinted(
        &mut returning,
        START_OF_MAIN_LOOP,
        &begin,
        &StepContext {
            hints: &[(0x3dc, &INGRESS_RECEIVEQUEUE_DRIVER_POSITIONS)],
        },
    );

    info!("({}ms) Stepping until egress try_remove fork", begin.elapsed().as_millis());
    step_until(&mut continuing, 0x428, &begin);
    let mut successors = continuing.step_forking(None);
    let mut continuing = successors.remove(0);
    let mut returning = successors.remove(0);

    info!("({}ms) Stepping egress empty until start of main loop", begin.elapsed().as_millis());
    step_until_hinted(
        &mut returning,
        START_OF_MAIN_LOOP,
        &begin,
        &StepContext {
            hints: &[(0x3dc, &INGRESS_RECEIVEQUEUE_DRIVER_POSITIONS)],
        },
    );

    info!("({}ms) stepping success until start of main loop", begin.elapsed().as_millis());
    step_until_hinted(
        &mut continuing,
        START_OF_MAIN_LOOP,
        &begin,
        &StepContext {
            hints: &[
                (0x150, &EGRESS_RECEIVEQUEUE_DESCRIPTOR_ADDRESS_HIGHER_U32),
                (0x154, &EGRESS_RECEIVEQUEUE_DESCRIPTOR_ADDRESS_LOWER_U32),
                (0x158, &COPY_FROM_3), // Read buffer len from descriptor table
                (0x160, &INGRESS_RECEIVEQUEUE_DESCRIPTOR_ADDRESS_HIGHER_U32),
                (0x164, &INGRESS_RECEIVEQUEUE_DESCRIPTOR_ADDRESS_LOWER_U32),
                (0x168, &COPY_FROM_3), // Read buffer len from descriptor table
                (0x460, &[0x46c1d004]),
            ],
        },
    );
    info!("SUCCESS");
}
