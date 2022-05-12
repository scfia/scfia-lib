use std::fmt;

use z3_sys::Z3_ast;

use super::ast::ActiveAst;

pub trait Bool: ActiveAst {
}
