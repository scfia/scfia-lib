use std::fmt::Debug;

use crate::ScfiaComposition;

use super::active_value::{ActiveValue, ActiveValueWeak};

#[derive(Debug)]
pub struct BVConcatExpression<SC: ScfiaComposition> {
    pub s1: ActiveValue<SC>,
    pub s2: ActiveValue<SC>,
    pub width: u32,
}

pub struct RetiredBVConcatExpression<SC: ScfiaComposition> {
    pub s1: ActiveValueWeak<SC>,
    pub s2: ActiveValueWeak<SC>,
    pub width: u32,
}

impl<SC: ScfiaComposition> Debug for RetiredBVConcatExpression<SC> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str("RetiredBVConcatExpression)")
    }
}

#[cfg(test)]
mod tests {
    use crate::scfia::Scfia;

    #[test]
    #[allow(unused_must_use)]
    fn test_concrete_concat() {
        simple_logger::SimpleLogger::new().env().init();
        let scfia = Scfia::new();
        let s1 = scfia.new_bv_concrete(1, 32, &mut None);
        let s2 = scfia.new_bv_concrete(2, 32, &mut None);
        let concat = scfia.new_bv_concat(s1, s2, 64, &mut None);
        assert_eq!(concat.try_borrow().unwrap().try_as_concrete_bv().unwrap(), 0x100000002);

        let s1 = scfia.new_bv_concrete(0b01, 2, &mut None);
        let s2 = scfia.new_bv_concrete(0b10, 2, &mut None);
        let concat = scfia.new_bv_concat(s1, s2, 4, &mut None);
        assert_eq!(concat.try_borrow().unwrap().try_as_concrete_bv().unwrap(), 0b0110);
    }

    #[test]
    #[allow(unused_must_use)]
    fn test_symbol_concat() {
        simple_logger::SimpleLogger::new().env().init();
        let scfia = Scfia::new();
        let s1 = scfia.new_bv_concrete(0, 32, &mut None);
        let s2 = scfia.new_bv_symbol(32, &mut None);
        let _ = scfia.new_bv_concat(s1, s2, 32, &mut None);
        //TODO better test
    }
}
