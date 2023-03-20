use std::fmt::Debug;

use super::active_value::{ActiveValue, ActiveValueWeak};

#[derive(Debug)]
pub struct BVSubExpression {
    pub s1: ActiveValue,
    pub s2: ActiveValue,
    pub width: u32,
}

pub struct RetiredBVSubExpression {
    pub s1: ActiveValueWeak,
    pub s2: ActiveValueWeak,
    pub width: u32,
}

impl Debug for RetiredBVSubExpression {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str("RetiredBVSubExpression)")
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
        let s1 = scfia.new_bv_concrete(42, 32);
        let s2 = scfia.new_bv_concrete(41, 32);
        let sub = scfia.new_bv_sub(s1, s2, 32);
        assert_eq!(sub.try_borrow().unwrap().try_as_concrete_bv().unwrap(), 1);
    }
    //TODO test
}
