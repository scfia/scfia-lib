use std::fmt::Debug;

use super::active_value::ActiveValue;

#[derive(Debug)]
pub struct BVSllExpression {
    pub s1: ActiveValue,
    pub s2: ActiveValue,
    pub width: u32,
    pub shamt_width: u32,
}
//TODO tests
