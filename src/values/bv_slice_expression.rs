use std::{cell::RefCell, fmt::Debug, marker::PhantomData, rc::{Weak, Rc}};

use crate::ScfiaComposition;

use super::{active_value::{ActiveValue, ActiveValueZ3}, retired_value::ParentWeakReference};

#[derive(Debug)]
pub struct BVSliceExpression<SC: ScfiaComposition> {
    pub s1: Rc<RefCell<ActiveValueZ3<SC>>>,
    pub width: u32,
    pub high: u32,
    pub low: u32,
}

#[derive(Debug)]
pub struct RetiredBVSliceExpression<SC: ScfiaComposition> {
    pub s1: ParentWeakReference<SC>,
    pub width: u32,
    pub high: u32,
    pub low: u32,
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
        let slice = scfia.new_bv_slice(s1, 4, 1, &mut None);
        assert_eq!(slice.try_borrow().unwrap().try_as_concrete_bv().unwrap(), 0b1001);
    }
    //TODO test
}
*/
