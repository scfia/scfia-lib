use std::fmt::Debug;

use super::active_value::ActiveValueWeak;

pub struct RetiredBVSliceExpression {
    pub s1: ActiveValueWeak,
    pub width: u32,
    pub high: u32,
    pub low: u32,
}

impl Debug for RetiredBVSliceExpression {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str("RetiredBVSliceExpression)")
    }
}
