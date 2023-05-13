use std::fmt::Debug;

use crate::ScfiaComposition;

use super::active_value::{ActiveValue, ActiveValueWeak};

#[derive(Debug)]
pub struct BVUnsignedRemainderExpression<SC: ScfiaComposition> {
    pub s1: ActiveValue<SC>,
    pub s2: ActiveValue<SC>,
    pub width: u32,
}

pub struct RetiredBVUnsignedRemainderExpression<SC: ScfiaComposition> {
    pub s1: ActiveValueWeak<SC>,
    pub s2: ActiveValueWeak<SC>,
    pub width: u32,
}

impl<SC: ScfiaComposition> Debug for RetiredBVUnsignedRemainderExpression<SC> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str("RetiredBVXorExpression)")
    }
}

#[cfg(test)]
mod tests {
    use crate::{models::riscv::rv32i::RV32iScfiaComposition, scfia::Scfia};

    #[test]
    #[allow(unused_must_use)]
    fn test_concrete() {
        simple_logger::SimpleLogger::new().env().init();
        let scfia: Scfia<RV32iScfiaComposition> = Scfia::new();
        let s1 = scfia.new_bv_concrete(110, 32, &mut None);
        let s2 = scfia.new_bv_concrete(100, 30, &mut None);
        let unsigned_remainder = scfia.new_bv_unsigned_remainder(s1, s2, 32, &mut None);
        assert_eq!(unsigned_remainder.try_borrow().unwrap().try_as_concrete_bv().unwrap(), 10);
    }
    //TODO test
}
