use std::fmt::Debug;

use super::active_value::{ActiveValue, ActiveValueWeak};

#[derive(Debug)]
pub struct BVUnsignedRemainderExpression {
    pub s1: ActiveValue,
    pub s2: ActiveValue,
    pub width: u32,
}

pub struct RetiredBVUnsignedRemainderExpression {
    pub s1: ActiveValueWeak,
    pub s2: ActiveValueWeak,
    pub width: u32,
}

impl Debug for RetiredBVUnsignedRemainderExpression {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str("RetiredBVXorExpression)")
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
        let s1 = scfia.new_bv_concrete(110, 32);
        let s2 = scfia.new_bv_concrete(100, 30);
        let unsigned_remainder = scfia.new_bv_unsigned_remainder(s1, s2, 32);
        assert_eq!(unsigned_remainder.try_borrow().unwrap().try_as_concrete_bv().unwrap(), 10);
    }
    //TODO test
}
