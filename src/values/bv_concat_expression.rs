use std::{fmt::Debug, marker::PhantomData, cell::RefCell, rc::Weak};

use crate::ScfiaComposition;

use super::active_value::{ActiveValue, ActiveValueWeak, ActiveValueInner};

#[derive(Debug)]
pub struct BVConcatExpression<SC: ScfiaComposition> {
    pub s1: ActiveValue<SC>,
    pub s2: ActiveValue<SC>,
    pub width: u32,
}

#[derive(Debug)]
pub struct RetiredBVConcatExpression<SC: ScfiaComposition> {
    pub s1: Weak<RefCell<ActiveValueInner<SC>>>,
    pub s1_id: u64,
    pub s2: Weak<RefCell<ActiveValueInner<SC>>>,
    pub s2_id: u64,
    pub width: u32,
    pub phantom: PhantomData<SC>,
}

#[cfg(test)]
mod tests {
    use crate::{models::riscv::rv32i::RV32iScfiaComposition, scfia::Scfia};

    #[test]
    #[allow(unused_must_use)]
    fn test_concrete_concat() {
        simple_logger::SimpleLogger::new().env().init();
        let scfia: Scfia<RV32iScfiaComposition> = Scfia::new(None);
        let s1 = scfia.new_bv_concrete(0x10, 32, &mut None);
        let s2 = scfia.new_bv_concrete(2, 32, &mut None);
        let concat = scfia.new_bv_concat(s1, s2, 64, &mut None);
        assert_eq!(concat.try_borrow().unwrap().try_as_concrete_bv().unwrap(), 0x10_00000002);

        let s1 = scfia.new_bv_concrete(0b01, 2, &mut None);
        let s2 = scfia.new_bv_concrete(0b10, 2, &mut None);
        let concat = scfia.new_bv_concat(s1, s2, 4, &mut None);
        assert_eq!(concat.try_borrow().unwrap().try_as_concrete_bv().unwrap(), 0b0110);
    }

    #[test]
    #[allow(unused_must_use)]
    fn test_symbol_concat() {
        simple_logger::SimpleLogger::new().env().init();
        let scfia: Scfia<RV32iScfiaComposition> = Scfia::new(None);
        let s1 = scfia.new_bv_concrete(0, 32, &mut None);
        let s2 = scfia.new_bv_symbol(32, &mut None);
        let _ = scfia.new_bv_concat(s1, s2, 32, &mut None);
        //TODO better test
    }
}
