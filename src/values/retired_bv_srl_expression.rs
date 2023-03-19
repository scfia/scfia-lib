use std::fmt::Debug;

use super::active_value::ActiveValueWeak;

pub struct RetiredBVSrlExpression {
    pub s1: ActiveValueWeak,
    pub s2: ActiveValueWeak,
    pub width: u32,
    pub shamt: u32,
}

impl Debug for RetiredBVSrlExpression {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str("RetiredBVSrlExpression)")
    }
}
