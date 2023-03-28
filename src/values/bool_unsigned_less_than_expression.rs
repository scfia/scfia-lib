use std::fmt::Debug;

use super::active_value::{ActiveValue, ActiveValueWeak};

#[derive(Debug)]
pub struct BoolUnsignedLessThanExpression {
    pub s1: ActiveValue,
    pub s2: ActiveValue,
    pub is_assert: bool,
}

pub struct RetiredBoolUnsignedLessThanExpression {
    pub s1: ActiveValueWeak,
    pub s2: ActiveValueWeak,
    pub is_assert: bool,
}

impl Debug for RetiredBoolUnsignedLessThanExpression {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str("RetiredBoolUnsignedLessThanExpression")
    }
}

#[cfg(test)]
mod tests {
    use crate::scfia::Scfia;

    #[test]
    #[allow(unused_must_use)]
    fn test_concrete_ult() {
        simple_logger::SimpleLogger::new().env().init();
        let scfia = Scfia::new();
        let s1 = scfia.new_bv_concrete(2, 32, &mut None);
        let s2 = scfia.new_bv_concrete(3, 32, &mut None);
        let slt = scfia.new_bool_unsigned_less_than(s1, s2, &mut None);
        assert!(slt.try_borrow().unwrap().try_as_concrete_bool().unwrap());
    }

    #[test]
    #[allow(unused_must_use)]
    fn test_symbolic_ult() {
        simple_logger::SimpleLogger::new().env().init();
        let scfia = Scfia::new();
        let s1 = scfia.new_bv_symbol(32, &mut None);
        let s2 = scfia.new_bv_concrete(1, 32, &mut None);
        let slt = scfia.new_bool_unsigned_less_than(s1.clone(), s2, &mut None);
        slt.try_borrow_mut().unwrap().assert();
        let mut candidates = vec![0];
        s1.try_borrow().unwrap().monomorphize(&mut candidates);
        assert_eq!(candidates.len(), 1)
    }
}
