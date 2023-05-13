use std::fmt::Debug;

use crate::ScfiaComposition;

use super::active_value::{ActiveValue, ActiveValueWeak};

#[derive(Debug)]
pub struct BVSllExpression<SC: ScfiaComposition> {
    pub s1: ActiveValue<SC>,
    pub s2: ActiveValue<SC>,
    pub width: u32,
    pub shamt_width: u32,
}

pub struct RetiredBVSllExpression<SC: ScfiaComposition> {
    pub s1: ActiveValueWeak<SC>,
    pub s2: ActiveValueWeak<SC>,
    pub width: u32,
    pub shamt: u32,
}

impl<SC: ScfiaComposition> Debug for RetiredBVSllExpression<SC> {
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
        let s1 = scfia.new_bv_concrete(0b110011, 6, &mut None);
        let s2 = scfia.new_bv_concrete(1, 6, &mut None);
        let sll = scfia.new_bv_sll(s1, s2, 6, 6, &mut None);
        assert_eq!(sll.try_borrow().unwrap().try_as_concrete_bv().unwrap(), 0b100110);
    }
    //TODO better test
}
