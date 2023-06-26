use std::{cell::RefCell, fmt::Debug, marker::PhantomData, rc::Weak};

use crate::ScfiaComposition;

use super::{active_value::{ActiveValue, ActiveValueExpression}, retired_value::ParentWeakReference};

#[derive(Debug)]
pub struct BVSllExpression<SC: ScfiaComposition> {
    pub s1: ActiveValue<SC>,
    pub s2: ActiveValue<SC>,
    pub width: u32,
}

#[derive(Debug)]
pub struct RetiredBVSllExpression<SC: ScfiaComposition> {
    pub s1: ParentWeakReference<SC>,
    pub s2: ParentWeakReference<SC>,
    pub width: u32,
    pub phantom: PhantomData<SC>,
}
