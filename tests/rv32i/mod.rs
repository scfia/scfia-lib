use std::fs;
use std::time::Instant;

use scfia_lib::memory::stable_memory_region32::StableMemoryRegion32;
use scfia_lib::memory::MemoryRegion32;
use scfia_lib::memory::volatile_memory_region::VolatileMemoryRegion32;
use scfia_lib::models::riscv::rv32i::RV32iSystemState;
use scfia_lib::values::ActiveValue;
use scfia_lib::{
    memory::memory32::Memory32, models::riscv::rv32i,
    values::bit_vector_concrete::BitVectorConcrete, ScfiaStdlib,
};
use xmas_elf::program::ProgramHeader::Ph32;
use xmas_elf::{program, ElfFile};


#[test]
fn test_system_state() {
    let binary_blob = fs::read("./tests/rv32i/data/simple_router_risc_v").unwrap();
    let elf = ElfFile::new(&binary_blob).unwrap();

    let mut stdlib = ScfiaStdlib::new();
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
                            ActiveValue::BitvectorConcrete(BitVectorConcrete::new(
                                *b as u64,
                                8,
                                &mut stdlib,
                            ))
                            .into(),
                            &mut stdlib,
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
            x0: BitVectorConcrete::new(0b0, 32, &mut stdlib).into(),
            x1: BitVectorConcrete::new(0b0, 32, &mut stdlib).into(),
            x2: BitVectorConcrete::new(0b0, 32, &mut stdlib).into(),
            x3: BitVectorConcrete::new(0b0, 32, &mut stdlib).into(),
            x4: BitVectorConcrete::new(0b0, 32, &mut stdlib).into(),
            x5: BitVectorConcrete::new(0b0, 32, &mut stdlib).into(),
            x6: BitVectorConcrete::new(0b0, 32, &mut stdlib).into(),
            x7: BitVectorConcrete::new(0b0, 32, &mut stdlib).into(),
            x8: BitVectorConcrete::new(0b0, 32, &mut stdlib).into(),
            x9: BitVectorConcrete::new(0b0, 32, &mut stdlib).into(),
            x10: BitVectorConcrete::new(0b0, 32, &mut stdlib).into(),
            x11: BitVectorConcrete::new(0b0, 32, &mut stdlib).into(),
            x12: BitVectorConcrete::new(0b0, 32, &mut stdlib).into(),
            x13: BitVectorConcrete::new(0b0, 32, &mut stdlib).into(),
            x14: BitVectorConcrete::new(0b0, 32, &mut stdlib).into(),
            x15: BitVectorConcrete::new(0b0, 32, &mut stdlib).into(),
            x16: BitVectorConcrete::new(0b0, 32, &mut stdlib).into(),
            x17: BitVectorConcrete::new(0b0, 32, &mut stdlib).into(),
            x18: BitVectorConcrete::new(0b0, 32, &mut stdlib).into(),
            x19: BitVectorConcrete::new(0b0, 32, &mut stdlib).into(),
            x20: BitVectorConcrete::new(0b0, 32, &mut stdlib).into(),
            x21: BitVectorConcrete::new(0b0, 32, &mut stdlib).into(),
            x22: BitVectorConcrete::new(0b0, 32, &mut stdlib).into(),
            x23: BitVectorConcrete::new(0b0, 32, &mut stdlib).into(),
            x24: BitVectorConcrete::new(0b0, 32, &mut stdlib).into(),
            x25: BitVectorConcrete::new(0b0, 32, &mut stdlib).into(),
            x26: BitVectorConcrete::new(0b0, 32, &mut stdlib).into(),
            x27: BitVectorConcrete::new(0b0, 32, &mut stdlib).into(),
            x28: BitVectorConcrete::new(0b0, 32, &mut stdlib).into(),
            x29: BitVectorConcrete::new(0b0, 32, &mut stdlib).into(),
            x30: BitVectorConcrete::new(0b0, 32, &mut stdlib).into(),
            x31: BitVectorConcrete::new(0b0, 32, &mut stdlib).into(),
            pc: BitVectorConcrete::new(0x00000004, 32, &mut stdlib).into(),
        },
        memory,
        stdlib,
    };

    println!("Stepping until NIC1 queue_pfn check");
    while rv32i_system_state.system_state.pc.try_borrow().unwrap().as_concrete_bitvector().value != 0x770 {
        rv32i_system_state.step();
    }
    
    let mut successors = rv32i_system_state.step_forking();
    let mut panicking = successors.remove(0);
    let mut continuing = successors.remove(0);

    println!("Stepping panic until loop");
    while panicking.system_state.pc.try_borrow().unwrap().as_concrete_bitvector().value != 0x18 {
        panicking.step()
    }

    println!("Stepping until NIC1 queue_num_max 0 check");
    while continuing.system_state.pc.try_borrow().unwrap().as_concrete_bitvector().value != 0x77c {
        continuing.step()
    }
    
    let mut successors = continuing.step_forking();
    let mut panicking = successors.remove(0);
    let mut continuing = successors.remove(0);
    
    println!("Stepping panic until loop");
    while panicking.system_state.pc.try_borrow().unwrap().as_concrete_bitvector().value != 0x18 {
        panicking.step()
    }

    println!("Stepping until NIC1 queue_num_max <1024 check");
    while continuing.system_state.pc.try_borrow().unwrap().as_concrete_bitvector().value != 0x784 {
        continuing.step()
    }

    let mut successors = continuing.step_forking();
    let mut panicking = successors.remove(0);
    let mut continuing = successors.remove(0);

    println!("Stepping panic until loop");
    while panicking.system_state.pc.try_borrow().unwrap().as_concrete_bitvector().value != 0x18 {
        panicking.step()
    }

    println!("Stepping until ???");
    while continuing.system_state.pc.try_borrow().unwrap().as_concrete_bitvector().value != 0x1460 {
        continuing.step();
    }

    let mut successors = continuing.step_forking();
    panic!("{}", successors.len());

    /*
    println!("Stepping until X...");
    let mut steps_total = 0;
    let mut steps = 0;
    let mut then = Instant::now();
    loop {
        continuing.step();
        steps_total += 1;
        steps += 1;
        let now = Instant::now();
        if (now - then).as_secs_f64() >= 1.0 {
            println!("{} steps/second", steps);
            then = now;
            steps = 0;
        }
    }
    */

}
