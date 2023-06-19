use std::{fmt::Debug, marker::PhantomData, cell::RefCell, rc::Weak};

use crate::ScfiaComposition;

use super::active_value::{ActiveValue, ActiveValueWeak, ActiveValueInner};

#[derive(Debug)]
pub struct BVSliceExpression<SC: ScfiaComposition> {
    pub s1: ActiveValue<SC>,
    pub width: u32,
    pub high: u32,
    pub low: u32,
}

#[derive(Debug)]
pub struct RetiredBVSliceExpression<SC: ScfiaComposition> {
    pub s1: Weak<RefCell<ActiveValueInner<SC>>>,
    pub s1_id: u64,
    pub width: u32,
    pub high: u32,
    pub low: u32,
    pub phantom: PhantomData<SC>,
}

#[cfg(test)]
mod tests {
    use crate::{models::riscv::rv32i::RV32iScfiaComposition, scfia::Scfia};

    #[test]
    #[allow(unused_must_use)]
    fn test_concrete() {
        simple_logger::SimpleLogger::new().env().init();
        let scfia: Scfia<RV32iScfiaComposition> = Scfia::new(None);
        let s1 = scfia.new_bv_concrete(0b110011, 6, &mut None);
        let slice = scfia.new_bv_slice(s1, 4, 1, &mut None);
        assert_eq!(slice.try_borrow().unwrap().try_as_concrete_bv().unwrap(), 0b1001);
    }
    //TODO test
}
