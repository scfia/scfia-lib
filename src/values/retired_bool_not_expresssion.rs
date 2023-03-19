use std::fmt::Debug;

use super::active_value::ActiveValueWeak;

pub struct RetiredBoolNotExpression {
    pub s1: ActiveValueWeak,
    pub is_assert: bool,
}

impl Debug for RetiredBoolNotExpression {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str("(!x)")
    }
}
