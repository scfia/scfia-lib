use std::{cell::RefCell, fmt::Debug, marker::PhantomData, rc::Rc};

use crate::ScfiaComposition;

use super::{active_value::ActiveValueZ3, retired_value::ParentWeakReference};

#[derive(Debug)]
pub struct BVConcatExpression<SC: ScfiaComposition> {
    pub s1: Rc<RefCell<ActiveValueZ3<SC>>>,
    pub s2: Rc<RefCell<ActiveValueZ3<SC>>>,
    pub width: u32,
}

#[derive(Debug)]
pub struct RetiredBVConcatExpression<SC: ScfiaComposition> {
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
    fn test_concrete_concat() {
        simple_logger::SimpleLogger::new().env().init();
        let scfia: ScfiaOld<RV32iScfiaComposition> = ScfiaOld::new(None);
        let s1 = scfia.new_bv_concrete(0x10, 32, &mut None);
        let s2 = scfia.new_bv_concrete(2, 32, &mut None);
        let concat = scfia.new_bv_concat(s1, s2, 64, &mut None);
        assert_eq!(concat.try_borrow().unwrap().try_as_concrete_bv().unwrap(), 0x10_00000002);

        let s1 = scfia.new_bv_concrete(0b01, 2, &mut None);
        let s2 = scfia.new_bv_concrete(0b10, 2, &mut None);
        let concat = scfia.new_bv_concat(s1, s2, 4, &mut None);
        assert_eq!(concat.try_borrow().unwrap().try_as_concrete_bv().unwrap(), 0b0110);
    }

    #[test]
    #[allow(unused_must_use)]
    fn test_symbol_concat() {
        simple_logger::SimpleLogger::new().env().init();
        let scfia: ScfiaOld<RV32iScfiaComposition> = ScfiaOld::new(None);
        let s1 = scfia.new_bv_concrete(0, 32, &mut None);
        let s2 = scfia.new_bv_symbol(32, &mut None);
        let _ = scfia.new_bv_concat(s1, s2, 32, &mut None);
        //TODO better test
    }
}
*/
