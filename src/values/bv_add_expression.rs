use std::fmt::Debug;

use crate::ScfiaComposition;

use super::active_value::{ActiveValue, ActiveValueWeak};

pub struct BVAddExpression<SC: ScfiaComposition> {
    pub s1: ActiveValue<SC>,
    pub s2: ActiveValue<SC>,
    pub width: u32,
}

pub struct RetiredBVAddExpression<SC: ScfiaComposition> {
    pub s1: ActiveValueWeak<SC>,
    pub s2: ActiveValueWeak<SC>,
    pub width: u32,
}

impl<SC: ScfiaComposition> Debug for BVAddExpression<SC> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str("(")?;
        self.s1.try_borrow().unwrap().fmt(f)?;
        f.write_str(" + ")?;
        self.s2.try_borrow().unwrap().fmt(f)?;
        f.write_str(")")
    }
}

impl<SC: ScfiaComposition> Debug for RetiredBVAddExpression<SC> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str("RetiredBVAddExpression)")
    }
}

#[cfg(test)]
mod tests {
    use crate::{models::riscv::rv32i::RV32iScfiaComposition, scfia::Scfia};

    #[test]
    #[allow(unused_must_use)]
    fn test_concrete_add() {
        simple_logger::SimpleLogger::new().env().init();
        let scfia: Scfia<RV32iScfiaComposition> = Scfia::new(None);
        let s1 = scfia.new_bv_concrete(1, 32, &mut None);
        let s2 = scfia.new_bv_concrete(2, 32, &mut None);
        let add = scfia.new_bv_add(s1, s2, 32, &mut None);
        assert_eq!(add.try_borrow().unwrap().try_as_concrete_bv().unwrap(), 3);

        let s1 = scfia.new_bv_concrete(1, 32, &mut None);
        let s2 = scfia.new_bv_concrete(u32::MAX as u64, 32, &mut None);
        let add = scfia.new_bv_add(s1, s2, 32, &mut None);
        assert_eq!(add.try_borrow().unwrap().try_as_concrete_bv().unwrap(), 0);
    }

    #[test]
    #[allow(unused_must_use)]
    fn test_symbol_add() {
        simple_logger::SimpleLogger::new().env().init();
        let scfia: Scfia<RV32iScfiaComposition> = Scfia::new(None);
        let s1 = scfia.new_bv_concrete(1, 32, &mut None);
        let s2 = scfia.new_bv_symbol(32, &mut None);
        let _ = scfia.new_bv_add(s1, s2, 32, &mut None);
        //TODO better test
    }
}
