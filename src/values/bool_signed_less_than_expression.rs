use std::{fmt::Debug, marker::PhantomData, cell::RefCell, rc::Weak};

use crate::ScfiaComposition;

use super::active_value::{ActiveValue, ActiveValueWeak, ActiveValueInner};

#[derive(Debug)]
pub struct BoolSignedLessThanExpression<SC: ScfiaComposition> {
    pub s1: ActiveValue<SC>,
    pub s2: ActiveValue<SC>,
    pub is_assert: bool,
}

#[derive(Debug)]
pub struct RetiredBoolSignedLessThanExpression<SC: ScfiaComposition> {
    pub s1: Weak<RefCell<ActiveValueInner<SC>>>,
    pub s1_id: u64,
    pub s2: Weak<RefCell<ActiveValueInner<SC>>>,
    pub s2_id: u64,
    pub is_assert: bool,
    pub phantom: PhantomData<SC>,
}

#[cfg(test)]
mod tests {
    use crate::{models::riscv::rv32i::RV32iScfiaComposition, scfia::Scfia};

    #[test]
    #[allow(unused_must_use)]
    fn test_concrete_slt() {
        simple_logger::SimpleLogger::new().env().init();
        let scfia: Scfia<RV32iScfiaComposition> = Scfia::new(None);
        let s1 = scfia.new_bv_concrete(2, 32, &mut None);
        let s2 = scfia.new_bv_concrete(3, 32, &mut None);
        let slt = scfia.new_bool_signed_less_than(s1, s2, &mut None);
        assert!(slt.try_borrow().unwrap().try_as_concrete_bool().unwrap());

        let scfia: Scfia<RV32iScfiaComposition> = Scfia::new(None);
        let s1 = scfia.new_bv_concrete(-2i32 as u32 as u64, 32, &mut None);
        let s2 = scfia.new_bv_concrete(-1i32 as u32 as u64, 32, &mut None);
        let slt = scfia.new_bool_signed_less_than(s1, s2, &mut None);
        assert!(slt.try_borrow().unwrap().try_as_concrete_bool().unwrap());
    }

    #[test]
    #[allow(unused_must_use)]
    fn test_symbolic_slt() {
        simple_logger::SimpleLogger::new().env().init();
        let scfia: Scfia<RV32iScfiaComposition> = Scfia::new(None);
        let s1 = scfia.new_bv_symbol(32, &mut None);
        let s2 = scfia.new_bv_concrete((i32::MIN as u64) + 1, 32, &mut None);
        let slt = scfia.new_bool_signed_less_than(s1.clone(), s2, &mut None);
        slt.try_borrow_mut().unwrap().assert();
        let mut candidates = vec![i32::MIN as u64];
        s1.try_borrow().unwrap().monomorphize(&mut candidates);
        assert_eq!(candidates.len(), 1)
    }
}
