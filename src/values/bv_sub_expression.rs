use std::{cell::RefCell, fmt::Debug, marker::PhantomData, rc::Weak};

use crate::ScfiaComposition;

use super::{active_value::{ActiveValue, ActiveValueExpression}, retired_value::ParentWeakReference};

#[derive(Debug)]
pub struct BVSubExpression<SC: ScfiaComposition> {
    pub s1: ActiveValue<SC>,
    pub s2: ActiveValue<SC>,
    pub width: u32,
}

#[derive(Debug)]
pub struct RetiredBVSubExpression<SC: ScfiaComposition> {
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
    fn test_concrete() {
        simple_logger::SimpleLogger::new().env().init();
        let scfia: ScfiaOld<RV32iScfiaComposition> = ScfiaOld::new(None);
        let s1 = scfia.new_bv_concrete(42, 32, &mut None);
        let s2 = scfia.new_bv_concrete(41, 32, &mut None);
        let sub = scfia.new_bv_sub(s1, s2, 32, &mut None);
        assert_eq!(sub.try_borrow().unwrap().try_as_concrete_bv().unwrap(), 1);

        let s1 = scfia.new_bv_concrete(0, 32, &mut None);
        let s2 = scfia.new_bv_concrete(1, 32, &mut None);
        let sub = scfia.new_bv_sub(s1, s2, 32, &mut None);
        assert_eq!(sub.try_borrow().unwrap().try_as_concrete_bv().unwrap(), u32::MAX as u64);
    }
    //TODO test
}
*/
