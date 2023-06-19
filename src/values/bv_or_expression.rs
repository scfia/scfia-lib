use std::{fmt::Debug, marker::PhantomData, cell::RefCell, rc::Weak};

use crate::ScfiaComposition;

use super::active_value::{ActiveValue, ActiveValueWeak, ActiveValueInner};

#[derive(Debug)]
pub struct BVOrExpression<SC: ScfiaComposition> {
    pub s1: ActiveValue<SC>,
    pub s2: ActiveValue<SC>,
    pub width: u32,
}

#[derive(Debug)]
pub struct RetiredBVOrExpression<SC: ScfiaComposition> {
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
    fn test_concrete_or() {
        simple_logger::SimpleLogger::new().env().init();
        let scfia: ScfiaOld<RV32iScfiaComposition> = ScfiaOld::new(None);
        let s1 = scfia.new_bv_concrete(1, 32, &mut None);
        let s2 = scfia.new_bv_concrete(2, 32, &mut None);
        let or = scfia.new_bv_or(s1, s2, 32, &mut None);
        assert_eq!(or.try_borrow().unwrap().try_as_concrete_bv().unwrap(), 3);
    }

    #[test]
    #[allow(unused_must_use)]
    fn test_symbol_or() {
        simple_logger::SimpleLogger::new().env().init();
        let scfia: ScfiaOld<RV32iScfiaComposition> = ScfiaOld::new(None);
        let s1 = scfia.new_bv_concrete(u32::MAX as u64 - 1, 32, &mut None);
        let s2 = scfia.new_bv_symbol(32, &mut None);
        let or = scfia.new_bv_or(s1, s2, 32, &mut None);
        let mut candidates = vec![u32::MAX as u64 - 1, u32::MAX as u64];
        or.try_borrow().unwrap().monomorphize(&mut candidates);
        assert_eq!(candidates.len(), 2)
    }
}
*/
