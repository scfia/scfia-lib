use std::fmt::Debug;

use crate::ScfiaComposition;

use super::active_value::{ActiveValue, ActiveValueWeak};

#[derive(Debug)]
pub struct BVAndExpression<SC: ScfiaComposition> {
    pub s1: ActiveValue<SC>,
    pub s2: ActiveValue<SC>,
    pub width: u32,
}

pub struct RetiredBVAndExpression<SC: ScfiaComposition> {
    pub s1: ActiveValueWeak<SC>,
    pub s2: ActiveValueWeak<SC>,
    pub width: u32,
}

impl<SC: ScfiaComposition> Debug for RetiredBVAndExpression<SC> {
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
        let s1 = scfia.new_bv_concrete(1, 32, &mut None);
        let s2 = scfia.new_bv_concrete(0xffffff, 32, &mut None);
        let add = scfia.new_bv_and(s1, s2, 32, &mut None);
        assert_eq!(add.try_borrow().unwrap().try_as_concrete_bv().unwrap(), 1);
    }

    #[test]
    #[allow(unused_must_use)]
    fn test_symbol_and() {
        simple_logger::SimpleLogger::new().env().init();
        let scfia = Scfia::new();
        let s1 = scfia.new_bv_concrete(0, 32, &mut None);
        let s2 = scfia.new_bv_symbol(32, &mut None);
        let and = scfia.new_bv_and(s1, s2, 32, &mut None);
        let mut candidates = vec![0];
        and.try_borrow().unwrap().monomorphize(&mut candidates);
        assert_eq!(candidates.len(), 1)
    }
}
