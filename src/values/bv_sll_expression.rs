use std::{cell::RefCell, fmt::Debug, marker::PhantomData, rc::{Weak, Rc}};

use crate::ScfiaComposition;

use super::{active_value::{ActiveValue, ActiveValueZ3}, retired_value::ParentWeakReference};

#[derive(Debug)]
pub struct BVSllExpression<SC: ScfiaComposition> {
    pub s1: Rc<RefCell<ActiveValueZ3<SC>>>,
    pub s2: Rc<RefCell<ActiveValueZ3<SC>>>,
    pub width: u32,
}

#[derive(Debug)]
pub struct RetiredBVSllExpression<SC: ScfiaComposition> {
    pub s1: ParentWeakReference<SC>,
    pub s2: ParentWeakReference<SC>,
    pub width: u32,
    pub phantom: PhantomData<SC>,
}
