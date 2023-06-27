use std::{cell::RefCell, fmt::Debug, marker::PhantomData, rc::{Weak, Rc}};

use crate::ScfiaComposition;

use super::{active_value::{ActiveValue, ActiveValueZ3}, retired_value::ParentWeakReference};

#[derive(Debug)]
pub struct BoolUnsignedLessThanExpression<SC: ScfiaComposition> {
    pub s1: Rc<RefCell<ActiveValueZ3<SC>>>,
    pub s2: Rc<RefCell<ActiveValueZ3<SC>>>,
    pub is_assert: bool,
}

#[derive(Debug)]
pub struct RetiredBoolUnsignedLessThanExpression<SC: ScfiaComposition> {
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
    fn test_concrete_ult() {
        simple_logger::SimpleLogger::new().env().init();
        let scfia: ScfiaOld<RV32iScfiaComposition> = ScfiaOld::new(None);
        let s1 = scfia.new_bv_concrete(2, 32, &mut None);
        let s2 = scfia.new_bv_concrete(3, 32, &mut None);
        let slt = scfia.new_bool_unsigned_less_than(s1, s2, &mut None);
        assert!(slt.try_borrow().unwrap().try_as_concrete_bool().unwrap());
    }

    #[test]
    #[allow(unused_must_use)]
    fn test_symbolic_ult() {
        simple_logger::SimpleLogger::new().env().init();
        let scfia: ScfiaOld<RV32iScfiaComposition> = ScfiaOld::new(None);
        let s1 = scfia.new_bv_symbol(32, &mut None);
        let s2 = scfia.new_bv_concrete(1, 32, &mut None);
        let slt = scfia.new_bool_unsigned_less_than(s1.clone(), s2, &mut None);
        slt.try_borrow_mut().unwrap().assert();
        let mut candidates = vec![0];
        s1.try_borrow().unwrap().monomorphize(&mut candidates);
        assert_eq!(candidates.len(), 1)
    }
}
*/
