use std::fmt::Debug;

use super::active_value::{ActiveValue, ActiveValueWeak};

#[derive(Debug)]
pub struct BoolSignedLessThanExpression {
    pub s1: ActiveValue,
    pub s2: ActiveValue,
    pub is_assert: bool,
}

pub struct RetiredBoolSignedLessThanExpression {
    pub s1: ActiveValueWeak,
    pub s2: ActiveValueWeak,
    pub is_assert: bool,
}

impl Debug for RetiredBoolSignedLessThanExpression {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str("RetiredBoolSignedLessThanExpression")
    }
}

#[cfg(test)]
mod tests {
    use crate::scfia::Scfia;

    #[test]
    #[allow(unused_must_use)]
    fn test_concrete_slt() {
        simple_logger::SimpleLogger::new().env().init();
        let scfia = Scfia::new();
        let s1 = scfia.new_bv_concrete(2, 32);
        let s2 = scfia.new_bv_concrete(3, 32);
        let slt = scfia.new_bool_signed_less_than(s1, s2);
        assert!(slt.try_borrow().unwrap().try_as_concrete_bool().unwrap());

        let scfia = Scfia::new();
        let s1 = scfia.new_bv_concrete(-2i32 as u32 as u64, 32);
        let s2 = scfia.new_bv_concrete(-1i32 as u32 as u64, 32);
        let slt = scfia.new_bool_signed_less_than(s1, s2);
        assert!(slt.try_borrow().unwrap().try_as_concrete_bool().unwrap());
    }

    #[test]
    #[allow(unused_must_use)]
    fn test_symbolic_slt() {
        simple_logger::SimpleLogger::new().env().init();
        let scfia = Scfia::new();
        let s1 = scfia.new_bv_symbol(32);
        let s2 = scfia.new_bv_concrete((i32::MIN as u64) + 1, 32);
        let slt = scfia.new_bool_signed_less_than(s1.clone(), s2);
        slt.try_borrow_mut().unwrap().assert();
        let mut candidates = vec![i32::MIN as u64];
        s1.try_borrow().unwrap().monomorphize(&mut candidates);
        assert_eq!(candidates.len(), 1)
    }
}