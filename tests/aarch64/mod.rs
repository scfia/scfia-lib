use std::{collections::HashMap, sync::Arc};

use log::{info, trace};
use scfia_lib::{
    asserts::{scfia_assert::ScfiaAssert, scfia_assert_expression::ScfiaAssertExpression},
    debug_indented::DebugIndented,
    isla::aarch64::isla_handle::AARCH64_ISLA_HANDLE,
    memory::{
        stable_memory_region::StableMemoryRegion, volatile_memory_region::VolatileMemoryRegion,
        ScfiaMemory,
    },
    system_states::aarch64::ScfiaAarch64SystemState,
    transformation_functions::{
        aarch64::{
            transformation_definition::ScfiaAarch64TransformationDefinition,
            transformation_function::{
                ScfiaAarch64TransformationFunction,
                ScfiaAarch64TransformationFunctionSuccessorDefinition,
            },
        },
        transformation_function_expression::ScfiaTransformationFunctionExpression,
        ScfiaTransformationDefinition,
    },
    values::{value64::Value64, value8::Value8},
};

#[test]
fn test_concrete_add() {
    crate::init();
    let mut s0 = ScfiaAarch64SystemState::new();
    s0.pc = 0x8000;
    s0.r0 = Value64::Concrete(2);
    s0.r1 = Value64::Concrete(3);

    // add X2, X1, X0
    let mut data = HashMap::new();
    data.insert(0x8000, Value8::Concrete(0x22));
    data.insert(0x8001, Value8::Concrete(0x00));
    data.insert(0x8002, Value8::Concrete(0x00));
    data.insert(0x8003, Value8::Concrete(0x8b));
    s0.memory.stable_regions.push(StableMemoryRegion {
        range: 0x8000..0x8004,
        data: data,
    });

    let ttf = AARCH64_ISLA_HANDLE.derive_ttf(&s0);
    let s1 = s0.apply_transformation_function(&ttf).remove(0);
    assert_eq!(s1.pc, 0x8004);
    assert_eq!(s1.r2.as_concrete(), 5);
}

#[test]
fn test_symbolic_add() {
    crate::init();
    let mut s0 = ScfiaAarch64SystemState::new();
    s0.pc = 0x8000;
    let r0_symbol = s0
        .symbol_manager
        .active_symbols
        .get_mut(&s0.r0.as_symbolic())
        .unwrap();
    r0_symbol.asserts.push(ScfiaAssert {
        id: 0,
        expression: ScfiaAssertExpression::Eq(
            Arc::new(ScfiaAssertExpression::Var(s0.r0.as_symbolic())),
            Arc::new(ScfiaAssertExpression::Bits64(0x42, 64)),
        ),
    });

    // add X2, X1, X0
    let mut data = HashMap::new();
    data.insert(0x8000, Value8::Concrete(0x22));
    data.insert(0x8001, Value8::Concrete(0x00));
    data.insert(0x8002, Value8::Concrete(0x00));
    data.insert(0x8003, Value8::Concrete(0x8b));
    s0.memory.stable_regions.push(StableMemoryRegion {
        range: 0x8000..0x8004,
        data: data,
    });

    let ttf = AARCH64_ISLA_HANDLE.derive_ttf(&s0);
    let s1 = s0.apply_transformation_function(&ttf).remove(0);
    trace!(
        "r2: {}",
        s1.symbol_manager
            .active_symbols
            .get(&s1.r2.as_symbolic())
            .unwrap()
            .debug_indented(&s1.symbol_manager, 0, 4)
    );
    assert_eq!(s1.pc, 0x8004);
}

#[test]
fn test_symbolic_lifetimes() {
    crate::init();
    let mut s0 = ScfiaAarch64SystemState::new();
    s0.pc = 0x8000;
    let mut data = HashMap::new();
    // mov x0, #1000
    data.insert(0x8000, Value8::Concrete(0x00));
    data.insert(0x8001, Value8::Concrete(0x00));
    data.insert(0x8002, Value8::Concrete(0x82));
    data.insert(0x8003, Value8::Concrete(0xd2));
    // ldr x1, [x0]
    data.insert(0x8004, Value8::Concrete(0x01));
    data.insert(0x8005, Value8::Concrete(0x00));
    data.insert(0x8006, Value8::Concrete(0x40));
    data.insert(0x8007, Value8::Concrete(0xf9));
    // ldr x2, [x0, #8]
    data.insert(0x8008, Value8::Concrete(0x02));
    data.insert(0x8009, Value8::Concrete(0x04));
    data.insert(0x800A, Value8::Concrete(0x40));
    data.insert(0x800B, Value8::Concrete(0xf9));
    // add x3, x2, x1
    data.insert(0x800C, Value8::Concrete(0x43));
    data.insert(0x800D, Value8::Concrete(0x00));
    data.insert(0x800E, Value8::Concrete(0x01));
    data.insert(0x800F, Value8::Concrete(0x8b));
    // add x4, x1, #1
    data.insert(0x8010, Value8::Concrete(0x24));
    data.insert(0x8011, Value8::Concrete(0x04));
    data.insert(0x8012, Value8::Concrete(0x00));
    data.insert(0x8013, Value8::Concrete(0x91));
    // cmp x4, #42
    data.insert(0x8014, Value8::Concrete(0x9f));
    data.insert(0x8015, Value8::Concrete(0xa8));
    data.insert(0x8016, Value8::Concrete(0x00));
    data.insert(0x8017, Value8::Concrete(0xf1));
    // bne far_away
    data.insert(0x8018, Value8::Concrete(0xa1));
    data.insert(0x8019, Value8::Concrete(0x03));
    data.insert(0x801A, Value8::Concrete(0x00));
    data.insert(0x801B, Value8::Concrete(0x54));
    // mov x4, #0
    data.insert(0x801C, Value8::Concrete(0x04));
    data.insert(0x801D, Value8::Concrete(0x00));
    data.insert(0x801E, Value8::Concrete(0x80));
    data.insert(0x801F, Value8::Concrete(0xd2));
    // cmp x4, 0 (clear PSTATE.Z)
    data.insert(0x8020, Value8::Concrete(0x9f));
    data.insert(0x8021, Value8::Concrete(0x00));
    data.insert(0x8022, Value8::Concrete(0x00));
    data.insert(0x8023, Value8::Concrete(0xf1));
    // mov x2, #0
    data.insert(0x8024, Value8::Concrete(0x02));
    data.insert(0x8025, Value8::Concrete(0x00));
    data.insert(0x8026, Value8::Concrete(0x80));
    data.insert(0x8027, Value8::Concrete(0xd2));
    // mov x3, #0
    data.insert(0x8028, Value8::Concrete(0x03));
    data.insert(0x8029, Value8::Concrete(0x00));
    data.insert(0x802A, Value8::Concrete(0x80));
    data.insert(0x802B, Value8::Concrete(0xd2));
    // mov x1, #0
    data.insert(0x802C, Value8::Concrete(0x01));
    data.insert(0x802D, Value8::Concrete(0x00));
    data.insert(0x802E, Value8::Concrete(0x80));
    data.insert(0x802F, Value8::Concrete(0xd2));
    // add x1, x1, #0 clear other flags
    data.insert(0x8030, Value8::Concrete(0x21));
    data.insert(0x8031, Value8::Concrete(0x00));
    data.insert(0x8032, Value8::Concrete(0x00));
    data.insert(0x8033, Value8::Concrete(0xb1));

    s0.memory.stable_regions.push(StableMemoryRegion {
        range: 0x8000..0x8034, //TODO isla should error and not fork when the memory is not found (?)
        data: data,
    });
    s0.memory.volatile_regions.push(VolatileMemoryRegion {
        range: 0x1000..0x2000,
    });

    let ldr1 = s0
        .apply_transformation_function(&AARCH64_ISLA_HANDLE.derive_ttf(&s0))
        .remove(0);
    let ldr2 = ldr1
        .apply_transformation_function(&AARCH64_ISLA_HANDLE.derive_ttf(&ldr1))
        .remove(0);
    let add1 = ldr2
        .apply_transformation_function(&AARCH64_ISLA_HANDLE.derive_ttf(&ldr2))
        .remove(0);
    let add2 = add1
        .apply_transformation_function(&AARCH64_ISLA_HANDLE.derive_ttf(&add1))
        .remove(0);
    let cmp = add2
        .apply_transformation_function(&AARCH64_ISLA_HANDLE.derive_ttf(&add2))
        .remove(0);
    let bne = cmp
        .apply_transformation_function(&AARCH64_ISLA_HANDLE.derive_ttf(&cmp))
        .remove(0);

    //info!("----------");
    //info!("{}", bne.symbol_manager.referenced_symbols.get(&bne.pstate.z.as_symbolic()).unwrap().debug_indented(&bne.symbol_manager, 0, 4));
    //info!("----------");

    let mut branches = bne.apply_transformation_function(&AARCH64_ISLA_HANDLE.derive_ttf(&bne));
    assert_eq!(2, branches.len());
    branches.retain(|s| s.pc == 0x801c);
    assert_eq!(1, branches.len());
    let zero_x4 = branches.remove(0);

    let zero_z = zero_x4
        .apply_transformation_function(&AARCH64_ISLA_HANDLE.derive_ttf(&zero_x4))
        .remove(0);

    let zero_x2 = zero_z
        .apply_transformation_function(&AARCH64_ISLA_HANDLE.derive_ttf(&zero_z))
        .remove(0);

    let zero_x3 = zero_x2
        .apply_transformation_function(&AARCH64_ISLA_HANDLE.derive_ttf(&zero_x2))
        .remove(0);

    let zero_x1 = zero_x3
        .apply_transformation_function(&AARCH64_ISLA_HANDLE.derive_ttf(&zero_x3))
        .remove(0);
    trace!("----");
    let x1_candidates =
        AARCH64_ISLA_HANDLE.monomorphize_u64_scfia_symbol(&zero_x1, zero_x1.r1.as_symbolic());
    trace!("{:?}", &x1_candidates);

    let zero_flags = zero_x1
        .apply_transformation_function(&AARCH64_ISLA_HANDLE.derive_ttf(&zero_x1))
        .remove(0);
    trace!("pc={:x}", zero_flags.pc);
    trace!("r1={:?}", zero_flags.r1);
    trace!("r2={:?}", zero_flags.r2);
    trace!("r3={:?}", zero_flags.r3);
    trace!("r4={:?}", zero_flags.r4);
    trace!("pstate.n={:?}", zero_flags.pstate.n);
    trace!("pstate.z={:?}", zero_flags.pstate.z);
    trace!("pstate.c={:?}", zero_flags.pstate.c);
    trace!("pstate.v={:?}", zero_flags.pstate.v);

    let final_state = zero_flags
        .apply_transformation_function(&AARCH64_ISLA_HANDLE.derive_ttf(&zero_flags))
        .remove(0);

    //let pstate_n_candidates = AARCH64_ISLA_HANDLE.monomorphize_u64_scfia_symbol(&final_state, final_state.pstate.n.as_symbolic());
    //trace!("{:?}", &pstate_n_candidates);
    trace!("pc={:x}", final_state.pc);
    trace!("r1={:?}", final_state.r1);
    trace!("r2={:?}", final_state.r2);
    trace!("r3={:?}", final_state.r3);
    trace!("r4={:?}", final_state.r4);
    trace!("pstate.n={:?}", final_state.pstate.n);
    trace!("pstate.z={:?}", final_state.pstate.z);
    trace!("pstate.c={:?}", final_state.pstate.c);
    trace!("pstate.v={:?}", final_state.pstate.v);
    assert!(final_state.r1.as_concrete() == 0);
    assert!(final_state.r2.as_concrete() == 0);
    assert!(final_state.r3.as_concrete() == 0);
    assert!(final_state.r4.as_concrete() == 0);
    trace!(
        "active symbols: {}",
        final_state.symbol_manager.active_symbols.len()
    );
    println!("{:?}", &final_state);
    assert_eq!(final_state.symbol_manager.retired_symbols.len(), 0);
    /*
    trace!(
        "r2: {}",
        s1.symbol_manager
            .referenced_symbols
            .get(&s1.r2.as_symbolic())
            .unwrap()
            .debug_indented(&s1.symbol_manager, 0, 4)
    );
     */
}

#[test]
fn test_bne() {
    crate::init();
    let mut s0 = ScfiaAarch64SystemState::new();
    s0.pc = 0x8000;
    s0.r21 = Value64::Concrete(0x400);
    s0.r24 = Value64::Concrete(0x400);
    let mut data = HashMap::new();
    // CMP             X21, X24
    data.insert(0x8000, Value8::Concrete(0xbf));
    data.insert(0x8001, Value8::Concrete(0x02));
    data.insert(0x8002, Value8::Concrete(0x18));
    data.insert(0x8003, Value8::Concrete(0xeb));
    // MOV             X1, X24
    data.insert(0x8004, Value8::Concrete(0xe1));
    data.insert(0x8005, Value8::Concrete(0x03));
    data.insert(0x8006, Value8::Concrete(0x18));
    data.insert(0x8007, Value8::Concrete(0xaa));
    // B.NE            loc_40085A08
    data.insert(0x8008, Value8::Concrete(0xa1));
    data.insert(0x8009, Value8::Concrete(0xfe));
    data.insert(0x800A, Value8::Concrete(0xff));
    data.insert(0x800B, Value8::Concrete(0x54));

    s0.memory.stable_regions.push(StableMemoryRegion {
        range: 0x8000..0x800C,
        data: data,
    });

    let mut s1 = s0.step().remove(0);
    let mut s2 = s1.step().remove(0);
    let mut s3_vec = s2.step();
    assert!(s3_vec.len() == 1);
    let s3 = s3_vec.remove(0);
    info!("{}", s3);
}
