use std::cell::RefCell;
use std::collections::BTreeMap;
use std::ptr;
use std::rc::Rc;

use log::debug;
use log::trace;
use log::warn;

use z3_sys::AstKind;
use z3_sys::SortKind;
use z3_sys::Z3_ast;
use z3_sys::Z3_context;
use z3_sys::Z3_dec_ref;
use z3_sys::Z3_del_config;
use z3_sys::Z3_del_context;
use z3_sys::Z3_get_ast_kind;
use z3_sys::Z3_get_bv_sort_size;
use z3_sys::Z3_get_numeral_uint64;
use z3_sys::Z3_get_sort;
use z3_sys::Z3_get_sort_kind;
use z3_sys::Z3_inc_ref;
use z3_sys::Z3_mk_bv_sort;
use z3_sys::Z3_mk_bvadd;
use z3_sys::Z3_mk_bvand;
use z3_sys::Z3_mk_bvlshr;
use z3_sys::Z3_mk_bvmul;
use z3_sys::Z3_mk_bvor;
use z3_sys::Z3_mk_bvshl;
use z3_sys::Z3_mk_bvslt;
use z3_sys::Z3_mk_bvsub;
use z3_sys::Z3_mk_bvult;
use z3_sys::Z3_mk_bvurem;
use z3_sys::Z3_mk_bvxor;
use z3_sys::Z3_mk_concat;
use z3_sys::Z3_mk_config;
use z3_sys::Z3_mk_context_rc;
use z3_sys::Z3_mk_eq;
use z3_sys::Z3_mk_extract;
use z3_sys::Z3_mk_false;
use z3_sys::Z3_mk_fresh_const;
use z3_sys::Z3_mk_not;
use z3_sys::Z3_mk_sign_ext;
use z3_sys::Z3_mk_solver;
use z3_sys::Z3_mk_true;
use z3_sys::Z3_mk_unsigned_int64;
use z3_sys::Z3_mk_zero_ext;
use z3_sys::Z3_model_eval;
use z3_sys::Z3_solver;
use z3_sys::Z3_solver_assert;
use z3_sys::Z3_solver_check_assumptions;
use z3_sys::Z3_solver_get_model;
use z3_sys::Z3_solver_inc_ref;
use z3_sys::Z3_string;
use z3_sys::Z3_L_FALSE;

use crate::values::active_value::ActiveExpression;
use crate::values::active_value::ActiveValue;
use crate::values::active_value::ActiveValueInner;
use crate::values::active_value::ActiveValueWeak;
use crate::values::bool_concrete::BoolConcrete;
use crate::values::bool_eq_expression::BoolEqExpression;
use crate::values::bool_not_expresssion::BoolNotExpression;
use crate::values::bool_signed_less_than_expression::BoolSignedLessThanExpression;
use crate::values::bool_unsigned_less_than_expression::BoolUnsignedLessThanExpression;
use crate::values::bv_add_expression::BVAddExpression;
use crate::values::bv_and_expression::BVAndExpression;
use crate::values::bv_concat_expression::BVConcatExpression;
use crate::values::bv_concrete::BVConcrete;
use crate::GenericForkSink;
use crate::ScfiaComposition;

use crate::values::bool_concrete::RetiredBoolConcrete;
use crate::values::bool_eq_expression::RetiredBoolEqExpression;
use crate::values::bool_not_expresssion::RetiredBoolNotExpression;
use crate::values::bool_signed_less_than_expression::RetiredBoolSignedLessThanExpression;
use crate::values::bool_unsigned_less_than_expression::RetiredBoolUnsignedLessThanExpression;
use crate::values::bv_add_expression::RetiredBVAddExpression;
use crate::values::bv_and_expression::RetiredBVAndExpression;
use crate::values::bv_concat_expression::RetiredBVConcatExpression;
use crate::values::bv_concrete::RetiredBVConcrete;
use crate::values::bv_multiply_expression::BVMultiplyExpression;
use crate::values::bv_multiply_expression::RetiredBVMultiplyExpression;
use crate::values::bv_or_expression::BVOrExpression;
use crate::values::bv_or_expression::RetiredBVOrExpression;
use crate::values::bv_sign_extend_expression::BVSignExtendExpression;
use crate::values::bv_sign_extend_expression::RetiredBVSignExtendExpression;
use crate::values::bv_slice_expression::BVSliceExpression;
use crate::values::bv_slice_expression::RetiredBVSliceExpression;
use crate::values::bv_sll_expression::BVSllExpression;
use crate::values::bv_sll_expression::RetiredBVSllExpression;
use crate::values::bv_srl_expression::RetiredBVSrlExpression;
use crate::values::bv_sub_expression::BVSubExpression;
use crate::values::bv_sub_expression::RetiredBVSubExpression;
use crate::values::bv_symbol::BVSymbol;
use crate::values::bv_symbol::RetiredBVSymbol;
use crate::values::bv_unsigned_remainder_expression::BVUnsignedRemainderExpression;
use crate::values::bv_xor_expression::BVXorExpression;
use crate::values::bv_xor_expression::RetiredBVXorExpression;
use crate::values::retired_value::RetiredExpression;
use crate::values::retired_value::RetiredValue;
use crate::values::retired_value::RetiredValueInner;
use crate::values::retired_value::RetiredValueWeak;

#[derive(Clone)]
pub struct Scfia<SC: ScfiaComposition> {
    pub(crate) inner: Rc<RefCell<ScfiaInner<SC>>>,
}

pub struct ScfiaInner<SC: ScfiaComposition> {
    next_symbol_id: u64,
    pub(crate) active_symbols: BTreeMap<u64, ActiveValueWeak<SC>>,
    pub(crate) retired_symbols: BTreeMap<u64, RetiredValueWeak<SC>>,
    pub(crate) z3_context: Z3_context,
    pub(crate) z3_solver: Z3_solver,
}

impl<SC: ScfiaComposition> Scfia<SC> {
    pub fn new() -> Scfia<SC> {
        unsafe {
            let z3_config = Z3_mk_config();
            let z3_context = Z3_mk_context_rc(z3_config);
            let z3_solver = Z3_mk_solver(z3_context);
            Z3_solver_inc_ref(z3_context, z3_solver);
            Z3_del_config(z3_config);
            Scfia {
                inner: Rc::new(RefCell::new(ScfiaInner {
                    next_symbol_id: 0,
                    active_symbols: BTreeMap::new(),
                    retired_symbols: BTreeMap::new(),
                    z3_context,
                    z3_solver,
                })),
            }
        }
    }

    pub fn try_get_active_value(&self, id: u64) -> Option<ActiveValue<SC>> {
        let selff = self.inner.try_borrow_mut().unwrap();
        selff.active_symbols.get(&id).map_or(None, |e| Some(e.upgrade().unwrap().clone()))
    }

    pub fn check_condition(&self, value: ActiveValue<SC>, fork_sink: &mut Option<SC::ForkSink>) -> bool {
        unsafe {
            if let ActiveExpression::BoolConcrete(e) = &value.try_borrow().unwrap().expression {
                return e.value;
            }

            let mut can_be_true = false;
            let mut can_be_false = false;

            let neg_condition_symbol = self.new_bool_not(value.clone(), fork_sink);
            let neg_condition_ast = neg_condition_symbol.try_borrow().unwrap().z3_ast;

            let selff = self.inner.try_borrow_mut().unwrap();
            if Z3_solver_check_assumptions(selff.z3_context, selff.z3_solver, 1, &value.try_borrow().unwrap().z3_ast) != Z3_L_FALSE {
                can_be_true = true
            }

            if Z3_solver_check_assumptions(selff.z3_context, selff.z3_solver, 1, &neg_condition_ast) != Z3_L_FALSE {
                can_be_false = true
            }

            // warn!("{:?} {} {}", value, can_be_true, can_be_false);
            if can_be_true && can_be_false {
                if let Some(fork_sink) = fork_sink {
                    fork_sink.fork(neg_condition_symbol.clone());
                    value.try_borrow_mut().unwrap().assert();
                    true
                } else {
                    warn!("unexpected fork");
                    panic!("unexpected fork")
                }
            } else if can_be_true {
                true
            } else if can_be_false {
                false
            } else {
                unreachable!()
            }
        }
    }

    pub fn new_bool_eq(&self, s1: ActiveValue<SC>, s2: ActiveValue<SC>, fork_sink: &mut Option<SC::ForkSink>) -> ActiveValue<SC> {
        let mut selff = self.inner.try_borrow_mut().unwrap();
        selff.new_bool_eq(self.clone(), s1.clone(), s2.clone(), None, fork_sink)
    }

    pub fn new_bool_not(&self, s1: ActiveValue<SC>, fork_sink: &mut Option<SC::ForkSink>) -> ActiveValue<SC> {
        let mut selff = self.inner.try_borrow_mut().unwrap();
        selff.new_bool_not(self.clone(), s1.clone(), None, fork_sink)
    }

    pub fn new_bool_signed_less_than(&self, s1: ActiveValue<SC>, s2: ActiveValue<SC>, fork_sink: &mut Option<SC::ForkSink>) -> ActiveValue<SC> {
        let mut selff = self.inner.try_borrow_mut().unwrap();
        selff.new_bool_signed_less_than(self.clone(), s1.clone(), s2.clone(), None, fork_sink)
    }

    pub fn new_bool_unsigned_less_than(&self, s1: ActiveValue<SC>, s2: ActiveValue<SC>, fork_sink: &mut Option<SC::ForkSink>) -> ActiveValue<SC> {
        let mut selff = self.inner.try_borrow_mut().unwrap();
        selff.new_bool_unsigned_less_than(self.clone(), s1.clone(), s2.clone(), None, fork_sink)
    }

    pub fn new_bv_add(&self, s1: ActiveValue<SC>, s2: ActiveValue<SC>, width: u32, fork_sink: &mut Option<SC::ForkSink>) -> ActiveValue<SC> {
        let mut selff = self.inner.try_borrow_mut().unwrap();
        selff.new_bv_add(self.clone(), s1.clone(), s2.clone(), width, None, fork_sink)
    }

    pub fn new_bv_and(&self, s1: ActiveValue<SC>, s2: ActiveValue<SC>, width: u32, fork_sink: &mut Option<SC::ForkSink>) -> ActiveValue<SC> {
        let mut selff = self.inner.try_borrow_mut().unwrap();
        selff.new_bv_and(self.clone(), s1.clone(), s2.clone(), width, None, fork_sink)
    }

    pub fn new_bv_concat(&self, s1: ActiveValue<SC>, s2: ActiveValue<SC>, width: u32, fork_sink: &mut Option<SC::ForkSink>) -> ActiveValue<SC> {
        let mut selff = self.inner.try_borrow_mut().unwrap();
        selff.new_bv_concat(self.clone(), s1.clone(), s2.clone(), width, None, fork_sink)
    }

    pub fn new_bv_concrete(&self, value: u64, width: u32, fork_sink: &mut Option<SC::ForkSink>) -> ActiveValue<SC> {
        let mut selff = self.inner.try_borrow_mut().unwrap();
        selff.new_bv_concrete(self.clone(), value, width, None, fork_sink)
    }

    pub fn new_bv_multiply(&self, s1: ActiveValue<SC>, s2: ActiveValue<SC>, width: u32, fork_sink: &mut Option<SC::ForkSink>) -> ActiveValue<SC> {
        let mut selff = self.inner.try_borrow_mut().unwrap();
        selff.new_bv_multiply(self.clone(), s1.clone(), s2.clone(), width, None, fork_sink)
    }

    pub fn new_bv_or(&self, s1: ActiveValue<SC>, s2: ActiveValue<SC>, width: u32, fork_sink: &mut Option<SC::ForkSink>) -> ActiveValue<SC> {
        let mut selff = self.inner.try_borrow_mut().unwrap();
        selff.new_bv_or(self.clone(), s1.clone(), s2.clone(), width, None, fork_sink)
    }

    pub fn new_bv_sign_extend(&self, s1: ActiveValue<SC>, input_width: u32, output_width: u32, fork_sink: &mut Option<SC::ForkSink>) -> ActiveValue<SC> {
        let mut selff = self.inner.try_borrow_mut().unwrap();
        selff.new_bv_sign_extend(self.clone(), s1.clone(), input_width, output_width, None, fork_sink)
    }

    pub fn new_bv_slice(&self, s1: ActiveValue<SC>, high: u32, low: u32, fork_sink: &mut Option<SC::ForkSink>) -> ActiveValue<SC> {
        let mut selff = self.inner.try_borrow_mut().unwrap();
        selff.new_bv_slice(self.clone(), s1.clone(), high, low, None, fork_sink)
    }

    pub fn new_bv_sll(&self, s1: ActiveValue<SC>, s2: ActiveValue<SC>, width: u32, shamt_width: u32, fork_sink: &mut Option<SC::ForkSink>) -> ActiveValue<SC> {
        let mut selff = self.inner.try_borrow_mut().unwrap();
        selff.new_bv_sll(self.clone(), s1.clone(), s2.clone(), width, shamt_width, None, fork_sink)
    }

    pub fn new_bv_srl(&self, s1: ActiveValue<SC>, s2: ActiveValue<SC>, width: u32, shamt_width: u32, fork_sink: &mut Option<SC::ForkSink>) -> ActiveValue<SC> {
        let mut selff = self.inner.try_borrow_mut().unwrap();
        selff.new_bv_srl(self.clone(), s1.clone(), s2.clone(), width, shamt_width, None, fork_sink)
    }

    pub fn new_bv_sub(&self, s1: ActiveValue<SC>, s2: ActiveValue<SC>, width: u32, fork_sink: &mut Option<SC::ForkSink>) -> ActiveValue<SC> {
        let mut selff = self.inner.try_borrow_mut().unwrap();
        selff.new_bv_sub(self.clone(), s1.clone(), s2.clone(), width, None, fork_sink)
    }

    pub fn new_bv_symbol(&self, width: u32, fork_sink: &mut Option<SC::ForkSink>) -> ActiveValue<SC> {
        let mut selff = self.inner.try_borrow_mut().unwrap();
        selff.new_bv_symbol(self.clone(), width, None, fork_sink)
    }

    pub fn new_bv_unsigned_remainder(&self, s1: ActiveValue<SC>, s2: ActiveValue<SC>, width: u32, fork_sink: &mut Option<SC::ForkSink>) -> ActiveValue<SC> {
        let mut selff = self.inner.try_borrow_mut().unwrap();
        selff.new_bv_unsigned_remainder(self.clone(), s1.clone(), s2.clone(), width, None, fork_sink)
    }

    pub fn new_bv_xor(&self, s1: ActiveValue<SC>, s2: ActiveValue<SC>, width: u32, fork_sink: &mut Option<SC::ForkSink>) -> ActiveValue<SC> {
        let mut selff = self.inner.try_borrow_mut().unwrap();
        selff.new_bv_xor(self.clone(), s1.clone(), s2.clone(), width, None, fork_sink)
    }

    pub fn drop_active(&self, value: &ActiveValueInner<SC>) -> RetiredValue<SC> {
        let mut selff = self.inner.try_borrow_mut().unwrap();
        debug!("dropping active {}", value.id);
        assert!(selff.active_symbols.remove(&value.id).is_some());

        let retired_value = match &value.expression {
            ActiveExpression::BVConcrete(e) => selff.insert_retired(
                RetiredExpression::BVConcrete(RetiredBVConcrete {
                    value: e.value,
                    width: e.width,
                }),
                value.z3_ast,
                value.id,
                self.clone(),
            ),
            ActiveExpression::BVSymbol(e) => selff.insert_retired(
                RetiredExpression::BVSymbol(RetiredBVSymbol { width: e.width }),
                value.z3_ast,
                value.id,
                self.clone(),
            ),
            ActiveExpression::BVAddExpression(e) => selff.insert_retired(
                RetiredExpression::BVAddExpression(RetiredBVAddExpression {
                    s1: Rc::downgrade(&e.s1),
                    s2: Rc::downgrade(&e.s2),
                    width: e.width,
                }),
                value.z3_ast,
                value.id,
                self.clone(),
            ),
            ActiveExpression::BoolConcrete(e) => selff.insert_retired(
                RetiredExpression::BoolConcrete(RetiredBoolConcrete { value: e.value }),
                value.z3_ast,
                value.id,
                self.clone(),
            ),
            ActiveExpression::BoolEqExpression(e) => selff.insert_retired(
                RetiredExpression::BoolEqExpression(RetiredBoolEqExpression {
                    s1: Rc::downgrade(&e.s1),
                    s2: Rc::downgrade(&e.s2),
                    is_assert: e.is_assert,
                }),
                value.z3_ast,
                value.id,
                self.clone(),
            ),
            ActiveExpression::BoolNotExpression(e) => selff.insert_retired(
                RetiredExpression::BoolNotExpression(RetiredBoolNotExpression {
                    s1: Rc::downgrade(&e.s1),
                    is_assert: e.is_assert,
                }),
                value.z3_ast,
                value.id,
                self.clone(),
            ),
            ActiveExpression::BoolSignedLessThanExpression(e) => selff.insert_retired(
                RetiredExpression::BoolSignedLessThanExpression(RetiredBoolSignedLessThanExpression {
                    s1: Rc::downgrade(&e.s1),
                    s2: Rc::downgrade(&e.s2),
                    is_assert: e.is_assert,
                }),
                value.z3_ast,
                value.id,
                self.clone(),
            ),
            ActiveExpression::BoolUnsignedLessThanExpression(e) => selff.insert_retired(
                RetiredExpression::BoolUnsignedLessThanExpression(RetiredBoolUnsignedLessThanExpression {
                    s1: Rc::downgrade(&e.s1),
                    s2: Rc::downgrade(&e.s2),
                    is_assert: e.is_assert,
                }),
                value.z3_ast,
                value.id,
                self.clone(),
            ),
            ActiveExpression::BVAndExpression(e) => selff.insert_retired(
                RetiredExpression::BVAndExpression(RetiredBVAndExpression {
                    s1: Rc::downgrade(&e.s1),
                    s2: Rc::downgrade(&e.s2),
                    width: e.width,
                }),
                value.z3_ast,
                value.id,
                self.clone(),
            ),
            ActiveExpression::BVConcatExpression(e) => selff.insert_retired(
                RetiredExpression::BVConcatExpression(RetiredBVConcatExpression {
                    s1: Rc::downgrade(&e.s1),
                    s2: Rc::downgrade(&e.s2),
                    width: e.width,
                }),
                value.z3_ast,
                value.id,
                self.clone(),
            ),
            ActiveExpression::BVMultiplyExpression(e) => selff.insert_retired(
                RetiredExpression::BVMultiplyExpression(RetiredBVMultiplyExpression {
                    s1: Rc::downgrade(&e.s1),
                    s2: Rc::downgrade(&e.s2),
                    width: e.width,
                }),
                value.z3_ast,
                value.id,
                self.clone(),
            ),
            ActiveExpression::BVOrExpression(e) => selff.insert_retired(
                RetiredExpression::BVOrExpression(RetiredBVOrExpression {
                    s1: Rc::downgrade(&e.s1),
                    s2: Rc::downgrade(&e.s2),
                    width: e.width,
                }),
                value.z3_ast,
                value.id,
                self.clone(),
            ),
            ActiveExpression::BVSignExtendExpression(e) => selff.insert_retired(
                RetiredExpression::BVSignExtendExpression(RetiredBVSignExtendExpression {
                    s1: Rc::downgrade(&e.s1),
                    width: e.width,
                }),
                value.z3_ast,
                value.id,
                self.clone(),
            ),
            ActiveExpression::BVSliceExpression(e) => selff.insert_retired(
                RetiredExpression::BVSliceExpression(RetiredBVSliceExpression {
                    s1: Rc::downgrade(&e.s1),
                    width: e.width,
                    high: e.width,
                    low: e.width,
                }),
                value.z3_ast,
                value.id,
                self.clone(),
            ),
            ActiveExpression::BVSllExpression(e) => selff.insert_retired(
                RetiredExpression::BVSllExpression(RetiredBVSllExpression {
                    s1: Rc::downgrade(&e.s1),
                    s2: Rc::downgrade(&e.s2),
                    width: e.width,
                    shamt: e.shamt_width,
                }),
                value.z3_ast,
                value.id,
                self.clone(),
            ),
            ActiveExpression::BVSrlExpression(e) => selff.insert_retired(
                RetiredExpression::BVSrlExpression(RetiredBVSrlExpression {
                    s1: Rc::downgrade(&e.s1),
                    s2: Rc::downgrade(&e.s2),
                    width: e.width,
                    shamt: e.shamt,
                }),
                value.z3_ast,
                value.id,
                self.clone(),
            ),
            ActiveExpression::BVSubExpression(e) => selff.insert_retired(
                RetiredExpression::BVSubExpression(RetiredBVSubExpression {
                    s1: Rc::downgrade(&e.s1),
                    s2: Rc::downgrade(&e.s2),
                    width: e.width,
                }),
                value.z3_ast,
                value.id,
                self.clone(),
            ),
            ActiveExpression::BVUnsignedRemainderExpression(e) => selff.insert_retired(
                RetiredExpression::BVXorExpression(RetiredBVXorExpression {
                    s1: Rc::downgrade(&e.s1),
                    s2: Rc::downgrade(&e.s2),
                    width: e.width,
                }),
                value.z3_ast,
                value.id,
                self.clone(),
            ),
            ActiveExpression::BVXorExpression(e) => selff.insert_retired(
                RetiredExpression::BVXorExpression(RetiredBVXorExpression {
                    s1: Rc::downgrade(&e.s1),
                    s2: Rc::downgrade(&e.s2),
                    width: e.width,
                }),
                value.z3_ast,
                value.id,
                self.clone(),
            ),
        };

        // Heirs are parents and discovered symbols
        let mut heirs = vec![];
        value.expression.get_parents(&mut heirs);
        for discovered_ast in value.discovered_asts.values() {
            let acquaintance = discovered_ast.upgrade().unwrap();
            let mut acquaintance_ref = acquaintance.try_borrow_mut().unwrap();
            assert!(acquaintance_ref.discovered_asts.remove(&value.id).is_some());
            heirs.push(discovered_ast.upgrade().unwrap())
        }

        trace!("Retiring {:?}", value);
        for heir in &heirs {
            let mut heir_mut = heir.try_borrow_mut().unwrap();
            // Inherit
            assert!(heir_mut.inherited_asts.insert(value.id, retired_value.clone()).is_none());

            // Acquaint
            for other_heir in &heirs {
                if !Rc::ptr_eq(heir, other_heir) {
                    let mut other_heir_mut = other_heir.try_borrow_mut().unwrap();
                    other_heir_mut.discovered_asts.insert(heir_mut.id, Rc::downgrade(heir));
                    heir_mut.discovered_asts.insert(other_heir_mut.id, Rc::downgrade(other_heir));
                }
            }
        }

        selff.retired_symbols.insert(value.id, Rc::downgrade(&retired_value));
        retired_value
    }

    pub fn drop_retired(&self, value: &RetiredValueInner<SC>) {
        unsafe {
            trace!("Dropping {:?}", value);
            let mut selff = self.inner.try_borrow_mut().unwrap();
            assert!(selff.retired_symbols.remove(&value.id).is_some());
            Z3_dec_ref(selff.z3_context, value.z3_ast);
        }
    }

    pub(crate) fn assert(&self, value: &mut ActiveValueInner<SC>) {
        unsafe {
            let selff = self.inner.try_borrow_mut().unwrap();
            match &mut value.expression {
                ActiveExpression::BoolConcrete(e) => assert!(e.value),
                ActiveExpression::BoolNotExpression(e) => {
                    e.is_assert = true;
                    Z3_solver_assert(selff.z3_context, selff.z3_solver, value.z3_ast);
                }
                ActiveExpression::BoolEqExpression(e) => {
                    e.is_assert = true;
                    Z3_solver_assert(selff.z3_context, selff.z3_solver, value.z3_ast);
                }
                ActiveExpression::BoolSignedLessThanExpression(e) => {
                    e.is_assert = true;
                    Z3_solver_assert(selff.z3_context, selff.z3_solver, value.z3_ast);
                }
                ActiveExpression::BoolUnsignedLessThanExpression(e) => {
                    e.is_assert = true;
                    Z3_solver_assert(selff.z3_context, selff.z3_solver, value.z3_ast);
                }
                _ => panic!(),
            }
        }
    }

    // TODO check refcounting
    pub(crate) fn monomorphize_active(&self, value: &ActiveValueInner<SC>, candidates: &mut Vec<u64>) {
        unsafe {
            let selff = self.inner.try_borrow_mut().unwrap();
            let sort = Z3_get_sort(selff.z3_context, value.z3_ast);
            let sort_kind = Z3_get_sort_kind(selff.z3_context, sort);
            assert_eq!(SortKind::BV, sort_kind);

            // Fill assumptions with known candidates
            let mut assumptions: Vec<Z3_ast> = vec![];
            for candidate in candidates.iter() {
                let assumption = Z3_mk_not(
                    selff.z3_context,
                    Z3_mk_eq(selff.z3_context, Z3_mk_unsigned_int64(selff.z3_context, *candidate, sort), value.z3_ast),
                );
                Z3_inc_ref(selff.z3_context, assumption);
                assumptions.push(assumption)
            }

            // Find all remaining candidates
            loop {
                let assumptions_count = assumptions.len().try_into().unwrap();
                if Z3_solver_check_assumptions(selff.z3_context, selff.z3_solver, assumptions_count, assumptions.as_ptr()) == Z3_L_FALSE {
                    break;
                }

                let model = Z3_solver_get_model(selff.z3_context, selff.z3_solver);

                let mut z3_ast: Z3_ast = ptr::null_mut();
                assert!(Z3_model_eval(selff.z3_context, model, value.z3_ast, true, &mut z3_ast));

                let ast_kind = Z3_get_ast_kind(selff.z3_context, z3_ast);
                let mut v: u64 = 0;
                if ast_kind == AstKind::Numeral {
                    let size = Z3_get_bv_sort_size(selff.z3_context, Z3_get_sort(selff.z3_context, z3_ast));
                    assert_eq!(32, size);
                    assert!(Z3_get_numeral_uint64(selff.z3_context, z3_ast, &mut v));
                } else {
                    panic!("{:?}", ast_kind)
                }

                warn!("WARNING: Unpredicted monomorphization candidate 0x{:x} ", v);
                candidates.push(v);

                let assumption = Z3_mk_not(
                    selff.z3_context,
                    Z3_mk_eq(selff.z3_context, Z3_mk_unsigned_int64(selff.z3_context, v, sort), value.z3_ast),
                );
                Z3_inc_ref(selff.z3_context, assumption);
                assumptions.push(assumption)
            }

            for assumption in assumptions {
                Z3_dec_ref(selff.z3_context, assumption);
            }
        }
    }
}

impl<SC: ScfiaComposition> Default for Scfia<SC> {
    fn default() -> Self {
        Self::new()
    }
}

impl<SC: ScfiaComposition> ScfiaInner<SC> {
    pub fn new_bool_concrete(&mut self, self_rc: Scfia<SC>, value: bool, id: Option<u64>, fork_sink: &mut Option<SC::ForkSink>) -> ActiveValue<SC> {
        unsafe {
            let id = if let Some(id) = id { id } else { self.next_symbol_id() };
            let z3_ast = if value { Z3_mk_true(self.z3_context) } else { Z3_mk_false(self.z3_context) };
            Z3_inc_ref(self.z3_context, z3_ast);
            return self.insert_active(ActiveExpression::BoolConcrete(BoolConcrete { value }), z3_ast, id, self_rc, fork_sink);
        }
    }

    pub fn new_bool_eq(
        &mut self,
        self_rc: Scfia<SC>,
        s1: ActiveValue<SC>,
        s2: ActiveValue<SC>,
        id: Option<u64>,
        fork_sink: &mut Option<SC::ForkSink>,
    ) -> ActiveValue<SC> {
        unsafe {
            let s1_inner = s1.try_borrow().unwrap();
            let s2_inner = s2.try_borrow().unwrap();
            assert!(Rc::ptr_eq(&s1_inner.scfia.inner, &self_rc.inner));
            assert!(Rc::ptr_eq(&s2_inner.scfia.inner, &self_rc.inner));
            // TODO also assert width?
            let id = if let Some(id) = id { id } else { self.next_symbol_id() };
            if let ActiveExpression::BVConcrete(s1) = &s1_inner.expression {
                if let ActiveExpression::BVConcrete(s2) = &s2_inner.expression {
                    let z3_ast = if s1.value == s2.value {
                        Z3_mk_true(self.z3_context)
                    } else {
                        Z3_mk_false(self.z3_context)
                    };
                    Z3_inc_ref(self.z3_context, z3_ast);
                    return self.insert_active(
                        ActiveExpression::BoolConcrete(BoolConcrete { value: s1.value == s2.value }),
                        z3_ast,
                        id,
                        self_rc,
                        fork_sink,
                    );
                }
            };

            let z3_ast = Z3_mk_eq(self.z3_context, s1.try_borrow().unwrap().z3_ast, s2.try_borrow().unwrap().z3_ast);
            Z3_inc_ref(self.z3_context, z3_ast);
            self.insert_active(
                ActiveExpression::BoolEqExpression(BoolEqExpression {
                    s1: s1.clone(),
                    s2: s2.clone(),
                    is_assert: false,
                }),
                z3_ast,
                id,
                self_rc,
                fork_sink,
            )
        }
    }

    pub fn new_bool_not(&mut self, self_rc: Scfia<SC>, s1: ActiveValue<SC>, id: Option<u64>, fork_sink: &mut Option<SC::ForkSink>) -> ActiveValue<SC> {
        unsafe {
            let s1_inner = s1.try_borrow().unwrap();
            assert!(Rc::ptr_eq(&s1_inner.scfia.inner, &self_rc.inner));
            let id = if let Some(id) = id { id } else { self.next_symbol_id() };
            if let ActiveExpression::BoolConcrete(s1) = &s1_inner.expression {
                let z3_ast = if !s1.value {
                    Z3_mk_true(self.z3_context)
                } else {
                    Z3_mk_false(self.z3_context)
                };
                Z3_inc_ref(self.z3_context, z3_ast);
                return self.insert_active(
                    ActiveExpression::BoolConcrete(BoolConcrete { value: !s1.value }),
                    z3_ast,
                    id,
                    self_rc,
                    fork_sink,
                );
            }

            let z3_ast = Z3_mk_not(self.z3_context, s1.try_borrow().unwrap().z3_ast);
            Z3_inc_ref(self.z3_context, z3_ast);
            self.insert_active(
                ActiveExpression::BoolNotExpression(BoolNotExpression {
                    s1: s1.clone(),
                    is_assert: false,
                }),
                z3_ast,
                id,
                self_rc,
                fork_sink,
            )
        }
    }

    pub fn new_bool_signed_less_than(
        &mut self,
        self_rc: Scfia<SC>,
        s1: ActiveValue<SC>,
        s2: ActiveValue<SC>,
        id: Option<u64>,
        fork_sink: &mut Option<SC::ForkSink>,
    ) -> ActiveValue<SC> {
        unsafe {
            let s1_inner = s1.try_borrow().unwrap();
            let s2_inner = s2.try_borrow().unwrap();
            assert!(Rc::ptr_eq(&s1_inner.scfia.inner, &self_rc.inner));
            assert!(Rc::ptr_eq(&s2_inner.scfia.inner, &self_rc.inner));
            let id = if let Some(id) = id { id } else { self.next_symbol_id() };
            if let ActiveExpression::BVConcrete(s1) = &s1_inner.expression {
                if let ActiveExpression::BVConcrete(s2) = &s2_inner.expression {
                    let slt = (s1.value as i64) < (s2.value as i64);
                    let z3_ast = if slt { Z3_mk_true(self.z3_context) } else { Z3_mk_false(self.z3_context) };
                    Z3_inc_ref(self.z3_context, z3_ast);
                    return self.insert_active(ActiveExpression::BoolConcrete(BoolConcrete { value: slt }), z3_ast, id, self_rc, fork_sink);
                }
            }

            let z3_ast = Z3_mk_bvslt(self.z3_context, s1.try_borrow().unwrap().z3_ast, s2.try_borrow().unwrap().z3_ast);
            Z3_inc_ref(self.z3_context, z3_ast);
            self.insert_active(
                ActiveExpression::BoolSignedLessThanExpression(BoolSignedLessThanExpression {
                    s1: s1.clone(),
                    s2: s2.clone(),
                    is_assert: false,
                }),
                z3_ast,
                id,
                self_rc,
                fork_sink,
            )
        }
    }

    pub fn new_bool_unsigned_less_than(
        &mut self,
        self_rc: Scfia<SC>,
        s1: ActiveValue<SC>,
        s2: ActiveValue<SC>,
        id: Option<u64>,
        fork_sink: &mut Option<SC::ForkSink>,
    ) -> ActiveValue<SC> {
        unsafe {
            let s1_inner = s1.try_borrow().unwrap();
            let s2_inner = s2.try_borrow().unwrap();
            assert!(Rc::ptr_eq(&s1_inner.scfia.inner, &self_rc.inner));
            assert!(Rc::ptr_eq(&s2_inner.scfia.inner, &self_rc.inner));
            let id = if let Some(id) = id { id } else { self.next_symbol_id() };
            if let ActiveExpression::BVConcrete(s1) = &s1_inner.expression {
                if let ActiveExpression::BVConcrete(s2) = &s2_inner.expression {
                    let slt = s1.value < s2.value;
                    let z3_ast = if slt { Z3_mk_true(self.z3_context) } else { Z3_mk_false(self.z3_context) };
                    Z3_inc_ref(self.z3_context, z3_ast);
                    return self.insert_active(ActiveExpression::BoolConcrete(BoolConcrete { value: slt }), z3_ast, id, self_rc, fork_sink);
                }
            }

            let z3_ast = Z3_mk_bvult(self.z3_context, s1.try_borrow().unwrap().z3_ast, s2.try_borrow().unwrap().z3_ast);
            Z3_inc_ref(self.z3_context, z3_ast);
            self.insert_active(
                ActiveExpression::BoolUnsignedLessThanExpression(BoolUnsignedLessThanExpression {
                    s1: s1.clone(),
                    s2: s2.clone(),
                    is_assert: false,
                }),
                z3_ast,
                id,
                self_rc,
                fork_sink,
            )
        }
    }

    pub fn new_bv_add(
        &mut self,
        self_rc: Scfia<SC>,
        s1: ActiveValue<SC>,
        s2: ActiveValue<SC>,
        width: u32,
        id: Option<u64>,
        fork_sink: &mut Option<SC::ForkSink>,
    ) -> ActiveValue<SC> {
        unsafe {
            let s1_inner = s1.try_borrow().unwrap();
            let s2_inner = s2.try_borrow().unwrap();
            assert!(Rc::ptr_eq(&s1_inner.scfia.inner, &self_rc.inner));
            assert!(Rc::ptr_eq(&s2_inner.scfia.inner, &self_rc.inner));
            let id = if let Some(id) = id { id } else { self.next_symbol_id() };
            if let ActiveExpression::BVConcrete(s1) = &s1_inner.expression {
                if let ActiveExpression::BVConcrete(s2) = &s2_inner.expression {
                    let one: u64 = 1;
                    let mask = one.rotate_left(s2.width).overflowing_sub(1).0;
                    let sum = s1.value.overflowing_add(s2.value).0;
                    let value = mask & sum;
                    let sort = Z3_mk_bv_sort(self.z3_context, width);
                    let z3_ast = Z3_mk_unsigned_int64(self.z3_context, value, sort);
                    Z3_inc_ref(self.z3_context, z3_ast);
                    return self.insert_active(ActiveExpression::BVConcrete(BVConcrete { value, width }), z3_ast, id, self_rc, fork_sink);
                }
            };

            let z3_ast = Z3_mk_bvadd(self.z3_context, s1.try_borrow().unwrap().z3_ast, s2.try_borrow().unwrap().z3_ast);
            Z3_inc_ref(self.z3_context, z3_ast);
            self.insert_active(
                ActiveExpression::BVAddExpression(BVAddExpression {
                    s1: s1.clone(),
                    s2: s2.clone(),
                    width,
                }),
                z3_ast,
                id,
                self_rc,
                fork_sink,
            )
        }
    }

    pub fn new_bv_and(
        &mut self,
        self_rc: Scfia<SC>,
        s1: ActiveValue<SC>,
        s2: ActiveValue<SC>,
        width: u32,
        id: Option<u64>,
        fork_sink: &mut Option<SC::ForkSink>,
    ) -> ActiveValue<SC> {
        unsafe {
            let s1_inner = s1.try_borrow().unwrap();
            let s2_inner = s2.try_borrow().unwrap();
            assert!(Rc::ptr_eq(&s1_inner.scfia.inner, &self_rc.inner));
            let id = if let Some(id) = id { id } else { self.next_symbol_id() };
            if let ActiveExpression::BVConcrete(s1) = &s1_inner.expression {
                if let ActiveExpression::BVConcrete(s2) = &s2_inner.expression {
                    let value = s1.value & s2.value;
                    let sort = Z3_mk_bv_sort(self.z3_context, width);
                    let z3_ast = Z3_mk_unsigned_int64(self.z3_context, value, sort);
                    Z3_inc_ref(self.z3_context, z3_ast);
                    return self.insert_active(ActiveExpression::BVConcrete(BVConcrete { value, width }), z3_ast, id, self_rc, fork_sink);
                }
            };

            let z3_ast = Z3_mk_bvand(self.z3_context, s1.try_borrow().unwrap().z3_ast, s2.try_borrow().unwrap().z3_ast);
            Z3_inc_ref(self.z3_context, z3_ast);
            self.insert_active(
                ActiveExpression::BVAndExpression(BVAndExpression {
                    s1: s1.clone(),
                    s2: s2.clone(),
                    width,
                }),
                z3_ast,
                id,
                self_rc,
                fork_sink,
            )
        }
    }

    pub fn new_bv_concat(
        &mut self,
        self_rc: Scfia<SC>,
        s1: ActiveValue<SC>,
        s2: ActiveValue<SC>,
        width: u32,
        id: Option<u64>,
        fork_sink: &mut Option<SC::ForkSink>,
    ) -> ActiveValue<SC> {
        unsafe {
            let s1_inner = s1.try_borrow().unwrap();
            let s2_inner = s2.try_borrow().unwrap();
            assert!(Rc::ptr_eq(&s1_inner.scfia.inner, &self_rc.inner));
            let id = if let Some(id) = id { id } else { self.next_symbol_id() };
            if let ActiveExpression::BVConcrete(s1) = &s1_inner.expression {
                if let ActiveExpression::BVConcrete(s2) = &s2_inner.expression {
                    let e1_shifted = s1.value << s2.width;
                    let value = e1_shifted | s2.value;
                    let sort = Z3_mk_bv_sort(self.z3_context, width);
                    let z3_ast = Z3_mk_unsigned_int64(self.z3_context, value, sort);
                    Z3_inc_ref(self.z3_context, z3_ast);
                    return self.insert_active(ActiveExpression::BVConcrete(BVConcrete { value, width }), z3_ast, id, self_rc, fork_sink);
                }
            };

            let z3_ast = Z3_mk_concat(self.z3_context, s1.try_borrow().unwrap().z3_ast, s2.try_borrow().unwrap().z3_ast);
            Z3_inc_ref(self.z3_context, z3_ast);
            self.insert_active(
                ActiveExpression::BVConcatExpression(BVConcatExpression {
                    s1: s1.clone(),
                    s2: s2.clone(),
                    width,
                }),
                z3_ast,
                id,
                self_rc,
                fork_sink,
            )
        }
    }

    pub fn new_bv_concrete(&mut self, self_rc: Scfia<SC>, value: u64, width: u32, id: Option<u64>, fork_sink: &mut Option<SC::ForkSink>) -> ActiveValue<SC> {
        unsafe {
            //TODO assert value sticking to the width mask
            let sort = Z3_mk_bv_sort(self.z3_context, width);
            let z3_ast = Z3_mk_unsigned_int64(self.z3_context, value, sort);
            Z3_inc_ref(self.z3_context, z3_ast);
            let id = if let Some(id) = id { id } else { self.next_symbol_id() };
            self.insert_active(ActiveExpression::BVConcrete(BVConcrete { value, width }), z3_ast, id, self_rc, fork_sink)
        }
    }

    pub fn new_bv_multiply(
        &mut self,
        self_rc: Scfia<SC>,
        s1: ActiveValue<SC>,
        s2: ActiveValue<SC>,
        width: u32,
        id: Option<u64>,
        fork_sink: &mut Option<SC::ForkSink>,
    ) -> ActiveValue<SC> {
        unsafe {
            let s1_inner = s1.try_borrow().unwrap();
            let s2_inner = s2.try_borrow().unwrap();
            assert!(Rc::ptr_eq(&s1_inner.scfia.inner, &self_rc.inner));
            let id = if let Some(id) = id { id } else { self.next_symbol_id() };
            if let ActiveExpression::BVConcrete(s1) = &s1_inner.expression {
                if let ActiveExpression::BVConcrete(s2) = &s2_inner.expression {
                    // TODO check
                    let one: u64 = 1;
                    let mask = one.rotate_left(s2.width).overflowing_sub(1).0;
                    let product = s1.value.overflowing_mul(s2.value).0;
                    let value = mask & product;
                    let sort = Z3_mk_bv_sort(self.z3_context, width);
                    let z3_ast = Z3_mk_unsigned_int64(self.z3_context, value, sort);
                    Z3_inc_ref(self.z3_context, z3_ast);
                    return self.insert_active(ActiveExpression::BVConcrete(BVConcrete { value, width }), z3_ast, id, self_rc, fork_sink);
                }
            };

            let z3_ast = Z3_mk_bvmul(self.z3_context, s1.try_borrow().unwrap().z3_ast, s2.try_borrow().unwrap().z3_ast);
            Z3_inc_ref(self.z3_context, z3_ast);
            self.insert_active(
                ActiveExpression::BVMultiplyExpression(BVMultiplyExpression {
                    s1: s1.clone(),
                    s2: s2.clone(),
                    width,
                }),
                z3_ast,
                id,
                self_rc,
                fork_sink,
            )
        }
    }

    pub fn new_bv_or(
        &mut self,
        self_rc: Scfia<SC>,
        s1: ActiveValue<SC>,
        s2: ActiveValue<SC>,
        width: u32,
        id: Option<u64>,
        fork_sink: &mut Option<SC::ForkSink>,
    ) -> ActiveValue<SC> {
        unsafe {
            let s1_inner = s1.try_borrow().unwrap();
            let s2_inner = s2.try_borrow().unwrap();
            assert!(Rc::ptr_eq(&s1_inner.scfia.inner, &self_rc.inner));
            let id = if let Some(id) = id { id } else { self.next_symbol_id() };
            if let ActiveExpression::BVConcrete(s1) = &s1_inner.expression {
                if let ActiveExpression::BVConcrete(s2) = &s2_inner.expression {
                    let value = s1.value | s2.value;
                    let sort = Z3_mk_bv_sort(self.z3_context, width);
                    let z3_ast = Z3_mk_unsigned_int64(self.z3_context, value, sort);
                    Z3_inc_ref(self.z3_context, z3_ast);
                    return self.insert_active(ActiveExpression::BVConcrete(BVConcrete { value, width }), z3_ast, id, self_rc, fork_sink);
                }
            };

            let z3_ast = Z3_mk_bvor(self.z3_context, s1.try_borrow().unwrap().z3_ast, s2.try_borrow().unwrap().z3_ast);
            Z3_inc_ref(self.z3_context, z3_ast);
            self.insert_active(
                ActiveExpression::BVOrExpression(BVOrExpression {
                    s1: s1.clone(),
                    s2: s2.clone(),
                    width,
                }),
                z3_ast,
                id,
                self_rc,
                fork_sink,
            )
        }
    }

    pub fn new_bv_sign_extend(
        &mut self,
        self_rc: Scfia<SC>,
        s1: ActiveValue<SC>,
        input_width: u32,
        output_width: u32,
        id: Option<u64>,
        fork_sink: &mut Option<SC::ForkSink>,
    ) -> ActiveValue<SC> {
        unsafe {
            let s1_inner = s1.try_borrow().unwrap();
            assert!(Rc::ptr_eq(&s1_inner.scfia.inner, &self_rc.inner));
            let id = if let Some(id) = id { id } else { self.next_symbol_id() };
            if let ActiveExpression::BVConcrete(s1) = &s1_inner.expression {
                // https://graphics.stanford.edu/~seander/bithacks.html#VariableSignExtend
                let m: u64 = 1 << (s1.width - 1);
                let x = s1.value & ((1 << s1.width) - 1);
                let value = (x ^ m).overflowing_sub(m).0;
                let sort = Z3_mk_bv_sort(self.z3_context, output_width);
                let z3_ast = Z3_mk_unsigned_int64(self.z3_context, value, sort);
                Z3_inc_ref(self.z3_context, z3_ast);
                return self.insert_active(
                    ActiveExpression::BVConcrete(BVConcrete { value, width: output_width }),
                    z3_ast,
                    id,
                    self_rc,
                    fork_sink,
                );
            };

            let z3_ast = Z3_mk_sign_ext(self.z3_context, output_width - input_width, s1.try_borrow().unwrap().z3_ast);
            Z3_inc_ref(self.z3_context, z3_ast);
            self.insert_active(
                ActiveExpression::BVSignExtendExpression(BVSignExtendExpression {
                    s1: s1.clone(),
                    width: output_width,
                }),
                z3_ast,
                id,
                self_rc,
                fork_sink,
            )
        }
    }

    pub fn new_bv_slice(
        &mut self,
        self_rc: Scfia<SC>,
        s1: ActiveValue<SC>,
        high: u32,
        low: u32,
        id: Option<u64>,
        fork_sink: &mut Option<SC::ForkSink>,
    ) -> ActiveValue<SC> {
        unsafe {
            let s1_inner = s1.try_borrow().unwrap();
            assert!(Rc::ptr_eq(&s1_inner.scfia.inner, &self_rc.inner));
            let id = if let Some(id) = id { id } else { self.next_symbol_id() };
            let width = high - low + 1;
            if let ActiveExpression::BVConcrete(s1) = &s1_inner.expression {
                let shifted = s1.value >> low;
                let mask = (1 << width) - 1;
                let value = shifted & mask;
                let sort = Z3_mk_bv_sort(self.z3_context, width);
                let z3_ast = Z3_mk_unsigned_int64(self.z3_context, value, sort);
                Z3_inc_ref(self.z3_context, z3_ast);
                return self.insert_active(ActiveExpression::BVConcrete(BVConcrete { value, width }), z3_ast, id, self_rc, fork_sink);
            };

            let z3_ast = Z3_mk_extract(self.z3_context, high, low, s1.try_borrow().unwrap().z3_ast);
            Z3_inc_ref(self.z3_context, z3_ast);
            self.insert_active(
                ActiveExpression::BVSliceExpression(BVSliceExpression {
                    s1: s1.clone(),
                    width,
                    high,
                    low,
                }),
                z3_ast,
                id,
                self_rc,
                fork_sink,
            )
        }
    }

    pub fn new_bv_sll(
        &mut self,
        self_rc: Scfia<SC>,
        s1: ActiveValue<SC>,
        s2: ActiveValue<SC>,
        width: u32,
        shamt_width: u32,
        id: Option<u64>,
        fork_sink: &mut Option<SC::ForkSink>,
    ) -> ActiveValue<SC> {
        unsafe {
            let s1_inner = s1.try_borrow().unwrap();
            let s2_inner = s2.try_borrow().unwrap();
            assert!(Rc::ptr_eq(&s1_inner.scfia.inner, &self_rc.inner));
            assert!(Rc::ptr_eq(&s2_inner.scfia.inner, &self_rc.inner));
            let id = if let Some(id) = id { id } else { self.next_symbol_id() };
            if let ActiveExpression::BVConcrete(s1) = &s1_inner.expression {
                if let ActiveExpression::BVConcrete(s2) = &s2_inner.expression {
                    let one: u64 = 1;
                    let mask = one.rotate_left(s2.width).overflowing_sub(1).0;
                    let value = (s1.value << s2.value) & mask;
                    let sort = Z3_mk_bv_sort(self.z3_context, width);
                    let z3_ast = Z3_mk_unsigned_int64(self.z3_context, value, sort);
                    Z3_inc_ref(self.z3_context, z3_ast);
                    return self.insert_active(ActiveExpression::BVConcrete(BVConcrete { value, width }), z3_ast, id, self_rc, fork_sink);
                }
            };

            let shamt_ast = if width == shamt_width {
                let ast = Z3_mk_zero_ext(self.z3_context, width - shamt_width, s2_inner.z3_ast);
                Z3_inc_ref(self.z3_context, ast); //TODO decrement secondary ASTs
                Some(ast)
            } else {
                None
            };

            let z3_ast = Z3_mk_bvshl(self.z3_context, s1_inner.z3_ast, shamt_ast.unwrap_or(s2_inner.z3_ast));
            Z3_inc_ref(self.z3_context, z3_ast);
            self.insert_active(
                ActiveExpression::BVSllExpression(BVSllExpression {
                    s1: s1.clone(),
                    s2: s2.clone(),
                    width,
                    shamt_width,
                }),
                z3_ast,
                id,
                self_rc,
                fork_sink,
            )
        }
    }

    pub fn new_bv_srl(
        &mut self,
        self_rc: Scfia<SC>,
        s1: ActiveValue<SC>,
        s2: ActiveValue<SC>,
        width: u32,
        shamt_width: u32,
        id: Option<u64>,
        fork_sink: &mut Option<SC::ForkSink>,
    ) -> ActiveValue<SC> {
        unsafe {
            let s1_inner = s1.try_borrow().unwrap();
            let s2_inner = s2.try_borrow().unwrap();
            assert!(Rc::ptr_eq(&s1_inner.scfia.inner, &self_rc.inner));
            assert!(Rc::ptr_eq(&s2_inner.scfia.inner, &self_rc.inner));
            let id = if let Some(id) = id { id } else { self.next_symbol_id() };
            if let ActiveExpression::BVConcrete(s1) = &s1_inner.expression {
                if let ActiveExpression::BVConcrete(s2) = &s2_inner.expression {
                    let one: u64 = 1;
                    let mask = one.rotate_left(s2.width).overflowing_sub(1).0;
                    let value = (s1.value >> s2.value) & mask;
                    let sort = Z3_mk_bv_sort(self.z3_context, width);
                    let z3_ast = Z3_mk_unsigned_int64(self.z3_context, value, sort);
                    Z3_inc_ref(self.z3_context, z3_ast);
                    return self.insert_active(ActiveExpression::BVConcrete(BVConcrete { value, width }), z3_ast, id, self_rc, fork_sink);
                }
            };

            let shamt_ast = if width == shamt_width {
                let ast = Z3_mk_zero_ext(self.z3_context, width - shamt_width, s2_inner.z3_ast);
                Z3_inc_ref(self.z3_context, ast); //TODO decrement secondary ASTs
                Some(ast)
            } else {
                None
            };

            let z3_ast = Z3_mk_bvlshr(self.z3_context, s1_inner.z3_ast, shamt_ast.unwrap_or(s2_inner.z3_ast));
            Z3_inc_ref(self.z3_context, z3_ast);
            self.insert_active(
                ActiveExpression::BVSllExpression(BVSllExpression {
                    s1: s1.clone(),
                    s2: s2.clone(),
                    width,
                    shamt_width,
                }),
                z3_ast,
                id,
                self_rc,
                fork_sink,
            )
        }
    }

    pub fn new_bv_sub(
        &mut self,
        self_rc: Scfia<SC>,
        s1: ActiveValue<SC>,
        s2: ActiveValue<SC>,
        width: u32,
        id: Option<u64>,
        fork_sink: &mut Option<SC::ForkSink>,
    ) -> ActiveValue<SC> {
        unsafe {
            let s1_inner = s1.try_borrow().unwrap();
            let s2_inner = s2.try_borrow().unwrap();
            assert!(Rc::ptr_eq(&s1_inner.scfia.inner, &self_rc.inner));
            assert!(Rc::ptr_eq(&s2_inner.scfia.inner, &self_rc.inner));
            let id = if let Some(id) = id { id } else { self.next_symbol_id() };
            if let ActiveExpression::BVConcrete(s1) = &s1_inner.expression {
                if let ActiveExpression::BVConcrete(s2) = &s2_inner.expression {
                    // TODO check
                    let one: u64 = 1;
                    let mask = one.rotate_left(s2.width).overflowing_sub(1).0;
                    let sum = s1.value.overflowing_sub(s2.value).0;
                    let value = mask & sum;
                    let sort = Z3_mk_bv_sort(self.z3_context, width);
                    let z3_ast = Z3_mk_unsigned_int64(self.z3_context, value, sort);
                    Z3_inc_ref(self.z3_context, z3_ast);
                    return self.insert_active(ActiveExpression::BVConcrete(BVConcrete { value, width }), z3_ast, id, self_rc, fork_sink);
                }
            };

            let z3_ast = Z3_mk_bvsub(self.z3_context, s1.try_borrow().unwrap().z3_ast, s2.try_borrow().unwrap().z3_ast);
            Z3_inc_ref(self.z3_context, z3_ast);
            self.insert_active(
                ActiveExpression::BVSubExpression(BVSubExpression {
                    s1: s1.clone(),
                    s2: s2.clone(),
                    width,
                }),
                z3_ast,
                id,
                self_rc,
                fork_sink,
            )
        }
    }

    pub fn new_bv_symbol(&mut self, self_rc: Scfia<SC>, width: u32, id: Option<u64>, fork_sink: &mut Option<SC::ForkSink>) -> ActiveValue<SC> {
        unsafe {
            let id = if let Some(id) = id { id } else { self.next_symbol_id() };
            let z3_ast = Z3_mk_fresh_const(self.z3_context, 0 as Z3_string, Z3_mk_bv_sort(self.z3_context, width));
            Z3_inc_ref(self.z3_context, z3_ast);

            self.insert_active(ActiveExpression::BVSymbol(BVSymbol { width }), z3_ast, id, self_rc, fork_sink)
        }
    }

    pub fn new_bv_unsigned_remainder(
        &mut self,
        self_rc: Scfia<SC>,
        s1: ActiveValue<SC>,
        s2: ActiveValue<SC>,
        width: u32,
        id: Option<u64>,
        fork_sink: &mut Option<SC::ForkSink>,
    ) -> ActiveValue<SC> {
        unsafe {
            let s1_inner = s1.try_borrow().unwrap();
            let s2_inner = s2.try_borrow().unwrap();
            assert!(Rc::ptr_eq(&s1_inner.scfia.inner, &self_rc.inner));
            let id = if let Some(id) = id { id } else { self.next_symbol_id() };
            if let ActiveExpression::BVConcrete(s1) = &s1_inner.expression {
                if let ActiveExpression::BVConcrete(s2) = &s2_inner.expression {
                    // TODO check
                    let one: u64 = 1;
                    let mask = one.rotate_left(s2.width).overflowing_sub(1).0;
                    let remainder = s1.value % s2.value;
                    let value = mask & remainder;
                    let sort = Z3_mk_bv_sort(self.z3_context, width);
                    let z3_ast = Z3_mk_unsigned_int64(self.z3_context, value, sort);
                    Z3_inc_ref(self.z3_context, z3_ast);
                    return self.insert_active(ActiveExpression::BVConcrete(BVConcrete { value, width }), z3_ast, id, self_rc, fork_sink);
                }
            };

            let z3_ast = Z3_mk_bvurem(self.z3_context, s1.try_borrow().unwrap().z3_ast, s2.try_borrow().unwrap().z3_ast);
            Z3_inc_ref(self.z3_context, z3_ast);
            self.insert_active(
                ActiveExpression::BVUnsignedRemainderExpression(BVUnsignedRemainderExpression {
                    s1: s1.clone(),
                    s2: s2.clone(),
                    width,
                }),
                z3_ast,
                id,
                self_rc,
                fork_sink,
            )
        }
    }

    fn new_bv_xor(
        &mut self,
        self_rc: Scfia<SC>,
        s1: ActiveValue<SC>,
        s2: ActiveValue<SC>,
        width: u32,
        id: Option<u64>,
        fork_sink: &mut Option<SC::ForkSink>,
    ) -> ActiveValue<SC> {
        unsafe {
            let s1_inner = s1.try_borrow().unwrap();
            let s2_inner = s2.try_borrow().unwrap();
            assert!(Rc::ptr_eq(&s1_inner.scfia.inner, &self_rc.inner));
            assert!(Rc::ptr_eq(&s2_inner.scfia.inner, &self_rc.inner));
            let id = if let Some(id) = id { id } else { self.next_symbol_id() };
            if let ActiveExpression::BVConcrete(s1) = &s1_inner.expression {
                if let ActiveExpression::BVConcrete(s2) = &s2_inner.expression {
                    // TODO check
                    let value = s1.value ^ s2.value;
                    let sort = Z3_mk_bv_sort(self.z3_context, width);
                    let z3_ast = Z3_mk_unsigned_int64(self.z3_context, value, sort);
                    Z3_inc_ref(self.z3_context, z3_ast);
                    return self.insert_active(ActiveExpression::BVConcrete(BVConcrete { value, width }), z3_ast, id, self_rc, fork_sink);
                }
            };

            let z3_ast = Z3_mk_bvxor(self.z3_context, s1.try_borrow().unwrap().z3_ast, s2.try_borrow().unwrap().z3_ast);
            Z3_inc_ref(self.z3_context, z3_ast);
            self.insert_active(
                ActiveExpression::BVXorExpression(BVXorExpression {
                    s1: s1.clone(),
                    s2: s2.clone(),
                    width,
                }),
                z3_ast,
                id,
                self_rc,
                fork_sink,
            )
        }
    }

    fn next_symbol_id(&mut self) -> u64 {
        let id = self.next_symbol_id;
        self.next_symbol_id = id + 1;
        id
    }

    fn insert_active(
        &mut self,
        expression: ActiveExpression<SC>,
        z3_ast: Z3_ast,
        id: u64,
        scfia: Scfia<SC>,
        fork_sink: &mut Option<SC::ForkSink>,
    ) -> ActiveValue<SC> {
        let value = Rc::new(RefCell::new(ActiveValueInner {
            id,
            z3_ast,
            expression,
            inherited_asts: BTreeMap::new(),
            discovered_asts: BTreeMap::new(),
            scfia,
        }));
        trace!("Creating new expression ({:?})", value.try_borrow().unwrap());
        self.active_symbols.insert(id, Rc::downgrade(&value));
        if let Some(fork_sink) = fork_sink {
            fork_sink.push_value(value.clone())
        }
        value
    }

    fn insert_retired(&mut self, expression: RetiredExpression<SC>, z3_ast: Z3_ast, id: u64, scfia: Scfia<SC>) -> RetiredValue<SC> {
        let value = Rc::new(RefCell::new(RetiredValueInner { id, z3_ast, expression, scfia }));
        trace!("Creating new retired expression ({:?})", value.try_borrow().unwrap());
        self.retired_symbols.insert(id, Rc::downgrade(&value));
        value
    }

    pub fn clone_values(&self) -> (Scfia<SC>, BTreeMap<u64, ActiveValue<SC>>) {
        unsafe {
            let z3_config = Z3_mk_config();
            let z3_context = Z3_mk_context_rc(z3_config);
            let z3_solver = Z3_mk_solver(z3_context);
            Z3_solver_inc_ref(z3_context, z3_solver);
            Z3_del_config(z3_config);
            let mut cloned_actives = BTreeMap::new();
            let cloned_scfia_rc = Scfia {
                inner: Rc::new(RefCell::new(ScfiaInner {
                    next_symbol_id: self.next_symbol_id,
                    active_symbols: BTreeMap::new(),
                    retired_symbols: BTreeMap::new(),
                    z3_context,
                    z3_solver,
                })),
            };
            let mut cloned_scfia = cloned_scfia_rc.inner.try_borrow_mut().unwrap();

            debug!("Cloning {} actives and {} retireds", self.active_symbols.len(), self.retired_symbols.len());
            let mut active_iter = self.active_symbols.iter();
            let mut retired_iter = self.retired_symbols.iter();
            let mut next_active_option = active_iter.next();
            let mut next_retired_option = retired_iter.next();

            loop {
                if let Some(next_active) = next_active_option {
                    if let Some(next_retired) = next_retired_option {
                        if next_active.0 < next_retired.0 {
                            // Take active
                            trace!("Cloning active {:?}", next_active.1.upgrade());
                            let next_active_ref = next_active.1.upgrade().unwrap();
                            let next_active_ref = next_active_ref.try_borrow().unwrap();
                            let cloned_active = next_active_ref
                                .clone_to(self, &mut cloned_scfia, cloned_scfia_rc.clone(), Some(*next_active.0));
                            cloned_actives.insert(*next_active.0, cloned_active);
                            next_active_option = active_iter.next();
                        } else {
                            // Take retired
                            trace!("Cloning retired {:?}", next_retired.1.upgrade());
                            next_retired_option = retired_iter.next();
                            todo!()
                        }
                    } else {
                        // Take active
                        trace!("Cloning active {:?}", next_active.1.upgrade());
                        let next_active_ref = next_active.1.upgrade().unwrap();
                        let next_active_ref = next_active_ref.try_borrow().unwrap();
                        let cloned_active = next_active_ref
                            .clone_to(self, &mut cloned_scfia, cloned_scfia_rc.clone(), Some(*next_active.0));
                        cloned_actives.insert(*next_active.0, cloned_active);
                        next_active_option = active_iter.next();
                    }
                } else if let Some(next_retired) = next_retired_option {
                    // Take retired
                    trace!("Cloning retired {:?}", next_retired.1.upgrade());
                    next_retired_option = retired_iter.next();
                    todo!()
                } else {
                    break;
                }
            }
            // warn!("{:?}", cloned_scfia.active_symbols.get(&5).unwrap().upgrade());
            debug_assert_eq!(cloned_scfia.active_symbols.len(), self.active_symbols.len());

            (cloned_scfia_rc.clone(), cloned_actives)
        }
    }
}

impl<SC: ScfiaComposition> Drop for ScfiaInner<SC> {
    fn drop(&mut self) {
        trace!("Dropping ScfiaInner");
        unsafe { Z3_del_context(self.z3_context) }
    }
}
