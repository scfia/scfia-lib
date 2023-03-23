use std::fmt::Debug;

use super::active_value::{ActiveValue, ActiveValueWeak};

#[derive(Debug)]
pub struct BVSignExtendExpression {
    pub s1: ActiveValue,
    pub width: u32,
}

pub struct RetiredBVSignExtendExpression {
    pub s1: ActiveValueWeak,
    pub width: u32,
}

impl Debug for RetiredBVSignExtendExpression {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str("RetiredBVSignExtendExpression)")
    }
}

#[cfg(test)]
mod tests {
    use log::LevelFilter;

    use crate::scfia::Scfia;

    #[test]
    #[allow(unused_must_use)]
    fn test_concrete() {
        simple_logger::SimpleLogger::new().with_level(LevelFilter::Trace).env().init();
        let scfia = Scfia::new();
        let s1 = scfia.new_bv_concrete(-1i32 as u32 as u64, 32);
        let sign_extend = scfia.new_bv_sign_extend(s1.clone(), 32, 64);
        assert_eq!(sign_extend.try_borrow().unwrap().try_as_concrete_bv().unwrap(), u64::MAX);
    }
    //TODO better tests
}
