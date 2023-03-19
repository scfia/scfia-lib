use std::fmt::Debug;

use super::active_value::ActiveValue;

#[derive(Debug)]
pub struct BVOrExpression {
    pub s1: ActiveValue,
    pub s2: ActiveValue,
    pub width: u32,
}

#[cfg(test)]
mod tests {
    use crate::scfia::Scfia;

    #[test]
    #[allow(unused_must_use)]
    fn test_concrete_or() {
        simple_logger::SimpleLogger::new().env().init();
        let scfia = Scfia::new();
        let s1 = scfia.new_bv_concrete(1, 32);
        let s2 = scfia.new_bv_concrete(2, 32);
        let or = scfia.new_bv_or(s1, s2, 32);
        assert_eq!(or.try_borrow().unwrap().try_as_concrete_bv().unwrap(), 3);
    }

    #[test]
    #[allow(unused_must_use)]
    fn test_symbol_or() {
        simple_logger::SimpleLogger::new().env().init();
        let scfia = Scfia::new();
        let s1 = scfia.new_bv_concrete(u32::MAX as u64 - 1, 32);
        let s2 = scfia.new_bv_symbol(32);
        let or = scfia.new_bv_or(s1, s2, 32);
        let mut candidates = vec![u32::MAX as u64 - 1, u32::MAX as u64];
        or.try_borrow().unwrap().monomorphize(&mut candidates);
        assert_eq!(candidates.len(), 2)
    }
}
