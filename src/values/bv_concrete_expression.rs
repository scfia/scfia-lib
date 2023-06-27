use std::{fmt::Debug, marker::PhantomData};

use crate::ScfiaComposition;

#[derive(Debug)]
pub struct BVConcreteExpression<SC: ScfiaComposition> {
    pub value: u64,
    pub width: u32,
    phantom: PhantomData<SC>,
}

#[derive(Debug)]
pub struct RetiredBVConcreteExpression {
    pub value: u64,
    pub width: u32,
}
