use std::fmt::Debug;

use super::active_value::ActiveValue;

#[derive(Debug)]
pub struct BVSubExpression {
    pub s1: ActiveValue,
    pub s2: ActiveValue,
    pub width: u32,
}
//TODO tests
