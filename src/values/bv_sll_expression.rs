use std::fmt::Debug;

use super::active_value::{ActiveValue, ActiveValueWeak};

#[derive(Debug)]
pub struct BVSllExpression {
    pub s1: ActiveValue,
    pub s2: ActiveValue,
    pub width: u32,
    pub shamt_width: u32,
}

pub struct RetiredBVSllExpression {
    pub s1: ActiveValueWeak,
    pub s2: ActiveValueWeak,
    pub width: u32,
    pub shamt: u32,
}

impl Debug for RetiredBVSllExpression {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str("RetiredBVSllExpression)")
    }
}

#[cfg(test)]
mod tests {
    use crate::scfia::Scfia;

    #[test]
    #[allow(unused_must_use)]
    fn test_concrete() {
        simple_logger::SimpleLogger::new().env().init();
        let scfia = Scfia::new();
        let s1 = scfia.new_bv_concrete(0b110011, 6);
        let s2 = scfia.new_bv_concrete(1, 6);
        let sll = scfia.new_bv_sll(s1, s2, 6, 6);
        assert_eq!(sll.try_borrow().unwrap().try_as_concrete_bv().unwrap(), 0b100110);
    }
    //TODO better test
}
