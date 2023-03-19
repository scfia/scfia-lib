use std::fmt::Debug;

use super::active_value::ActiveValueWeak;

pub struct RetiredBoolEqExpression {
    pub s1: ActiveValueWeak,
    pub s2: ActiveValueWeak,
    pub is_assert: bool,
}

impl Debug for RetiredBoolEqExpression {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str("RetiredBoolEqExpression")
    }
}
