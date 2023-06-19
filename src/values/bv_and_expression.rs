use std::{fmt::Debug, marker::PhantomData, cell::RefCell, rc::Weak};

use crate::ScfiaComposition;

use super::active_value::{ActiveValue, ActiveValueWeak, ActiveValueInner};

#[derive(Debug)]
pub struct BVAndExpression<SC: ScfiaComposition> {
    pub s1: ActiveValue<SC>,
    pub s2: ActiveValue<SC>,
    pub width: u32,
}

#[derive(Debug)]
pub struct RetiredBVAndExpression<SC: ScfiaComposition> {
    pub s1: Weak<RefCell<ActiveValueInner<SC>>>,
    pub s1_id: u64,
    pub s2: Weak<RefCell<ActiveValueInner<SC>>>,
    pub s2_id: u64,
    pub width: u32,
    pub phantom: PhantomData<SC>,
}

/*
#[cfg(test)]
mod tests {
    use crate::{models::riscv::rv32i::RV32iScfiaComposition, scfia::ScfiaOld};

    #[test]
    #[allow(unused_must_use)]
    fn test_concrete_and1() {
        simple_logger::SimpleLogger::new().env().init();
        let scfia: ScfiaOld<RV32iScfiaComposition> = ScfiaOld::new(None);
        let s1 = scfia.new_bv_concrete(1, 32, &mut None);
        let s2 = scfia.new_bv_concrete(0xffffff, 32, &mut None);
        let add = scfia.new_bv_and(s1, s2, 32, &mut None);
        assert_eq!(add.try_borrow().unwrap().try_as_concrete_bv().unwrap(), 1);
    }

    #[test]
    #[allow(unused_must_use)]
    fn test_concrete_and2() {
        simple_logger::SimpleLogger::new().env().init();
        let scfia: ScfiaOld<RV32iScfiaComposition> = ScfiaOld::new(None);
        let s1 = scfia.new_bv_concrete(0b010001, 32, &mut None);
        let s2 = scfia.new_bv_concrete(0b010101, 32, &mut None);
        let add = scfia.new_bv_and(s1, s2, 32, &mut None);
        assert_eq!(add.try_borrow().unwrap().try_as_concrete_bv().unwrap(), 0b010001);

        let s1 = scfia.new_bv_concrete(0xfffff000, 32, &mut None);
        let s2 = scfia.new_bv_concrete(0x3005, 32, &mut None);
        let add = scfia.new_bv_and(s2, s1, 32, &mut None);
        assert_eq!(add.try_borrow().unwrap().try_as_concrete_bv().unwrap(), 0x3000);
    }

    #[test]
    #[allow(unused_must_use)]
    fn test_symbol_and() {
        simple_logger::SimpleLogger::new().env().init();
        let scfia: ScfiaOld<RV32iScfiaComposition> = ScfiaOld::new(None);
        let s1 = scfia.new_bv_concrete(0, 32, &mut None);
        let s2 = scfia.new_bv_symbol(32, &mut None);
        let and = scfia.new_bv_and(s1, s2, 32, &mut None);
        let mut candidates = vec![0];
        and.try_borrow().unwrap().monomorphize(&mut candidates);
        assert_eq!(candidates.len(), 1)
    }
}
*/
