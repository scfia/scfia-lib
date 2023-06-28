use std::{cell::RefCell, fmt::Debug, marker::PhantomData, rc::Rc};

use crate::ScfiaComposition;

use super::{active_value::ActiveValueZ3, retired_value::ParentWeakReference};

#[derive(Debug)]
pub struct BVSrlExpression<SC: ScfiaComposition> {
    pub s1: Rc<RefCell<ActiveValueZ3<SC>>>,
    pub s2: Rc<RefCell<ActiveValueZ3<SC>>>,
    pub width: u32,
    pub shamt: u32,
}

#[derive(Debug)]
pub struct RetiredBVSrlExpression<SC: ScfiaComposition> {
    pub s1: ParentWeakReference<SC>,
    pub s2: ParentWeakReference<SC>,
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
