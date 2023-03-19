use std::fmt::Debug;

use super::active_value::ActiveValue;

#[derive(Debug)]
pub struct BVSignExtendExpression {
    pub s1: ActiveValue,
    pub width: u32,
}

//TODO test
