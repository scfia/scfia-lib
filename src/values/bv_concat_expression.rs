use std::fmt::Debug;

use super::active_value::ActiveValue;

#[derive(Debug)]
pub struct BVConcatExpression {
    pub s1: ActiveValue,
    pub s2: ActiveValue,
    pub width: u32,
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
        let concat = scfia.new_bv_concat(s1, s2, 32);
        assert_eq!(concat.try_borrow().unwrap().try_as_concrete_bv().unwrap(), 0x100000002);
    }

    #[test]
    #[allow(unused_must_use)]
    fn test_symbol_concat() {
        simple_logger::SimpleLogger::new().env().init();
        let scfia = Scfia::new();
        let s1 = scfia.new_bv_concrete(0, 32);
        let s2 = scfia.new_bv_symbol(32);
        let _ = scfia.new_bv_concat(s1, s2, 32);
        // TODO better test
    }
}
