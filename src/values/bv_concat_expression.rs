use std::fmt::Debug;

use super::active_value::{ActiveValue, ActiveValueWeak};

#[derive(Debug)]
pub struct BVConcatExpression {
    pub s1: ActiveValue,
    pub s2: ActiveValue,
    pub width: u32,
}

pub struct RetiredBVConcatExpression {
    pub s1: ActiveValueWeak,
    pub s2: ActiveValueWeak,
    pub width: u32,
}

impl Debug for RetiredBVConcatExpression {
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
        let s1 = scfia.new_bv_concrete(1, 32);
        let s2 = scfia.new_bv_concrete(2, 32);
        let concat = scfia.new_bv_concat(s1, s2, 64);
        assert_eq!(concat.try_borrow().unwrap().try_as_concrete_bv().unwrap(), 0x100000002);

        let s1 = scfia.new_bv_concrete(0b01, 2);
        let s2 = scfia.new_bv_concrete(0b10, 2);
        let concat = scfia.new_bv_concat(s1, s2, 4);
        assert_eq!(concat.try_borrow().unwrap().try_as_concrete_bv().unwrap(), 0b0110);
    }

    #[test]
    #[allow(unused_must_use)]
    fn test_symbol_concat() {
        simple_logger::SimpleLogger::new().env().init();
        let scfia = Scfia::new();
        let s1 = scfia.new_bv_concrete(0, 32);
        let s2 = scfia.new_bv_symbol(32);
        let _ = scfia.new_bv_concat(s1, s2, 32);
        //TODO better test
    }
}
