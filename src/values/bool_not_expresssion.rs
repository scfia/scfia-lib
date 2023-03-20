use std::fmt::Debug;

use super::active_value::{ActiveValue, ActiveValueWeak};

pub struct BoolNotExpression {
    pub s1: ActiveValue,
    pub is_assert: bool,
}

pub struct RetiredBoolNotExpression {
    pub s1: ActiveValueWeak,
    pub is_assert: bool,
}

impl Debug for BoolNotExpression {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str("(!")?;
        self.s1.try_borrow().unwrap().fmt(f)?;
        f.write_str(")")
    }
}

impl Debug for RetiredBoolNotExpression {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str("(!x)")
    }
}

#[cfg(test)]
mod tests {
    use crate::scfia::Scfia;

    #[test]
    #[allow(unused_must_use)]
    fn test_concrete_not() {
        simple_logger::SimpleLogger::new().env().init();
        let scfia = Scfia::new();
        let s1 = scfia.new_bv_concrete(2, 32);
        let s2 = scfia.new_bv_concrete(2, 32);
        let eq = scfia.new_bool_eq(s1, s2);
        let not = scfia.new_bool_not(eq);
        assert!(!not.try_borrow().unwrap().try_as_concrete_bool().unwrap());
        assert!(!scfia.check_condition(not));

        let scfia = Scfia::new();
        let s1 = scfia.new_bv_concrete(1, 32);
        let s2 = scfia.new_bv_concrete(2, 32);
        let eq = scfia.new_bool_eq(s1, s2);
        let not = scfia.new_bool_not(eq);
        assert!(not.try_borrow().unwrap().try_as_concrete_bool().unwrap());
        assert!(scfia.check_condition(not));
    }

    #[test]
    #[allow(unused_must_use)]
    fn test_symbolic_not() {
        simple_logger::SimpleLogger::new().env().init();
        let scfia = Scfia::new();
        let s1 = scfia.new_bv_concrete(42, 32);
        let s2 = scfia.new_bv_symbol(32);
        let eq = scfia.new_bool_eq(s1, s2.clone());
        let not = scfia.new_bool_not(eq.clone());
        not.try_borrow_mut().unwrap().assert();
        // TODO better test
        assert!(scfia.check_condition(not));
        assert!(!scfia.check_condition(eq));
    }
}