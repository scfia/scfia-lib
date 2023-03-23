use std::fmt::Debug;

use super::active_value::{ActiveValue, ActiveValueWeak};

#[derive(Debug)]
pub struct BVSliceExpression {
    pub s1: ActiveValue,
    pub width: u32,
    pub high: u32,
    pub low: u32,
}

pub struct RetiredBVSliceExpression {
    pub s1: ActiveValueWeak,
    pub width: u32,
    pub high: u32,
    pub low: u32,
}

impl Debug for RetiredBVSliceExpression {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str("RetiredBVSliceExpression)")
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
        let slice = scfia.new_bv_slice(s1, 4, 1);
        assert_eq!(slice.try_borrow().unwrap().try_as_concrete_bv().unwrap(), 0b1001);
    }
    //TODO test
}
