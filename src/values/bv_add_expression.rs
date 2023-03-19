use std::{cell::RefCell, collections::BTreeMap, fmt::Debug, rc::Rc};

use z3_sys::{Z3_ast, Z3_context, Z3_inc_ref, Z3_mk_bvadd};

use crate::scfia::Scfia;

use super::active_value::{ActiveExpression, ActiveValue, ActiveValueInner};

pub struct BVAddExpression {
    pub s1: ActiveValue,
    pub s2: ActiveValue,
    pub width: u32,
}

impl Debug for BVAddExpression {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str("(")?;
        self.s1.try_borrow().unwrap().fmt(f)?;
        f.write_str(" + ")?;
        self.s2.try_borrow().unwrap().fmt(f)?;
        f.write_str(")")
    }
}

#[cfg(test)]
mod tests {
    use crate::scfia::Scfia;

    #[test]
    fn test_concrete_add() {
        simple_logger::SimpleLogger::new().env().init().unwrap();
        let scfia = Scfia::new();
        let s1 = scfia.new_bv_concrete(1, 32);
        let s2 = scfia.new_bv_concrete(2, 32);
        let add = scfia.new_bv_add(s1, s2, 32);
        assert_eq!(add.try_borrow().unwrap().try_as_concrete().unwrap(), 3);

        let s1 = scfia.new_bv_concrete(1, 32);
        let s2 = scfia.new_bv_concrete(u32::MAX as u64, 32);
        let add = scfia.new_bv_add(s1, s2, 32);
        assert_eq!(add.try_borrow().unwrap().try_as_concrete().unwrap(), 0);
    }
}
