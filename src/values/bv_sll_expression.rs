use std::{cell::RefCell, fmt::Debug, marker::PhantomData, rc::Weak};

use crate::ScfiaComposition;

use super::active_value::{ActiveValue, ActiveValueInner};

#[derive(Debug)]
pub struct BVSllExpression<SC: ScfiaComposition> {
    pub s1: ActiveValue<SC>,
    pub s2: ActiveValue<SC>,
    pub width: u32,
}

#[derive(Debug)]
pub struct RetiredBVSllExpression<SC: ScfiaComposition> {
    pub s1: Weak<RefCell<ActiveValueInner<SC>>>,
    pub s1_id: u64,
    pub s2: Weak<RefCell<ActiveValueInner<SC>>>,
    pub s2_id: u64,
    pub width: u32,
    pub phantom: PhantomData<SC>,
}
