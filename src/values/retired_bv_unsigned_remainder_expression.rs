use std::fmt::Debug;

use super::active_value::ActiveValueWeak;

pub struct RetiredBVUnsignedRemainderExpression {
    pub s1: ActiveValueWeak,
    pub s2: ActiveValueWeak,
    pub width: u32,
}

impl Debug for RetiredBVUnsignedRemainderExpression {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str("RetiredBVXorExpression)")
    }
}
