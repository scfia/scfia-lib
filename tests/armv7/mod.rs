use log::{debug, info, trace, warn, LevelFilter};
use scfia_lib::{
    memory::{regions::{StableMemoryRegion, VolatileMemoryRegion}, Memory},
    models::armv7::stm32::{self, STM32ScfiaComposition, STM32},
    scfia::Scfia,
    values::active_value::ActiveValueImpl,
};
use std::{fs, rc::Rc, thread, time::Instant};
use xmas_elf::program::ProgramHeader::Ph32;
use xmas_elf::{program, ElfFile};

pub struct StepContext<'a> {
    hints: &'a [(u64, &'a [u64])],
}

fn step_until(state: &mut STM32, address: u64, begin: &Instant) {
    while state.state.PC.to_u64() != address {
        assert!(state.state.PC.to_u64() != 0x508);
        //_dump_regs(state);
        debug!(
            "({}ms) Executing {:#x} ({} asts)",
            begin.elapsed().as_millis(),
            state.state.PC.to_u64(),
            state.scfia.z3.ast_refs.get()
        );
        state.step(None);
    }
}

#[test]
fn test_stm32_system_state() {
    let builder = thread::Builder::new().stack_size(4 * 1024 * 1024 * 1024);
    let handler = builder.spawn(test_system_state_inner).unwrap();
    handler.join().unwrap();
}

fn _dump_regs(state: &STM32) {
    println!("R0  = {:x?}", state.state.R0);
    println!("R1  = {:x?}", state.state.R1);
    println!("R2  = {:x?}", state.state.R2);
    println!("R3  = {:x?}", state.state.R3);
    println!("R4  = {:x?}", state.state.R4);
    println!("R5  = {:x?}", state.state.R5);
    println!("R6  = {:x?}", state.state.R6);
    println!("R7  = {:x?}", state.state.R7);
    println!("R8  = {:x?}", state.state.R8);
    println!("R9  = {:x?}", state.state.R9);
    println!("R10 = {:x?}", state.state.R10);
    println!("R11 = {:x?}", state.state.R11);
    println!("R12 = {:x?}", state.state.R12);
    println!("SP  = {:x?}", state.state.SP);
    println!("LR  = {:x?}", state.state.LR);
    println!("PC  = {:x?}", state.state.PC);
}

fn test_system_state_inner() {
    simple_logger::SimpleLogger::new().with_level(LevelFilter::Debug).env().init().unwrap();
    let binary_blob = fs::read("./tests/armv7/data/p2im_drone.bin").unwrap();

    let scfia: Rc<Scfia<STM32ScfiaComposition>> = Scfia::new(None);
    let mut memory = Memory::default();

    let code = StableMemoryRegion::new(0, 0x2000_0000);
    memory.stables.push(code);

    for (i, b) in binary_blob.iter().enumerate() {
        memory.write(
            &scfia.new_bv_concrete(0x8000000 + i as u64, 8),
            &scfia.new_bv_concrete(*b as u64, 8),
            8,
            &scfia,
            &mut None,
            &mut None,
        );
    }

    let sram = StableMemoryRegion::new(0x2000_0000, 0x2000_0000);
    memory.stables.push(sram);

    memory.volatiles.push(VolatileMemoryRegion {
        start_address: 0x40000000,
        length: 0x20000000,
    });

    let mut system_state: STM32 = STM32 {
        state: stm32::SystemState {
            R0: scfia.new_bv_concrete(0b0, 32),
            R1: scfia.new_bv_concrete(0b0, 32),
            R2: scfia.new_bv_concrete(0b0, 32),
            R3: scfia.new_bv_concrete(0b0, 32),
            R4: scfia.new_bv_concrete(0b0, 32),
            R5: scfia.new_bv_concrete(0b0, 32),
            R6: scfia.new_bv_concrete(0b0, 32),
            R7: scfia.new_bv_concrete(0b0, 32),
            R8: scfia.new_bv_concrete(0b0, 32),
            R9: scfia.new_bv_concrete(0b0, 32),
            R10: scfia.new_bv_concrete(0b0, 32),
            R11: scfia.new_bv_concrete(0b0, 32),
            R12: scfia.new_bv_concrete(0b0, 32),
            SP: scfia.new_bv_concrete(0b0, 32),
            LR: scfia.new_bv_concrete(0b0, 32),
            PC: scfia.new_bv_concrete(0x8000000 + 0x52b4, 32),
            apsr: stm32::ApplicationProgramStatusRegister {
                N: scfia.new_bv_concrete(0b0, 1),
                Z: scfia.new_bv_concrete(0b0, 1),
                C: scfia.new_bv_concrete(0b0, 1),
                V: scfia.new_bv_concrete(0b0, 1),
                Q: scfia.new_bv_concrete(0b0, 1),
                GE: scfia.new_bv_concrete(0b0, 4),
            },
        },
        memory,
        scfia,
    };

    let begin = Instant::now();
    info!("Yolo");
    step_until(&mut system_state, 69420, &begin);
}
