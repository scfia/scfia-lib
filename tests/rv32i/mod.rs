use std::cell::RefCell;
use std::collections::HashMap;
use std::rc::Rc;
use std::{env, fs};

use scfia_lib::expressions::bool_eq_expression::BoolEqExpression;
use scfia_lib::expressions::bool_neq_expression::BoolNEqExpression;
use scfia_lib::expressions::bool_not_expression::BoolNotExpression;
use scfia_lib::memory::stable_memory_region32::StableMemoryRegion32;
use scfia_lib::memory::MemoryRegion32;
use scfia_lib::memory::volatile_memory_region::VolatileMemoryRegion32;
use scfia_lib::models::riscv::rv32i::RV32iSystemState;
use scfia_lib::traits::ast::Ast;
use scfia_lib::values::ActiveValue;
use scfia_lib::values::bit_vector_symbol::BitVectorSymbol;
use scfia_lib::{
    memory::memory32::Memory32, models::riscv::rv32i,
    values::bit_vector_concrete::BitVectorConcrete, ScfiaStdlib,
};
use xmas_elf::program::ProgramHeader::Ph32;
use xmas_elf::{program, ElfFile};
use z3_sys::{Z3_solver_check_assumptions, Z3_L_TRUE};


fn wtf3() {
    unsafe {
        let mut stdlib = ScfiaStdlib::new();
        let expression_replacement = ActiveValue::BoolEqExpression(BoolEqExpression::new(
            ActiveValue::BitvectorConcrete(BitVectorConcrete::new(0x64, 7, &mut stdlib)).into(),
            ActiveValue::BitvectorConcrete(BitVectorConcrete::new(0x3, 7, &mut stdlib)).into(),
            &mut stdlib)).into();
        
        let negated_expression_replacement = ActiveValue::BoolNotExpression(BoolNotExpression::new(expression_replacement, &mut stdlib));
        let negated_z3_ast = negated_expression_replacement.get_z3_ast();
        assert_eq!(Z3_solver_check_assumptions(stdlib.z3_context, stdlib.z3_solver, 1, &negated_z3_ast), Z3_L_TRUE);
    }
}

fn wtf2() {
    let mut stdlib = ScfiaStdlib::new();
    let expression = ActiveValue::BoolEqExpression(
        BoolEqExpression::new(
            ActiveValue::BitvectorConcrete(BitVectorConcrete::new(0x64, 7, &mut stdlib)).into(),
            ActiveValue::BitvectorConcrete(BitVectorConcrete::new(0x3, 7, &mut stdlib)).into(),
            &mut stdlib)).into();
    
    println!("{}", stdlib.do_condition(expression, None));
}

fn wtf() {
    unsafe {
        let op: u32= 0x0404a583;
        let mut stdlib = ScfiaStdlib::new();
        let op_value = ActiveValue::BitvectorConcrete(BitVectorConcrete::new(op as u64, 32, &mut stdlib));
        let dummy_value = ActiveValue::BitvectorConcrete(BitVectorConcrete::new(42, 32, &mut stdlib));
        let mut memory = Memory32::new();
        let mut stable = StableMemoryRegion32::new(0 as u32, 0x4000 as u32);
        stable.write(0x1000, op_value.into(), &mut stdlib);
        stable.write(0x40, dummy_value.into(), &mut stdlib);
        memory.stable_memory_regions.push(stable);

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
                pc: BitVectorConcrete::new(0x00001000, 32, &mut stdlib).into(),
            },
            memory,
            stdlib,
        };

        rv32i_system_state.step()
    }
}

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

    println!("----------------------------------------------------------------------------------------------------------------------------------------");
    let mut successors = continuing.step_forking();
    let mut panicking = successors.remove(0);
    let mut continuing = successors.remove(0);

    println!("Stepping until X...");
    loop {
        continuing.step()
    }

}
