use std::{cell::RefCell, fmt::Debug, marker::PhantomData, rc::Rc};

use crate::ScfiaComposition;

use super::{active_value::ActiveValueZ3, retired_value::ParentWeakReference};

#[derive(Debug)]
pub struct BVMultiplyExpression<SC: ScfiaComposition> {
    pub s1: Rc<RefCell<ActiveValueZ3<SC>>>,
    pub s2: Rc<RefCell<ActiveValueZ3<SC>>>,
    pub width: u32,
}

#[derive(Debug)]
pub struct RetiredBVMultiplyExpression<SC: ScfiaComposition> {
    pub s1: ParentWeakReference<SC>,
    pub s2: ParentWeakReference<SC>,
    pub width: u32,
    pub phantom: PhantomData<SC>,
}

/*
#[cfg(test)]
mod tests {
    use crate::{models::riscv::rv32i::RV32iScfiaComposition, scfia::ScfiaOld};

    #[test]
    #[allow(unused_must_use)]
    fn test_concrete_multiply() {
        simple_logger::SimpleLogger::new().env().init();
        let scfia: ScfiaOld<RV32iScfiaComposition> = ScfiaOld::new(None);
        let s1 = scfia.new_bv_concrete(2, 32, &mut None);
        let s2 = scfia.new_bv_concrete(3, 32, &mut None);
        let mul = scfia.new_bv_multiply(s1, s2, 32, &mut None);
        assert_eq!(mul.try_borrow().unwrap().try_as_concrete_bv().unwrap(), 6);

        let s1 = scfia.new_bv_concrete(u32::MAX as _, 32, &mut None);
        let s2 = scfia.new_bv_concrete(2, 32, &mut None);
        let mul = scfia.new_bv_multiply(s1, s2, 32, &mut None);
        assert_eq!(mul.try_borrow().unwrap().try_as_concrete_bv().unwrap(), 0xFFFFFFFE);
    }

    #[test]
    #[allow(unused_must_use)]
    fn test_symbol_multiply() {
        simple_logger::SimpleLogger::new().env().init();
        let scfia: ScfiaOld<RV32iScfiaComposition> = ScfiaOld::new(None);
        let s1 = scfia.new_bv_concrete(0, 32, &mut None);
        let s2 = scfia.new_bv_symbol(32, &mut None);
        let mul = scfia.new_bv_multiply(s1, s2, 32, &mut None);
        let mut candidates = vec![0];
        mul.try_borrow().unwrap().monomorphize(&mut candidates);
        assert_eq!(candidates.len(), 1)
    }
}
*/
