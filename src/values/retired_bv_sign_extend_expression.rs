use std::fmt::Debug;

use super::active_value::ActiveValueWeak;

pub struct RetiredBVSignExtendExpression {
    pub s1: ActiveValueWeak,
    pub width: u32,
}

impl Debug for RetiredBVSignExtendExpression {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str("RetiredBVSignExtendExpression)")
    }
}
