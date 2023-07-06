use std::{
    cell::RefCell,
    collections::BTreeMap,
    fmt::Debug,
    marker::PhantomData,
    rc::{Rc, Weak},
};

use log::{error, trace};

use crate::{scfia::Scfia, z3_handle::Z3Ast, ScfiaComposition};

use super::{
    active_value::{ActiveValue, ActiveValueZ3, ValueComment},
    bool_eq_expression::RetiredBoolEqExpression,
    bool_not_expresssion::RetiredBoolNotExpression,
    bool_signed_less_than_expression::RetiredBoolSignedLessThanExpression,
    bool_unsigned_less_than_expression::RetiredBoolUnsignedLessThanExpression,
    bv_add_expression::RetiredBVAddExpression,
    bv_and_expression::RetiredBVAndExpression,
    bv_concat_expression::RetiredBVConcatExpression,
    bv_concrete_expression::RetiredBVConcreteExpression,
    bv_multiply_expression::RetiredBVMultiplyExpression,
    bv_or_expression::RetiredBVOrExpression,
    bv_sign_extend_expression::RetiredBVSignExtendExpression,
    bv_slice_expression::RetiredBVSliceExpression,
    bv_sll_expression::RetiredBVSllExpression,
    bv_srl_expression::RetiredBVSrlExpression,
    bv_sub_expression::RetiredBVSubExpression,
    bv_symbol::RetiredBVSymbol,
    bv_unsigned_remainder_expression::RetiredBVUnsignedRemainderExpression,
    bv_xor_expression::RetiredBVXorExpression, bv_not_expression::RetiredBVNotExpression,
};

pub type RetiredValue<Model> = Rc<RefCell<RetiredValueInner<Model>>>;
pub type RetiredValueWeak<Model> = Weak<RefCell<RetiredValueInner<Model>>>;

pub struct RetiredValueInner<SC: ScfiaComposition> {
    pub id: u64,
    pub z3_ast: Z3Ast<SC>,
    pub expression: RetiredExpression<SC>,
    pub scfia: Weak<Scfia<SC>>,
    pub comment: Option<ValueComment>,
}

#[derive(Debug)]
pub struct ParentWeakReference<SC: ScfiaComposition> {
    pub id: u64,
    pub weak: Weak<RefCell<ActiveValueZ3<SC>>>,
}

pub enum RetiredExpression<SC: ScfiaComposition> {
    BoolEqExpression(RetiredBoolEqExpression<SC>),
    BoolNotExpression(RetiredBoolNotExpression<SC>),
    BoolSignedLessThanExpression(RetiredBoolSignedLessThanExpression<SC>),
    BoolUnsignedLessThanExpression(RetiredBoolUnsignedLessThanExpression<SC>),
    BVAddExpression(RetiredBVAddExpression<SC>),
    BVAndExpression(RetiredBVAndExpression<SC>),
    BVConcatExpression(RetiredBVConcatExpression<SC>),
    BVConcreteExpression(RetiredBVConcreteExpression),
    BVMultiplyExpression(RetiredBVMultiplyExpression<SC>),
    BVNotExpression(RetiredBVNotExpression<SC>),
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

fn get_cloned_parent<SC: ScfiaComposition>(
    parent_ref: &ParentWeakReference<SC>,
    cloned_scfia: &Scfia<SC>,
    cloned_actives: &mut BTreeMap<u64, ActiveValue<SC>>,
    cloned_retired: &mut BTreeMap<u64, RetiredValue<SC>>,
) -> (ParentWeakReference<SC>, Z3Ast<SC>) {
    if let Some(cloned_active) = cloned_actives.get(&parent_ref.id) {
        // Parent is active and cloned
        (
            ParentWeakReference {
                id: parent_ref.id,
                weak: cloned_active.get_weak(),
            },
            cloned_active.get_z3_ast(),
        )
    } else if let Some(v) = parent_ref.weak.upgrade() {
        // Parent is active and not yet cloned
        let cloned_parent = v.try_borrow().unwrap().clone_to_stdlib(cloned_scfia, cloned_actives, cloned_retired);
        (
            ParentWeakReference {
                id: parent_ref.id,
                weak: cloned_parent.get_weak(),
            },
            cloned_parent.get_z3_ast(),
        )
    } else if let Some(cloned_retired) = cloned_retired.get(&parent_ref.id) {
        // Parent is retired and already cloned
        (
            ParentWeakReference {
                id: parent_ref.id,
                weak: Weak::new(),
            },
            cloned_retired.try_borrow().unwrap().z3_ast.clone(),
        )
    } else {
        error!("cannot find {}", parent_ref.id);
        panic!();
    }
}

impl<SC: ScfiaComposition> RetiredValueInner<SC> {
    pub(crate) fn clone_to_stdlib(
        &self,
        cloned_scfia: &Scfia<SC>,
        cloned_actives: &mut BTreeMap<u64, ActiveValue<SC>>,
        cloned_retired: &mut BTreeMap<u64, RetiredValue<SC>>,
    ) -> RetiredValue<SC> {
        if let Some(value) = cloned_retired.get(&self.id) {
            return value.clone();
        }

        trace!("Cloning {:?}", self);
        let cloned_inactive: RetiredValue<SC> = match &self.expression {
            RetiredExpression::BoolEqExpression(e) => {
                let (s1, s1_ast) = get_cloned_parent(&e.s1, cloned_scfia, cloned_actives, cloned_retired);
                let (s2, s2_ast) = get_cloned_parent(&e.s2, cloned_scfia, cloned_actives, cloned_retired);
                if let Some(value) = cloned_retired.get(&self.id) {
                    return value.clone();
                }
                let z3_ast = cloned_scfia.z3.new_eq(&s1_ast, &s2_ast, e.is_assert);
                cloned_scfia.new_inactive(
                    RetiredExpression::BoolEqExpression(RetiredBoolEqExpression {
                        s1,
                        s2,
                        is_assert: e.is_assert,
                        phantom: PhantomData,
                    }),
                    z3_ast,
                    self.id,
                )
            }
            RetiredExpression::BoolNotExpression(e) => {
                let (s1, s1_ast) = get_cloned_parent(&e.s1, cloned_scfia, cloned_actives, cloned_retired);
                if let Some(value) = cloned_retired.get(&self.id) {
                    return value.clone();
                }
                let z3_ast = cloned_scfia.z3.new_not(&s1_ast, e.is_assert);
                cloned_scfia.new_inactive(
                    RetiredExpression::BoolNotExpression(RetiredBoolNotExpression {
                        s1,
                        is_assert: e.is_assert,
                        phantom: PhantomData,
                    }),
                    z3_ast,
                    self.id,
                )
            }
            RetiredExpression::BoolSignedLessThanExpression(e) => {
                let (s1, s1_ast) = get_cloned_parent(&e.s1, cloned_scfia, cloned_actives, cloned_retired);
                let (s2, s2_ast) = get_cloned_parent(&e.s2, cloned_scfia, cloned_actives, cloned_retired);
                if let Some(value) = cloned_retired.get(&self.id) {
                    return value.clone();
                }
                let z3_ast = cloned_scfia.z3.new_bvslt(&s1_ast, &s2_ast, e.is_assert);
                cloned_scfia.new_inactive(
                    RetiredExpression::BoolSignedLessThanExpression(RetiredBoolSignedLessThanExpression {
                        s1,
                        s2,
                        is_assert: e.is_assert,
                        phantom: PhantomData,
                    }),
                    z3_ast,
                    self.id,
                )
            }
            RetiredExpression::BoolUnsignedLessThanExpression(e) => {
                let (s1, s1_ast) = get_cloned_parent(&e.s1, cloned_scfia, cloned_actives, cloned_retired);
                let (s2, s2_ast) = get_cloned_parent(&e.s2, cloned_scfia, cloned_actives, cloned_retired);
                if let Some(value) = cloned_retired.get(&self.id) {
                    return value.clone();
                }
                let z3_ast = cloned_scfia.z3.new_bvult(&s1_ast, &s2_ast, e.is_assert);
                cloned_scfia.new_inactive(
                    RetiredExpression::BoolUnsignedLessThanExpression(RetiredBoolUnsignedLessThanExpression {
                        s1,
                        s2,
                        is_assert: e.is_assert,
                        phantom: PhantomData,
                    }),
                    z3_ast,
                    self.id,
                )
            }
            RetiredExpression::BVAddExpression(e) => {
                let (s1, s1_ast) = get_cloned_parent(&e.s1, cloned_scfia, cloned_actives, cloned_retired);
                let (s2, s2_ast) = get_cloned_parent(&e.s2, cloned_scfia, cloned_actives, cloned_retired);
                if let Some(value) = cloned_retired.get(&self.id) {
                    return value.clone();
                }
                let z3_ast = cloned_scfia.z3.new_bvadd(&s1_ast, &s2_ast);
                cloned_scfia.new_inactive(
                    RetiredExpression::BVAddExpression(RetiredBVAddExpression {
                        s1,
                        s2,
                        phantom: PhantomData,
                        width: e.width,
                    }),
                    z3_ast,
                    self.id,
                )
            }
            RetiredExpression::BVAndExpression(e) => {
                let (s1, s1_ast) = get_cloned_parent(&e.s1, cloned_scfia, cloned_actives, cloned_retired);
                let (s2, s2_ast) = get_cloned_parent(&e.s2, cloned_scfia, cloned_actives, cloned_retired);
                if let Some(value) = cloned_retired.get(&self.id) {
                    return value.clone();
                }
                let z3_ast = cloned_scfia.z3.new_bvand(&s1_ast, &s2_ast);
                cloned_scfia.new_inactive(
                    RetiredExpression::BVAndExpression(RetiredBVAndExpression {
                        s1,
                        s2,
                        phantom: PhantomData,
                        width: e.width,
                    }),
                    z3_ast,
                    self.id,
                )
            }
            RetiredExpression::BVConcatExpression(e) => {
                let (s1, s1_ast) = get_cloned_parent(&e.s1, cloned_scfia, cloned_actives, cloned_retired);
                let (s2, s2_ast) = get_cloned_parent(&e.s2, cloned_scfia, cloned_actives, cloned_retired);
                if let Some(value) = cloned_retired.get(&self.id) {
                    return value.clone();
                }
                let z3_ast = cloned_scfia.z3.new_bvconcat(&s1_ast, &s2_ast);
                cloned_scfia.new_inactive(
                    RetiredExpression::BVConcatExpression(RetiredBVConcatExpression {
                        s1,
                        s2,
                        phantom: PhantomData,
                        width: e.width,
                    }),
                    z3_ast,
                    self.id,
                )
            }
            RetiredExpression::BVMultiplyExpression(e) => {
                let (s1, s1_ast) = get_cloned_parent(&e.s1, cloned_scfia, cloned_actives, cloned_retired);
                let (s2, s2_ast) = get_cloned_parent(&e.s2, cloned_scfia, cloned_actives, cloned_retired);
                if let Some(value) = cloned_retired.get(&self.id) {
                    return value.clone();
                }
                let z3_ast = cloned_scfia.z3.new_bvmul(&s1_ast, &s2_ast);
                cloned_scfia.new_inactive(
                    RetiredExpression::BVMultiplyExpression(RetiredBVMultiplyExpression {
                        s1,
                        s2,
                        phantom: PhantomData,
                        width: e.width,
                    }),
                    z3_ast,
                    self.id,
                )
            }
            RetiredExpression::BVNotExpression(e) => {
                let (s1, s1_ast) = get_cloned_parent(&e.s1, cloned_scfia, cloned_actives, cloned_retired);
                if let Some(value) = cloned_retired.get(&self.id) {
                    return value.clone();
                }
                let z3_ast = cloned_scfia.z3.new_bvnot(&s1_ast);
                cloned_scfia.new_inactive(
                    RetiredExpression::BVNotExpression(RetiredBVNotExpression {
                        s1,
                        phantom: PhantomData,
                        width: e.width,
                    }),
                    z3_ast,
                    self.id,
                )
            }
            RetiredExpression::BVOrExpression(e) => {
                let (s1, s1_ast) = get_cloned_parent(&e.s1, cloned_scfia, cloned_actives, cloned_retired);
                let (s2, s2_ast) = get_cloned_parent(&e.s2, cloned_scfia, cloned_actives, cloned_retired);
                if let Some(value) = cloned_retired.get(&self.id) {
                    return value.clone();
                }
                let z3_ast = cloned_scfia.z3.new_bvor(&s1_ast, &s2_ast);
                cloned_scfia.new_inactive(
                    RetiredExpression::BVOrExpression(RetiredBVOrExpression {
                        s1,
                        s2,
                        phantom: PhantomData,
                        width: e.width,
                    }),
                    z3_ast,
                    self.id,
                )
            }
            RetiredExpression::BVSignExtendExpression(e) => {
                let (s1, s1_ast) = get_cloned_parent(&e.s1, cloned_scfia, cloned_actives, cloned_retired);
                if let Some(value) = cloned_retired.get(&self.id) {
                    return value.clone();
                }
                let z3_ast = cloned_scfia.z3.new_sign_ext(e.width - e.input_width, &s1_ast);
                cloned_scfia.new_inactive(
                    RetiredExpression::BVSignExtendExpression(RetiredBVSignExtendExpression {
                        s1,
                        width: e.width,
                        input_width: e.input_width,
                        phantom: PhantomData,
                    }),
                    z3_ast,
                    self.id,
                )
            }
            RetiredExpression::BVSliceExpression(e) => {
                let (s1, s1_ast) = get_cloned_parent(&e.s1, cloned_scfia, cloned_actives, cloned_retired);
                if let Some(value) = cloned_retired.get(&self.id) {
                    return value.clone();
                }
                let z3_ast = cloned_scfia.z3.new_extract(e.high, e.low, &s1_ast);
                cloned_scfia.new_inactive(
                    RetiredExpression::BVSliceExpression(RetiredBVSliceExpression {
                        s1,
                        width: e.width,
                        high: e.high,
                        low: e.low,
                        phantom: PhantomData,
                    }),
                    z3_ast,
                    self.id,
                )
            }
            RetiredExpression::BVSllExpression(e) => {
                let (s1, s1_ast) = get_cloned_parent(&e.s1, cloned_scfia, cloned_actives, cloned_retired);
                let (s2, s2_ast) = get_cloned_parent(&e.s2, cloned_scfia, cloned_actives, cloned_retired);
                if let Some(value) = cloned_retired.get(&self.id) {
                    return value.clone();
                }
                let z3_ast = cloned_scfia.z3.new_bvshl(&s1_ast, &s2_ast);
                cloned_scfia.new_inactive(
                    RetiredExpression::BVSllExpression(RetiredBVSllExpression {
                        s1,
                        s2,
                        phantom: PhantomData,
                        width: e.width,
                    }),
                    z3_ast,
                    self.id,
                )
            }
            RetiredExpression::BVSrlExpression(e) => {
                let (s1, s1_ast) = get_cloned_parent(&e.s1, cloned_scfia, cloned_actives, cloned_retired);
                let (s2, s2_ast) = get_cloned_parent(&e.s2, cloned_scfia, cloned_actives, cloned_retired);
                if let Some(value) = cloned_retired.get(&self.id) {
                    return value.clone();
                }
                let z3_ast = cloned_scfia.z3.new_bvlshr(&s1_ast, &s2_ast);
                cloned_scfia.new_inactive(
                    RetiredExpression::BVSrlExpression(RetiredBVSrlExpression {
                        s1,
                        s2,
                        shamt: e.shamt,
                        phantom: PhantomData,
                        width: e.width,
                    }),
                    z3_ast,
                    self.id,
                )
            }
            RetiredExpression::BVSubExpression(e) => {
                let (s1, s1_ast) = get_cloned_parent(&e.s1, cloned_scfia, cloned_actives, cloned_retired);
                let (s2, s2_ast) = get_cloned_parent(&e.s2, cloned_scfia, cloned_actives, cloned_retired);
                if let Some(value) = cloned_retired.get(&self.id) {
                    return value.clone();
                }
                let z3_ast = cloned_scfia.z3.new_bvsub(&s1_ast, &s2_ast);
                cloned_scfia.new_inactive(
                    RetiredExpression::BVSubExpression(RetiredBVSubExpression {
                        s1,
                        s2,
                        phantom: PhantomData,
                        width: e.width,
                    }),
                    z3_ast,
                    self.id,
                )
            }
            RetiredExpression::BVSymbol(e) => {
                let z3_ast = cloned_scfia.z3.new_fresh_const(e.width);
                cloned_scfia.new_inactive(RetiredExpression::BVSymbol(RetiredBVSymbol { width: e.width }), z3_ast, self.id)
            }
            RetiredExpression::BVUnsignedRemainderExpression(e) => {
                let (s1, s1_ast) = get_cloned_parent(&e.s1, cloned_scfia, cloned_actives, cloned_retired);
                let (s2, s2_ast) = get_cloned_parent(&e.s2, cloned_scfia, cloned_actives, cloned_retired);
                if let Some(value) = cloned_retired.get(&self.id) {
                    return value.clone();
                }
                let z3_ast = cloned_scfia.z3.new_bvurem(&s1_ast, &s2_ast);
                cloned_scfia.new_inactive(
                    RetiredExpression::BVUnsignedRemainderExpression(RetiredBVUnsignedRemainderExpression {
                        s1,
                        s2,
                        phantom: PhantomData,
                        width: e.width,
                    }),
                    z3_ast,
                    self.id,
                )
            }
            RetiredExpression::BVXorExpression(e) => {
                let (s1, s1_ast) = get_cloned_parent(&e.s1, cloned_scfia, cloned_actives, cloned_retired);
                let (s2, s2_ast) = get_cloned_parent(&e.s2, cloned_scfia, cloned_actives, cloned_retired);
                if let Some(value) = cloned_retired.get(&self.id) {
                    return value.clone();
                }
                let z3_ast = cloned_scfia.z3.new_bvxor(&s1_ast, &s2_ast);
                cloned_scfia.new_inactive(
                    RetiredExpression::BVXorExpression(RetiredBVXorExpression {
                        s1,
                        s2,
                        phantom: PhantomData,
                        width: e.width,
                    }),
                    z3_ast,
                    self.id,
                )
            }
            RetiredExpression::BVConcreteExpression(e) => {
                let z3_ast = cloned_scfia.z3.new_bv_concrete(e.value, e.width);
                cloned_scfia.new_inactive(
                    RetiredExpression::BVConcreteExpression(RetiredBVConcreteExpression {
                        value: e.value,
                        width: e.width,
                    }),
                    z3_ast,
                    self.id,
                )
            }
        };

        let old = cloned_retired.insert(self.id, cloned_inactive.clone());
        if old.is_some() {
            error!("clone_to_stdlib {} twice for scfia {:?}", self.id, Rc::as_ptr(&cloned_inactive));
            panic!();
        }
        cloned_inactive
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
            RetiredExpression::BVSymbol(e) => e.fmt(f),
            RetiredExpression::BVAddExpression(e) => e.fmt(f),
            RetiredExpression::BoolEqExpression(e) => e.fmt(f),
            RetiredExpression::BoolNotExpression(e) => e.fmt(f),
            RetiredExpression::BoolSignedLessThanExpression(e) => e.fmt(f),
            RetiredExpression::BoolUnsignedLessThanExpression(e) => e.fmt(f),
            RetiredExpression::BVAndExpression(e) => e.fmt(f),
            RetiredExpression::BVConcatExpression(e) => e.fmt(f),
            RetiredExpression::BVMultiplyExpression(e) => e.fmt(f),
            RetiredExpression::BVNotExpression(e) => e.fmt(f),
            RetiredExpression::BVOrExpression(e) => e.fmt(f),
            RetiredExpression::BVSignExtendExpression(e) => e.fmt(f),
            RetiredExpression::BVSliceExpression(e) => e.fmt(f),
            RetiredExpression::BVSllExpression(e) => e.fmt(f),
            RetiredExpression::BVSrlExpression(e) => e.fmt(f),
            RetiredExpression::BVSubExpression(e) => e.fmt(f),
            RetiredExpression::BVUnsignedRemainderExpression(e) => e.fmt(f),
            RetiredExpression::BVXorExpression(e) => e.fmt(f),
            RetiredExpression::BVConcreteExpression(e) => e.fmt(f),
        }
    }
}
