use std::{fmt::Debug, marker::PhantomData, cell::RefCell, rc::Weak};

use crate::ScfiaComposition;

use super::active_value::{ActiveValue, ActiveValueWeak, ActiveValueInner};

#[derive(Debug)]
pub struct BVSignExtendExpression<SC: ScfiaComposition> {
    pub s1: ActiveValue<SC>,
    pub width: u32,
    pub input_width: u32,
}

#[derive(Debug)]
pub struct RetiredBVSignExtendExpression<SC: ScfiaComposition> {
    pub s1: Weak<RefCell<ActiveValueInner<SC>>>,
    pub s1_id: u64,
    pub width: u32,
    pub input_width: u32,
    pub phantom: PhantomData<SC>,
}

/*
#[cfg(test)]
mod tests {
    use log::LevelFilter;

    use crate::{models::riscv::rv32i::RV32iScfiaComposition, scfia::ScfiaOld};

    #[test]
    #[allow(unused_must_use)]
    fn test_concrete() {
        simple_logger::SimpleLogger::new().with_level(LevelFilter::Trace).env().init();
        let scfia: ScfiaOld<RV32iScfiaComposition> = ScfiaOld::new(None);
        let s1 = scfia.new_bv_concrete(-1i32 as u32 as u64, 32, &mut None);
        let sign_extend = scfia.new_bv_sign_extend(s1.clone(), 32, 64, &mut None);
        assert_eq!(sign_extend.try_borrow().unwrap().try_as_concrete_bv().unwrap(), u64::MAX);
    }
    //TODO better tests
}
*/
