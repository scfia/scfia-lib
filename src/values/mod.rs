use std::{cell::RefCell, rc::{Rc, Weak}};

use crate::{expressions::{bool_eq_expression::{BoolEqExpression, RetiredBoolEqExpression}, bool_neq_expression::{BoolNEqExpression, RetiredBoolNEqExpression}, bv_add_expression::{BVAddExpression, RetiredBVAddExpression}, bv_concat_expression::{BVConcatExpression, RetiredBVConcatExpression}, bv_or_expression::{BVOrExpression, RetiredBVOrExpression}, bv_sign_extend_expression::{BVSignExtendExpression, RetiredBVSignExtendExpression}, bv_slice_expression::{BVSliceExpression, RetiredBVSliceExpression}}, traits::ast::Ast};

use self::{bit_vector_concrete::{BitVectorConcrete, RetiredBitvectorConcrete}, bit_vector_symbol::{BitVectorSymbol, RetiredBitvectorSymbol}};

pub mod bit_vector_concrete;
pub mod bit_vector_symbol;

#[derive(Debug)]
pub enum Value {
    Active(ActiveValue),
    Ref(RetiredValue),
}

#[derive(Debug)]
pub enum ActiveValue {
    BitvectorConcrete(BitVectorConcrete),
    BitvectorSymbol(BitVectorSymbol),
    BoolEqExpression(BoolEqExpression),
    BoolNEqExpression(BoolNEqExpression),
    BitvectorAddExpression(BVAddExpression),
    BitvectorConcatExpression(BVConcatExpression),
    BitvectorOrExpression(BVOrExpression),
    BitvectorSignExtendExpression(BVSignExtendExpression),
    BitvectorSliceExpression(BVSliceExpression),
}

#[derive(Debug)]
pub enum RetiredValue {
    RetiredBitvectorConcrete(RetiredBitvectorConcrete),
    RetiredBitvectorSymbol(RetiredBitvectorSymbol),
    RetiredBoolEqExpression(RetiredBoolEqExpression),
    RetiredBoolNEqExpression(RetiredBoolNEqExpression),
    RetiredBitvectorAddExpression(RetiredBVAddExpression),
    RetiredBitvectorConcatExpression(RetiredBVConcatExpression),
    RetiredBitvectorOrExpression(RetiredBVOrExpression),
    RetiredBitvectorSignExtendExpression(RetiredBVSignExtendExpression),
    RetiredBitvectorSliceExpression(RetiredBVSliceExpression),
}

impl ActiveValue {
    pub fn as_concrete_bitvector(&self) -> &BitVectorConcrete {
        match self {
            ActiveValue::BitvectorConcrete(e) => e,
            _ => panic!()
        }
    }
}

impl Ast for ActiveValue {
    fn get_id(&self) -> u64 {
        match self {
            ActiveValue::BitvectorConcrete(e) => e.id,
            ActiveValue::BitvectorSymbol(e) => e.id,
            ActiveValue::BoolEqExpression(e) => e.id,
            ActiveValue::BoolNEqExpression(e) => e.id,
            ActiveValue::BitvectorAddExpression(e) => e.id,
            ActiveValue::BitvectorConcatExpression(e) => e.id,
            ActiveValue::BitvectorOrExpression(e) => e.id,
            ActiveValue::BitvectorSignExtendExpression(e) => e.id,
            ActiveValue::BitvectorSliceExpression(e) => e.id,
        }
    }

    fn get_z3_ast(&self) -> z3_sys::Z3_ast {
        match self {
            ActiveValue::BitvectorConcrete(e) => e.z3_ast,
            ActiveValue::BitvectorSymbol(e) => e.z3_ast,
            ActiveValue::BoolEqExpression(e) => e.z3_ast,
            ActiveValue::BoolNEqExpression(e) => e.z3_ast,
            ActiveValue::BitvectorAddExpression(e) => e.z3_ast,
            ActiveValue::BitvectorConcatExpression(e) => e.z3_ast,
            ActiveValue::BitvectorOrExpression(e) => e.z3_ast,
            ActiveValue::BitvectorSignExtendExpression(e) => e.z3_ast,
            ActiveValue::BitvectorSliceExpression(e) => e.z3_ast,
        }
    }

    fn inherit(&mut self, ast: Rc<RefCell<RetiredValue>>) {
        match self {
            ActiveValue::BitvectorConcrete(e) => e.inherited_asts.push(ast),
            ActiveValue::BitvectorSymbol(e) => e.inherited_asts.push(ast),
            ActiveValue::BoolEqExpression(e) => e.inherited_asts.push(ast),
            ActiveValue::BoolNEqExpression(e) => e.inherited_asts.push(ast),
            ActiveValue::BitvectorAddExpression(e) => e.inherited_asts.push(ast),
            ActiveValue::BitvectorConcatExpression(e) => e.inherited_asts.push(ast),
            ActiveValue::BitvectorOrExpression(e) => e.inherited_asts.push(ast),
            ActiveValue::BitvectorSignExtendExpression(e) => e.inherited_asts.push(ast),
            ActiveValue::BitvectorSliceExpression(e) => e.inherited_asts.push(ast),
        }
    }

    fn forget(&mut self, id: u64) {
        match self {
            ActiveValue::BitvectorConcrete(e) => { e.discovered_asts.remove(&id); },
            ActiveValue::BitvectorSymbol(e) => { e.discovered_asts.remove(&id); },
            ActiveValue::BoolEqExpression(e) => { e.discovered_asts.remove(&id); },
            ActiveValue::BoolNEqExpression(e) => { e.discovered_asts.remove(&id); },
            ActiveValue::BitvectorAddExpression(e) => { e.discovered_asts.remove(&id); },
            ActiveValue::BitvectorConcatExpression(e) => { e.discovered_asts.remove(&id); },
            ActiveValue::BitvectorOrExpression(e) => { e.discovered_asts.remove(&id); },
            ActiveValue::BitvectorSignExtendExpression(e) => { e.discovered_asts.remove(&id); },
            ActiveValue::BitvectorSliceExpression(e) => { e.discovered_asts.remove(&id); },
        }
    }

    fn get_cloned(&self, clone_map: &mut std::collections::HashMap<u64, Rc<RefCell<Value>>>, cloned_stdlib: &mut crate::ScfiaStdlib) -> Rc<RefCell<dyn Ast>> {
        todo!()
    }

    fn discover(&mut self, ast_id: u64, ast: Weak<RefCell<ActiveValue>>) {
        match self {
            ActiveValue::BitvectorConcrete(e) => { e.discovered_asts.insert(ast_id, ast); },
            ActiveValue::BitvectorSymbol(e) => { e.discovered_asts.insert(ast_id, ast); },
            ActiveValue::BoolEqExpression(e) => { e.discovered_asts.insert(ast_id, ast); },
            ActiveValue::BoolNEqExpression(e) => { e.discovered_asts.insert(ast_id, ast); },
            ActiveValue::BitvectorAddExpression(e) => { e.discovered_asts.insert(ast_id, ast); },
            ActiveValue::BitvectorConcatExpression(e) => { e.discovered_asts.insert(ast_id, ast); },
            ActiveValue::BitvectorOrExpression(e) => { e.discovered_asts.insert(ast_id, ast); },
            ActiveValue::BitvectorSignExtendExpression(e) => { e.discovered_asts.insert(ast_id, ast); },
            ActiveValue::BitvectorSliceExpression(e) => { e.discovered_asts.insert(ast_id, ast); },
        }
    }
}

impl From<ActiveValue> for Rc<RefCell<ActiveValue>> {
    fn from(s: ActiveValue) -> Self {
        Rc::new(RefCell::new(s))
    }
}

impl From<BitVectorConcrete> for Rc<RefCell<ActiveValue>> {
    fn from(s: BitVectorConcrete) -> Self {
        Rc::new(RefCell::new(ActiveValue::BitvectorConcrete(s)))
    }
}

impl From<BitVectorSymbol> for Rc<RefCell<ActiveValue>> {
    fn from(s: BitVectorSymbol) -> Self {
        Rc::new(RefCell::new(ActiveValue::BitvectorSymbol(s)))
    }
}

impl From<BoolEqExpression> for Rc<RefCell<ActiveValue>> {
    fn from(s: BoolEqExpression) -> Self {
        Rc::new(RefCell::new(ActiveValue::BoolEqExpression(s)))
    }
}

impl From<BoolNEqExpression> for Rc<RefCell<ActiveValue>> {
    fn from(s: BoolNEqExpression) -> Self {
        Rc::new(RefCell::new(ActiveValue::BoolNEqExpression(s)))
    }
}

impl From<BVAddExpression> for Rc<RefCell<ActiveValue>> {
    fn from(s: BVAddExpression) -> Self {
        Rc::new(RefCell::new(ActiveValue::BitvectorAddExpression(s)))
    }
}

impl From<BVConcatExpression> for Rc<RefCell<ActiveValue>> {
    fn from(s: BVConcatExpression) -> Self {
        Rc::new(RefCell::new(ActiveValue::BitvectorConcatExpression(s)))
    }
}

impl From<BVOrExpression> for Rc<RefCell<ActiveValue>> {
    fn from(s: BVOrExpression) -> Self {
        Rc::new(RefCell::new(ActiveValue::BitvectorOrExpression(s)))
    }
}

impl From<BVSignExtendExpression> for Rc<RefCell<ActiveValue>> {
    fn from(s: BVSignExtendExpression) -> Self {
        Rc::new(RefCell::new(ActiveValue::BitvectorSignExtendExpression(s)))
    }
}

impl From<BVSliceExpression> for Rc<RefCell<ActiveValue>> {
    fn from(s: BVSliceExpression) -> Self {
        Rc::new(RefCell::new(ActiveValue::BitvectorSliceExpression(s)))
    }
}
