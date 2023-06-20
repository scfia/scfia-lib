use std::{
    cell::RefCell,
    fmt::Debug,
    rc::{Rc, Weak}, collections::BTreeMap, marker::PhantomData,
};

use log::{error, trace, debug};
use z3_sys::{Z3_ast, Z3_mk_true, Z3_mk_false, Z3_mk_bv_sort, Z3_mk_unsigned_int64, Z3_inc_ref, Z3_mk_eq, Z3_solver_assert, Z3_mk_not, Z3_mk_bvslt, Z3_mk_bvult, Z3_mk_bvadd, Z3_mk_bvand, Z3_mk_concat, Z3_mk_bvmul, Z3_mk_bvor, Z3_mk_sign_ext, Z3_mk_extract, Z3_mk_bvshl, Z3_mk_bvlshr, Z3_mk_bvsub, Z3_mk_fresh_const, Z3_string, Z3_mk_bvurem, Z3_mk_bvxor};

use crate::{scfia::{Scfia, PREFIX}, ScfiaComposition, z3_handle::Z3Ast};

use super::{
    bool_concrete::RetiredBoolConcrete, bool_eq_expression::RetiredBoolEqExpression, bool_not_expresssion::RetiredBoolNotExpression,
    bool_signed_less_than_expression::RetiredBoolSignedLessThanExpression, bool_unsigned_less_than_expression::RetiredBoolUnsignedLessThanExpression,
    bv_add_expression::RetiredBVAddExpression, bv_and_expression::RetiredBVAndExpression, bv_concat_expression::RetiredBVConcatExpression,
    bv_concrete::{RetiredBVConcrete}, bv_multiply_expression::RetiredBVMultiplyExpression, bv_or_expression::RetiredBVOrExpression,
    bv_sign_extend_expression::RetiredBVSignExtendExpression, bv_slice_expression::RetiredBVSliceExpression, bv_sll_expression::RetiredBVSllExpression,
    bv_srl_expression::RetiredBVSrlExpression, bv_sub_expression::RetiredBVSubExpression, bv_symbol::RetiredBVSymbol,
    bv_unsigned_remainder_expression::RetiredBVUnsignedRemainderExpression, bv_xor_expression::RetiredBVXorExpression, active_value::{ActiveValue, ActiveValueInner},
};

pub type RetiredValue<Model> = Rc<RefCell<RetiredValueInner<Model>>>;
pub type RetiredValueWeak<Model> = Weak<RefCell<RetiredValueInner<Model>>>;

pub struct RetiredValueInner<SC: ScfiaComposition> {
    pub id: u64,
    pub z3_ast: Z3Ast<SC>,
    pub expression: RetiredExpression<SC>,
    pub scfia: Weak<Scfia<SC>>,
}

pub enum RetiredExpression<SC: ScfiaComposition> {
    BoolConcrete(RetiredBoolConcrete),
    BoolEqExpression(RetiredBoolEqExpression<SC>),
    BoolNotExpression(RetiredBoolNotExpression<SC>),
    BoolSignedLessThanExpression(RetiredBoolSignedLessThanExpression<SC>),
    BoolUnsignedLessThanExpression(RetiredBoolUnsignedLessThanExpression<SC>),
    BVAddExpression(RetiredBVAddExpression<SC>),
    BVAndExpression(RetiredBVAndExpression<SC>),
    BVConcatExpression(RetiredBVConcatExpression<SC>),
    BVConcrete(RetiredBVConcrete),
    BVMultiplyExpression(RetiredBVMultiplyExpression<SC>),
    BVOrExpression(RetiredBVOrExpression<SC>),
    BVSignExtendExpression(RetiredBVSignExtendExpression<SC>),
    BVSliceExpression(RetiredBVSliceExpression<SC>),
    BVSllExpression(RetiredBVSllExpression<SC>),
    BVSrlExpression(RetiredBVSrlExpression<SC>),
    BVSubExpression(RetiredBVSubExpression<SC>),
    BVSymbol(RetiredBVSymbol),
    BVUnsignedRemainderExpression(RetiredBVUnsignedRemainderExpression<SC>),
    BVXorExpression(RetiredBVXorExpression<SC>),
}

/* 
fn get_cloned_parent<SC: ScfiaComposition>(weak: &Weak<RefCell<ActiveValueInner<SC>>>, id: u64, cloned_scfia: &mut Scfia<SC>, cloned_scfia_rc: &ScfiaOld<SC>, cloned_actives: &mut BTreeMap<u64, ActiveValue<SC>>, cloned_retired: &mut BTreeMap<u64, RetiredValue<SC>>) -> (Weak<RefCell<ActiveValueInner<SC>>>, Z3_ast) {
    if let Some(cloned_active) = cloned_actives.get(&id) { // Parent might be active and cloned...
        (Rc::downgrade(cloned_active), cloned_active.try_borrow().unwrap().z3_ast)
    } else if let Some(v) = weak.upgrade() { // ...or active and not yet cloned...
        let cloned_parent = v.try_borrow().unwrap().clone_to_stdlib(cloned_scfia, cloned_scfia_rc.clone(), cloned_actives, cloned_retired);
        let cloned_parent_ref = cloned_parent.try_borrow().unwrap();
        (Rc::downgrade(&cloned_parent), cloned_parent_ref.z3_ast)
    } else if let Some(cloned_retired) = cloned_retired.get(&id) { // ...or retired and already cloned
        (Weak::new(), cloned_retired.try_borrow().unwrap().z3_ast)
    } else {
        error!("cannot find {}", id);
        panic!();
    }
}
*/

impl<SC: ScfiaComposition> RetiredValueInner<SC> {
    pub(crate) fn clone_to_stdlib(&self, cloned_scfia: &Scfia<SC>, cloned_actives: &mut BTreeMap<u64, ActiveValue<SC>>, cloned_retired: &mut BTreeMap<u64, RetiredValue<SC>>) -> RetiredValue<SC> {
        unsafe {
            if let Some(value) = cloned_retired.get(&self.id)  {
                return value.clone()
            }

            trace!("Cloning {:?}", self);
            /* 
            let (cloned_value, cloned_z3_ast) = match &self.expression {
                RetiredExpression::BoolConcrete(e) => {
                    let z3_ast = if e.value { Z3_mk_true(cloned_scfia.z3_context) } else { Z3_mk_false(cloned_scfia.z3_context) };
                    Z3_inc_ref(cloned_scfia.z3_context, z3_ast);
                    (RetiredExpression::BoolConcrete(RetiredBoolConcrete { value: e.value }), z3_ast)
                }
                RetiredExpression::BoolEqExpression(e) => {
                    let (s1, s1_ast) = get_cloned_parent(&e.s1, e.s1_id, cloned_scfia, &cloned_scfia_rc, cloned_actives, cloned_retired);
                    let (s2, s2_ast) = get_cloned_parent(&e.s2, e.s2_id, cloned_scfia, &cloned_scfia_rc, cloned_actives, cloned_retired);
                    if let Some(value) = cloned_retired.get(&self.id)  {
                        return value.clone()
                    }
                    let z3_ast = Z3_mk_eq(cloned_scfia.z3_context, s1_ast, s2_ast);
                    Z3_inc_ref(cloned_scfia.z3_context, z3_ast);
                    if e.is_assert {
                        Z3_solver_assert(cloned_scfia.z3_context, cloned_scfia.z3_solver, z3_ast);
                    }
                    (RetiredExpression::BoolEqExpression(RetiredBoolEqExpression { s1_id: e.s1_id, s2_id: e.s2_id, is_assert: e.is_assert, phantom: PhantomData, s1, s2 }), z3_ast)
                }
                RetiredExpression::BoolNotExpression(e) => {
                    let (s1, s1_ast) = get_cloned_parent(&e.s1, e.s1_id, cloned_scfia, &cloned_scfia_rc, cloned_actives, cloned_retired);
                    if let Some(value) = cloned_retired.get(&self.id)  {
                        return value.clone()
                    }
                    let z3_ast = Z3_mk_not(cloned_scfia.z3_context, s1_ast);
                    Z3_inc_ref(cloned_scfia.z3_context, z3_ast);
                    if e.is_assert {
                        Z3_solver_assert(cloned_scfia.z3_context, cloned_scfia.z3_solver, z3_ast);
                    }
                    (RetiredExpression::BoolNotExpression(RetiredBoolNotExpression { s1_id: e.s1_id, is_assert: e.is_assert, phantom: PhantomData, s1 }), z3_ast)
                }
                RetiredExpression::BoolSignedLessThanExpression(e) => {
                    let (s1, s1_ast) = get_cloned_parent(&e.s1, e.s1_id, cloned_scfia, &cloned_scfia_rc, cloned_actives, cloned_retired);
                    let (s2, s2_ast) = get_cloned_parent(&e.s2, e.s2_id, cloned_scfia, &cloned_scfia_rc, cloned_actives, cloned_retired);
                    if let Some(value) = cloned_retired.get(&self.id)  {
                        return value.clone()
                    }
                    let z3_ast = Z3_mk_bvslt(cloned_scfia.z3_context, s1_ast, s2_ast);
                    Z3_inc_ref(cloned_scfia.z3_context, z3_ast);
                    if e.is_assert {
                        Z3_solver_assert(cloned_scfia.z3_context, cloned_scfia.z3_solver, z3_ast);
                    }
                    (RetiredExpression::BoolSignedLessThanExpression(RetiredBoolSignedLessThanExpression { s1_id: e.s1_id, s2_id: e.s2_id, is_assert: e.is_assert, phantom: PhantomData, s1, s2 }), z3_ast)
                }
                RetiredExpression::BoolUnsignedLessThanExpression(e) => {
                    let (s1, s1_ast) = get_cloned_parent(&e.s1, e.s1_id, cloned_scfia, &cloned_scfia_rc, cloned_actives, cloned_retired);
                    let (s2, s2_ast) = get_cloned_parent(&e.s2, e.s2_id, cloned_scfia, &cloned_scfia_rc, cloned_actives, cloned_retired);
                    if let Some(value) = cloned_retired.get(&self.id)  {
                        return value.clone()
                    }
                    let z3_ast = Z3_mk_bvult(cloned_scfia.z3_context, s1_ast, s2_ast);
                    Z3_inc_ref(cloned_scfia.z3_context, z3_ast);
                    if e.is_assert {
                        Z3_solver_assert(cloned_scfia.z3_context, cloned_scfia.z3_solver, z3_ast);
                    }
                    (RetiredExpression::BoolUnsignedLessThanExpression(RetiredBoolUnsignedLessThanExpression { s1_id: e.s1_id, s2_id: e.s2_id, is_assert: e.is_assert, phantom: PhantomData, s1, s2 }), z3_ast)
                }
                RetiredExpression::BVAddExpression(e) => {
                    let (s1, s1_ast) = get_cloned_parent(&e.s1, e.s1_id, cloned_scfia, &cloned_scfia_rc, cloned_actives, cloned_retired);
                    let (s2, s2_ast) = get_cloned_parent(&e.s2, e.s2_id, cloned_scfia, &cloned_scfia_rc, cloned_actives, cloned_retired);
                    if let Some(value) = cloned_retired.get(&self.id)  {
                        return value.clone()
                    }
                    let z3_ast = Z3_mk_bvadd(cloned_scfia.z3_context, s1_ast, s2_ast);
                    Z3_inc_ref(cloned_scfia.z3_context, z3_ast);
                    (RetiredExpression::BVAddExpression(RetiredBVAddExpression { s1_id: e.s1_id, s2_id: e.s2_id, phantom: PhantomData, width:e.width, s1, s2 }), z3_ast)
                }
                RetiredExpression::BVAndExpression(e) => {
                    let (s1, s1_ast) = get_cloned_parent(&e.s1, e.s1_id, cloned_scfia, &cloned_scfia_rc, cloned_actives, cloned_retired);
                    let (s2, s2_ast) = get_cloned_parent(&e.s2, e.s2_id, cloned_scfia, &cloned_scfia_rc, cloned_actives, cloned_retired);
                    if let Some(value) = cloned_retired.get(&self.id)  {
                        return value.clone()
                    }
                    let z3_ast = Z3_mk_bvand(cloned_scfia.z3_context, s1_ast, s2_ast);
                    Z3_inc_ref(cloned_scfia.z3_context, z3_ast);
                    (RetiredExpression::BVAndExpression(RetiredBVAndExpression { s1_id: e.s1_id, s2_id: e.s2_id, phantom: PhantomData, width:e.width, s1, s2 }), z3_ast)
                }
                RetiredExpression::BVConcatExpression(e) => {
                    let (s1, s1_ast) = get_cloned_parent(&e.s1, e.s1_id, cloned_scfia, &cloned_scfia_rc, cloned_actives, cloned_retired);
                    let (s2, s2_ast) = get_cloned_parent(&e.s2, e.s2_id, cloned_scfia, &cloned_scfia_rc, cloned_actives, cloned_retired);
                    if let Some(value) = cloned_retired.get(&self.id)  {
                        return value.clone()
                    }
                    let z3_ast = Z3_mk_concat(cloned_scfia.z3_context, s1_ast, s2_ast);
                    Z3_inc_ref(cloned_scfia.z3_context, z3_ast);
                    (RetiredExpression::BVConcatExpression(RetiredBVConcatExpression { s1_id: e.s1_id, s2_id: e.s2_id, phantom: PhantomData, width:e.width, s1, s2 }), z3_ast)
                }
                RetiredExpression::BVConcrete(e) => {
                    let sort = Z3_mk_bv_sort(cloned_scfia.z3_context, e.width);
                    let z3_ast = Z3_mk_unsigned_int64(cloned_scfia.z3_context, e.value, sort);
                    Z3_inc_ref(cloned_scfia.z3_context, z3_ast);
                    (RetiredExpression::BVConcrete(RetiredBVConcrete { value: e.value, width: e.width }), z3_ast)
                }
                RetiredExpression::BVMultiplyExpression(e) => {
                    let (s1, s1_ast) = get_cloned_parent(&e.s1, e.s1_id, cloned_scfia, &cloned_scfia_rc, cloned_actives, cloned_retired);
                    let (s2, s2_ast) = get_cloned_parent(&e.s2, e.s2_id, cloned_scfia, &cloned_scfia_rc, cloned_actives, cloned_retired);
                    if let Some(value) = cloned_retired.get(&self.id)  {
                        return value.clone()
                    }
                    let z3_ast = Z3_mk_bvmul(cloned_scfia.z3_context, s1_ast, s2_ast);
                    Z3_inc_ref(cloned_scfia.z3_context, z3_ast);
                    (RetiredExpression::BVMultiplyExpression(RetiredBVMultiplyExpression { s1_id: e.s1_id, s2_id: e.s2_id, phantom: PhantomData, width:e.width, s1, s2 }), z3_ast)
                }
                RetiredExpression::BVOrExpression(e) => {
                    let (s1, s1_ast) = get_cloned_parent(&e.s1, e.s1_id, cloned_scfia, &cloned_scfia_rc, cloned_actives, cloned_retired);
                    let (s2, s2_ast) = get_cloned_parent(&e.s2, e.s2_id, cloned_scfia, &cloned_scfia_rc, cloned_actives, cloned_retired);
                    if let Some(value) = cloned_retired.get(&self.id)  {
                        return value.clone()
                    }
                    let z3_ast = Z3_mk_bvor(cloned_scfia.z3_context, s1_ast, s2_ast);
                    Z3_inc_ref(cloned_scfia.z3_context, z3_ast);
                    (RetiredExpression::BVOrExpression(RetiredBVOrExpression { s1_id: e.s1_id, s2_id: e.s2_id, phantom: PhantomData, width:e.width, s1, s2 }), z3_ast)
                }
                RetiredExpression::BVSignExtendExpression(e) => {
                    let (s1, s1_ast) = get_cloned_parent(&e.s1, e.s1_id, cloned_scfia, &cloned_scfia_rc, cloned_actives, cloned_retired);
                    if let Some(value) = cloned_retired.get(&self.id)  {
                        return value.clone()
                    }
                    let z3_ast = Z3_mk_sign_ext(cloned_scfia.z3_context, e.width - e.input_width, s1_ast);
                    Z3_inc_ref(cloned_scfia.z3_context, z3_ast);
                    (RetiredExpression::BVSignExtendExpression(RetiredBVSignExtendExpression { s1_id: e.s1_id, width: e.width, input_width: e.input_width, phantom: PhantomData, s1 }), z3_ast)
                }
                RetiredExpression::BVSliceExpression(e) => {
                    let (s1, s1_ast) = get_cloned_parent(&e.s1, e.s1_id, cloned_scfia, &cloned_scfia_rc, cloned_actives, cloned_retired);
                    if let Some(value) = cloned_retired.get(&self.id)  {
                        return value.clone()
                    }
                    let z3_ast = Z3_mk_extract(cloned_scfia.z3_context, e.high, e.low, s1_ast);
                    Z3_inc_ref(cloned_scfia.z3_context, z3_ast);
                    (RetiredExpression::BVSliceExpression(RetiredBVSliceExpression { s1_id: e.s1_id, width: e.width, high: e.high, low: e.low, phantom: PhantomData, s1 }), z3_ast)
                }
                RetiredExpression::BVSllExpression(e) => {
                    let (s1, s1_ast) = get_cloned_parent(&e.s1, e.s1_id, cloned_scfia, &cloned_scfia_rc, cloned_actives, cloned_retired);
                    let (s2, s2_ast) = get_cloned_parent(&e.s2, e.s2_id, cloned_scfia, &cloned_scfia_rc, cloned_actives, cloned_retired);
                    if let Some(value) = cloned_retired.get(&self.id)  {
                        return value.clone()
                    }
                    let z3_ast = Z3_mk_bvshl(cloned_scfia.z3_context, s1_ast, s2_ast);
                    Z3_inc_ref(cloned_scfia.z3_context, z3_ast);
                    (RetiredExpression::BVSllExpression(RetiredBVSllExpression { s1_id: e.s1_id, s2_id: e.s2_id, phantom: PhantomData, width:e.width, s1, s2 }), z3_ast)
                }
                RetiredExpression::BVSrlExpression(e) => {
                    let (s1, s1_ast) = get_cloned_parent(&e.s1, e.s1_id, cloned_scfia, &cloned_scfia_rc, cloned_actives, cloned_retired);
                    let (s2, s2_ast) = get_cloned_parent(&e.s2, e.s2_id, cloned_scfia, &cloned_scfia_rc, cloned_actives, cloned_retired);
                    if let Some(value) = cloned_retired.get(&self.id)  {
                        return value.clone()
                    }
                    let z3_ast = Z3_mk_bvlshr(cloned_scfia.z3_context, s1_ast, s2_ast);
                    Z3_inc_ref(cloned_scfia.z3_context, z3_ast);
                    (RetiredExpression::BVSrlExpression(RetiredBVSrlExpression { s1_id: e.s1_id, s2_id: e.s2_id, shamt: e.shamt, phantom: PhantomData, width:e.width, s1, s2 }), z3_ast)
                }
                RetiredExpression::BVSubExpression(e) => {
                    let (s1, s1_ast) = get_cloned_parent(&e.s1, e.s1_id, cloned_scfia, &cloned_scfia_rc, cloned_actives, cloned_retired);
                    let (s2, s2_ast) = get_cloned_parent(&e.s2, e.s2_id, cloned_scfia, &cloned_scfia_rc, cloned_actives, cloned_retired);
                    if let Some(value) = cloned_retired.get(&self.id)  {
                        return value.clone()
                    }
                    let z3_ast = Z3_mk_bvsub(cloned_scfia.z3_context, s1_ast, s2_ast);
                    Z3_inc_ref(cloned_scfia.z3_context, z3_ast);
                    (RetiredExpression::BVSubExpression(RetiredBVSubExpression { s1_id: e.s1_id, s2_id: e.s2_id, phantom: PhantomData, width:e.width, s1, s2 }), z3_ast)
                }
                RetiredExpression::BVSymbol(e) => {
                    let z3_ast = Z3_mk_fresh_const(cloned_scfia.z3_context, PREFIX.as_ptr(), Z3_mk_bv_sort(cloned_scfia.z3_context, e.width));
                    Z3_inc_ref(cloned_scfia.z3_context, z3_ast);
                    (RetiredExpression::BVSymbol(RetiredBVSymbol { width:e.width }), z3_ast)
                }
                RetiredExpression::BVUnsignedRemainderExpression(e) => {
                    let (s1, s1_ast) = get_cloned_parent(&e.s1, e.s1_id, cloned_scfia, &cloned_scfia_rc, cloned_actives, cloned_retired);
                    let (s2, s2_ast) = get_cloned_parent(&e.s2, e.s2_id, cloned_scfia, &cloned_scfia_rc, cloned_actives, cloned_retired);
                    if let Some(value) = cloned_retired.get(&self.id)  {
                        return value.clone()
                    }
                    let z3_ast = Z3_mk_bvurem(cloned_scfia.z3_context, s1_ast, s2_ast);
                    Z3_inc_ref(cloned_scfia.z3_context, z3_ast);
                    (RetiredExpression::BVUnsignedRemainderExpression(RetiredBVUnsignedRemainderExpression { s1_id: e.s1_id, s2_id: e.s2_id, phantom: PhantomData, width:e.width, s1, s2 }), z3_ast)
                }
                RetiredExpression::BVXorExpression(e) => {
                    let (s1, s1_ast) = get_cloned_parent(&e.s1, e.s1_id, cloned_scfia, &cloned_scfia_rc, cloned_actives, cloned_retired);
                    let (s2, s2_ast) = get_cloned_parent(&e.s2, e.s2_id, cloned_scfia, &cloned_scfia_rc, cloned_actives, cloned_retired);
                    if let Some(value) = cloned_retired.get(&self.id)  {
                        return value.clone()
                    }
                    let z3_ast = Z3_mk_bvxor(cloned_scfia.z3_context, s1_ast, s2_ast);
                    Z3_inc_ref(cloned_scfia.z3_context, z3_ast);
                    (RetiredExpression::BVXorExpression(RetiredBVXorExpression { s1_id: e.s1_id, s2_id: e.s2_id, phantom: PhantomData, width:e.width, s1, s2 }), z3_ast)
                }
            };

            let cloned_value_rc = cloned_scfia.new_inactive(cloned_value, cloned_z3_ast, self.id, cloned_scfia_rc.clone());
            let old = cloned_retired.insert(self.id, cloned_value_rc.clone());
            if old.is_some() {
                error!("clone_to_stdlib {} twice for scfia {:?}", self.id, Rc::as_ptr(&cloned_scfia_rc.inner));
                panic!();
            }
            cloned_value_rc
            */
            todo!()
        }
    }
}

impl<SC: ScfiaComposition> Debug for RetiredValueInner<SC> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        self.expression.fmt(f)?;
        f.write_str(format!("[id={}]", self.id).as_str())
    }
}

impl<SC: ScfiaComposition> Debug for RetiredExpression<SC> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            RetiredExpression::BVConcrete(e) => e.fmt(f),
            RetiredExpression::BVSymbol(e) => e.fmt(f),
            RetiredExpression::BVAddExpression(e) => e.fmt(f),
            RetiredExpression::BoolConcrete(e) => e.fmt(f),
            RetiredExpression::BoolEqExpression(e) => e.fmt(f),
            RetiredExpression::BoolNotExpression(e) => e.fmt(f),
            RetiredExpression::BoolSignedLessThanExpression(e) => e.fmt(f),
            RetiredExpression::BoolUnsignedLessThanExpression(e) => e.fmt(f),
            RetiredExpression::BVAndExpression(e) => e.fmt(f),
            RetiredExpression::BVConcatExpression(e) => e.fmt(f),
            RetiredExpression::BVMultiplyExpression(e) => e.fmt(f),
            RetiredExpression::BVOrExpression(e) => e.fmt(f),
            RetiredExpression::BVSignExtendExpression(e) => e.fmt(f),
            RetiredExpression::BVSliceExpression(e) => e.fmt(f),
            RetiredExpression::BVSllExpression(e) => e.fmt(f),
            RetiredExpression::BVSrlExpression(e) => e.fmt(f),
            RetiredExpression::BVSubExpression(e) => e.fmt(f),
            RetiredExpression::BVUnsignedRemainderExpression(e) => e.fmt(f),
            RetiredExpression::BVXorExpression(e) => e.fmt(f),
        }
    }
}
