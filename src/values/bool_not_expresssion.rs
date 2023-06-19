use std::{fmt::Debug, marker::PhantomData, cell::RefCell, rc::Weak};

use crate::ScfiaComposition;

use super::active_value::{ActiveValue, ActiveValueWeak, ActiveValueInner};

#[derive(Debug)]
pub struct BoolNotExpression<SC: ScfiaComposition> {
    pub s1: ActiveValue<SC>,
    pub is_assert: bool,
}

#[derive(Debug)]
pub struct RetiredBoolNotExpression<SC: ScfiaComposition> {
    pub s1: Weak<RefCell<ActiveValueInner<SC>>>,
    pub s1_id: u64,
    pub is_assert: bool,
    pub phantom: PhantomData<SC>,
}

#[cfg(test)]
mod tests {
    use crate::{models::riscv::rv32i::RV32iScfiaComposition, scfia::Scfia};

    #[test]
    #[allow(unused_must_use)]
    fn test_concrete_not() {
        simple_logger::SimpleLogger::new().env().init();
        let scfia: Scfia<RV32iScfiaComposition> = Scfia::new(None);
        let s1 = scfia.new_bv_concrete(2, 32, &mut None);
        let s2 = scfia.new_bv_concrete(2, 32, &mut None);
        let eq = scfia.new_bool_eq(s1, s2, &mut None);
        let not = scfia.new_bool_not(eq, &mut None);
        assert!(!not.try_borrow().unwrap().try_as_concrete_bool().unwrap());
        assert!(!scfia.check_condition(not, &mut None));

        let scfia: Scfia<RV32iScfiaComposition> = Scfia::new(None);
        let s1 = scfia.new_bv_concrete(1, 32, &mut None);
        let s2 = scfia.new_bv_concrete(2, 32, &mut None);
        let eq = scfia.new_bool_eq(s1, s2, &mut None);
        let not = scfia.new_bool_not(eq, &mut None);
        assert!(not.try_borrow().unwrap().try_as_concrete_bool().unwrap());
        assert!(scfia.check_condition(not, &mut None));
    }

    #[test]
    #[allow(unused_must_use)]
    fn test_symbolic_not() {
        simple_logger::SimpleLogger::new().env().init();
        let scfia: Scfia<RV32iScfiaComposition> = Scfia::new(None);
        let s1 = scfia.new_bv_concrete(42, 32, &mut None);
        let s2 = scfia.new_bv_symbol(32, &mut None);
        let eq = scfia.new_bool_eq(s1, s2.clone(), &mut None);
        let not = scfia.new_bool_not(eq.clone(), &mut None);
        not.try_borrow_mut().unwrap().assert();
        // TODO better test
        assert!(scfia.check_condition(not, &mut None));
        assert!(!scfia.check_condition(eq, &mut None));
    }
}
