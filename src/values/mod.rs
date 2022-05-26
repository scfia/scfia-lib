use std::{cell::RefCell, rc::{Rc, Weak}};
use std::collections::HashMap;
use z3_sys::Z3_solver_assert;

use crate::ScfiaStdlib;

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

    pub fn clone_to_stdlib(
        &self,
        cloned_active_values: &mut HashMap<u64, Rc<RefCell<ActiveValue>>>,
        cloned_retired_values: &mut HashMap<u64, Rc<RefCell<RetiredValue>>>,
        cloned_stdlib: &mut ScfiaStdlib
    ) -> Rc<RefCell<ActiveValue>> {
        if let Some(cloned_active_value) = cloned_active_values.get(&self.get_id()) {
            return cloned_active_value.clone();
        }

        let id = self.get_id();
        match self {
            ActiveValue::BitvectorConcrete(_) => {},
            x => println!("clone_to_stdlib ActiveValue {:?}", &x),
        }
        if let Some(translated_value) = cloned_active_values.get(&id) {
            return translated_value.clone();
        }

        match self {
            ActiveValue::BitvectorConcrete(e) => e.clone_to_stdlib(cloned_active_values, cloned_retired_values, cloned_stdlib).into(),
            ActiveValue::BitvectorSymbol(e) => e.clone_to_stdlib(cloned_active_values, cloned_retired_values, cloned_stdlib),
            ActiveValue::BoolEqExpression(e) => e.clone_to_stdlib(cloned_active_values, cloned_retired_values, cloned_stdlib),
            ActiveValue::BoolNEqExpression(_) => unimplemented!(),
            ActiveValue::BitvectorAddExpression(_) => unimplemented!(),
            ActiveValue::BitvectorConcatExpression(_) => unimplemented!(),
            ActiveValue::BitvectorOrExpression(_) => unimplemented!(),
            ActiveValue::BitvectorSignExtendExpression(_) => unimplemented!(),
            ActiveValue::BitvectorSliceExpression(_) => unimplemented!(),
        }
    }

    pub fn assert(&mut self, stdlib: &mut ScfiaStdlib) {
        match self {
            ActiveValue::BoolEqExpression(x) => x.assert(stdlib),
            ActiveValue::BoolNEqExpression(x) => x.assert(stdlib),
            x => panic!("{:?}", &x),
        }
    }
}

impl RetiredValue {
    pub fn clone_to_stdlib(
        &self,
        cloned_active_values: &mut HashMap<u64, Rc<RefCell<ActiveValue>>>,
        cloned_retired_values: &mut HashMap<u64, Rc<RefCell<RetiredValue>>>,
        cloned_stdlib: &mut ScfiaStdlib
    ) -> Rc<RefCell<RetiredValue>> {
        if let Some(cloned_retired_value) = cloned_retired_values.get(&self.get_id()) {
            return cloned_retired_value.clone();
        }

        match self {
            RetiredValue::RetiredBitvectorConcrete(_) => {},
            x => println!("clone_to_stdlib RetiredValue {:?}", &x),
        }

        match self {
            RetiredValue::RetiredBitvectorConcrete(_) => todo!(),
            RetiredValue::RetiredBitvectorSymbol(_) => todo!(),
            RetiredValue::RetiredBoolEqExpression(_) => todo!(),
            RetiredValue::RetiredBoolNEqExpression(x) => x.clone_to_stdlib(cloned_active_values, cloned_retired_values, cloned_stdlib),
            RetiredValue::RetiredBitvectorAddExpression(_) => todo!(),
            RetiredValue::RetiredBitvectorConcatExpression(_) => todo!(),
            RetiredValue::RetiredBitvectorOrExpression(_) => todo!(),
            RetiredValue::RetiredBitvectorSignExtendExpression(_) => todo!(),
            RetiredValue::RetiredBitvectorSliceExpression(_) => todo!(),
        }
    }

    pub fn get_id(&self) -> u64 {
        match self {
            RetiredValue::RetiredBitvectorConcrete(e) => e.id,
            RetiredValue::RetiredBitvectorSymbol(e) => e.id,
            RetiredValue::RetiredBoolEqExpression(e) => e.id,
            RetiredValue::RetiredBoolNEqExpression(e) => e.id,
            RetiredValue::RetiredBitvectorAddExpression(e) => e.id,
            RetiredValue::RetiredBitvectorConcatExpression(e) => e.id,
            RetiredValue::RetiredBitvectorOrExpression(e) => e.id,
            RetiredValue::RetiredBitvectorSignExtendExpression(e) => e.id,
            RetiredValue::RetiredBitvectorSliceExpression(e) => e.id,
        }
    }

    pub fn get_z3_ast(&self) -> z3_sys::Z3_ast {
        match self {
            RetiredValue::RetiredBitvectorConcrete(e) => e.z3_ast,
            RetiredValue::RetiredBitvectorSymbol(e) => e.z3_ast,
            RetiredValue::RetiredBoolEqExpression(e) => e.z3_ast,
            RetiredValue::RetiredBoolNEqExpression(e) => e.z3_ast,
            RetiredValue::RetiredBitvectorAddExpression(e) => e.z3_ast,
            RetiredValue::RetiredBitvectorConcatExpression(e) => e.z3_ast,
            RetiredValue::RetiredBitvectorOrExpression(e) => e.z3_ast,
            RetiredValue::RetiredBitvectorSignExtendExpression(e) => e.z3_ast,
            RetiredValue::RetiredBitvectorSliceExpression(e) => e.z3_ast,
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

impl From<RetiredValue> for Rc<RefCell<RetiredValue>> {
    fn from(s: RetiredValue) -> Self {
        Rc::new(RefCell::new(s))
    }
}

impl From<RetiredBitvectorConcrete> for Rc<RefCell<RetiredValue>> {
    fn from(s: RetiredBitvectorConcrete) -> Self {
        Rc::new(RefCell::new(RetiredValue::RetiredBitvectorConcrete(s)))
    }
}

impl From<RetiredBitvectorSymbol> for Rc<RefCell<RetiredValue>> {
    fn from(s: RetiredBitvectorSymbol) -> Self {
        Rc::new(RefCell::new(RetiredValue::RetiredBitvectorSymbol(s)))
    }
}

impl From<RetiredBoolEqExpression> for Rc<RefCell<RetiredValue>> {
    fn from(s: RetiredBoolEqExpression) -> Self {
        Rc::new(RefCell::new(RetiredValue::RetiredBoolEqExpression(s)))
    }
}

impl From<RetiredBoolNEqExpression> for Rc<RefCell<RetiredValue>> {
    fn from(s: RetiredBoolNEqExpression) -> Self {
        Rc::new(RefCell::new(RetiredValue::RetiredBoolNEqExpression(s)))
    }
}

impl From<RetiredBVAddExpression> for Rc<RefCell<RetiredValue>> {
    fn from(s: RetiredBVAddExpression) -> Self {
        Rc::new(RefCell::new(RetiredValue::RetiredBitvectorAddExpression(s)))
    }
}

impl From<RetiredBVConcatExpression> for Rc<RefCell<RetiredValue>> {
    fn from(s: RetiredBVConcatExpression) -> Self {
        Rc::new(RefCell::new(RetiredValue::RetiredBitvectorConcatExpression(s)))
    }
}

impl From<RetiredBVOrExpression> for Rc<RefCell<RetiredValue>> {
    fn from(s: RetiredBVOrExpression) -> Self {
        Rc::new(RefCell::new(RetiredValue::RetiredBitvectorOrExpression(s)))
    }
}

impl From<RetiredBVSignExtendExpression> for Rc<RefCell<RetiredValue>> {
    fn from(s: RetiredBVSignExtendExpression) -> Self {
        Rc::new(RefCell::new(RetiredValue::RetiredBitvectorSignExtendExpression(s)))
    }
}

impl From<RetiredBVSliceExpression> for Rc<RefCell<RetiredValue>> {
    fn from(s: RetiredBVSliceExpression) -> Self {
        Rc::new(RefCell::new(RetiredValue::RetiredBitvectorSliceExpression(s)))
    }
}
