use isla_lib::{
    concrete::bitvector64::B64,
    smt::{smtlib::Exp, Solver, Sym},
};

pub mod aarch64;
pub mod map_isla_exp_to_transformation_expression;
pub mod memory;

pub fn find_isla_sym_maximum(sym: Sym, solver: &mut Solver<B64>, width: u32) -> u64 {
    let mut right = match width {
        64 => u64::MAX,
        32 => u32::MAX as u64,
        15 => u16::MAX as u64,
        8 => u8::MAX as u64,
        _ => panic!("{}", width),
    };
    let mut left = 0;

    if !solver
        .check_sat_with(&Exp::Bvuge(
            Box::new(Exp::Var(sym)),
            Box::new(Exp::Bits64(left, width)),
        ))
        .is_sat()
        .unwrap()
    {
        panic!("Symbol is not smaller or equal to 0")
    }
    let mut t = (right - left) / 2;

    while left + 1 != right {
        if solver
            .check_sat_with(&Exp::Bvuge(
                Box::new(Exp::Var(sym)),
                Box::new(Exp::Bits64(t, width)),
            ))
            .is_sat()
            .unwrap()
        {
            left = t;
            t = left + (right - left) / 2;
        } else {
            right = t;
            t = left + (right - left) / 2;
        }
    }
    if solver
        .check_sat_with(&Exp::Bvuge(
            Box::new(Exp::Var(sym)),
            Box::new(Exp::Bits64(right, width)),
        ))
        .is_sat()
        .unwrap()
    {
        right
    } else {
        left
    }
}

pub fn find_isla_sym_minimum(sym: Sym, solver: &mut Solver<B64>, width: u32) -> u64 {
    let mut right = match width {
        64 => u64::MAX,
        32 => u32::MAX as u64,
        15 => u16::MAX as u64,
        8 => u8::MAX as u64,
        _ => panic!("{}", width),
    };
    let mut left = 0;

    if !solver
        .check_sat_with(&Exp::Bvule(
            Box::new(Exp::Var(sym)),
            Box::new(Exp::Bits64(right, width)),
        ))
        .is_sat()
        .unwrap()
    {
        panic!("Symbol is not smaller or equal to 0")
    }
    let mut t = (right - left) / 2;

    while left + 1 != right {
        if solver
            .check_sat_with(&Exp::Bvule(
                Box::new(Exp::Var(sym)),
                Box::new(Exp::Bits64(t, width)),
            ))
            .is_sat()
            .unwrap()
        {
            right = t;
            t = left + (right - left) / 2;
        } else {
            left = t;
            t = left + (right - left) / 2;
        }
    }
    if solver
        .check_sat_with(&Exp::Bvule(
            Box::new(Exp::Var(sym)),
            Box::new(Exp::Bits64(left, width)),
        ))
        .is_sat()
        .unwrap()
    {
        left
    } else {
        right
    }
}
