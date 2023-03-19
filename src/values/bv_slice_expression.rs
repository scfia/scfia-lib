use std::fmt::Debug;

use super::active_value::ActiveValue;

#[derive(Debug)]
pub struct BVSliceExpression {
    pub s1: ActiveValue,
    pub width: u32,
    pub high: u32,
    pub low: u32,
}
//TODO test
