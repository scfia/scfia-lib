use std::fmt::Debug;

use super::active_value::{ActiveValue, ActiveValueWeak};

pub struct BVAddExpression {
    pub s1: ActiveValue,
    pub s2: ActiveValue,
    pub width: u32,
}

pub struct RetiredBVAddExpression {
    pub s1: ActiveValueWeak,
    pub s2: ActiveValueWeak,
    pub width: u32,
}

impl Debug for BVAddExpression {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str("(")?;
        self.s1.try_borrow().unwrap().fmt(f)?;
        f.write_str(" + ")?;
        self.s2.try_borrow().unwrap().fmt(f)?;
        f.write_str(")")
    }
}

impl Debug for RetiredBVAddExpression {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str("RetiredBVAddExpression)")
    }
}

#[cfg(test)]
mod tests {
    use crate::scfia::Scfia;

    #[test]
    #[allow(unused_must_use)]
    fn test_concrete_add() {
        simple_logger::SimpleLogger::new().env().init();
        let scfia = Scfia::new();
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
        let scfia = Scfia::new();
        let s1 = scfia.new_bv_concrete(1, 32, &mut None);
        let s2 = scfia.new_bv_symbol(32, &mut None);
        let _ = scfia.new_bv_add(s1, s2, 32, &mut None);
        //TODO better test
    }
}
