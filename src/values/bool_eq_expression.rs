use std::{cell::RefCell, fmt::Debug, marker::PhantomData, rc::{Rc}};

use crate::ScfiaComposition;

use super::{active_value::{ActiveValueZ3}, retired_value::ParentWeakReference};

#[derive(Debug)]
pub struct BoolEqExpression<SC: ScfiaComposition> {
    pub s1: Rc<RefCell<ActiveValueZ3<SC>>>,
    pub s2: Rc<RefCell<ActiveValueZ3<SC>>>,
    pub is_assert: bool,
}

#[derive(Debug)]
pub struct RetiredBoolEqExpression<SC: ScfiaComposition> {
    pub s1: ParentWeakReference<SC>,
    pub s2: ParentWeakReference<SC>,
    pub is_assert: bool,
    pub phantom: PhantomData<SC>,
}
/*
#[cfg(test)]
mod tests {
    use crate::{models::riscv::rv32i::RV32iScfiaComposition, scfia::ScfiaOld};

    #[test]
    #[allow(unused_must_use)]
    fn test_concrete_eq() {
        simple_logger::SimpleLogger::new().env().init();
        let scfia: ScfiaOld<RV32iScfiaComposition> = ScfiaOld::new(None);
        let s1 = scfia.new_bv_concrete(2, 32, &mut None);
        let s2 = scfia.new_bv_concrete(2, 32, &mut None);
        let eq = scfia.new_bool_eq(s1, s2, &mut None);
        assert!(eq.try_borrow().unwrap().try_as_concrete_bool().unwrap());

        let scfia: ScfiaOld<RV32iScfiaComposition> = ScfiaOld::new(None);
        let s1 = scfia.new_bv_concrete(1, 32, &mut None);
        let s2 = scfia.new_bv_concrete(2, 32, &mut None);
        let add = scfia.new_bool_eq(s1, s2, &mut None);
        assert!(!add.try_borrow().unwrap().try_as_concrete_bool().unwrap());
    }

    #[test]
    #[allow(unused_must_use)]
    fn test_symbolic_eq() {
        simple_logger::SimpleLogger::new().env().init();
        let scfia: ScfiaOld<RV32iScfiaComposition> = ScfiaOld::new(None);
        let s1 = scfia.new_bv_concrete(42, 32, &mut None);
        let s2 = scfia.new_bv_symbol(32, &mut None);
        let eq = scfia.new_bool_eq(s1, s2.clone(), &mut None);
        eq.try_borrow_mut().unwrap().assert();
        let mut candidates = vec![42];
        s2.try_borrow().unwrap().monomorphize(&mut candidates);
        assert_eq!(candidates.len(), 1)
    }
}
*/
