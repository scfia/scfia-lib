use std::fmt::Debug;

use super::active_value::{ActiveValue, ActiveValueWeak};

#[derive(Debug)]
pub struct BVAndExpression {
    pub s1: ActiveValue,
    pub s2: ActiveValue,
    pub width: u32,
}

pub struct RetiredBVAndExpression {
    pub s1: ActiveValueWeak,
    pub s2: ActiveValueWeak,
    pub width: u32,
}

impl Debug for RetiredBVAndExpression {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str("RetiredBVAndExpression)")
    }
}

#[cfg(test)]
mod tests {
    use crate::scfia::Scfia;

    #[test]
    #[allow(unused_must_use)]
    fn test_concrete_and() {
        simple_logger::SimpleLogger::new().env().init();
        let scfia = Scfia::new();
        let s1 = scfia.new_bv_concrete(1, 32);
        let s2 = scfia.new_bv_concrete(0xffffff, 32);
        let add = scfia.new_bv_and(s1, s2, 32);
        assert_eq!(add.try_borrow().unwrap().try_as_concrete_bv().unwrap(), 1);
    }

    #[test]
    #[allow(unused_must_use)]
    fn test_symbol_and() {
        simple_logger::SimpleLogger::new().env().init();
        let scfia = Scfia::new();
        let s1 = scfia.new_bv_concrete(0, 32);
        let s2 = scfia.new_bv_symbol(32);
        let and = scfia.new_bv_and(s1, s2, 32);
        let mut candidates = vec![0];
        and.try_borrow().unwrap().monomorphize(&mut candidates);
        assert_eq!(candidates.len(), 1)
    }
}
