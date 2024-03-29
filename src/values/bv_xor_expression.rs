use std::{cell::RefCell, fmt::Debug, marker::PhantomData, rc::Rc};

use crate::ScfiaComposition;

use super::{active_value::ActiveValueZ3, retired_value::ParentWeakReference};

#[derive(Debug)]
pub struct BVXorExpression<SC: ScfiaComposition> {
    pub s1: Rc<RefCell<ActiveValueZ3<SC>>>,
    pub s2: Rc<RefCell<ActiveValueZ3<SC>>>,
    pub width: u32,
}

#[derive(Debug)]
pub struct RetiredBVXorExpression<SC: ScfiaComposition> {
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
        let s1 = scfia.new_bv_concrete(0b01010101, 8, &mut None);
        let s2 = scfia.new_bv_concrete(0b11110101, 8, &mut None);
        let xor = scfia.new_bv_xor(s1, s2, 8, &mut None);
        assert_eq!(xor.try_borrow().unwrap().try_as_concrete_bv().unwrap(), 0b10100000);

        let s1 = scfia.new_bv_concrete(u32::MAX as u64, 32, &mut None);
        let s2 = scfia.new_bv_concrete(0b1010_10101010, 32, &mut None);
        let xor = scfia.new_bv_xor(s1, s2, 32, &mut None);
        assert_eq!(xor.try_borrow().unwrap().try_as_concrete_bv().unwrap(), !0b1010_10101010 as u32 as u64);

        let s1 = scfia.new_bv_concrete(u32::MAX as u64, 32, &mut None);
        let s2 = scfia.new_bv_concrete(u32::MAX as u64, 32, &mut None);
        let xor = scfia.new_bv_xor(s1, s2, 32, &mut None);
        assert_eq!(xor.try_borrow().unwrap().try_as_concrete_bv().unwrap(), u32::MIN as u64);
    }
    //TODO test
}
*/
