use std::{cell::RefCell, fmt::Debug, marker::PhantomData, rc::Weak};

use crate::ScfiaComposition;

use super::active_value::{ActiveValue, ActiveValueInner};

#[derive(Debug)]
pub struct BVUnsignedRemainderExpression<SC: ScfiaComposition> {
    pub s1: ActiveValue<SC>,
    pub s2: ActiveValue<SC>,
    pub width: u32,
}

#[derive(Debug)]
pub struct RetiredBVUnsignedRemainderExpression<SC: ScfiaComposition> {
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
    fn test_concrete() {
        simple_logger::SimpleLogger::new().env().init();
        let scfia: ScfiaOld<RV32iScfiaComposition> = ScfiaOld::new(None);
        let s1 = scfia.new_bv_concrete(110, 32, &mut None);
        let s2 = scfia.new_bv_concrete(100, 30, &mut None);
        let unsigned_remainder = scfia.new_bv_unsigned_remainder(s1, s2, 32, &mut None);
        assert_eq!(unsigned_remainder.try_borrow().unwrap().try_as_concrete_bv().unwrap(), 10);
    }
    //TODO test
}
*/
