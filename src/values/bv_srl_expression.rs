use std::{fmt::Debug, marker::PhantomData, cell::RefCell, rc::Weak};

use crate::ScfiaComposition;

use super::active_value::{ActiveValue, ActiveValueWeak, ActiveValueInner};

#[derive(Debug)]
pub struct BVSrlExpression<SC: ScfiaComposition> {
    pub s1: ActiveValue<SC>,
    pub s2: ActiveValue<SC>,
    pub width: u32,
    pub shamt: u32,
}

#[derive(Debug)]
pub struct RetiredBVSrlExpression<SC: ScfiaComposition> {
    pub s1: Weak<RefCell<ActiveValueInner<SC>>>,
    pub s1_id: u64,
    pub s2: Weak<RefCell<ActiveValueInner<SC>>>,
    pub s2_id: u64,
    pub width: u32,
    pub shamt: u32,
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
        let s1 = scfia.new_bv_concrete(0b110011, 6, &mut None);
        let s2 = scfia.new_bv_concrete(1, 6, &mut None);
        let sll = scfia.new_bv_srl(s1, s2, 6, 6, &mut None);
        assert_eq!(sll.try_borrow().unwrap().try_as_concrete_bv().unwrap(), 0b011001);
    }
    //TODO test
}
*/
