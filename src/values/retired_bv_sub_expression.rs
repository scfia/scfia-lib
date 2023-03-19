use std::fmt::Debug;

use super::active_value::ActiveValueWeak;

pub struct RetiredBVSubExpression {
    pub s1: ActiveValueWeak,
    pub s2: ActiveValueWeak,
    pub width: u32,
}

impl Debug for RetiredBVSubExpression {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str("RetiredBVSubExpression)")
    }
}
