use std::fmt::Debug;

use super::active_value::{ActiveValue, ActiveValueWeak};

pub struct BoolEqExpression {
    pub s1: ActiveValue,
    pub s2: ActiveValue,
    pub is_assert: bool,
}

pub struct RetiredBoolEqExpression {
    pub s1: ActiveValueWeak,
    pub s2: ActiveValueWeak,
    pub is_assert: bool,
}

impl Debug for BoolEqExpression {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str("(")?;
        self.s1.try_borrow().unwrap().fmt(f)?;
        f.write_str(" == ")?;
        self.s2.try_borrow().unwrap().fmt(f)?;
        f.write_str(")")
    }
}

impl Debug for RetiredBoolEqExpression {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str("RetiredBoolEqExpression")
    }
}

#[cfg(test)]
mod tests {
    use crate::scfia::Scfia;

    #[test]
    #[allow(unused_must_use)]
    fn test_concrete_eq() {
        simple_logger::SimpleLogger::new().env().init();
        let scfia = Scfia::new();
        let s1 = scfia.new_bv_concrete(2, 32, &mut None);
        let s2 = scfia.new_bv_concrete(2, 32, &mut None);
        let eq = scfia.new_bool_eq(s1, s2, &mut None);
        assert!(eq.try_borrow().unwrap().try_as_concrete_bool().unwrap());

        let scfia = Scfia::new();
        let s1 = scfia.new_bv_concrete(1, 32, &mut None);
        let s2 = scfia.new_bv_concrete(2, 32, &mut None);
        let add = scfia.new_bool_eq(s1, s2, &mut None);
        assert!(!add.try_borrow().unwrap().try_as_concrete_bool().unwrap());
    }

    #[test]
    #[allow(unused_must_use)]
    fn test_symbolic_eq() {
        simple_logger::SimpleLogger::new().env().init();
        let scfia = Scfia::new();
        let s1 = scfia.new_bv_concrete(42, 32, &mut None);
        let s2 = scfia.new_bv_symbol(32, &mut None);
        let eq = scfia.new_bool_eq(s1, s2.clone(), &mut None);
        eq.try_borrow_mut().unwrap().assert();
        let mut candidates = vec![42];
        s2.try_borrow().unwrap().monomorphize(&mut candidates);
        assert_eq!(candidates.len(), 1)
    }
}
