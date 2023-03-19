use std::fmt::Debug;

use super::active_value::ActiveValueWeak;

pub struct RetiredBVAndExpression {
    pub s1: ActiveValueWeak,
    pub s2: ActiveValueWeak,
    pub width: u32,
}

impl Debug for RetiredBVAndExpression {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str("RetiredBVAndExpression)")
    }
}
